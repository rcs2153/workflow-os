use std::fmt;

use sha2::{Digest, Sha256};

use super::{
    attestation_error, compute_binding_fingerprint, compute_requirement_fingerprint, hash_field,
    LocalCheckAttestationAssurance, LocalCheckAttestationBinding,
    LocalCheckAttestationExitCodePosture, LocalCheckAttestationFreshnessPolicy,
    LocalCheckAttestationRequirement, LocalCheckAttestationSource,
};
use crate::{
    compute_local_check_command_contract_fingerprint, IdempotencyKey,
    ImmutableLocalCheckExecutionBinding, ImmutableLocalCheckHandlerPosture,
    ImmutableRunBundleBinding, LocalCheckCommandContract, LocalCheckResult, LocalCheckResultId,
    LocalCheckResultStatus, SkillInvocationId, SpecContentHash, StepId, StoredImmutableRunBundle,
    Timestamp, WorkflowId, WorkflowOsError, WorkflowRunId,
};

/// Core-owned payload-free observation used by the pure verifier.
///
/// This type and its constructor are crate-private so public callers cannot
/// manufacture the authority that distinguishes observation from assertion.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct KernelObservedLocalCheck {
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    invocation_id: SkillInvocationId,
    idempotency_key: IdempotencyKey,
    immutable_run_bundle: ImmutableRunBundleBinding,
    command_contract_fingerprint: SpecContentHash,
    handler_selection_fingerprint: SpecContentHash,
    effective_policy_fingerprint: SpecContentHash,
    result_id: LocalCheckResultId,
    result_status: LocalCheckResultStatus,
    exit_code_posture: LocalCheckAttestationExitCodePosture,
    duration_ms: u64,
    timed_out: bool,
    stdout_truncated: bool,
    stderr_truncated: bool,
    started_at: Timestamp,
    completed_at: Timestamp,
}

pub(crate) struct KernelObservedLocalCheckDefinition {
    pub workflow_id: WorkflowId,
    pub run_id: WorkflowRunId,
    pub step_id: StepId,
    pub invocation_id: SkillInvocationId,
    pub idempotency_key: IdempotencyKey,
    pub immutable_run_bundle: ImmutableRunBundleBinding,
    pub command_contract_fingerprint: SpecContentHash,
    pub handler_selection_fingerprint: SpecContentHash,
    pub effective_policy_fingerprint: SpecContentHash,
    pub result_id: LocalCheckResultId,
    pub result_status: LocalCheckResultStatus,
    pub exit_code_posture: LocalCheckAttestationExitCodePosture,
    pub duration_ms: u64,
    pub timed_out: bool,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub started_at: Timestamp,
    pub completed_at: Timestamp,
}

impl KernelObservedLocalCheck {
    pub(crate) fn new(
        definition: KernelObservedLocalCheckDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.started_at > definition.completed_at {
            return Err(verification_error(
                "time_invalid",
                "local check attestation observation time is invalid",
            ));
        }
        if definition.timed_out
            != matches!(definition.result_status, LocalCheckResultStatus::TimedOut)
        {
            return Err(verification_error(
                "result_mismatch",
                "local check attestation observation result is inconsistent",
            ));
        }
        Ok(Self {
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            step_id: definition.step_id,
            invocation_id: definition.invocation_id,
            idempotency_key: definition.idempotency_key,
            immutable_run_bundle: definition.immutable_run_bundle,
            command_contract_fingerprint: definition.command_contract_fingerprint,
            handler_selection_fingerprint: definition.handler_selection_fingerprint,
            effective_policy_fingerprint: definition.effective_policy_fingerprint,
            result_id: definition.result_id,
            result_status: definition.result_status,
            exit_code_posture: definition.exit_code_posture,
            duration_ms: definition.duration_ms,
            timed_out: definition.timed_out,
            stdout_truncated: definition.stdout_truncated,
            stderr_truncated: definition.stderr_truncated,
            started_at: definition.started_at,
            completed_at: definition.completed_at,
        })
    }
}

