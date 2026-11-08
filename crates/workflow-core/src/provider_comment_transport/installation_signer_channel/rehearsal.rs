#![allow(clippy::expect_used)]

use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use super::decision::InstallationSignerDecisionChannel;
use super::{InstallationSignerReceiver, SignerPeerPolicy};
use crate::Timestamp;

pub(crate) const POLICY: SignerPeerPolicy = SignerPeerPolicy {
    exchange_uid: 61_021,
    signer_uid: 61_022,
    harness_uids: [61_023, 61_024],
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Trial {
    Success,
    BeforeCommit,
    AfterCommit,
    AfterCallback,
}

pub(crate) fn trial() -> Trial {
    match std::env::var("WORKFLOW_OS_SIGNER_TRIAL").as_deref() {
        Ok("success") => Trial::Success,
        Ok("before-commit") => Trial::BeforeCommit,
        Ok("after-commit") => Trial::AfterCommit,
        Ok("after-callback") => Trial::AfterCallback,
        _ => unreachable!("invalid isolated rehearsal trial"),
    }
}

pub(crate) fn assert_isolated(uid: u32) {
    assert_eq!(nix::unistd::geteuid().as_raw(), uid);
    assert_eq!(nix::unistd::getuid().as_raw(), uid);
    let gid = if POLICY.harness_uids.contains(&uid) {
        uid
    } else {
        61_020
    };
    assert_eq!(nix::unistd::getegid().as_raw(), gid);
    assert_eq!(nix::unistd::getgid().as_raw(), gid);
    assert_platform_isolation(gid);
}

#[cfg(target_os = "linux")]
fn assert_platform_isolation(_gid: u32) {
    assert_eq!(
        nix::unistd::getgroups().expect("supplementary groups"),
        [] as [nix::unistd::Gid; 0]
    );
    let status = fs::read_to_string("/proc/self/status").expect("process isolation status");
    let field = |name| {
        status
            .lines()
            .filter_map(|line| line.split_once(':'))
            .find_map(|(key, value)| (key == name).then_some(value.trim()))
            .expect("process status field")
    };
    assert_eq!(field("NoNewPrivs"), "1");
    for name in ["CapInh", "CapPrm", "CapEff", "CapBnd", "CapAmb"] {
        assert_eq!(field(name), "0000000000000000");
    }
    assert_eq!(
        fs::read_to_string("/proc/net/dev")
            .expect("network namespace interfaces")
            .lines()
            .skip(2)
            .filter_map(|line| line.split_once(':').map(|(name, _)| name.trim()))
            .collect::<Vec<_>>(),
        ["lo"]
    );
    // Ignore the descriptor opened by read_dir itself.
    let descriptor_directory = format!("/proc/{}/fd", std::process::id());
    for entry in fs::read_dir("/proc/self/fd").expect("descriptor list") {
        let entry = entry.expect("descriptor");
        let fd = entry
            .file_name()
            .to_string_lossy()
            .parse::<u32>()
            .expect("fd number");
        let target = fs::read_link(entry.path()).expect("descriptor target");
        assert!(
            fd <= 2 || (fd == 3 && target == Path::new(&descriptor_directory)),
            "unexpected inherited descriptor"
        );
    }
}

#[cfg(target_os = "macos")]
fn assert_platform_isolation(gid: u32) {
    use std::net::{TcpListener, TcpStream};
    use std::process::Command;

    let groups = Command::new("/usr/bin/id")
        .arg("-G")
        .env_clear()
        .output()
        .expect("inspect process groups");
    assert!(groups.status.success());
    let groups = std::str::from_utf8(&groups.stdout).expect("numeric groups");
    let actual = groups
        .split_whitespace()
        .map(|group| group.parse::<u32>().expect("group ID"))
        .collect::<Vec<_>>();
    assert_eq!(actual, [gid]);

    for address in ["127.0.0.1:0", "[::1]:0"] {
        assert_eq!(
            TcpListener::bind(address).err().map(|error| error.kind()),
            Some(ErrorKind::PermissionDenied),
            "IP bind must be denied by the rehearsal profile"
        );
    }
    for address in ["127.0.0.1:9", "[::1]:9"] {
        assert_eq!(
            TcpStream::connect(address).err().map(|error| error.kind()),
            Some(ErrorKind::PermissionDenied),
            "IP connect must be denied by the rehearsal profile"
        );
    }
}

pub(crate) fn fixture_root() -> PathBuf {
    let root =
        PathBuf::from(std::env::var_os("WORKFLOW_OS_SIGNER_REHEARSAL").expect("fixture root"));
    assert!(root.is_absolute());
    root
}

pub(crate) fn bind_socket(path: &Path) -> UnixListener {
    let listener = UnixListener::bind(path).expect("bind rehearsal listener under service UID");
    fs::set_permissions(path, fs::Permissions::from_mode(0o660)).expect("restrict socket access");
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    listener
}

pub(crate) fn accept_socket(listener: &UnixListener) -> UnixStream {
    let stop = Instant::now() + Duration::from_secs(15);
    loop {
        match listener.accept() {
            Ok((stream, _)) => return stream,
            Err(error) => {
                assert_eq!(error.kind(), ErrorKind::WouldBlock);
                assert!(Instant::now() < stop, "rehearsal accept timed out");
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

pub(crate) fn connect_socket(path: &Path) -> UnixStream {
    let stop = Instant::now() + Duration::from_secs(15);
    loop {
        match UnixStream::connect(path) {
            Ok(stream) => return stream,
            Err(error) => {
                assert!(matches!(
                    error.kind(),
                    ErrorKind::NotFound
                        | ErrorKind::ConnectionRefused
                        | ErrorKind::PermissionDenied
                ));
                assert!(Instant::now() < stop, "rehearsal connect timed out");
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

pub(crate) fn assert_file_inaccessible(path: &Path) {
    assert_eq!(
        OpenOptions::new()
            .read(true)
            .open(path)
            .err()
            .map(|e| e.kind()),
        Some(ErrorKind::PermissionDenied)
    );
    assert_eq!(
        OpenOptions::new()
            .write(true)
            .open(path)
            .err()
            .map(|e| e.kind()),
        Some(ErrorKind::PermissionDenied)
    );
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_signer_process() {
    assert_isolated(POLICY.signer_uid);
    let trial = trial();
    let root = fixture_root();
    assert_file_inaccessible(&root.join("core-private/authority"));
    assert!(fs::metadata(root.join("signer-private/custody")).is_ok());
    let probe_path = root.join("signer-socket/probe");
    let probe = bind_socket(&probe_path);
    let wrong_peer = connect_socket(&probe_path);
    assert!(InstallationSignerReceiver::authenticate(accept_socket(&probe), &POLICY).is_err());
    drop(wrong_peer);
    drop(probe);
    let listener = bind_socket(&root.join("signer-socket/assertion"));
    let receiver = InstallationSignerReceiver::authenticate(accept_socket(&listener), &POLICY)
        .expect("assertion exchange peer");
    let decision = InstallationSignerDecisionChannel::authenticate(
        connect_socket(&root.join("core-socket/decision")),
        &POLICY,
    )
    .expect("Core decision peer");
    let result = receiver.serve_once_via_core(decision, |_| {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join("signer-private/callback"))
            .expect("one synthetic callback");
        if trial == Trial::AfterCallback {
            std::process::exit(77);
        }
        Some((
            "fixture-header.fixture-payload.fixture-signature".to_owned(),
            Timestamp::from_offset_date_time(
                Timestamp::now_utc().as_offset_date_time() + time::Duration::seconds(30),
            ),
        ))
    });
    assert_eq!(result.is_ok(), trial == Trial::Success);
    assert_eq!(
        root.join("signer-private/callback").exists(),
        trial == Trial::Success
    );
}

#[test]
#[ignore = "requires the hosted signer rehearsal launcher"]
fn rehearsal_untrusted_process() {
    assert_isolated(POLICY.harness_uids[0]);
    let root = fixture_root();
    for file in ["core-private/authority", "signer-private/custody"] {
        assert_file_inaccessible(&root.join(file));
    }
    for socket in ["core-socket/decision", "signer-socket/assertion"] {
        let path = root.join(socket);
        assert_eq!(
            UnixStream::connect(&path).err().map(|e| e.kind()),
            Some(ErrorKind::PermissionDenied)
        );
        assert_eq!(
            fs::remove_file(path).err().map(|e| e.kind()),
            Some(ErrorKind::PermissionDenied)
        );
    }
}
