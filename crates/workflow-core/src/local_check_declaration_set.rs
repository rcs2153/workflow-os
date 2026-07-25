use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    compute_local_check_command_contract_fingerprint, ImmutableRunBundleVersion,
    LocalCheckAttestationFreshnessPolicy, LocalCheckAttestationRequirement,
    LocalCheckAttestationRequirementDefinition, LocalCheckCommandContract, LocalCheckCommandId,
    LocalCheckCommandKind, LocalCheckNetworkPolicy, LocalCheckRequirementDeclaration,
    LocalCheckRequirementId, LocalCheckRequirementLevel, LocalCheckResultStatus,
    LocalCheckSideEffectClass, SpecContentHash, StepId, WorkflowDefinition, WorkflowId,
    WorkflowOsError, WorkflowVersion,
};

const DECLARATION_SET_DOMAIN: &str = "workflow-os/local-check-declaration-set/v1";
const OBLIGATION_DOMAIN: &str = "workflow-os/local-check-declaration-obligation/v1";

/// Versioned algorithm used to resolve and fingerprint local-check declarations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalLocalCheckDeclarationSetAlgorithm {
    /// First canonical declaration-set algorithm.
    V1,
}

/// Explicit validated inventory of allowlisted local-check command contracts.
#[derive(Clone, Eq, PartialEq)]
pub struct LocalCheckCommandContractInventory {
    contracts: Vec<LocalCheckCommandContract>,
}

impl LocalCheckCommandContractInventory {
    /// Creates an explicit validated command-contract inventory.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when a contract is invalid or a
    /// command identity is ambiguous.
    pub fn new(contracts: Vec<LocalCheckCommandContract>) -> Result<Self, WorkflowOsError> {
        let mut seen = BTreeSet::new();
        for contract in &contracts {
            contract.validate().map_err(|_| {
                declaration_set_error(
                    "inventory.invalid",
                    "local check command inventory contains an invalid contract",
                )
            })?;
            if !seen.insert(contract.command_id().clone()) {
                return Err(declaration_set_error(
                    "inventory.duplicate_command",
                    "local check command inventory contains an ambiguous command identity",
                ));
            }
        }
        Ok(Self { contracts })
    }

    /// Returns the validated contracts.
    #[must_use]
    pub fn contracts(&self) -> &[LocalCheckCommandContract] {
        &self.contracts
    }

    fn resolve(&self, command_id: &LocalCheckCommandId) -> Option<&LocalCheckCommandContract> {
        self.contracts
            .iter()
            .find(|contract| contract.command_id() == command_id)
    }
}

impl fmt::Debug for LocalCheckCommandContractInventory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckCommandContractInventory")
            .field("contract_count", &self.contracts.len())
            .finish()
    }
}

/// One canonical resolved local-check obligation.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CanonicalLocalCheckDeclaration {
    requirement_id: LocalCheckRequirementId,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    command_contract_fingerprint: SpecContentHash,
    attestation_requirement_fingerprint: SpecContentHash,
    requirement_level: LocalCheckRequirementLevel,
    minimum_assurance: crate::LocalCheckAttestationAssurance,
    accepted_statuses: Vec<LocalCheckResultStatus>,
    freshness: LocalCheckAttestationFreshnessPolicy,
    exact_immutable_run_binding_required: bool,
    truncation_allowed: bool,
    network_maximum: LocalCheckNetworkPolicy,
    side_effect_maximum: LocalCheckSideEffectClass,
    obligation_identity: SpecContentHash,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CanonicalLocalCheckDeclarationWire {
    requirement_id: LocalCheckRequirementId,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    command_contract_fingerprint: SpecContentHash,
    attestation_requirement_fingerprint: SpecContentHash,
    requirement_level: LocalCheckRequirementLevel,
    minimum_assurance: crate::LocalCheckAttestationAssurance,
    accepted_statuses: Vec<LocalCheckResultStatus>,
    freshness: LocalCheckAttestationFreshnessPolicy,
    exact_immutable_run_binding_required: bool,
    truncation_allowed: bool,
    network_maximum: LocalCheckNetworkPolicy,
    side_effect_maximum: LocalCheckSideEffectClass,
    obligation_identity: SpecContentHash,
}

