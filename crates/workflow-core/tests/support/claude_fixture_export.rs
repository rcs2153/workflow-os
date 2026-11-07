//! Opt-in evaluator wiring, not a production fixture or dispatch command.

use std::fs::{self, DirBuilder, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};
use workflow_core::local_fixture_replacement_authority::LocalFixturePrincipalVerifier;
use workflow_core::{
    ActorId, ApprovalDecisionKind, GitHubPrCommentFixtureContinuationValidationRequest,
    GitHubPullRequestCommentTarget, HarnessContractId, LocalApprovalPresentationDecisionRequest,
    LocalApprovalPresentationProof, LocalExecutor, SideEffectId, SideEffectRecordStore,
    SideEffectReference, SideEffectReferenceKind, SqliteStateBackend, Timestamp, WorkflowRunEvent,
    WorkflowRunEventKind, WorkflowRunId, WorkflowRunStatus,
};
use workflow_core::{ApprovalPresentationRecordStore, EventLogStore};

use super::{
    approval_presentation_record_for_action, exact_fixture_grant, fixture_cli_input,
    github_pr_comment_attempted_record_with_id, github_pr_comment_fixture_continuation_request,
    TestProject,
};

type Case<'a> = (&'static str, &'a str, &'static str, Value);

enum Variant {
    Baseline,
    ChangedEvidence,
    StaleWindow,
}

pub(super) struct Export {
    kernel: PathBuf,
    root: PathBuf,
    published: bool,
    variant: Variant,
}

