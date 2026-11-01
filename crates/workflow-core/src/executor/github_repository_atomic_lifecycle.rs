use std::fmt;

use super::github_repository_transient_access::{
    GitHubRepositoryEnvironmentReader, GitHubRepositoryMetadataAccessReference,
    GitHubRepositoryTransientAccessResolver,
};
use crate::authorized_execution_continuity_state::internal::TrustedTimeObservation;
use crate::current_authority_source::{
    derive_github_repository_authority, GitHubRepositoryAccessPresence,
    GitHubRepositoryAuthorityInput,
};
use crate::trusted_host_supervisor::{
    TrustedHostAttemptExecutionContext, TrustedHostAttemptExecutionResult,
    TrustedHostAttemptExecutor, TrustedHostSupervisorPersistenceInput,
};
use crate::{
    AdapterOperationMode, AuthorizedExecutionAttemptId, AuthorizedExecutionWindowId, CorrelationId,
    GitHubRepositoryMetadataReadConfiguration, GitHubRepositoryMetadataResource,
    OperationalExecutionWindowOpeningOperationId, OperationalExecutionWindowOpeningReceiptId,
    RedactionMetadata, RequiredContextContractBinding, RequiredContextExecutionBinding, SkillInput,
    SkillOutput, SpecContentHash, SqliteStateBackend, Timestamp,
    TrustedHostLocalApplicationPreparedSession, WorkflowOsError,
};

/// Provider result classifications accepted by the private atomic read path.
pub(super) enum GitHubRepositoryMetadataAttemptResult {
    Succeeded(SkillOutput),
    TerminalFailure,
    AmbiguousMayHaveStarted,
}

/// Same-call provider boundary for the exact repository-metadata operation.
///
/// The access value is borrowed only for the call. Implementations must not
/// retain it or include it in outputs, errors, telemetry, or debug text.
pub(super) trait GitHubRepositoryMetadataAttemptOperation {
    fn execute(
        &self,
        input: &SkillInput,
        access: Option<&str>,
    ) -> GitHubRepositoryMetadataAttemptResult;
}

enum GitHubRepositoryMetadataAccessMode<R> {
    Fixture,
    Live {
        reference: GitHubRepositoryMetadataAccessReference,
        resolver: GitHubRepositoryTransientAccessResolver<R>,
    },
}

struct GitHubRepositoryMetadataAttemptExecutor<R, O> {
    access: GitHubRepositoryMetadataAccessMode<R>,
    binding_commitment: SpecContentHash,
    operation: O,
    workflow_lifecycle:
        Option<crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext>,
}

impl<R, O> GitHubRepositoryMetadataAttemptExecutor<R, O>
where
    R: GitHubRepositoryEnvironmentReader,
    O: GitHubRepositoryMetadataAttemptOperation,
{
    fn execute_with_access(&self, input: &SkillInput) -> TrustedHostAttemptExecutionResult {
        let result = match &self.access {
            GitHubRepositoryMetadataAccessMode::Fixture => Ok(self.operation.execute(input, None)),
            GitHubRepositoryMetadataAccessMode::Live {
                reference,
                resolver,
            } => resolver.with_access(reference, |access| {
                self.operation.execute(input, Some(access))
            }),
        };
        match result {
            Ok(GitHubRepositoryMetadataAttemptResult::Succeeded(output)) => {
                TrustedHostAttemptExecutionResult::Succeeded(output)
            }
            Ok(GitHubRepositoryMetadataAttemptResult::TerminalFailure) | Err(_) => {
                TrustedHostAttemptExecutionResult::TerminalFailure
            }
            Ok(GitHubRepositoryMetadataAttemptResult::AmbiguousMayHaveStarted) => {
                TrustedHostAttemptExecutionResult::AmbiguousMayHaveStarted
            }
        }
    }
}

