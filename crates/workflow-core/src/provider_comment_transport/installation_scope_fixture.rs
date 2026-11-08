//! Synthetic scope verification. Neither these observations nor the receipt
//! can cross into production admission.

use std::num::NonZeroU64;

use serde::Deserialize;

use crate::{
    GitHubPullRequestCommentProviderAuth, GitHubPullRequestCommentTarget, SpecContentHash,
    Timestamp, WorkflowRunId,
};

use super::LiveSandboxProviderBinding;

#[derive(Clone, Copy, Eq, PartialEq)]
struct InstallationIdentity {
    app: NonZeroU64,
    installation: NonZeroU64,
    author: NonZeroU64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct ScopeRequest<'a> {
    run: &'a WorkflowRunId,
    immutable_run: &'a SpecContentHash,
    target: &'a GitHubPullRequestCommentTarget,
    provider: &'a LiveSandboxProviderBinding,
    identity: InstallationIdentity,
    deadline: Timestamp,
}

struct Issuance<'a> {
    bearer: &'a GitHubPullRequestCommentProviderAuth,
    identity: InstallationIdentity,
    selected_repository_ids: &'a [NonZeroU64],
    effective_permissions_json: &'a [u8],
    issued_at: Timestamp,
    expires_at: Timestamp,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Permissions {
    metadata: MetadataPermission,
    pull_requests: PullRequestPermission,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum MetadataPermission {
    Read,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum PullRequestPermission {
    Write,
}

#[derive(Clone, Copy)]
struct Repository<'a> {
    id: NonZeroU64,
    full_name: &'a str,
}

struct RepositoryEnumeration<'a> {
    bearer: &'a GitHubPullRequestCommentProviderAuth,
    total_count: usize,
    complete: bool,
    rows: &'a [Repository<'a>],
}

struct FixtureScopeReceipt<'a> {
    scope: ScopeRequest<'a>,
    verified_at: Timestamp,
    effective_deadline: Timestamp,
}

impl std::fmt::Debug for FixtureScopeReceipt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixtureScopeReceipt")
            .field("effective_deadline", &self.effective_deadline)
            .finish_non_exhaustive()
    }
}

impl FixtureScopeReceipt<'_> {
    fn matches(&self, scope: ScopeRequest<'_>, now: Timestamp) -> bool {
        self.scope == scope && self.verified_at <= now && now < self.effective_deadline
    }
}

fn verify<'a>(
    scope: ScopeRequest<'a>,
    resolved: InstallationIdentity,
    issuance: &Issuance<'_>,
    repositories: &RepositoryEnumeration<'_>,
    now: Timestamp,
) -> Option<FixtureScopeReceipt<'a>> {
    let [repository] = repositories.rows else {
        return None;
    };
    let effective_deadline = scope.deadline.min(issuance.expires_at);
    let max_expiry = issuance
        .issued_at
        .as_offset_date_time()
        .checked_add(time::Duration::hours(1))?;
    if resolved != scope.identity
        || issuance.identity != resolved
        || resolved.author != scope.provider.author_id
        || scope.provider.adapter_commitment != super::loopback_comment_exchange_commitment()
        || !std::ptr::eq(issuance.bearer, repositories.bearer)
        || issuance.selected_repository_ids != [scope.provider.repository_id]
        || repositories.total_count != 1
        || !repositories.complete
        || repository.id != scope.provider.repository_id
        || repository.full_name != format!("{}/{}", scope.target.owner(), scope.target.repository())
        || issuance.issued_at > now
        || issuance.expires_at.as_offset_date_time() > max_expiry
        || now >= effective_deadline
        || issuance.effective_permissions_json.len() > 256
    {
        return None;
    }
    // Struct decoding rejects missing, duplicate, unknown, and broader grants.
    let Permissions {
        metadata: MetadataPermission::Read,
        pull_requests: PullRequestPermission::Write,
    } = serde_json::from_slice(issuance.effective_permissions_json).ok()?;
    Some(FixtureScopeReceipt {
        scope,
        verified_at: now,
        effective_deadline,
    })
}

const PERMISSIONS: &[u8] = br#"{"metadata":"read","pull_requests":"write"}"#;
const SYNTHETIC_BEARER: &str = "synthetic-fixture-value-not-for-receipts";

#[allow(clippy::expect_used)]
fn id(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("fixture ID")
}

#[allow(clippy::expect_used)]
fn at(second: i64) -> Timestamp {
    Timestamp::from_offset_date_time(
        time::OffsetDateTime::from_unix_timestamp(second).expect("fixture time"),
    )
}

