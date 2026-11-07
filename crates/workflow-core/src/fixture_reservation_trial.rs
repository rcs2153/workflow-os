//! Test-only operator setup. Never exposes the admitted capability to the CLI.
#![allow(clippy::expect_used)]

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};

use super::{
    prepare_fixture_window, GitHubPrCommentFixtureContinuationValidationRequest, LocalExecutor,
    LocalSkillRegistry,
};
use crate::authorized_execution_continuity_state::internal::AttemptUseCapability;
use crate::dispatch_reservation::{
    request_commitment, DispatchReservationOperationId, DispatchReservationOutcome,
    DispatchReservationReceiptId, DispatchReservationRequest, DispatchReservationStore,
};
use crate::local_fixture_replacement_authority::LocalFixturePrincipalVerifier;
use crate::operational_execution_window_opening::{
    OperationalExecutionAttemptUseCapability, OperationalExecutionWindowOpeningResult,
};
use crate::state::StateBackend;
use crate::{
    ActorId, ApprovalDecision, ApprovalPresentationDefaultEnforcementPolicy,
    ApprovalPresentationSensitiveActionPosture, ApprovalReferenceId, CapabilityDelegationPosture,
    CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
    CapabilityGrantRequirements, CapabilityGrantScope, CapabilityReference, CapabilityResourceKind,
    CapabilityResourceScope, GitHubPullRequestCommentFixture,
    GitHubPullRequestCommentFixtureDefinition, GitHubPullRequestCommentWriteRequest,
    HarnessContractId, ImmutableRunBundleStore, LocalApprovalPresentationProof, LocalCheckResultId,
    RedactionMetadata, SideEffectRecordStore, SpecContentHash, SqliteStateBackend, Timestamp,
    WorkReportSensitivity, WorkflowRunEventKind,
};

#[test]
#[ignore = "requires a fresh disposable reservation-input fixture and reviewed CLI"]
fn export_reserved_attempt_for_native_fixture_trial() {
    let (directory, kernel, input) = read_stage();
    let root = directory.as_path();
    verify_reserved_attempt(&kernel, root, &input);
}

fn read_stage() -> (PathBuf, PathBuf, Value) {
    let path = fs::canonicalize(
        std::env::var_os("WORKFLOW_OS_CLAUDE_RESERVATION_INPUT").expect("reservation input path"),
    )
    .expect("canonical reservation input");
    let root = path.parent().expect("private fixture directory");
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("reservation-input.json")
    );
    assert_eq!(
        root.parent(),
        Some(
            fs::canonicalize(std::env::temp_dir())
                .expect("scratch root")
                .as_path()
        )
    );
    assert!(root
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("workflow-os-claude-fixture-")));
    assert_eq!(
        fs::metadata(root)
            .expect("private root")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(&path)
            .expect("private input")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    let mut stage: Value =
        serde_json::from_slice(&fs::read(&path).expect("stage bytes")).expect("stage JSON");
    let kernel = PathBuf::from(stage["kernel"].as_str().expect("reviewed kernel"));
    assert!(kernel.is_absolute() && kernel.is_file());
    (root.to_owned(), kernel, stage["input"].take())
}

