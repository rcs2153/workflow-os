use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ImmutableRunBundleBinding, LocalCheckCommandContract, LocalCheckCommandId,
    LocalCheckCommandKind, LocalCheckEnvironmentPolicy, LocalCheckExecutionPosture,
    LocalCheckNetworkPolicy, LocalCheckRedactionPolicy, LocalCheckSideEffectClass,
    LocalCheckSideEffectKind, LocalCheckWorkingDirectoryPolicy, SkillId, SkillVersion,
    SpecContentHash, StepId, Timestamp, WorkReportCitationKind, WorkflowId, WorkflowOsError,
    WorkflowRunId,
};

/// Versioned algorithm for immutable local-check execution bindings.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImmutableLocalCheckExecutionBindingAlgorithm {
    /// Initial fixed-width framed SHA-256 binding.
    V1,
}

impl ImmutableLocalCheckExecutionBindingAlgorithm {
    /// Returns the domain-separated algorithm identifier.
    #[must_use]
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::V1 => "workflow-os/immutable-local-check-execution-binding/v1",
        }
    }
}

impl<'de> Deserialize<'de> for ImmutableLocalCheckExecutionBindingAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "immutable local check execution binding algorithm is invalid",
            )),
        }
    }
}

/// Registration mode frozen for a selected local-check handler.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImmutableLocalCheckHandlerRegistrationMode {
    /// The caller supplied an explicit reviewed registration profile.
    ExplicitProfile,
}

/// Honest handler assurance frozen before local-check observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImmutableLocalCheckHandlerPosture {
    /// A handler selection is explicit, but its implementation is unattested.
    RegisteredUnattested,
}

/// Payload-free handler selection metadata frozen before execution.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ImmutableLocalCheckHandlerSelection {
    command_kind: LocalCheckCommandKind,
    skill_id: SkillId,
    skill_version: SkillVersion,
    registration_mode: ImmutableLocalCheckHandlerRegistrationMode,
    posture: ImmutableLocalCheckHandlerPosture,
    selection_fingerprint: SpecContentHash,
}

impl ImmutableLocalCheckHandlerSelection {
    /// Creates a validated handler selection commitment.
    ///
    /// This commits typed registration metadata. It does not attest handler
    /// source, binary, or implementation identity.
    #[must_use]
    pub fn registered_unattested(
        command_kind: LocalCheckCommandKind,
        skill_id: SkillId,
        skill_version: SkillVersion,
    ) -> Self {
        let mut selection = Self {
            command_kind,
            skill_id,
            skill_version,
            registration_mode: ImmutableLocalCheckHandlerRegistrationMode::ExplicitProfile,
            posture: ImmutableLocalCheckHandlerPosture::RegisteredUnattested,
            selection_fingerprint: SpecContentHash::from_text("pending"),
        };
        selection.selection_fingerprint = compute_handler_selection_fingerprint(&selection);
        selection
    }

    /// Returns the selected command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the selected skill ID.
    #[must_use]
    pub const fn skill_id(&self) -> &SkillId {
        &self.skill_id
    }

    /// Returns the selected skill version.
    #[must_use]
    pub const fn skill_version(&self) -> &SkillVersion {
        &self.skill_version
    }

    /// Returns the explicit registration mode.
    #[must_use]
    pub const fn registration_mode(&self) -> ImmutableLocalCheckHandlerRegistrationMode {
        self.registration_mode
    }

    /// Returns the honest handler posture.
    #[must_use]
    pub const fn posture(&self) -> ImmutableLocalCheckHandlerPosture {
        self.posture
    }

