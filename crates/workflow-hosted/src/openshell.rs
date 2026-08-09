//! Optional OpenShell-backed, no-write hosted execution provider.
//!
//! The provider owns the governed lifecycle and validation boundary. An
//! injected client owns `OpenShell` transport. No caller-selected command,
//! credential, provider mutation, or filesystem write is represented here.

use std::fmt;
use std::sync::Arc;

use workflow_core::{
    HostedExecutionAttemptPosture, HostedExecutionAttestation, HostedExecutionCleanupPosture,
    HostedExecutionControlPosture, HostedExecutionEnforcementMode, HostedExecutionErrorCategory,
    HostedExecutionInvocationError, HostedExecutionObservationSummary,
    HostedExecutionPolicyRevision, HostedExecutionProvider, HostedExecutionProviderId,
    HostedExecutionProviderVersion, HostedExecutionReceipt, HostedExecutionReference,
    HostedExecutionReferenceKind, HostedExecutionRequest, HostedExecutionStatus, SpecContentHash,
    Timestamp, WorkflowOsError,
};

/// Provider-observed `OpenShell` sandbox security posture.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenShellSandboxSnapshot {
    environment_reference: HostedExecutionReference,
    runtime_image_digest: SpecContentHash,
    effective_policy_revision: HostedExecutionPolicyRevision,
    effective_policy_hash: SpecContentHash,
    enforcement_mode: HostedExecutionEnforcementMode,
    filesystem_control: HostedExecutionControlPosture,
    process_control: HostedExecutionControlPosture,
    network_control: HostedExecutionControlPosture,
}

impl OpenShellSandboxSnapshot {
    /// Creates one payload-free sandbox snapshot.
    ///
    /// # Errors
    ///
    /// Rejects an environment reference outside the telemetry family.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        environment_reference: HostedExecutionReference,
        runtime_image_digest: SpecContentHash,
        effective_policy_revision: HostedExecutionPolicyRevision,
        effective_policy_hash: SpecContentHash,
        enforcement_mode: HostedExecutionEnforcementMode,
        filesystem_control: HostedExecutionControlPosture,
        process_control: HostedExecutionControlPosture,
        network_control: HostedExecutionControlPosture,
    ) -> Result<Self, WorkflowOsError> {
        if environment_reference.kind() != HostedExecutionReferenceKind::Telemetry {
            return Err(WorkflowOsError::validation(
                "hosted.openshell.snapshot.environment_reference.invalid",
                "OpenShell environment reference is invalid",
            ));
        }
        Ok(Self {
            environment_reference,
            runtime_image_digest,
            effective_policy_revision,
            effective_policy_hash,
            enforcement_mode,
            filesystem_control,
            process_control,
            network_control,
        })
    }

    /// Returns the stable sandbox reference.
    #[must_use]
    pub const fn environment_reference(&self) -> &HostedExecutionReference {
        &self.environment_reference
    }
}

impl fmt::Debug for OpenShellSandboxSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellSandboxSnapshot")
            .field("identity", &"[REDACTED]")
            .field("enforcement_mode", &self.enforcement_mode)
            .field("filesystem_control", &self.filesystem_control)
            .field("process_control", &self.process_control)
            .field("network_control", &self.network_control)
            .finish_non_exhaustive()
    }
}

/// Bounded outcome from the provider-owned fixed no-write operation.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenShellFixedOperationOutcome {
    exit_status: i32,
    observations: HostedExecutionObservationSummary,
    references: Vec<HostedExecutionReference>,
}

impl OpenShellFixedOperationOutcome {
    /// Creates a fixed-operation outcome.
    ///
    /// # Errors
    ///
    /// Rejects unsupported reference kinds or missing denied-egress proof.
    pub fn new(
        exit_status: i32,
        observations: HostedExecutionObservationSummary,
        references: Vec<HostedExecutionReference>,
    ) -> Result<Self, WorkflowOsError> {
        if references.iter().any(|reference| {
            !matches!(
                reference.kind(),
                HostedExecutionReferenceKind::Artifact
                    | HostedExecutionReferenceKind::Log
                    | HostedExecutionReferenceKind::DeniedAction
                    | HostedExecutionReferenceKind::Telemetry
            )
        }) {
            return Err(WorkflowOsError::validation(
                "hosted.openshell.fixed_operation.reference.invalid",
                "OpenShell fixed-operation reference is invalid",
            ));
        }
        if observations.denied_network_events() == 0
            || !references
                .iter()
                .any(|reference| reference.kind() == HostedExecutionReferenceKind::DeniedAction)
        {
            return Err(WorkflowOsError::validation(
                "hosted.openshell.fixed_operation.denied_egress_proof.missing",
                "OpenShell fixed operation requires denied-egress proof",
            ));
        }
        Ok(Self {
            exit_status,
            observations,
            references,
        })
    }
}

