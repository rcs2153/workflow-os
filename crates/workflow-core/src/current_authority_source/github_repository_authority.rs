use serde::Serialize;

use super::{
    hash_serializable, registered_in_memory_source::RegisteredInMemoryCurrentAuthoritySourceInput,
    CurrentAuthoritySourceContractVersion, CurrentAuthoritySourceId,
};
use crate::{
    ActorId, AdapterOperationMode, CapabilityAvailability, CapabilityAvailabilityRecord,
    CapabilityDelegationPosture, CapabilityGrant, CapabilityGrantDefinition, CapabilityGrantId,
    CapabilityGrantLifecycle, CapabilityGrantRequirements, CapabilityGrantScope,
    CapabilityReference, CapabilityResourceKind, CapabilityResourceScope,
    GitHubRepositoryMetadataReadConfiguration, GitHubRepositoryMetadataResource, HarnessContractId,
    RedactionMetadata, SpecContentHash, StepId, Timestamp, WorkReportSensitivity, WorkflowId,
    WorkflowOsError, WorkflowRunId,
};

use super::registered_in_memory_source::RegisteredInMemoryCurrentAuthoritySource;

const AUTHORED_OPERATION: &str = "github.repository.metadata";
const INTERNAL_CAPABILITY: &str = "github.read";
const SOURCE_ID: &str = "source/core/github-repository-metadata";
const SOURCE_CONTRACT_VERSION: &str = "v1";
const CORE_ISSUER: &str = "system/workflow-os-core";
const MAXIMUM_OBSERVATION_AGE_SECONDS: u32 = 300;

/// Presence-only posture for the exact configured access reference.
///
/// This type intentionally cannot carry access material.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GitHubRepositoryAccessPresence {
    Present,
    Missing,
}

/// Exact, caller-supplied facts from which Core derives GitHub read authority.
///
/// Authority inventories, availability records, generations, watermarks, and
/// access values are deliberately absent from this input.
pub(crate) struct GitHubRepositoryAuthorityInput<'a> {
    pub(crate) configuration: &'a GitHubRepositoryMetadataReadConfiguration,
    pub(crate) actor: &'a ActorId,
    pub(crate) workflow_id: &'a WorkflowId,
    pub(crate) run_id: &'a WorkflowRunId,
    pub(crate) step_id: &'a StepId,
    pub(crate) harness_contract_id: &'a HarnessContractId,
    pub(crate) target: &'a GitHubRepositoryMetadataResource,
    pub(crate) sensitivity: WorkReportSensitivity,
    pub(crate) evaluated_at: Timestamp,
    pub(crate) expires_at: Timestamp,
    pub(crate) operation_mode: AdapterOperationMode,
    pub(crate) access_presence: GitHubRepositoryAccessPresence,
}

#[derive(Serialize)]
struct AuthorityCommitmentInput<'a> {
    declaration: &'a GitHubRepositoryMetadataReadConfiguration,
    authored_operation: &'static str,
    internal_capability: &'static str,
    actor: &'a ActorId,
    workflow_id: &'a WorkflowId,
    run_id: &'a WorkflowRunId,
    step_id: &'a StepId,
    harness_contract_id: &'a HarnessContractId,
    target: &'a GitHubRepositoryMetadataResource,
    sensitivity: WorkReportSensitivity,
    evaluated_at: Timestamp,
    expires_at: Timestamp,
    operation_mode: AdapterOperationMode,
    access_presence: GitHubRepositoryAccessPresence,
}

struct DerivedAuthorityMaterial {
    configuration_commitment: SpecContentHash,
    grant: CapabilityGrant,
    availability: CapabilityAvailabilityRecord,
}

/// Derives one Core-owned, payload-free authority source for the exact GitHub
/// repository metadata operation.
///
/// The returned source is unversioned. Its snapshot watermark is derived by
/// the registered source from the committed inventory; callers cannot provide
/// either a source generation or a watermark.
pub(crate) fn derive_github_repository_authority(
    input: &GitHubRepositoryAuthorityInput<'_>,
) -> Result<RegisteredInMemoryCurrentAuthoritySource, WorkflowOsError> {
    let sensitivity = input.sensitivity;
    let evaluated_at = input.evaluated_at;
    let material = derive_authority_material(input)?;

    RegisteredInMemoryCurrentAuthoritySource::register(
        RegisteredInMemoryCurrentAuthoritySourceInput {
            source_id: CurrentAuthoritySourceId::new(SOURCE_ID)?,
            contract_version: CurrentAuthoritySourceContractVersion::new(SOURCE_CONTRACT_VERSION)?,
            configuration_commitment: material.configuration_commitment,
            core_maximum_observation_age_seconds: MAXIMUM_OBSERVATION_AGE_SECONDS,
            sensitivity,
            observed_at: evaluated_at,
            source_valid_through: Some(input.expires_at),
            generation: None,
            complete_grant_inventory: vec![material.grant],
            complete_availability_inventory: vec![material.availability],
            complete_context_reference_inventory: Vec::new(),
        },
    )
}