impl<R, O> TrustedHostAttemptExecutor for GitHubRepositoryMetadataAttemptExecutor<R, O>
where
    R: GitHubRepositoryEnvironmentReader,
    O: GitHubRepositoryMetadataAttemptOperation,
{
    fn binding_commitment(&self) -> SpecContentHash {
        self.binding_commitment.clone()
    }

    fn workflow_lifecycle_context(
        &self,
    ) -> Option<&crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext> {
        self.workflow_lifecycle.as_ref()
    }

    fn execute(
        &self,
        context: &TrustedHostAttemptExecutionContext<'_>,
    ) -> TrustedHostAttemptExecutionResult {
        self.execute_with_access(context.skill_input())
    }
}

impl<R, O> fmt::Debug for GitHubRepositoryMetadataAttemptExecutor<R, O> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubRepositoryMetadataAttemptExecutor")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

/// Private Phase 2 preparation input.
///
/// The caller supplies immutable execution bindings and lifecycle identities,
/// but cannot supply authority inventories, availability facts, source
/// generations, source watermarks, or credential values.
pub(super) struct GitHubRepositoryAtomicLifecycleInput<'a, R, O> {
    pub(super) backend: &'a SqliteStateBackend,
    pub(super) execution_binding: RequiredContextExecutionBinding,
    pub(super) contract: RequiredContextContractBinding,
    pub(super) correlation_id: CorrelationId,
    pub(super) workflow_lifecycle:
        Option<crate::trusted_host_workflow_lifecycle::TrustedHostWorkflowLifecycleContext>,
    pub(super) window_id: AuthorizedExecutionWindowId,
    pub(super) attempt_id: AuthorizedExecutionAttemptId,
    pub(super) opening_operation_id: OperationalExecutionWindowOpeningOperationId,
    pub(super) opening_receipt_id: OperationalExecutionWindowOpeningReceiptId,
    pub(super) expires_at: Timestamp,
    pub(super) maximum_attempts: u32,
    pub(super) trusted_time: TrustedTimeObservation,
    pub(super) evaluated_at: Timestamp,
    pub(super) redaction: RedactionMetadata,
    pub(super) persistence: TrustedHostSupervisorPersistenceInput,
    pub(super) operation_mode: AdapterOperationMode,
    pub(super) environment: R,
    pub(super) operation: O,
}