    /// Returns the canonical selection fingerprint.
    #[must_use]
    pub const fn selection_fingerprint(&self) -> &SpecContentHash {
        &self.selection_fingerprint
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.selection_fingerprint != compute_handler_selection_fingerprint(self) {
            return Err(binding_error(
                "handler_selection.fingerprint_mismatch",
                "immutable local check handler selection fingerprint is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for ImmutableLocalCheckHandlerSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableLocalCheckHandlerSelection")
            .field("command_kind", &self.command_kind)
            .field("skill_identity", &"[REDACTED]")
            .field("registration_mode", &self.registration_mode)
            .field("posture", &self.posture)
            .field("selection_fingerprint", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct ImmutableLocalCheckHandlerSelectionWire {
    command_kind: LocalCheckCommandKind,
    skill_id: SkillId,
    skill_version: SkillVersion,
    registration_mode: ImmutableLocalCheckHandlerRegistrationMode,
    posture: ImmutableLocalCheckHandlerPosture,
    selection_fingerprint: SpecContentHash,
}

impl<'de> Deserialize<'de> for ImmutableLocalCheckHandlerSelection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ImmutableLocalCheckHandlerSelectionWire::deserialize(deserializer)?;
        let selection = Self {
            command_kind: wire.command_kind,
            skill_id: wire.skill_id,
            skill_version: wire.skill_version,
            registration_mode: wire.registration_mode,
            posture: wire.posture,
            selection_fingerprint: wire.selection_fingerprint,
        };
        selection.validate().map_err(|_| {
            serde::de::Error::custom("invalid immutable local check handler selection")
        })?;
        Ok(selection)
    }
}

/// Immutable payload-free context for one future local-check execution.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ImmutableLocalCheckExecutionBinding {
    algorithm: ImmutableLocalCheckExecutionBindingAlgorithm,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    skill_id: SkillId,
    skill_version: SkillVersion,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    command_contract_fingerprint: SpecContentHash,
    handler_selection: ImmutableLocalCheckHandlerSelection,
    effective_policy_fingerprint: SpecContentHash,
    created_at: Timestamp,
    binding_fingerprint: SpecContentHash,
}

/// Input fields for an immutable local-check execution binding.
pub struct ImmutableLocalCheckExecutionBindingDefinition<'a> {
    /// Exact immutable run-bundle binding.
    pub immutable_run_bundle: ImmutableRunBundleBinding,
    /// Workflow identity.
    pub workflow_id: WorkflowId,
    /// Run identity.
    pub run_id: WorkflowRunId,
    /// Step identity.
    pub step_id: StepId,
    /// Resolved skill identity.
    pub skill_id: SkillId,
    /// Resolved skill version.
    pub skill_version: SkillVersion,
    /// Exact validated command contract selected before execution.
    pub command_contract: &'a LocalCheckCommandContract,
    /// Exact registered handler selection.
    pub handler_selection: ImmutableLocalCheckHandlerSelection,
    /// Binding creation time, before any future observation.
    pub created_at: Timestamp,
}

impl ImmutableLocalCheckExecutionBinding {
    /// Creates an immutable pre-execution local-check binding.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking validation errors when skill, command, handler,
    /// or policy context is inconsistent.
    pub fn new(
        definition: ImmutableLocalCheckExecutionBindingDefinition<'_>,
    ) -> Result<Self, WorkflowOsError> {
        definition.command_contract.validate()?;
        definition.handler_selection.validate()?;
        if definition.skill_id != *definition.handler_selection.skill_id()
            || definition.skill_version != *definition.handler_selection.skill_version()
        {
            return Err(binding_error(
                "handler_selection.skill_mismatch",
                "immutable local check handler selection does not match the resolved skill",
            ));
        }
        if definition.command_contract.command_kind() != definition.handler_selection.command_kind()
        {
            return Err(binding_error(
                "handler_selection.command_mismatch",
                "immutable local check handler selection does not match the command kind",
            ));
        }

        let mut binding = Self {
            algorithm: ImmutableLocalCheckExecutionBindingAlgorithm::V1,
            immutable_run_bundle: definition.immutable_run_bundle,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            step_id: definition.step_id,
            skill_id: definition.skill_id,
            skill_version: definition.skill_version,
            command_id: definition.command_contract.command_id().clone(),
            command_kind: definition.command_contract.command_kind(),
            command_contract_fingerprint: compute_local_check_command_contract_fingerprint(
                definition.command_contract,
            ),
            handler_selection: definition.handler_selection,
            effective_policy_fingerprint: compute_effective_policy_fingerprint(
                definition.command_contract,
            ),
            created_at: definition.created_at,
            binding_fingerprint: SpecContentHash::from_text("pending"),
        };
        binding.binding_fingerprint = compute_binding_fingerprint(&binding);
        binding.validate()?;
        Ok(binding)
    }

