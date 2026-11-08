//! Fixed-origin installation scope exchange. It cannot authorize or send a comment.

use std::num::NonZeroU64;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    bounded_read, comment_path, github_agent, github_request, CommentSendWindow,
    GitHubPullRequestIdentity, GitHubRepositoryIdentity, GITHUB_ORIGIN,
    MAX_IDENTITY_RESPONSE_BYTES,
};
use crate::live_sandbox_continuation_authority::LiveSandboxProviderBinding;
use crate::provider_comment_observation::{
    verify_created_comment_response, CreatedCommentResponse, MAX_CREATION_RESPONSE_BYTES,
    MAX_RAW_BODY_BYTES,
};
use crate::sqlite_state::InstallationCommentSendPermit;
use crate::{
    GitHubPullRequestCommentProviderAuth, GitHubPullRequestCommentTarget, SpecContentHash,
    Timestamp, WorkflowRunId,
};

#[derive(Clone, Copy)]
pub(crate) struct ReviewedInstallation<'a> {
    pub(crate) app_id: NonZeroU64,
    pub(crate) app_slug: &'a str,
    pub(crate) bot_id: NonZeroU64,
    pub(crate) installation_id: NonZeroU64,
    pub(crate) repository_id: NonZeroU64,
    pub(crate) target: &'a GitHubPullRequestCommentTarget,
    pub(crate) run_id: &'a WorkflowRunId,
    pub(crate) immutable_run: &'a SpecContentHash,
    pub(crate) comment_adapter: &'a SpecContentHash,
    pub(crate) deadline: Timestamp,
}

impl ReviewedInstallation<'_> {
    pub(crate) fn issuance_commitment(self) -> Option<SpecContentHash> {
        valid_app_slug(self.app_slug)
            .then(|| scope_commitment(self, &fixed_adapter_commitment()))
            .flatten()
    }
}

pub(crate) fn installation_target_revision(
    target: &GitHubPullRequestCommentTarget,
    app_id: NonZeroU64,
    app_slug: &str,
    installation_id: NonZeroU64,
    provider: &crate::live_sandbox_continuation_authority::LiveSandboxProviderBinding,
) -> Option<SpecContentHash> {
    if !valid_app_slug(app_slug) {
        return None;
    }
    serde_json::to_vec(&(
        "workflow-os/github-installation-target-revision/v1",
        target,
        app_id,
        app_slug,
        installation_id,
        provider,
    ))
    .ok()
    .map(SpecContentHash::from_bytes)
}

// No export, conversion to comment auth, or serialization. This is provider
// scope evidence, not proof of current Core authority.
pub(crate) struct VerifiedInstallationScope {
    bearer: GitHubPullRequestCommentProviderAuth,
    receipt: InstallationScopeReceipt,
}

pub(crate) struct InstallationCommentTransport<'a> {
    exchange: Exchange<'a>,
    verified: VerifiedInstallationScope,
    reviewed: ReviewedInstallation<'a>,
}

impl<'a> InstallationCommentTransport<'a> {
    pub(crate) fn new(
        verified: VerifiedInstallationScope,
        reviewed: ReviewedInstallation<'a>,
    ) -> Self {
        Self {
            exchange: Exchange {
                agent: github_agent(),
                origin: GITHUB_ORIGIN,
            },
            verified,
            reviewed,
        }
    }

    pub(crate) const fn reviewed(&self) -> ReviewedInstallation<'a> {
        self.reviewed
    }

    pub(crate) const fn verified(&self) -> &VerifiedInstallationScope {
        &self.verified
    }

    pub(crate) fn is_fixture(&self) -> bool {
        self.exchange.origin != GITHUB_ORIGIN
    }

    pub(crate) fn verify_pull_request(&self) -> Option<()> {
        let scope = self.reviewed;
        self.verified
            .admission_binding(scope, Timestamp::now_utc(), self.is_fixture())?;
        let path = format!(
            "/repos/{}/{}/pulls/{}",
            scope.target.owner(),
            scope.target.repository(),
            scope.target.pull_request_number()
        );
        let pull: GitHubPullRequestIdentity = self
            .exchange
            .get(&path, self.verified.bearer.secret_for_provider())?;
        (pull.number.get() == scope.target.pull_request_number()
            && matches_repo(&pull.base.repo, scope))
        .then_some(())
    }

    pub(crate) fn send(
        self,
        permit: InstallationCommentSendPermit<'_>,
    ) -> Option<VerifiedInstallationCreation> {
        let (context, window) = permit.into_parts();
        self.verified.send_creation(
            &self.exchange,
            self.reviewed,
            context,
            window,
            Timestamp::now_utc,
        )
    }
}