/// Prepares one private, atomic repository-metadata attempt.
///
/// Authority is derived inside Core from the immutable binding and exact
/// project declaration. Credential presence influences availability, while
/// the credential value stays inside the same-call executor closure. The
/// returned session delegates admission, dispatch, terminal persistence, and
/// ambiguity recovery to the existing authorized-execution continuity state.
pub(super) fn prepare_atomic_github_repository_metadata_read<'a, R, O>(
    input: GitHubRepositoryAtomicLifecycleInput<'a, R, O>,
) -> Result<TrustedHostLocalApplicationPreparedSession<'a>, WorkflowOsError>
where
    R: GitHubRepositoryEnvironmentReader + 'a,
    O: GitHubRepositoryMetadataAttemptOperation + 'a,
{
    let resolved = resolve_immutable_operation(
        input.backend,
        &input.execution_binding,
        &input.correlation_id,
    )?;
    let (access, access_presence) = match input.operation_mode {
        AdapterOperationMode::Fixture => (
            GitHubRepositoryMetadataAccessMode::Fixture,
            GitHubRepositoryAccessPresence::Missing,
        ),
        AdapterOperationMode::LiveReadOnly => {
            let reference = GitHubRepositoryMetadataAccessReference::parse(
                resolved.configuration.access_reference(),
            )?;
            let access_resolver = GitHubRepositoryTransientAccessResolver::new(input.environment);
            let presence = match access_resolver.presence(&reference) {
                super::github_repository_transient_access::GitHubRepositoryMetadataAccessPresence::Present => GitHubRepositoryAccessPresence::Present,
                super::github_repository_transient_access::GitHubRepositoryMetadataAccessPresence::Missing => GitHubRepositoryAccessPresence::Missing,
            };
            (
                GitHubRepositoryMetadataAccessMode::Live {
                    reference,
                    resolver: access_resolver,
                },
                presence,
            )
        }
        _ => {
            return Err(atomic_lifecycle_error(
                "operation_mode_unsupported",
                "GitHub repository metadata atomic lifecycle requires fixture or live read-only mode",
            ));
        }
    };
    let source = derive_github_repository_authority(&GitHubRepositoryAuthorityInput {
        configuration: &resolved.configuration,
        actor: input.execution_binding.actor(),
        workflow_id: input.execution_binding.workflow_id(),
        run_id: input.execution_binding.run_id(),
        step_id: input.execution_binding.step_id(),
        harness_contract_id: input.execution_binding.harness_contract_id(),
        target: &resolved.target,
        sensitivity: input.execution_binding.maximum_sensitivity(),
        evaluated_at: input.evaluated_at,
        expires_at: input.expires_at,
        operation_mode: input.operation_mode,
        access_presence,
    })?;
    let locator = crate::sqlite_state::TrustedHostOperationalEntryLocator {
        workflow_id: input.execution_binding.workflow_id().clone(),
        run_id: input.execution_binding.run_id().clone(),
        step_id: input.execution_binding.step_id().clone(),
        window_id: input.window_id,
        subject_actor_id: input.execution_binding.actor().clone(),
        immutable_run_bundle: input.execution_binding.immutable_run_bundle().clone(),
    };
    let executor = Box::new(GitHubRepositoryMetadataAttemptExecutor {
        access,
        binding_commitment: resolved.operation_binding,
        operation: input.operation,
        workflow_lifecycle: input.workflow_lifecycle,
    });
    let fresh = crate::sqlite_state::TrustedHostLocalApplicationOwnedFreshPreparation {
        source,
        execution_binding: input.execution_binding,
        contract: input.contract,
        operation_id: input.opening_operation_id,
        receipt_id: input.opening_receipt_id,
        attempt_id: input.attempt_id,
        expires_at: input.expires_at,
        maximum_attempts: input.maximum_attempts,
        trusted_time: input.trusted_time,
        evaluated_at: input.evaluated_at,
        redaction: input.redaction,
        persistence: input.persistence,
    };

    crate::sqlite_state::prepare_owned_trusted_host_local_application_session(
        crate::sqlite_state::TrustedHostLocalApplicationOwnedPreparationInput {
            backend: input.backend,
            locator,
            posture: crate::sqlite_state::TrustedHostLocalApplicationOwnedPreparationPosture::Fresh(
                Box::new(fresh),
            ),
            executor,
            skill_input: resolved.skill_input,
        },
    )
}

struct ResolvedGitHubRepositoryMetadataOperation {
    configuration: GitHubRepositoryMetadataReadConfiguration,
    target: GitHubRepositoryMetadataResource,
    skill_input: SkillInput,
    operation_binding: SpecContentHash,
}