fn derive_authority_material(
    input: &GitHubRepositoryAuthorityInput<'_>,
) -> Result<DerivedAuthorityMaterial, WorkflowOsError> {
    validate_input(input)?;

    let configuration_commitment = hash_serializable(
        "core-github-repository-metadata-authority-v1",
        &AuthorityCommitmentInput {
            declaration: input.configuration,
            authored_operation: AUTHORED_OPERATION,
            internal_capability: INTERNAL_CAPABILITY,
            actor: input.actor,
            workflow_id: input.workflow_id,
            run_id: input.run_id,
            step_id: input.step_id,
            harness_contract_id: input.harness_contract_id,
            target: input.target,
            sensitivity: input.sensitivity,
            evaluated_at: input.evaluated_at,
            expires_at: input.expires_at,
            operation_mode: input.operation_mode,
            access_presence: input.access_presence,
        },
    )?;
    let capability = mapped_internal_capability()?;
    let resource = CapabilityResourceScope::new(
        CapabilityResourceKind::AdapterResource,
        canonical_resource_reference(input.target),
    )?;
    let grant = CapabilityGrant::new(CapabilityGrantDefinition {
        grant_id: CapabilityGrantId::new(format!(
            "grant/core/github-repository-metadata/{}",
            configuration_commitment.as_str()
        ))?,
        subject: input.actor.clone(),
        capability: capability.clone(),
        resource: resource.clone(),
        scope: CapabilityGrantScope::new(
            input.workflow_id.clone(),
            Some(input.run_id.clone()),
            Some(input.step_id.clone()),
            Some(input.harness_contract_id.clone()),
        )?,
        issuer: ActorId::new(CORE_ISSUER)?,
        issued_at: input.evaluated_at,
        expires_at: Some(input.expires_at),
        lifecycle: CapabilityGrantLifecycle::Active,
        revocation_reference: None,
        delegation: CapabilityDelegationPosture::Disabled,
        requirements: CapabilityGrantRequirements::default(),
        sensitivity_ceiling: input.sensitivity,
        redaction: RedactionMetadata::empty(),
    })?;
    let availability = CapabilityAvailabilityRecord::new(
        capability,
        resource,
        match (input.operation_mode, input.access_presence) {
            (AdapterOperationMode::Fixture, _) | (_, GitHubRepositoryAccessPresence::Present) => {
                CapabilityAvailability::Available
            }
            (_, GitHubRepositoryAccessPresence::Missing) => {
                CapabilityAvailability::DeclaredNotConnected
            }
        },
        input.evaluated_at,
        RedactionMetadata::empty(),
    )?;

    Ok(DerivedAuthorityMaterial {
        configuration_commitment,
        grant,
        availability,
    })
}

fn validate_input(input: &GitHubRepositoryAuthorityInput<'_>) -> Result<(), WorkflowOsError> {
    if !matches!(
        input.operation_mode,
        AdapterOperationMode::Fixture | AdapterOperationMode::LiveReadOnly
    ) {
        return Err(authority_error(
            "operation_mode_unsupported",
            "GitHub repository authority requires fixture or live read-only mode",
        ));
    }
    if input.sensitivity == WorkReportSensitivity::Unknown {
        return Err(authority_error(
            "sensitivity_unknown",
            "GitHub repository authority requires known sensitivity",
        ));
    }
    if !input
        .configuration
        .resources()
        .iter()
        .any(|resource| resource == input.target)
    {
        return Err(authority_error(
            "target_not_declared",
            "GitHub repository authority target is not declared",
        ));
    }
    Ok(())
}

fn mapped_internal_capability() -> Result<CapabilityReference, WorkflowOsError> {
    match AUTHORED_OPERATION {
        "github.repository.metadata" => CapabilityReference::new(INTERNAL_CAPABILITY),
        _ => Err(authority_error(
            "operation_unsupported",
            "GitHub repository authority operation is unsupported",
        )),
    }
}

fn canonical_resource_reference(target: &GitHubRepositoryMetadataResource) -> String {
    format!("github/{}/{}", target.owner(), target.repository())
}