    /// Validates all internal commitments.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when serialized or constructed fields
    /// are inconsistent.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.handler_selection.validate()?;
        if self.skill_id != *self.handler_selection.skill_id()
            || self.skill_version != *self.handler_selection.skill_version()
        {
            return Err(binding_error(
                "handler_selection.skill_mismatch",
                "immutable local check handler selection does not match the resolved skill",
            ));
        }
        if self.command_kind != self.handler_selection.command_kind() {
            return Err(binding_error(
                "handler_selection.command_mismatch",
                "immutable local check handler selection does not match the command kind",
            ));
        }
        if self.binding_fingerprint != compute_binding_fingerprint(self) {
            return Err(binding_error(
                "fingerprint_mismatch",
                "immutable local check execution binding fingerprint is invalid",
            ));
        }
        Ok(())
    }

    /// Returns the algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> ImmutableLocalCheckExecutionBindingAlgorithm {
        self.algorithm
    }

    /// Returns the immutable run-bundle binding.
    #[must_use]
    pub const fn immutable_run_bundle(&self) -> &ImmutableRunBundleBinding {
        &self.immutable_run_bundle
    }

    /// Returns the workflow identity.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    /// Returns the run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }

    /// Returns the step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    /// Returns the resolved skill ID.
    #[must_use]
    pub const fn skill_id(&self) -> &SkillId {
        &self.skill_id
    }

    /// Returns the resolved skill version.
    #[must_use]
    pub const fn skill_version(&self) -> &SkillVersion {
        &self.skill_version
    }

    /// Returns the command identity.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the command kind.
    #[must_use]
    pub const fn command_kind(&self) -> LocalCheckCommandKind {
        self.command_kind
    }

    /// Returns the canonical command-contract fingerprint.
    #[must_use]
    pub const fn command_contract_fingerprint(&self) -> &SpecContentHash {
        &self.command_contract_fingerprint
    }

    /// Returns the frozen handler selection.
    #[must_use]
    pub const fn handler_selection(&self) -> &ImmutableLocalCheckHandlerSelection {
        &self.handler_selection
    }

    /// Returns the effective execution-policy fingerprint.
    #[must_use]
    pub const fn effective_policy_fingerprint(&self) -> &SpecContentHash {
        &self.effective_policy_fingerprint
    }

    /// Returns the binding creation timestamp.
    #[must_use]
    pub const fn created_at(&self) -> &Timestamp {
        &self.created_at
    }

    /// Returns the content-addressed binding fingerprint.
    #[must_use]
    pub const fn binding_fingerprint(&self) -> &SpecContentHash {
        &self.binding_fingerprint
    }
}

