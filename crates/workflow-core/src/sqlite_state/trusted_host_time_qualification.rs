//! Synthetic qualification model; never supplies a live authority clock.

use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Domain {
    source: &'static str,
    provenance: &'static str,
    epoch: &'static str,
}

#[derive(Clone, Copy)]
struct Sample {
    domain: Domain,
    captured_tick: u64,
    lower_utc_ms: u64,
    upper_utc_ms: u64,
}

#[derive(Clone, Copy)]
struct Policy {
    domain: Domain,
    max_age_ms: u64,
    max_width_ms: u64,
    // Synthetic bound on projection error for any accepted sample age.
    projection_error_ms: u64,
    not_before_ms: u64,
    expires_ms: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum Denial {
    ObservationUnavailable,
    InvalidObservation,
    EpochUnavailable,
    EpochMismatch,
    DomainMismatch,
    FutureSample,
    StaleSample,
    InvalidInterval,
    ExcessUncertainty,
    OutsideWindow,
}

const MAX_OBSERVATION_BYTES: u16 = 256;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FixtureObservation {
    source: String,
    provenance: String,
    epoch: String,
    captured_tick: u64,
    lower_utc_ms: u64,
    upper_utc_ms: u64,
}

fn assess_read_only(
    input: impl Read,
    decision_tick: u64,
    current_epoch: impl FnOnce() -> Option<String>,
    policy: Policy,
) -> Result<(u64, u64), Denial> {
    let mut bytes = Vec::new();
    input
        .take(u64::from(MAX_OBSERVATION_BYTES) + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Denial::ObservationUnavailable)?;
    if bytes.len() > usize::from(MAX_OBSERVATION_BYTES) {
        return Err(Denial::InvalidObservation);
    }
    let frame: FixtureObservation =
        serde_json::from_slice(&bytes).map_err(|_| Denial::InvalidObservation)?;
    let epoch = current_epoch();
    if frame.source != policy.domain.source
        || frame.provenance != policy.domain.provenance
        || frame.epoch != policy.domain.epoch
    {
        return Err(Denial::DomainMismatch);
    }
    qualify_at_epoch(
        Sample {
            domain: policy.domain,
            captured_tick: frame.captured_tick,
            lower_utc_ms: frame.lower_utc_ms,
            upper_utc_ms: frame.upper_utc_ms,
        },
        decision_tick,
        policy,
        epoch.as_deref(),
    )
}

fn qualify(sample: Sample, decision_tick: u64, policy: Policy) -> Result<(u64, u64), Denial> {
    qualify_at_epoch(sample, decision_tick, policy, Some(DOMAIN.epoch))
}

fn qualify_at_epoch(
    sample: Sample,
    decision_tick: u64,
    policy: Policy,
    external_epoch: Option<&str>,
) -> Result<(u64, u64), Denial> {
    let epoch = external_epoch
        .filter(|epoch| !epoch.is_empty())
        .ok_or(Denial::EpochUnavailable)?;
    if policy.domain.epoch != epoch {
        return Err(Denial::EpochMismatch);
    }
    if sample.domain != policy.domain {
        return Err(Denial::DomainMismatch);
    }
    let age = decision_tick
        .checked_sub(sample.captured_tick)
        .ok_or(Denial::FutureSample)?;
    if age > policy.max_age_ms {
        return Err(Denial::StaleSample);
    }
    if sample.lower_utc_ms >= sample.upper_utc_ms || policy.not_before_ms >= policy.expires_ms {
        return Err(Denial::InvalidInterval);
    }
    let lower = sample
        .lower_utc_ms
        .checked_add(age)
        .and_then(|value| value.checked_sub(policy.projection_error_ms))
        .ok_or(Denial::InvalidInterval)?;
    let upper = sample
        .upper_utc_ms
        .checked_add(age)
        .and_then(|value| value.checked_add(policy.projection_error_ms))
        .ok_or(Denial::InvalidInterval)?;
    if upper - lower > policy.max_width_ms {
        return Err(Denial::ExcessUncertainty);
    }
    if lower < policy.not_before_ms || upper >= policy.expires_ms {
        return Err(Denial::OutsideWindow);
    }
    Ok((lower, upper))
}

const DOMAIN: Domain = Domain {
    source: "fixture/source",
    provenance: "fixture/provenance",
    epoch: "fixture/epoch",
};

const SAMPLE: Sample = Sample {
    domain: DOMAIN,
    captured_tick: 30,
    lower_utc_ms: 100,
    upper_utc_ms: 104,
};

const POLICY: Policy = Policy {
    domain: DOMAIN,
    max_age_ms: 10,
    max_width_ms: 8,
    projection_error_ms: 2,
    not_before_ms: 100,
    expires_ms: 120,
};

const OBSERVATION: &[u8] = br#"{"source":"fixture/source","provenance":"fixture/provenance","epoch":"fixture/epoch","captured_tick":30,"lower_utc_ms":100,"upper_utc_ms":104}"#;

#[cfg(target_os = "linux")]
#[test]
#[ignore = "invoked by the parent clock bracket test"]
fn linux_boottime_child() -> Result<(), nix::errno::Errno> {
    let tick = nix::time::clock_gettime(nix::time::ClockId::CLOCK_BOOTTIME)?;
    println!("boottime={}:{}", tick.tv_sec(), tick.tv_nsec());
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
#[allow(clippy::expect_used)]
fn linux_boottime_is_comparable_across_processes() -> Result<(), Box<dyn std::error::Error>> {
    use nix::time::{clock_gettime, ClockId};

    let mut command = std::process::Command::new(std::env::current_exe()?);
    command.args([
        "--ignored",
        "--exact",
        concat!(module_path!(), "::linux_boottime_child")
            .split_once("::")
            .expect("crate-qualified test module")
            .1,
        "--nocapture",
    ]);
    let before = clock_gettime(ClockId::CLOCK_BOOTTIME)?;
    let output = command.output()?;
    let after = clock_gettime(ClockId::CLOCK_BOOTTIME)?;
    let stdout = std::str::from_utf8(&output.stdout)?;
    assert!(output.status.success(), "{stdout}");
    let mut samples = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("boottime="));
    let (seconds, nanos) = samples
        .next()
        .expect("child clock sample")
        .split_once(':')
        .expect("seconds and nanoseconds");
    assert!(samples.next().is_none());
    let sample = (seconds.parse()?, nanos.parse()?);
    assert!((0..1_000_000_000).contains(&sample.1));
    assert!((before.tv_sec(), before.tv_nsec()) <= sample);
    assert!(sample <= (after.tv_sec(), after.tv_nsec()));
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires a root-owned observation and an unprivileged Linux reader"]
#[allow(clippy::expect_used)]
fn protected_observation_qualifies_only_inside_its_epoch_and_window() {
    use crate::protected_launch::unix::{bounded_bytes, protected_file};

    assert!(!nix::unistd::geteuid().is_root());
    let root = std::path::PathBuf::from(
        std::env::var_os("WORKFLOW_OS_CLOCK_CUSTODY_FIXTURE").expect("fixture directory"),
    );
    let path = root.join("observation.json");
    let read_generation = |name| {
        let file = protected_file(&root.join(name), 0o444, true).ok()?;
        let value = String::from_utf8(bounded_bytes(file, 64).ok()?).ok()?;
        (!value.is_empty()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-')))
        .then_some(value)
    };

    for (tick, generation, policy, expected) in [
        (40, "generation", POLICY, Ok((108, 116))),
        (
            40,
            "missing-generation",
            POLICY,
            Err(Denial::EpochUnavailable),
        ),
        (40, "changed-generation", POLICY, Err(Denial::EpochMismatch)),
        (
            40,
            "malformed-generation",
            POLICY,
            Err(Denial::EpochUnavailable),
        ),
        (
            40,
            "writable-generation",
            POLICY,
            Err(Denial::EpochUnavailable),
        ),
        (41, "generation", POLICY, Err(Denial::StaleSample)),
        (
            40,
            "generation",
            Policy {
                expires_ms: 116,
                ..POLICY
            },
            Err(Denial::OutsideWindow),
        ),
    ] {
        let file = protected_file(&path, 0o444, true).expect("protected observation");
        let bytes =
            bounded_bytes(file, u64::from(MAX_OBSERVATION_BYTES)).expect("bounded observation");
        assert_eq!(
            assess_read_only(
                bytes.as_slice(),
                tick,
                || read_generation(generation),
                policy
            ),
            expected,
            "{generation} at {tick}",
        );
    }
}

#[test]
fn reads_a_fixture_without_changing_its_bytes() -> std::io::Result<()> {
    use std::io::Write;

    struct RemoveFixture(std::path::PathBuf);
    impl Drop for RemoveFixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    let path = std::env::temp_dir().join(format!(
        "workflow-os-time-observation-{}-{}.json",
        std::process::id(),
        crate::Timestamp::now_utc()
            .as_offset_date_time()
            .unix_timestamp_nanos()
    ));
    let mut writer = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    let fixture = RemoveFixture(path);
    writer.write_all(OBSERVATION)?;
    drop(writer);
    let reader = std::fs::File::open(&fixture.0)?;
    assert_eq!(
        assess_read_only(reader, 40, || Some(DOMAIN.epoch.to_owned()), POLICY),
        Ok((108, 116))
    );
    assert_eq!(std::fs::read(&fixture.0)?, OBSERVATION);
    Ok(())
}

#[test]
fn rejects_incomplete_oversized_and_trailing_observation_data() {
    let mut padded = OBSERVATION.to_vec();
    padded.resize(usize::from(MAX_OBSERVATION_BYTES) + 1, b' ');
    let mut trailing = OBSERVATION.to_vec();
    trailing.extend_from_slice(b"{}");
    for input in [&OBSERVATION[..OBSERVATION.len() - 1], &padded, &trailing] {
        let mut requested_epoch = false;
        assert_eq!(
            assess_read_only(
                input,
                40,
                || {
                    requested_epoch = true;
                    None
                },
                POLICY
            ),
            Err(Denial::InvalidObservation)
        );
        assert!(!requested_epoch);
    }
}

#[test]
fn refuses_an_epoch_that_changes_after_observation_capture() {
    for (epoch, expected) in [
        (None, Denial::EpochUnavailable),
        (Some("fixture/post-restore"), Denial::EpochMismatch),
    ] {
        assert_eq!(
            assess_read_only(OBSERVATION, 40, || epoch.map(str::to_owned), POLICY),
            Err(expected)
        );
    }
}

#[test]
fn refuses_joint_rollback_even_when_sample_and_policy_agree() {
    assert_eq!(qualify(SAMPLE, 40, POLICY), Ok((108, 116)));
    assert_eq!(
        qualify_at_epoch(SAMPLE, 40, POLICY, Some("fixture/post-restore")),
        Err(Denial::EpochMismatch)
    );
}

#[test]
fn refuses_missing_external_epoch_even_when_sample_and_policy_agree() {
    for external_epoch in [None, Some("")] {
        assert_eq!(
            qualify_at_epoch(SAMPLE, 40, POLICY, external_epoch),
            Err(Denial::EpochUnavailable)
        );
    }
}

#[test]
fn projects_the_full_interval_at_the_maximum_sample_age() {
    assert_eq!(qualify(SAMPLE, 40, POLICY), Ok((108, 116)));
}

#[test]
fn refuses_future_samples_and_age_advanced_by_suspend() {
    assert_eq!(qualify(SAMPLE, 29, POLICY), Err(Denial::FutureSample));
    assert_eq!(qualify(SAMPLE, 41, POLICY), Err(Denial::StaleSample));
}

#[test]
fn refuses_source_provenance_and_epoch_substitutions() {
    for domain in [
        Domain {
            source: "other",
            ..DOMAIN
        },
        Domain {
            provenance: "other",
            ..DOMAIN
        },
        Domain {
            epoch: "other",
            ..DOMAIN
        },
    ] {
        assert_eq!(
            qualify(Sample { domain, ..SAMPLE }, 40, POLICY),
            Err(Denial::DomainMismatch)
        );
    }
}

#[test]
fn checks_inclusive_start_and_exclusive_expiry_against_both_bounds() {
    for (not_before_ms, expires_ms, result) in [
        (108, 117, Ok((108, 116))),
        (109, 117, Err(Denial::OutsideWindow)),
        (100, 116, Err(Denial::OutsideWindow)),
        (100, 112, Err(Denial::OutsideWindow)),
        (100, 107, Err(Denial::OutsideWindow)),
    ] {
        assert_eq!(
            qualify(
                SAMPLE,
                40,
                Policy {
                    not_before_ms,
                    expires_ms,
                    ..POLICY
                }
            ),
            result
        );
    }
}

#[test]
fn refuses_excess_uncertainty_even_inside_the_window() {
    assert_eq!(
        qualify(
            SAMPLE,
            40,
            Policy {
                max_width_ms: 7,
                ..POLICY
            }
        ),
        Err(Denial::ExcessUncertainty)
    );
}

#[test]
fn refuses_point_reversed_and_unrepresentable_intervals() {
    for (lower_utc_ms, upper_utc_ms) in [(100, 100), (104, 100), (u64::MAX - 2, u64::MAX)] {
        assert_eq!(
            qualify(
                Sample {
                    lower_utc_ms,
                    upper_utc_ms,
                    ..SAMPLE
                },
                40,
                POLICY
            ),
            Err(Denial::InvalidInterval)
        );
    }
}