fn verify_reserved_attempt(kernel: &Path, root: &Path, input: &Value) {
    let action: GitHubPullRequestCommentWriteRequest =
        serde_json::from_value(input["request"].clone()).expect("exact original action");
    assert_eq!(action.run_id().as_str(), "run/fixture-continuation-success");
    let database = root.join("github-pr-comment-continuation.sqlite3");
    assert!(fs::symlink_metadata(&database)
        .expect("fresh fixture database")
        .file_type()
        .is_file());
    let store = SqliteStateBackend::open(&database).expect("fixture store");
    let registry = LocalSkillRegistry::new();
    let executor = LocalExecutor::new(&store, &registry);
    let run = store
        .rehydrate_run(action.run_id())
        .expect("active fixture run");
    let bundle = store
        .read_exact_bundle(
            action.run_id(),
            run.snapshot
                .identity
                .immutable_run_bundle
                .as_ref()
                .expect("bound run")
                .bundle_id(),
        )
        .expect("durable fixture bundle");
    let attempted = store
        .read_side_effect_record(action.side_effect_id())
        .expect("attempt lookup")
        .expect("attempted effect");
    let fixture = fixture_for(&action);
    let original = action.actor().clone();
    let mut request = GitHubPrCommentFixtureContinuationValidationRequest {
        run: &run,
        attempted_record: &attempted,
        write_request: action,
        fixture: &fixture,
        replacement_actor: &original,
        delegated_by: None,
        presentation_policy:
            ApprovalPresentationDefaultEnforcementPolicy::required_for_sensitive_action(
                LocalApprovalPresentationProof::ResolveByRunAndApproval,
                ApprovalPresentationSensitiveActionPosture::WriteAdjacent,
            )
            .with_max_presentation_age(std::time::Duration::from_secs(600)),
        immutable_run_bundle: &bundle,
    };
    let (_, opening) =
        prepare_fixture_window(&executor, request.clone()).expect("open exact fixture");
    let OperationalExecutionWindowOpeningResult::Opened { capability, .. } = opening.result else {
        unreachable!("fresh fixture must yield exactly one attempt capability")
    };
    let interrupted = store
        .rehydrate_run(request.write_request.run_id())
        .expect("interrupted fixture");
    let decision = interrupted
        .events
        .iter()
        .find_map(|event| match &event.kind {
            WorkflowRunEventKind::ApprovalGranted(decision) => Some(decision),
            _ => None,
        })
        .expect("proof-backed decision");
    let actor = ActorId::new("agent/claude-desktop").expect("fixture principal");
    let harness = HarnessContractId::new("harness/claude-desktop").expect("fixture harness");
    let mut proof = [0_u8; 32];
    getrandom::getrandom(&mut proof).expect("OS random source");
    store
        .register_local_fixture_principal(&LocalFixturePrincipalVerifier::new(
            actor.clone(),
            harness.clone(),
            &proof,
        ))
        .expect("register distinct principal");
    request.run = &interrupted;
    request.replacement_actor = &actor;
    let grant = grant_for(&request, decision, harness);
    crate::issue_local_fixture_replacement_grant(&executor, &request, &decision.actor, &grant)
        .expect("issue exact grant before reservation");
    let verdict = || delegated_cli(kernel, root, grant.grant_id(), &proof, input);
    let before = verdict();
    assert!(before.status.success());
    let positive: Value = serde_json::from_slice(&before.stdout).expect("verified baseline");
    assert_eq!(positive["actor_posture"], "verified_delegation");
    assert_eq!(positive["provider_call_performed"], false);

    assert_reservation_blocks_validation(&executor, &database, request, *capability, verdict);
    private_write(&root.join("principal"), &proof);
    let manifest = root.join("manifest.json");
    private_write(&manifest, &serde_json::to_vec_pretty(&json!({
        "kernel": kernel, "project": root, "state": root,
        "cases": [{"id": "reserved-attempt", "grant": grant.grant_id(), "principal_file": "principal", "input": input}],
    })).expect("operator manifest"));
    println!("claude_fixture_manifest: {}", manifest.display());
}

fn assert_reservation_blocks_validation(
    executor: &LocalExecutor<'_, SqliteStateBackend>,
    database: &Path,
    request: GitHubPrCommentFixtureContinuationValidationRequest<'_>,
    capability: OperationalExecutionAttemptUseCapability,
    verdict: impl FnOnce() -> Output,
) {
    let store = executor.backend;
    let attempt_id = capability.attempt_id.clone();
    reserve_without_dispatch(store, capability);
    let reserved = store
        .rehydrate_run(request.write_request.run_id())
        .expect("reserved run");
    assert_eq!(reserved.events.len(), request.run.events.len() + 1);
    assert!(matches!(
        reserved.events.last().map(|event| &event.kind),
        Some(WorkflowRunEventKind::AuthorizedExecutionAttemptDispatchAdmitted(admission))
            if admission.attempt_id() == &attempt_id
    ));
    assert!(reserved
        .snapshot
        .last_dispatch_admission_projection
        .is_some());
    assert_eq!(
        store
            .read_side_effect_record(request.write_request.side_effect_id())
            .expect("read effect")
            .as_ref(),
        Some(request.attempted_record)
    );
    let original = request.write_request.actor().clone();
    let same_actor = crate::validate_github_pr_comment_fixture_continuation(
        executor,
        store,
        GitHubPrCommentFixtureContinuationValidationRequest {
            run: &reserved,
            replacement_actor: &original,
            ..request
        },
    )
    .expect_err("reservation invalidates exact read-only continuation");
    assert_eq!(
        same_actor.code(),
        "github_pr_comment_fixture_continuation.execution_window_not_resumable"
    );
    let stored = fs::read(database).expect("snapshot after reservation");
    let denied = verdict();
    assert!(!denied.status.success() && denied.stdout.is_empty());
    assert!(String::from_utf8_lossy(&denied.stderr)
        .starts_with("error[local_fixture_replacement_authority.denied]"));
    assert_eq!(
        fs::read(database).expect("snapshot after validation"),
        stored
    );
}