impl fmt::Debug for KernelObservedLocalCheck {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KernelObservedLocalCheck")
            .field("identities", &"[REDACTED]")
            .field("result_status", &self.result_status)
            .field("exit_code_posture", &self.exit_code_posture)
            .field("duration_ms", &self.duration_ms)
            .field("timed_out", &self.timed_out)
            .field("stdout_truncated", &self.stdout_truncated)
            .field("stderr_truncated", &self.stderr_truncated)
            .field("timestamps", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) struct LocalCheckAttestationVerificationInput<'a> {
    pub requirement: &'a LocalCheckAttestationRequirement,
    pub candidate: &'a LocalCheckAttestationBinding,
    pub stored_immutable_run_bundle: &'a StoredImmutableRunBundle,
    pub execution_binding: &'a ImmutableLocalCheckExecutionBinding,
    pub command_contract: &'a LocalCheckCommandContract,
    pub observation: &'a KernelObservedLocalCheck,
    pub result: &'a LocalCheckResult,
    pub evaluated_at: Timestamp,
}

/// Read-only accepted proof returned only by the crate-private verifier.
#[derive(Clone, Eq, PartialEq)]
pub struct AcceptedLocalCheckAttestation {
    assurance: LocalCheckAttestationAssurance,
    requirement_fingerprint: SpecContentHash,
    candidate_binding_fingerprint: SpecContentHash,
    execution_binding_fingerprint: SpecContentHash,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    invocation_id: SkillInvocationId,
    result_id: LocalCheckResultId,
    handler_selection_fingerprint: SpecContentHash,
    result_status: LocalCheckResultStatus,
    exit_code_posture: LocalCheckAttestationExitCodePosture,
    observed_completed_at: Timestamp,
    verified_at: Timestamp,
    freshness: LocalCheckAttestationFreshnessPolicy,
    stdout_truncated: bool,
    stderr_truncated: bool,
    proof_fingerprint: SpecContentHash,
}

impl AcceptedLocalCheckAttestation {
    /// Returns the accepted assurance level.
    #[must_use]
    pub const fn assurance(&self) -> LocalCheckAttestationAssurance {
        self.assurance
    }

    /// Returns the complete requirement fingerprint.
    #[must_use]
    pub const fn requirement_fingerprint(&self) -> &SpecContentHash {
        &self.requirement_fingerprint
    }

    /// Returns the accepted candidate binding fingerprint.
    #[must_use]
    pub const fn candidate_binding_fingerprint(&self) -> &SpecContentHash {
        &self.candidate_binding_fingerprint
    }

    /// Returns the pre-execution binding fingerprint.
    #[must_use]
    pub const fn execution_binding_fingerprint(&self) -> &SpecContentHash {
        &self.execution_binding_fingerprint
    }

    /// Returns the exact immutable run-bundle binding.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the accepted workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the accepted run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the accepted step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the accepted invocation identity.
    #[must_use]
    pub const fn invocation_id(&self) -> &SkillInvocationId {
        &self.invocation_id
    }

    /// Returns the accepted result identity.
    #[must_use]
    pub const fn result_id(&self) -> &LocalCheckResultId {
        &self.result_id
    }

    /// Returns the selected handler commitment.
    #[must_use]
    pub const fn handler_selection_fingerprint(&self) -> &SpecContentHash {
        &self.handler_selection_fingerprint
    }

    /// Returns the accepted structured result status.
    #[must_use]
    pub const fn result_status(&self) -> LocalCheckResultStatus {
        self.result_status
    }

    /// Returns the accepted process exit posture.
    #[must_use]
    pub const fn exit_code_posture(&self) -> LocalCheckAttestationExitCodePosture {
        self.exit_code_posture
    }

    /// Returns observation completion time.
    #[must_use]
    pub const fn observed_completed_at(&self) -> &Timestamp {
        &self.observed_completed_at
    }

    /// Returns verifier evaluation time.
    #[must_use]
    pub const fn verified_at(&self) -> &Timestamp {
        &self.verified_at
    }

    /// Returns the freshness policy evaluated by the verifier.
    #[must_use]
    pub const fn freshness(&self) -> LocalCheckAttestationFreshnessPolicy {
        self.freshness
    }

    /// Returns whether the accepted stdout summary was truncated.
    #[must_use]
    pub const fn stdout_truncated(&self) -> bool {
        self.stdout_truncated
    }

    /// Returns whether the accepted stderr summary was truncated.
    #[must_use]
    pub const fn stderr_truncated(&self) -> bool {
        self.stderr_truncated
    }

    /// Returns the canonical accepted-proof fingerprint.
    #[must_use]
    pub const fn proof_fingerprint(&self) -> &SpecContentHash {
        &self.proof_fingerprint
    }
}