impl fmt::Debug for ImmutableLocalCheckExecutionBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableLocalCheckExecutionBinding")
            .field("algorithm", &self.algorithm)
            .field("immutable_run_bundle", &self.immutable_run_bundle)
            .field("workflow_identity", &"[REDACTED]")
            .field("run_identity", &"[REDACTED]")
            .field("step_identity", &"[REDACTED]")
            .field("skill_identity", &"[REDACTED]")
            .field("command_identity", &"[REDACTED]")
            .field("command_kind", &self.command_kind)
            .field("handler_selection", &self.handler_selection)
            .field("fingerprints", &"[REDACTED]")
            .field("created_at", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct ImmutableLocalCheckExecutionBindingWire {
    algorithm: ImmutableLocalCheckExecutionBindingAlgorithm,
    immutable_run_bundle: ImmutableRunBundleBinding,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    skill_id: SkillId,
    skill_version: SkillVersion,
    command_id: LocalCheckCommandId,
    command_kind: LocalCheckCommandKind,
    command_contract_fingerprint: SpecContentHash,
    handler_selection: ImmutableLocalCheckHandlerSelection,
    effective_policy_fingerprint: SpecContentHash,
    created_at: Timestamp,
    binding_fingerprint: SpecContentHash,
}

impl<'de> Deserialize<'de> for ImmutableLocalCheckExecutionBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ImmutableLocalCheckExecutionBindingWire::deserialize(deserializer)?;
        let binding = Self {
            algorithm: wire.algorithm,
            immutable_run_bundle: wire.immutable_run_bundle,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            skill_id: wire.skill_id,
            skill_version: wire.skill_version,
            command_id: wire.command_id,
            command_kind: wire.command_kind,
            command_contract_fingerprint: wire.command_contract_fingerprint,
            handler_selection: wire.handler_selection,
            effective_policy_fingerprint: wire.effective_policy_fingerprint,
            created_at: wire.created_at,
            binding_fingerprint: wire.binding_fingerprint,
        };
        binding.validate().map_err(|_| {
            serde::de::Error::custom("invalid immutable local check execution binding")
        })?;
        Ok(binding)
    }
}