#[derive(Clone, Copy)]
pub(crate) struct InstallationCreationContext<'a> {
    pub(crate) reservation: &'a SpecContentHash,
    pub(crate) scope: &'a SpecContentHash,
    pub(crate) target: &'a GitHubPullRequestCommentTarget,
    pub(crate) provider: &'a LiveSandboxProviderBinding,
    pub(crate) marked_body: &'a str,
}

// Only a bounded creation exchange can mint this proof in production.
pub(crate) struct VerifiedInstallationCreation {
    binding: SpecContentHash,
    comment_id: NonZeroU64,
    fixture: bool,
}

impl VerifiedInstallationCreation {
    #[cfg(test)]
    pub(crate) fn matches(
        &self,
        expected: InstallationCreationContext<'_>,
        fixture: bool,
    ) -> Option<NonZeroU64> {
        self.matches_committed_body(
            expected.reservation,
            expected.scope,
            expected.target,
            expected.provider,
            &SpecContentHash::from_text(expected.marked_body),
            fixture,
        )
    }

    pub(crate) fn matches_committed_body(
        &self,
        reservation: &SpecContentHash,
        scope: &SpecContentHash,
        target: &GitHubPullRequestCommentTarget,
        provider: &LiveSandboxProviderBinding,
        body: &SpecContentHash,
        fixture: bool,
    ) -> Option<NonZeroU64> {
        (self.fixture == fixture
            && creation_binding(reservation, scope, target, provider, body).as_ref()
                == Some(&self.binding))
        .then_some(self.comment_id)
    }
}

#[derive(Debug)]
struct InstallationScopeReceipt {
    scope_commitment: SpecContentHash,
    verified_at: Timestamp,
    effective_deadline: Timestamp,
}

impl InstallationScopeReceipt {
    fn matches(
        &self,
        expected: ReviewedInstallation<'_>,
        adapter: &SpecContentHash,
        now: Timestamp,
    ) -> bool {
        self.verified_at <= now
            && now < self.effective_deadline
            && scope_commitment(expected, adapter).as_ref() == Some(&self.scope_commitment)
    }
}

impl VerifiedInstallationScope {
    pub(crate) const fn verified_at(&self) -> Timestamp {
        self.receipt.verified_at
    }

    pub(crate) fn admission_binding(
        &self,
        expected: ReviewedInstallation<'_>,
        now: Timestamp,
        fixture: bool,
    ) -> Option<(SpecContentHash, Timestamp)> {
        let adapter = if fixture {
            #[cfg(test)]
            {
                fixture_adapter_commitment()
            }
            #[cfg(not(test))]
            {
                return None;
            }
        } else {
            fixed_adapter_commitment()
        };
        if !self.receipt.matches(expected, &adapter, now) {
            return None;
        }
        let commitment = serde_json::to_vec(&(
            "workflow-os/github-installation-scope-admission/v1",
            &self.receipt.scope_commitment,
            self.receipt.verified_at,
            self.receipt.effective_deadline,
        ))
        .ok()
        .map(SpecContentHash::from_bytes)?;
        Some((commitment, self.receipt.effective_deadline))
    }

    fn send_creation(
        self,
        exchange: &Exchange<'_>,
        reviewed: ReviewedInstallation<'_>,
        context: InstallationCreationContext<'_>,
        window: CommentSendWindow,
        now: impl FnOnce() -> Timestamp,
    ) -> Option<VerifiedInstallationCreation> {
        let fixture = exchange.origin != GITHUB_ORIGIN;
        let adapter = exchange.adapter_commitment()?;
        if !self
            .receipt
            .matches(reviewed, &adapter, self.receipt.verified_at)
            || context.target != reviewed.target
            || context.provider.repository_id != reviewed.repository_id
            || context.provider.author_id != reviewed.bot_id
            || &context.provider.adapter_commitment != reviewed.comment_adapter
            || context.marked_body.is_empty()
            || context.marked_body.len() > MAX_RAW_BODY_BYTES
        {
            return None;
        }
        let body = serde_json::json!({"body": context.marked_body}).to_string();
        let request = github_request(
            &exchange.agent,
            exchange.origin,
            "POST",
            &comment_path(context.target),
            self.bearer.secret_for_provider(),
        )
        .set("Content-Type", "application/json")
        .set("Accept", "application/vnd.github.raw+json");
        let now = now();
        if now < window.checked_at || now >= window.deadline {
            return None;
        }
        let (scope, _) = self.admission_binding(reviewed, now, fixture)?;
        if &scope != context.scope {
            return None;
        }
        let response = request.send_string(&body).ok()?;
        if response.status() != 201 || response.header("Link").is_some() {
            return None;
        }
        let response = CreatedCommentResponse {
            status: 201,
            repository_id: reviewed.repository_id,
            json: bounded_read(response.into_reader(), MAX_CREATION_RESPONSE_BYTES)?,
        };
        capture_creation(context, &response, fixture)
    }

