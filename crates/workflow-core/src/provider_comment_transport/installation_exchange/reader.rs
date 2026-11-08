//! Private issuance for an independent, repository-scoped recovery reader.
use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use super::GITHUB_ORIGIN;
use super::{
    decode, github_agent, github_request, valid_app_assertion, valid_app_slug, App, Bot, Exchange,
    GitHubRepositoryIdentity, Installation, InstallationRepositories, MetadataPermission,
};
use crate::{
    GitHubPullRequestCommentProviderAuth, GitHubPullRequestCommentTarget, SpecContentHash,
    Timestamp,
};

#[derive(Clone, Copy)]
pub(crate) struct ReviewedReadInstallation<'a> {
    pub(crate) app_id: NonZeroU64,
    pub(crate) app_slug: &'a str,
    pub(crate) installation: NonZeroU64,
    pub(crate) reader: NonZeroU64,
    pub(crate) repository: NonZeroU64,
    pub(crate) target: &'a GitHubPullRequestCommentTarget,
    pub(crate) deadline: Timestamp,
}

impl ReviewedReadInstallation<'_> {
    pub(crate) fn access_commitment(&self) -> Option<SpecContentHash> {
        self.target.validate().ok()?;
        if !valid_app_slug(self.app_slug) {
            return None;
        }
        serde_json::to_vec(&(
            "workflow-os/github-installation-comment-reader/v1",
            self.app_id,
            self.app_slug,
            self.installation,
            self.reader,
            self.repository,
            self.target,
            self.deadline,
            ReadPermissions::exact(),
        ))
        .ok()
        .map(SpecContentHash::from_bytes)
    }

    fn matches_repo(&self, repo: &GitHubRepositoryIdentity) -> bool {
        repo.id == self.repository
            && repo.full_name == format!("{}/{}", self.target.owner(), self.target.repository())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadPermissions {
    metadata: MetadataPermission,
    pull_requests: ReadPermission,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReadPermission {
    Read,
}

impl ReadPermissions {
    const fn exact() -> Self {
        Self {
            metadata: MetadataPermission::Read,
            pull_requests: ReadPermission::Read,
        }
    }
}

#[derive(Deserialize)]
struct IssuedReadAccess {
    token: String,
    expires_at: Timestamp,
    #[serde(rename = "permissions")]
    _permissions: ReadPermissions,
    repositories: [GitHubRepositoryIdentity; 1],
}

#[derive(Deserialize)]
struct ReadInstallation {
    #[serde(flatten)]
    installation: Installation,
    #[serde(rename = "permissions")]
    _permissions: ReadPermissions,
}

pub(crate) struct VerifiedReadInstallationScope {
    bearer: GitHubPullRequestCommentProviderAuth,
    target: GitHubPullRequestCommentTarget,
    reader: NonZeroU64,
    repository: NonZeroU64,
    access_review: SpecContentHash,
    verified_at: Timestamp,
    deadline: Timestamp,
    origin: String,
}

impl VerifiedReadInstallationScope {
    pub(crate) fn inventory<'a>(
        &'a self,
        writer_scope: &'a SpecContentHash,
    ) -> super::super::IndependentCommentInventory<'a> {
        super::super::IndependentCommentInventory {
            origin: self.origin.clone(),
            agent: github_agent(),
            reader: self.reader,
            repository: self.repository,
            writer_scope,
            access_review: Some(self.access_review.clone()),
            bearer: self.bearer.secret_for_provider(),
            verified_installation: Some((&self.target, self.verified_at, self.deadline)),
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) fn exchange_read_installation_scope(
    scope: ReviewedReadInstallation<'_>,
    expected_access: &SpecContentHash,
    assertion: super::super::installation_signer_channel::InstallationAppAssertion,
) -> Option<VerifiedReadInstallationScope> {
    let (assertion, deadline) = assertion.into_parts();
    Exchange {
        agent: github_agent(),
        origin: GITHUB_ORIGIN,
    }
    .exchange_reader(
        scope,
        expected_access,
        &assertion,
        deadline,
        Timestamp::now_utc,
    )
}

impl Exchange<'_> {
    pub(super) fn exchange_reader(
        &self,
        scope: ReviewedReadInstallation<'_>,
        expected_access: &SpecContentHash,
        assertion: &str,
        assertion_deadline: Timestamp,
        now: impl Fn() -> Timestamp,
    ) -> Option<VerifiedReadInstallationScope> {
        self.adapter_commitment()?;
        let access_review = scope.access_commitment()?;
        let started = now();
        let issuance_deadline = scope.deadline.min(assertion_deadline);
        if &access_review != expected_access
            || started >= issuance_deadline
            || !valid_app_assertion(assertion)
        {
            return None;
        }
        let installation_path = format!("/app/installations/{}", scope.installation);
        let app: App = self.get("/app", assertion)?;
        if app.id != scope.app_id || app.slug != scope.app_slug {
            return None;
        }
        let installation = self
            .get::<ReadInstallation>(&installation_path, assertion)?
            .installation;
        if installation.id != scope.installation
            || installation.app_id != scope.app_id
            || !installation.suspended_at.is_null()
        {
            return None;
        }
        let before_issue = now();
        if before_issue < started || before_issue >= issuance_deadline {
            return None;
        }
        let request = serde_json::json!({
            "repository_ids": [scope.repository],
            "permissions": ReadPermissions::exact(),
        });
        let issued: IssuedReadAccess = decode(
            github_request(
                &self.agent,
                self.origin,
                "POST",
                &format!("{installation_path}/access_tokens"),
                assertion,
            )
            .set("Content-Type", "application/json")
            .send_string(&request.to_string())
            .ok()?,
            201,
        )?;
        let issued_at = now();
        let deadline = scope.deadline.min(issued.expires_at);
        if issued_at < before_issue
            || issued_at >= deadline
            || issued.expires_at.as_offset_date_time()
                > issued_at
                    .as_offset_date_time()
                    .checked_add(time::Duration::hours(1))?
            || !scope.matches_repo(&issued.repositories[0])
        {
            return None;
        }
        let bearer = GitHubPullRequestCommentProviderAuth::new(issued.token, None).ok()?;
        let response = github_request(
            &self.agent,
            self.origin,
            "GET",
            "/installation/repositories?per_page=2&page=1",
            bearer.secret_for_provider(),
        )
        .call()
        .ok()?;
        if !response.all("Link").is_empty() {
            return None;
        }
        let listed: InstallationRepositories = decode(response, 200)?;
        if listed.total_count != 1 || !scope.matches_repo(&listed.repositories[0]) {
            return None;
        }
        let bot: Bot = self.get(
            &format!("/user/{}", scope.reader),
            bearer.secret_for_provider(),
        )?;
        if bot.id != scope.reader || bot.login != format!("{}[bot]", scope.app_slug) {
            return None;
        }
        let verified_at = now();
        if verified_at < issued_at || verified_at >= deadline {
            return None;
        }
        Some(VerifiedReadInstallationScope {
            bearer,
            target: scope.target.clone(),
            reader: scope.reader,
            repository: scope.repository,
            access_review,
            verified_at,
            deadline,
            origin: self.origin.to_owned(),
        })
    }
}
