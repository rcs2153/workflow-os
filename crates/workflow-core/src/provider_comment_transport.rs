//! Private, single-use GitHub comment exchange. Not connected to dispatch.
#![allow(dead_code)]

use std::{io::Read, num::NonZeroU64, time::Duration};

use serde::Deserialize;

use crate::live_sandbox_continuation_authority::LiveSandboxProviderBinding;
use crate::provider_comment_observation::{
    CreatedCommentResponse, MAX_CREATION_RESPONSE_BYTES, MAX_RAW_BODY_BYTES,
};
use crate::{GitHubPullRequestCommentProviderAuth, GitHubPullRequestCommentTarget};

const GITHUB_ORIGIN: &str = "https://api.github.com";
const MAX_IDENTITY_RESPONSE_BYTES: usize = 128 * 1024;

pub(crate) struct GitHubCommentCreationTransport<'a> {
    origin: String,
    agent: ureq::Agent,
    auth: &'a GitHubPullRequestCommentProviderAuth,
}

pub(crate) struct PreparedGitHubCommentCreation<'a> {
    transport: GitHubCommentCreationTransport<'a>,
    path: String,
    repository_id: NonZeroU64,
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

impl<'a> GitHubCommentCreationTransport<'a> {
    pub(crate) fn new(auth: &'a GitHubPullRequestCommentProviderAuth) -> Self {
        Self {
            origin: GITHUB_ORIGIN.to_owned(),
            agent: ureq::AgentBuilder::new()
                .try_proxy_from_env(false)
                .redirects(0)
                .max_idle_connections(0)
                .max_idle_connections_per_host(0)
                .timeout_connect(Duration::from_secs(5))
                .timeout(Duration::from_secs(15))
                .build(),
            auth,
        }
    }

    pub(crate) fn prepare(
        self,
        target: &GitHubPullRequestCommentTarget,
        expected: &LiveSandboxProviderBinding,
    ) -> Option<PreparedGitHubCommentCreation<'a>> {
        self.auth.validate().ok()?;
        target.validate().ok()?;
        let full_name = format!("{}/{}", target.owner(), target.repository());
        let repo = format!("/repos/{full_name}");
        let repository: GitHubRepositoryIdentity = self.read_identity(&repo)?;
        if repository.id != expected.repository_id || repository.full_name != full_name {
            return None;
        }
        let author: GitHubUserIdentity = self.read_identity("/user")?;
        if author.id != expected.author_id {
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
        Some(PreparedGitHubCommentCreation {
            transport: self,
            path: format!("{repo}/issues/{number}/comments"),
            repository_id: repository.id,
        })
    }

    fn request(&self, method: &str, path: &str) -> ureq::Request {
        self.agent
            .request(method, &format!("{}{path}", self.origin))
            .set(
                "Authorization",
                &format!("Bearer {}", self.auth.secret_for_provider()),
            )
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", "workflow-os-comment-sandbox")
            .set("X-GitHub-Api-Version", "2022-11-28")
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

impl PreparedGitHubCommentCreation<'_> {
    /// No response is evidence for another attempt. Only Core can authorize dispatch.
    pub(crate) fn send(self, raw_body: &str) -> Option<CreatedCommentResponse> {
        if raw_body.is_empty() || raw_body.len() > MAX_RAW_BODY_BYTES {
            return None;
        }
        let body = serde_json::json!({ "body": raw_body }).to_string();
        let response = self
            .transport
            .request("POST", &self.path)
            .set("Content-Type", "application/json")
            .set("Accept", "application/vnd.github.raw+json")
            .send_string(&body)
            .ok()?;
        if response.status() != 201 {
            return None;
        }
        let json = bounded_read(response.into_reader(), MAX_CREATION_RESPONSE_BYTES)?;
        Some(CreatedCommentResponse {
            status: 201,
            repository_id: self.repository_id,
            json,
        })
    }
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
mod tests {
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
        let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let origin = format!(
            "http://{}",
            listener.local_addr().expect("loopback address")
        );
        thread::scope(|scope| {
            let server = scope.spawn(move || {
                for reply in replies {
                    serve(&listener, reply);
                }
                thread::sleep(Duration::from_millis(100));
                assert!(matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock), "unexpected extra HTTP request");
            });
            let auth = GitHubPullRequestCommentProviderAuth::new("synthetic-only", None)
                .expect("fixture auth");
            let target =
                GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
            let binding = LiveSandboxProviderBinding {
                repository_id: NonZeroU64::new(11).expect("repository id"),
                author_id: NonZeroU64::new(22).expect("author id"),
                adapter_commitment: crate::SpecContentHash::from_text("loopback fixture"),
            };
            let result = GitHubCommentCreationTransport {
                origin,
                ..GitHubCommentCreationTransport::new(&auth)
            }
            .prepare(&target, &binding)
            .and_then(|prepared| prepared.send(MARKED_BODY));
            server.join().expect("fixture server");
            result
        })
    }

    fn serve(listener: &TcpListener, reply: &Reply<'_>) {
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
        if reply.line == TARGET {
            let mut body = vec![0; length];
            reader.read_exact(&mut body).expect("request body");
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&body).expect("request JSON"),
                serde_json::json!({"body": MARKED_BODY})
            );
        }
        if reply.status.is_empty() {
            return;
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
