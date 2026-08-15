use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    EventId, EventSequenceNumber, IdempotencyKey, IdempotencyResult, IdempotencyWrite,
    ImmutableRunBundleBinding, SpecContentHash, StepId, WorkflowOsError, WorkflowOsErrorKind,
    WorkflowRun, WorkflowRunId, WorkflowRunStatus,
};

const CONTINUATION_ALGORITHM: &str = "workflow-os/governed-continuation/v1";

/// The only material operation authorized by the first continuation slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedNextAction {
    /// Invoke the current immutable workflow step through the local skill path.
    InvokeCurrentStepSkill,
}

impl GovernedNextAction {
    const fn code(self) -> &'static str {
        match self {
            Self::InvokeCurrentStepSkill => "invoke_current_step_skill",
        }
    }
}

/// Exact durable position and immutable scope described by a continuation brief.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedContinuationBinding {
    run_id: WorkflowRunId,
    immutable_run_bundle: ImmutableRunBundleBinding,
    last_sequence_number: EventSequenceNumber,
    last_event_id: EventId,
    step_id: StepId,
    invocation_idempotency_key: IdempotencyKey,
    governance_commitment: SpecContentHash,
}

impl GovernedContinuationBinding {
    /// Returns the bound run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the immutable run-bundle binding.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the exact durable event sequence described by the brief.
    #[must_use]
    pub const fn last_sequence_number(&self) -> EventSequenceNumber {
        self.last_sequence_number
    }

    /// Returns the exact durable event identity described by the brief.
    #[must_use]
    pub const fn last_event_id(&self) -> &EventId {
        &self.last_event_id
    }

    /// Returns the current immutable step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the payload-free commitment to the current governance inputs.
    #[must_use]
    pub const fn governance_commitment(&self) -> &SpecContentHash {
        &self.governance_commitment
    }
}

impl fmt::Debug for GovernedContinuationBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContinuationBinding")
            .field("run_id", &"[REDACTED]")
            .field("immutable_run_bundle", &"[REDACTED]")
            .field("last_sequence_number", &self.last_sequence_number)
            .field("last_event_id", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("invocation_idempotency_key", &"[REDACTED]")
            .field("governance_commitment", &"[REDACTED]")
            .finish()
    }
}

/// Bounded orientation record for one current material continuation.
///
/// A valid brief is not authority. Core rehydrates and consumes the exact
/// binding again inside the concrete consumer call.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedContinuationBrief {
    algorithm: String,
    binding: GovernedContinuationBinding,
    run_status: WorkflowRunStatus,
    allowed_next_action: GovernedNextAction,
}

impl GovernedContinuationBrief {
    /// Returns the versioned projection algorithm.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Returns the exact orientation binding.
    #[must_use]
    pub const fn binding(&self) -> &GovernedContinuationBinding {
        &self.binding
    }

    /// Returns the durable run posture observed during projection.
    #[must_use]
    pub const fn run_status(&self) -> WorkflowRunStatus {
        self.run_status
    }

    /// Returns the sole action vocabulary supported by this slice.
    #[must_use]
    pub const fn allowed_next_action(&self) -> GovernedNextAction {
        self.allowed_next_action
    }
}

impl fmt::Debug for GovernedContinuationBrief {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedContinuationBrief")
            .field("algorithm", &self.algorithm)
            .field("binding", &self.binding)
            .field("run_status", &self.run_status)
            .field("allowed_next_action", &self.allowed_next_action)
            .finish()
    }
}

pub(crate) struct GovernedContinuationProjectionInput<'a> {
    pub(crate) run: &'a WorkflowRun,
    pub(crate) step_id: &'a StepId,
    pub(crate) invocation_idempotency_key: &'a IdempotencyKey,
    pub(crate) governance_commitment: SpecContentHash,
}

