//! Offline verification of a current GitHub comment observation. This is
//! neither acceptance proof nor a dispatch authorization.
#![allow(dead_code)] // No trusted production observer is wired yet.

use serde::Deserialize;
use std::num::NonZeroU64;
use time::Duration;

use crate::live_sandbox_continuation_authority::LiveSandboxProviderBinding;
use crate::{GitHubPullRequestCommentTarget, Timestamp};

const MAX_COMMENTS: usize = 128;
const MAX_PAGES: usize = 8;
pub(crate) const MAX_RAW_BODY_BYTES: usize = 64 * 1024;
pub(crate) const MAX_CREATION_RESPONSE_BYTES: usize = 512 * 1024;
const MARKER_PREFIX: &str = "<!-- workflow-os/live-comment/";
pub(crate) const MARKER_NAMESPACE: &str = "workflow-os/live-comment/";
const SUFFIX_START: &str = "\n\n<!-- ";
const SUFFIX_END: &str = " -->";
const MARKER_DIGEST_BYTES: usize = 64;

pub(crate) fn can_encode_marked_comment(body: &str) -> bool {
    let suffix_bytes =
        SUFFIX_START.len() + MARKER_NAMESPACE.len() + MARKER_DIGEST_BYTES + SUFFIX_END.len();
    !body.is_empty()
        && !body.contains(MARKER_PREFIX)
        && body.len() <= MAX_RAW_BODY_BYTES - suffix_bytes
}

fn marked_comment_suffix(body: &str, marker: &str) -> Option<String> {
    let digest = marker.strip_prefix(MARKER_NAMESPACE)?;
    if digest.len() != MARKER_DIGEST_BYTES
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || !can_encode_marked_comment(body)
    {
        return None;
    }
    Some(format!("{SUFFIX_START}{marker}{SUFFIX_END}"))
}

pub(crate) fn encode_marked_comment(body: &str, marker: &str) -> Option<String> {
    let suffix = marked_comment_suffix(body, marker)?;
    Some(format!("{body}{suffix}"))
}

/// Bounded input from the creation exchange, not a subsequent GET. The
/// repository ID must be resolved independently by the trusted adapter.
pub(crate) struct CreatedCommentResponse {
    pub(crate) status: u16,
    pub(crate) repository_id: NonZeroU64,
    pub(crate) json: Vec<u8>,
}

#[derive(Deserialize)]
struct GitHubCreatedComment {
    id: NonZeroU64,
    url: String,
    issue_url: String,
    user: GitHubCreatedCommentAuthor,
    body: String,
}

#[derive(Deserialize)]
struct GitHubCreatedCommentAuthor {
    id: NonZeroU64,
}

pub(crate) fn verify_created_comment_response(
    response: Option<&CreatedCommentResponse>,
    target: &GitHubPullRequestCommentTarget,
    provider: &LiveSandboxProviderBinding,
    sent_body: &str,
) -> Option<NonZeroU64> {
    let response = response?;
    if response.status != 201
        || response.repository_id != provider.repository_id
        || response.json.len() > MAX_CREATION_RESPONSE_BYTES
        || sent_body.len() > MAX_RAW_BODY_BYTES
        || target.validate().is_err()
    {
        return None;
    }
    let comment: GitHubCreatedComment = serde_json::from_slice(&response.json).ok()?;
    let issues = format!(
        "https://api.github.com/repos/{}/{}/issues",
        target.owner(),
        target.repository()
    );
    (comment.issue_url == format!("{issues}/{}", target.pull_request_number())
        && comment.url == format!("{issues}/comments/{}", comment.id)
        && comment.user.id == provider.author_id
        && comment.body == sent_body)
        .then_some(comment.id)
}

pub(crate) struct ExpectedComment<'a> {
    pub(crate) target: &'a GitHubPullRequestCommentTarget,
    pub(crate) repository_id: NonZeroU64,
    pub(crate) author_id: NonZeroU64,
    pub(crate) known_comment_id: Option<NonZeroU64>,
    pub(crate) body: &'a str,
    pub(crate) reservation_marker: &'a str,
    pub(crate) not_before: Timestamp,
    pub(crate) not_after: Timestamp,
}

