#![allow(clippy::wildcard_imports)]

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use super::*;
use crate::current_authority_source::{
    CurrentAuthorityGitHubRepositoryReadInput, CurrentAuthorityGitHubRepositoryReadOutcome,
    RegisteredInMemoryCurrentAuthoritySource,
};
use crate::{
    AdapterOperationMode, AdapterPolicyPrecheck, GitHubReadOnlyAdapter, GitHubReadOnlyClient,
    RequiredContextContractBinding, RequiredContextExecutionBinding, SqliteStateBackend,
    StepScopedCapabilityProjection,
};

const OPERATION_CODE: &str = "github.repository.metadata";
const SYMBOLIC_ADAPTER_ID: &str = "symbolic/github-read-only";

pub(crate) struct ExecutorGitHubRepositoryMetadataReadInput<'a, C> {
    pub(crate) execution_binding: &'a RequiredContextExecutionBinding,
    pub(crate) contract: &'a RequiredContextContractBinding,
    pub(crate) source: &'a RegisteredInMemoryCurrentAuthoritySource,
    pub(crate) expected_last_sequence_number: EventSequenceNumber,
    pub(crate) expected_last_event_id: &'a EventId,
    pub(crate) expected_owner: &'a str,
    pub(crate) expected_repository: &'a str,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) correlation_id: &'a CorrelationId,
    pub(crate) redaction: &'a RedactionMetadata,
    pub(crate) adapter: &'a GitHubReadOnlyAdapter<C>,
}

pub(crate) enum ExecutorGitHubRepositoryMetadataReadOutcome {
    Succeeded {
        run: WorkflowRun,
        outcome: crate::GitHubReadOutcome,
        projection: Box<StepScopedCapabilityProjection>,
        receipt: Box<crate::CapabilityUseAuthorityReceipt>,
    },
    ReadSucceededReceiptFailed {
        run: WorkflowRun,
        outcome: crate::GitHubReadOutcome,
        projection: Box<StepScopedCapabilityProjection>,
        error: WorkflowOsError,
    },
    BlockedBeforeRead {
        run: WorkflowRun,
        error: WorkflowOsError,
    },
    ReadFailed {
        run: WorkflowRun,
        error: crate::AdapterError,
    },
}

impl fmt::Debug for ExecutorGitHubRepositoryMetadataReadOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded { run, .. } => formatter
                .debug_struct("Succeeded")
                .field("run_status", &run.snapshot.status)
                .field("provider_result", &"[REDACTED]")
                .finish(),
            Self::ReadSucceededReceiptFailed { run, error, .. } => formatter
                .debug_struct("ReadSucceededReceiptFailed")
                .field("run_status", &run.snapshot.status)
                .field("provider_result", &"[REDACTED]")
                .field("error_code", &error.code())
                .finish(),
            Self::BlockedBeforeRead { run, error } => formatter
                .debug_struct("BlockedBeforeRead")
                .field("run_status", &run.snapshot.status)
                .field("error_code", &error.code())
                .finish(),
            Self::ReadFailed { run, error } => formatter
                .debug_struct("ReadFailed")
                .field("run_status", &run.snapshot.status)
                .field("error_kind", &error.kind)
                .field("error_code", &error.code)
                .finish(),
        }
    }
}

