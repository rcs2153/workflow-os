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
use crate::protected_launch::unix::{
    bounded_bytes, matches_identity, only_standard_descriptors, protected_file,
};
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
    let gid = if POLICY.harness_uids.contains(&uid) {
        uid
    } else {
        61_020
    };
    assert!(matches_identity(uid, gid));
    assert_platform_isolation(gid);
    let _ = launch_commitment();
}

#[test]
fn rehearsal_descriptor_guard_distinguishes_clean_and_inherited_launches() {
    let executable = std::env::current_exe().expect("fixture executable");
    let worker = format!("{}::rehearsal_descriptor_probe", module_path!());
    let (_, worker) = worker.split_once("::").expect("test path");
    for (injected, expected) in [("no", 0), ("yes", 76)] {
        let result = std::process::Command::new("/bin/bash")
            .args([
                "-c",
                r#"
                    exec 42</dev/null
                    for path in /dev/fd/*; do
                        fd=${path##*/}
                        case $fd in 0|1|2|""|*[!0-9]*) continue;; esac
                        eval "exec ${fd}<&-"
                    done
                    if [ "$1" = yes ]; then exec 42</dev/null; fi
                    shift
                    exec "$@"
                "#,
                "bash",
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
    let file = fs::File::open(std::env::current_exe().expect("fixture executable"))
        .expect("open fixture executable");
    crate::protected_launch::unix::executable_digest(file).expect("digest fixture executable")
}

fn file_digest(path: &Path) -> SpecContentHash {
    let file = protected_file(path, 0o444, true).expect("open protected fixture file");
    SpecContentHash::from_bytes(bounded_bytes(file, 131_072).expect("read bounded fixture file"))
}

pub(crate) fn launch_commitment() -> SpecContentHash {
    let root = fixture_root();
    let binary = std::env::current_exe().expect("running worker");
    assert_eq!(
        binary,
        root.parent().expect("protected root").join("worker")
    );
    let executable = protected_file(&binary, 0o555, true).expect("protected worker");
    let digest = crate::protected_launch::unix::executable_digest(executable)
        .expect("digest protected worker");
    let file = protected_file(&root.join("operator-launch"), 0o444, true)
        .expect("protected launch record");
    let record = String::from_utf8(bounded_bytes(file, 1024).expect("bounded launch record"))
        .expect("UTF-8 launch record");
    let mut lines = record.lines();
    assert_eq!(
        lines.next(),
        Some("workflow-os/hosted-operator-rehearsal/v1")
    );
    assert_eq!(lines.next(), Some(digest.as_str()));
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
    assert_eq!(lines.next(), Some(file_digest(&launcher).as_str()));
    #[cfg(target_os = "linux")]
    assert_eq!(lines.next(), Some("linux-netns"));
    #[cfg(target_os = "macos")]
    {
        let profile = root.parent().expect("protected root").join("network.sb");
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
    assert!(crate::protected_launch::unix::linux_isolation());
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
