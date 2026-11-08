//! Private GitHub comment exchange and bounded read-only observation.
#![allow(dead_code)]

pub(crate) mod installation_exchange;
#[cfg(test)]
mod installation_scope_fixture;

use std::{io::Read, num::NonZeroU64, time::Duration};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::live_sandbox_continuation_authority::LiveSandboxProviderBinding;
use crate::provider_comment_observation::{
    verify_current_comment_identity, CommentEnumeration, CommentPage, CreatedCommentResponse,
    CurrentCommentPosture, ExpectedComment, ProviderComment, ProviderTimestampBounds, MAX_COMMENTS,
    MAX_CREATION_RESPONSE_BYTES, MAX_PAGES, MAX_RAW_BODY_BYTES,
};
use crate::{
    GitHubPullRequestCommentProviderAuth, GitHubPullRequestCommentTarget, SpecContentHash,
    Timestamp,
};

const GITHUB_ORIGIN: &str = "https://api.github.com";
const GITHUB_API_VERSION: &str = "2022-11-28";
const CONNECT_SECONDS: u64 = 5;
const REQUEST_SECONDS: u64 = 15;
const MAX_IDENTITY_RESPONSE_BYTES: usize = 128 * 1024;
const MAX_COMMENT_PAGE_BYTES: usize = 512 * 1024;
const COMMENTS_PER_PAGE: usize = MAX_COMMENTS / MAX_PAGES;
const MAX_LINK_BYTES: usize = 4096;

pub(crate) struct GitHubCommentCreationTransport<'a> {
    origin: String,
    agent: ureq::Agent,
    auth: &'a GitHubPullRequestCommentProviderAuth,
}

pub(crate) struct PreparedGitHubCommentCreation<'a> {
    transport: GitHubCommentCreationTransport<'a>,
    path: String,
    target: GitHubPullRequestCommentTarget,
    binding: LiveSandboxProviderBinding,
}

#[derive(Deserialize)]
struct GitHubRepositoryIdentity {
    id: NonZeroU64,
    full_name: String,
}

#[derive(Deserialize)]
struct GitHubUserIdentity {
    id: NonZeroU64,
}

#[derive(Deserialize)]
struct GitHubPullRequestIdentity {
    number: NonZeroU64,
    base: GitHubPullRequestBase,
}

#[derive(Deserialize)]
struct GitHubPullRequestBase {
    repo: GitHubRepositoryIdentity,
}

#[derive(Deserialize)]
struct GitHubListedComment {
    id: NonZeroU64,
    url: String,
    issue_url: String,
    user: GitHubUserIdentity,
    body: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}

struct ObservedPage {
    cursor: Option<NonZeroU64>,
    next: Option<NonZeroU64>,
    comments: Vec<GitHubListedComment>,
}

struct PageLinks {
    next: Option<NonZeroU64>,
    last: Option<NonZeroU64>,
}

/// An exact current match from the fixed provider reader, not proof of creation.
/// No public constructor, deserialization, body, or authentication material.
/// No production constructor until trusted creation-time provenance is available.
pub(crate) struct VerifiedGitHubCommentObservation {
    projection: StoredGitHubCommentObservation,
}

/// Historical local record. Decoding it cannot mint a verifier witness.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StoredGitHubCommentObservation {
    reservation: SpecContentHash,
    context: SpecContentHash,
    comment_id: NonZeroU64,
    observed_at: Timestamp,
}

impl VerifiedGitHubCommentObservation {
    pub(crate) fn into_stored(self) -> StoredGitHubCommentObservation {
        self.projection
    }

    #[cfg(test)]
    pub(crate) fn test_witness(
        reservation: SpecContentHash,
        expected: &ExpectedComment<'_>,
        comment_id: NonZeroU64,
        observed_at: Timestamp,
    ) -> Option<Self> {
        Some(Self {
            projection: StoredGitHubCommentObservation {
                reservation,
                context: comment_observation_context(expected)?,
                comment_id,
                observed_at,
            },
        })
    }
}

impl StoredGitHubCommentObservation {
    pub(crate) fn matches(
        &self,
        reservation: &SpecContentHash,
        expected: &ExpectedComment<'_>,
    ) -> bool {
        &self.reservation == reservation
            && comment_observation_context(expected).as_ref() == Some(&self.context)
            && self.observed_at >= expected.not_before
            && !expected
                .known_comment_id
                .is_some_and(|id| id != self.comment_id)
    }

    pub(crate) const fn comment_id(&self) -> NonZeroU64 {
        self.comment_id
    }

    pub(crate) const fn observed_at(&self) -> Timestamp {
        self.observed_at
    }
}

pub(crate) fn fixed_comment_observer_commitment() -> SpecContentHash {
    SpecContentHash::from_text(
        "workflow-os/github-comment-observer/v1\0https://api.github.com\0api=2022-11-28\0stable-repository-pagination\0no-redirect-or-proxy",
    )
}