impl Drop for Export {
    fn drop(&mut self) {
        if !self.published {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

impl Export {
    pub(super) fn from_environment() -> Option<Self> {
        let kernel = PathBuf::from(std::env::var_os("WORKFLOW_OS_CLAUDE_FIXTURE_KERNEL")?);
        let variant = match std::env::var("WORKFLOW_OS_CLAUDE_FIXTURE_VARIANT").as_deref() {
            Err(std::env::VarError::NotPresent) => Some(Variant::Baseline),
            Ok("changed-evidence") => Some(Variant::ChangedEvidence),
            Ok("stale-window") => Some(Variant::StaleWindow),
            _ => None,
        }
        .expect("known evaluator fixture variant");
        assert!(
            kernel.is_absolute() && kernel.is_file(),
            "expected an absolute kernel executable"
        );
        let parent = fs::canonicalize(std::env::temp_dir()).expect("scratch parent");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert!(!parent.starts_with(fs::canonicalize(repo).expect("repo root")));
        let nonce = u128::from_be_bytes(random_bytes());
        let root = parent.join(format!("workflow-os-claude-fixture-{nonce:032x}"));
        DirBuilder::new()
            .mode(0o700)
            .create(&root)
            .expect("fresh private scratch directory");
        Some(Self {
            kernel,
            root,
            published: false,
            variant,
        })
    }

    pub(super) fn root(&self) -> &Path {
        &self.root
    }

    pub(super) fn write(
        self,
        backend: &SqliteStateBackend,
        executor: &LocalExecutor<'_, SqliteStateBackend>,
        database: &Path,
        request: GitHubPrCommentFixtureContinuationValidationRequest<'_>,
        project: &TestProject,
        later_check: &WorkflowRunEvent,
    ) {
        let principal = ActorId::new("agent/claude-desktop").expect("fixture principal");
        let request = GitHubPrCommentFixtureContinuationValidationRequest {
            replacement_actor: &principal,
            presentation_policy: request
                .presentation_policy
                .with_max_presentation_age(std::time::Duration::from_secs(300)),
            ..request
        };
        let decision = request
            .run
            .events
            .iter()
            .find_map(|event| match &event.kind {
                WorkflowRunEventKind::ApprovalGranted(decision) => Some(decision),
                _ => None,
            })
            .expect("proof-backed original approval");
        let proof = random_bytes::<32>();
        let mut wrong = proof;
        wrong[0] ^= 1;
        backend
            .register_local_fixture_principal(&LocalFixturePrincipalVerifier::new(
                principal.clone(),
                HarnessContractId::new("harness/claude-desktop").expect("fixture harness"),
                &proof,
            ))
            .expect("register distinct fixture principal");

        let issue = |id, lifetime| {
            let grant = exact_fixture_grant(id, &request, decision, lifetime);
            workflow_core::issue_local_fixture_replacement_grant(
                executor,
                &request,
                &decision.actor,
                &grant,
            )
            .expect("issue exact fixture grant");
            grant
        };
        let exact = issue("grant/claude-exact", 210);
        let envelope = fixture_cli_input(&request.write_request);
        if !matches!(self.variant, Variant::Baseline) {
            let grant = exact.grant_id().as_str();
            assert!(self.invoke(grant, &proof, &envelope).status.success());
            let (id, diagnostic) = self.change_state(backend, executor, request, later_check);
            let before = fs::read(database).expect("database before rejection");
            assert_rejected(&self.invoke(grant, &proof, &envelope), diagnostic);
            assert_eq!(before, fs::read(database).expect("database unchanged"));
            self.publish(&[(id, grant, "principal", envelope)], &proof, &wrong);
            return;
        }
        let revoked = issue("grant/claude-revoked", 210);
        backend
            .revoke_local_fixture_grant(revoked.grant_id())
            .expect("revoke fixture grant");
        let expired = issue("grant/claude-expired", 3);

        let mut cases = fixture_cases(
            envelope.clone(),
            request.write_request.target(),
            exact.grant_id().as_str(),
            revoked.grant_id().as_str(),
            expired.grant_id().as_str(),
        )
        .to_vec();
        cases.extend(terminal_cases(
            backend,
            executor,
            project,
            &request,
            &envelope,
            exact.grant_id().as_str(),
        ));
        self.verify_cases(database, &cases, &proof, &wrong);
        println!(
            "claude_fixture_exact_grant_expires: {}",
            exact.expires_at().expect("bounded expiry")
        );
        self.publish(&cases, &proof, &wrong);
    }

    fn publish(mut self, cases: &[Case<'_>], proof: &[u8; 32], wrong: &[u8; 32]) {
        private_write(&self.root.join("principal"), proof);
        private_write(&self.root.join("wrong-principal"), wrong);
        // Actor overrides are a CLI-boundary probe, not a valid MCP fixture envelope.
        let cases: Vec<_> = cases
            .iter()
            .filter(|(id, _, _, _)| *id != "forged-actor")
            .map(|(id, grant, principal_file, input)| {
                json!({ "id": id, "grant": grant, "principal_file": principal_file, "input": input })
            })
            .collect();
        let manifest = self.root.join("manifest.json");
        private_write(
            &manifest,
            &serde_json::to_vec_pretty(&json!({
                "kernel": self.kernel, "project": self.root, "state": self.root, "cases": cases,
            }))
            .expect("operator manifest"),
        );
        self.published = true;
        println!("claude_fixture_manifest: {}", manifest.display());
    }

    fn change_state(
        &self,
        backend: &SqliteStateBackend,
        executor: &LocalExecutor<'_, SqliteStateBackend>,
        request: GitHubPrCommentFixtureContinuationValidationRequest<'_>,
        later_check: &WorkflowRunEvent,
    ) -> (&'static str, &'static str) {
        match self.variant {
            Variant::ChangedEvidence => {
                backend
                    .append_event(later_check)
                    .expect("persist superseding check");
                let events = backend
                    .read_events(request.write_request.run_id())
                    .expect("current history");
                assert_eq!(events.last(), Some(later_check));
                (
                    "changed-evidence",
                    "github_pr_comment_fixture_continuation.check_evidence_invalid",
                )
            }
            Variant::StaleWindow => {
                // The Core fixture window lasts five minutes. Let trusted time advance normally.
                std::thread::sleep(std::time::Duration::from_secs(301));
                let original = request.write_request.actor().clone();
                let expired = workflow_core::validate_github_pr_comment_fixture_continuation(
                    executor,
                    backend,
                    GitHubPrCommentFixtureContinuationValidationRequest {
                        replacement_actor: &original,
                        presentation_policy: request
                            .presentation_policy
                            .with_max_presentation_age(std::time::Duration::from_secs(600)),
                        ..request
                    },
                )
                .expect_err(
                    "same-actor probe must confirm stale window rather than a grant problem",
                );
                assert_eq!(
                    expired.code(),
                    "github_pr_comment_fixture_continuation.execution_window_not_resumable"
                );
                ("stale-window", "local_fixture_replacement_authority.denied")
            }
            Variant::Baseline => unreachable!("baseline has no state change"),
        }
    }

    fn verify_cases(
        &self,
        database: &Path,
        cases: &[Case<'_>],
        proof: &[u8; 32],
        wrong: &[u8; 32],
    ) {
        let before = fs::read(database).expect("read database for byte comparison");
        // The real CLI consults trusted current time; let this one grant expire naturally.
        std::thread::sleep(std::time::Duration::from_secs(3));
        for (id, grant, principal_file, input) in cases {
            let credential = if *principal_file == "principal" {
                proof
            } else {
                wrong
            };
            let result = self.invoke(grant, credential, input);
            assert_eq!(
                result.status.success(),
                *id == "exact",
                "fixture verdict for {id}"
            );
            if *id == "exact" {
                assert_eq!(result.stdout, self.invoke(grant, credential, input).stdout);
                let result: Value =
                    serde_json::from_slice(&result.stdout).expect("bounded CLI output");
                assert_eq!(result["actor_posture"], "verified_delegation");
                for key in [
                    "provider_call_performed",
                    "side_effect_mutated",
                    "workflow_event_appended",
                    "report_artifact_written",
                ] {
                    assert_eq!(result[key], false);
                }
            } else if *id == "denied-approval" {
                assert_rejected(
                    &result,
                    "github_pr_comment_fixture_continuation_cli.side_effect.missing",
                );
            } else if *id == "terminal-replay" {
                assert_rejected(
                    &result,
                    "github_pr_comment_fixture_continuation.run_not_active",
                );
            } else {
                assert!(
                    result.stdout.is_empty(),
                    "rejection must not project a success"
                );
            }
        }
        assert_eq!(before, fs::read(database).expect("database unchanged"));
    }

    fn invoke(&self, grant: &str, proof: &[u8; 32], input: &Value) -> Output {
        let mut child = Command::new(&self.kernel)
            .arg("--project-dir")
            .arg(&self.root)
            .arg("--state-dir")
            .arg(&self.root)
            .args([
                "--json",
                "provider",
                "github-pr-comment",
                "continue-sandbox-delegated",
                "--grant",
                grant,
                "--credential-and-request-stdin",
            ])
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("public delegated CLI");
        let mut stdin = child.stdin.take().expect("private stdin");
        stdin.write_all(proof).expect("write local proof");
        serde_json::to_writer(&mut stdin, input).expect("write bounded input");
        drop(stdin);
        child.wait_with_output().expect("read delegated verdict")
    }
}

fn terminal_cases<'a>(
    backend: &SqliteStateBackend,
    executor: &LocalExecutor<'_, SqliteStateBackend>,
    project: &TestProject,
    exact: &GitHubPrCommentFixtureContinuationValidationRequest<'_>,
    envelope: &Value,
    grant: &'a str,
) -> [Case<'a>; 2] {
    [
        ("denied-approval", ApprovalDecisionKind::Denied),
        ("terminal-replay", ApprovalDecisionKind::Granted),
    ]
    .map(|(id, decision)| {
        let run_id = WorkflowRunId::new(format!("run/claude-{id}")).expect("separate run");
        let paused = workflow_core::execute_with_immutable_run_bundle(
            executor,
            backend,
            &project.immutable_bundle_request(run_id.clone(), &format!("bundle/claude-{id}")),
        )
        .expect("bound fixture run");
        let approval = &paused.run().snapshot.approval_requests[0];
        let effect_id = SideEffectId::new(format!("side-effect/claude-{id}")).expect("effect id");
        let (input, action) = if decision == ApprovalDecisionKind::Granted {
            let record = github_pr_comment_attempted_record_with_id(
                backend,
                project,
                run_id.clone(),
                SideEffectReference::new(SideEffectReferenceKind::ApprovalDecision, &approval.approval_id)
                    .expect("approval ref"),
                exact.write_request.target(),
                effect_id,
            );
            let request = github_pr_comment_fixture_continuation_request(project, run_id.clone(), &record);
            let action = workflow_core::github_pr_comment_fixture_approval_action(&request).expect("exact action");
            (fixture_cli_input(&request), action)
        } else {
            assert!(backend.read_side_effect_record(&effect_id).expect("effect lookup").is_none());
            let mut input = envelope.clone();
            input["request"]["run_id"] = json!(run_id);
            input["request"]["side_effect_id"] = json!(effect_id);
            input["request"]["preflight"]["side_effect_id"] = json!(effect_id);
            input["fixture"]["side_effect_id"] = json!(effect_id);
            (input, "deny synthetic fixture attempt".to_owned())
        };
        let presentation = approval_presentation_record_for_action(
            approval,
            &format!("presentation/claude-{id}"),
            Timestamp::now_utc(),
            &action,
        );
        backend.write_approval_presentation_record(&presentation).expect("persist decision presentation");
        let result = executor.decide_approval_with_presentation(LocalApprovalPresentationDecisionRequest {
            approval: project.approval_request(run_id, approval.approval_id.clone(), decision),
            proof: LocalApprovalPresentationProof::PresentationId(presentation.presentation_id().clone()),
            max_presentation_age: Some(std::time::Duration::from_secs(60)),
        }).expect("terminal proof-backed decision");
        if decision == ApprovalDecisionKind::Denied {
            assert_eq!(result.snapshot.status, WorkflowRunStatus::Failed);
            assert!(result.events.iter().any(|event| matches!(&event.kind, WorkflowRunEventKind::ApprovalDenied(decision) if decision.proof_marker.is_some())));
        } else {
            assert_eq!(result.snapshot.status, WorkflowRunStatus::Completed);
        }
        (id, grant, "principal", input)
    })
}

fn assert_rejected(result: &Output, diagnostic: &str) {
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.starts_with(&format!("error[{diagnostic}]")),
        "unexpected fixture diagnostic: {stderr}"
    );
}

fn fixture_cases<'a>(
    envelope: Value,
    original_target: &GitHubPullRequestCommentTarget,
    exact: &'a str,
    revoked: &'a str,
    expired: &'a str,
) -> [Case<'a>; 8] {
    let mut changed_action = envelope.clone();
    changed_action["request"]["comment_body"] = json!("Different synthetic action.");
    let mut forged_actor = envelope.clone();
    forged_actor["replacement_actor"] = json!("agent/forged");
    let target = GitHubPullRequestCommentTarget::new(
        original_target.owner(),
        original_target.repository(),
        original_target.pull_request_number() + 1,
    )
    .expect("different synthetic target");
    let mut changed_target = envelope.clone();
    changed_target["request"]["target"] = json!(target);
    changed_target["request"]["preflight"]["target"]["reference"] = json!(target.reference());
    changed_target["fixture"]["target"] = json!(target);
    [
        ("exact", exact, "principal", envelope.clone()),
        ("action-drift", exact, "principal", changed_action),
        ("target-drift", exact, "principal", changed_target),
        ("forged-actor", exact, "principal", forged_actor),
        (
            "wrong-principal",
            exact,
            "wrong-principal",
            envelope.clone(),
        ),
        (
            "missing-grant",
            "grant/claude-missing",
            "principal",
            envelope.clone(),
        ),
        ("revoked-grant", revoked, "principal", envelope.clone()),
        ("expired-grant", expired, "principal", envelope),
    ]
}

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0; N];
    getrandom::getrandom(&mut bytes).expect("OS random source");
    bytes
}

fn private_write(path: &Path, bytes: &[u8]) {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .expect("create private operator file")
        .write_all(bytes)
        .expect("write private operator file");
}