impl CanonicalLocalCheckDeclaration {
    fn resolve(
        workflow_id: &WorkflowId,
        workflow_version: &WorkflowVersion,
        step_id: &StepId,
        bundle_version: &ImmutableRunBundleVersion,
        declaration: &LocalCheckRequirementDeclaration,
        contract: &LocalCheckCommandContract,
    ) -> Result<Self, WorkflowOsError> {
        validate_contract_maxima(declaration, contract)?;
        let attestation_requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: declaration.command_id().clone(),
                minimum_assurance: declaration.minimum_assurance(),
                accepted_statuses: declaration.accepted_statuses().to_vec(),
                freshness: declaration.freshness(),
                exact_immutable_run_binding_required: declaration
                    .exact_immutable_run_binding_required(),
                truncation_allowed: declaration.truncation_allowed(),
            })
            .map_err(|_| {
                declaration_set_error(
                    "requirement.invalid",
                    "local check declaration cannot form an independent attestation requirement",
                )
            })?;
        let command_contract_fingerprint =
            compute_local_check_command_contract_fingerprint(contract);
        let attestation_requirement_fingerprint =
            attestation_requirement.requirement_fingerprint().clone();
        let mut resolved = Self {
            requirement_id: declaration.id().clone(),
            command_id: declaration.command_id().clone(),
            command_kind: contract.command_kind(),
            command_contract_fingerprint,
            attestation_requirement_fingerprint,
            requirement_level: declaration.requirement_level(),
            minimum_assurance: declaration.minimum_assurance(),
            accepted_statuses: declaration.accepted_statuses().to_vec(),
            freshness: declaration.freshness(),
            exact_immutable_run_binding_required: declaration
                .exact_immutable_run_binding_required(),
            truncation_allowed: declaration.truncation_allowed(),
            network_maximum: declaration.network_maximum(),
            side_effect_maximum: declaration.side_effect_maximum(),
            obligation_identity: SpecContentHash::from_bytes([]),
        };
        resolved.obligation_identity = compute_obligation_identity(
            workflow_id,
            workflow_version,
            step_id,
            bundle_version,
            &resolved,
        );
        Ok(resolved)
    }

    fn validate(
        &self,
        workflow_id: &WorkflowId,
        workflow_version: &WorkflowVersion,
        step_id: &StepId,
        bundle_version: &ImmutableRunBundleVersion,
    ) -> Result<(), WorkflowOsError> {
        let requirement =
            LocalCheckAttestationRequirement::new(LocalCheckAttestationRequirementDefinition {
                command_id: self.command_id.clone(),
                minimum_assurance: self.minimum_assurance,
                accepted_statuses: self.accepted_statuses.clone(),
                freshness: self.freshness,
                exact_immutable_run_binding_required: self.exact_immutable_run_binding_required,
                truncation_allowed: self.truncation_allowed,
            })
            .map_err(|_| {
                declaration_set_error(
                    "record.requirement_invalid",
                    "canonical local check declaration contains an invalid requirement",
                )
            })?;
        if requirement.requirement_fingerprint() != &self.attestation_requirement_fingerprint {
            return Err(declaration_set_error(
                "record.requirement_fingerprint_mismatch",
                "canonical local check declaration requirement fingerprint does not match",
            ));
        }
        if self.network_maximum != LocalCheckNetworkPolicy::Disabled
            || self.side_effect_maximum == LocalCheckSideEffectClass::Unclassified
        {
            return Err(declaration_set_error(
                "record.posture_invalid",
                "canonical local check declaration contains unsupported posture",
            ));
        }
        let expected = compute_obligation_identity(
            workflow_id,
            workflow_version,
            step_id,
            bundle_version,
            self,
        );
        if expected != self.obligation_identity {
            return Err(declaration_set_error(
                "record.obligation_identity_mismatch",
                "canonical local check declaration obligation identity does not match",
            ));
        }
        Ok(())
    }

    /// Returns the authored requirement identity.
    #[must_use]
    pub const fn requirement_id(&self) -> &LocalCheckRequirementId {
        &self.requirement_id
    }

    /// Returns the resolved command identity.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the resolved command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the canonical command-contract fingerprint.
    #[must_use]
    pub const fn command_contract_fingerprint(&self) -> &SpecContentHash {
        &self.command_contract_fingerprint
    }

    /// Returns the independent attestation-requirement fingerprint.
    #[must_use]
    pub const fn attestation_requirement_fingerprint(&self) -> &SpecContentHash {
        &self.attestation_requirement_fingerprint
    }

    /// Returns the requirement level.
    #[must_use]
    pub const fn requirement_level(&self) -> LocalCheckRequirementLevel {
        self.requirement_level
    }

    /// Returns the minimum independent assurance.
    #[must_use]
    pub const fn minimum_assurance(&self) -> crate::LocalCheckAttestationAssurance {
        self.minimum_assurance
    }

    /// Returns the accepted result statuses.
    #[must_use]
    pub fn accepted_statuses(&self) -> &[LocalCheckResultStatus] {
        &self.accepted_statuses
    }

    /// Returns the freshness policy.
    #[must_use]
    pub const fn freshness(&self) -> LocalCheckAttestationFreshnessPolicy {
        self.freshness
    }

    /// Returns whether exact immutable-run binding is required.
    #[must_use]
    pub const fn exact_immutable_run_binding_required(&self) -> bool {
        self.exact_immutable_run_binding_required
    }

    /// Returns whether bounded truncation is allowed.
    #[must_use]
    pub const fn truncation_allowed(&self) -> bool {
        self.truncation_allowed
    }

    /// Returns the declared network maximum.
    #[must_use]
    pub const fn network_maximum(&self) -> LocalCheckNetworkPolicy {
        self.network_maximum
    }

    /// Returns the declared `SideEffect` maximum.
    #[must_use]
    pub const fn side_effect_maximum(&self) -> LocalCheckSideEffectClass {
        self.side_effect_maximum
    }

    /// Returns the deterministic obligation identity.
    #[must_use]
    pub const fn obligation_identity(&self) -> &SpecContentHash {
        &self.obligation_identity
    }
}

