//! Fixed-origin installation scope exchange. It cannot authorize or send a comment.

use std::num::NonZeroU64;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    bounded_read, github_agent, github_request, GitHubRepositoryIdentity, GITHUB_ORIGIN,
    MAX_IDENTITY_RESPONSE_BYTES,
};
use crate::{GitHubPullRequestCommentProviderAuth, GitHubPullRequestCommentTarget, Timestamp};

#[derive(Clone, Copy)]
pub(super) struct ReviewedInstallation<'a> {
    pub(super) app_id: NonZeroU64,
    pub(super) installation_id: NonZeroU64,
    pub(super) repository_id: NonZeroU64,
    pub(super) target: &'a GitHubPullRequestCommentTarget,
    pub(super) deadline: Timestamp,
}

// No export, conversion to comment auth, or serialization. The next phase
// must bind verified bot identity and run authority before using this bearer.
pub(super) struct VerifiedInstallationScope<'a> {
    bearer: GitHubPullRequestCommentProviderAuth,
    scope: ReviewedInstallation<'a>,
    effective_deadline: Timestamp,
}

#[derive(Deserialize)]
struct App {
    id: NonZeroU64,
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

pub(super) fn exchange_installation_scope<'a>(
    scope: ReviewedInstallation<'a>,
    signed_app_assertion: &str,
) -> Option<VerifiedInstallationScope<'a>> {
    Exchange {
        agent: github_agent(),
        origin: GITHUB_ORIGIN,
    }
    .exchange(scope, signed_app_assertion, Timestamp::now_utc)
}

impl Exchange<'_> {
    fn exchange<'a>(
        &self,
        scope: ReviewedInstallation<'a>,
        app_assertion: &str,
        now: impl Fn() -> Timestamp,
    ) -> Option<VerifiedInstallationScope<'a>> {
        scope.target.validate().ok()?;
        let started = now();
        if started >= scope.deadline || !valid_app_assertion(app_assertion) {
            return None;
        }
        let installation_path = format!("/app/installations/{}", scope.installation_id);
        if self.get::<App>("/app", app_assertion)?.id != scope.app_id {
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
        if before_issue < started || before_issue >= scope.deadline {
            return None;
        }
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
            token,
            expires_at,
            repositories: [issued_repo],
            permissions:
                Permissions {
                    metadata: MetadataPermission::Read,
                    pull_requests: PullRequestPermission::Write,
                },
            repository_selection: Selection::Selected,
        } = issued;
        let after_issue = now();
        let effective_deadline = scope.deadline.min(expires_at);
        let upper_expiry = after_issue
            .as_offset_date_time()
            .checked_add(time::Duration::hours(1))?;
        if after_issue < before_issue
            || after_issue >= effective_deadline
            || expires_at.as_offset_date_time() > upper_expiry
            || !matches_repo(&issued_repo, scope)
        {
            return None;
        }
        let bearer = GitHubPullRequestCommentProviderAuth::new(token, None).ok()?;
        let InstallationRepositories {
            total_count,
            repositories: [listed],
        } = self.get(
            "/installation/repositories?per_page=2&page=1",
            bearer.secret_for_provider(),
        )?;
        let verified_at = now();
        if total_count != 1
            || !matches_repo(&listed, scope)
            || verified_at < after_issue
            || verified_at >= effective_deadline
        {
            return None;
        }
        Some(VerifiedInstallationScope {
            bearer,
            scope,
            effective_deadline,
        })
    }

    fn get<T: DeserializeOwned>(&self, path: &str, bearer: &str) -> Option<T> {
        let response = github_request(&self.agent, self.origin, "GET", path, bearer)
            .call()
            .ok()?;
        decode(response, 200)
    }
}

fn valid_app_assertion(assertion: &str) -> bool {
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
mod tests {
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
        body: r#"{"id":33}"#,
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
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("loopback");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let origin = format!("http://{}", listener.local_addr().expect("local address"));
        let target = GitHubPullRequestCommentTarget::new("example", "sandbox", 7).expect("target");
        let scope = ReviewedInstallation {
            app_id: NonZeroU64::new(33).expect("app"),
            installation_id: NonZeroU64::new(44).expect("installation"),
            repository_id: NonZeroU64::new(11).expect("repository"),
            target: &target,
            deadline: at(50),
        };
        thread::scope(|threads| {
            let server = threads.spawn(|| {
                for reply in replies {
                    serve(&listener, *reply);
                }
            });
            let verified = Exchange {
                agent: github_agent(),
                origin: &origin,
            }
            .exchange(scope, ASSERTION, now);
            assert_eq!(
                verified
                    .as_ref()
                    .map(|verified| verified.effective_deadline),
                expected
            );
            if let Some(verified) = verified {
                let issued_bearer_retained = verified.bearer.secret_for_provider() == ISSUED;
                assert!(issued_bearer_retained);
                assert_eq!(verified.scope.target, &target);
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

    fn serve(listener: &TcpListener, reply: Reply<'_>) {
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
        let expected_bearer = if reply.request == LIST.request {
            ISSUED
        } else {
            ASSERTION
        };
        let bearer_matches = auth.as_deref() == Some(format!("Bearer {expected_bearer}").as_str());
        assert!(bearer_matches, "unexpected fixture authorization");
        if reply.request == ISSUE.request {
            assert!(length <= 1024);
            let mut body = vec![0; length];
            reader.read_exact(&mut body).expect("issue request");
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&body).expect("JSON"),
                serde_json::json!({"repository_ids":[11],"permissions":{"metadata":"read","pull_requests":"write"}}),
            );
        } else {
            assert_eq!(length, 0);
        }
        write!(
            stream,
            "HTTP/1.1 {}\r\nConnection: close\r\n{}Content-Length: {}\r\n\r\n{}",
            reply.status,
            reply.header,
            reply.body.len(),
            reply.body,
        )
        .expect("fixture response");
    }

    #[test]
    fn comment_installation_exchange_uses_issued_bearer_for_exact_scope() {
        exercise(&[APP, INSTALLATION, ISSUE, LIST], || at(20), Some(at(40)));
    }

    #[test]
    fn comment_installation_exchange_rejects_wrong_app_or_broad_installation_before_issue() {
        exercise(
            &[Reply {
                body: r#"{"id":99}"#,
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
            &[APP, INSTALLATION, ISSUE, LIST],
            || {
                let sample = samples.get();
                samples.set(sample + 1);
                at(if sample == 3 { 40 } else { 20 })
            },
            None,
        );
    }
}
