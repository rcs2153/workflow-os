use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Component, Path};

use crate::{SpecContentHash, WorkflowOsError};

const MAX_EXECUTABLE_BYTES: u64 = 512 * 1024 * 1024;

fn denied(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(code, message)
}

fn unprotected() -> WorkflowOsError {
    denied(
        "protected_launch.file.unprotected",
        "launch input is not a protected regular file beneath protected directories",
    )
}

fn size_invalid() -> WorkflowOsError {
    denied(
        "protected_launch.file.size_invalid",
        "launch input exceeds its size limit",
    )
}

pub(crate) fn only_standard_descriptors() -> bool {
    only_launch_descriptors(&[])
}

fn only_launch_descriptors(allowed: &[u32]) -> bool {
    // Close the enumeration descriptor before checking for inherited ones.
    let descriptor_root = if cfg!(target_os = "linux") {
        "/proc/self/fd"
    } else {
        "/dev/fd"
    };
    let Ok(entries) = fs::read_dir(descriptor_root) else {
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
                    || allowed.contains(&fd)
                    || fs::symlink_metadata(path).is_err_and(|error| {
                        error.kind() == ErrorKind::NotFound
                            || error.raw_os_error() == Some(nix::libc::EBADF)
                    })
            })
    })
}

pub(crate) fn matches_identity(uid: u32, gid: u32) -> bool {
    uid != 0
        && gid != 0
        && nix::unistd::getuid().as_raw() == uid
        && nix::unistd::geteuid().as_raw() == uid
        && nix::unistd::getgid().as_raw() == gid
        && nix::unistd::getegid().as_raw() == gid
}

#[cfg(target_os = "linux")]
pub(crate) fn linux_isolation() -> bool {
    if !nix::unistd::getgroups().is_ok_and(|groups| groups.is_empty()) {
        return false;
    }
    let read = |path| {
        let mut text = String::new();
        File::open(path)
            .ok()?
            .take(65_537)
            .read_to_string(&mut text)
            .ok()?;
        (text.len() <= 65_536).then_some(text)
    };
    let Some(status) = read("/proc/self/status") else {
        return false;
    };
    let field = |name| {
        status
            .lines()
            .filter_map(|line| line.split_once(':'))
            .find_map(|(key, value)| (key == name).then_some(value.trim()))
    };
    let matches_slots = |name, expected| {
        field(name).is_some_and(|slots| {
            slots
                .split_whitespace()
                .map(|slot| slot.parse::<u32>().ok())
                .eq([Some(expected); 4])
        })
    };
    matches_slots("Uid", nix::unistd::getuid().as_raw())
        && matches_slots("Gid", nix::unistd::getgid().as_raw())
        && field("NoNewPrivs") == Some("1")
        && ["CapInh", "CapPrm", "CapEff", "CapBnd", "CapAmb"]
            .iter()
            .all(|name| field(*name) == Some("0000000000000000"))
        && read("/proc/net/dev").is_some_and(|interfaces| {
            interfaces
                .lines()
                .skip(2)
                .map(|line| line.split_once(':').map(|(name, _)| name.trim()))
                .eq([Some("lo")])
        })
}

fn protected_parents(path: &Path, rehearsal: bool) -> Result<(), WorkflowOsError> {
    if !path.is_absolute()
        || path.components().count() > 32
        || path
            .components()
            .any(|part| !matches!(part, Component::RootDir | Component::Normal(_)))
    {
        return Err(unprotected());
    }
    for parent in path.ancestors().skip(1) {
        let metadata = fs::symlink_metadata(parent).map_err(|_| unprotected())?;
        // Native fixtures use root-owned children of the sticky system temp directory.
        let fixture_temp = rehearsal
            && (parent == Path::new("/tmp") || parent == Path::new("/private/tmp"))
            && metadata.mode() & 0o7777 == 0o1777;
        if !metadata.is_dir()
            || metadata.uid() != 0
            || metadata.gid() != 0
            || (!fixture_temp && metadata.mode() & 0o7022 != 0)
        {
            return Err(unprotected());
        }
    }
    Ok(())
}

pub(crate) fn protected_file(
    path: &Path,
    mode: u32,
    rehearsal: bool,
) -> Result<File, WorkflowOsError> {
    protected_parents(path, rehearsal)?;
    let acceptable = |metadata: &fs::Metadata| {
        metadata.is_file()
            && metadata.uid() == 0
            && metadata.gid() == 0
            && metadata.nlink() == 1
            && metadata.mode() & 0o7777 == mode
    };
    let before = fs::symlink_metadata(path).map_err(|_| unprotected())?;
    if !acceptable(&before) {
        return Err(unprotected());
    }
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| unprotected())?;
    let opened = file.metadata().map_err(|_| unprotected())?;
    if !acceptable(&opened) || opened.dev() != before.dev() || opened.ino() != before.ino() {
        return Err(unprotected());
    }
    Ok(file)
}

pub(crate) fn bounded_bytes(file: File, limit: u64) -> Result<Vec<u8>, WorkflowOsError> {
    if file.metadata().map_err(|_| unprotected())?.len() > limit {
        return Err(size_invalid());
    }
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| unprotected())?;
    if bytes.len() as u64 > limit {
        return Err(size_invalid());
    }
    Ok(bytes)
}

pub(crate) fn executable_digest(file: File) -> Result<SpecContentHash, WorkflowOsError> {
    if file.metadata().map_err(|_| unprotected())?.len() > MAX_EXECUTABLE_BYTES {
        return Err(size_invalid());
    }
    let mut input = file.take(MAX_EXECUTABLE_BYTES + 1);
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    let mut buffer = [0; 8192];
    loop {
        let count = input.read(&mut buffer).map_err(|_| unprotected())?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    if input.limit() == 0 {
        return Err(size_invalid());
    }
    let digest = hasher.finish();
    let mut hex = String::with_capacity(64);
    for byte in digest.as_ref() {
        write!(&mut hex, "{byte:02x}").map_err(|_| {
            WorkflowOsError::new(
                crate::WorkflowOsErrorKind::Internal,
                "protected_launch.digest.encoding",
                "cannot encode executable digest",
            )
        })?;
    }
    SpecContentHash::new(hex)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn shared_executable_digest_matches_spec_content_hash() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/protected_launch/unix.rs");
        let file = File::open(path).expect("digest fixture");
        let expected = SpecContentHash::from_bytes(fs::read(path).expect("fixture bytes"));
        assert_eq!(executable_digest(file).expect("streaming digest"), expected);
    }
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(super) use linux::{inspect, inspect_peer, inspect_receiver_descriptors};