fn authority_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("current_authority.github_repository.{suffix}"),
        message,
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::*;

    fn evaluated_at() -> Timestamp {
        Timestamp::parse_rfc3339("2026-10-31T12:00:00Z").expect("timestamp")
    }

    struct Fixture {
        configuration: GitHubRepositoryMetadataReadConfiguration,
        actor: ActorId,
        workflow_id: WorkflowId,
        run_id: WorkflowRunId,
        step_id: StepId,
        harness_contract_id: HarnessContractId,
        target: GitHubRepositoryMetadataResource,
    }

    impl Fixture {
        fn new() -> Self {
            let target = GitHubRepositoryMetadataResource::new("acme", "widgets").expect("target");
            Self {
                configuration: GitHubRepositoryMetadataReadConfiguration::new(
                    true,
                    GitHubRepositoryMetadataReadConfiguration::ENDPOINT,
                    GitHubRepositoryMetadataReadConfiguration::ACCESS_REFERENCE,
                    vec![target.clone()],
                )
                .expect("configuration"),
                actor: ActorId::new("agent/github-reader").expect("actor"),
                workflow_id: WorkflowId::new("workflow/github-read").expect("workflow"),
                run_id: WorkflowRunId::new("run/github-read/1").expect("run"),
                step_id: StepId::new("read-repository").expect("step"),
                harness_contract_id: HarnessContractId::new("harness/github-read")
                    .expect("harness"),
                target,
            }
        }

        fn input(
            &self,
            access_presence: GitHubRepositoryAccessPresence,
        ) -> GitHubRepositoryAuthorityInput<'_> {
            GitHubRepositoryAuthorityInput {
                configuration: &self.configuration,
                actor: &self.actor,
                workflow_id: &self.workflow_id,
                run_id: &self.run_id,
                step_id: &self.step_id,
                harness_contract_id: &self.harness_contract_id,
                target: &self.target,
                sensitivity: WorkReportSensitivity::Internal,
                evaluated_at: evaluated_at(),
                expires_at: Timestamp::parse_rfc3339("2026-10-31T12:05:00Z").expect("expiry"),
                operation_mode: AdapterOperationMode::LiveReadOnly,
                access_presence,
            }
        }
    }

    #[test]
    fn derives_exact_internal_capability_and_bound_scope() {
        let fixture = Fixture::new();
        let material =
            derive_authority_material(&fixture.input(GitHubRepositoryAccessPresence::Present))
                .expect("authority material");

        assert_eq!(material.grant.capability().as_str(), "github.read");
        assert_eq!(material.grant.resource().reference(), "github/acme/widgets");
        assert_eq!(material.grant.subject(), &fixture.actor);
        assert_eq!(material.grant.scope().workflow_id(), &fixture.workflow_id);
        assert_eq!(material.grant.scope().run_id(), Some(&fixture.run_id));
        assert_eq!(material.grant.scope().step_id(), Some(&fixture.step_id));
        assert_eq!(
            material.grant.expires_at(),
            Some(Timestamp::parse_rfc3339("2026-10-31T12:05:00Z").expect("expiry"))
        );
        assert_eq!(
            material.grant.scope().harness_contract_id(),
            Some(&fixture.harness_contract_id)
        );
        assert_eq!(
            material.availability.availability(),
            CapabilityAvailability::Available
        );
    }

    #[test]
    fn missing_access_is_declared_not_connected() {
        let fixture = Fixture::new();
        let material =
            derive_authority_material(&fixture.input(GitHubRepositoryAccessPresence::Missing))
                .expect("authority material");

        assert_eq!(
            material.availability.availability(),
            CapabilityAvailability::DeclaredNotConnected
        );
    }

    #[test]
    fn rejects_target_outside_validated_declaration() {
        let fixture = Fixture::new();
        let undeclared =
            GitHubRepositoryMetadataResource::new("acme", "other").expect("undeclared target");
        let mut input = fixture.input(GitHubRepositoryAccessPresence::Present);
        input.target = &undeclared;

        let error = derive_authority_material(&input)
            .err()
            .expect("undeclared target must fail");

        assert_eq!(
            error.code(),
            "current_authority.github_repository.target_not_declared"
        );
    }

    #[test]
    fn commitments_bind_declaration_and_execution_without_access_values() {
        let fixture = Fixture::new();
        let first =
            derive_authority_material(&fixture.input(GitHubRepositoryAccessPresence::Present))
                .expect("first authority material");
        let other_actor = ActorId::new("agent/other-reader").expect("other actor");
        let mut changed = fixture.input(GitHubRepositoryAccessPresence::Present);
        changed.actor = &other_actor;
        let second = derive_authority_material(&changed).expect("changed authority material");

        assert_ne!(
            first.configuration_commitment,
            second.configuration_commitment
        );
        assert_ne!(first.grant.grant_id(), second.grant.grant_id());
        let serialized = serde_json::to_string(&AuthorityCommitmentInput {
            declaration: &fixture.configuration,
            authored_operation: AUTHORED_OPERATION,
            internal_capability: INTERNAL_CAPABILITY,
            actor: &fixture.actor,
            workflow_id: &fixture.workflow_id,
            run_id: &fixture.run_id,
            step_id: &fixture.step_id,
            harness_contract_id: &fixture.harness_contract_id,
            target: &fixture.target,
            sensitivity: WorkReportSensitivity::Internal,
            evaluated_at: evaluated_at(),
            expires_at: Timestamp::parse_rfc3339("2026-10-31T12:05:00Z").expect("expiry"),
            operation_mode: AdapterOperationMode::LiveReadOnly,
            access_presence: GitHubRepositoryAccessPresence::Present,
        })
        .expect("commitment serialization");
        assert!(serialized.contains(GitHubRepositoryMetadataReadConfiguration::ACCESS_REFERENCE));
        assert!(!serialized.contains("token-value"));
    }

    #[test]
    fn registers_one_core_owned_source_without_caller_generation_or_watermark() {
        let fixture = Fixture::new();

        let input = fixture.input(GitHubRepositoryAccessPresence::Present);
        let source = derive_github_repository_authority(&input);

        assert!(source.is_ok());
    }
}