pub(crate) fn fixed_comment_exchange_commitment() -> SpecContentHash {
    SpecContentHash::from_text(&format!(
        "workflow-os/github-comment-exchange/v1\0{GITHUB_ORIGIN}\0{GITHUB_API_VERSION}\0{}\0POST:issue-comment:raw-json\0connect={CONNECT_SECONDS}\0request={REQUEST_SECONDS}\0response={MAX_CREATION_RESPONSE_BYTES}\0no-redirect-proxy-reuse",
        fixed_comment_observer_commitment(),
    ))
}

#[cfg(test)]
pub(crate) fn loopback_comment_exchange_commitment() -> SpecContentHash {
    SpecContentHash::from_text("workflow-os/github-comment-exchange/loopback-fixture/v1")
}

fn comment_observation_context(expected: &ExpectedComment<'_>) -> Option<SpecContentHash> {
    serde_json::to_vec(&(
        "workflow-os/github-comment-observation-context/v1",
        expected.target.reference(),
        expected.repository_id,
        expected.author_id,
        expected.known_comment_id,
        SpecContentHash::from_text(expected.body),
        expected.reservation_marker,
        expected.not_before,
        expected.not_after,
    ))
    .ok()
    .map(SpecContentHash::from_bytes)
}

impl<'a> GitHubCommentCreationTransport<'a> {
    pub(crate) fn new(auth: &'a GitHubPullRequestCommentProviderAuth) -> Self {
        Self {
            origin: GITHUB_ORIGIN.to_owned(),
            agent: github_agent(),
            auth,
        }
    }