fn fixture_for(action: &GitHubPullRequestCommentWriteRequest) -> GitHubPullRequestCommentFixture {
    GitHubPullRequestCommentFixture::new(GitHubPullRequestCommentFixtureDefinition {
        target: action.target().clone(),
        side_effect_id: action.side_effect_id().clone(),
        idempotency_key: action.idempotency_key().clone(),
        mode: action.mode(),
        fixture_reference: Some("fixture/github-pr-comment/continuation".to_owned()),
        summary: "fixture continuation validated without provider call".to_owned(),
        sensitivity: action.sensitivity(),
        redaction: action.redaction().clone(),
    })
    .expect("fixture response")
}

fn grant_for(
    request: &GitHubPrCommentFixtureContinuationValidationRequest<'_>,
    decision: &ApprovalDecision,
    harness: HarnessContractId,
) -> CapabilityGrant {
    let issued = Timestamp::now_utc();
    CapabilityGrant::new(CapabilityGrantDefinition {
        grant_id: CapabilityGrantId::new("grant/claude-reserved").expect("exact grant"),
        subject: request.replacement_actor.clone(),
        capability: CapabilityReference::new("fixture/github-pr-comment-continuation")
            .expect("fixture capability"),
        resource: CapabilityResourceScope::new(
            CapabilityResourceKind::AdapterResource,
            request.write_request.target().reference(),
        )
        .expect("exact target"),
        scope: CapabilityGrantScope::new(
            request.run.snapshot.identity.workflow_id.clone(),
            Some(request.write_request.run_id().clone()),
            request.write_request.step_id().cloned(),
            Some(harness),
        )
        .expect("exact scope"),
        issuer: decision.actor.clone(),
        issued_at: issued,
        expires_at: Some(Timestamp::from_offset_date_time(
            issued.as_offset_date_time() + time::Duration::minutes(3),
        )),
        lifecycle: CapabilityGrantLifecycle::Active,
        revocation_reference: None,
        delegation: CapabilityDelegationPosture::Disabled,
        requirements: CapabilityGrantRequirements::new(
            vec![],
            vec![ApprovalReferenceId::new(&decision.approval_id).expect("approval")],
            vec![],
            vec![LocalCheckResultId::new("result/fixture-continuation-check").expect("check")],
        )
        .expect("requirements"),
        sensitivity_ceiling: WorkReportSensitivity::Secret,
        redaction: RedactionMetadata::empty(),
    })
    .expect("fixture grant")
}

fn reserve_without_dispatch(
    store: &SqliteStateBackend,
    opened: OperationalExecutionAttemptUseCapability,
) {
    let mut reservation = DispatchReservationRequest {
        operation_id: DispatchReservationOperationId::new("dispatch/claude-fixture")
            .expect("operation"),
        receipt_id: DispatchReservationReceiptId::new("receipt/claude-fixture-reservation")
            .expect("receipt"),
        request_commitment: SpecContentHash::from_text("pending test reservation"),
        capability: AttemptUseCapability {
            attempt_id: opened.attempt_id,
            subject_actor_id: opened.subject_actor_id,
            window_id: opened.window_id,
            window_revision: opened.window_revision,
            cursor: opened.cursor,
            authority_commitment: opened.authority_commitment,
            window_binding_commitment: opened.window_binding_commitment,
            consume_operation_id: opened.consume_operation_id,
            dispatch_reservation: None,
        },
        expected_window_binding: opened.expected_window_binding,
        operation_binding_commitment: opened.operation_binding_commitment,
        invocation_commitment: SpecContentHash::from_text("synthetic fixture invocation"),
        executor_commitment: SpecContentHash::from_text("test-only no-dispatch executor"),
        workflow_lifecycle_commitment: None,
    };
    reservation.request_commitment = request_commitment(&reservation);
    assert!(matches!(
        store
            .reserve_attempt_dispatch(reservation)
            .expect("real Core reservation"),
        DispatchReservationOutcome::Admitted { .. }
    ));
    // The test drops the admitted capability. No dispatcher is present.
}

fn delegated_cli(
    kernel: &Path,
    root: &Path,
    grant: &CapabilityGrantId,
    proof: &[u8; 32],
    input: &Value,
) -> Output {
    let mut child = Command::new(kernel)
        .arg("--project-dir")
        .arg(root)
        .arg("--state-dir")
        .arg(root)
        .args([
            "--json",
            "provider",
            "github-pr-comment",
            "continue-sandbox-delegated",
            "--grant",
            grant.as_str(),
            "--credential-and-request-stdin",
        ])
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("public read-only CLI");
    let mut stdin = child.stdin.take().expect("private stdin");
    stdin.write_all(proof).expect("write proof");
    serde_json::to_writer(&mut stdin, input).expect("write envelope");
    drop(stdin);
    child.wait_with_output().expect("read verdict")
}

fn private_write(path: &Path, contents: &[u8]) {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .expect("private output")
        .write_all(contents)
        .expect("write private output");
}
