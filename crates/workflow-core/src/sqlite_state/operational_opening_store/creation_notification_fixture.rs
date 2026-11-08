//! Offline experiment only. The receiver and its clock are fixture inputs,
//! not a provider integration or an authority source.
#![allow(clippy::expect_used)]

use std::{borrow::Cow, num::NonZeroU64};

use ring::{digest, hmac};
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::provider_comment_observation::{
    encode_marked_comment, ExpectedComment, ProviderComment,
};
use crate::{SpecContentHash, Timestamp};

const RECEIVER_EPOCH: u8 = 1;
const MAX_NOTIFICATION_BYTES: usize = 128 * 1024;

pub(super) fn correlate(body: &str) -> String {
    let mut entropy = [0_u8; 32];
    getrandom::getrandom(&mut entropy).expect("private fixture correlation");
    format!(
        "{body}\n\nFixture correlation: {}",
        SpecContentHash::from_bytes(entropy)
    )
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Creation<'a> {
    action: &'a str,
    installation: NonZeroU64,
    repository: NonZeroU64,
    owner: &'a str,
    repository_name: &'a str,
    pull_request: u64,
    author: NonZeroU64,
    comment: NonZeroU64,
    #[serde(borrow)]
    body: Cow<'a, str>,
}

#[derive(Clone, Copy)]
struct Interval {
    earliest: Timestamp,
    latest: Timestamp,
}

impl Interval {
    const fn point(time: Timestamp) -> Self {
        Self {
            earliest: time,
            latest: time,
        }
    }
}

#[derive(Clone, Copy)]
struct Receipt {
    epoch: u8,
    interval: Interval,
    body_digest: digest::Digest,
}

impl Receipt {
    fn for_payload(self, payload: &[u8]) -> Self {
        Self {
            body_digest: digest::digest(&digest::SHA256, payload),
            ..self
        }
    }
}

struct Expected<'a> {
    comment: &'a ExpectedComment<'a>,
    installation: NonZeroU64,
    release: Interval,
}