impl fmt::Debug for AcceptedLocalCheckAttestation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptedLocalCheckAttestation")
            .field("assurance", &self.assurance)
            .field("identities", &"[REDACTED]")
            .field("fingerprints", &"[REDACTED]")
            .field("result_status", &self.result_status)
            .field("exit_code_posture", &self.exit_code_posture)
            .field("timestamps", &"[REDACTED]")
            .field("freshness", &self.freshness)
            .field("stdout_truncated", &self.stdout_truncated)
            .field("stderr_truncated", &self.stderr_truncated)
            .finish_non_exhaustive()
    }
}

// Keep the security-sensitive verification order linear and directly auditable.
#[allow(clippy::too_many_lines)]
pub(crate) fn verify_local_check_attestation(
    input: &LocalCheckAttestationVerificationInput<'_>,
) -> Result<AcceptedLocalCheckAttestation, WorkflowOsError> {
    input.execution_binding.validate()?;
    input.command_contract.validate()?;
    input.result.validate()?;
    let stored_manifest = input.stored_immutable_run_bundle.manifest();
    let stored_run_binding = stored_manifest.run_binding();

    if input.requirement.requirement_fingerprint
        != compute_requirement_fingerprint(input.requirement)
        || input.candidate.requirement_fingerprint != input.requirement.requirement_fingerprint
    {
        return Err(verification_error(
            "requirement_mismatch",
            "local check attestation requirement does not match",
        ));
    }
    if input.candidate.binding_fingerprint != compute_binding_fingerprint(input.candidate) {
        return Err(verification_error(
            "binding_mismatch",
            "local check attestation candidate binding does not match",
        ));
    }
    if input.execution_binding.immutable_run_bundle() != &stored_run_binding
        || input.candidate.immutable_run_bundle != stored_run_binding
        || input.observation.immutable_run_bundle != stored_run_binding
    {
        return Err(verification_error(
            "bundle_mismatch",
            "local check attestation immutable bundle does not match",
        ));
    }
    if input.execution_binding.workflow_id() != stored_manifest.workflow_id()
        || input.execution_binding.run_id() != stored_manifest.run_id()
    {
        return Err(verification_error(
            "bundle_mismatch",
            "local check attestation immutable bundle does not match",
        ));
    }
    if input.candidate.workflow_id != *input.execution_binding.workflow_id()
        || input.candidate.run_id != *input.execution_binding.run_id()
        || input.candidate.step_id != *input.execution_binding.step_id()
        || input.observation.workflow_id != *input.execution_binding.workflow_id()
        || input.observation.run_id != *input.execution_binding.run_id()
        || input.observation.step_id != *input.execution_binding.step_id()
    {
        return Err(verification_error(
            "observation_mismatch",
            "local check attestation execution identity does not match",
        ));
    }

    let command_fingerprint =
        compute_local_check_command_contract_fingerprint(input.command_contract);
    if input.command_contract.command_id() != input.requirement.command_id()
        || input.command_contract.command_id() != input.execution_binding.command_id()
        || input.command_contract.command_id() != &input.candidate.command_id
        || input.command_contract.command_id() != input.result.command_id()
        || input.command_contract.command_kind() != input.execution_binding.command_kind()
        || input.command_contract.command_kind() != input.result.command_kind()
        || &command_fingerprint != input.execution_binding.command_contract_fingerprint()
        || command_fingerprint != input.candidate.command_contract_fingerprint
        || command_fingerprint != input.observation.command_contract_fingerprint
    {
        return Err(verification_error(
            "command_mismatch",
            "local check attestation command contract does not match",
        ));
    }
    if input.execution_binding.handler_selection().posture()
        != ImmutableLocalCheckHandlerPosture::RegisteredUnattested
        || input
            .execution_binding
            .handler_selection()
            .selection_fingerprint()
            != &input.candidate.handler_fingerprint
        || input
            .execution_binding
            .handler_selection()
            .selection_fingerprint()
            != &input.observation.handler_selection_fingerprint
        || input.execution_binding.effective_policy_fingerprint()
            != &input.observation.effective_policy_fingerprint
    {
        return Err(verification_error(
            "handler_mismatch",
            "local check attestation handler selection does not match",
        ));
    }
    if input.candidate.assurance != LocalCheckAttestationAssurance::KernelObservedLocalProcess
        || input.candidate.source != LocalCheckAttestationSource::KernelLocalProcessRunner
        || input.requirement.minimum_assurance
            != LocalCheckAttestationAssurance::KernelObservedLocalProcess
    {
        return Err(verification_error(
            "assurance_insufficient",
            "local check attestation assurance is insufficient",
        ));
    }
    if input.candidate.invocation_id != input.observation.invocation_id
        || input.candidate.idempotency_key != input.observation.idempotency_key
        || input.candidate.result_id != input.observation.result_id
        || input.candidate.result_status != input.observation.result_status
        || input.candidate.exit_code_posture != input.observation.exit_code_posture
        || input.candidate.stdout_truncated != input.observation.stdout_truncated
        || input.candidate.stderr_truncated != input.observation.stderr_truncated
        || input.candidate.observed_started_at != input.observation.started_at
        || input.candidate.observed_completed_at != input.observation.completed_at
    {
        return Err(verification_error(
            "observation_mismatch",
            "local check attestation observation does not match",
        ));
    }

    let result_exit = exit_posture(input.result);
    if input.result.status() != input.observation.result_status
        || result_exit != input.observation.exit_code_posture
        || input.result.duration_ms() != input.observation.duration_ms
        || input.result.stdout_truncated() != input.observation.stdout_truncated
        || input.result.stderr_truncated() != input.observation.stderr_truncated
    {
        return Err(verification_error(
            "result_mismatch",
            "local check attestation structured result does not match",
        ));
    }
    if !input
        .requirement
        .accepted_statuses
        .contains(&input.result.status())
    {
        return Err(verification_error(
            "status_not_accepted",
            "local check attestation status is not accepted",
        ));
    }
    if !input.requirement.truncation_allowed
        && (input.result.stdout_truncated() || input.result.stderr_truncated())
    {
        return Err(verification_error(
            "policy_exceeded",
            "local check attestation policy was exceeded",
        ));
    }
    let timeout_ms = u64::from(input.command_contract.timeout_seconds())
        .checked_mul(1_000)
        .ok_or_else(|| {
            verification_error(
                "policy_exceeded",
                "local check attestation policy was exceeded",
            )
        })?;
    if input.result.duration_ms() > timeout_ms {
        return Err(verification_error(
            "policy_exceeded",
            "local check attestation policy was exceeded",
        ));
    }
    if input.execution_binding.created_at() > &input.observation.started_at
        || input.observation.completed_at > input.evaluated_at
    {
        return Err(verification_error(
            "time_invalid",
            "local check attestation time ordering is invalid",
        ));
    }
    if input.candidate.freshness != input.requirement.freshness {
        return Err(verification_error(
            "requirement_mismatch",
            "local check attestation freshness requirement does not match",
        ));
    }
    if let LocalCheckAttestationFreshnessPolicy::MaxAgeSeconds { seconds } =
        input.requirement.freshness
    {
        let age = input.evaluated_at.as_offset_date_time()
            - input.observation.completed_at.as_offset_date_time();
        if age.is_negative() || age > time::Duration::seconds(i64::from(seconds)) {
            return Err(verification_error(
                "freshness_expired",
                "local check attestation freshness has expired",
            ));
        }
    }

    let mut accepted = AcceptedLocalCheckAttestation {
        assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
        requirement_fingerprint: input.requirement.requirement_fingerprint.clone(),
        candidate_binding_fingerprint: input.candidate.binding_fingerprint.clone(),
        execution_binding_fingerprint: input.execution_binding.binding_fingerprint().clone(),
        immutable_run_bundle: stored_run_binding,
        workflow_id: input.observation.workflow_id.clone(),
        run_id: input.observation.run_id.clone(),
        step_id: input.observation.step_id.clone(),
        invocation_id: input.observation.invocation_id.clone(),
        result_id: input.observation.result_id.clone(),
        handler_selection_fingerprint: input.observation.handler_selection_fingerprint.clone(),
        result_status: input.observation.result_status,
        exit_code_posture: input.observation.exit_code_posture,
        observed_completed_at: input.observation.completed_at,
        verified_at: input.evaluated_at,
        freshness: input.requirement.freshness,
        stdout_truncated: input.observation.stdout_truncated,
        stderr_truncated: input.observation.stderr_truncated,
        proof_fingerprint: SpecContentHash::from_text("pending"),
    };
    accepted.proof_fingerprint = compute_accepted_proof_fingerprint(&accepted);
    Ok(accepted)
}

