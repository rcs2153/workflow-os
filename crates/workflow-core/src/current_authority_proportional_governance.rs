use crate::current_authority_source::{
    RegisteredCurrentAuthorityConsumerResult, RegisteredCurrentAuthorityUseInput,
    RegisteredCurrentAuthorityUsePosture, RegisteredInMemoryCurrentAuthoritySource,
};
use crate::executor::{
    route_authoritative_explicit_local_check_profile_governance,
    LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest,
};
use crate::{
    GovernanceWorkloadAuthorityPosture, LocalExecutionAuthoritativeVisibleGovernanceDependencies,
    LocalExecutionWithAuthoritativeGovernanceRouteResult, LocalExecutor,
    LocalImmutableRunBundleStore, RedactionMetadata, RequiredContextContractBinding,
    RequiredContextExecutionBinding, ResolvedExplicitLocalCheckProfile, StateBackend, Timestamp,
    WorkflowOsError, WorkflowOsErrorKind,
};

#[allow(dead_code)]
pub(crate) struct CurrentAuthorityGovernanceRouteInput<'a> {
    pub(crate) execution_binding: &'a RequiredContextExecutionBinding,
    pub(crate) contract: &'a RequiredContextContractBinding,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) redaction: &'a RedactionMetadata,
}

#[allow(dead_code)]
pub(crate) fn route_authoritative_governance_with_current_authority<B>(
    source: &RegisteredInMemoryCurrentAuthoritySource,
    authority: &CurrentAuthorityGovernanceRouteInput<'_>,
    executor: &LocalExecutor<'_, B>,
    store: &LocalImmutableRunBundleStore,
    profile: &ResolvedExplicitLocalCheckProfile,
    visible_dependencies: Option<LocalExecutionAuthoritativeVisibleGovernanceDependencies<'_>>,
    request: &LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest,
) -> Result<LocalExecutionWithAuthoritativeGovernanceRouteResult, WorkflowOsError>
where
    B: StateBackend,
{
    validate_current_authority_route_input(authority, request)?;
    let fact = request
        .runtime_facts
        .first()
        .ok_or_else(|| current_authority_governance_error("runtime_fact.missing"))?;
    consume_current_authority_for_governance(source, authority, fact, |bound_fact| {
        let mut bound_request = request.clone();
        bound_request.runtime_facts = vec![bound_fact];
        route_authoritative_explicit_local_check_profile_governance(
            executor,
            store,
            profile,
            visible_dependencies,
            &bound_request,
        )
    })
}

pub(crate) fn consume_current_authority_for_governance<T, F>(
    source: &RegisteredInMemoryCurrentAuthoritySource,
    authority: &CurrentAuthorityGovernanceRouteInput<'_>,
    fact: &crate::StepGovernanceRuntimeFacts,
    consumer: F,
) -> Result<T, WorkflowOsError>
where
    F: FnOnce(crate::StepGovernanceRuntimeFacts) -> Result<T, WorkflowOsError>,
{
    if fact.step_id() != authority.execution_binding.step_id() {
        return Err(current_authority_governance_error(
            "runtime_fact.step_mismatch",
        ));
    }
    if fact.authority().is_some() {
        return Err(current_authority_governance_error(
            "runtime_fact.authority_preclassified",
        ));
    }

    let mut routed_result = None;
    let outcome = source.use_current_authority(
        &RegisteredCurrentAuthorityUseInput {
            execution_binding: authority.execution_binding,
            contract: authority.contract,
            evaluated_at: authority.evaluated_at,
            redaction: authority.redaction,
        },
        |_| {
            let result = consumer(
                fact.with_authoritative_authority(GovernanceWorkloadAuthorityPosture::Sufficient),
            );
            let consumer_result = if result.is_ok() {
                RegisteredCurrentAuthorityConsumerResult::Succeeded
            } else {
                RegisteredCurrentAuthorityConsumerResult::Failed
            };
            routed_result = Some(result);
            consumer_result
        },
    )?;

    match outcome.posture() {
        RegisteredCurrentAuthorityUsePosture::BlockedBeforeUse => {
            ensure_no_consumed_result(routed_result.as_ref())?;
            Err(current_authority_governance_error("authority.blocked"))
        }
        RegisteredCurrentAuthorityUsePosture::SourceFailure => {
            ensure_no_consumed_result(routed_result.as_ref())?;
            Err(current_authority_governance_error(
                "authority.source_failure",
            ))
        }
        RegisteredCurrentAuthorityUsePosture::ConsumerSucceeded => match routed_result {
            Some(Ok(result)) => Ok(result),
            _ => Err(current_authority_governance_error(
                "consumer.result_inconsistent",
            )),
        },
        RegisteredCurrentAuthorityUsePosture::ConsumerFailed => match routed_result {
            Some(Err(error)) => Err(error),
            _ => Err(current_authority_governance_error(
                "consumer.result_inconsistent",
            )),
        },
        RegisteredCurrentAuthorityUsePosture::ConsumerOutcomeAmbiguous => Err(
            current_authority_governance_error("consumer.outcome_ambiguous"),
        ),
    }
}

fn validate_current_authority_route_input(
    authority: &CurrentAuthorityGovernanceRouteInput<'_>,
    request: &LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest,
) -> Result<(), WorkflowOsError> {
    authority.execution_binding.validate()?;
    let execution = &request.execution.execution;
    let run_id = execution
        .run_id
        .as_ref()
        .ok_or_else(|| current_authority_governance_error("run_id.required"))?;
    if authority.execution_binding.workflow_id() != &execution.workflow_id
        || authority.execution_binding.run_id() != run_id
        || authority.execution_binding.step_id() != &request.selected_step_id
        || authority.execution_binding.actor() != &execution.actor
        || authority.execution_binding.harness_contract_id() != authority.contract.contract_id()
        || authority.execution_binding.harness_contract_version()
            != authority.contract.contract_version()
        || authority.execution_binding.contract_content_hash() != authority.contract.content_hash()
    {
        return Err(current_authority_governance_error(
            "execution_binding.mismatch",
        ));
    }
    if request.runtime_facts.len() != 1 {
        return Err(current_authority_governance_error(
            "runtime_fact.count_invalid",
        ));
    }
    let fact = &request.runtime_facts[0];
    if fact.step_id() != &request.selected_step_id {
        return Err(current_authority_governance_error(
            "runtime_fact.step_mismatch",
        ));
    }
    if fact.authority().is_some() {
        return Err(current_authority_governance_error(
            "runtime_fact.authority_preclassified",
        ));
    }
    Ok(())
}

fn ensure_no_consumed_result<T>(
    consumed: Option<&Result<T, WorkflowOsError>>,
) -> Result<(), WorkflowOsError> {
    if consumed.is_some() {
        return Err(current_authority_governance_error(
            "consumer.blocked_result_inconsistent",
        ));
    }
    Ok(())
}

fn current_authority_governance_error(suffix: &str) -> WorkflowOsError {
    WorkflowOsError::new(
        WorkflowOsErrorKind::Validation,
        format!("executor.authoritative_current_authority.{suffix}"),
        "authoritative current-authority governance composition failed",
    )
}