#[derive(Clone, Copy)]
pub(crate) struct ProviderComment<'a> {
    pub(crate) target: &'a GitHubPullRequestCommentTarget,
    pub(crate) repository_id: NonZeroU64,
    pub(crate) author_id: NonZeroU64,
    pub(crate) comment_id: NonZeroU64,
    pub(crate) raw_body: &'a str,
    pub(crate) created_at: Timestamp,
    pub(crate) updated_at: Timestamp,
}

/// Inclusive corrections from reported provider time to the trusted local
/// timeline. They must cover both timestamp precision and clock uncertainty.
#[derive(Clone, Copy)]
pub(crate) struct ProviderTimestampBounds {
    pub(crate) earliest_correction: Duration,
    pub(crate) latest_correction: Duration,
}

impl ProviderTimestampBounds {
    fn proves_creation(
        self,
        reported: Timestamp,
        expected: &ExpectedComment<'_>,
        observed_at: Timestamp,
    ) -> bool {
        let reported = reported.as_offset_date_time();
        reported
            .checked_add(self.earliest_correction)
            .zip(reported.checked_add(self.latest_correction))
            .is_some_and(|(earliest, latest)| {
                earliest >= expected.not_before.as_offset_date_time()
                    && latest < expected.not_after.as_offset_date_time()
                    && latest <= observed_at.as_offset_date_time()
            })
    }
}

/// Only an operator-controlled adapter may classify enumeration as complete.
/// A model-supplied page count, marker, or digest is not that evidence.
#[derive(Clone, Copy)]
pub(crate) enum CommentEnumeration<'a> {
    Complete {
        observed_at: Timestamp,
        timestamp_bounds: Option<ProviderTimestampBounds>,
        pages: &'a [CommentPage<'a>],
    },
    Incomplete,
    Unauthorized,
    Unavailable,
}

/// Adapter-normalized cursor identities, not URLs or permission to fetch.
#[derive(Clone, Copy)]
pub(crate) struct CommentPage<'a> {
    pub(crate) request_cursor: Option<NonZeroU64>,
    pub(crate) next_cursor: Option<NonZeroU64>,
    pub(crate) comments: &'a [ProviderComment<'a>],
}

impl<'a> CommentPage<'a> {
    pub(crate) const fn single(comments: &'a [ProviderComment<'a>]) -> Self {
        Self {
            request_cursor: None,
            next_cursor: None,
            comments,
        }
    }
}

