//! Offline verification of a current GitHub comment observation. This is
//! neither acceptance proof nor a dispatch authorization.
#![allow(dead_code)] // No trusted production observer is wired yet.

use std::num::NonZeroU64;

use crate::{GitHubPullRequestCommentTarget, Timestamp};

const MAX_COMMENTS: usize = 128;
const MAX_RAW_BODY_BYTES: usize = 64 * 1024;
const MARKER_PREFIX: &str = "<!-- workflow-os/live-comment/";
const MARKER_NAMESPACE: &str = "workflow-os/live-comment/";

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

/// Only an operator-controlled adapter may classify enumeration as complete.
/// A model-supplied page count, marker, or digest is not that evidence.
#[derive(Clone, Copy)]
pub(crate) enum CommentEnumeration<'a> {
    Complete {
        observed_at: Timestamp,
        comments: &'a [ProviderComment<'a>],
    },
    Incomplete,
    Unauthorized,
    Unavailable,
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
        comments,
    } = enumeration
    else {
        return Unresolved;
    };
    let Some(marker_digest) = expected.reservation_marker.strip_prefix(MARKER_NAMESPACE) else {
        return Unresolved;
    };
    if marker_digest.len() != 64
        || !marker_digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        || expected.body.contains(MARKER_PREFIX)
        || expected.target.validate().is_err()
        || expected.not_before >= expected.not_after
        || observed_at < expected.not_before
        || comments.len() > MAX_COMMENTS
    {
        return Unresolved;
    }
    let suffix = format!("\n\n<!-- {} -->", expected.reservation_marker);
    if expected.body.len().saturating_add(suffix.len()) > MAX_RAW_BODY_BYTES {
        return Unresolved;
    }
    let mut candidates = comments.iter().filter(|comment| {
        comment.raw_body.contains(expected.reservation_marker)
            || expected.known_comment_id == Some(comment.comment_id)
    });
    // Refuse an oversized response rather than reasoning from a truncated scan.
    if comments
        .iter()
        .any(|comment| comment.raw_body.len() > MAX_RAW_BODY_BYTES)
    {
        return Unresolved;
    }
    let Some(comment) = candidates.next() else {
        return Absent;
    };
    let mut ambiguous = false;
    for candidate in std::iter::once(comment).chain(candidates) {
        // A later page can repeat the same ID after an edit removes the marker.
        if comments
            .iter()
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
        || comment.created_at < expected.not_before
        || comment.created_at >= expected.not_after
        || comment.created_at > observed_at
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
                    comments,
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
                    comments,
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
                    comments: &comments,
                }
            ),
            CurrentCommentPosture::Unresolved,
        );
    }
}