pub(crate) fn project_governed_continuation_brief(
    input: GovernedContinuationProjectionInput<'_>,
) -> Result<GovernedContinuationBrief, WorkflowOsError> {
    if input.run.snapshot.status != WorkflowRunStatus::Running {
        return Err(continuation_error("run.not_running"));
    }
    let immutable_run_bundle = input
        .run
        .snapshot
        .identity
        .immutable_run_bundle
        .clone()
        .ok_or_else(|| continuation_error("immutable_bundle.required"))?;
    Ok(GovernedContinuationBrief {
        algorithm: CONTINUATION_ALGORITHM.to_owned(),
        binding: GovernedContinuationBinding {
            run_id: input.run.snapshot.identity.run_id.clone(),
            immutable_run_bundle,
            last_sequence_number: input.run.snapshot.last_sequence_number,
            last_event_id: input.run.snapshot.last_event_id.clone(),
            step_id: input.step_id.clone(),
            invocation_idempotency_key: input.invocation_idempotency_key.clone(),
            governance_commitment: input.governance_commitment,
        },
        run_status: input.run.snapshot.status,
        allowed_next_action: GovernedNextAction::InvokeCurrentStepSkill,
    })
}

pub(crate) fn consume_governed_continuation<B, T, F>(
    backend: &B,
    brief: &GovernedContinuationBrief,
    consumer: F,
) -> Result<T, WorkflowOsError>
where
    B: crate::StateBackend,
    F: FnOnce() -> Result<T, WorkflowOsError>,
{
    consume_governed_continuation_with_after_claim(backend, brief, || Ok(()), consumer)
}

fn consume_governed_continuation_with_after_claim<B, T, H, F>(
    backend: &B,
    brief: &GovernedContinuationBrief,
    after_claim: H,
    consumer: F,
) -> Result<T, WorkflowOsError>
where
    B: crate::StateBackend,
    H: FnOnce() -> Result<(), WorkflowOsError>,
    F: FnOnce() -> Result<T, WorkflowOsError>,
{
    validate_current_cursor(backend, brief.binding())?;
    let claim_key = continuation_claim_key(brief)?;
    let claim = backend.record_idempotency_result(
        &claim_key,
        IdempotencyResult {
            result_ref: "governed-continuation-consumed".to_owned(),
        },
    )?;
    if matches!(claim, IdempotencyWrite::Duplicate(_)) {
        return Err(continuation_error("claim.already_consumed"));
    }
    after_claim()?;
    validate_current_cursor(backend, brief.binding())?;
    consumer()
}

fn validate_current_cursor<B>(
    backend: &B,
    binding: &GovernedContinuationBinding,
) -> Result<(), WorkflowOsError>
where
    B: crate::StateBackend,
{
    let run = backend.rehydrate_run(binding.run_id())?;
    if run.snapshot.status != WorkflowRunStatus::Running
        || run.snapshot.identity.immutable_run_bundle.as_ref()
            != Some(binding.immutable_run_bundle())
        || run.snapshot.last_sequence_number != binding.last_sequence_number()
        || &run.snapshot.last_event_id != binding.last_event_id()
    {
        return Err(continuation_error("cursor.stale"));
    }
    Ok(())
}

fn continuation_claim_key(
    brief: &GovernedContinuationBrief,
) -> Result<IdempotencyKey, WorkflowOsError> {
    let binding = brief.binding();
    let mut hasher = Sha256::new();
    update_hash(&mut hasher, "algorithm", brief.algorithm());
    update_hash(&mut hasher, "run", binding.run_id().as_str());
    update_hash(
        &mut hasher,
        "bundle-root",
        binding.immutable_run_bundle().root_hash().as_str(),
    );
    update_hash(
        &mut hasher,
        "sequence",
        &binding.last_sequence_number().get().to_string(),
    );
    update_hash(&mut hasher, "event", binding.last_event_id().as_str());
    update_hash(&mut hasher, "step", binding.step_id().as_str());
    update_hash(
        &mut hasher,
        "invocation",
        binding.invocation_idempotency_key.as_str(),
    );
    update_hash(
        &mut hasher,
        "governance",
        binding.governance_commitment().as_str(),
    );
    update_hash(&mut hasher, "action", brief.allowed_next_action().code());
    let digest = hasher.finalize();
    IdempotencyKey::new(format!("continuation/{}", hex_lower(&digest)))
        .map_err(|_| continuation_error("claim.invalid"))
}

