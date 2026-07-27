use std::fmt;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    availability_order, hash_field, same_availability_key, AuthorityFactCompletenessPosture,
    AuthorityFactSourceKind, CurrentAuthorityFactSet, CurrentAuthorityFactSetInput,
    CurrentAuthorityQuerySet,
};
use crate::{
    capability_authority::grant_matches_execution_scope, CapabilityAvailabilityRecord,
    CapabilityGrant, RequiredContextContractBinding, RequiredContextExecutionBinding,
    SpecContentHash, Timestamp, WorkflowOsError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum InMemoryCurrentAuthoritySourceVersion {
    V1,
}

pub(super) struct InMemoryCurrentAuthoritySourceInput {
    pub(super) observed_at: Timestamp,
    pub(super) complete_grant_inventory: Vec<CapabilityGrant>,
    pub(super) complete_availability_inventory: Vec<CapabilityAvailabilityRecord>,
}

pub(super) struct CurrentAuthoritySourceQueryInput<'a> {
    pub(super) execution_binding: &'a RequiredContextExecutionBinding,
    pub(super) contract: &'a RequiredContextContractBinding,
    pub(super) evaluated_at: Timestamp,
}

pub(super) struct InMemoryCurrentAuthoritySource {
    version: InMemoryCurrentAuthoritySourceVersion,
    observed_at: Timestamp,
    grants: Vec<CapabilityGrant>,
    availability_records: Vec<CapabilityAvailabilityRecord>,
    inventory_hash: SpecContentHash,
}

impl InMemoryCurrentAuthoritySource {
    pub(super) fn new(input: InMemoryCurrentAuthoritySourceInput) -> Result<Self, WorkflowOsError> {
        let mut grants = input.complete_grant_inventory;
        for grant in &grants {
            grant.validate().map_err(|_| {
                source_error(
                    "inventory.grant_invalid",
                    "current authority source contains an invalid grant",
                )
            })?;
            if grant.issued_at() > input.observed_at {
                return Err(source_error(
                    "inventory.time_invalid",
                    "current authority source inventory time is invalid",
                ));
            }
        }
        grants.sort_by(|left, right| left.grant_id().as_str().cmp(right.grant_id().as_str()));
        if grants
            .windows(2)
            .any(|pair| pair[0].grant_id() == pair[1].grant_id())
        {
            return Err(source_error(
                "inventory.grant_duplicate",
                "current authority source contains duplicate grants",
            ));
        }

        let mut availability_records = input.complete_availability_inventory;
        if availability_records
            .iter()
            .any(|record| record.observed_at() > input.observed_at)
        {
            return Err(source_error(
                "inventory.time_invalid",
                "current authority source inventory time is invalid",
            ));
        }
        availability_records.sort_by(availability_order);
        if availability_records
            .windows(2)
            .any(|pair| same_availability_key(&pair[0], &pair[1]))
        {
            return Err(source_error(
                "inventory.availability_duplicate",
                "current authority source contains duplicate availability records",
            ));
        }

        let version = InMemoryCurrentAuthoritySourceVersion::V1;
        let inventory_hash = source_hash(
            "in-memory-current-authority-inventory-v1",
            &(version, input.observed_at, &grants, &availability_records),
        )?;
        Ok(Self {
            version,
            observed_at: input.observed_at,
            grants,
            availability_records,
            inventory_hash,
        })
    }