    pub(crate) fn prepare(
        self,
        target: &GitHubPullRequestCommentTarget,
        expected: &LiveSandboxProviderBinding,
    ) -> Option<PreparedGitHubCommentCreation<'a>> {
        let adapter_commitment = self.creation_adapter_commitment()?;
        if adapter_commitment != expected.adapter_commitment {
            return None;
        }
        let (repository_id, author_id) =
            self.verify_target(target, expected.repository_id, expected.author_id)?;
        Some(PreparedGitHubCommentCreation {
            transport: self,
            path: comment_path(target),
            target: target.clone(),
            binding: LiveSandboxProviderBinding {
                repository_id,
                author_id,
                adapter_commitment,
            },
        })
    }

    fn creation_adapter_commitment(&self) -> Option<SpecContentHash> {
        if self.origin == GITHUB_ORIGIN {
            return Some(fixed_comment_exchange_commitment());
        }
        #[cfg(test)]
        {
            let origin = Url::parse(&self.origin).ok()?;
            if origin.scheme() == "http"
                && origin.host_str() == Some("127.0.0.1")
                && origin.port().is_some()
                && origin.username().is_empty()
                && origin.password().is_none()
                && origin.path() == "/"
                && origin.query().is_none()
                && origin.fragment().is_none()
            {
                return Some(loopback_comment_exchange_commitment());
            }
        }
        None
    }

    fn verify_target(
        &self,
        target: &GitHubPullRequestCommentTarget,
        repository_id: NonZeroU64,
        author_id: NonZeroU64,
    ) -> Option<(NonZeroU64, NonZeroU64)> {
        self.auth.validate().ok()?;
        target.validate().ok()?;
        let full_name = format!("{}/{}", target.owner(), target.repository());
        let repo = format!("/repos/{full_name}");
        let repository: GitHubRepositoryIdentity = self.read_identity(&repo)?;
        if repository.id != repository_id || repository.full_name != full_name {
            return None;
        }
        let author: GitHubUserIdentity = self.read_identity("/user")?;
        if author.id != author_id {
            return None;
        }
        // Issue-comment creation also accepts ordinary issues.
        let number = target.pull_request_number();
        let pull_request: GitHubPullRequestIdentity =
            self.read_identity(&format!("{repo}/pulls/{number}"))?;
        if pull_request.number.get() != number
            || pull_request.base.repo.id != repository.id
            || pull_request.base.repo.full_name != full_name
        {
            return None;
        }
        Some((repository.id, author.id))
    }

    /// Reads only the reservation's target. A provider snapshot or acceptance
    /// proof cannot be inferred from a complete traversal.
    #[cfg(test)]
    fn observe_comment(
        self,
        expected: &ExpectedComment<'_>,
        timestamp_bounds: Option<ProviderTimestampBounds>,
        observe_time: impl FnOnce() -> Option<Timestamp>,
    ) -> CurrentCommentPosture {
        self.read_exact_comment(expected, timestamp_bounds, observe_time)
            .map_or_else(
                |posture| posture,
                |_| CurrentCommentPosture::ExactCurrentMatch,
            )
    }

    fn read_exact_comment(
        &self,
        expected: &ExpectedComment<'_>,
        timestamp_bounds: Option<ProviderTimestampBounds>,
        observe_time: impl FnOnce() -> Option<Timestamp>,
    ) -> Result<(NonZeroU64, Timestamp), CurrentCommentPosture> {
        let pages = self
            .read_comment_pages(expected)
            .ok_or(CurrentCommentPosture::Unresolved)?;
        let observed_at = observe_time().ok_or(CurrentCommentPosture::Unresolved)?;
        let normalized = pages
            .iter()
            .map(|page| {
                page.comments
                    .iter()
                    .map(|row| ProviderComment {
                        target: expected.target,
                        repository_id: expected.repository_id,
                        author_id: row.user.id,
                        comment_id: row.id,
                        raw_body: &row.body,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let borrowed = pages
            .iter()
            .zip(&normalized)
            .map(|(page, comments)| CommentPage {
                request_cursor: page.cursor,
                next_cursor: page.next,
                comments,
            })
            .collect::<Vec<_>>();
        verify_current_comment_identity(
            expected,
            CommentEnumeration::Complete {
                observed_at,
                timestamp_bounds,
                pages: &borrowed,
            },
        )
        .map(|id| (id, observed_at))
    }

    fn read_comment_pages(&self, expected: &ExpectedComment<'_>) -> Option<Vec<ObservedPage>> {
        let target = expected.target;
        self.verify_target(target, expected.repository_id, expected.author_id)?;
        let origin = Url::parse(&self.origin).ok()?;
        let named = comment_path(target);
        let stable = format!(
            "/repositories/{}/issues/{}/comments",
            expected.repository_id,
            target.pull_request_number()
        );
        let issue = format!(
            "{GITHUB_ORIGIN}/repos/{}/{}/issues/{}",
            target.owner(),
            target.repository(),
            target.pull_request_number()
        );
        let comments = format!(
            "{GITHUB_ORIGIN}/repos/{}/{}/issues/comments",
            target.owner(),
            target.repository()
        );
        let mut pages = Vec::new();
        let mut previous_id = None;
        let mut advertised_last = None;
        let page_limit = u64::try_from(MAX_PAGES).ok()?;
        for page in 1..=page_limit {
            let request_path = format!("{stable}?per_page={COMMENTS_PER_PAGE}&page={page}");
            let response = self
                .request("GET", &request_path)
                .set("Accept", "application/vnd.github.raw+json")
                .call()
                .ok()?;
            if response.status() != 200 {
                return None;
            }
            let links = response.all("Link");
            if response
                .headers_names()
                .iter()
                .filter(|name| name.eq_ignore_ascii_case("Link"))
                .count()
                != links.len()
            {
                return None;
            }
            let PageLinks { next, last } =
                comment_page_links(&links, &origin, &named, &stable, page)?;
            if let Some(last) = last {
                if last.get() > page_limit || advertised_last.is_some_and(|prior| prior != last) {
                    return None;
                }
                advertised_last = Some(last);
            }
            if advertised_last
                .is_some_and(|last| last.get() < page || (last.get() > page) != next.is_some())
            {
                return None;
            }
            let bytes = bounded_read(response.into_reader(), MAX_COMMENT_PAGE_BYTES)?;
            let rows: Vec<GitHubListedComment> = serde_json::from_slice(&bytes).ok()?;
            if rows.len() > COMMENTS_PER_PAGE
                || (next.is_some() && rows.len() != COMMENTS_PER_PAGE)
                || (page > 1 && rows.is_empty())
            {
                return None;
            }
            for row in &rows {
                if row.issue_url != issue
                    || row.url != format!("{comments}/{}", row.id)
                    || row.body.len() > MAX_RAW_BODY_BYTES
                    || previous_id.is_some_and(|prior| row.id <= prior)
                {
                    return None;
                }
                previous_id = Some(row.id);
            }
            pages.push(ObservedPage {
                cursor: (page > 1).then(|| NonZeroU64::new(page)).flatten(),
                next,
                comments: rows,
            });
            if next.is_none() {
                return Some(pages);
            }
        }
        None
    }

    fn request(&self, method: &str, path: &str) -> ureq::Request {
        github_request(
            &self.agent,
            &self.origin,
            method,
            path,
            self.auth.secret_for_provider(),
        )
    }

    fn read_identity<T: serde::de::DeserializeOwned>(&self, path: &str) -> Option<T> {
        let response = self.request("GET", path).call().ok()?;
        if response.status() != 200 {
            return None;
        }
        let body = bounded_read(response.into_reader(), MAX_IDENTITY_RESPONSE_BYTES)?;
        serde_json::from_slice(&body).ok()
    }
}

fn github_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .try_proxy_from_env(false)
        .redirects(0)
        .max_idle_connections(0)
        .max_idle_connections_per_host(0)
        .timeout_connect(Duration::from_secs(CONNECT_SECONDS))
        .timeout(Duration::from_secs(REQUEST_SECONDS))
        .build()
}

fn github_request(
    agent: &ureq::Agent,
    origin: &str,
    method: &str,
    path: &str,
    bearer: &str,
) -> ureq::Request {
    agent
        .request(method, &format!("{origin}{path}"))
        .set("Authorization", &format!("Bearer {bearer}"))
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "workflow-os-comment-sandbox")
        .set("X-GitHub-Api-Version", GITHUB_API_VERSION)
}

fn comment_path(target: &GitHubPullRequestCommentTarget) -> String {
    format!(
        "/repos/{}/{}/issues/{}/comments",
        target.owner(),
        target.repository(),
        target.pull_request_number()
    )
}

// Accept only GitHub's simple pagination links. Rebuild the request from
// the validated page number instead of following any supplied URL.
fn comment_page_links(
    headers: &[&str],
    origin: &Url,
    named: &str,
    stable: &str,
    page: u64,
) -> Option<PageLinks> {
    let mut budget = MAX_LINK_BYTES;
    let mut seen = 0u8;
    let mut next = None;
    let mut last = None;
    for header in headers {
        budget = budget.checked_sub(header.len())?;
        for link in header.split(',') {
            let (url, relation) = link.trim().strip_prefix('<')?.split_once('>')?;
            let relation = relation.trim().strip_prefix(';')?.trim();
            let url = Url::parse(url).ok()?;
            if url.origin() != origin.origin()
                || !url.username().is_empty()
                || url.password().is_some()
                || url.fragment().is_some()
                || (url.path() != named && url.path() != stable)
            {
                return None;
            }
            let mut number = None;
            let mut per_page = false;
            for (key, value) in url.query_pairs() {
                match key.as_ref() {
                    "page" if number.is_none() => number = Some(value.parse::<NonZeroU64>().ok()?),
                    "per_page"
                        if !per_page && value.parse::<usize>().ok()? == COMMENTS_PER_PAGE =>
                    {
                        per_page = true;
                    }
                    _ => return None,
                }
            }
            let number = number.filter(|_| per_page)?;
            let bit = match relation {
                "rel=\"next\"" if number.get() == page.checked_add(1)? => {
                    next = Some(number);
                    1
                }
                "rel=\"last\"" if number.get() >= page => {
                    last = Some(number);
                    2
                }
                "rel=\"first\"" if number.get() == 1 => 4,
                "rel=\"prev\"" if number.get().checked_add(1)? == page => 8,
                _ => return None,
            };
            if seen & bit != 0 {
                return None;
            }
            seen |= bit;
        }
    }
    if last.is_some_and(|last| (last.get() > page) != next.is_some()) {
        return None;
    }
    Some(PageLinks { next, last })
}

impl PreparedGitHubCommentCreation<'_> {
    pub(crate) fn binding_for(
        &self,
        target: &GitHubPullRequestCommentTarget,
    ) -> Option<&LiveSandboxProviderBinding> {
        (&self.target == target).then_some(&self.binding)
    }

    /// No response is evidence for another attempt. Only Core can authorize dispatch.
    pub(crate) fn send(
        self,
        raw_body: &str,
        window: CommentSendWindow,
    ) -> Option<CreatedCommentResponse> {
        self.send_with_clock(raw_body, window, Timestamp::now_utc)
    }

    #[cfg(test)]
    pub(crate) fn send_with_test_clock(
        self,
        raw_body: &str,
        window: CommentSendWindow,
        observe: impl FnOnce() -> Timestamp,
    ) -> Option<CreatedCommentResponse> {
        self.send_with_clock(raw_body, window, observe)
    }

    fn send_with_clock(
        self,
        raw_body: &str,
        window: CommentSendWindow,
        observe: impl FnOnce() -> Timestamp,
    ) -> Option<CreatedCommentResponse> {
        if raw_body.is_empty() || raw_body.len() > MAX_RAW_BODY_BYTES {
            return None;
        }
        let body = serde_json::json!({ "body": raw_body }).to_string();
        let request = self
            .transport
            .request("POST", &self.path)
            .set("Content-Type", "application/json")
            .set("Accept", "application/vnd.github.raw+json");
        // This limits the local HTTP handoff, not provider-side execution.
        let now = observe();
        if now < window.checked_at || now >= window.deadline {
            return None;
        }
        let response = request.send_string(&body).ok()?;
        if response.status() != 201 {
            return None;
        }
        let json = bounded_read(response.into_reader(), MAX_CREATION_RESPONSE_BYTES)?;
        Some(CreatedCommentResponse {
            status: 201,
            repository_id: self.binding.repository_id,
            json,
        })
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CommentSendWindow {
    pub(crate) checked_at: Timestamp,
    pub(crate) deadline: Timestamp,
}

fn bounded_read(reader: impl Read, max: usize) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    reader
        .take(u64::try_from(max).ok()?.checked_add(1)?)
        .read_to_end(&mut bytes)
        .ok()?;
    (bytes.len() <= max).then_some(bytes)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
pub(crate) mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::thread;
    use std::time::Instant;

    const REPO: &[u8] = br#"{"id":11,"full_name":"example/sandbox"}"#;
    const USER: &[u8] = br#"{"id":22}"#;
    const PR: &[u8] = br#"{"number":7,"base":{"repo":{"id":11,"full_name":"example/sandbox"}}}"#;
    const TARGET: &str = "POST /repos/example/sandbox/issues/7/comments HTTP/1.1";
    const MARKED_BODY: &str = "exact\r\nbody";

    struct Reply<'a> {
        line: &'a str,
        status: &'a str,
        headers: &'a str,
        body: &'a [u8],
    }

    fn exchange(replies: &[Reply<'_>]) -> Option<CreatedCommentResponse> {
        exchange_with_binding(replies, loopback_comment_exchange_commitment())
    }

    fn exchange_with_binding(
        replies: &[Reply<'_>],
        adapter_commitment: SpecContentHash,
    ) -> Option<CreatedCommentResponse> {
        exchange_at(replies, adapter_commitment, observation_time(10))
    }

    fn exchange_at(
        replies: &[Reply<'_>],
        adapter_commitment: SpecContentHash,
        observed_at: Timestamp,
    ) -> Option<CreatedCommentResponse> {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let origin = format!(
            "http://{}",
            listener.local_addr().expect("loopback address")
        );
        thread::scope(|scope| {
            let server = scope.spawn(|| {
                for reply in replies {
                    if let Some(body) = serve(&listener, reply) {
                        assert_eq!(body, MARKED_BODY);
                    }
                }
            });
            let auth = GitHubPullRequestCommentProviderAuth::new("synthetic-only", None)
                .expect("fixture auth");
            let target =
                GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
            let binding = LiveSandboxProviderBinding {
                repository_id: NonZeroU64::new(11).expect("repository id"),
                author_id: NonZeroU64::new(22).expect("author id"),
                adapter_commitment,
            };
            let result = GitHubCommentCreationTransport {
                origin,
                ..GitHubCommentCreationTransport::new(&auth)
            }
            .prepare(&target, &binding)
            .and_then(|prepared| {
                prepared.send_with_test_clock(
                    MARKED_BODY,
                    CommentSendWindow {
                        checked_at: observation_time(10),
                        deadline: observation_time(20),
                    },
                    || observed_at,
                )
            });
            server.join().expect("fixture server");
            assert!(
                matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock),
                "unexpected extra HTTP request"
            );
            result
        })
    }

    fn serve(listener: &TcpListener, reply: &Reply<'_>) -> Option<String> {
        let start = Instant::now();
        let accepted = loop {
            match listener.accept() {
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        && start.elapsed() < Duration::from_secs(5) =>
                {
                    thread::sleep(Duration::from_millis(5));
                }
                result => break result,
            }
        };
        let (mut stream, _) = accepted.expect("fixture HTTP request");
        stream
            .set_nonblocking(false)
            .expect("blocking fixture stream");
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("fixture read timeout");
        let mut reader = BufReader::new(&mut stream);
        let mut line = String::new();
        reader.read_line(&mut line).expect("request line");
        assert_eq!(line.trim_end(), reply.line);
        let mut length = 0;
        let mut authorized = false;
        loop {
            line.clear();
            assert_ne!(reader.read_line(&mut line).expect("header line"), 0);
            if line == "\r\n" {
                break;
            }
            let (name, value) = line.split_once(':').expect("header");
            if name.eq_ignore_ascii_case("authorization") {
                authorized = value.trim() == "Bearer synthetic-only";
            } else if name.eq_ignore_ascii_case("content-length") {
                length = value.trim().parse().expect("body length");
            }
        }
        assert!(authorized);
        let posted = if reply.line == TARGET {
            let mut body = vec![0; length];
            reader.read_exact(&mut body).expect("request body");
            let json: serde_json::Value = serde_json::from_slice(&body).expect("request JSON");
            Some(json["body"].as_str().expect("comment body").to_owned())
        } else {
            None
        };
        if reply.status.is_empty() {
            return posted;
        } // Drop the accepted request without replying.
        write!(
            stream,
            "HTTP/1.1 {}\r\nConnection: close\r\nContent-Length: {}\r\n{}\r\n",
            reply.status,
            reply.body.len(),
            reply.headers
        )
        .expect("response headers");
        // An over-limit response can be dropped while the fixture is still writing.
        let _ = stream.write_all(reply.body);
        posted
    }

    pub(crate) fn with_lost_acknowledgement_server<T>(
        provider: &LiveSandboxProviderBinding,
        expect_post: bool,
        exercise: impl FnOnce(GitHubCommentCreationTransport<'_>) -> T,
    ) -> (T, Option<String>) {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let origin = format!(
            "http://{}",
            listener.local_addr().expect("loopback address")
        );
        let repo_body =
            serde_json::json!({"id": provider.repository_id, "full_name": "example/sandbox"});
        let pr_body = serde_json::json!({"number": 7, "base": {"repo": repo_body}});
        let user_body = serde_json::json!({"id": provider.author_id});
        let auth = GitHubPullRequestCommentProviderAuth::new("synthetic-only", None)
            .expect("fixture auth");
        thread::scope(|scope| {
            let server = scope.spawn(|| {
                assert!(serve(&listener, &repo(repo_body.to_string().as_bytes())).is_none());
                assert!(serve(&listener, &user(user_body.to_string().as_bytes())).is_none());
                assert!(serve(
                    &listener,
                    &pull_request("200 OK", pr_body.to_string().as_bytes())
                )
                .is_none());
                expect_post.then(|| {
                    serve(
                        &listener,
                        &Reply {
                            line: TARGET,
                            status: "",
                            headers: "",
                            body: &[],
                        },
                    )
                    .expect("one accepted POST")
                })
            });
            let result = exercise(GitHubCommentCreationTransport {
                origin,
                ..GitHubCommentCreationTransport::new(&auth)
            });
            let body = server.join().expect("fixture server");
            assert!(
                matches!(
                    listener.accept(),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock
                ),
                "unexpected request before the caller returned"
            );
            (result, body)
        })
    }

    fn repo(body: &[u8]) -> Reply<'_> {
        Reply {
            line: "GET /repos/example/sandbox HTTP/1.1",
            status: "200 OK",
            headers: "",
            body,
        }
    }

    fn user(body: &[u8]) -> Reply<'_> {
        Reply {
            line: "GET /user HTTP/1.1",
            status: "200 OK",
            headers: "",
            body,
        }
    }

    fn pull_request<'a>(status: &'a str, body: &'a [u8]) -> Reply<'a> {
        Reply {
            line: "GET /repos/example/sandbox/pulls/7 HTTP/1.1",
            status,
            headers: "",
            body,
        }
    }

    const MARKER: &str =
        "workflow-os/live-comment/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const STABLE_PATH: &str = "/repositories/11/issues/7/comments";
    type ObservationReply = (&'static str, String, Vec<u8>);

    fn observation_time(seconds: i64) -> Timestamp {
        Timestamp::from_offset_date_time(
            time::OffsetDateTime::from_unix_timestamp(seconds).expect("test clock"),
        )
    }

    fn observation_exchange(
        replies: impl FnOnce(&str) -> Vec<ObservationReply>,
        bounds: Option<ProviderTimestampBounds>,
    ) -> CurrentCommentPosture {
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener.set_nonblocking(true).expect("nonblocking");
        let origin = format!("http://{}", listener.local_addr().expect("address"));
        let replies = replies(&origin);
        let auth = GitHubPullRequestCommentProviderAuth::new("synthetic-only", None)
            .expect("fixture auth");
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let expected = ExpectedComment {
            target: &target,
            repository_id: NonZeroU64::new(11).expect("repository ID"),
            author_id: NonZeroU64::new(22).expect("author ID"),
            known_comment_id: None,
            body: "approved",
            reservation_marker: MARKER,
            not_before: observation_time(10),
            not_after: observation_time(20),
        };
        thread::scope(|scope| {
            let server = scope.spawn(|| {
                assert!(serve(&listener, &repo(REPO)).is_none());
                assert!(serve(&listener, &user(USER)).is_none());
                assert!(serve(&listener, &pull_request("200 OK", PR)).is_none());
                for (index, (status, headers, body)) in replies.iter().enumerate() {
                    let line = format!(
                        "GET {STABLE_PATH}?per_page={COMMENTS_PER_PAGE}&page={} HTTP/1.1",
                        index + 1,
                    );
                    assert!(serve(
                        &listener,
                        &Reply {
                            line: &line,
                            status,
                            headers,
                            body,
                        }
                    )
                    .is_none());
                }
            });
            let transport = GitHubCommentCreationTransport {
                origin,
                ..GitHubCommentCreationTransport::new(&auth)
            };
            let posture =
                transport.observe_comment(&expected, bounds, || Some(observation_time(21)));
            server.join().expect("read-only fixture");
            assert!(
                matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
            );
            posture
        })
    }

    const EXACT_TEST_CLOCK: Option<ProviderTimestampBounds> = Some(ProviderTimestampBounds {
        earliest_correction: time::Duration::ZERO,
        latest_correction: time::Duration::ZERO,
    });

    #[test]
    fn comment_observation_witness_rejects_context_substitution() {
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let reservation = SpecContentHash::from_text("original reservation");
        let expected = ExpectedComment {
            target: &target,
            repository_id: NonZeroU64::new(11).expect("repository ID"),
            author_id: NonZeroU64::new(22).expect("author ID"),
            known_comment_id: None,
            body: "approved",
            reservation_marker: MARKER,
            not_before: observation_time(10),
            not_after: observation_time(20),
        };
        let witness = StoredGitHubCommentObservation {
            reservation: reservation.clone(),
            context: comment_observation_context(&expected).expect("context"),
            comment_id: NonZeroU64::new(44).expect("comment ID"),
            observed_at: observation_time(21),
        };
        assert!(witness.matches(&reservation, &expected));
        assert!(!witness.matches(
            &SpecContentHash::from_text("different reservation"),
            &expected
        ));
        for changed in [
            ExpectedComment {
                body: "changed body",
                ..expected
            },
            ExpectedComment {
                not_after: observation_time(22),
                ..expected
            },
            ExpectedComment {
                repository_id: NonZeroU64::new(12).expect("other repository"),
                ..expected
            },
        ] {
            assert!(!witness.matches(&reservation, &changed));
        }
        assert_eq!(witness.comment_id().get(), 44);
        assert_eq!(witness.observed_at(), observation_time(21));
    }

    fn listed_comment(id: usize, body: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "url": format!("{GITHUB_ORIGIN}/repos/example/sandbox/issues/comments/{id}"),
            "issue_url": format!("{GITHUB_ORIGIN}/repos/example/sandbox/issues/7"),
            "user": {"id": 22},
            "body": body,
            "created_at": "1970-01-01T00:00:15Z",
            "updated_at": "1970-01-01T00:00:15Z",
        })
    }

    fn full_page(first: usize) -> Vec<u8> {
        let records = (first..first + COMMENTS_PER_PAGE)
            .map(|id| listed_comment(id, "unrelated"))
            .collect::<Vec<_>>();
        serde_json::to_vec(&records).expect("page")
    }

    fn next_link(origin: &str, path: &str, next: usize) -> String {
        format!("Link: <{origin}{path}?per_page={COMMENTS_PER_PAGE}&page={next}>; rel=\"next\"\r\n")
    }

    #[test]
    fn comment_observer_requires_complete_pages_and_a_trusted_time_bound() {
        let first = |origin: &str| ("200 OK", next_link(origin, STABLE_PATH, 2), full_page(1));
        let exact = crate::provider_comment_observation::encode_marked_comment("approved", MARKER)
            .expect("marked body");
        let candidate = serde_json::to_vec(&[listed_comment(COMMENTS_PER_PAGE + 1, &exact)])
            .expect("last page");
        for (bounds, posture) in [
            (EXACT_TEST_CLOCK, CurrentCommentPosture::ExactCurrentMatch),
            (None, CurrentCommentPosture::Unresolved),
        ] {
            assert_eq!(
                observation_exchange(
                    |origin| vec![first(origin), ("200 OK", String::new(), candidate.clone()),],
                    bounds
                ),
                posture
            );
        }
        assert_eq!(
            observation_exchange(
                |origin| vec![first(origin), ("403 Forbidden", String::new(), Vec::new()),],
                EXACT_TEST_CLOCK
            ),
            CurrentCommentPosture::Unresolved
        );
        let duplicate = serde_json::to_vec(&[listed_comment(COMMENTS_PER_PAGE, &exact)])
            .expect("overlapping page");
        assert_eq!(
            observation_exchange(
                |origin| vec![first(origin), ("200 OK", String::new(), duplicate),],
                EXACT_TEST_CLOCK
            ),
            CurrentCommentPosture::Unresolved
        );
    }

    #[test]
    fn comment_observer_rejects_untrusted_links_and_never_requests_page_nine() {
        let paths = [
            "/repositories/12/issues/7/comments",
            "/repositories/11/issues/8/comments",
        ];
        for path in paths {
            assert_eq!(
                observation_exchange(
                    |origin| vec![("200 OK", next_link(origin, path, 2), full_page(1)),],
                    EXACT_TEST_CLOCK
                ),
                CurrentCommentPosture::Unresolved
            );
        }
        for link in [
            next_link("https://example.invalid", STABLE_PATH, 2),
            "Link: invalid\r\n".to_owned(),
        ] {
            assert_eq!(
                observation_exchange(|_| vec![("200 OK", link, full_page(1))], EXACT_TEST_CLOCK),
                CurrentCommentPosture::Unresolved
            );
        }
        assert_eq!(
            observation_exchange(
                |origin| (0..MAX_PAGES)
                    .map(|page| (
                        "200 OK",
                        next_link(origin, STABLE_PATH, page + 2),
                        full_page(page * COMMENTS_PER_PAGE + 1),
                    ))
                    .collect(),
                EXACT_TEST_CLOCK
            ),
            CurrentCommentPosture::Unresolved
        );
    }

    #[test]
    fn comment_observer_rejects_a_truncated_advertised_chain() {
        assert_eq!(
            observation_exchange(
                |origin| {
                    vec![
                    ("200 OK", format!(
                        "Link: <{origin}{STABLE_PATH}?page=2&per_page={COMMENTS_PER_PAGE}>; rel=\"next\", <{origin}{STABLE_PATH}?page=3&per_page={COMMENTS_PER_PAGE}>; rel=\"last\"\r\n"
                    ), full_page(1)),
                    ("200 OK", String::new(), full_page(COMMENTS_PER_PAGE + 1)),
                ]
                },
                EXACT_TEST_CLOCK,
            ),
            CurrentCommentPosture::Unresolved,
        );
    }

    #[test]
    fn comment_observer_never_turns_invalid_response_into_absence() {
        let check = |body| {
            observation_exchange(|_| vec![("200 OK", String::new(), body)], EXACT_TEST_CLOCK)
        };
        assert_eq!(check(b"[]".to_vec()), CurrentCommentPosture::Absent);
        assert_eq!(
            check(b"not JSON".to_vec()),
            CurrentCommentPosture::Unresolved
        );
        assert_eq!(
            check(vec![b' '; MAX_COMMENT_PAGE_BYTES + 1]),
            CurrentCommentPosture::Unresolved
        );
        let mut row = listed_comment(1, "other");
        row["issue_url"] = format!("{GITHUB_ORIGIN}/repos/example/sandbox/issues/8").into();
        assert_eq!(
            check(serde_json::to_vec(&[row]).expect("wrong issue")),
            CurrentCommentPosture::Unresolved
        );
    }

    #[test]
    fn verified_identity_allows_one_exact_exchange() {
        let body = br#"{"id":33}"#;
        let result = exchange(&[
            repo(REPO),
            user(USER),
            pull_request("200 OK", PR),
            Reply {
                line: TARGET,
                status: "201 Created",
                headers: "",
                body,
            },
        ])
        .expect("response");
        assert_eq!(result.status, 201);
        assert_eq!(result.repository_id.get(), 11);
        assert_eq!(result.json, body);
    }

    #[test]
    fn comment_creation_adapter_substitution_rejects_before_http() {
        let creation = fixed_comment_exchange_commitment();
        let read_only = fixed_comment_observer_commitment();
        assert_ne!(creation, read_only);
        assert_ne!(creation, loopback_comment_exchange_commitment());
        for wrong in [
            creation,
            read_only,
            SpecContentHash::from_text("other adapter"),
        ] {
            assert!(exchange_with_binding(&[], wrong).is_none());
        }
    }

    #[test]
    fn comment_creation_rejects_elapsed_or_regressed_handoff_before_post() {
        for seconds in [9, 20, 21] {
            assert!(exchange_at(
                &[repo(REPO), user(USER), pull_request("200 OK", PR)],
                loopback_comment_exchange_commitment(),
                observation_time(seconds),
            )
            .is_none());
        }
    }

    #[test]
    fn wrong_identity_never_posts() {
        assert!(exchange(&[repo(br#"{"id":12,"full_name":"example/sandbox"}"#)]).is_none());
        assert!(exchange(&[repo(br#"{"id":11,"full_name":"example/other"}"#)]).is_none());
        assert!(exchange(&[repo(REPO), user(br#"{"id":23}"#)]).is_none());
    }

    #[test]
    fn missing_unverified_or_oversized_pull_request_never_posts() {
        let large = vec![b' '; MAX_IDENTITY_RESPONSE_BYTES + 1];
        for response in [
            pull_request("404 Not Found", br#"{"message":"Not Found"}"#),
            pull_request("200 OK", br#"{"number":7}"#),
            pull_request(
                "200 OK",
                br#"{"number":8,"base":{"repo":{"id":11,"full_name":"example/sandbox"}}}"#,
            ),
            pull_request(
                "200 OK",
                br#"{"number":7,"base":{"repo":{"id":12,"full_name":"example/sandbox"}}}"#,
            ),
            pull_request(
                "200 OK",
                br#"{"number":7,"base":{"repo":{"id":11,"full_name":"example/other"}}}"#,
            ),
            pull_request("200 OK", &large),
        ] {
            assert!(exchange(&[repo(REPO), user(USER), response]).is_none());
        }
    }

    #[test]
    fn ambiguous_post_responses_do_not_redirect_or_retry() {
        let large = vec![b' '; MAX_CREATION_RESPONSE_BYTES + 1];
        for reply in [
            Reply {
                line: TARGET,
                status: "307 Temporary Redirect",
                headers: "Location: /retry\r\n",
                body: &[],
            },
            Reply {
                line: TARGET,
                status: "",
                headers: "",
                body: &[],
            },
            Reply {
                line: TARGET,
                status: "201 Created",
                headers: "",
                body: &large,
            },
        ] {
            assert!(
                exchange(&[repo(REPO), user(USER), pull_request("200 OK", PR), reply]).is_none()
            );
        }
    }
}