pub(super) fn exit_posture(result: &LocalCheckResult) -> LocalCheckAttestationExitCodePosture {
    match result.exit_code() {
        Some(0) => LocalCheckAttestationExitCodePosture::Zero,
        Some(_) => LocalCheckAttestationExitCodePosture::NonZero,
        None => LocalCheckAttestationExitCodePosture::Unavailable,
    }
}

fn compute_accepted_proof_fingerprint(accepted: &AcceptedLocalCheckAttestation) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "algorithm",
        "workflow-os/accepted-local-check-attestation/v1",
    );
    hash_field(
        &mut hasher,
        "requirement_fingerprint",
        accepted.requirement_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "candidate_binding_fingerprint",
        accepted.candidate_binding_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "execution_binding_fingerprint",
        accepted.execution_binding_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_root",
        accepted.immutable_run_bundle.root_hash().as_str(),
    );
    hash_field(&mut hasher, "workflow_id", accepted.workflow_id.as_str());
    hash_field(&mut hasher, "run_id", accepted.run_id.as_str());
    hash_field(&mut hasher, "step_id", accepted.step_id.as_str());
    hash_field(
        &mut hasher,
        "invocation_id",
        accepted.invocation_id.as_str(),
    );
    hash_field(&mut hasher, "result_id", accepted.result_id.as_str());
    hash_field(
        &mut hasher,
        "handler_selection_fingerprint",
        accepted.handler_selection_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "result_status",
        &accepted.result_status.to_string(),
    );
    hash_field(
        &mut hasher,
        "exit_code_posture",
        match accepted.exit_code_posture {
            LocalCheckAttestationExitCodePosture::Zero => "zero",
            LocalCheckAttestationExitCodePosture::NonZero => "non_zero",
            LocalCheckAttestationExitCodePosture::Unavailable => "unavailable",
        },
    );
    hash_field(
        &mut hasher,
        "observed_completed_at",
        &accepted.observed_completed_at.to_rfc3339(),
    );
    hash_field(
        &mut hasher,
        "verified_at",
        &accepted.verified_at.to_rfc3339(),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn verification_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    attestation_error(&format!("verify.{suffix}"), message)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::too_many_lines)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::json;

    use super::*;
    use crate::{
        build_immutable_run_bundle, load_project, ActorId,
        ImmutableLocalCheckExecutionBindingDefinition, ImmutableLocalCheckHandlerSelection,
        ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalCheckAttestationAlgorithm,
        LocalCheckAttestationBindingDefinition, LocalCheckAttestationId,
        LocalCheckAttestationRequirementDefinition, LocalCheckCommandId, LocalCheckCommandKind,
        LocalCheckResultDefinition, LocalImmutableRunBundleStore, SkillId, SkillVersion,
        SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot {
        path: PathBuf,
    }

    impl TestRoot {
        fn new(name: &str) -> Self {
            let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-local-check-verifier-{name}-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.path.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("parent created");
            }
            fs::write(path, content).expect("fixture written");
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn stored_bundle(workflow_name: &str) -> StoredImmutableRunBundle {
        let project = TestRoot::new("project");
        let storage = TestRoot::new("storage");
        project.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: verifier/project\n  name: Verifier Project\n"
            ),
        );
        project.write(
            "workflows/check.workflow.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: workflow/test\nversion: v1\ndisplay_name: {workflow_name}\ntriggers:\n  - id: manual-start\n    kind: manual\nsteps:\n  - id: check-docs\n    skill_ref:\n      id: local/check-docs\n      version: v0\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated, RunCompleted]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project.write(
            "skills/check.skill.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check-docs\nversion: v0\ndisplay_name: Check Docs\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: check_failed\n    description: Check failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read Only\nrules:\n  - id: allow-local\n    effect: allow_local\n"
            ),
        );

        let loaded = load_project(project.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project_bundle = loaded.bundle.expect("loaded project");
        let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
            project: &project_bundle,
            workflow_id: &WorkflowId::new("workflow/test").expect("workflow id"),
            bundle_id: ImmutableRunBundleId::new("bundle/test").expect("bundle id"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
            run_id: WorkflowRunId::new("run-test").expect("run id"),
            resolved_execution_context_hash: SpecContentHash::from_text("resolved context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
            )
            .expect("execution posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check-docs").expect("skill id"),
                skill_version: SkillVersion::new("v0").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: Timestamp::parse_rfc3339("2026-07-19T11:59:59Z").expect("timestamp"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        })
        .expect("bundle built");
        let store = LocalImmutableRunBundleStore::new(storage.path());
        store.write_bundle(&built).expect("bundle written");
        store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("bundle read")
    }

    struct Fixture {
        requirement: LocalCheckAttestationRequirement,
        candidate: LocalCheckAttestationBinding,
        stored_bundle: StoredImmutableRunBundle,
        execution_binding: ImmutableLocalCheckExecutionBinding,
        contract: LocalCheckCommandContract,
        observation: KernelObservedLocalCheck,
        result: LocalCheckResult,
        evaluated_at: Timestamp,
    }

    impl Fixture {
        fn input(&self) -> LocalCheckAttestationVerificationInput<'_> {
            LocalCheckAttestationVerificationInput {
                requirement: &self.requirement,
                candidate: &self.candidate,
                stored_immutable_run_bundle: &self.stored_bundle,
                execution_binding: &self.execution_binding,
                command_contract: &self.contract,
                observation: &self.observation,
                result: &self.result,
                evaluated_at: self.evaluated_at,
            }
        }
    }

    fn make_fixture(freshness: LocalCheckAttestationFreshnessPolicy) -> Fixture {
        let stored_bundle = stored_bundle("Check Workflow");
        let bundle = stored_bundle.manifest().run_binding();
        let workflow_id = WorkflowId::new("workflow/test").expect("workflow id");
        let run_id = WorkflowRunId::new("run-test").expect("run id");
        let step_id = StepId::new("check-docs").expect("step id");
        let skill_id = SkillId::new("local/check-docs").expect("skill id");
        let skill_version = SkillVersion::new("v0").expect("skill version");
        let invocation_id = SkillInvocationId::new("invocation/check-docs").expect("invocation");
        let idempotency_key = IdempotencyKey::new("idempotency/check-docs").expect("key");
        let result_id = LocalCheckResultId::new("result/check-docs").expect("result id");
        let started_at = Timestamp::parse_rfc3339("2026-07-19T12:00:01Z").expect("start");
        let completed_at = Timestamp::parse_rfc3339("2026-07-19T12:00:02Z").expect("complete");
        let evaluated_at = Timestamp::parse_rfc3339("2026-07-19T12:00:03Z").expect("evaluated");
        let contract = LocalCheckCommandContract::docs_check_model_only().expect("contract");
        let handler = ImmutableLocalCheckHandlerSelection::registered_unattested(
            LocalCheckCommandKind::DocsCheck,
            skill_id.clone(),
            skill_version.clone(),
        );
        let execution_binding = ImmutableLocalCheckExecutionBinding::new(
            ImmutableLocalCheckExecutionBindingDefinition {
                immutable_run_bundle: bundle.clone(),
                workflow_id: workflow_id.clone(),
                run_id: run_id.clone(),
                step_id: step_id.clone(),
                skill_id,
                skill_version,
                command_contract: &contract,
                handler_selection: handler,
                created_at: Timestamp::parse_rfc3339("2026-07-19T12:00:00Z").expect("binding time"),
            },
        )
        .expect("execution binding");
        let requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: LocalCheckCommandId::new("local-check/docs").expect("command id"),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Passed],
                freshness,
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .expect("requirement");
        let result = LocalCheckResult::new(LocalCheckResultDefinition {
            command_id: contract.command_id().clone(),
            command_kind: contract.command_kind(),
            status: LocalCheckResultStatus::Passed,
            exit_code: Some(0),
            duration_ms: 1_000,
            stdout_summary: "documentation check passed".to_owned(),
            stderr_summary: String::new(),
            stdout_truncated: false,
            stderr_truncated: false,
            error_code: None,
        })
        .expect("result");
        let observation = KernelObservedLocalCheck::new(KernelObservedLocalCheckDefinition {
            workflow_id: workflow_id.clone(),
            run_id: run_id.clone(),
            step_id: step_id.clone(),
            invocation_id: invocation_id.clone(),
            idempotency_key: idempotency_key.clone(),
            immutable_run_bundle: bundle.clone(),
            command_contract_fingerprint: execution_binding.command_contract_fingerprint().clone(),
            handler_selection_fingerprint: execution_binding
                .handler_selection()
                .selection_fingerprint()
                .clone(),
            effective_policy_fingerprint: execution_binding.effective_policy_fingerprint().clone(),
            result_id: result_id.clone(),
            result_status: result.status(),
            exit_code_posture: LocalCheckAttestationExitCodePosture::Zero,
            duration_ms: result.duration_ms(),
            timed_out: false,
            stdout_truncated: false,
            stderr_truncated: false,
            started_at,
            completed_at,
        })
        .expect("observation");
        let candidate = LocalCheckAttestationBinding::new(LocalCheckAttestationBindingDefinition {
            attestation_id: LocalCheckAttestationId::new("attestation/check-docs")
                .expect("attestation id"),
            algorithm: LocalCheckAttestationAlgorithm::V1,
            assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
            source: LocalCheckAttestationSource::KernelLocalProcessRunner,
            command_id: contract.command_id().clone(),
            command_contract_fingerprint: execution_binding.command_contract_fingerprint().clone(),
            requirement_fingerprint: requirement.requirement_fingerprint().clone(),
            immutable_run_bundle: bundle.clone(),
            workflow_id,
            run_id,
            step_id,
            invocation_id,
            idempotency_key,
            handler_fingerprint: execution_binding
                .handler_selection()
                .selection_fingerprint()
                .clone(),
            result_id,
            result_status: result.status(),
            exit_code_posture: LocalCheckAttestationExitCodePosture::Zero,
            stdout_truncated: false,
            stderr_truncated: false,
            observed_started_at: started_at,
            observed_completed_at: completed_at,
            freshness,
        })
        .expect("candidate");
        Fixture {
            requirement,
            candidate,
            stored_bundle,
            execution_binding,
            contract,
            observation,
            result,
            evaluated_at,
        }
    }

    #[test]
    fn exact_kernel_owned_context_returns_accepted_proof() {
        let fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        let accepted = verify_local_check_attestation(&fixture.input()).expect("accepted proof");
        assert_eq!(
            accepted.assurance(),
            LocalCheckAttestationAssurance::KernelObservedLocalProcess
        );
        assert_eq!(accepted.result_status(), LocalCheckResultStatus::Passed);
        assert_eq!(
            accepted.execution_binding_fingerprint(),
            fixture.execution_binding.binding_fingerprint()
        );
        assert_eq!(
            accepted.immutable_run_bundle(),
            &fixture.stored_bundle.manifest().run_binding()
        );
        assert!(!fixture.stored_bundle.definition_records().is_empty());
        assert_eq!(
            accepted.proof_fingerprint().as_str(),
            "1e30df3ab6665557fccca17eabaee45742adac18b88663f2027787a9b875b1aa"
        );
    }

    #[test]
    fn consistently_relabelled_execution_context_cannot_escape_stored_manifest_identity() {
        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        let workflow_id = WorkflowId::new("workflow/relabelled").expect("workflow id");
        let run_id = WorkflowRunId::new("run-relabelled").expect("run id");
        let skill_id = SkillId::new("local/check-docs").expect("skill id");
        let skill_version = SkillVersion::new("v0").expect("skill version");
        let handler = ImmutableLocalCheckHandlerSelection::registered_unattested(
            LocalCheckCommandKind::DocsCheck,
            skill_id.clone(),
            skill_version.clone(),
        );
        fixture.execution_binding = ImmutableLocalCheckExecutionBinding::new(
            ImmutableLocalCheckExecutionBindingDefinition {
                immutable_run_bundle: fixture.stored_bundle.manifest().run_binding(),
                workflow_id: workflow_id.clone(),
                run_id: run_id.clone(),
                step_id: StepId::new("check-docs").expect("step id"),
                skill_id,
                skill_version,
                command_contract: &fixture.contract,
                handler_selection: handler,
                created_at: Timestamp::parse_rfc3339("2026-07-19T12:00:00Z").expect("binding time"),
            },
        )
        .expect("relabelled execution binding");
        fixture.candidate.workflow_id = workflow_id.clone();
        fixture.candidate.run_id = run_id.clone();
        fixture.candidate.binding_fingerprint = compute_binding_fingerprint(&fixture.candidate);
        fixture.observation.workflow_id = workflow_id;
        fixture.observation.run_id = run_id;

        let error = verify_local_check_attestation(&fixture.input())
            .expect_err("stored manifest identity must remain authoritative");

        assert_eq!(
            error.code(),
            "local_check_attestation.verify.bundle_mismatch"
        );
    }

    #[test]
    fn command_and_observation_substitution_fail_closed() {
        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.observation.command_contract_fingerprint = SpecContentHash::from_text("changed");
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.command_mismatch"
        );

        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.observation.result_id = LocalCheckResultId::new("result/other").expect("result id");
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.observation_mismatch"
        );
    }

    #[test]
    fn bundle_assurance_and_time_substitution_fail_closed() {
        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.stored_bundle = stored_bundle("Changed Workflow");
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.bundle_mismatch"
        );

        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.observation.immutable_run_bundle = serde_json::from_value(json!({
            "bundle_id": "bundle/other",
            "bundle_version": "v1",
            "root_hash": SpecContentHash::from_text("other-root").as_str(),
        }))
        .expect("other bundle");
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.bundle_mismatch"
        );

        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.candidate.assurance = LocalCheckAttestationAssurance::CallerAsserted;
        fixture.candidate.source = LocalCheckAttestationSource::Caller;
        fixture.candidate.binding_fingerprint = compute_binding_fingerprint(&fixture.candidate);
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.assurance_insufficient"
        );

        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.evaluated_at =
            Timestamp::parse_rfc3339("2026-07-19T12:00:01Z").expect("early time");
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(error.code(), "local_check_attestation.verify.time_invalid");
    }

    #[test]
    fn structured_result_duration_and_exit_mismatch_fail_closed() {
        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.result = LocalCheckResult::new(LocalCheckResultDefinition {
            command_id: fixture.contract.command_id().clone(),
            command_kind: fixture.contract.command_kind(),
            status: LocalCheckResultStatus::Passed,
            exit_code: Some(0),
            duration_ms: 999,
            stdout_summary: "documentation check passed".to_owned(),
            stderr_summary: String::new(),
            stdout_truncated: false,
            stderr_truncated: false,
            error_code: None,
        })
        .expect("changed result");
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.result_mismatch"
        );
    }

    #[test]
    fn status_truncation_and_duration_policy_fail_closed() {
        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: fixture.contract.command_id().clone(),
                minimum_assurance: LocalCheckAttestationAssurance::KernelObservedLocalProcess,
                accepted_statuses: vec![LocalCheckResultStatus::Failed],
                freshness: LocalCheckAttestationFreshnessPolicy::NoReuse,
                exact_immutable_run_binding_required: true,
                truncation_allowed: false,
            })
            .expect("requirement");
        fixture.candidate.requirement_fingerprint =
            fixture.requirement.requirement_fingerprint().clone();
        fixture.candidate.binding_fingerprint = compute_binding_fingerprint(&fixture.candidate);
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.status_not_accepted"
        );

        let mut fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        fixture.result = LocalCheckResult::new(LocalCheckResultDefinition {
            command_id: fixture.contract.command_id().clone(),
            command_kind: fixture.contract.command_kind(),
            status: LocalCheckResultStatus::Passed,
            exit_code: Some(0),
            duration_ms: 1_000,
            stdout_summary: "documentation check passed".to_owned(),
            stderr_summary: String::new(),
            stdout_truncated: true,
            stderr_truncated: false,
            error_code: None,
        })
        .expect("truncated result");
        fixture.observation.stdout_truncated = true;
        fixture.candidate.stdout_truncated = true;
        fixture.candidate.binding_fingerprint = compute_binding_fingerprint(&fixture.candidate);
        let error = verify_local_check_attestation(&fixture.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.policy_exceeded"
        );
    }

    #[test]
    fn freshness_boundary_is_deterministic() {
        let freshness =
            LocalCheckAttestationFreshnessPolicy::max_age_seconds(1).expect("freshness");
        let boundary = make_fixture(freshness);
        verify_local_check_attestation(&boundary.input()).expect("exact boundary accepted");

        let mut stale = make_fixture(freshness);
        stale.evaluated_at =
            Timestamp::parse_rfc3339("2026-07-19T12:00:03.001Z").expect("stale time");
        let error = verify_local_check_attestation(&stale.input()).expect_err("must reject");
        assert_eq!(
            error.code(),
            "local_check_attestation.verify.freshness_expired"
        );
    }

    #[test]
    fn debug_output_redacts_bound_context() {
        let fixture = make_fixture(LocalCheckAttestationFreshnessPolicy::NoReuse);
        let observation_debug = format!("{:?}", fixture.observation);
        let accepted = verify_local_check_attestation(&fixture.input()).expect("accepted proof");
        let accepted_debug = format!("{accepted:?}");
        for forbidden in ["workflow/test", "run-test", "check-docs", "bundle/test"] {
            assert!(!observation_debug.contains(forbidden));
            assert!(!accepted_debug.contains(forbidden));
        }
    }
}
