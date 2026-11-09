#![allow(clippy::expect_used)]

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Read};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use super::decision::InstallationSignerDecisionChannel;
use super::{InstallationSignerReceiver, SignerPeerPolicy};
use crate::{SpecContentHash, Timestamp};

pub(crate) mod reader;

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
        Ok("success" | "operator") => Trial::Success,
        Ok("before-commit") => Trial::BeforeCommit,
        Ok("after-commit") => Trial::AfterCommit,
        Ok("after-callback") => Trial::AfterCallback,
        _ => unreachable!("invalid isolated rehearsal trial"),
    }
}

pub(crate) fn exchange_operator_assertion(
    reviewed: crate::provider_comment_transport::installation_exchange::ReviewedInstallation<'_>,
    request: &super::InstallationSignerRequest<'_>,
    assertion: super::InstallationAppAssertion,
) -> Option<crate::provider_comment_transport::installation_exchange::VerifiedInstallationScope> {
    let (value, expires) =
        assertion.into_writer_parts(request, &reviewed.issuance_commitment()?)?;
    (value == "fixture-header.fixture-payload.fixture-signature").then(|| {
        crate::provider_comment_transport::installation_exchange::VerifiedInstallationScope::test_receipt_with_live_shape(
            reviewed, Timestamp::now_utc(), expires,
        )
    })
}

pub(crate) fn assert_isolated(uid: u32) {
    if !only_standard_descriptors() {
        std::process::exit(76);
    }
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
    let _ = launch_commitment();
}

fn only_standard_descriptors() -> bool {
    // Collect first so the directory's own descriptor is closed before probing.
    let Ok(entries) = fs::read_dir("/dev/fd") else {
        return false;
    };
    let Ok(entries) = entries
        .map(|entry| entry.map(|entry| (entry.file_name(), entry.path())))
        .collect::<Result<Vec<_>, _>>()
    else {
        return false;
    };
    entries.iter().all(|(name, path)| {
        name.to_str()
            .and_then(|name| name.parse::<u32>().ok())
            .is_some_and(|fd| {
                fd <= 2
                    || fs::symlink_metadata(path).is_err_and(|error| {
                        error.kind() == ErrorKind::NotFound
                            || error.raw_os_error() == Some(nix::libc::EBADF)
                    })
            })
    })
}

#[test]
fn rehearsal_descriptor_guard_distinguishes_clean_and_inherited_launches() {
    let executable = std::env::current_exe().expect("fixture executable");
    let worker = format!("{}::rehearsal_descriptor_probe", module_path!());
    let (_, worker) = worker.split_once("::").expect("test path");
    for (injected, expected) in [("no", 0), ("yes", 76)] {
        let result = std::process::Command::new("/bin/sh")
            .args([
                "-c",
                "exec 3<&- 4<&- 5<&- 6<&- 7<&- 8<&- 9<&-; \
                 if [ \"$1\" = yes ]; then exec 9</dev/null; fi; shift; exec \"$@\"",
                "sh",
                injected,
            ])
            .arg(&executable)
            .args(["--ignored", "--exact", worker, "--test-threads=1"])
            .env_clear()
            .output()
            .expect("launch descriptor probe");
        assert_eq!(
            result.status.code(),
            Some(expected),
            "{} {}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        if !matches!(injected, "yes") {
            assert!(String::from_utf8_lossy(&result.stdout).contains("1 passed"));
        }
    }
}

#[test]
#[ignore = "launched by the descriptor regression fixture"]
fn rehearsal_descriptor_probe() {
    if !only_standard_descriptors() {
        std::process::exit(76);
    }
}

pub(crate) fn executable_digest() -> SpecContentHash {
    file_digest(&std::env::current_exe().expect("fixture executable"))
}

fn file_digest(path: &Path) -> SpecContentHash {
    let mut file = fs::File::open(path).expect("open reviewed fixture file");
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    let mut buffer = [0; 8192];
    loop {
        let read = file.read(&mut buffer).expect("read executable bytes");
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let bytes = hasher.finish();
    let hex = bytes
        .as_ref()
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            use std::fmt::Write as _;
            write!(&mut hex, "{byte:02x}").expect("write digest");
            hex
        });
    SpecContentHash::new(hex).expect("SHA-256 digest")
}

pub(crate) fn launch_commitment() -> SpecContentHash {
    let root = fixture_root();
    let binary = std::env::current_exe().expect("running worker");
    assert_eq!(
        binary,
        root.parent().expect("protected root").join("worker")
    );
    let meta = binary.metadata().expect("worker metadata");
    assert!(meta.is_file() && meta.uid() == 0 && meta.mode() & 0o7777 == 0o555);
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(root.join("operator-launch"))
        .expect("protected launch record");
    let meta = file.metadata().expect("launch record metadata");
    assert!(
        meta.is_file() && meta.uid() == 0 && meta.nlink() == 1 && meta.mode() & 0o7777 == 0o444
    );
    let mut record = String::new();
    file.take(1025)
        .read_to_string(&mut record)
        .expect("bounded launch record");
    assert!(record.len() <= 1024);
    let mut lines = record.lines();
    assert_eq!(
        lines.next(),
        Some("workflow-os/hosted-operator-rehearsal/v1")
    );
    assert_eq!(lines.next(), Some(executable_digest().as_str()));
    let trial = root
        .file_name()
        .and_then(|name| name.to_str())
        .expect("ASCII trial");
    assert_eq!(
        std::env::var("WORKFLOW_OS_SIGNER_TRIAL").as_deref(),
        Ok(trial)
    );
    assert_eq!(lines.next(), Some(trial));
    assert_eq!(
        lines.next(),
        Some("core=61021 signer=61022 reader=61025 receiver=61026 harness=61023 group=61020")
    );
    let launcher = root.parent().expect("protected root").join("launcher");
    let metadata = launcher.symlink_metadata().expect("launcher metadata");
    assert!(metadata.is_file() && metadata.uid() == 0 && metadata.mode() & 0o7777 == 0o444);
    assert_eq!(lines.next(), Some(file_digest(&launcher).as_str()));
    #[cfg(target_os = "linux")]
    assert_eq!(lines.next(), Some("linux-netns"));
    #[cfg(target_os = "macos")]
    {
        let profile = root.parent().expect("protected root").join("network.sb");
        let metadata = profile
            .symlink_metadata()
            .expect("sandbox profile metadata");
        assert!(metadata.is_file() && metadata.uid() == 0 && metadata.mode() & 0o7777 == 0o444);
        assert_eq!(lines.next(), Some(file_digest(&profile).as_str()));
    }
    if trial == "operator" {
        let deadline = std::env::var("WORKFLOW_OS_INSTALLATION_RECOVERY_READER_DEADLINE")
            .expect("reviewed operator reader deadline");
        assert!(Timestamp::parse_rfc3339(&deadline).is_ok());
        assert_eq!(lines.next(), Some(deadline.as_str()));
    }
    assert!(lines.next().is_none());
    SpecContentHash::from_text(&record)
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