    #[cfg(test)]
    #[allow(clippy::expect_used)]
    pub(crate) fn test_receipt(
        scope: ReviewedInstallation<'_>,
        verified_at: Timestamp,
        effective_deadline: Timestamp,
    ) -> Self {
        Self::test_receipt_for_adapter(
            scope,
            verified_at,
            effective_deadline,
            &fixture_adapter_commitment(),
        )
    }

    #[cfg(test)]
    pub(crate) fn test_receipt_with_live_shape(
        scope: ReviewedInstallation<'_>,
        verified_at: Timestamp,
        effective_deadline: Timestamp,
    ) -> Self {
        Self::test_receipt_for_adapter(
            scope,
            verified_at,
            effective_deadline,
            &fixed_adapter_commitment(),
        )
    }

    #[cfg(test)]
    #[allow(clippy::expect_used)]
    fn test_receipt_for_adapter(
        scope: ReviewedInstallation<'_>,
        verified_at: Timestamp,
        effective_deadline: Timestamp,
        adapter: &SpecContentHash,
    ) -> Self {
        Self {
            bearer: GitHubPullRequestCommentProviderAuth::new("fixture-installation-bearer", None)
                .expect("fixture bearer"),
            receipt: InstallationScopeReceipt {
                scope_commitment: scope_commitment(scope, adapter).expect("fixture scope"),
                verified_at,
                effective_deadline,
            },
        }
    }
}

fn creation_binding(
    reservation: &SpecContentHash,
    scope: &SpecContentHash,
    target: &GitHubPullRequestCommentTarget,
    provider: &LiveSandboxProviderBinding,
    body: &SpecContentHash,
) -> Option<SpecContentHash> {
    serde_json::to_vec(&(
        "workflow-os/github-installation-comment-creation/v1",
        reservation,
        scope,
        target,
        provider,
        body,
    ))
    .ok()
    .map(SpecContentHash::from_bytes)
}

fn capture_creation(
    context: InstallationCreationContext<'_>,
    response: &CreatedCommentResponse,
    fixture: bool,
) -> Option<VerifiedInstallationCreation> {
    let comment_id = verify_created_comment_response(
        Some(response),
        context.target,
        context.provider,
        context.marked_body,
    )?;
    Some(VerifiedInstallationCreation {
        binding: creation_binding(
            context.reservation,
            context.scope,
            context.target,
            context.provider,
            &SpecContentHash::from_text(context.marked_body),
        )?,
        comment_id,
        fixture,
    })
}

#[cfg(test)]
pub(crate) fn fixture_creation_proof(
    context: InstallationCreationContext<'_>,
    response: &CreatedCommentResponse,
) -> Option<VerifiedInstallationCreation> {
    capture_creation(context, response, true)
}

#[derive(Deserialize)]
struct App {
    id: NonZeroU64,
    slug: String,
}

#[derive(Deserialize)]
struct Bot {
    id: NonZeroU64,
    login: String,
    #[serde(rename = "type")]
    account_type: BotAccountType,
}

#[derive(Deserialize)]
enum BotAccountType {
    Bot,
}

#[derive(Deserialize)]
struct Installation {
    id: NonZeroU64,
    app_id: NonZeroU64,
    target_type: InstallationTarget,
    repository_selection: Selection,
    suspended_at: serde_json::Value,
}