#[allow(clippy::too_many_lines)]
fn resolve_immutable_operation(
    backend: &SqliteStateBackend,
    execution_binding: &RequiredContextExecutionBinding,
    correlation_id: &CorrelationId,
) -> Result<ResolvedGitHubRepositoryMetadataOperation, WorkflowOsError> {
    execution_binding.validate()?;
    let stored = crate::ImmutableRunBundleStore::read_exact_bundle(
        backend,
        execution_binding.run_id(),
        execution_binding.immutable_run_bundle().bundle_id(),
    )?;
    if stored.manifest().run_binding() != *execution_binding.immutable_run_bundle()
        || stored.manifest().run_id() != execution_binding.run_id()
        || stored.manifest().workflow_id() != execution_binding.workflow_id()
    {
        return Err(atomic_lifecycle_error(
            "immutable_bundle_mismatch",
            "GitHub repository read does not match the immutable run bundle",
        ));
    }

    let workflow = super::immutable_continuation_workflow(&stored)?;
    let [step] = workflow.steps.as_slice() else {
        return Err(atomic_lifecycle_error(
            "operation_contract_invalid",
            "GitHub repository read requires one immutable workflow step",
        ));
    };
    if &step.id != execution_binding.step_id() {
        return Err(atomic_lifecycle_error(
            "step_mismatch",
            "GitHub repository read does not match the immutable workflow step",
        ));
    }
    let skill = super::immutable_continuation_skill(&stored, step)?;
    let policy_effects = super::immutable_continuation_policy_effects(&stored, step)?;
    let mapped = step
        .input_mapping
        .iter()
        .map(|mapping| {
            let crate::MappingExpression::Literal { value } = &mapping.from else {
                return Err(atomic_lifecycle_error(
                    "input_mapping_invalid",
                    "GitHub repository read inputs must be immutable literals",
                ));
            };
            Ok((mapping.to.as_str(), value.as_str()))
        })
        .collect::<Result<std::collections::BTreeMap<_, _>, WorkflowOsError>>()?;
    let owner = *mapped.get("owner").ok_or_else(|| {
        atomic_lifecycle_error(
            "target_unresolved",
            "GitHub repository owner is unavailable from the immutable step",
        )
    })?;
    let repository = *mapped.get("repo").ok_or_else(|| {
        atomic_lifecycle_error(
            "target_unresolved",
            "GitHub repository name is unavailable from the immutable step",
        )
    })?;
    let operation_binding = super::github_repository_read_composition::derive_operation_binding(
        &stored,
        workflow,
        step,
        skill,
        &policy_effects,
        owner,
        repository,
    )?;

    let activation = stored
        .manifest()
        .execution_posture()
        .authoritative_execution()
        .ok_or_else(|| {
            atomic_lifecycle_error(
                "authoritative_execution_missing",
                "GitHub repository read requires immutable authoritative execution",
            )
        })?;
    let configuration = activation
        .provider_reads()
        .map(|reads| reads.github_repository_metadata.clone())
        .ok_or_else(|| {
            atomic_lifecycle_error(
                "provider_read_missing",
                "GitHub repository read declaration is absent from the immutable bundle",
            )
        })?;
    let target = configuration
        .resources()
        .iter()
        .find(|candidate| candidate.owner() == owner && candidate.repository() == repository)
        .cloned()
        .ok_or_else(|| {
            atomic_lifecycle_error(
                "target_not_allowed",
                "GitHub repository target is not allowed by the immutable project declaration",
            )
        })?;
    let values = [
        ("owner".to_owned(), owner.to_owned()),
        ("repo".to_owned(), repository.to_owned()),
    ]
    .into_iter()
    .collect();
    let skill_input = SkillInput {
        run_id: execution_binding.run_id().clone(),
        workflow_id: workflow.id.clone(),
        workflow_version: workflow.version.clone(),
        schema_version: workflow.schema_version.clone(),
        spec_hash: stored.manifest().workflow_content_hash().clone(),
        step_id: step.id.clone(),
        skill_id: skill.id.clone(),
        skill_version: skill.version.clone(),
        correlation_id: correlation_id.clone(),
        values,
    };
    Ok(ResolvedGitHubRepositoryMetadataOperation {
        configuration,
        target,
        skill_input,
        operation_binding,
    })
}