impl fmt::Debug for OpenShellFixedOperationOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellFixedOperationOutcome")
            .field("exit_status", &self.exit_status)
            .field("observations", &self.observations)
            .field("reference_count", &self.references.len())
            .finish_non_exhaustive()
    }
}

/// Injected `OpenShell` lifecycle transport.
pub trait OpenShellNoWriteClient: Send + Sync {
    /// Provisions a sandbox under the request's exact policy binding.
    ///
    /// # Errors
    ///
    /// Returns a bounded invocation failure when provisioning cannot be proven.
    fn create_sandbox(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<OpenShellSandboxSnapshot, HostedExecutionInvocationError>;

    /// Executes the provider-owned fixed no-write operation.
    ///
    /// # Errors
    ///
    /// Returns a bounded invocation failure when execution cannot be proven.
    fn execute_fixed_operation(
        &self,
        sandbox: &OpenShellSandboxSnapshot,
    ) -> Result<OpenShellFixedOperationOutcome, HostedExecutionInvocationError>;

    /// Re-reads the effective security posture after execution.
    ///
    /// # Errors
    ///
    /// Returns a bounded invocation failure when posture cannot be inspected.
    fn inspect_sandbox(
        &self,
        sandbox: &OpenShellSandboxSnapshot,
    ) -> Result<OpenShellSandboxSnapshot, HostedExecutionInvocationError>;

    /// Deletes the sandbox and returns a stable cleanup telemetry reference.
    ///
    /// # Errors
    ///
    /// Returns a bounded invocation failure when cleanup cannot be proven.
    fn delete_sandbox(
        &self,
        sandbox: &OpenShellSandboxSnapshot,
    ) -> Result<HostedExecutionReference, HostedExecutionInvocationError>;
}

/// Optional `OpenShell` provider for one fixed, no-write execution proof.
pub struct OpenShellNoWriteExecutionProvider {
    provider_id: HostedExecutionProviderId,
    provider_version: HostedExecutionProviderVersion,
    configuration_hash: SpecContentHash,
    expected_runtime_image_digest: SpecContentHash,
    client: Arc<dyn OpenShellNoWriteClient>,
}

impl OpenShellNoWriteExecutionProvider {
    /// Creates an optional `OpenShell` provider with an injected transport.
    ///
    /// # Errors
    ///
    /// Returns an error only if the built-in identifiers violate Core rules.
    pub fn new(
        provider_version: HostedExecutionProviderVersion,
        configuration_hash: SpecContentHash,
        expected_runtime_image_digest: SpecContentHash,
        client: Arc<dyn OpenShellNoWriteClient>,
    ) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            provider_id: HostedExecutionProviderId::new("provider/openshell-no-write")?,
            provider_version,
            configuration_hash,
            expected_runtime_image_digest,
            client,
        })
    }

    fn validate_snapshot(
        &self,
        request: &HostedExecutionRequest,
        snapshot: &OpenShellSandboxSnapshot,
    ) -> Result<(), HostedExecutionInvocationError> {
        if snapshot.runtime_image_digest != self.expected_runtime_image_digest
            || snapshot.effective_policy_hash != *request.policy().policy_hash()
            || snapshot.enforcement_mode != HostedExecutionEnforcementMode::Enforce
            || snapshot.filesystem_control != HostedExecutionControlPosture::Enforced
            || snapshot.process_control != HostedExecutionControlPosture::Enforced
            || snapshot.network_control != HostedExecutionControlPosture::Enforced
        {
            return Err(invocation_error(
                HostedExecutionErrorCategory::Policy,
                HostedExecutionAttemptPosture::MayHaveStarted,
            ));
        }
        Ok(())
    }

    fn cleanup_after_error(
        &self,
        sandbox: &OpenShellSandboxSnapshot,
        error: HostedExecutionInvocationError,
    ) -> HostedExecutionInvocationError {
        match self.client.delete_sandbox(sandbox) {
            Ok(_) => error,
            Err(_) => invocation_error(
                HostedExecutionErrorCategory::Ambiguous,
                HostedExecutionAttemptPosture::MayHaveStarted,
            ),
        }
    }
}