fn update_hash(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(label.as_bytes());
    hasher.update([0]);
    hasher.update(value.as_bytes());
    hasher.update([0xff]);
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn continuation_error(suffix: &str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::InvalidState,
        format!("executor.governed_continuation.{suffix}"),
        "governed continuation could not be consumed",
    )
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};

    use super::*;
    use crate::{
        Action, ActorId, Capability, CorrelationId, EventLogStore, ImmutableRunBundleBinding,
        LocalStateBackend, PolicyDecision, SchemaVersion, StateBackend, Timestamp, WorkflowId,
        WorkflowRunEvent, WorkflowRunEventKind, WorkflowVersion,
    };

    static NEXT_TEST: AtomicUsize = AtomicUsize::new(1);

    #[test]
    fn brief_is_orientation_only_and_exact_binding_consumes_once() {
        let (backend, run) = running_fixture("consume-once");
        let brief = project_governed_continuation_brief(GovernedContinuationProjectionInput {
            run: &run,
            step_id: &StepId::new("step-one").expect("step"),
            invocation_idempotency_key: &IdempotencyKey::new("invocation/one").expect("key"),
            governance_commitment: SpecContentHash::from_bytes("governance"),
        })
        .expect("brief");
        let calls = AtomicUsize::new(0);

        consume_governed_continuation(&backend, &brief, || {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect("first consumer");
        let error = consume_governed_continuation(&backend, &brief, || {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .expect_err("duplicate blocks");

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            error.code(),
            "executor.governed_continuation.claim.already_consumed"
        );
        assert_eq!(
            brief.allowed_next_action(),
            GovernedNextAction::InvokeCurrentStepSkill
        );
    }

    #[test]
    fn concurrent_consumers_have_one_durable_first_writer() {
        let (backend, run) = running_fixture("concurrent");
        let brief = project_governed_continuation_brief(GovernedContinuationProjectionInput {
            run: &run,
            step_id: &StepId::new("step-one").expect("step"),
            invocation_idempotency_key: &IdempotencyKey::new("invocation/concurrent").expect("key"),
            governance_commitment: SpecContentHash::from_bytes("governance"),
        })
        .expect("brief");
        let backend = Arc::new(backend);
        let barrier = Arc::new(Barrier::new(2));
        let calls = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        for _ in 0..2 {
            let backend = Arc::clone(&backend);
            let barrier = Arc::clone(&barrier);
            let calls = Arc::clone(&calls);
            let brief = brief.clone();
            workers.push(std::thread::spawn(move || {
                barrier.wait();
                consume_governed_continuation(backend.as_ref(), &brief, || {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
            }));
        }
        let outcomes: Vec<_> = workers
            .into_iter()
            .map(|worker| worker.join().expect("worker"))
            .collect();

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
        assert_eq!(
            outcomes
                .iter()
                .filter_map(|outcome| outcome.as_ref().err())
                .map(WorkflowOsError::code)
                .collect::<Vec<_>>(),
            vec!["executor.governed_continuation.claim.already_consumed"]
        );
    }

    #[test]
    fn cursor_change_after_claim_burns_claim_before_consumer() {
        let (backend, run) = running_fixture("post-claim-change");
        let brief = project_governed_continuation_brief(GovernedContinuationProjectionInput {
            run: &run,
            step_id: &StepId::new("step-one").expect("step"),
            invocation_idempotency_key: &IdempotencyKey::new("invocation/change").expect("key"),
            governance_commitment: SpecContentHash::from_bytes("governance"),
        })
        .expect("brief");
        let changed = next_event(
            &run,
            WorkflowRunEventKind::PolicyDecisionRecorded(Box::new(PolicyDecision {
                allowed: true,
                requires_approval: false,
                reason_codes: vec!["policy.test.cursor_change".to_owned()],
                violations: Vec::new(),
                action: Action::InvokeSkill,
                capabilities: vec![Capability::LocalRead],
                actor: None,
                workflow_id: Some(run.snapshot.identity.workflow_id.clone()),
                run_id: Some(run.snapshot.identity.run_id.clone()),
                correlation_id: None,
            })),
        );
        let calls = AtomicUsize::new(0);

        let error = consume_governed_continuation_with_after_claim(
            &backend,
            &brief,
            || backend.append_event(&changed),
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            },
        )
        .expect_err("changed cursor blocks");

        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(error.code(), "executor.governed_continuation.cursor.stale");
        let duplicate = consume_governed_continuation(&backend, &brief, || Ok(()))
            .expect_err("burned claim cannot be reused");
        assert_eq!(
            duplicate.code(),
            "executor.governed_continuation.cursor.stale"
        );
    }

    #[test]
    fn brief_debug_redacts_binding_values_and_serde_round_trips() {
        let (_, run) = running_fixture("redaction");
        let brief = project_governed_continuation_brief(GovernedContinuationProjectionInput {
            run: &run,
            step_id: &StepId::new("sensitive-step").expect("step"),
            invocation_idempotency_key: &IdempotencyKey::new("invocation/redacted").expect("key"),
            governance_commitment: SpecContentHash::from_bytes("secret-like-governance"),
        })
        .expect("brief");

        let debug = format!("{brief:?}");
        assert!(!debug.contains("sensitive-step"));
        assert!(!debug.contains("invocation/redacted"));
        let encoded = serde_json::to_string(&brief).expect("serialize");
        let decoded: GovernedContinuationBrief =
            serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded, brief);
    }

    fn running_fixture(name: &str) -> (LocalStateBackend, WorkflowRun) {
        let id = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-governed-continuation-{name}-{}-{id}",
            std::process::id()
        ));
        let backend = LocalStateBackend::new(root).expect("backend");
        let run_id = WorkflowRunId::new(format!("run-{name}-{id}")).expect("run id");
        let workflow_id = WorkflowId::new("local/continuation").expect("workflow id");
        let schema_version = SchemaVersion::new("workflowos.dev/v0").expect("schema");
        let workflow_version = WorkflowVersion::new("1.0.0").expect("version");
        let spec_hash = SpecContentHash::from_bytes("spec");
        let bundle: ImmutableRunBundleBinding = serde_json::from_value(serde_json::json!({
            "bundle_id": format!("bundle/{name}"),
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_bytes("bundle-root").as_str(),
        }))
        .expect("bundle binding");
        let kinds = [
            WorkflowRunEventKind::RunCreated {
                summary: None,
                immutable_run_bundle: Some(bundle),
            },
            WorkflowRunEventKind::RunValidated,
            WorkflowRunEventKind::RunStarted,
            WorkflowRunEventKind::StepScheduled {
                step_id: StepId::new("step-one").expect("step"),
            },
        ];
        for (index, kind) in kinds.into_iter().enumerate() {
            backend
                .append_event(&WorkflowRunEvent {
                    sequence_number: EventSequenceNumber::new((index + 1) as u64)
                        .expect("sequence"),
                    event_id: EventId::new(format!("event-{name}-{index}")).expect("event"),
                    timestamp: Timestamp::now_utc(),
                    run_id: run_id.clone(),
                    workflow_id: workflow_id.clone(),
                    schema_version: schema_version.clone(),
                    workflow_version: workflow_version.clone(),
                    spec_content_hash: spec_hash.clone(),
                    correlation_id: Some(
                        CorrelationId::new(format!("correlation-{name}")).expect("correlation"),
                    ),
                    actor: Some(ActorId::new("system/test").expect("actor")),
                    idempotency_key: None,
                    kind,
                })
                .expect("append event");
        }
        let run = backend.rehydrate_run(&run_id).expect("rehydrate");
        (backend, run)
    }

    fn next_event(run: &WorkflowRun, kind: WorkflowRunEventKind) -> WorkflowRunEvent {
        WorkflowRunEvent {
            sequence_number: EventSequenceNumber::new(run.snapshot.last_sequence_number.get() + 1)
                .expect("sequence"),
            event_id: EventId::new("event-cursor-change").expect("event"),
            timestamp: Timestamp::now_utc(),
            run_id: run.snapshot.identity.run_id.clone(),
            workflow_id: run.snapshot.identity.workflow_id.clone(),
            schema_version: run.snapshot.identity.schema_version.clone(),
            workflow_version: run.snapshot.identity.workflow_version.clone(),
            spec_content_hash: run.snapshot.identity.spec_content_hash.clone(),
            correlation_id: None,
            actor: Some(ActorId::new("system/test").expect("actor")),
            idempotency_key: None,
            kind,
        }
    }
}