impl fmt::Debug for CanonicalLocalCheckDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CanonicalLocalCheckDeclaration")
            .field("requirement_id", &"[REDACTED]")
            .field("command_id", &"[REDACTED]")
            .field("command_kind", &self.command_kind)
            .field("requirement_level", &self.requirement_level)
            .field("obligation_identity", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl CanonicalLocalCheckDeclarationWire {
    fn into_declaration(self) -> CanonicalLocalCheckDeclaration {
        CanonicalLocalCheckDeclaration {
            requirement_id: self.requirement_id,
            command_id: self.command_id,
            command_kind: self.command_kind,
            command_contract_fingerprint: self.command_contract_fingerprint,
            attestation_requirement_fingerprint: self.attestation_requirement_fingerprint,
            requirement_level: self.requirement_level,
            minimum_assurance: self.minimum_assurance,
            accepted_statuses: self.accepted_statuses,
            freshness: self.freshness,
            exact_immutable_run_binding_required: self.exact_immutable_run_binding_required,
            truncation_allowed: self.truncation_allowed,
            network_maximum: self.network_maximum,
            side_effect_maximum: self.side_effect_maximum,
            obligation_identity: self.obligation_identity,
        }
    }
}

/// Content-addressed canonical declaration-set record for one workflow step.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CanonicalLocalCheckDeclarationSetRecord {
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    step_id: StepId,
    immutable_bundle_version: ImmutableRunBundleVersion,
    algorithm: CanonicalLocalCheckDeclarationSetAlgorithm,
    declarations: Vec<CanonicalLocalCheckDeclaration>,
    declaration_set_fingerprint: SpecContentHash,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CanonicalLocalCheckDeclarationSetRecordWire {
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    step_id: StepId,
    immutable_bundle_version: ImmutableRunBundleVersion,
    algorithm: CanonicalLocalCheckDeclarationSetAlgorithm,
    declarations: Vec<CanonicalLocalCheckDeclarationWire>,
    declaration_set_fingerprint: SpecContentHash,
}

impl CanonicalLocalCheckDeclarationSetRecord {
    fn build(
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        step_id: StepId,
        immutable_bundle_version: ImmutableRunBundleVersion,
        algorithm: CanonicalLocalCheckDeclarationSetAlgorithm,
        mut declarations: Vec<CanonicalLocalCheckDeclaration>,
        expected_fingerprint: Option<SpecContentHash>,
    ) -> Result<Self, WorkflowOsError> {
        declarations
            .sort_by(|left, right| left.obligation_identity.cmp(&right.obligation_identity));
        let mut requirement_ids = BTreeSet::new();
        let mut command_ids = BTreeSet::new();
        let mut obligation_ids = BTreeSet::new();
        for declaration in &declarations {
            declaration.validate(
                &workflow_id,
                &workflow_version,
                &step_id,
                &immutable_bundle_version,
            )?;
            if !requirement_ids.insert(declaration.requirement_id.clone()) {
                return Err(declaration_set_error(
                    "record.duplicate_requirement",
                    "canonical local check declaration set repeats a requirement identity",
                ));
            }
            if !command_ids.insert(declaration.command_id.clone()) {
                return Err(declaration_set_error(
                    "record.duplicate_command",
                    "canonical local check declaration set repeats a command obligation",
                ));
            }
            if !obligation_ids.insert(declaration.obligation_identity.clone()) {
                return Err(declaration_set_error(
                    "record.duplicate_obligation",
                    "canonical local check declaration set repeats an obligation identity",
                ));
            }
        }
        let mut record = Self {
            workflow_id,
            workflow_version,
            step_id,
            immutable_bundle_version,
            algorithm,
            declarations,
            declaration_set_fingerprint: SpecContentHash::from_bytes([]),
        };
        record.declaration_set_fingerprint = compute_declaration_set_fingerprint(&record);
        if expected_fingerprint
            .is_some_and(|expected| expected != record.declaration_set_fingerprint)
        {
            return Err(declaration_set_error(
                "record.fingerprint_mismatch",
                "canonical local check declaration-set fingerprint does not match",
            ));
        }
        Ok(record)
    }

    /// Returns the workflow identity bound by this record.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow version bound by this record.
    #[must_use]
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }

    /// Returns the exact workflow step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the immutable bundle model version used during resolution.
    #[must_use]
    pub const fn immutable_bundle_version(&self) -> &ImmutableRunBundleVersion {
        &self.immutable_bundle_version
    }

    /// Returns the declaration-set algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> CanonicalLocalCheckDeclarationSetAlgorithm {
        self.algorithm
    }

    /// Returns declarations in canonical obligation-identity order.
    #[must_use]
    pub fn declarations(&self) -> &[CanonicalLocalCheckDeclaration] {
        &self.declarations
    }

    /// Returns the deterministic content-derived declaration-set fingerprint.
    #[must_use]
    pub const fn declaration_set_fingerprint(&self) -> &SpecContentHash {
        &self.declaration_set_fingerprint
    }

    /// Returns the payload-free reference used by immutable run bundles.
    #[must_use]
    pub fn declaration_set_reference(&self) -> CanonicalLocalCheckDeclarationSetReference {
        CanonicalLocalCheckDeclarationSetReference {
            workflow_id: self.workflow_id.clone(),
            workflow_version: self.workflow_version.clone(),
            step_id: self.step_id.clone(),
            immutable_bundle_version: self.immutable_bundle_version.clone(),
            algorithm: self.algorithm,
            declaration_set_fingerprint: self.declaration_set_fingerprint.clone(),
        }
    }
}