impl<A, O, L> LocalExecutor<'_, SqliteStateBackend, A, O, L>
where
    A: AuditSink,
    O: ObservabilitySink,
    L: StructuredLogger,
{
    #[allow(clippy::too_many_lines)]
    pub(crate) fn execute_current_github_repository_metadata_read<C: GitHubReadOnlyClient>(
        &self,
        input: &ExecutorGitHubRepositoryMetadataReadInput<'_, C>,
    ) -> Result<ExecutorGitHubRepositoryMetadataReadOutcome, WorkflowOsError> {
        input.execution_binding.validate().map_err(|_| {
            composition_error("execution_binding_invalid", "executor binding is invalid")
        })?;
        validate_contract_binding(input)?;

        let run = self
            .backend
            .rehydrate_run(input.execution_binding.run_id())?;
        validate_initial_run_binding(&run, input)?;
        let stored = crate::ImmutableRunBundleStore::read_exact_bundle(
            self.backend,
            input.execution_binding.run_id(),
            input.execution_binding.immutable_run_bundle().bundle_id(),
        )?;
        validate_stored_bundle_binding(&stored, &run, input)?;

        let workflow = immutable_continuation_workflow(&stored)?;
        let step_id = current_scheduled_continuation_step(&run)?;
        if &step_id != input.execution_binding.step_id() {
            return Err(composition_error(
                "step_mismatch",
                "executor operation does not match the current immutable step",
            ));
        }
        let step = workflow
            .steps
            .iter()
            .find(|candidate| candidate.id == step_id)
            .ok_or_else(|| composition_error("step_unresolved", "immutable step is unavailable"))?;
        let skill = immutable_continuation_skill(&stored, step)?;
        let policy_effects = immutable_continuation_policy_effects(&stored, step)?;
        let operation_binding = derive_operation_binding(
            &stored,
            workflow,
            step,
            skill,
            &policy_effects,
            input.expected_owner,
            input.expected_repository,
        )?;

        let approval_already_granted = approval_granted_for_step(&run, &step_id);
        if policy_effects.requires_approval() && !approval_already_granted {
            return Err(composition_error(
                "approval_required",
                "executor operation requires a granted step approval",
            ));
        }

        let capabilities = capabilities_for_skill(skill);
        let policy_context = PolicyEvaluationContext {
            action: Action::InvokeAdapter,
            capabilities: capabilities.clone(),
            actor: Some(input.execution_binding.actor().clone()),
            workflow_id: Some(workflow.id.clone()),
            run_id: Some(run.snapshot.identity.run_id.clone()),
            step_id: Some(step.id.clone()),
            skill_id: Some(skill.id.clone()),
            autonomy_level: Some(workflow.autonomy_level),
            approval_sensitivity: Some(skill.approval_sensitivity),
            has_approval_policy: policy_effects.requires_approval(),
            policy_effects: policy_effects.clone(),
            adapter_id: Some(SYMBOLIC_ADAPTER_ID.to_owned()),
            correlation_id: Some(input.correlation_id.clone()),
        };
        let decision = self.policy_engine.evaluate(&policy_context);
        let mut policy_builder = EventBuilder::from_snapshot(
            &run.snapshot,
            input.correlation_id.clone(),
            input.execution_binding.actor().clone(),
        );
        self.evaluate_and_record_policy(&mut policy_builder, &policy_context)?;
        let policy_precheck = match input.adapter.operation_mode() {
            AdapterOperationMode::LiveReadOnly => {
                AdapterPolicyPrecheck::runtime_allowed(decision.reason_codes.clone())
            }
            AdapterOperationMode::Fixture => {
                AdapterPolicyPrecheck::fixture_test_allowed(decision.reason_codes.clone())
            }
            _ => {
                return Err(composition_error(
                    "adapter_mode_unsupported",
                    "executor operation requires a read-only adapter mode",
                ));
            }
        };

        let authority_input = CurrentAuthorityGitHubRepositoryReadInput {
            actor: input.execution_binding.actor(),
            workflow_id: &run.snapshot.identity.workflow_id,
            workflow_version: &run.snapshot.identity.workflow_version,
            schema_version: &run.snapshot.identity.schema_version,
            spec_hash: &run.snapshot.identity.spec_content_hash,
            run_id: &run.snapshot.identity.run_id,
            step_id: &step.id,
            harness_contract_id: input.execution_binding.harness_contract_id(),
            harness_contract_version: input.execution_binding.harness_contract_version(),
            owner: input.expected_owner,
            repository: input.expected_repository,
            requested_sensitivity: input.execution_binding.maximum_sensitivity(),
            evaluated_at: input.evaluated_at,
            correlation_id: input.correlation_id,
            redaction: input.redaction,
            policy_precheck: &policy_precheck,
            adapter: input.adapter,
        };
        let prepared = match input
            .source
            .prepare_github_repository_read_with_durable_continuity(self.backend, &authority_input)
        {
            Ok(prepared) => prepared,
            Err(error) => {
                let run = self
                    .backend
                    .rehydrate_run(input.execution_binding.run_id())?;
                return Ok(
                    ExecutorGitHubRepositoryMetadataReadOutcome::BlockedBeforeRead { run, error },
                );
            }
        };

        let current = self
            .backend
            .rehydrate_run(input.execution_binding.run_id())?;
        let invocation_key = invocation_idempotency_key(
            &current.snapshot.identity.run_id,
            &workflow.id,
            &workflow.version,
            &step.id,
            &skill.id,
            &skill.version,
        )?;
        let governance_commitment = operation_governance_commitment(
            &operation_binding,
            stored.manifest().resolved_execution_context_hash(),
            input.execution_binding,
            &policy_effects,
            &capabilities,
            approval_already_granted,
            skill.approval_sensitivity,
        );
        let brief = crate::governed_continuation::project_governed_continuation_brief(
            crate::governed_continuation::GovernedContinuationProjectionInput {
                run: &current,
                step_id: &step.id,
                invocation_idempotency_key: &invocation_key,
                governance_commitment,
            },
        )?;
        let claimed_sequence_number = current.snapshot.last_sequence_number;
        let claimed_event_id = current.snapshot.last_event_id.clone();

        crate::governed_continuation::consume_governed_continuation(self.backend, &brief, || {
            self.invoke_prepared_github_repository_metadata_read(
                input,
                &authority_input,
                prepared,
                step,
                skill,
                &invocation_key,
                claimed_sequence_number,
                &claimed_event_id,
            )
        })
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn invoke_prepared_github_repository_metadata_read<C: GitHubReadOnlyClient>(
        &self,
        input: &ExecutorGitHubRepositoryMetadataReadInput<'_, C>,
        authority_input: &CurrentAuthorityGitHubRepositoryReadInput<'_, C>,
        prepared: crate::current_authority_source::PreparedGitHubRepositoryRead,
        step: &StepDefinition,
        skill: &SkillDefinition,
        invocation_key: &IdempotencyKey,
        claimed_sequence_number: EventSequenceNumber,
        claimed_event_id: &EventId,
    ) -> Result<ExecutorGitHubRepositoryMetadataReadOutcome, WorkflowOsError> {
        let current = self
            .backend
            .rehydrate_run(input.execution_binding.run_id())?;
        validate_claimed_run_cursor(&current, claimed_sequence_number, claimed_event_id)?;
        let mut builder = EventBuilder::from_snapshot(
            &current.snapshot,
            input.correlation_id.clone(),
            input.execution_binding.actor().clone(),
        );
        let invocation_id = SkillInvocationId::generate();
        self.append(
            &mut builder,
            WorkflowRunEventKind::SkillInvocationRequested(SkillInvocation {
                invocation_id: invocation_id.clone(),
                step_id: step.id.clone(),
                skill_id: skill.id.clone(),
                skill_version: skill.version.clone(),
                idempotency_key: Some(invocation_key.clone()),
                attempts: Vec::new(),
            }),
            Some(invocation_key.clone()),
        )?;
        let attempt_key = attempt_idempotency_key(invocation_key, 1)?;
        self.append(
            &mut builder,
            WorkflowRunEventKind::SkillInvocationStarted(SkillInvocationAttempt {
                invocation_id: invocation_id.clone(),
                attempt_id: SkillAttemptId::generate(),
                step_id: step.id.clone(),
                skill_id: skill.id.clone(),
                skill_version: skill.version.clone(),
                attempt_number: 1,
            }),
            Some(attempt_key.clone()),
        )?;

        let outcome =
            RegisteredInMemoryCurrentAuthoritySource::execute_prepared_github_repository_read(
                authority_input,
                prepared,
                |proof| crate::CapabilityUseAuthorityReceipt::from_successful_use(&proof),
            );
        match outcome {
            CurrentAuthorityGitHubRepositoryReadOutcome::Succeeded {
                outcome,
                projection,
                receipt,
            } => {
                let telemetry = [AdapterTelemetryRecord::new(
                    outcome.invocation.clone(),
                    outcome.observability.clone(),
                )];
                self.emit_adapter_telemetry_for_context(
                    &step.id,
                    &skill.id,
                    &skill.version,
                    &telemetry,
                )?;
                self.append(
                    &mut builder,
                    WorkflowRunEventKind::SkillInvocationSucceeded {
                        invocation_id,
                        step_id: step.id.clone(),
                        skill_id: skill.id.clone(),
                        skill_version: skill.version.clone(),
                        output_ref: Some("github-repository-metadata-read".to_owned()),
                    },
                    Some(attempt_key),
                )?;
                self.append(&mut builder, WorkflowRunEventKind::RunCompleted, None)?;
                let run = self.rehydrate_and_project(input.execution_binding.run_id())?;
                Ok(ExecutorGitHubRepositoryMetadataReadOutcome::Succeeded {
                    run,
                    outcome,
                    projection,
                    receipt,
                })
            }
            CurrentAuthorityGitHubRepositoryReadOutcome::ReadSucceededReceiptFailed {
                outcome,
                projection,
                error,
            } => {
                let telemetry = [AdapterTelemetryRecord::new(
                    outcome.invocation.clone(),
                    outcome.observability.clone(),
                )];
                self.emit_adapter_telemetry_for_context(
                    &step.id,
                    &skill.id,
                    &skill.version,
                    &telemetry,
                )?;
                let run = self.fail_run(
                    builder,
                    "executor.github_repository_metadata.receipt_failed",
                    "GitHub repository metadata read completed but authority receipt construction failed",
                )?;
                Ok(
                    ExecutorGitHubRepositoryMetadataReadOutcome::ReadSucceededReceiptFailed {
                        run,
                        outcome,
                        projection,
                        error,
                    },
                )
            }
            CurrentAuthorityGitHubRepositoryReadOutcome::ReadFailed { error } => {
                let workflow_error = WorkflowOsError::from(error.clone());
                self.append(
                    &mut builder,
                    WorkflowRunEventKind::SkillInvocationFailed {
                        invocation_id,
                        step_id: step.id.clone(),
                        skill_id: skill.id.clone(),
                        skill_version: skill.version.clone(),
                        failure: FailureRecord {
                            code: workflow_error.code().to_owned(),
                            message: "GitHub repository metadata read failed".to_owned(),
                            failure_class: classify_failure(&workflow_error),
                        },
                    },
                    Some(attempt_key),
                )?;
                let run = self.fail_run(
                    builder,
                    workflow_error.code().to_owned(),
                    "GitHub repository metadata read failed",
                )?;
                Ok(ExecutorGitHubRepositoryMetadataReadOutcome::ReadFailed { run, error })
            }
            CurrentAuthorityGitHubRepositoryReadOutcome::BlockedBeforeRead { error } => {
                let run = self
                    .backend
                    .rehydrate_run(input.execution_binding.run_id())?;
                Ok(ExecutorGitHubRepositoryMetadataReadOutcome::BlockedBeforeRead { run, error })
            }
        }
    }
}

fn validate_contract_binding<C>(
    input: &ExecutorGitHubRepositoryMetadataReadInput<'_, C>,
) -> Result<(), WorkflowOsError> {
    if input.execution_binding.harness_contract_id() != input.contract.contract_id()
        || input.execution_binding.harness_contract_version() != input.contract.contract_version()
        || input.execution_binding.contract_content_hash() != input.contract.content_hash()
    {
        return Err(composition_error(
            "contract_mismatch",
            "executor binding does not match the required-context contract",
        ));
    }
    Ok(())
}

fn validate_claimed_run_cursor(
    run: &WorkflowRun,
    claimed_sequence_number: EventSequenceNumber,
    claimed_event_id: &EventId,
) -> Result<(), WorkflowOsError> {
    if run.snapshot.status != WorkflowRunStatus::Running
        || run.snapshot.last_sequence_number != claimed_sequence_number
        || &run.snapshot.last_event_id != claimed_event_id
    {
        return Err(composition_error(
            "cursor_changed_after_claim",
            "executor operation cursor changed after continuation claim",
        ));
    }
    Ok(())
}

fn validate_initial_run_binding<C>(
    run: &WorkflowRun,
    input: &ExecutorGitHubRepositoryMetadataReadInput<'_, C>,
) -> Result<(), WorkflowOsError> {
    let identity = &run.snapshot.identity;
    if run.snapshot.status != WorkflowRunStatus::Running
        || identity.run_id != *input.execution_binding.run_id()
        || identity.workflow_id != *input.execution_binding.workflow_id()
        || identity.immutable_run_bundle.as_ref()
            != Some(input.execution_binding.immutable_run_bundle())
        || run.snapshot.last_sequence_number != input.expected_last_sequence_number
        || &run.snapshot.last_event_id != input.expected_last_event_id
    {
        return Err(composition_error(
            "run_binding_mismatch",
            "executor operation does not match current durable run state",
        ));
    }
    let current_correlation = run
        .events
        .last()
        .and_then(|event| event.correlation_id.as_ref());
    if current_correlation != Some(input.correlation_id) {
        return Err(composition_error(
            "correlation_mismatch",
            "executor operation correlation does not match current durable state",
        ));
    }
    Ok(())
}

fn validate_stored_bundle_binding<C>(
    stored: &crate::StoredImmutableRunBundle,
    run: &WorkflowRun,
    input: &ExecutorGitHubRepositoryMetadataReadInput<'_, C>,
) -> Result<(), WorkflowOsError> {
    if stored.manifest().run_binding() != *input.execution_binding.immutable_run_bundle()
        || stored.manifest().run_id() != &run.snapshot.identity.run_id
        || stored.manifest().workflow_id() != &run.snapshot.identity.workflow_id
        || stored.manifest().workflow_version() != &run.snapshot.identity.workflow_version
        || stored.manifest().schema_version() != &run.snapshot.identity.schema_version
        || stored.manifest().workflow_content_hash() != &run.snapshot.identity.spec_content_hash
    {
        return Err(composition_error(
            "immutable_bundle_mismatch",
            "executor operation immutable bundle does not match the durable run",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub(super) fn derive_operation_binding(
    stored: &crate::StoredImmutableRunBundle,
    workflow: &WorkflowDefinition,
    step: &StepDefinition,
    skill: &SkillDefinition,
    policy_effects: &PolicyEffectSet,
    expected_owner: &str,
    expected_repository: &str,
) -> Result<SpecContentHash, WorkflowOsError> {
    if workflow.steps.len() != 1
        || matches!(step.terminal_behavior, Some(TerminalBehavior::Continue))
        || skill.adapter_requirements.len() != 1
        || skill.input_contract.fields.len() != 2
        || skill.input_contract.required.len() != 2
        || !skill.input_contract.examples.is_empty()
        || !policy_effects.allows_external_read()
    {
        return Err(composition_error(
            "operation_contract_invalid",
            "immutable step does not select the accepted repository metadata operation",
        ));
    }
    let adapter = &skill.adapter_requirements[0];
    if adapter.adapter_id.as_str() != SYMBOLIC_ADAPTER_ID
        || adapter
            .operation
            .as_ref()
            .map(crate::AdapterOperationId::as_str)
            != Some(OPERATION_CODE)
        || adapter.integration_id.is_some()
        || adapter.capabilities.as_slice() != ["external.read"]
    {
        return Err(composition_error(
            "adapter_contract_invalid",
            "immutable skill does not select the accepted read-only adapter contract",
        ));
    }
    let declared_capabilities = skill
        .allowed_capabilities
        .iter()
        .map(|capability| capability.name.as_str())
        .collect::<BTreeSet<_>>();
    if declared_capabilities != BTreeSet::from(["adapter.invoke", "external.read"])
        || skill.allowed_capabilities.len() != 2
    {
        return Err(composition_error(
            "capability_contract_invalid",
            "immutable skill capability contract is incompatible",
        ));
    }
    let fields = skill
        .input_contract
        .fields
        .iter()
        .map(|field| (field.name.as_str(), field))
        .collect::<std::collections::BTreeMap<_, _>>();
    if fields.len() != 2
        || !["owner", "repo"].into_iter().all(|name| {
            fields.get(name).is_some_and(|field| {
                field.field_type == crate::ContractFieldType::String && !field.sensitive
            })
        })
        || skill
            .input_contract
            .required
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            != BTreeSet::from(["owner", "repo"])
    {
        return Err(composition_error(
            "input_contract_invalid",
            "immutable skill input contract is incompatible",
        ));
    }
    let mut mapped = std::collections::BTreeMap::new();
    for mapping in &step.input_mapping {
        let MappingExpression::Literal { value } = &mapping.from else {
            return Err(composition_error(
                "input_mapping_invalid",
                "repository metadata inputs must be immutable literals",
            ));
        };
        if mapped.insert(mapping.to.as_str(), value.as_str()).is_some() {
            return Err(composition_error(
                "input_mapping_invalid",
                "repository metadata input mapping is ambiguous",
            ));
        }
    }
    if mapped.len() != 2
        || mapped.get("owner") != Some(&expected_owner)
        || mapped.get("repo") != Some(&expected_repository)
    {
        return Err(composition_error(
            "target_mismatch",
            "requested repository does not match the immutable step target",
        ));
    }

    let mut hasher = Sha256::new();
    update_idempotency_hash(
        &mut hasher,
        "domain",
        "workflow-os/executor-github-repository-metadata-operation/v1",
    );
    update_idempotency_hash(&mut hasher, "operation", OPERATION_CODE);
    update_idempotency_hash(
        &mut hasher,
        "bundle-root",
        stored.manifest().root_hash().as_str(),
    );
    update_idempotency_hash(&mut hasher, "step", step.id.as_str());
    update_idempotency_hash(&mut hasher, "skill", skill.id.as_str());
    update_idempotency_hash(&mut hasher, "skill-version", skill.version.as_str());
    update_idempotency_hash(&mut hasher, "owner", expected_owner);
    update_idempotency_hash(&mut hasher, "repository", expected_repository);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn approval_granted_for_step(run: &WorkflowRun, step_id: &StepId) -> bool {
    run.snapshot
        .approval_requests
        .iter()
        .rev()
        .find(|approval| approval.step_id.as_ref() == Some(step_id))
        .and_then(|approval| approval.decision.as_ref())
        .is_some_and(|decision| decision.decision == ApprovalDecisionKind::Granted)
}

fn operation_governance_commitment(
    operation_binding: &SpecContentHash,
    resolved_execution_context_hash: &SpecContentHash,
    execution_binding: &RequiredContextExecutionBinding,
    policy_effects: &PolicyEffectSet,
    capabilities: &[Capability],
    approval_granted: bool,
    approval_sensitivity: crate::ApprovalSensitivity,
) -> SpecContentHash {
    let base = continuation_governance_commitment_from_material(
        resolved_execution_context_hash,
        Some(execution_binding.immutable_run_bundle()),
        None,
        approval_granted,
        approval_sensitivity,
        policy_effects,
        capabilities,
        false,
    );
    let mut hasher = Sha256::new();
    update_idempotency_hash(
        &mut hasher,
        "domain",
        "workflow-os/executor-github-repository-metadata-continuation/v1",
    );
    update_idempotency_hash(&mut hasher, "executor-governance", base.as_str());
    update_idempotency_hash(&mut hasher, "operation-binding", operation_binding.as_str());
    update_idempotency_hash(
        &mut hasher,
        "required-context-binding",
        execution_binding.binding_hash().as_str(),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn composition_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    executor_error(
        WorkflowOsErrorKind::InvalidState,
        format!("executor.github_repository_metadata.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic, clippy::too_many_lines)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::current_authority_source::RegisteredInMemoryCurrentAuthoritySourceInput;
    use crate::{
        execute_with_immutable_run_bundle, AdapterError, AdapterErrorKind, CapabilityAvailability,
        CapabilityAvailabilityRecord, CapabilityDelegationPosture, CapabilityGrant,
        CapabilityGrantDefinition, CapabilityGrantId, CapabilityGrantLifecycle,
        CapabilityGrantRequirements, CapabilityGrantScope, CapabilityReference,
        CapabilityResourceKind, CapabilityResourceScope, CurrentAuthoritySourceContractVersion,
        CurrentAuthoritySourceGeneration, CurrentAuthoritySourceId, GitHubHttpResponse,
        GitHubReadOnlyConfig, HarnessContractId, HarnessContractVersion, ImmutableRunBundleId,
        ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
        LocalExecutionBeforeSkillInvocationCheckpointInputs,
        LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
        LocalExecutionWithImmutableRunBundleRequest, RequiredContextExecutionBindingInput,
    };

    static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(1);

    #[derive(Clone)]
    struct CountingClient {
        calls: Arc<AtomicUsize>,
        paths: Arc<Mutex<Vec<String>>>,
        fail: bool,
    }

    impl GitHubReadOnlyClient for CountingClient {
        fn get(&self, path: &str, _accept: &str) -> Result<GitHubHttpResponse, AdapterError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.paths.lock().expect("paths lock").push(path.to_owned());
            if self.fail {
                return Err(AdapterError::new(
                    AdapterErrorKind::TransientNetworkFailure,
                    "github.fixture.unavailable",
                    "provider-secret-marker",
                ));
            }
            Ok(GitHubHttpResponse {
                status: 200,
                body: r#"{"full_name":"acme/widgets","default_branch":"main","private":true,"html_url":"https://github.com/acme/widgets","secret":"provider-secret-marker"}"#.to_owned(),
                headers: BTreeMap::new(),
            })
        }
    }

    struct Fixture {
        root: PathBuf,
        backend: SqliteStateBackend,
        registry: LocalSkillRegistry,
        run_id: WorkflowRunId,
        contract: RequiredContextContractBinding,
        binding: RequiredContextExecutionBinding,
        source: RegisteredInMemoryCurrentAuthoritySource,
        actor: ActorId,
        correlation_id: CorrelationId,
        redaction: RedactionMetadata,
        expected_last_sequence_number: EventSequenceNumber,
        expected_last_event_id: EventId,
        calls: Arc<AtomicUsize>,
        paths: Arc<Mutex<Vec<String>>>,
        audit: LocalAuditSink,
        observability: LocalObservabilitySink,
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "workflow-os-executor-github-read-{name}-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::SeqCst)
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(root.join("workflows")).expect("workflow directory");
            fs::create_dir_all(root.join("skills")).expect("skill directory");
            fs::create_dir_all(root.join("policies")).expect("policy directory");
            write_project(&root);

            let backend = SqliteStateBackend::open(root.join("state.sqlite3")).expect("backend");
            let registry = LocalSkillRegistry::new();
            let run_id = WorkflowRunId::new(format!("run-{name}")).expect("run id");
            let actor = ActorId::new("agent/github-reader").expect("actor");
            let correlation_id =
                CorrelationId::new(format!("correlation/{name}")).expect("correlation id");
            let request = LocalExecutionWithImmutableRunBundleRequest {
                execution: LocalExecutionRequest {
                    project_root: root.clone(),
                    workflow_id: WorkflowId::new("workflow/read").expect("workflow"),
                    run_id: Some(run_id.clone()),
                    correlation_id: correlation_id.clone(),
                    actor: actor.clone(),
                    before_skill_invocation_checkpoints:
                        LocalExecutionBeforeSkillInvocationCheckpointInputs::default(),
                    before_skill_invocation_hook: None,
                    side_effect_events: Vec::new(),
                    side_effect_lifecycle_events: Vec::new(),
                },
                bundle: LocalExecutionImmutableRunBundleInputs {
                    bundle_id: ImmutableRunBundleId::new(format!("bundle/{name}"))
                        .expect("bundle id"),
                    bundle_version: ImmutableRunBundleVersion::new("v1").expect("bundle version"),
                    created_at: timestamp("2026-10-22T10:00:00Z"),
                    sensitivity: ImmutableRunBundleSensitivity::Internal,
                    redaction_required: true,
                },
            };
            let paused = execute_with_immutable_run_bundle(
                &LocalExecutor::new(&backend, &registry),
                &backend,
                &request,
            )
            .expect("run pauses");
            assert_eq!(
                paused.run().snapshot.status,
                WorkflowRunStatus::WaitingForApproval
            );

            let approval = paused.run().snapshot.approval_requests[0].clone();
            let executor = LocalExecutor::new(&backend, &registry);
            let mut builder = EventBuilder::from_snapshot(
                &paused.run().snapshot,
                correlation_id.clone(),
                actor.clone(),
            );
            executor
                .append(
                    &mut builder,
                    WorkflowRunEventKind::ApprovalGranted(ApprovalDecision {
                        approval_id: approval.approval_id,
                        actor: actor.clone(),
                        decided_at: timestamp("2026-10-22T10:05:00Z"),
                        decision: ApprovalDecisionKind::Granted,
                        reason: "bounded repository metadata read".to_owned(),
                        correlation_id: correlation_id.clone(),
                        proof_marker: None,
                    }),
                    None,
                )
                .expect("approval appended");
            executor
                .append(&mut builder, WorkflowRunEventKind::RunResumed, None)
                .expect("resume appended");
            let running = backend.rehydrate_run(&run_id).expect("running run");
            assert_eq!(running.snapshot.status, WorkflowRunStatus::Running);
            let expected_last_sequence_number = running.snapshot.last_sequence_number;
            let expected_last_event_id = running.snapshot.last_event_id.clone();

            let stored = crate::ImmutableRunBundleStore::read_exact_bundle(
                &backend,
                &run_id,
                &request.bundle.bundle_id,
            )
            .expect("stored bundle");
            let contract = RequiredContextContractBinding::explicit_none_for_test(
                HarnessContractId::new("harness/github-read").expect("contract"),
                HarnessContractVersion::new("v1").expect("contract version"),
            )
            .expect("contract");
            let binding =
                RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
                    bundle: &stored,
                    contract: &contract,
                    actor: actor.clone(),
                    step_id: StepId::new("read").expect("step"),
                    maximum_sensitivity: WorkReportSensitivity::Internal,
                    bound_at: timestamp("2026-10-22T10:06:00Z"),
                })
                .expect("binding");
            let capability = CapabilityReference::new("github.read").expect("capability");
            let resource = CapabilityResourceScope::new(
                CapabilityResourceKind::AdapterResource,
                "github/acme/widgets",
            )
            .expect("resource");
            let observed_at = timestamp("2026-10-22T10:07:00Z");
            let grant = CapabilityGrant::new(CapabilityGrantDefinition {
                grant_id: CapabilityGrantId::new(format!("grant/{name}")).expect("grant id"),
                subject: actor.clone(),
                capability: capability.clone(),
                resource: resource.clone(),
                scope: CapabilityGrantScope::new(
                    WorkflowId::new("workflow/read").expect("workflow"),
                    Some(run_id.clone()),
                    Some(StepId::new("read").expect("step")),
                    Some(contract.contract_id().clone()),
                )
                .expect("scope"),
                issuer: ActorId::new("system/authority").expect("issuer"),
                issued_at: timestamp("2026-10-22T10:00:00Z"),
                expires_at: Some(timestamp("2026-10-22T11:00:00Z")),
                lifecycle: CapabilityGrantLifecycle::Active,
                revocation_reference: None,
                delegation: CapabilityDelegationPosture::Disabled,
                requirements: CapabilityGrantRequirements::default(),
                sensitivity_ceiling: WorkReportSensitivity::Internal,
                redaction: RedactionMetadata::empty(),
            })
            .expect("grant");
            let availability = CapabilityAvailabilityRecord::new(
                capability,
                resource,
                CapabilityAvailability::Available,
                observed_at,
                RedactionMetadata::empty(),
            )
            .expect("availability");
            let source = RegisteredInMemoryCurrentAuthoritySource::register(
                RegisteredInMemoryCurrentAuthoritySourceInput {
                    source_id: CurrentAuthoritySourceId::new(format!("source/{name}"))
                        .expect("source"),
                    contract_version: CurrentAuthoritySourceContractVersion::new("v1")
                        .expect("source version"),
                    configuration_commitment: SpecContentHash::from_text("github read authority"),
                    core_maximum_observation_age_seconds: 3_600,
                    sensitivity: WorkReportSensitivity::Internal,
                    observed_at,
                    source_valid_through: Some(timestamp("2026-10-22T10:30:00Z")),
                    generation: Some(CurrentAuthoritySourceGeneration::new(1).expect("generation")),
                    complete_grant_inventory: vec![grant],
                    complete_availability_inventory: vec![availability],
                    complete_context_reference_inventory: Vec::new(),
                },
            )
            .expect("source");
            Self {
                root,
                backend,
                registry,
                run_id,
                contract,
                binding,
                source,
                actor,
                correlation_id,
                redaction: RedactionMetadata::empty(),
                expected_last_sequence_number,
                expected_last_event_id,
                calls: Arc::new(AtomicUsize::new(0)),
                paths: Arc::new(Mutex::new(Vec::new())),
                audit: LocalAuditSink::new(),
                observability: LocalObservabilitySink::new(),
            }
        }

        fn adapter(&self, fail: bool) -> GitHubReadOnlyAdapter<CountingClient> {
            GitHubReadOnlyAdapter::new(
                GitHubReadOnlyConfig::fixture().expect("fixture config"),
                CountingClient {
                    calls: Arc::clone(&self.calls),
                    paths: Arc::clone(&self.paths),
                    fail,
                },
            )
        }

        fn input<'a, C>(
            &'a self,
            adapter: &'a GitHubReadOnlyAdapter<C>,
        ) -> ExecutorGitHubRepositoryMetadataReadInput<'a, C> {
            ExecutorGitHubRepositoryMetadataReadInput {
                execution_binding: &self.binding,
                contract: &self.contract,
                source: &self.source,
                expected_last_sequence_number: self.expected_last_sequence_number,
                expected_last_event_id: &self.expected_last_event_id,
                expected_owner: "acme",
                expected_repository: "widgets",
                evaluated_at: timestamp("2026-10-22T10:10:00Z"),
                correlation_id: &self.correlation_id,
                redaction: &self.redaction,
                adapter,
            }
        }

        fn executor(
            &self,
        ) -> LocalExecutor<
            '_,
            SqliteStateBackend,
            LocalAuditSink,
            LocalObservabilitySink,
            LocalStructuredLogger,
        > {
            LocalExecutor::new_with_sinks(
                &self.backend,
                &self.registry,
                ConservativePolicyEngine::new(),
                self.audit.clone(),
                self.observability.clone(),
                LocalStructuredLogger::new(),
            )
        }
    }

    #[test]
    fn exact_composition_reads_once_and_records_ordered_terminal_history() {
        let fixture = Fixture::new("success");
        let adapter = fixture.adapter(false);
        let input = fixture.input(&adapter);
        let outcome = fixture
            .executor()
            .execute_current_github_repository_metadata_read(&input)
            .expect("composition executes");
        let debug = format!("{outcome:?}");

        let ExecutorGitHubRepositoryMetadataReadOutcome::Succeeded {
            run,
            projection,
            receipt,
            ..
        } = outcome
        else {
            panic!("expected successful composition");
        };
        assert_eq!(fixture.calls.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.audit.adapter_records().len(), 1);
        assert_eq!(fixture.observability.adapter_events().len(), 1);
        assert_eq!(
            fixture.paths.lock().expect("paths").as_slice(),
            ["/repos/acme/widgets"]
        );
        assert_eq!(run.snapshot.status, WorkflowRunStatus::Completed);
        assert_eq!(projection.step_id(), &StepId::new("read").expect("step"));
        receipt.validate().expect("receipt validates");
        let names = run
            .events
            .iter()
            .map(WorkflowRunEvent::kind)
            .collect::<Vec<_>>();
        let requested = names
            .iter()
            .position(|name| *name == crate::WorkflowRunEventKindName::SkillInvocationRequested)
            .expect("requested");
        let started = names
            .iter()
            .position(|name| *name == crate::WorkflowRunEventKindName::SkillInvocationStarted)
            .expect("started");
        let succeeded = names
            .iter()
            .position(|name| *name == crate::WorkflowRunEventKindName::SkillInvocationSucceeded)
            .expect("succeeded");
        let completed = names
            .iter()
            .position(|name| *name == crate::WorkflowRunEventKindName::RunCompleted)
            .expect("completed");
        assert!(requested < started && started < succeeded && succeeded < completed);

        let error = fixture
            .executor()
            .execute_current_github_repository_metadata_read(&input)
            .expect_err("terminal replay is rejected");
        assert_eq!(
            error.code(),
            "executor.github_repository_metadata.run_binding_mismatch"
        );
        assert_eq!(fixture.calls.load(Ordering::SeqCst), 1);
        assert!(!debug.contains("provider-secret-marker"));
        assert!(!debug.contains("acme/widgets"));
        assert!(!durable_files_contain(
            &fixture.root,
            b"provider-secret-marker"
        ));
    }

    #[test]
    fn target_mismatch_fails_before_provider_and_preserves_running_state() {
        let fixture = Fixture::new("target-mismatch");
        let adapter = fixture.adapter(false);
        let mut input = fixture.input(&adapter);
        input.expected_repository = "other";

        let error = fixture
            .executor()
            .execute_current_github_repository_metadata_read(&input)
            .expect_err("target mismatch fails closed");

        assert_eq!(
            error.code(),
            "executor.github_repository_metadata.target_mismatch"
        );
        assert_eq!(fixture.calls.load(Ordering::SeqCst), 0);
        assert_eq!(
            fixture
                .backend
                .rehydrate_run(&fixture.run_id)
                .expect("run")
                .snapshot
                .status,
            WorkflowRunStatus::Running
        );
        assert!(!error.to_string().contains("other"));
    }

    #[test]
    fn provider_failure_is_terminal_and_does_not_leak_provider_text() {
        let fixture = Fixture::new("provider-failure");
        let adapter = fixture.adapter(true);
        let input = fixture.input(&adapter);
        let outcome = fixture
            .executor()
            .execute_current_github_repository_metadata_read(&input)
            .expect("provider failure is projected");

        let ExecutorGitHubRepositoryMetadataReadOutcome::ReadFailed { run, error } = &outcome
        else {
            panic!("expected provider failure");
        };
        assert_eq!(fixture.calls.load(Ordering::SeqCst), 1);
        assert_eq!(run.snapshot.status, WorkflowRunStatus::Failed);
        assert_eq!(error.code, "github.fixture.unavailable");
        let debug = format!("{outcome:?}");
        assert!(!debug.contains("provider-secret-marker"));
        assert!(!debug.contains("acme/widgets"));
        assert!(run.events.iter().any(|event| matches!(
            event.kind,
            WorkflowRunEventKind::SkillInvocationFailed { .. }
        )));
        assert!(run
            .events
            .iter()
            .any(|event| matches!(event.kind, WorkflowRunEventKind::RunFailed(_))));
    }

    #[test]
    fn concurrent_exact_consumers_call_provider_at_most_once() {
        let fixture = Fixture::new("concurrent");
        let adapter = fixture.adapter(false);
        let input = fixture.input(&adapter);
        let backend = &fixture.backend;
        let successes = std::thread::scope(|scope| {
            let first = scope.spawn(|| {
                let registry = LocalSkillRegistry::new();
                matches!(
                    LocalExecutor::new(backend, &registry)
                        .execute_current_github_repository_metadata_read(&input),
                    Ok(ExecutorGitHubRepositoryMetadataReadOutcome::Succeeded { .. })
                )
            });
            let second = scope.spawn(|| {
                let registry = LocalSkillRegistry::new();
                matches!(
                    LocalExecutor::new(backend, &registry)
                        .execute_current_github_repository_metadata_read(&input),
                    Ok(ExecutorGitHubRepositoryMetadataReadOutcome::Succeeded { .. })
                )
            });
            usize::from(first.join().expect("first consumer"))
                + usize::from(second.join().expect("second consumer"))
        });

        assert_eq!(successes, 1);
        assert_eq!(fixture.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn claimed_cursor_change_is_rejected_before_provider_entry() {
        let fixture = Fixture::new("claimed-cursor-change");
        let run = fixture.backend.rehydrate_run(&fixture.run_id).expect("run");
        let stale_sequence = EventSequenceNumber::new(
            run.snapshot
                .last_sequence_number
                .get()
                .checked_sub(1)
                .expect("prior sequence"),
        )
        .expect("sequence");

        let error = validate_claimed_run_cursor(&run, stale_sequence, &run.snapshot.last_event_id)
            .expect_err("stale claimed cursor is rejected");

        assert_eq!(
            error.code(),
            "executor.github_repository_metadata.cursor_changed_after_claim"
        );
        assert_eq!(fixture.calls.load(Ordering::SeqCst), 0);
    }

    fn write_project(root: &Path) {
        fs::write(
            root.join("workflow-os.yml"),
            r"schema_version: workflowos.dev/v0
project:
  id: executor/github-read
  name: Executor GitHub Read
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
  provider_reads:
    github_repository_metadata:
      enabled: true
      endpoint: github.com
      access_reference: env/WORKFLOW_OS_GITHUB_TOKEN
      resources:
        - owner: acme
          repository: widgets
",
        )
        .expect("manifest");
        fs::write(
            root.join("workflows/read.workflow.yml"),
            r"schema_version: workflowos.dev/v0
id: workflow/read
version: v1
display_name: Repository Metadata Read
owner:
  lifecycle_status: stable
autonomy_level: level_2
triggers:
  - id: manual
    kind: manual
steps:
  - id: read
    skill_ref:
      id: adapter/github-repository-metadata
      version: v1
    input_mapping:
      - from:
          type: literal
          value: acme
        to: owner
      - from:
          type: literal
          value: widgets
        to: repo
    policy_requirements:
      - id: external/read
    approval_policy:
      policy:
        id: approval/github-read
    timeout:
      duration: 5m
    terminal_behavior: fail_workflow
approval_requirements:
  - id: repository-read
    reason: Approval is required before the repository metadata read.
    expires_after:
      duration: 30m
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated, ApprovalRequested, ApprovalGranted, SkillInvocationRequested, SkillInvocationSucceeded]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency, skill_latency]
  tracing: true
  latency_tracking: true
",
        )
        .expect("workflow");
        fs::write(
            root.join("skills/read.skill.yml"),
            r"schema_version: workflowos.dev/v0
id: adapter/github-repository-metadata
version: v1
display_name: GitHub Repository Metadata Read
allowed_capabilities:
  - name: external.read
  - name: adapter.invoke
input_contract:
  fields:
    - name: owner
      field_type: string
    - name: repo
      field_type: string
  required: [owner, repo]
output_contract:
  fields:
    - name: summary
      field_type: string
failure_modes:
  - code: github_read_failed
    description: GitHub repository metadata read failed.
    retryable: false
evaluation_criteria:
  - name: bounded_read
    description: The operation reads only repository metadata for the immutable target.
adapter_requirements:
  - adapter_id: symbolic/github-read-only
    operation: github.repository.metadata
    capabilities: [external.read]
approval_sensitivity: low
audit_requirements:
  required: true
  events: [SkillInvocationRequested, SkillInvocationSucceeded]
  store_references_only: true
observability_requirements:
  metrics: [skill_latency]
  tracing: true
  latency_tracking: true
",
        )
        .expect("skill");
        fs::write(
            root.join("policies/read.policy.yml"),
            "schema_version: workflowos.dev/v0\nid: external/read\nname: External Read\nrules:\n  - id: allow\n    effect: allow_external_read\n",
        )
        .expect("read policy");
        fs::write(
            root.join("policies/approval.policy.yml"),
            "schema_version: workflowos.dev/v0\nid: approval/github-read\nname: Repository Read Approval\nrules:\n  - id: approve\n    effect: require_approval\n",
        )
        .expect("approval policy");
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    fn durable_files_contain(root: &Path, needle: &[u8]) -> bool {
        fs::read_dir(root)
            .expect("fixture directory")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| fs::read(entry.path()).ok())
            .any(|bytes| bytes.windows(needle.len()).any(|window| window == needle))
    }
}