#[allow(clippy::expect_used)]
fn fixture(test: impl FnOnce(ScopeRequest<'_>, Issuance<'_>, RepositoryEnumeration<'_>)) {
    let auth =
        GitHubPullRequestCommentProviderAuth::new(SYNTHETIC_BEARER, None).expect("fixture bearer");
    let run = WorkflowRunId::new("run-scope-fixture").expect("fixture run");
    let immutable_run = SpecContentHash::from_text("fixture run snapshot");
    let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("fixture PR");
    let provider = LiveSandboxProviderBinding {
        repository_id: id(11),
        author_id: id(22),
        adapter_commitment: super::loopback_comment_exchange_commitment(),
    };
    let identity = InstallationIdentity {
        app: id(33),
        installation: id(44),
        author: provider.author_id,
    };
    test(
        ScopeRequest {
            run: &run,
            immutable_run: &immutable_run,
            target: &target,
            provider: &provider,
            identity,
            deadline: at(50),
        },
        Issuance {
            bearer: &auth,
            identity,
            selected_repository_ids: &[provider.repository_id],
            effective_permissions_json: PERMISSIONS,
            issued_at: at(10),
            expires_at: at(40),
        },
        RepositoryEnumeration {
            bearer: &auth,
            total_count: 1,
            complete: true,
            rows: &[Repository {
                id: provider.repository_id,
                full_name: "example/sandbox",
            }],
        },
    );
}

#[test]
#[allow(clippy::expect_used)]
fn comment_installation_scope_receipt_is_exact_and_payload_free() {
    fixture(|scope, issuance, repositories| {
        let receipt =
            verify(scope, scope.identity, &issuance, &repositories, at(20)).expect("exact scope");
        assert!(receipt.matches(scope, at(39)));
        assert!(!receipt.matches(scope, at(19)));
        assert!(!receipt.matches(scope, at(40)));
        let shorter = ScopeRequest {
            deadline: at(30),
            ..scope
        };
        assert!(!receipt.matches(shorter, at(25)));
        let local_bound =
            verify(shorter, scope.identity, &issuance, &repositories, at(20)).expect("local bound");
        assert!(!local_bound.matches(shorter, at(30)));
        let other_run = WorkflowRunId::new("run-other-fixture").expect("other run");
        let other_snapshot = SpecContentHash::from_text("other snapshot");
        let other_target =
            GitHubPullRequestCommentTarget::new("example", "sandbox", 8).expect("other PR");
        let other_adapter = LiveSandboxProviderBinding {
            adapter_commitment: super::fixed_comment_exchange_commitment(),
            ..scope.provider.clone()
        };
        for drift in [
            ScopeRequest {
                run: &other_run,
                ..scope
            },
            ScopeRequest {
                immutable_run: &other_snapshot,
                ..scope
            },
            ScopeRequest {
                target: &other_target,
                ..scope
            },
            ScopeRequest {
                provider: &other_adapter,
                ..scope
            },
        ] {
            assert!(!receipt.matches(drift, at(20)));
        }
        assert!(verify(
            ScopeRequest {
                provider: &other_adapter,
                ..scope
            },
            scope.identity,
            &issuance,
            &repositories,
            at(20),
        )
        .is_none());
        let diagnostic = format!("{receipt:?}");
        assert!(!diagnostic.contains(SYNTHETIC_BEARER));
        assert!(!diagnostic.contains("example/sandbox"));
        assert!(!diagnostic.contains("pull_requests"));
    });
}

#[test]
fn comment_installation_scope_rejects_omitted_or_broader_issuance() {
    fixture(|scope, mut issuance, repositories| {
        for raw in [
            b"{}".as_slice(),
            br#"{"pull_requests":"write"}"#,
            br#"{"metadata":"read","pull_requests":"read"}"#,
            br#"{"metadata":"read","metadata":"read","pull_requests":"write"}"#,
            br#"{"metadata":"read","pull_requests":"write","contents":"write"}"#,
        ] {
            issuance.effective_permissions_json = raw;
            assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
        }
        issuance.effective_permissions_json = PERMISSIONS;
        for ids in [&[][..], &[id(11), id(12)], &[id(12)]] {
            let substituted = Issuance {
                selected_repository_ids: ids,
                ..issuance
            };
            assert!(verify(scope, scope.identity, &substituted, &repositories, at(20)).is_none());
        }
    });
}

#[test]
#[allow(clippy::expect_used)]
fn comment_installation_scope_rejects_identity_or_bearer_substitution() {
    fixture(|scope, mut issuance, repositories| {
        let other = GitHubPullRequestCommentProviderAuth::new(SYNTHETIC_BEARER, None)
            .expect("distinct bearer object");
        for identity in [
            InstallationIdentity {
                app: id(55),
                ..scope.identity
            },
            InstallationIdentity {
                installation: id(55),
                ..scope.identity
            },
            InstallationIdentity {
                author: id(55),
                ..scope.identity
            },
        ] {
            assert!(verify(scope, identity, &issuance, &repositories, at(20)).is_none());
            issuance.identity = identity;
            assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
            issuance.identity = scope.identity;
        }
        let repositories = RepositoryEnumeration {
            bearer: &other,
            ..repositories
        };
        assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
    });
}

#[test]
fn comment_installation_scope_rejects_incomplete_enumeration_and_unusable_time() {
    fixture(|scope, mut issuance, mut repositories| {
        for now in [at(9), at(40), at(41)] {
            assert!(verify(scope, scope.identity, &issuance, &repositories, now).is_none());
        }
        issuance.expires_at = at(3611);
        assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
        issuance.expires_at = at(40);
        repositories.complete = false;
        assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
        repositories.complete = true;
        repositories.total_count = 2;
        assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
        repositories.total_count = 1;
        let expected = repositories.rows[0];
        let other_id = Repository {
            id: id(12),
            ..expected
        };
        let other_name = Repository {
            full_name: "example/other",
            ..expected
        };
        for rows in [&[][..], &[expected, expected], &[other_id], &[other_name]] {
            let repositories = RepositoryEnumeration {
                rows,
                ..repositories
            };
            assert!(verify(scope, scope.identity, &issuance, &repositories, at(20)).is_none());
        }
    });
}