fn scan_is_complete(pages: &[CommentPage<'_>]) -> bool {
    if pages.is_empty() || pages.len() > MAX_PAGES {
        return false;
    }
    let mut expected_cursor = None;
    let mut remaining = MAX_COMMENTS;
    for (index, page) in pages.iter().enumerate() {
        let Some(rest) = remaining.checked_sub(page.comments.len()) else {
            return false;
        };
        if page.request_cursor != expected_cursor
            || page.next_cursor.is_none() != (index + 1 == pages.len())
            || pages[..index]
                .iter()
                .any(|prior| prior.request_cursor == page.request_cursor)
        {
            return false;
        }
        expected_cursor = page.next_cursor;
        remaining = rest;
    }
    true
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CurrentCommentPosture {
    ExactCurrentMatch,
    Absent,
    Ambiguous,
    Unresolved,
}

pub(crate) fn verify_current_comment(
    expected: &ExpectedComment<'_>,
    enumeration: CommentEnumeration<'_>,
) -> CurrentCommentPosture {
    use CurrentCommentPosture::{Absent, Ambiguous, ExactCurrentMatch, Unresolved};

    let CommentEnumeration::Complete {
        observed_at,
        timestamp_bounds: Some(timestamp_bounds),
        pages,
    } = enumeration
    else {
        return Unresolved;
    };
    let Some(suffix) = marked_comment_suffix(expected.body, expected.reservation_marker) else {
        return Unresolved;
    };
    if expected.target.validate().is_err()
        || expected.not_before >= expected.not_after
        || observed_at < expected.not_before
        || timestamp_bounds.earliest_correction > timestamp_bounds.latest_correction
        || !scan_is_complete(pages)
    {
        return Unresolved;
    }
    let comments = || pages.iter().flat_map(|page| page.comments);
    let mut candidates = comments().filter(|comment| {
        comment.raw_body.contains(expected.reservation_marker)
            || expected.known_comment_id == Some(comment.comment_id)
    });
    // Refuse an oversized response rather than reasoning from a truncated scan.
    if comments().any(|comment| comment.raw_body.len() > MAX_RAW_BODY_BYTES) {
        return Unresolved;
    }
    let Some(comment) = candidates.next() else {
        return Absent;
    };
    let mut ambiguous = false;
    for candidate in std::iter::once(comment).chain(candidates) {
        // A later page can repeat the same ID after an edit removes the marker.
        if comments()
            .filter(|row| row.comment_id == candidate.comment_id)
            .take(2)
            .count()
            == 2
        {
            return Unresolved;
        }
        ambiguous |= candidate.comment_id != comment.comment_id;
    }
    if ambiguous {
        return Ambiguous;
    }
    if comment.target != expected.target
        || comment.repository_id != expected.repository_id
        || comment.author_id != expected.author_id
        || expected
            .known_comment_id
            .is_some_and(|id| id != comment.comment_id)
        || !timestamp_bounds.proves_creation(comment.created_at, expected, observed_at)
        || comment.updated_at != comment.created_at
        || comment.raw_body.strip_suffix(&suffix) != Some(expected.body)
    {
        return Unresolved;
    }
    // Matching timestamps can hide edits at the provider's clock resolution.
    ExactCurrentMatch
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    const MARKER: &str =
        "workflow-os/live-comment/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const EXACT_TIME: Option<ProviderTimestampBounds> = Some(ProviderTimestampBounds {
        earliest_correction: Duration::ZERO,
        latest_correction: Duration::ZERO,
    });

    fn id(value: u64) -> NonZeroU64 {
        NonZeroU64::new(value).expect("nonzero provider id")
    }

    fn time(seconds: i64) -> Timestamp {
        Timestamp::from_offset_date_time(
            time::OffsetDateTime::from_unix_timestamp(seconds).expect("time"),
        )
    }

    fn expected(target: &GitHubPullRequestCommentTarget) -> ExpectedComment<'_> {
        ExpectedComment {
            target,
            repository_id: id(1),
            author_id: id(2),
            known_comment_id: Some(id(3)),
            body: "exact body",
            reservation_marker: MARKER,
            not_before: time(10),
            not_after: time(20),
        }
    }

    fn comment<'a>(
        target: &'a GitHubPullRequestCommentTarget,
        raw_body: &'a str,
    ) -> ProviderComment<'a> {
        ProviderComment {
            target,
            repository_id: id(1),
            author_id: id(2),
            comment_id: id(3),
            raw_body,
            created_at: time(11),
            updated_at: time(11),
        }
    }

    #[test]
    fn only_exact_created_response_verifies() {
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let body = encode_marked_comment("exact body", MARKER).expect("marked body");
        let provider = LiveSandboxProviderBinding {
            repository_id: id(1),
            author_id: id(2),
            adapter_commitment: crate::SpecContentHash::from_text("adapter"),
        };
        let payload = serde_json::json!({
            "id": 3,
            "issue_url": "https://api.github.com/repos/example/sandbox/issues/7",
            "url": "https://api.github.com/repos/example/sandbox/issues/comments/3",
            "user": { "id": 2, "login": "ignored" },
            "body": body,
        });
        let encode = |value: &serde_json::Value| serde_json::to_vec(value).expect("response JSON");
        let exact = || CreatedCommentResponse {
            status: 201,
            repository_id: id(1),
            json: encode(&payload),
        };
        let verify = |response: Option<&CreatedCommentResponse>| {
            verify_created_comment_response(response, &target, &provider, &body)
        };
        assert_eq!(verify(Some(&exact())), Some(id(3)));
        assert_eq!(verify(None), None);
        for invalid in [
            CreatedCommentResponse {
                status: 200,
                ..exact()
            },
            CreatedCommentResponse {
                status: 403,
                ..exact()
            },
            CreatedCommentResponse {
                repository_id: id(4),
                ..exact()
            },
        ] {
            assert_eq!(verify(Some(&invalid)), None);
        }
        for (field, substitute) in [
            (
                "issue_url",
                serde_json::json!("https://api.github.com/repos/example/sandbox/issues/8"),
            ),
            (
                "url",
                serde_json::json!("https://api.github.com/repos/example/sandbox/issues/comments/4"),
            ),
            (
                "url",
                serde_json::json!("https://example.org/repos/example/sandbox/issues/comments/3"),
            ),
            ("user", serde_json::json!({ "id": 4 })),
            ("user", serde_json::Value::Null),
            ("body", serde_json::json!(format!("{body}\n"))),
            ("id", serde_json::json!(0)),
        ] {
            let mut changed = payload.clone();
            changed[field] = substitute;
            assert_eq!(
                verify(Some(&CreatedCommentResponse {
                    json: encode(&changed),
                    ..exact()
                })),
                None
            );
        }
        let mut duplicated = exact();
        assert_eq!(duplicated.json.pop(), Some(b'}'));
        duplicated.json.extend_from_slice(br#", "id": 3}"#);
        assert_eq!(verify(Some(&duplicated)), None);
        let mut oversized = exact();
        oversized.json.resize(MAX_CREATION_RESPONSE_BYTES + 1, b' ');
        assert_eq!(verify(Some(&oversized)), None);
        let mut malformed = exact();
        malformed.json.truncate(12);
        assert_eq!(verify(Some(&malformed)), None);
    }

    #[test]
    fn marked_encoding_is_exact_and_bounded_in_bytes() {
        let body = "first\r\nsecond\n";
        let exact = encode_marked_comment(body, MARKER).expect("valid encoding");
        assert_eq!(exact, format!("first\r\nsecond\n\n\n<!-- {MARKER} -->"));
        let max = "x".repeat(MAX_RAW_BODY_BYTES - (exact.len() - body.len()));
        assert_eq!(
            encode_marked_comment(&max, MARKER).expect("fits").len(),
            MAX_RAW_BODY_BYTES
        );
        assert!(encode_marked_comment(&format!("{max}x"), MARKER).is_none());
        assert!(encode_marked_comment("", MARKER).is_none());
        assert!(
            encode_marked_comment("<!-- workflow-os/live-comment/injected -->", MARKER).is_none()
        );
        assert!(encode_marked_comment(body, &MARKER.to_ascii_uppercase()).is_none());
        assert!(encode_marked_comment(body, &MARKER[..MARKER.len() - 1]).is_none());
    }

    #[test]
    fn verifies_only_the_exact_current_record_and_complete_bounded_enumeration() {
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let other_target =
            GitHubPullRequestCommentTarget::new("example", "sandbox", 8).expect("other target");
        let expected = expected(&target);
        let exact_body = format!("exact body\n\n<!-- {MARKER} -->");
        let exact = comment(&target, &exact_body);
        let inspect = |comments: &[ProviderComment<'_>]| {
            verify_current_comment(
                &expected,
                CommentEnumeration::Complete {
                    observed_at: time(30),
                    timestamp_bounds: EXACT_TIME,
                    pages: &[CommentPage::single(comments)],
                },
            )
        };
        assert_eq!(inspect(&[]), CurrentCommentPosture::Absent);
        assert_eq!(
            inspect(std::slice::from_ref(&exact)),
            CurrentCommentPosture::ExactCurrentMatch
        );
        let changed_body = format!("changed body\n\n<!-- {MARKER} -->");
        let copied_marker = format!("<!-- {MARKER} -->\n{exact_body}");
        let noncanonical_body = format!("exact body\r\n\r\n<!-- {MARKER} -->");
        let oversized_body = "x".repeat(MAX_RAW_BODY_BYTES + 1);
        for mismatch in [
            ProviderComment {
                author_id: id(4),
                ..exact
            },
            ProviderComment {
                repository_id: id(9),
                ..exact
            },
            ProviderComment {
                target: &other_target,
                ..exact
            },
            ProviderComment {
                comment_id: id(4),
                ..exact
            },
            ProviderComment {
                raw_body: &changed_body,
                ..exact
            },
            ProviderComment {
                raw_body: &copied_marker,
                ..exact
            },
            ProviderComment {
                raw_body: &noncanonical_body,
                ..exact
            },
            ProviderComment {
                raw_body: &oversized_body,
                ..exact
            },
            ProviderComment {
                raw_body: "marker deleted",
                ..exact
            },
            ProviderComment {
                updated_at: time(12),
                ..exact
            },
            ProviderComment {
                created_at: time(9),
                updated_at: time(9),
                ..exact
            },
            ProviderComment {
                created_at: time(21),
                updated_at: time(21),
                ..exact
            },
        ] {
            assert_eq!(inspect(&[mismatch]), CurrentCommentPosture::Unresolved);
        }
        assert_eq!(
            inspect(&[
                ProviderComment {
                    comment_id: id(4),
                    ..exact
                },
                exact
            ]),
            CurrentCommentPosture::Ambiguous
        );
        for enumeration in [
            CommentEnumeration::Incomplete,
            CommentEnumeration::Unauthorized,
            CommentEnumeration::Unavailable,
        ] {
            assert_eq!(
                verify_current_comment(&expected, enumeration),
                CurrentCommentPosture::Unresolved
            );
        }
    }

    #[test]
    fn rejects_repeated_candidate_identity_across_the_full_enumeration() {
        use CurrentCommentPosture::{Ambiguous, ExactCurrentMatch, Unresolved};

        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let expected = ExpectedComment {
            known_comment_id: None,
            ..expected(&target)
        };
        let raw = format!("exact body\n\n<!-- {MARKER} -->");
        let marked = comment(&target, &raw);
        let edited = ProviderComment {
            raw_body: "edited without marker",
            ..marked
        };
        let second = ProviderComment {
            comment_id: id(4),
            ..marked
        };
        let unrelated = ProviderComment {
            comment_id: id(5),
            ..edited
        };
        let inspect = |comments: &[ProviderComment<'_>]| {
            verify_current_comment(
                &expected,
                CommentEnumeration::Complete {
                    observed_at: time(30),
                    timestamp_bounds: EXACT_TIME,
                    pages: &[CommentPage::single(comments)],
                },
            )
        };

        assert_eq!(inspect(&[marked, edited]), Unresolved);
        assert_eq!(inspect(&[edited, marked]), Unresolved);
        assert_eq!(inspect(&[marked, marked]), Unresolved);
        assert_eq!(inspect(&[second, marked, edited]), Unresolved);
        assert_eq!(inspect(&[marked, second]), Ambiguous);
        assert_eq!(inspect(&[unrelated, marked]), ExactCurrentMatch);
    }

    #[test]
    fn requires_the_entire_creation_interval_to_fit_inside_the_window() {
        use CurrentCommentPosture::{ExactCurrentMatch, Unresolved};

        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let expected = expected(&target);
        let raw = format!("exact body\n\n<!-- {MARKER} -->");
        let marked = comment(&target, &raw);
        let inspect = |earliest_correction, latest_correction, observed_at| {
            verify_current_comment(
                &expected,
                CommentEnumeration::Complete {
                    observed_at,
                    timestamp_bounds: Some(ProviderTimestampBounds {
                        earliest_correction,
                        latest_correction,
                    }),
                    pages: &[CommentPage::single(std::slice::from_ref(&marked))],
                },
            )
        };
        let seconds = Duration::seconds;

        assert_eq!(
            inspect(seconds(-1), seconds(2), time(13)),
            ExactCurrentMatch
        );
        assert_eq!(inspect(seconds(-2), seconds(2), time(30)), Unresolved);
        assert_eq!(inspect(seconds(-1), seconds(9), time(30)), Unresolved);
        assert_eq!(inspect(seconds(-1), seconds(2), time(12)), Unresolved);
        assert_eq!(inspect(seconds(2), seconds(1), time(30)), Unresolved);
        assert_eq!(inspect(Duration::MIN, seconds(0), time(30)), Unresolved);
        assert_eq!(inspect(seconds(0), Duration::MAX, time(30)), Unresolved);
        for comments in [&[][..], std::slice::from_ref(&marked)] {
            assert_eq!(
                verify_current_comment(
                    &expected,
                    CommentEnumeration::Complete {
                        observed_at: time(30),
                        timestamp_bounds: None,
                        pages: &[CommentPage::single(comments)],
                    },
                ),
                Unresolved
            );
        }
    }

    #[test]
    fn only_a_complete_bounded_page_chain_can_classify_observation() {
        use CurrentCommentPosture::{Absent, Ambiguous, ExactCurrentMatch, Unresolved};

        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let expected = expected(&target);
        let raw = format!("exact body\n\n<!-- {MARKER} -->");
        let marked = comment(&target, &raw);
        let other = ProviderComment {
            comment_id: id(4),
            ..marked
        };
        let first = CommentPage {
            next_cursor: Some(id(1)),
            ..CommentPage::single(std::slice::from_ref(&marked))
        };
        let last = CommentPage {
            request_cursor: Some(id(1)),
            ..CommentPage::single(&[])
        };
        let inspect = |pages: &[CommentPage<'_>]| {
            verify_current_comment(
                &expected,
                CommentEnumeration::Complete {
                    observed_at: time(30),
                    timestamp_bounds: EXACT_TIME,
                    pages,
                },
            )
        };
        assert_eq!(inspect(&[CommentPage::single(&[])]), Absent);
        assert_eq!(inspect(&[first, last]), ExactCurrentMatch);
        assert_eq!(
            inspect(&[
                first,
                CommentPage {
                    comments: std::slice::from_ref(&other),
                    ..last
                }
            ]),
            Ambiguous
        );
        assert_eq!(inspect(&[]), Unresolved);
        assert_eq!(inspect(&[first]), Unresolved);
        assert_eq!(inspect(&[last]), Unresolved);
        assert_eq!(
            inspect(&[
                first,
                CommentPage {
                    request_cursor: Some(id(2)),
                    ..last
                }
            ]),
            Unresolved
        );
        assert_eq!(
            inspect(&[CommentPage::single(first.comments), last]),
            Unresolved
        );
        assert_eq!(
            inspect(&[
                first,
                CommentPage {
                    next_cursor: Some(id(1)),
                    ..last
                },
                last
            ]),
            Unresolved
        );

        let rows = vec![marked; MAX_COMMENTS];
        assert_eq!(
            inspect(&[
                first,
                CommentPage {
                    comments: &rows,
                    ..last
                }
            ]),
            Unresolved
        );
        let page_chain: Vec<_> = (0..=MAX_PAGES)
            .map(|index| CommentPage {
                request_cursor: NonZeroU64::new(index as u64),
                next_cursor: (index < MAX_PAGES).then(|| id(index as u64 + 1)),
                comments: &[],
            })
            .collect();
        assert_eq!(inspect(&page_chain), Unresolved);
    }

    #[test]
    fn fails_closed_when_the_enumeration_exceeds_its_bound() {
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let expected = expected(&target);
        let raw = format!("exact body\n\n<!-- {MARKER} -->");
        let comments = vec![comment(&target, &raw); MAX_COMMENTS + 1];
        assert_eq!(
            verify_current_comment(
                &expected,
                CommentEnumeration::Complete {
                    observed_at: time(30),
                    timestamp_bounds: EXACT_TIME,
                    pages: &[CommentPage::single(&comments)],
                }
            ),
            CurrentCommentPosture::Unresolved,
        );
    }
}