#[derive(Deserialize)]
enum InstallationTarget {
    User,
    Organization,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Selection {
    Selected,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Permissions {
    metadata: MetadataPermission,
    pull_requests: PullRequestPermission,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MetadataPermission {
    Read,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PullRequestPermission {
    Write,
}

#[derive(Deserialize)]
struct IssuedAccess {
    token: String,
    expires_at: Timestamp,
    permissions: Permissions,
    repository_selection: Selection,
    repositories: [GitHubRepositoryIdentity; 1],
}

#[derive(Deserialize)]
struct InstallationRepositories {
    total_count: u64,
    repositories: [GitHubRepositoryIdentity; 1],
}

struct Exchange<'a> {
    agent: ureq::Agent,
    origin: &'a str,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) fn exchange_installation_scope(
    scope: ReviewedInstallation<'_>,
    signed_app_assertion: super::installation_signer_channel::InstallationAppAssertion,
) -> Option<VerifiedInstallationScope> {
    let (assertion, expires_at) = signed_app_assertion.into_parts();
    Exchange {
        agent: github_agent(),
        origin: GITHUB_ORIGIN,
    }
    .exchange(scope, &assertion, expires_at, Timestamp::now_utc)
}

impl Exchange<'_> {
    fn exchange(
        &self,
        scope: ReviewedInstallation<'_>,
        app_assertion: &str,
        assertion_deadline: Timestamp,
        now: impl Fn() -> Timestamp,
    ) -> Option<VerifiedInstallationScope> {
        scope.target.validate().ok()?;
        let adapter = self.adapter_commitment()?;
        let started = now();
        let issuance_deadline = scope.deadline.min(assertion_deadline);
        if started >= issuance_deadline
            || !valid_app_assertion(app_assertion)
            || !valid_app_slug(scope.app_slug)
        {
            return None;
        }
        let scope_commitment = scope_commitment(scope, &adapter)?;
        let installation_path = format!("/app/installations/{}", scope.installation_id);
        let app: App = self.get("/app", app_assertion)?;
        if app.id != scope.app_id || app.slug != scope.app_slug {
            return None;
        }
        let Installation {
            id,
            app_id,
            suspended_at,
            target_type: InstallationTarget::User | InstallationTarget::Organization,
            repository_selection: Selection::Selected,
        } = self.get(&installation_path, app_assertion)?;
        if id != scope.installation_id || app_id != scope.app_id || !suspended_at.is_null() {
            return None;
        }
        let before_issue = now();
        if before_issue < started || before_issue >= issuance_deadline {
            return None;
        }
        let issued = self.issue(scope, app_assertion, &installation_path)?;
        let after_issue = now();
        let effective_deadline = scope.deadline.min(issued.expires_at);
        let upper_expiry = after_issue
            .as_offset_date_time()
            .checked_add(time::Duration::hours(1))?;
        if after_issue < before_issue
            || after_issue >= effective_deadline
            || issued.expires_at.as_offset_date_time() > upper_expiry
        {
            return None;
        }
        let bearer = GitHubPullRequestCommentProviderAuth::new(issued.token, None).ok()?;
        self.verify_issued_scope(scope, &bearer)?;
        let verified_at = now();
        if verified_at < after_issue || verified_at >= effective_deadline {
            return None;
        }
        Some(VerifiedInstallationScope {
            bearer,
            receipt: InstallationScopeReceipt {
                scope_commitment,
                verified_at,
                effective_deadline,
            },
        })
    }

    fn issue(
        &self,
        scope: ReviewedInstallation<'_>,
        app_assertion: &str,
        installation_path: &str,
    ) -> Option<IssuedAccess> {
        let request = serde_json::json!({
            "repository_ids": [scope.repository_id],
            "permissions": Permissions {
                metadata: MetadataPermission::Read,
                pull_requests: PullRequestPermission::Write,
            }
        });
        let issued: IssuedAccess = decode(
            github_request(
                &self.agent,
                self.origin,
                "POST",
                &format!("{installation_path}/access_tokens"),
                app_assertion,
            )
            .set("Content-Type", "application/json")
            .send_string(&request.to_string())
            .ok()?,
            201,
        )?;
        let IssuedAccess {
            repositories: [ref issued_repo],
            permissions:
                Permissions {
                    metadata: MetadataPermission::Read,
                    pull_requests: PullRequestPermission::Write,
                },
            repository_selection: Selection::Selected,
            ..
        } = issued;
        matches_repo(issued_repo, scope).then_some(issued)
    }

    fn verify_issued_scope(
        &self,
        scope: ReviewedInstallation<'_>,
        bearer: &GitHubPullRequestCommentProviderAuth,
    ) -> Option<()> {
        let InstallationRepositories {
            total_count,
            repositories: [listed],
        } = self.get(
            "/installation/repositories?per_page=2&page=1",
            bearer.secret_for_provider(),
        )?;
        if total_count != 1 || !matches_repo(&listed, scope) {
            return None;
        }
        let Bot {
            id,
            login,
            account_type: BotAccountType::Bot,
        } = self.get(
            &format!("/user/{}", scope.bot_id),
            bearer.secret_for_provider(),
        )?;
        (id == scope.bot_id && login == format!("{}[bot]", scope.app_slug)).then_some(())
    }

    fn adapter_commitment(&self) -> Option<SpecContentHash> {
        if self.origin == GITHUB_ORIGIN {
            return Some(fixed_adapter_commitment());
        }
        #[cfg(test)]
        if self.origin.starts_with("http://127.0.0.1:") {
            return Some(fixture_adapter_commitment());
        }
        None
    }

    fn get<T: DeserializeOwned>(&self, path: &str, bearer: &str) -> Option<T> {
        let response = github_request(&self.agent, self.origin, "GET", path, bearer)
            .call()
            .ok()?;
        decode(response, 200)
    }
}

fn fixed_adapter_commitment() -> SpecContentHash {
    SpecContentHash::from_text("workflow-os/github-installation-scope/github.com/2022-11-28/v1")
}

pub(crate) fn installation_comment_adapter_commitment() -> SpecContentHash {
    SpecContentHash::from_text("workflow-os/github-installation-comment/github.com/v1")
}

#[cfg(test)]
pub(crate) fn fixture_installation_comment_adapter_commitment() -> SpecContentHash {
    SpecContentHash::from_text("workflow-os/github-installation-comment/loopback-fixture/v1")
}

#[cfg(test)]
fn fixture_adapter_commitment() -> SpecContentHash {
    SpecContentHash::from_text("workflow-os/github-installation-scope/loopback-fixture/v1")
}

fn scope_commitment(
    scope: ReviewedInstallation<'_>,
    adapter: &SpecContentHash,
) -> Option<SpecContentHash> {
    serde_json::to_vec(&(
        "workflow-os/github-installation-scope-receipt/v1",
        scope.run_id,
        scope.immutable_run,
        scope.target,
        scope.app_id,
        scope.app_slug,
        scope.bot_id,
        scope.installation_id,
        scope.repository_id,
        scope.comment_adapter,
        scope.deadline,
        adapter,
    ))
    .ok()
    .map(SpecContentHash::from_bytes)
}

fn valid_app_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 100
        && slug
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

pub(super) fn valid_app_assertion(assertion: &str) -> bool {
    if assertion.len() > 8192 {
        return false;
    }
    let mut segments = assertion.split('.');
    (0..3).all(|_| {
        segments.next().is_some_and(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
        })
    }) && segments.next().is_none()
}

fn matches_repo(repo: &GitHubRepositoryIdentity, scope: ReviewedInstallation<'_>) -> bool {
    repo.id == scope.repository_id
        && repo.full_name == format!("{}/{}", scope.target.owner(), scope.target.repository())
}

fn decode<T: DeserializeOwned>(response: ureq::Response, status: u16) -> Option<T> {
    // A singleton response must not advertise another page or a redirect.
    if response.status() != status || response.header("Link").is_some() {
        return None;
    }
    let bytes = bounded_read(response.into_reader(), MAX_IDENTITY_RESPONSE_BYTES)?;
    serde_json::from_slice(&bytes).ok()
}

#[cfg(test)]
#[allow(clippy::expect_used)]
pub(crate) mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Read, Write},
        net::{Ipv4Addr, TcpListener},
        thread,
        time::{Duration, Instant},
    };

    const ASSERTION: &str = "fixture.app.assertion";
    const ISSUED: &str = "fixture-installation-bearer";
    const APP: Reply = Reply {
        request: "GET /app HTTP/1.1",
        status: "200 OK",
        header: "",
        body: r#"{"id":33,"slug":"fixture-app"}"#,
    };
    const INSTALLATION: Reply = Reply {
        request: "GET /app/installations/44 HTTP/1.1",
        status: "200 OK",
        header: "",
        body: r#"{"id":44,"app_id":33,"target_type":"User","repository_selection":"selected","suspended_at":null}"#,
    };
    const ISSUE: Reply = Reply {
        request: "POST /app/installations/44/access_tokens HTTP/1.1",
        status: "201 Created",
        header: "",
        body: r#"{"token":"fixture-installation-bearer","expires_at":"1970-01-01T00:00:40Z","repository_selection":"selected","permissions":{"metadata":"read","pull_requests":"write"},"repositories":[{"id":11,"full_name":"example/sandbox"}]}"#,
    };
    const LIST: Reply = Reply {
        request: "GET /installation/repositories?per_page=2&page=1 HTTP/1.1",
        status: "200 OK",
        header: "",
        body: r#"{"total_count":1,"repositories":[{"id":11,"full_name":"example/sandbox"}]}"#,
    };
    const BOT: Reply = Reply {
        request: "GET /user/22 HTTP/1.1",
        status: "200 OK",
        header: "",
        body: r#"{"id":22,"login":"fixture-app[bot]","type":"Bot"}"#,
    };
    const CREATE: Reply = Reply {
        request: "POST /repos/example/sandbox/issues/7/comments HTTP/1.1",
        status: "201 Created",
        header: "",
        body: r#"{"id":17,"login":"fixture-app[bot]","user":{"id":22},"body":"marked fixture comment","issue_url":"https://api.github.com/repos/example/sandbox/issues/7","url":"https://api.github.com/repos/example/sandbox/issues/comments/17"}"#,
    };
    const MARKED: &str = "marked fixture comment";
    const PR: Reply = Reply {
        request: "GET /repos/example/sandbox/pulls/7 HTTP/1.1",
        status: "200 OK",
        header: "",
        body: r#"{"number":7,"base":{"repo":{"id":123,"full_name":"example/sandbox"}}}"#,
    };

    #[derive(Clone, Copy)]
    struct Reply<'a> {
        request: &'a str,
        status: &'a str,
        header: &'a str,
        body: &'a str,
    }

    fn at(seconds: i64) -> Timestamp {
        Timestamp::from_offset_date_time(
            time::OffsetDateTime::from_unix_timestamp(seconds).expect("fixture time"),
        )
    }

    fn exercise(replies: &[Reply<'_>], now: impl Fn() -> Timestamp, expected: Option<Timestamp>) {
        exercise_with_creation(replies, now, expected, None);
    }

    fn exercise_with_creation(
        replies: &[Reply<'_>],
        now: impl Fn() -> Timestamp,
        expected: Option<Timestamp>,
        creation_expected: Option<bool>,
    ) {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let origin = format!("http://{}", listener.local_addr().expect("local address"));
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let run = WorkflowRunId::new("run-installation-fixture").expect("run");
        let immutable_run = SpecContentHash::from_text("immutable fixture");
        let comment_adapter = super::super::loopback_comment_exchange_commitment();
        let scope = ReviewedInstallation {
            app_id: NonZeroU64::new(33).expect("app"),
            app_slug: "fixture-app",
            bot_id: NonZeroU64::new(22).expect("bot"),
            installation_id: NonZeroU64::new(44).expect("installation"),
            repository_id: NonZeroU64::new(11).expect("repository"),
            target: &target,
            run_id: &run,
            immutable_run: &immutable_run,
            comment_adapter: &comment_adapter,
            deadline: at(50),
        };
        thread::scope(|threads| {
            let server = threads.spawn(|| {
                for reply in replies {
                    serve(&listener, *reply);
                }
            });
            let exchange = Exchange {
                agent: github_agent(),
                origin: &origin,
            };
            let verified = exchange.exchange(scope, ASSERTION, scope.deadline, &now);
            assert_eq!(
                verified
                    .as_ref()
                    .map(|verified| verified.receipt.effective_deadline),
                expected
            );
            if let Some(verified) = verified {
                let issued_bearer_retained = verified.bearer.secret_for_provider() == ISSUED;
                assert!(issued_bearer_retained);
                assert_receipt_binding(&verified.receipt, scope, &exchange);
                if let Some(expected) = creation_expected {
                    assert_creation_exchange(verified, &exchange, scope, &now, expected);
                }
            }
            server.join().expect("fixture exchange");
            assert!(
                matches!(
                    listener.accept(),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock
                ),
                "unexpected extra HTTP request"
            );
        });
    }

    fn assert_creation_exchange(
        verified: VerifiedInstallationScope,
        exchange: &Exchange<'_>,
        reviewed: ReviewedInstallation<'_>,
        now: impl FnOnce() -> Timestamp,
        expected: bool,
    ) {
        let (scope, _) = verified
            .admission_binding(reviewed, at(20), true)
            .expect("fixture scope");
        let provider = LiveSandboxProviderBinding {
            repository_id: reviewed.repository_id,
            author_id: reviewed.bot_id,
            adapter_commitment: reviewed.comment_adapter.clone(),
        };
        let reservation = SpecContentHash::from_text("durable fixture reservation");
        let context = InstallationCreationContext {
            reservation: &reservation,
            scope: &scope,
            target: reviewed.target,
            provider: &provider,
            marked_body: MARKED,
        };
        let proof = verified.send_creation(
            exchange,
            reviewed,
            context,
            CommentSendWindow {
                checked_at: at(20),
                deadline: at(40),
            },
            now,
        );
        assert_eq!(proof.is_some(), expected);
        if let Some(proof) = proof {
            assert_eq!(proof.matches(context, true), NonZeroU64::new(17));
            assert!(proof.matches(context, false).is_none());
        }
    }

    fn assert_receipt_binding(
        receipt: &InstallationScopeReceipt,
        scope: ReviewedInstallation<'_>,
        exchange: &Exchange<'_>,
    ) {
        let adapter = exchange.adapter_commitment().expect("fixture identity");
        assert!(receipt.matches(scope, &adapter, at(20)));
        assert!(!receipt.matches(scope, &adapter, at(19)));
        assert!(!receipt.matches(scope, &adapter, receipt.effective_deadline));
        let different_run = WorkflowRunId::new("run-different").expect("other run");
        let different_bundle = SpecContentHash::from_text("different immutable fixture");
        let different_target =
            GitHubPullRequestCommentTarget::new("example", "sandbox", 8).expect("other target");
        for drift in [
            ReviewedInstallation {
                run_id: &different_run,
                ..scope
            },
            ReviewedInstallation {
                immutable_run: &different_bundle,
                ..scope
            },
            ReviewedInstallation {
                target: &different_target,
                ..scope
            },
            ReviewedInstallation {
                bot_id: scope.app_id,
                ..scope
            },
            ReviewedInstallation {
                deadline: at(49),
                ..scope
            },
        ] {
            assert!(!receipt.matches(drift, &adapter, at(20)));
        }
        let live_adapter = Exchange {
            agent: github_agent(),
            origin: GITHUB_ORIGIN,
        }
        .adapter_commitment()
        .expect("live identity");
        assert!(!receipt.matches(scope, &live_adapter, at(20)));
        let diagnostic = format!("{receipt:?}");
        for value in [ISSUED, ASSERTION, "fixture-app", "example/sandbox"] {
            assert!(!diagnostic.contains(value));
        }
    }

    fn serve(listener: &TcpListener, reply: Reply<'_>) {
        serve_with_echo(listener, reply, false);
    }

    fn serve_with_echo(listener: &TcpListener, reply: Reply<'_>, echo: bool) {
        let start = Instant::now();
        let mut stream = loop {
            match listener.accept() {
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        && start.elapsed() < Duration::from_secs(5) =>
                {
                    thread::sleep(Duration::from_millis(5));
                }
                result => break result.expect("request").0,
            }
        };
        stream
            .set_nonblocking(false)
            .expect("blocking fixture stream");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("read timeout");
        let mut reader = BufReader::new(&mut stream);
        let mut line = String::new();
        reader.read_line(&mut line).expect("request line");
        assert!(
            line.trim_end() == reply.request,
            "unexpected fixture request"
        );
        let mut auth = None;
        let mut length = 0;
        loop {
            line.clear();
            assert!(reader.read_line(&mut line).expect("header line") > 0);
            if line == "\r\n" {
                break;
            }
            let (key, value) = line.split_once(':').expect("header");
            if key.eq_ignore_ascii_case("Authorization") {
                assert!(auth.is_none());
                auth = Some(value.trim().to_owned());
            } else if key.eq_ignore_ascii_case("Content-Length") {
                length = value.trim().parse().expect("body length");
            }
        }
        let expected_bearer =
            if [LIST.request, BOT.request, CREATE.request, PR.request].contains(&reply.request) {
                ISSUED
            } else {
                ASSERTION
            };
        let bearer_matches = auth.as_deref() == Some(format!("Bearer {expected_bearer}").as_str());
        assert!(bearer_matches, "unexpected fixture authorization");
        let mut response_body = reply.body.to_owned();
        if reply.request == ISSUE.request || reply.request == CREATE.request {
            assert!(length <= 1024);
            let mut body = vec![0; length];
            reader.read_exact(&mut body).expect("issue request");
            let expected = if reply.request == ISSUE.request {
                serde_json::json!({"repository_ids":[11],"permissions":{"metadata":"read","pull_requests":"write"}})
            } else {
                serde_json::json!({"body": MARKED})
            };
            let received = serde_json::from_slice::<serde_json::Value>(&body).expect("JSON");
            if echo {
                assert!(received["body"]
                    .as_str()
                    .expect("marked body")
                    .contains("workflow-os/live-comment/"));
                let mut response: serde_json::Value =
                    serde_json::from_str(reply.body).expect("response JSON");
                response["body"] = received["body"].clone();
                response["user"]["id"] = serde_json::json!(456);
                response_body = response.to_string();
            } else {
                assert_eq!(received, expected);
            }
        } else {
            assert_eq!(length, 0);
        }
        if reply.status.is_empty() {
            return;
        }
        write!(
            stream,
            "HTTP/1.1 {}\r\nConnection: close\r\n{}Content-Length: {}\r\n\r\n{}",
            reply.status,
            reply.header,
            response_body.len(),
            response_body,
        )
        .expect("fixture response");
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    pub(crate) enum ReservedExchange {
        Replay,
        WrongPull,
        PreflightOnly,
        Created,
        Lost,
    }

    pub(crate) fn with_reserved_exchange<T>(
        reviewed: ReviewedInstallation<'_>,
        verified_at: Timestamp,
        effective_deadline: Timestamp,
        mode: ReservedExchange,
        run: impl FnOnce(InstallationCommentTransport<'_>) -> T,
    ) -> T {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let origin = format!("http://{}", listener.local_addr().expect("address"));
        thread::scope(|threads| {
            let server = threads.spawn(|| match mode {
                ReservedExchange::Replay => {}
                ReservedExchange::WrongPull => serve(&listener, Reply {
                    body: r#"{"number":8,"base":{"repo":{"id":123,"full_name":"example/sandbox"}}}"#,
                    ..PR
                }),
                _ => {
                    serve(&listener, PR);
                    if matches!(mode, ReservedExchange::Created | ReservedExchange::Lost) {
                        let status = if mode == ReservedExchange::Lost { "" } else { CREATE.status };
                        serve_with_echo(&listener, Reply { status, ..CREATE }, true);
                    }
                }
            });
            let result = run(InstallationCommentTransport {
                exchange: Exchange {
                    agent: github_agent(),
                    origin: &origin,
                },
                reviewed,
                verified: VerifiedInstallationScope::test_receipt(
                    reviewed,
                    verified_at,
                    effective_deadline,
                ),
            });
            server.join().expect("reserved fixture exchange");
            assert!(
                matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock),
                "unexpected request or resend"
            );
            result
        })
    }

    #[test]
    fn comment_installation_exchange_uses_issued_bearer_for_exact_scope() {
        exercise(
            &[APP, INSTALLATION, ISSUE, LIST, BOT],
            || at(20),
            Some(at(40)),
        );
    }

    #[test]
    fn comment_installation_creation_requires_exact_acknowledgement() {
        exercise_with_creation(
            &[APP, INSTALLATION, ISSUE, LIST, BOT, CREATE],
            || at(20),
            Some(at(40)),
            Some(true),
        );
        for reply in [
            Reply {
                status: "",
                ..CREATE
            },
            Reply {
                status: "200 OK",
                ..CREATE
            },
            Reply {
                body: &CREATE.body.replace("\"id\":22", "\"id\":23"),
                ..CREATE
            },
            Reply {
                body: &CREATE.body.replace(MARKED, "substituted comment"),
                ..CREATE
            },
            Reply {
                body: &CREATE.body.replace("/issues/7", "/issues/8"),
                ..CREATE
            },
        ] {
            exercise_with_creation(
                &[APP, INSTALLATION, ISSUE, LIST, BOT, reply],
                || at(20),
                Some(at(40)),
                Some(false),
            );
        }
        let samples = std::cell::Cell::new(0);
        exercise_with_creation(
            &[APP, INSTALLATION, ISSUE, LIST, BOT],
            || {
                let sample = samples.get();
                samples.set(sample + 1);
                at(if sample == 4 { 40 } else { 20 })
            },
            Some(at(40)),
            Some(false),
        );
    }

    #[test]
    fn comment_installation_exchange_rejects_wrong_app_or_broad_installation_before_issue() {
        exercise(
            &[Reply {
                body: r#"{"id":99,"slug":"fixture-app"}"#,
                ..APP
            }],
            || at(20),
            None,
        );
        exercise(
            &[Reply {
                body: r#"{"id":33,"slug":"substituted"}"#,
                ..APP
            }],
            || at(20),
            None,
        );
        for body in [
            INSTALLATION.body.replace("\"selected\"", "\"all\""),
            INSTALLATION.body.replace("\"User\"", "\"Enterprise\""),
            INSTALLATION.body.replace("\"id\":44", "\"id\":45"),
            INSTALLATION
                .body
                .replace("null", "\"1970-01-01T00:00:15Z\""),
        ] {
            exercise(
                &[
                    APP,
                    Reply {
                        body: &body,
                        ..INSTALLATION
                    },
                ],
                || at(20),
                None,
            );
        }
    }

    #[test]
    fn comment_installation_exchange_rejects_bot_identity_substitution() {
        for body in [
            BOT.body.replace("\"id\":22", "\"id\":33"),
            BOT.body.replace("fixture-app[bot]", "another-app[bot]"),
            BOT.body.replace("\"Bot\"", "\"User\""),
        ] {
            exercise(
                &[APP, INSTALLATION, ISSUE, LIST, Reply { body: &body, ..BOT }],
                || at(20),
                None,
            );
        }
    }

    #[test]
    fn comment_installation_exchange_rejects_bad_issuance_before_listing() {
        for body in [
            ISSUE.body.replace("\"write\"", "\"read\""),
            ISSUE.body.replace("\"metadata\":\"read\",", ""),
            ISSUE.body.replace(
                "\"metadata\":\"read\",",
                "\"metadata\":\"read\",\"contents\":\"write\",",
            ),
            ISSUE.body.replace("\"id\":11", "\"id\":12"),
            ISSUE.body.replace("00:00:40", "00:00:20"),
            ISSUE.body.replace("00:00:40", "01:00:21"),
        ] {
            exercise(
                &[
                    APP,
                    INSTALLATION,
                    Reply {
                        body: &body,
                        ..ISSUE
                    },
                ],
                || at(20),
                None,
            );
        }
    }

    #[test]
    fn comment_installation_exchange_rejects_incomplete_or_wrong_enumeration() {
        for reply in [
            Reply {
                header: "Link: <http://127.0.0.1:1/extra>; rel=\"next\"\r\n",
                ..LIST
            },
            Reply {
                status: "302 Found",
                header: "Location: http://127.0.0.1:1/other\r\n",
                ..LIST
            },
            Reply {
                body: r#"{"total_count":2,"repositories":[{"id":11,"full_name":"example/sandbox"}]}"#,
                ..LIST
            },
            Reply {
                body: r#"{"total_count":1,"repositories":[{"id":11,"full_name":"example/other"}]}"#,
                ..LIST
            },
        ] {
            exercise(&[APP, INSTALLATION, ISSUE, reply], || at(20), None);
        }
        let samples = std::cell::Cell::new(0);
        exercise(
            &[APP, INSTALLATION, ISSUE, LIST, BOT],
            || {
                let sample = samples.get();
                samples.set(sample + 1);
                at(if sample == 3 { 40 } else { 20 })
            },
            None,
        );
    }
}