/// Computes the canonical fingerprint of every decision-relevant command field.
#[must_use]
pub fn compute_local_check_command_contract_fingerprint(
    contract: &LocalCheckCommandContract,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "algorithm",
        "workflow-os/local-check-command-contract/v1",
    );
    hash_field(&mut hasher, "command_id", contract.command_id().as_str());
    hash_field(
        &mut hasher,
        "command_kind",
        command_kind_label(contract.command_kind()),
    );
    hash_field(
        &mut hasher,
        "execution_posture",
        execution_posture_label(contract.execution_posture()),
    );
    hash_field(&mut hasher, "executable", contract.executable());
    for argument in contract.arguments() {
        hash_field(&mut hasher, "argument", argument);
    }
    hash_field(
        &mut hasher,
        "working_directory_policy",
        working_directory_label(contract.working_directory_policy()),
    );
    hash_field(
        &mut hasher,
        "environment_policy",
        environment_policy_label(contract.environment_policy()),
    );
    let mut environment_names = contract.allowed_environment_variables().to_vec();
    environment_names.sort();
    for name in environment_names {
        hash_field(&mut hasher, "allowed_environment_variable", &name);
    }
    hash_field(
        &mut hasher,
        "network_policy",
        network_policy_label(contract.network_policy()),
    );
    hash_field(
        &mut hasher,
        "timeout_seconds",
        &contract.timeout_seconds().to_string(),
    );
    hash_field(
        &mut hasher,
        "side_effect_class",
        side_effect_class_label(contract.side_effect_class()),
    );
    let mut effects = contract.side_effect_boundary().allowed_effects().to_vec();
    effects.sort();
    for effect in effects {
        hash_field(
            &mut hasher,
            "allowed_side_effect",
            side_effect_kind_label(effect),
        );
    }
    let mut directories = contract.permitted_output_directories().to_vec();
    directories.sort();
    for directory in directories {
        hash_field(&mut hasher, "permitted_output_directory", &directory);
    }
    hash_field(
        &mut hasher,
        "stdout_max_bytes",
        &contract.output_capture().stdout_max_bytes.to_string(),
    );
    hash_field(
        &mut hasher,
        "stderr_max_bytes",
        &contract.output_capture().stderr_max_bytes.to_string(),
    );
    hash_field(
        &mut hasher,
        "persist_raw_output",
        bool_label(contract.output_capture().persist_raw_output),
    );
    hash_field(
        &mut hasher,
        "redaction_policy",
        redaction_policy_label(contract.redaction_policy()),
    );
    let mut citations = contract.citation_kinds().to_vec();
    citations.sort();
    for citation in citations {
        hash_field(&mut hasher, "citation_kind", citation_kind_label(citation));
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn compute_effective_policy_fingerprint(contract: &LocalCheckCommandContract) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "algorithm",
        "workflow-os/local-check-effective-policy/v1",
    );
    hash_field(
        &mut hasher,
        "working_directory_policy",
        working_directory_label(contract.working_directory_policy()),
    );
    hash_field(
        &mut hasher,
        "environment_policy",
        environment_policy_label(contract.environment_policy()),
    );
    let mut environment_names = contract.allowed_environment_variables().to_vec();
    environment_names.sort();
    for name in environment_names {
        hash_field(&mut hasher, "allowed_environment_variable", &name);
    }
    hash_field(
        &mut hasher,
        "network_policy",
        network_policy_label(contract.network_policy()),
    );
    hash_field(
        &mut hasher,
        "timeout_seconds",
        &contract.timeout_seconds().to_string(),
    );
    hash_field(
        &mut hasher,
        "side_effect_class",
        side_effect_class_label(contract.side_effect_class()),
    );
    let mut effects = contract.side_effect_boundary().allowed_effects().to_vec();
    effects.sort();
    for effect in effects {
        hash_field(
            &mut hasher,
            "allowed_side_effect",
            side_effect_kind_label(effect),
        );
    }
    let mut directories = contract.permitted_output_directories().to_vec();
    directories.sort();
    for directory in directories {
        hash_field(&mut hasher, "permitted_output_directory", &directory);
    }
    hash_field(
        &mut hasher,
        "stdout_max_bytes",
        &contract.output_capture().stdout_max_bytes.to_string(),
    );
    hash_field(
        &mut hasher,
        "stderr_max_bytes",
        &contract.output_capture().stderr_max_bytes.to_string(),
    );
    hash_field(
        &mut hasher,
        "persist_raw_output",
        bool_label(contract.output_capture().persist_raw_output),
    );
    hash_field(
        &mut hasher,
        "redaction_policy",
        redaction_policy_label(contract.redaction_policy()),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn compute_handler_selection_fingerprint(
    selection: &ImmutableLocalCheckHandlerSelection,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(
        &mut hasher,
        "algorithm",
        "workflow-os/local-check-handler-selection/v1",
    );
    hash_field(
        &mut hasher,
        "command_kind",
        command_kind_label(selection.command_kind),
    );
    hash_field(&mut hasher, "skill_id", selection.skill_id.as_str());
    hash_field(
        &mut hasher,
        "skill_version",
        selection.skill_version.as_str(),
    );
    hash_field(&mut hasher, "registration_mode", "explicit_profile");
    hash_field(&mut hasher, "posture", "registered_unattested");
    SpecContentHash::from_bytes(hasher.finalize())
}

fn compute_binding_fingerprint(binding: &ImmutableLocalCheckExecutionBinding) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "algorithm", binding.algorithm.identifier());
    hash_field(
        &mut hasher,
        "bundle_id",
        binding.immutable_run_bundle.bundle_id().as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_version",
        binding.immutable_run_bundle.bundle_version().as_str(),
    );
    hash_field(
        &mut hasher,
        "bundle_root",
        binding.immutable_run_bundle.root_hash().as_str(),
    );
    hash_field(&mut hasher, "workflow_id", binding.workflow_id.as_str());
    hash_field(&mut hasher, "run_id", binding.run_id.as_str());
    hash_field(&mut hasher, "step_id", binding.step_id.as_str());
    hash_field(&mut hasher, "skill_id", binding.skill_id.as_str());
    hash_field(&mut hasher, "skill_version", binding.skill_version.as_str());
    hash_field(&mut hasher, "command_id", binding.command_id.as_str());
    hash_field(
        &mut hasher,
        "command_kind",
        command_kind_label(binding.command_kind),
    );
    hash_field(
        &mut hasher,
        "command_contract_fingerprint",
        binding.command_contract_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "handler_selection_fingerprint",
        binding.handler_selection.selection_fingerprint.as_str(),
    );
    hash_field(
        &mut hasher,
        "effective_policy_fingerprint",
        binding.effective_policy_fingerprint.as_str(),
    );
    hash_field(&mut hasher, "created_at", &binding.created_at.to_rfc3339());
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
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

