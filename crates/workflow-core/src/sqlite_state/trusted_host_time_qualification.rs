//! Synthetic qualification model; never supplies a live authority clock.

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
    DomainMismatch,
    FutureSample,
    StaleSample,
    InvalidInterval,
    ExcessUncertainty,
    OutsideWindow,
}

fn qualify(sample: Sample, decision_tick: u64, policy: Policy) -> Result<(u64, u64), Denial> {
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