impl fmt::Debug for CanonicalLocalCheckDeclarationSetRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CanonicalLocalCheckDeclarationSetRecord")
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("immutable_bundle_version", &"[REDACTED]")
            .field("algorithm", &self.algorithm)
            .field("declaration_count", &self.declarations.len())
            .field("declaration_set_fingerprint", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CanonicalLocalCheckDeclarationSetRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CanonicalLocalCheckDeclarationSetRecordWire::deserialize(deserializer)?;
        let declarations = wire
            .declarations
            .into_iter()
            .map(CanonicalLocalCheckDeclarationWire::into_declaration)
            .collect();
        Self::build(
            wire.workflow_id,
            wire.workflow_version,
            wire.step_id,
            wire.immutable_bundle_version,
            wire.algorithm,
            declarations,
            Some(wire.declaration_set_fingerprint),
        )
        .map_err(|_| serde::de::Error::custom("invalid canonical local check declaration set"))
    }
}

/// Payload-free immutable-bundle reference to one canonical step declaration set.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalLocalCheckDeclarationSetReference {
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    step_id: StepId,
    immutable_bundle_version: ImmutableRunBundleVersion,
    algorithm: CanonicalLocalCheckDeclarationSetAlgorithm,
    declaration_set_fingerprint: SpecContentHash,
}