const fn execution_posture_label(value: LocalCheckExecutionPosture) -> &'static str {
    match value {
        LocalCheckExecutionPosture::ModelOnly => "model_only",
        LocalCheckExecutionPosture::AllowlistedHandlerOnly => "allowlisted_handler_only",
    }
}

const fn working_directory_label(value: LocalCheckWorkingDirectoryPolicy) -> &'static str {
    match value {
        LocalCheckWorkingDirectoryPolicy::RepositoryRoot => "repository_root",
        LocalCheckWorkingDirectoryPolicy::DogfoodProjectRoot => "dogfood_project_root",
    }
}

const fn environment_policy_label(value: LocalCheckEnvironmentPolicy) -> &'static str {
    match value {
        LocalCheckEnvironmentPolicy::SanitizedMinimal => "sanitized_minimal",
        LocalCheckEnvironmentPolicy::ExplicitAllowlistOnly => "explicit_allowlist_only",
    }
}

const fn network_policy_label(value: LocalCheckNetworkPolicy) -> &'static str {
    match value {
        LocalCheckNetworkPolicy::Disabled => "disabled",
    }
}

const fn side_effect_class_label(value: LocalCheckSideEffectClass) -> &'static str {
    match value {
        LocalCheckSideEffectClass::NoSourceWrites => "no_source_writes",
        LocalCheckSideEffectClass::BuildOrCacheWrites => "build_or_cache_writes",
        LocalCheckSideEffectClass::Unclassified => "unclassified",
    }
}

const fn side_effect_kind_label(value: LocalCheckSideEffectKind) -> &'static str {
    match value {
        LocalCheckSideEffectKind::SourceReadOnly => "source_read_only",
        LocalCheckSideEffectKind::CacheWriteOnly => "cache_write_only",
        LocalCheckSideEffectKind::BuildOutputWrite => "build_output_write",
        LocalCheckSideEffectKind::TempWriteOnly => "temp_write_only",
        LocalCheckSideEffectKind::SourceWrite => "source_write",
        LocalCheckSideEffectKind::NetworkAccess => "network_access",
        LocalCheckSideEffectKind::Unclassified => "unclassified",
    }
}

const fn redaction_policy_label(value: LocalCheckRedactionPolicy) -> &'static str {
    match value {
        LocalCheckRedactionPolicy::BoundedRedactedSummary => "bounded_redacted_summary",
    }
}

const fn citation_kind_label(value: WorkReportCitationKind) -> &'static str {
    match value {
        WorkReportCitationKind::EvidenceReference => "evidence_reference",
        WorkReportCitationKind::WorkflowEvent => "workflow_event",
        WorkReportCitationKind::AuditEvent => "audit_event",
        WorkReportCitationKind::AdapterTelemetry => "adapter_telemetry",
        WorkReportCitationKind::ValidationDiagnostic => "validation_diagnostic",
        WorkReportCitationKind::LocalCheckResult => "local_check_result",
        WorkReportCitationKind::TypedHandoff => "typed_handoff",
        WorkReportCitationKind::AgentHarnessHook => "agent_harness_hook",
        WorkReportCitationKind::AgentHarnessHookDisclosure => "agent_harness_hook_disclosure",
        WorkReportCitationKind::SideEffect => "side_effect",
        WorkReportCitationKind::ApprovalDecision => "approval_decision",
        WorkReportCitationKind::PolicyDecision => "policy_decision",
        WorkReportCitationKind::ReasoningLineageNode => "reasoning_lineage_node",
    }
}

const fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn binding_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("immutable_local_check_execution_binding.{suffix}"),
        message,
    )
}