    pub(super) fn query(
        &self,
        input: &CurrentAuthoritySourceQueryInput<'_>,
    ) -> Result<CurrentAuthorityFactSet, WorkflowOsError> {
        if input.execution_binding.contract_content_hash() != input.contract.content_hash() {
            return Err(source_error(
                "query.binding_mismatch",
                "current authority source query does not match its execution binding",
            ));
        }
        if input.evaluated_at < input.execution_binding.bound_at()
            || self.observed_at > input.evaluated_at
        {
            return Err(source_error(
                "query.time_invalid",
                "current authority source query time is invalid",
            ));
        }

        let query_set = CurrentAuthorityQuerySet::from_contract(input.contract).map_err(|_| {
            source_error("query.invalid", "current authority source query is invalid")
        })?;
        let grants = self
            .grants
            .iter()
            .filter(|grant| {
                query_set.queries().iter().any(|query| {
                    grant.capability() == query.capability()
                        && grant.resource() == query.resource()
                        && grant_matches_execution_scope(
                            grant,
                            input.execution_binding.actor(),
                            input.execution_binding.workflow_id(),
                            input.execution_binding.run_id(),
                            input.execution_binding.step_id(),
                            Some(input.execution_binding.harness_contract_id()),
                        )
                })
            })
            .cloned()
            .collect::<Vec<_>>();

        let mut availability_records = Vec::with_capacity(query_set.queries().len());
        for query in query_set.queries() {
            let mut matches = self.availability_records.iter().filter(|record| {
                record.capability() == query.capability() && record.resource() == query.resource()
            });
            let record = matches.next().ok_or_else(|| {
                source_error(
                    "query.availability_missing",
                    "current authority source query is missing availability",
                )
            })?;
            if matches.next().is_some() {
                return Err(source_error(
                    "query.availability_ambiguous",
                    "current authority source query has ambiguous availability",
                ));
            }
            availability_records.push(record.clone());
        }

        CurrentAuthorityFactSet::new(CurrentAuthorityFactSetInput {
            execution_binding: input.execution_binding,
            contract: input.contract,
            source_kind: AuthorityFactSourceKind::InMemoryInventorySnapshot,
            source_snapshot_hash: self.inventory_hash.clone(),
            source_observed_at: self.observed_at,
            completeness: AuthorityFactCompletenessPosture::CompleteForExactQuery,
            evaluated_at: input.evaluated_at,
            grants,
            availability_records,
        })
        .map_err(|_| {
            source_error(
                "fact_set.invalid",
                "current authority source could not construct a valid fact set",
            )
        })
    }

    pub(super) const fn inventory_hash(&self) -> &SpecContentHash {
        &self.inventory_hash
    }

    pub(super) const fn observed_at(&self) -> Timestamp {
        self.observed_at
    }
}

impl fmt::Debug for InMemoryCurrentAuthoritySource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryCurrentAuthoritySource")
            .field("version", &self.version)
            .field("observed_at", &"[REDACTED]")
            .field("grant_count", &self.grants.len())
            .field("availability_count", &self.availability_records.len())
            .field("inventory_hash", &"[REDACTED]")
            .finish()
    }
}