impl CanonicalLocalCheckDeclarationSetReference {
    /// Returns the workflow identity bound by the referenced record.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the workflow version bound by the referenced record.
    #[must_use]
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }

    /// Returns the exact workflow step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the immutable bundle model version used during resolution.
    #[must_use]
    pub const fn immutable_bundle_version(&self) -> &ImmutableRunBundleVersion {
        &self.immutable_bundle_version
    }

    /// Returns the canonical resolution algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> CanonicalLocalCheckDeclarationSetAlgorithm {
        self.algorithm
    }

    /// Returns the referenced declaration-set content address.
    #[must_use]
    pub const fn declaration_set_fingerprint(&self) -> &SpecContentHash {
        &self.declaration_set_fingerprint
    }
}

impl fmt::Debug for CanonicalLocalCheckDeclarationSetReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CanonicalLocalCheckDeclarationSetReference")
            .field("workflow_id", &"[REDACTED]")
            .field("workflow_version", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("immutable_bundle_version", &"[REDACTED]")
            .field("algorithm", &self.algorithm)
            .field("declaration_set_fingerprint", &"[REDACTED]")
            .finish()
    }
}

/// Explicit inputs to pure canonical declaration-set resolution.
pub struct ResolveCanonicalLocalCheckDeclarationSetInput<'a> {
    /// Already validated workflow definition.
    pub workflow: &'a WorkflowDefinition,
    /// Exact step identity to resolve.
    pub step_id: &'a StepId,
    /// Explicit validated allowlisted command-contract inventory.
    pub command_inventory: &'a LocalCheckCommandContractInventory,
    /// Immutable bundle model version to bind into the record.
    pub immutable_bundle_version: ImmutableRunBundleVersion,
}

/// Resolves one step's declarations without repository inspection or execution.
///
/// # Errors
///
/// Returns a stable non-leaking error for a missing or ambiguous step,
/// unresolved command reference, incompatible declaration maximum, or invalid
/// canonical record.
pub fn resolve_canonical_local_check_declaration_set(
    input: ResolveCanonicalLocalCheckDeclarationSetInput<'_>,
) -> Result<CanonicalLocalCheckDeclarationSetRecord, WorkflowOsError> {
    let matching_steps = input
        .workflow
        .steps
        .iter()
        .filter(|step| &step.id == input.step_id)
        .collect::<Vec<_>>();
    let step = match matching_steps.as_slice() {
        [step] => *step,
        [] => {
            return Err(declaration_set_error(
                "step.missing",
                "canonical local check declaration resolution requires an exact workflow step",
            ));
        }
        _ => {
            return Err(declaration_set_error(
                "step.ambiguous",
                "canonical local check declaration resolution found an ambiguous workflow step",
            ));
        }
    };

    let mut declarations = Vec::with_capacity(step.local_check_requirements.len());
    for declaration in &step.local_check_requirements {
        let contract = input
            .command_inventory
            .resolve(declaration.command_id())
            .ok_or_else(|| {
                declaration_set_error(
                    "command.unresolved",
                    "canonical local check declaration references an unavailable command contract",
                )
            })?;
        declarations.push(CanonicalLocalCheckDeclaration::resolve(
            &input.workflow.id,
            &input.workflow.version,
            &step.id,
            &input.immutable_bundle_version,
            declaration,
            contract,
        )?);
    }

    CanonicalLocalCheckDeclarationSetRecord::build(
        input.workflow.id.clone(),
        input.workflow.version.clone(),
        step.id.clone(),
        input.immutable_bundle_version,
        CanonicalLocalCheckDeclarationSetAlgorithm::V1,
        declarations,
        None,
    )
}