fn atomic_lifecycle_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::invalid_state(
        format!("executor.github_repository_metadata.atomic.{suffix}"),
        message,
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::cell::Cell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::authorized_execution_continuity_state::internal::{
        trusted_time_observation, ContinuityOperationId, ContinuityReceiptId,
        ContinuityTrustedTimeEpochId, TrustedTimeSourceKind,
    };
    use crate::executor::EventBuilder;
    use crate::{
        ActorId, ApprovalDecision, ApprovalDecisionKind, HarnessContractId, HarnessContractVersion,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
        LocalExecutionBeforeSkillInvocationCheckpointInputs,
        LocalExecutionImmutableRunBundleInputs, LocalExecutionRequest,
        LocalExecutionWithImmutableRunBundleRequest, LocalExecutor, LocalSkillRegistry,
        RequiredContextExecutionBindingInput, SkillId, SkillVersion, StepId, WorkReportSensitivity,
        WorkflowId, WorkflowRunEventKind, WorkflowRunId,
    };

    static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(1);

    struct FixedOperation<'a> {
        calls: &'a Cell<usize>,
        observed_length: &'a Cell<usize>,
        result: fn() -> GitHubRepositoryMetadataAttemptResult,
    }

    impl GitHubRepositoryMetadataAttemptOperation for FixedOperation<'_> {
        fn execute(
            &self,
            _input: &SkillInput,
            access: Option<&str>,
        ) -> GitHubRepositoryMetadataAttemptResult {
            self.calls.set(self.calls.get() + 1);
            self.observed_length.set(access.map_or(0, str::len));
            (self.result)()
        }
    }

    struct LifecycleFixture {
        root: PathBuf,
        backend: SqliteStateBackend,
        binding: RequiredContextExecutionBinding,
        contract: RequiredContextContractBinding,
        correlation_id: CorrelationId,
    }

    impl Drop for LifecycleFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    impl LifecycleFixture {
        #[allow(clippy::too_many_lines)]
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "workflow-os-github-atomic-{name}-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::SeqCst)
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(root.join("workflows")).expect("workflow directory");
            fs::create_dir_all(root.join("skills")).expect("skill directory");
            fs::create_dir_all(root.join("policies")).expect("policy directory");
            write_project(&root);

            let backend = SqliteStateBackend::open(root.join("state.sqlite3")).expect("backend");
            let run_id = WorkflowRunId::new(format!("run-{name}")).expect("run id");
            let actor = ActorId::new("agent/github-reader").expect("actor");
            let correlation_id =
                CorrelationId::new(format!("correlation/{name}")).expect("correlation");
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
                    created_at: timestamp("2026-10-31T10:00:00Z"),
                    sensitivity: ImmutableRunBundleSensitivity::Internal,
                    redaction_required: true,
                },
            };
            let project = super::super::load_validated_project_bundle(
                &root,
                crate::ProjectValidationCapability::Default,
            )
            .expect("validated project");
            let expected = project
                .manifest
                .definition
                .governance
                .as_ref()
                .and_then(|governance| governance.authoritative_execution)
                .expect("authoritative execution");
            let activation =
                super::super::authoritative_execution_activation(&project, Some(expected))
                    .expect("activation");
            let posture = super::super::immutable_run_bundle_execution_posture(
                &request.execution,
                activation,
            )
            .expect("execution posture");
            let bundle = crate::immutable_run_bundle_builder::build_immutable_run_bundle(
                crate::ImmutableRunBundleBuildRequest {
                    project: &project,
                    workflow_id: &request.execution.workflow_id,
                    bundle_id: request.bundle.bundle_id.clone(),
                    bundle_version: request.bundle.bundle_version.clone(),
                    run_id: run_id.clone(),
                    resolved_execution_context_hash: SpecContentHash::from_text(
                        "github atomic lifecycle fixture",
                    ),
                    execution_posture: posture,
                    handlers: vec![ImmutableRunBundleHandlerReference {
                        skill_id: SkillId::new("adapter/github-repository-metadata")
                            .expect("skill"),
                        skill_version: SkillVersion::new("v1").expect("skill version"),
                        posture: ImmutableRunBundleHandlerPosture::HostedProviderBound,
                    }],
                    created_at: request.bundle.created_at,
                    created_by: request.execution.actor.clone(),
                    sensitivity: request.bundle.sensitivity,
                    redaction_required: request.bundle.redaction_required,
                },
            )
            .expect("immutable bundle");
            crate::ImmutableRunBundleStore::publish_bundle_create_only(&backend, &bundle)
                .expect("bundle published");
            let registry = LocalSkillRegistry::new();
            let executor = LocalExecutor::new(&backend, &registry);
            let mut plan = LocalExecutor::<SqliteStateBackend>::prepare_execution(
                &request.execution,
                run_id.clone(),
            )
            .expect("execution plan");
            plan.immutable_run_bundle = Some(bundle.manifest().run_binding());
            executor
                .evaluate_pre_run_policy(&plan, &request.execution.actor, &correlation_id)
                .expect("pre-run policy");
            executor.append_run_start(&mut plan).expect("run start");
            executor
                .append_step_scheduled(&mut plan)
                .expect("step scheduled");
            plan.step_scheduled = true;
            let paused = executor.pause_for_approval(plan).expect("approval pause");
            let approval = paused.snapshot.approval_requests[0].clone();
            let mut builder = EventBuilder::from_snapshot(
                &paused.snapshot,
                correlation_id.clone(),
                request.execution.actor.clone(),
            );
            executor
                .append(
                    &mut builder,
                    WorkflowRunEventKind::ApprovalGranted(ApprovalDecision {
                        approval_id: approval.approval_id,
                        actor: request.execution.actor.clone(),
                        decided_at: timestamp("2026-10-31T10:05:00Z"),
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
                .expect("run resumed");
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
                    actor,
                    step_id: StepId::new("read").expect("step"),
                    maximum_sensitivity: WorkReportSensitivity::Internal,
                    bound_at: timestamp("2026-10-31T10:06:00Z"),
                })
                .expect("binding");
            Self {
                root,
                backend,
                binding,
                contract,
                correlation_id,
            }
        }

        fn prepare<'a>(
            &'a self,
            name: &str,
            calls: &'a Cell<usize>,
            observed_length: &'a Cell<usize>,
            result: fn() -> GitHubRepositoryMetadataAttemptResult,
        ) -> Result<TrustedHostLocalApplicationPreparedSession<'a>, WorkflowOsError> {
            let evaluated_at = Timestamp::now_utc();
            let expires_at = Timestamp::from_offset_date_time(
                evaluated_at.as_offset_date_time() + time::Duration::hours(1),
            );
            prepare_atomic_github_repository_metadata_read(GitHubRepositoryAtomicLifecycleInput {
                backend: &self.backend,
                execution_binding: self.binding.clone(),
                contract: self.contract.clone(),
                correlation_id: self.correlation_id.clone(),
                workflow_lifecycle: None,
                window_id: AuthorizedExecutionWindowId::new(format!("window/{name}"))?,
                attempt_id: AuthorizedExecutionAttemptId::new(format!("attempt/{name}"))?,
                opening_operation_id: OperationalExecutionWindowOpeningOperationId::new(format!(
                    "opening/{name}"
                ))?,
                opening_receipt_id: OperationalExecutionWindowOpeningReceiptId::new(format!(
                    "opening-receipt/{name}"
                ))?,
                expires_at,
                maximum_attempts: 1,
                trusted_time: trusted_time_observation(
                    evaluated_at,
                    TrustedTimeSourceKind::CoreInjectedClockV1,
                    SpecContentHash::new(crate::sqlite_state::CONTINUITY_CLOCK_PROVENANCE)?,
                    ContinuityTrustedTimeEpochId::new(crate::sqlite_state::CONTINUITY_CLOCK_EPOCH)?,
                ),
                evaluated_at,
                redaction: RedactionMetadata::empty(),
                persistence: TrustedHostSupervisorPersistenceInput {
                    operation: ContinuityOperationId::new(format!("supervisor/{name}"))?,
                    receipt: ContinuityReceiptId::new(format!("supervisor-receipt/{name}"))?,
                    yield_generation: None,
                },
                operation_mode: AdapterOperationMode::LiveReadOnly,
                environment: |key: &str| {
                    (key == "WORKFLOW_OS_GITHUB_TOKEN").then(|| "exact-secret-marker".to_owned())
                },
                operation: FixedOperation {
                    calls,
                    observed_length,
                    result,
                },
            })
        }
    }

    fn success() -> GitHubRepositoryMetadataAttemptResult {
        GitHubRepositoryMetadataAttemptResult::Succeeded(SkillOutput::new(
            BTreeMap::new(),
            Some("github-repository-metadata".to_owned()),
        ))
    }

    fn terminal_failure() -> GitHubRepositoryMetadataAttemptResult {
        GitHubRepositoryMetadataAttemptResult::TerminalFailure
    }

    fn ambiguous() -> GitHubRepositoryMetadataAttemptResult {
        GitHubRepositoryMetadataAttemptResult::AmbiguousMayHaveStarted
    }

    fn fixture_skill_input() -> SkillInput {
        SkillInput {
            run_id: WorkflowRunId::new("run/fixture").expect("run"),
            workflow_id: WorkflowId::new("workflow/fixture").expect("workflow"),
            workflow_version: crate::WorkflowVersion::new("v1").expect("workflow version"),
            schema_version: crate::SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            spec_hash: SpecContentHash::from_text("fixture"),
            step_id: StepId::new("read").expect("step"),
            skill_id: SkillId::new("adapter/github-repository-metadata").expect("skill"),
            skill_version: SkillVersion::new("v1").expect("skill version"),
            correlation_id: CorrelationId::new("correlation/fixture").expect("correlation"),
            values: BTreeMap::new(),
        }
    }

    fn executor<'a>(
        token: Option<&'static str>,
        calls: &'a Cell<usize>,
        observed_length: &'a Cell<usize>,
        result: fn() -> GitHubRepositoryMetadataAttemptResult,
    ) -> GitHubRepositoryMetadataAttemptExecutor<
        impl GitHubRepositoryEnvironmentReader,
        FixedOperation<'a>,
    > {
        let environment = move |name: &str| {
            (name == "WORKFLOW_OS_GITHUB_TOKEN")
                .then(|| token.map(str::to_owned))
                .flatten()
        };
        GitHubRepositoryMetadataAttemptExecutor {
            access: GitHubRepositoryMetadataAccessMode::Live {
                reference: GitHubRepositoryMetadataAccessReference::parse(
                    GitHubRepositoryMetadataReadConfiguration::ACCESS_REFERENCE,
                )
                .expect("access reference"),
                resolver: GitHubRepositoryTransientAccessResolver::new(environment),
            },
            binding_commitment: SpecContentHash::from_text(
                "github repository metadata fixture operation",
            ),
            operation: FixedOperation {
                calls,
                observed_length,
                result,
            },
            workflow_lifecycle: None,
        }
    }

    #[test]
    fn same_call_access_maps_success_without_retaining_the_value() {
        let calls = Cell::new(0);
        let observed_length = Cell::new(0);
        let executor = executor(
            Some("exact-secret-marker"),
            &calls,
            &observed_length,
            success,
        );

        assert!(matches!(
            executor.execute_with_access(&fixture_skill_input()),
            TrustedHostAttemptExecutionResult::Succeeded(_)
        ));
        assert_eq!(calls.get(), 1);
        assert_eq!(observed_length.get(), "exact-secret-marker".len());
        let rendered = format!("{executor:?}");
        assert!(!rendered.contains("exact-secret-marker"));
    }

    #[test]
    fn missing_access_blocks_provider_entry() {
        let calls = Cell::new(0);
        let observed_length = Cell::new(0);
        let executor = executor(None, &calls, &observed_length, success);

        assert!(matches!(
            executor.execute_with_access(&fixture_skill_input()),
            TrustedHostAttemptExecutionResult::TerminalFailure
        ));
        assert_eq!(calls.get(), 0);
        assert_eq!(observed_length.get(), 0);
    }

    #[test]
    fn provider_failure_and_ambiguity_keep_distinct_continuity_postures() {
        let failure_calls = Cell::new(0);
        let failure_length = Cell::new(0);
        let failure = executor(
            Some("secret"),
            &failure_calls,
            &failure_length,
            terminal_failure,
        );
        assert!(matches!(
            failure.execute_with_access(&fixture_skill_input()),
            TrustedHostAttemptExecutionResult::TerminalFailure
        ));

        let ambiguous_calls = Cell::new(0);
        let ambiguous_length = Cell::new(0);
        let ambiguous = executor(
            Some("secret"),
            &ambiguous_calls,
            &ambiguous_length,
            ambiguous,
        );
        assert!(matches!(
            ambiguous.execute_with_access(&fixture_skill_input()),
            TrustedHostAttemptExecutionResult::AmbiguousMayHaveStarted
        ));
        assert_eq!(failure_calls.get(), 1);
        assert_eq!(ambiguous_calls.get(), 1);
    }

    #[test]
    fn exact_wrapper_derives_immutable_inputs_and_persists_terminal_success_without_secret() {
        let fixture = LifecycleFixture::new("lifecycle-success");
        let calls = Cell::new(0);
        let observed_length = Cell::new(0);
        let prepared = fixture
            .prepare("lifecycle-success", &calls, &observed_length, success)
            .expect("exact wrapper prepares");
        let (session, _cancellation) = prepared.into_parts();
        let outcome = session.run().expect("session runs");

        assert_eq!(calls.get(), 1);
        assert_eq!(observed_length.get(), "exact-secret-marker".len());
        assert!(matches!(
            outcome,
            crate::TrustedHostLocalApplicationOutcome::EntryStopped(
                crate::TrustedHostLocalApplicationEntryStopReason::Terminal
            )
        ));
        assert!(!durable_files_contain(
            &fixture.root,
            b"exact-secret-marker"
        ));

        let replay_calls = Cell::new(0);
        let replay_length = Cell::new(0);
        let replay = fixture.prepare("lifecycle-success", &replay_calls, &replay_length, success);
        if let Ok(prepared) = replay {
            let (session, _cancellation) = prepared.into_parts();
            let _ = session.run();
        }
        assert_eq!(replay_calls.get(), 0);
    }

    #[test]
    fn exact_wrapper_persists_ambiguity_and_never_blindly_redispatches() {
        let fixture = LifecycleFixture::new("lifecycle-ambiguous");
        let calls = Cell::new(0);
        let observed_length = Cell::new(0);
        let prepared = fixture
            .prepare("lifecycle-ambiguous", &calls, &observed_length, ambiguous)
            .expect("exact wrapper prepares");
        let (session, _cancellation) = prepared.into_parts();
        let _ = session.run();
        assert_eq!(calls.get(), 1);

        let redispatch_calls = Cell::new(0);
        let redispatch_length = Cell::new(0);
        let redispatch = fixture.prepare(
            "lifecycle-ambiguous",
            &redispatch_calls,
            &redispatch_length,
            success,
        );
        if let Ok(prepared) = redispatch {
            let (session, _cancellation) = prepared.into_parts();
            let _ = session.run();
        }
        assert_eq!(redispatch_calls.get(), 0);
        assert!(!durable_files_contain(
            &fixture.root,
            b"exact-secret-marker"
        ));
    }

    #[allow(clippy::too_many_lines)]
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
      - from: { type: literal, value: acme }
        to: owner
      - from: { type: literal, value: widgets }
        to: repo
    policy_requirements: [{ id: external/read }]
    approval_policy:
      policy: { id: approval/github-read }
    timeout: { duration: 5m }
    terminal_behavior: fail_workflow
approval_requirements:
  - id: repository-read
    reason: Approval is required before the repository metadata read.
    expires_after: { duration: 30m }
cancellation_behavior: stop
audit_requirements:
  required: true
  events: [RunCreated, ApprovalRequested]
  store_references_only: true
observability_requirements:
  metrics: [workflow_latency]
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
allowed_capabilities: [{ name: external.read }, { name: adapter.invoke }]
input_contract:
  fields:
    - { name: owner, field_type: string }
    - { name: repo, field_type: string }
  required: [owner, repo]
output_contract:
  fields: [{ name: summary, field_type: string }]
failure_modes:
  - { code: github_read_failed, description: GitHub read failed., retryable: false }
evaluation_criteria:
  - { name: bounded_read, description: Reads immutable repository metadata. }
adapter_requirements:
  - adapter_id: symbolic/github-read-only
    operation: github.repository.metadata
    capabilities: [external.read]
approval_sensitivity: low
audit_requirements:
  required: true
  events: [SkillInvocationRequested]
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