impl fmt::Debug for OpenShellNoWriteExecutionProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenShellNoWriteExecutionProvider")
            .field("identity", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl HostedExecutionProvider for OpenShellNoWriteExecutionProvider {
    fn provider_id(&self) -> &HostedExecutionProviderId {
        &self.provider_id
    }

    fn provider_version(&self) -> &HostedExecutionProviderVersion {
        &self.provider_version
    }

    fn configuration_hash(&self) -> &SpecContentHash {
        &self.configuration_hash
    }

    fn requires_attestation(&self) -> bool {
        true
    }

    fn validate_request(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<(), HostedExecutionInvocationError> {
        if !request.approved_side_effects().is_empty()
            || !request.access_material_references().is_empty()
            || request
                .authorized_capabilities()
                .iter()
                .any(|capability| capability.as_str().strip_suffix(".read").is_none())
        {
            return Err(invocation_error(
                HostedExecutionErrorCategory::Policy,
                HostedExecutionAttemptPosture::NotStarted,
            ));
        }
        Ok(())
    }

    fn validate_attestation(
        &self,
        request: &HostedExecutionRequest,
        attestation: &HostedExecutionAttestation,
    ) -> Result<(), WorkflowOsError> {
        if attestation.runtime_image_digest() != &self.expected_runtime_image_digest
            || attestation.effective_policy_hash() != request.policy().policy_hash()
            || !attestation.satisfies_hard_requirements()
        {
            return Err(WorkflowOsError::invalid_state(
                "hosted.openshell.attestation.binding.invalid",
                "OpenShell attestation binding is invalid",
            ));
        }
        Ok(())
    }

    fn execute(
        &self,
        request: &HostedExecutionRequest,
    ) -> Result<HostedExecutionReceipt, HostedExecutionInvocationError> {
        self.validate_request(request)?;
        let started_at = Timestamp::now_utc();
        let sandbox = self.client.create_sandbox(request)?;

        let operation = (|| {
            self.validate_snapshot(request, &sandbox)?;
            let outcome = self.client.execute_fixed_operation(&sandbox)?;
            let final_snapshot = self.client.inspect_sandbox(&sandbox)?;
            self.validate_snapshot(request, &final_snapshot)?;
            if final_snapshot.environment_reference != sandbox.environment_reference
                || final_snapshot.effective_policy_revision != sandbox.effective_policy_revision
                || final_snapshot.effective_policy_hash != sandbox.effective_policy_hash
                || outcome.observations.allowed_network_events() != 0
                || outcome.observations.policy_change_events() != 0
                || outcome.observations.security_findings() != 0
                || outcome.observations.process_start_events() == 0
                || outcome.observations.process_terminal_events() == 0
            {
                return Err(invocation_error(
                    HostedExecutionErrorCategory::Policy,
                    HostedExecutionAttemptPosture::MayHaveStarted,
                ));
            }
            Ok((outcome, final_snapshot))
        })();

        let (outcome, final_snapshot) = match operation {
            Ok(value) => value,
            Err(error) => return Err(self.cleanup_after_error(&sandbox, error)),
        };
        let cleanup_reference = self.client.delete_sandbox(&sandbox).map_err(|_| {
            invocation_error(
                HostedExecutionErrorCategory::Ambiguous,
                HostedExecutionAttemptPosture::MayHaveStarted,
            )
        })?;
        if cleanup_reference.kind() != HostedExecutionReferenceKind::Telemetry {
            return Err(invocation_error(
                HostedExecutionErrorCategory::Protocol,
                HostedExecutionAttemptPosture::MayHaveStarted,
            ));
        }

        let attestation = HostedExecutionAttestation::new(
            final_snapshot.runtime_image_digest,
            final_snapshot.effective_policy_revision,
            final_snapshot.effective_policy_hash,
            final_snapshot.enforcement_mode,
            final_snapshot.filesystem_control,
            final_snapshot.process_control,
            final_snapshot.network_control,
            outcome.observations.clone(),
            HostedExecutionCleanupPosture::Completed,
            cleanup_reference.clone(),
        )
        .map_err(|_| {
            invocation_error(
                HostedExecutionErrorCategory::Protocol,
                HostedExecutionAttemptPosture::MayHaveStarted,
            )
        })?;

        let mut references = outcome.references;
        references.push(outcome.observations.observation_reference().clone());
        references.push(cleanup_reference);
        let (status, error_category) = if outcome.exit_status == 0 {
            (HostedExecutionStatus::Completed, None)
        } else {
            (
                HostedExecutionStatus::Failed,
                Some(HostedExecutionErrorCategory::Execution),
            )
        };
        HostedExecutionReceipt::new_attested(
            self.execution_id(request)?,
            self.provider_id.clone(),
            self.provider_version.clone(),
            self.configuration_hash.clone(),
            request.fingerprint(),
            sandbox.environment_reference,
            request.policy().policy_hash().clone(),
            started_at,
            Timestamp::now_utc(),
            status,
            error_category,
            Some(outcome.exit_status),
            references,
            attestation,
        )
        .map_err(|_| {
            invocation_error(
                HostedExecutionErrorCategory::Protocol,
                HostedExecutionAttemptPosture::MayHaveStarted,
            )
        })
    }
}

fn invocation_error(
    category: HostedExecutionErrorCategory,
    posture: HostedExecutionAttemptPosture,
) -> HostedExecutionInvocationError {
    HostedExecutionInvocationError::new(category, posture)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use workflow_core::{
        CapabilityReference, CorrelationId, HostedExecutionBudget, HostedExecutionPolicyBinding,
        HostedExecutionPolicyId, IdempotencyKey, ImmutableRunBundleId, ImmutableRunBundleVersion,
        SchemaVersion, SideEffectId, StepId, WorkflowId, WorkflowRunId, WorkflowVersion,
    };

    use super::*;

    #[derive(Clone)]
    struct ScriptedClient {
        initial: OpenShellSandboxSnapshot,
        final_snapshot: OpenShellSandboxSnapshot,
        outcome: OpenShellFixedOperationOutcome,
        cleanup_reference: HostedExecutionReference,
        cleanup_fails: bool,
        create_calls: Arc<AtomicUsize>,
        cleanup_calls: Arc<AtomicUsize>,
    }

    impl OpenShellNoWriteClient for ScriptedClient {
        fn create_sandbox(
            &self,
            _request: &HostedExecutionRequest,
        ) -> Result<OpenShellSandboxSnapshot, HostedExecutionInvocationError> {
            self.create_calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.initial.clone())
        }

        fn execute_fixed_operation(
            &self,
            _sandbox: &OpenShellSandboxSnapshot,
        ) -> Result<OpenShellFixedOperationOutcome, HostedExecutionInvocationError> {
            Ok(self.outcome.clone())
        }

        fn inspect_sandbox(
            &self,
            _sandbox: &OpenShellSandboxSnapshot,
        ) -> Result<OpenShellSandboxSnapshot, HostedExecutionInvocationError> {
            Ok(self.final_snapshot.clone())
        }

        fn delete_sandbox(
            &self,
            _sandbox: &OpenShellSandboxSnapshot,
        ) -> Result<HostedExecutionReference, HostedExecutionInvocationError> {
            self.cleanup_calls.fetch_add(1, Ordering::SeqCst);
            if self.cleanup_fails {
                Err(invocation_error(
                    HostedExecutionErrorCategory::Transport,
                    HostedExecutionAttemptPosture::MayHaveStarted,
                ))
            } else {
                Ok(self.cleanup_reference.clone())
            }
        }
    }

    fn reference(kind: HostedExecutionReferenceKind, value: &str) -> HostedExecutionReference {
        HostedExecutionReference::new(kind, value).unwrap_or_else(|error| panic!("{error}"))
    }

    fn snapshot() -> OpenShellSandboxSnapshot {
        OpenShellSandboxSnapshot::new(
            reference(
                HostedExecutionReferenceKind::Telemetry,
                "openshell/sandbox/test",
            ),
            SpecContentHash::from_text("openshell-image"),
            HostedExecutionPolicyRevision::new("revision/1")
                .unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("openshell-policy"),
            HostedExecutionEnforcementMode::Enforce,
            HostedExecutionControlPosture::Enforced,
            HostedExecutionControlPosture::Enforced,
            HostedExecutionControlPosture::Enforced,
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn outcome() -> OpenShellFixedOperationOutcome {
        OpenShellFixedOperationOutcome::new(
            0,
            HostedExecutionObservationSummary::new(
                0,
                1,
                1,
                1,
                0,
                0,
                reference(
                    HostedExecutionReferenceKind::Telemetry,
                    "openshell/observations/test",
                ),
            )
            .unwrap_or_else(|error| panic!("{error}")),
            vec![reference(
                HostedExecutionReferenceKind::DeniedAction,
                "openshell/denied-egress/test",
            )],
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn client() -> ScriptedClient {
        ScriptedClient {
            initial: snapshot(),
            final_snapshot: snapshot(),
            outcome: outcome(),
            cleanup_reference: reference(
                HostedExecutionReferenceKind::Telemetry,
                "openshell/cleanup/test",
            ),
            cleanup_fails: false,
            create_calls: Arc::new(AtomicUsize::new(0)),
            cleanup_calls: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn request(side_effects: Vec<SideEffectId>) -> HostedExecutionRequest {
        HostedExecutionRequest::new(
            WorkflowRunId::new("run-openshell-test").unwrap_or_else(|error| panic!("{error}")),
            WorkflowId::new("hosted/openshell-test").unwrap_or_else(|error| panic!("{error}")),
            WorkflowVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SchemaVersion::new("workflowos.dev/v0").unwrap_or_else(|error| panic!("{error}")),
            StepId::new("verify").unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleId::new("bundle-openshell-test")
                .unwrap_or_else(|error| panic!("{error}")),
            ImmutableRunBundleVersion::new("v1").unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("bundle"),
            Vec::new(),
            vec![CapabilityReference::new("repository.read")
                .unwrap_or_else(|error| panic!("{error}"))],
            side_effects,
            HostedExecutionPolicyBinding::new(
                HostedExecutionPolicyId::new("policy/openshell-no-write")
                    .unwrap_or_else(|error| panic!("{error}")),
                SpecContentHash::from_text("openshell-policy"),
            ),
            HostedExecutionBudget::new(30, 1024).unwrap_or_else(|error| panic!("{error}")),
            CorrelationId::new("correlation-openshell-test")
                .unwrap_or_else(|error| panic!("{error}")),
            IdempotencyKey::new("openshell-test").unwrap_or_else(|error| panic!("{error}")),
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    fn provider(client: ScriptedClient) -> OpenShellNoWriteExecutionProvider {
        OpenShellNoWriteExecutionProvider::new(
            HostedExecutionProviderVersion::new("v0alpha1")
                .unwrap_or_else(|error| panic!("{error}")),
            SpecContentHash::from_text("openshell-provider-configuration"),
            SpecContentHash::from_text("openshell-image"),
            Arc::new(client),
        )
        .unwrap_or_else(|error| panic!("{error}"))
    }

    #[test]
    fn fixed_no_write_operation_returns_attested_receipt() {
        let scripted = client();
        let cleanup_calls = Arc::clone(&scripted.cleanup_calls);
        let provider = provider(scripted);
        let request = request(Vec::new());
        let receipt = provider
            .execute(&request)
            .unwrap_or_else(|error| panic!("{error:?}"));

        assert_eq!(receipt.status(), HostedExecutionStatus::Completed);
        assert!(receipt.attestation().is_some());
        receipt
            .validate_for(&request, &provider)
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(cleanup_calls.load(Ordering::SeqCst), 1);
        assert!(receipt
            .references()
            .iter()
            .any(|reference| { reference.kind() == HostedExecutionReferenceKind::DeniedAction }));
    }

    #[test]
    fn serialized_policy_binding_tamper_fails_closed_without_leaking() {
        let private_marker = "private-policy-marker";
        let receipt = provider(client())
            .execute(&request(Vec::new()))
            .unwrap_or_else(|error| panic!("{error:?}"));
        let mut wire = serde_json::to_value(receipt)
            .unwrap_or_else(|error| panic!("failed to serialize receipt: {error}"));
        wire["attestation"]["effective_policy_hash"] =
            serde_json::to_value(SpecContentHash::from_text(private_marker))
                .unwrap_or_else(|error| panic!("failed to serialize hash: {error}"));

        let error = serde_json::from_value::<HostedExecutionReceipt>(wire)
            .expect_err("policy binding tamper must fail closed");
        let rendered = error.to_string();

        assert!(!rendered.contains(private_marker));
        assert!(!rendered.is_empty());
    }

    #[test]
    fn provider_specific_image_tamper_is_rejected_without_leaking() {
        let private_marker = "private-image-marker";
        let provider = provider(client());
        let request = request(Vec::new());
        let receipt = provider
            .execute(&request)
            .unwrap_or_else(|error| panic!("{error:?}"));
        let mut wire = serde_json::to_value(receipt)
            .unwrap_or_else(|error| panic!("failed to serialize receipt: {error}"));
        wire["attestation"]["runtime_image_digest"] =
            serde_json::to_value(SpecContentHash::from_text(private_marker))
                .unwrap_or_else(|error| panic!("failed to serialize hash: {error}"));
        let tampered = serde_json::from_value::<HostedExecutionReceipt>(wire)
            .unwrap_or_else(|error| panic!("receipt should remain structurally valid: {error}"));

        let error = tampered
            .validate_for(&request, &provider)
            .expect_err("provider image binding tamper must fail closed");

        assert_eq!(error.code(), "hosted.openshell.attestation.binding.invalid");
        assert!(!error.to_string().contains(private_marker));
    }

    #[test]
    fn audit_mode_is_rejected_and_sandbox_is_cleaned_up() {
        let mut scripted = client();
        scripted.initial.enforcement_mode = HostedExecutionEnforcementMode::Audit;
        let cleanup_calls = Arc::clone(&scripted.cleanup_calls);
        let error = provider(scripted)
            .execute(&request(Vec::new()))
            .expect_err("audit mode must fail closed");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Policy);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
        assert_eq!(cleanup_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn policy_revision_drift_is_rejected() {
        let mut scripted = client();
        scripted.final_snapshot.effective_policy_revision =
            HostedExecutionPolicyRevision::new("revision/2")
                .unwrap_or_else(|error| panic!("{error}"));
        let error = provider(scripted)
            .execute(&request(Vec::new()))
            .expect_err("policy revision drift must fail closed");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Policy);
    }

    #[test]
    fn cleanup_failure_is_ambiguous() {
        let mut scripted = client();
        scripted.cleanup_fails = true;
        let error = provider(scripted)
            .execute(&request(Vec::new()))
            .expect_err("cleanup failure must require reconciliation");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Ambiguous);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::MayHaveStarted
        );
    }

    #[test]
    fn side_effect_request_is_rejected_before_sandbox_creation() {
        let scripted = client();
        let create_calls = Arc::clone(&scripted.create_calls);
        let error = provider(scripted)
            .execute(&request(vec![
                SideEffectId::new("side-effect/write").unwrap_or_else(|error| panic!("{error}"))
            ]))
            .expect_err("side effects must fail closed");

        assert_eq!(error.category(), HostedExecutionErrorCategory::Policy);
        assert_eq!(
            error.attempt_posture(),
            HostedExecutionAttemptPosture::NotStarted
        );
        assert_eq!(create_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn denied_egress_proof_is_required() {
        let error = OpenShellFixedOperationOutcome::new(
            0,
            HostedExecutionObservationSummary::new(
                0,
                0,
                1,
                1,
                0,
                0,
                reference(
                    HostedExecutionReferenceKind::Telemetry,
                    "openshell/observations/private-marker",
                ),
            )
            .unwrap_or_else(|error| panic!("{error}")),
            Vec::new(),
        )
        .expect_err("missing denied-egress proof must fail closed");

        assert_eq!(
            error.code(),
            "hosted.openshell.fixed_operation.denied_egress_proof.missing"
        );
        assert!(!error.to_string().contains("private-marker"));
    }

    #[test]
    fn debug_output_redacts_provider_and_sandbox_identity() {
        let provider_debug = format!("{:?}", provider(client()));
        let snapshot_debug = format!("{:?}", snapshot());

        assert!(!provider_debug.contains("openshell-provider-configuration"));
        assert!(!snapshot_debug.contains("sandbox/test"));
        assert!(!snapshot_debug.contains("revision/1"));
    }
}