fn validate_contract_maxima(
    declaration: &LocalCheckRequirementDeclaration,
    contract: &LocalCheckCommandContract,
) -> Result<(), WorkflowOsError> {
    if contract.network_policy() != declaration.network_maximum() {
        return Err(declaration_set_error(
            "command.network_exceeds_maximum",
            "local check command contract exceeds the declared network maximum",
        ));
    }
    let contract_rank = side_effect_rank(contract.side_effect_class())?;
    let maximum_rank = side_effect_rank(declaration.side_effect_maximum())?;
    if contract_rank > maximum_rank {
        return Err(declaration_set_error(
            "command.side_effect_exceeds_maximum",
            "local check command contract exceeds the declared SideEffect maximum",
        ));
    }
    Ok(())
}

fn side_effect_rank(value: LocalCheckSideEffectClass) -> Result<u8, WorkflowOsError> {
    match value {
        LocalCheckSideEffectClass::NoSourceWrites => Ok(0),
        LocalCheckSideEffectClass::BuildOrCacheWrites => Ok(1),
        LocalCheckSideEffectClass::Unclassified => Err(declaration_set_error(
            "command.side_effect_unclassified",
            "local check declaration resolution requires classified SideEffect posture",
        )),
    }
}

fn compute_obligation_identity(
    workflow_id: &WorkflowId,
    workflow_version: &WorkflowVersion,
    step_id: &StepId,
    bundle_version: &ImmutableRunBundleVersion,
    declaration: &CanonicalLocalCheckDeclaration,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", OBLIGATION_DOMAIN);
    hash_field(&mut hasher, "workflow_id", workflow_id.as_str());
    hash_field(&mut hasher, "workflow_version", workflow_version.as_str());
    hash_field(&mut hasher, "step_id", step_id.as_str());
    hash_field(
        &mut hasher,
        "immutable_bundle_version",
        bundle_version.as_str(),
    );
    hash_declaration(&mut hasher, declaration);
    SpecContentHash::from_bytes(hasher.finalize())
}