fn verify(
    expected: &Expected<'_>,
    key: &hmac::Key,
    payload: &[u8],
    signature: &[u8],
    receipt: Option<Receipt>,
) -> Option<NonZeroU64> {
    let Receipt {
        epoch,
        interval,
        body_digest,
    } = receipt?;
    let release = expected.release;
    let comment = expected.comment;
    if epoch != RECEIVER_EPOCH
        || release.earliest < comment.not_before
        || release.latest < release.earliest
        || interval.earliest < release.latest
        || interval.latest < interval.earliest
        || interval.latest >= comment.not_after
        || payload.len() > MAX_NOTIFICATION_BYTES
        || body_digest.as_ref() != digest::digest(&digest::SHA256, payload).as_ref()
    {
        return None;
    }
    hmac::verify(key, payload, signature).ok()?;
    let event: Creation<'_> = serde_json::from_slice(payload).ok()?;
    let marked = encode_marked_comment(comment.body, comment.reservation_marker)?;
    (event.action == "created"
        && event.installation == expected.installation
        && event.repository == comment.repository_id
        && event.owner == comment.target.owner()
        && event.repository_name == comment.target.repository()
        && event.pull_request == comment.target.pull_request_number()
        && event.author == comment.author_id
        && event.body == marked
        && !comment
            .known_comment_id
            .is_some_and(|id| id != event.comment))
    .then_some(event.comment)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Evidence {
    Missing,
    Candidate(NonZeroU64),
    Conflict,
}

impl Evidence {
    fn observe(self, result: Option<NonZeroU64>) -> Self {
        match (self, result) {
            (Self::Missing, Some(id)) => Self::Candidate(id),
            (Self::Candidate(prior), Some(id)) if prior != id => Self::Conflict,
            (existing, _) => existing,
        }
    }
}

pub(super) fn exercise(
    expected: &ExpectedComment<'_>,
    observed: ProviderComment<'_>,
    installation: NonZeroU64,
    release: Timestamp,
    observed_at: Timestamp,
) {
    let key = hmac::Key::new(
        hmac::HMAC_SHA256,
        b"offline fixture only, not a provider key",
    );
    let expected = Expected {
        comment: expected,
        installation,
        release: Interval::point(release),
    };
    let event = Creation {
        action: "created",
        installation,
        repository: observed.repository_id,
        owner: observed.target.owner(),
        repository_name: observed.target.repository(),
        pull_request: observed.target.pull_request_number(),
        author: observed.author_id,
        comment: observed.comment_id,
        body: Cow::Borrowed(observed.raw_body),
    };
    let receipt = Receipt {
        epoch: RECEIVER_EPOCH,
        interval: Interval::point(observed_at),
        body_digest: digest::digest(
            &digest::SHA256,
            &serde_json::to_vec(&event).expect("fixture notification"),
        ),
    };
    let signed = |event: &Creation<'_>, receipt: Option<Receipt>| {
        let payload = serde_json::to_vec(event).expect("fixture notification");
        let signature = hmac::sign(&key, &payload);
        verify(
            &expected,
            &key,
            &payload,
            signature.as_ref(),
            receipt.map(|receipt| receipt.for_payload(&payload)),
        )
    };
    let result = signed(&event, Some(receipt));
    assert_eq!(result, Some(observed.comment_id));
    let candidate = Evidence::Missing.observe(result);
    assert_eq!(candidate.observe(result), candidate);
    let different = NonZeroU64::new(observed.comment_id.get() + 1).expect("conflicting id");
    let conflicting = Creation {
        comment: different,
        ..event.clone()
    };
    let payload = serde_json::to_vec(&conflicting).expect("conflicting notification");
    assert_eq!(
        verify(
            &expected,
            &key,
            &payload,
            hmac::sign(&key, &payload).as_ref(),
            Some(receipt)
        ),
        None
    );
    let conflict = candidate.observe(signed(&conflicting, Some(receipt)));
    assert_eq!(conflict, Evidence::Conflict);
    assert_eq!(conflict.observe(result), Evidence::Conflict);

    assert_untrusted_events(&event, &expected, &key, receipt, different);
    for receipt in [
        None,
        Some(Receipt {
            epoch: 2,
            ..receipt
        }),
        Some(Receipt {
            interval: Interval::point(expected.comment.not_after),
            ..receipt
        }),
        Some(Receipt {
            interval: Interval::point(Timestamp::from_offset_date_time(
                release.as_offset_date_time() - Duration::nanoseconds(1),
            )),
            ..receipt
        }),
    ] {
        assert_eq!(signed(&event, receipt), None);
    }
}

fn assert_untrusted_events(
    event: &Creation<'_>,
    expected: &Expected<'_>,
    key: &hmac::Key,
    receipt: Receipt,
    different: NonZeroU64,
) {
    for wrong in [
        Creation {
            action: "edited",
            ..event.clone()
        },
        Creation {
            installation: different,
            ..event.clone()
        },
        Creation {
            repository: different,
            ..event.clone()
        },
        Creation {
            pull_request: event.pull_request + 1,
            ..event.clone()
        },
        Creation {
            author: different,
            ..event.clone()
        },
        Creation {
            body: Cow::Borrowed("other correlation"),
            ..event.clone()
        },
    ] {
        let payload = serde_json::to_vec(&wrong).expect("signed mismatch");
        let signature = hmac::sign(key, &payload);
        assert_eq!(
            verify(
                expected,
                key,
                &payload,
                signature.as_ref(),
                Some(receipt.for_payload(&payload))
            ),
            None
        );
    }
    let mut tampered = serde_json::to_vec(event).expect("signed creation");
    let signature = hmac::sign(key, &tampered);
    tampered.push(b' ');
    assert_eq!(
        verify(
            expected,
            key,
            &tampered,
            signature.as_ref(),
            Some(receipt.for_payload(&tampered))
        ),
        None
    );
}