fn source_hash(domain: &str, value: &impl Serialize) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        source_error(
            "inventory.hash_failed",
            "current authority source hashing failed",
        )
    })?;
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", domain.as_bytes());
    hash_field(&mut hasher, "value", &bytes);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn source_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("current_authority.source.{suffix}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::{
        build_immutable_run_bundle, load_project, ActorId, CapabilityAvailability,
        CapabilityDelegationPosture, CapabilityGrantDefinition, CapabilityGrantId,
        CapabilityGrantLifecycle, CapabilityGrantRequirements, CapabilityGrantScope,
        GovernedContextAccessLevel, GovernedContextReferenceTarget, HarnessContractId,
        HarnessContractVersion, ImmutableRunBundleBuildRequest, ImmutableRunBundleExecutionPosture,
        ImmutableRunBundleHandlerPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
        ImmutableRunBundleReferencePosture, ImmutableRunBundleSensitivity,
        ImmutableRunBundleVersion, LocalImmutableRunBundleStore, RedactionMetadata,
        RequiredContextExecutionBindingInput, RequiredContextObligation,
        RequiredContextRequirement, RequiredContextRequirementId, SkillId, SkillVersion, StepId,
        WorkReportId, WorkReportSensitivity, WorkflowId, WorkflowRunId, SUPPORTED_SCHEMA_VERSION,
    };

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(1);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-authority-source-{name}-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("test root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("parent");
            }
            fs::write(path, content).expect("fixture");
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("timestamp")
    }

    fn fixture() -> (
        RequiredContextContractBinding,
        RequiredContextExecutionBinding,
    ) {
        let project_root = TestRoot::new("project");
        let store_root = TestRoot::new("store");
        project_root.write(
            "workflow-os.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nproject:\n  id: authority/project\n  name: Authority Project\n"
            ),
        );
        project_root.write(
            "workflows/build.workflow.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: authority/build\nversion: v1\ndisplay_name: Authority Build\ntriggers:\n  - id: manual\n    kind: manual\nsteps:\n  - id: consume\n    skill_ref:\n      id: local/check\n      version: v1\n    policy_requirements:\n      - id: local/read-only\n    terminal_behavior: fail_workflow\ncancellation_behavior: stop\naudit_requirements:\n  required: true\n  events: [RunCreated]\n  store_references_only: true\nobservability_requirements:\n  metrics: [workflow_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project_root.write(
            "skills/check.skill.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/check\nversion: v1\ndisplay_name: Check\nallowed_capabilities:\n  - name: local.read\ninput_contract:\n  fields:\n    - name: request\n      field_type: string\noutput_contract:\n  fields:\n    - name: summary\n      field_type: string\nfailure_modes:\n  - code: failed\n    description: Failed.\n    retryable: false\naudit_requirements:\n  required: true\n  events: [SkillInvocationRequested]\n  store_references_only: true\nobservability_requirements:\n  metrics: [skill_latency]\n  tracing: true\n  latency_tracking: true\n"
            ),
        );
        project_root.write(
            "policies/read-only.policy.yml",
            &format!(
                "schema_version: {SUPPORTED_SCHEMA_VERSION}\nid: local/read-only\nname: Read only\nrules:\n  - id: allow\n    effect: allow_local\n"
            ),
        );
        let loaded = load_project(project_root.path());
        assert!(!loaded.has_errors(), "{:?}", loaded.diagnostics);
        let project = loaded.bundle.expect("project");
        let built = build_immutable_run_bundle(ImmutableRunBundleBuildRequest {
            project: &project,
            workflow_id: &WorkflowId::new("authority/build").expect("workflow"),
            bundle_id: ImmutableRunBundleId::new("bundle/authority").expect("bundle"),
            bundle_version: ImmutableRunBundleVersion::new("v1").expect("version"),
            run_id: WorkflowRunId::new("run-authority").expect("run"),
            resolved_execution_context_hash: SpecContentHash::from_text("context"),
            execution_posture: ImmutableRunBundleExecutionPosture::new(
                Vec::new(),
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
            )
            .expect("posture"),
            handlers: vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("local/check").expect("skill"),
                skill_version: SkillVersion::new("v1").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            created_at: timestamp("2026-07-26T10:00:00Z"),
            created_by: ActorId::new("system/kernel").expect("actor"),
            sensitivity: ImmutableRunBundleSensitivity::Internal,
            redaction_required: true,
        })
        .expect("built");
        let store = LocalImmutableRunBundleStore::new(store_root.path());
        store.write_bundle(&built).expect("write");
        let stored = store
            .read_bundle(built.manifest().run_id(), built.manifest().bundle_id())
            .expect("read");
        let contract = RequiredContextContractBinding::new(
            HarnessContractId::new("harness/context").expect("contract"),
            HarnessContractVersion::new("v1").expect("contract version"),
            vec![
                RequiredContextRequirement::new(
                    RequiredContextRequirementId::new("required/report-reference")
                        .expect("requirement"),
                    GovernedContextReferenceTarget::WorkReport(
                        WorkReportId::new("report/current").expect("report"),
                    ),
                    GovernedContextAccessLevel::ReferenceOnly,
                    RequiredContextObligation::Required,
                    WorkReportSensitivity::Internal,
                )
                .expect("requirement"),
                RequiredContextRequirement::new(
                    RequiredContextRequirementId::new("required/report-metadata")
                        .expect("requirement"),
                    GovernedContextReferenceTarget::WorkReport(
                        WorkReportId::new("report/metadata").expect("report"),
                    ),
                    GovernedContextAccessLevel::BoundedMetadata,
                    RequiredContextObligation::Required,
                    WorkReportSensitivity::Internal,
                )
                .expect("requirement"),
            ],
        )
        .expect("contract");
        let binding = RequiredContextExecutionBinding::new(RequiredContextExecutionBindingInput {
            bundle: &stored,
            contract: &contract,
            actor: ActorId::new("agent/consumer").expect("actor"),
            step_id: StepId::new("consume").expect("step"),
            maximum_sensitivity: WorkReportSensitivity::Internal,
            bound_at: timestamp("2026-07-26T10:10:00Z"),
        })
        .expect("binding");
        (contract, binding)
    }

    fn availability(
        contract: &RequiredContextContractBinding,
        posture: CapabilityAvailability,
    ) -> Vec<CapabilityAvailabilityRecord> {
        contract
            .requirements()
            .iter()
            .map(|requirement| {
                CapabilityAvailabilityRecord::new(
                    requirement
                        .access_level()
                        .required_capability()
                        .expect("capability"),
                    requirement
                        .target()
                        .capability_resource()
                        .expect("resource"),
                    posture,
                    timestamp("2026-07-26T10:20:00Z"),
                    RedactionMetadata::empty(),
                )
                .expect("availability")
            })
            .collect()
    }

    fn grant(
        contract: &RequiredContextContractBinding,
        grant_id: &str,
        actor: &str,
        lifecycle: CapabilityGrantLifecycle,
        exact_scope: bool,
        expires_at: Option<&str>,
    ) -> CapabilityGrant {
        let requirement = &contract.requirements()[0];
        CapabilityGrant::new(CapabilityGrantDefinition {
            grant_id: CapabilityGrantId::new(grant_id).expect("grant id"),
            subject: ActorId::new(actor).expect("actor"),
            capability: requirement
                .access_level()
                .required_capability()
                .expect("capability"),
            resource: requirement
                .target()
                .capability_resource()
                .expect("resource"),
            scope: CapabilityGrantScope::new(
                WorkflowId::new("authority/build").expect("workflow"),
                exact_scope.then(|| WorkflowRunId::new("run-authority").expect("run")),
                exact_scope.then(|| StepId::new("consume").expect("step")),
                exact_scope.then(|| HarnessContractId::new("harness/context").expect("harness")),
            )
            .expect("scope"),
            issuer: ActorId::new("system/authority").expect("issuer"),
            issued_at: timestamp("2026-07-26T10:05:00Z"),
            expires_at: expires_at.map(timestamp),
            lifecycle,
            revocation_reference: (lifecycle == CapabilityGrantLifecycle::Revoked)
                .then(|| "revocation/record".to_owned()),
            delegation: CapabilityDelegationPosture::Disabled,
            requirements: CapabilityGrantRequirements::default(),
            sensitivity_ceiling: WorkReportSensitivity::Internal,
            redaction: RedactionMetadata::empty(),
        })
        .expect("grant")
    }

    fn source(
        grants: Vec<CapabilityGrant>,
        availability_records: Vec<CapabilityAvailabilityRecord>,
    ) -> Result<InMemoryCurrentAuthoritySource, WorkflowOsError> {
        InMemoryCurrentAuthoritySource::new(InMemoryCurrentAuthoritySourceInput {
            observed_at: timestamp("2026-07-26T10:20:00Z"),
            complete_grant_inventory: grants,
            complete_availability_inventory: availability_records,
        })
    }

    fn query(
        source: &InMemoryCurrentAuthoritySource,
        binding: &RequiredContextExecutionBinding,
        contract: &RequiredContextContractBinding,
    ) -> Result<CurrentAuthorityFactSet, WorkflowOsError> {
        source.query(&CurrentAuthoritySourceQueryInput {
            execution_binding: binding,
            contract,
            evaluated_at: timestamp("2026-07-26T10:30:00Z"),
        })
    }

    #[test]
    fn complete_inventory_produces_exact_query_fact_set() {
        let (contract, binding) = fixture();
        let grants = vec![
            grant(
                &contract,
                "grant/workflow",
                "agent/consumer",
                CapabilityGrantLifecycle::Active,
                false,
                None,
            ),
            grant(
                &contract,
                "grant/exact-revoked",
                "agent/consumer",
                CapabilityGrantLifecycle::Revoked,
                true,
                None,
            ),
            grant(
                &contract,
                "grant/other-actor",
                "agent/other",
                CapabilityGrantLifecycle::Active,
                true,
                None,
            ),
        ];
        let source = source(
            grants,
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect("source");
        let fact_set = query(&source, &binding, &contract).expect("fact set");

        assert_eq!(fact_set.query_set().queries().len(), 2);
        assert_eq!(fact_set.grants().len(), 2);
        assert!(fact_set
            .grants()
            .iter()
            .any(|grant| grant.lifecycle() == CapabilityGrantLifecycle::Revoked));
        assert!(fact_set
            .grants()
            .iter()
            .all(|grant| grant.subject() == binding.actor()));
        assert_eq!(fact_set.availability_records().len(), 2);
        assert_eq!(
            fact_set.source().completeness,
            AuthorityFactCompletenessPosture::CompleteForExactQuery
        );
    }

    #[test]
    fn complete_inventory_retains_expired_candidate_and_allows_zero_grants() {
        let (contract, binding) = fixture();
        let expired = grant(
            &contract,
            "grant/expired",
            "agent/consumer",
            CapabilityGrantLifecycle::Active,
            true,
            Some("2026-07-26T10:15:00Z"),
        );
        let source_with_grant = source(
            vec![expired],
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect("source");
        assert_eq!(
            query(&source_with_grant, &binding, &contract)
                .expect("fact set")
                .grants()
                .len(),
            1
        );

        let source_without_grants = source(
            Vec::new(),
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect("source");
        assert!(query(&source_without_grants, &binding, &contract)
            .expect("fact set")
            .grants()
            .is_empty());
    }

    #[test]
    fn inventory_order_is_canonical_and_out_of_query_records_remain_committed() {
        let (contract, binding) = fixture();
        let matching = grant(
            &contract,
            "grant/matching",
            "agent/consumer",
            CapabilityGrantLifecycle::Active,
            true,
            None,
        );
        let unrelated = grant(
            &contract,
            "grant/unrelated",
            "agent/other",
            CapabilityGrantLifecycle::Active,
            true,
            None,
        );
        let records = availability(&contract, CapabilityAvailability::Available);
        let first =
            source(vec![matching.clone(), unrelated.clone()], records.clone()).expect("source");
        let second = source(vec![unrelated.clone(), matching.clone()], {
            let mut reversed = records.clone();
            reversed.reverse();
            reversed
        })
        .expect("source");
        assert_eq!(first.inventory_hash(), second.inventory_hash());

        let without_unrelated = source(vec![matching], records).expect("source");
        assert_ne!(first.inventory_hash(), without_unrelated.inventory_hash());
        assert_eq!(
            query(&first, &binding, &contract)
                .expect("fact set")
                .grants()
                .len(),
            1
        );
    }

    #[test]
    fn missing_or_duplicate_inventory_records_fail_closed() {
        let (contract, binding) = fixture();
        let missing = source(Vec::new(), Vec::new()).expect("source");
        let error = query(&missing, &binding, &contract).expect_err("missing");
        assert_eq!(
            error.code(),
            "current_authority.source.query.availability_missing"
        );

        let mut duplicate_availability = availability(&contract, CapabilityAvailability::Available);
        duplicate_availability.push(duplicate_availability[0].clone());
        let error = source(Vec::new(), duplicate_availability).expect_err("duplicate");
        assert_eq!(
            error.code(),
            "current_authority.source.inventory.availability_duplicate"
        );

        let duplicate_grant = grant(
            &contract,
            "grant/duplicate",
            "agent/consumer",
            CapabilityGrantLifecycle::Active,
            true,
            None,
        );
        let error = source(
            vec![duplicate_grant.clone(), duplicate_grant],
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect_err("duplicate");
        assert_eq!(
            error.code(),
            "current_authority.source.inventory.grant_duplicate"
        );
    }

    #[test]
    fn explicit_nonready_availability_remains_a_complete_fact() {
        let (contract, binding) = fixture();
        for posture in [
            CapabilityAvailability::DeclaredNotConnected,
            CapabilityAvailability::KnownUnsupported,
            CapabilityAvailability::Unknown,
        ] {
            let source = source(Vec::new(), availability(&contract, posture)).expect("source");
            let fact_set = query(&source, &binding, &contract).expect("fact set");
            assert!(fact_set
                .availability_records()
                .iter()
                .all(|record| record.availability() == posture));
        }
    }

    #[test]
    fn inconsistent_inventory_and_query_times_fail_closed() {
        let (contract, binding) = fixture();
        let future_record = CapabilityAvailabilityRecord::new(
            contract.requirements()[0]
                .access_level()
                .required_capability()
                .expect("capability"),
            contract.requirements()[0]
                .target()
                .capability_resource()
                .expect("resource"),
            CapabilityAvailability::Available,
            timestamp("2026-07-26T10:21:00Z"),
            RedactionMetadata::empty(),
        )
        .expect("availability");
        let error = source(Vec::new(), vec![future_record]).expect_err("time");
        assert_eq!(
            error.code(),
            "current_authority.source.inventory.time_invalid"
        );

        let source = source(
            Vec::new(),
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect("source");
        let error = source
            .query(&CurrentAuthoritySourceQueryInput {
                execution_binding: &binding,
                contract: &contract,
                evaluated_at: timestamp("2026-07-26T10:15:00Z"),
            })
            .expect_err("query time");
        assert_eq!(error.code(), "current_authority.source.query.time_invalid");
    }

    #[test]
    fn substituted_contract_fails_closed() {
        let (contract, binding) = fixture();
        let substituted = RequiredContextContractBinding::new(
            HarnessContractId::new("harness/context").expect("contract"),
            HarnessContractVersion::new("v1").expect("contract version"),
            vec![RequiredContextRequirement::new(
                RequiredContextRequirementId::new("required/substituted").expect("requirement"),
                GovernedContextReferenceTarget::WorkReport(
                    WorkReportId::new("report/substituted").expect("report"),
                ),
                GovernedContextAccessLevel::ReferenceOnly,
                RequiredContextObligation::Required,
                WorkReportSensitivity::Internal,
            )
            .expect("requirement")],
        )
        .expect("contract");
        let source = source(
            Vec::new(),
            availability(&substituted, CapabilityAvailability::Available),
        )
        .expect("source");
        let error = query(&source, &binding, &substituted).expect_err("substitution");
        assert_eq!(
            error.code(),
            "current_authority.source.query.binding_mismatch"
        );
        assert_ne!(contract.content_hash(), substituted.content_hash());
    }

    #[test]
    fn debug_and_errors_do_not_leak_source_values() {
        let (contract, binding) = fixture();
        let source = source(
            vec![grant(
                &contract,
                "grant/private-reference",
                "agent/consumer",
                CapabilityGrantLifecycle::Active,
                true,
                None,
            )],
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect("source");
        let debug = format!("{source:?}");
        for forbidden in [
            "grant/private-reference",
            "agent/consumer",
            "report/current",
            source.inventory_hash().as_str(),
            "2026-07-26T10:20:00Z",
        ] {
            assert!(!debug.contains(forbidden));
        }

        let missing = InMemoryCurrentAuthoritySource::new(InMemoryCurrentAuthoritySourceInput {
            observed_at: timestamp("2026-07-26T10:20:00Z"),
            complete_grant_inventory: Vec::new(),
            complete_availability_inventory: Vec::new(),
        })
        .expect("source");
        let error = query(&missing, &binding, &contract).expect_err("missing");
        assert!(!error.to_string().contains("report/current"));
        assert!(!error.to_string().contains("agent/consumer"));
    }

    #[test]
    fn v1_inventory_hash_is_stable_and_framing_is_unambiguous() {
        let (contract, _binding) = fixture();
        let source = source(
            vec![grant(
                &contract,
                "grant/vector",
                "agent/consumer",
                CapabilityGrantLifecycle::Active,
                true,
                None,
            )],
            availability(&contract, CapabilityAvailability::Available),
        )
        .expect("source");
        assert_eq!(
            source.inventory_hash().as_str(),
            "7a6b4d1950768957abc7807420bed208a8267592d5a99f5cb842b3ac1b67bf2e"
        );

        let left = source_hash("a", &"bc").expect("hash");
        let right = source_hash("ab", &"c").expect("hash");
        assert_ne!(left, right);
    }
}