fn compute_declaration_set_fingerprint(
    record: &CanonicalLocalCheckDeclarationSetRecord,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", DECLARATION_SET_DOMAIN);
    hash_field(&mut hasher, "workflow_id", record.workflow_id.as_str());
    hash_field(
        &mut hasher,
        "workflow_version",
        record.workflow_version.as_str(),
    );
    hash_field(&mut hasher, "step_id", record.step_id.as_str());
    hash_field(
        &mut hasher,
        "immutable_bundle_version",
        record.immutable_bundle_version.as_str(),
    );
    hash_field(
        &mut hasher,
        "declaration_algorithm",
        declaration_algorithm_label(record.algorithm),
    );
    for declaration in &record.declarations {
        hash_field(
            &mut hasher,
            "obligation_identity",
            declaration.obligation_identity.as_str(),
        );
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_declaration(hasher: &mut Sha256, declaration: &CanonicalLocalCheckDeclaration) {
    hash_field(
        hasher,
        "requirement_id",
        declaration.requirement_id.as_str(),
    );
    hash_field(hasher, "command_id", declaration.command_id.as_str());
    hash_field(
        hasher,
        "command_kind",
        command_kind_label(declaration.command_kind),
    );
    hash_field(
        hasher,
        "command_contract_fingerprint",
        declaration.command_contract_fingerprint.as_str(),
    );
    hash_field(
        hasher,
        "attestation_requirement_fingerprint",
        declaration.attestation_requirement_fingerprint.as_str(),
    );
    hash_field(
        hasher,
        "requirement_level",
        requirement_level_label(declaration.requirement_level),
    );
    hash_field(
        hasher,
        "minimum_assurance",
        assurance_label(declaration.minimum_assurance),
    );
    for status in &declaration.accepted_statuses {
        hash_field(hasher, "accepted_status", &status.to_string());
    }
    match declaration.freshness {
        LocalCheckAttestationFreshnessPolicy::NoReuse => {
            hash_field(hasher, "freshness", "no_reuse");
        }
        LocalCheckAttestationFreshnessPolicy::MaxAgeSeconds { seconds } => {
            hash_field(hasher, "freshness", "max_age_seconds");
            hash_field(hasher, "freshness_seconds", &seconds.to_string());
        }
    }
    hash_field(
        hasher,
        "exact_immutable_run_binding_required",
        bool_label(declaration.exact_immutable_run_binding_required),
    );
    hash_field(
        hasher,
        "truncation_allowed",
        bool_label(declaration.truncation_allowed),
    );
    hash_field(
        hasher,
        "network_maximum",
        network_policy_label(declaration.network_maximum),
    );
    hash_field(
        hasher,
        "side_effect_maximum",
        side_effect_label(declaration.side_effect_maximum),
    );
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn declaration_algorithm_label(
    value: CanonicalLocalCheckDeclarationSetAlgorithm,
) -> &'static str {
    match value {
        CanonicalLocalCheckDeclarationSetAlgorithm::V1 => "v1",
    }
}

const fn requirement_level_label(value: LocalCheckRequirementLevel) -> &'static str {
    match value {
        LocalCheckRequirementLevel::Required => "required",
        LocalCheckRequirementLevel::Optional => "optional",
    }
}

const fn assurance_label(value: crate::LocalCheckAttestationAssurance) -> &'static str {
    match value {
        crate::LocalCheckAttestationAssurance::CallerAsserted => "caller_asserted",
        crate::LocalCheckAttestationAssurance::MockObserved => "mock_observed",
        crate::LocalCheckAttestationAssurance::KernelObservedLocalProcess => {
            "kernel_observed_local_process"
        }
        crate::LocalCheckAttestationAssurance::ExternalVerifier => "external_verifier",
    }
}

const fn command_kind_label(value: LocalCheckCommandKind) -> &'static str {
    match value {
        LocalCheckCommandKind::WorkflowOsValidateDogfood => "workflow_os_validate_dogfood",
        LocalCheckCommandKind::DocsCheck => "docs_check",
        LocalCheckCommandKind::CargoFmtCheck => "cargo_fmt_check",
        LocalCheckCommandKind::CargoClippyWorkspace => "cargo_clippy_workspace",
        LocalCheckCommandKind::CargoTestWorkspace => "cargo_test_workspace",
        LocalCheckCommandKind::TypeScriptCheck => "typescript_check",
        LocalCheckCommandKind::ContractCheck => "contract_check",
        LocalCheckCommandKind::IntegrationCheck => "integration_check",
    }
}

const fn network_policy_label(value: LocalCheckNetworkPolicy) -> &'static str {
    match value {
        LocalCheckNetworkPolicy::Disabled => "disabled",
    }
}

const fn side_effect_label(value: LocalCheckSideEffectClass) -> &'static str {
    match value {
        LocalCheckSideEffectClass::NoSourceWrites => "no_source_writes",
        LocalCheckSideEffectClass::BuildOrCacheWrites => "build_or_cache_writes",
        LocalCheckSideEffectClass::Unclassified => "unclassified",
    }
}

const fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn declaration_set_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("local_check.declaration_set.{suffix}"), message)
}
