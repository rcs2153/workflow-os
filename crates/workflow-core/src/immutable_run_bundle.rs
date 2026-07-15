use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ActorId, SchemaVersion, SkillId, SkillVersion, SpecContentHash, StepId, Timestamp, WorkflowId,
    WorkflowOsError, WorkflowRunId, WorkflowVersion,
};

const ID_MAX_BYTES: usize = 128;
const REFERENCE_MAX_COUNT: usize = 256;
const BUNDLE_DOMAIN: &str = "workflow-os/immutable-run-bundle/v1";

macro_rules! bundle_id {
    ($name:ident, $label:literal, $code:literal) => {
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        #[doc = concat!("Validated ", $label, ".")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a validated ", $label, ".")]
            ///
            /// # Errors
            ///
            /// Returns a bounded validation error when the value is empty, too long, or contains
            /// unsupported characters.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                if value.is_empty()
                    || value.len() > ID_MAX_BYTES
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.')
                    })
                {
                    return Err(WorkflowOsError::validation(
                        $code,
                        concat!($label, " is invalid"),
                    ));
                }
                Ok(Self(value))
            }

            #[must_use]
            #[doc = concat!("Returns the ", $label, " text.")]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

bundle_id!(
    ImmutableRunBundleId,
    "immutable run bundle id",
    "immutable_run_bundle.id.invalid"
);
bundle_id!(
    ImmutableRunBundleVersion,
    "immutable run bundle version",
    "immutable_run_bundle.version.invalid"
);

/// Durable identity binding from a workflow run to its immutable bundle.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImmutableRunBundleBinding {
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    root_hash: SpecContentHash,
}

impl ImmutableRunBundleBinding {
    /// Returns the immutable bundle ID.
    #[must_use]
    pub const fn bundle_id(&self) -> &ImmutableRunBundleId {
        &self.bundle_id
    }

    /// Returns the immutable bundle model version.
    #[must_use]
    pub const fn bundle_version(&self) -> &ImmutableRunBundleVersion {
        &self.bundle_version
    }

    /// Returns the bundle integrity root.
    #[must_use]
    pub const fn root_hash(&self) -> &SpecContentHash {
        &self.root_hash
    }
}

impl fmt::Debug for ImmutableRunBundleBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleBinding")
            .field("bundle_id", &"[REDACTED]")
            .field("bundle_version", &"[REDACTED]")
            .field("root_hash", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Definition kinds that may be referenced by an immutable run bundle.
pub enum ImmutableRunBundleDefinitionKind {
    /// One validated workflow definition.
    Workflow,
    /// One resolved skill definition for an ordered step.
    Skill,
    /// One policy referenced by an executable step.
    Policy,
}

impl ImmutableRunBundleDefinitionKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Workflow => "workflow",
            Self::Skill => "skill",
            Self::Policy => "policy",
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ImmutableRunBundleDefinitionReferenceWire")]
/// Content-addressed reference to one canonical validated definition.
pub struct ImmutableRunBundleDefinitionReference {
    kind: ImmutableRunBundleDefinitionKind,
    definition_id: String,
    definition_version: Option<String>,
    schema_version: SchemaVersion,
    content_hash: SpecContentHash,
    step_id: Option<StepId>,
}

#[derive(Clone, Serialize, Deserialize)]
struct ImmutableRunBundleDefinitionReferenceWire {
    kind: ImmutableRunBundleDefinitionKind,
    definition_id: String,
    definition_version: Option<String>,
    schema_version: SchemaVersion,
    content_hash: SpecContentHash,
    step_id: Option<StepId>,
}

impl ImmutableRunBundleDefinitionReference {
    /// Creates and validates a definition reference.
    ///
    /// # Errors
    ///
    /// Returns a bounded validation error for an invalid identity or incompatible step posture.
    pub fn new(
        kind: ImmutableRunBundleDefinitionKind,
        definition_id: impl Into<String>,
        definition_version: Option<String>,
        schema_version: SchemaVersion,
        content_hash: SpecContentHash,
        step_id: Option<StepId>,
    ) -> Result<Self, WorkflowOsError> {
        let definition_id = definition_id.into();
        validate_bounded_identifier(&definition_id)?;
        if let Some(version) = &definition_version {
            validate_bounded_identifier(version)?;
        }
        if matches!(kind, ImmutableRunBundleDefinitionKind::Skill) != step_id.is_some() {
            return Err(WorkflowOsError::validation(
                "immutable_run_bundle.reference.step_posture",
                "only resolved skill references require a step id",
            ));
        }
        Ok(Self {
            kind,
            definition_id,
            definition_version,
            schema_version,
            content_hash,
            step_id,
        })
    }

    #[must_use]
    /// Returns the definition kind.
    pub const fn kind(&self) -> ImmutableRunBundleDefinitionKind {
        self.kind
    }
    #[must_use]
    /// Returns the stable definition ID.
    pub fn definition_id(&self) -> &str {
        &self.definition_id
    }
    #[must_use]
    /// Returns the optional definition version.
    pub fn definition_version(&self) -> Option<&str> {
        self.definition_version.as_deref()
    }
    #[must_use]
    /// Returns the definition schema version.
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }
    #[must_use]
    /// Returns the canonical definition content hash.
    pub const fn content_hash(&self) -> &SpecContentHash {
        &self.content_hash
    }
    #[must_use]
    /// Returns the resolved step ID for skill references.
    pub const fn step_id(&self) -> Option<&StepId> {
        self.step_id.as_ref()
    }
}

impl fmt::Debug for ImmutableRunBundleDefinitionReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleDefinitionReference")
            .field("kind", &self.kind)
            .field("definition_id", &"[REDACTED]")
            .field(
                "definition_version_present",
                &self.definition_version.is_some(),
            )
            .field("content_hash", &"[REDACTED]")
            .field("step_id_present", &self.step_id.is_some())
            .finish_non_exhaustive()
    }
}

impl TryFrom<ImmutableRunBundleDefinitionReferenceWire> for ImmutableRunBundleDefinitionReference {
    type Error = WorkflowOsError;
    fn try_from(value: ImmutableRunBundleDefinitionReferenceWire) -> Result<Self, Self::Error> {
        Self::new(
            value.kind,
            value.definition_id,
            value.definition_version,
            value.schema_version,
            value.content_hash,
            value.step_id,
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Preservation posture for an execution-context input.
pub enum ImmutableRunBundleReferencePosture {
    /// Stable references are committed and preserved.
    CommittedReference,
    /// Input was present but cannot yet be reconstructed.
    PresentNotPreserved,
    #[default]
    /// No input was supplied.
    NotSupplied,
    /// The input category is unsupported in this build.
    Unsupported,
}

#[derive(Clone, Default, Eq, PartialEq, Serialize)]
/// Bounded non-definition execution posture committed by the manifest.
pub struct ImmutableRunBundleExecutionPosture {
    /// Required before-skill checkpoint step IDs.
    required_checkpoint_step_ids: Vec<StepId>,
    /// Hook-input preservation posture.
    hook_inputs: ImmutableRunBundleReferencePosture,
    /// SideEffect-input preservation posture.
    side_effect_inputs: ImmutableRunBundleReferencePosture,
    /// Report-artifact policy preservation posture.
    report_artifact_policy: ImmutableRunBundleReferencePosture,
}

#[derive(Deserialize)]
struct ImmutableRunBundleExecutionPostureWire {
    required_checkpoint_step_ids: Vec<StepId>,
    hook_inputs: ImmutableRunBundleReferencePosture,
    side_effect_inputs: ImmutableRunBundleReferencePosture,
    report_artifact_policy: ImmutableRunBundleReferencePosture,
}

impl fmt::Debug for ImmutableRunBundleExecutionPosture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleExecutionPosture")
            .field(
                "required_checkpoint_count",
                &self.required_checkpoint_step_ids.len(),
            )
            .field("hook_inputs", &self.hook_inputs)
            .field("side_effect_inputs", &self.side_effect_inputs)
            .field("report_artifact_policy", &self.report_artifact_policy)
            .finish()
    }
}

impl ImmutableRunBundleExecutionPosture {
    /// Creates a validated execution posture.
    ///
    /// # Errors
    ///
    /// Returns a bounded validation error when checkpoint references are duplicated or exceed
    /// the supported count.
    pub fn new(
        required_checkpoint_step_ids: Vec<StepId>,
        hook_inputs: ImmutableRunBundleReferencePosture,
        side_effect_inputs: ImmutableRunBundleReferencePosture,
        report_artifact_policy: ImmutableRunBundleReferencePosture,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            required_checkpoint_step_ids,
            hook_inputs,
            side_effect_inputs,
            report_artifact_policy,
        };
        value.validate()?;
        Ok(value)
    }

    #[must_use]
    /// Returns required before-skill checkpoint step IDs.
    pub fn required_checkpoint_step_ids(&self) -> &[StepId] {
        &self.required_checkpoint_step_ids
    }

    #[must_use]
    /// Returns the hook-input preservation posture.
    pub const fn hook_inputs(&self) -> ImmutableRunBundleReferencePosture {
        self.hook_inputs
    }

    #[must_use]
    /// Returns the SideEffect-input preservation posture.
    pub const fn side_effect_inputs(&self) -> ImmutableRunBundleReferencePosture {
        self.side_effect_inputs
    }

    #[must_use]
    /// Returns the report-artifact policy preservation posture.
    pub const fn report_artifact_policy(&self) -> ImmutableRunBundleReferencePosture {
        self.report_artifact_policy
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.required_checkpoint_step_ids.len() > REFERENCE_MAX_COUNT {
            return Err(WorkflowOsError::validation(
                "immutable_run_bundle.execution_posture.too_many_checkpoints",
                "immutable run bundle has too many checkpoint references",
            ));
        }
        let unique = self
            .required_checkpoint_step_ids
            .iter()
            .collect::<BTreeSet<_>>();
        if unique.len() != self.required_checkpoint_step_ids.len() {
            return Err(WorkflowOsError::validation(
                "immutable_run_bundle.execution_posture.duplicate_checkpoint",
                "immutable run bundle checkpoint references must be unique",
            ));
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for ImmutableRunBundleExecutionPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ImmutableRunBundleExecutionPostureWire::deserialize(deserializer)?;
        Self::new(
            wire.required_checkpoint_step_ids,
            wire.hook_inputs,
            wire.side_effect_inputs,
            wire.report_artifact_policy,
        )
        .map_err(|_| serde::de::Error::custom("invalid immutable run bundle execution posture"))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Honest handler/check implementation posture for a resolved skill.
pub enum ImmutableRunBundleHandlerPosture {
    /// Only declarative skill identity is known.
    DeclaredOnly,
    /// A local handler was registered but its implementation is unattested.
    RegisteredUnattested,
    /// Preview mock execution was selected.
    MockSelected,
    /// No handler was available.
    Unavailable,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
/// Handler posture associated with one resolved skill identity.
pub struct ImmutableRunBundleHandlerReference {
    /// Resolved skill ID.
    pub skill_id: SkillId,
    /// Resolved skill version.
    pub skill_version: SkillVersion,
    /// Handler attestation posture.
    pub posture: ImmutableRunBundleHandlerPosture,
}

impl fmt::Debug for ImmutableRunBundleHandlerReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleHandlerReference")
            .field("skill_identity", &"[REDACTED]")
            .field("posture", &self.posture)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Conservative sensitivity classification for an immutable run bundle.
pub enum ImmutableRunBundleSensitivity {
    #[default]
    /// Internal project material.
    Internal,
    /// Confidential project material.
    Confidential,
    /// Restricted project material.
    Restricted,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
/// Immutable manifest of validated declarative run inputs and bounded posture.
pub struct ImmutableRunBundleManifest {
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    workflow_content_hash: SpecContentHash,
    resolved_execution_context_hash: SpecContentHash,
    definitions: Vec<ImmutableRunBundleDefinitionReference>,
    execution_posture: ImmutableRunBundleExecutionPosture,
    handlers: Vec<ImmutableRunBundleHandlerReference>,
    created_at: Timestamp,
    created_by: ActorId,
    sensitivity: ImmutableRunBundleSensitivity,
    redaction_required: bool,
    root_hash: SpecContentHash,
}

#[derive(Deserialize)]
struct ImmutableRunBundleManifestWire {
    bundle_id: ImmutableRunBundleId,
    bundle_version: ImmutableRunBundleVersion,
    run_id: WorkflowRunId,
    workflow_id: WorkflowId,
    workflow_version: WorkflowVersion,
    schema_version: SchemaVersion,
    workflow_content_hash: SpecContentHash,
    resolved_execution_context_hash: SpecContentHash,
    definitions: Vec<ImmutableRunBundleDefinitionReference>,
    execution_posture: ImmutableRunBundleExecutionPosture,
    handlers: Vec<ImmutableRunBundleHandlerReference>,
    created_at: Timestamp,
    created_by: ActorId,
    sensitivity: ImmutableRunBundleSensitivity,
    redaction_required: bool,
    root_hash: SpecContentHash,
}

impl ImmutableRunBundleManifest {
    /// Creates, validates, and deterministically hashes a manifest.
    ///
    /// # Errors
    ///
    /// Returns a bounded validation error when definition, handler, or execution-posture
    /// invariants are not satisfied.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bundle_id: ImmutableRunBundleId,
        bundle_version: ImmutableRunBundleVersion,
        run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        workflow_version: WorkflowVersion,
        schema_version: SchemaVersion,
        workflow_content_hash: SpecContentHash,
        resolved_execution_context_hash: SpecContentHash,
        mut definitions: Vec<ImmutableRunBundleDefinitionReference>,
        execution_posture: ImmutableRunBundleExecutionPosture,
        mut handlers: Vec<ImmutableRunBundleHandlerReference>,
        created_at: Timestamp,
        created_by: ActorId,
        sensitivity: ImmutableRunBundleSensitivity,
        redaction_required: bool,
    ) -> Result<Self, WorkflowOsError> {
        canonicalize_definition_references(&mut definitions);
        handlers.sort_by(|left, right| {
            (&left.skill_id, &left.skill_version).cmp(&(&right.skill_id, &right.skill_version))
        });
        validate_manifest_collections(&definitions, &execution_posture, &handlers)?;
        validate_workflow_reference_alignment(
            &definitions,
            &workflow_id,
            &workflow_version,
            &schema_version,
            &workflow_content_hash,
        )?;
        let root_hash = compute_root_hash(&BundleHashInput {
            bundle_id: &bundle_id,
            bundle_version: &bundle_version,
            run_id: &run_id,
            workflow_id: &workflow_id,
            workflow_version: &workflow_version,
            schema_version: &schema_version,
            workflow_content_hash: &workflow_content_hash,
            resolved_execution_context_hash: &resolved_execution_context_hash,
            definitions: &definitions,
            execution_posture: &execution_posture,
            handlers: &handlers,
            created_at: &created_at,
            created_by: &created_by,
            sensitivity,
            redaction_required,
        });
        Ok(Self {
            bundle_id,
            bundle_version,
            run_id,
            workflow_id,
            workflow_version,
            schema_version,
            workflow_content_hash,
            resolved_execution_context_hash,
            definitions,
            execution_posture,
            handlers,
            created_at,
            created_by,
            sensitivity,
            redaction_required,
            root_hash,
        })
    }

    #[must_use]
    /// Returns the bundle ID.
    pub const fn bundle_id(&self) -> &ImmutableRunBundleId {
        &self.bundle_id
    }
    #[must_use]
    /// Returns the bundle model version.
    pub const fn bundle_version(&self) -> &ImmutableRunBundleVersion {
        &self.bundle_version
    }
    #[must_use]
    /// Returns the immutable workflow run ID.
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }
    #[must_use]
    /// Returns the immutable workflow ID.
    pub const fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }
    #[must_use]
    /// Returns the immutable workflow version.
    pub const fn workflow_version(&self) -> &WorkflowVersion {
        &self.workflow_version
    }
    #[must_use]
    /// Returns the manifest schema version.
    pub const fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }
    #[must_use]
    /// Returns the workflow definition content hash.
    pub const fn workflow_content_hash(&self) -> &SpecContentHash {
        &self.workflow_content_hash
    }
    #[must_use]
    /// Returns the resolved execution-context commitment.
    pub const fn resolved_execution_context_hash(&self) -> &SpecContentHash {
        &self.resolved_execution_context_hash
    }
    #[must_use]
    /// Returns the deterministic bundle root hash.
    pub const fn root_hash(&self) -> &SpecContentHash {
        &self.root_hash
    }
    #[must_use]
    /// Returns read-only canonical definition references.
    pub fn definitions(&self) -> &[ImmutableRunBundleDefinitionReference] {
        &self.definitions
    }
    #[must_use]
    /// Returns read-only handler posture references.
    pub fn handlers(&self) -> &[ImmutableRunBundleHandlerReference] {
        &self.handlers
    }
    #[must_use]
    /// Returns the bounded execution posture.
    pub const fn execution_posture(&self) -> &ImmutableRunBundleExecutionPosture {
        &self.execution_posture
    }
    #[must_use]
    /// Returns when the bundle manifest was created.
    pub const fn created_at(&self) -> &Timestamp {
        &self.created_at
    }
    #[must_use]
    /// Returns the actor that created the bundle manifest.
    pub const fn created_by(&self) -> &ActorId {
        &self.created_by
    }
    #[must_use]
    /// Returns the bundle sensitivity posture.
    pub const fn sensitivity(&self) -> ImmutableRunBundleSensitivity {
        self.sensitivity
    }
    #[must_use]
    /// Returns whether downstream handling requires redaction.
    pub const fn redaction_required(&self) -> bool {
        self.redaction_required
    }

    /// Returns the bounded identity stored with a bundle-backed workflow run.
    #[must_use]
    pub fn run_binding(&self) -> ImmutableRunBundleBinding {
        ImmutableRunBundleBinding {
            bundle_id: self.bundle_id.clone(),
            bundle_version: self.bundle_version.clone(),
            root_hash: self.root_hash.clone(),
        }
    }
}

impl<'de> Deserialize<'de> for ImmutableRunBundleManifest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ImmutableRunBundleManifestWire::deserialize(deserializer)?;
        let expected_root = wire.root_hash.clone();
        let manifest = Self::new(
            wire.bundle_id,
            wire.bundle_version,
            wire.run_id,
            wire.workflow_id,
            wire.workflow_version,
            wire.schema_version,
            wire.workflow_content_hash,
            wire.resolved_execution_context_hash,
            wire.definitions,
            wire.execution_posture,
            wire.handlers,
            wire.created_at,
            wire.created_by,
            wire.sensitivity,
            wire.redaction_required,
        )
        .map_err(|_| serde::de::Error::custom("invalid immutable run bundle manifest"))?;
        if manifest.root_hash != expected_root {
            return Err(serde::de::Error::custom(
                "invalid immutable run bundle root hash",
            ));
        }
        Ok(manifest)
    }
}

impl fmt::Debug for ImmutableRunBundleManifest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleManifest")
            .field("bundle_id", &"[REDACTED]")
            .field("run_identity", &"[REDACTED]")
            .field("definition_count", &self.definitions.len())
            .field("handler_count", &self.handlers.len())
            .field("sensitivity", &self.sensitivity)
            .field("redaction_required", &self.redaction_required)
            .field("root_hash", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

struct BundleHashInput<'a> {
    bundle_id: &'a ImmutableRunBundleId,
    bundle_version: &'a ImmutableRunBundleVersion,
    run_id: &'a WorkflowRunId,
    workflow_id: &'a WorkflowId,
    workflow_version: &'a WorkflowVersion,
    schema_version: &'a SchemaVersion,
    workflow_content_hash: &'a SpecContentHash,
    resolved_execution_context_hash: &'a SpecContentHash,
    definitions: &'a [ImmutableRunBundleDefinitionReference],
    execution_posture: &'a ImmutableRunBundleExecutionPosture,
    handlers: &'a [ImmutableRunBundleHandlerReference],
    created_at: &'a Timestamp,
    created_by: &'a ActorId,
    sensitivity: ImmutableRunBundleSensitivity,
    redaction_required: bool,
}

#[allow(clippy::too_many_lines)]
fn compute_root_hash(input: &BundleHashInput<'_>) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", BUNDLE_DOMAIN);
    hash_field(&mut hasher, "bundle_id", input.bundle_id.as_str());
    hash_field(&mut hasher, "bundle_version", input.bundle_version.as_str());
    hash_field(&mut hasher, "run_id", input.run_id.as_str());
    hash_field(&mut hasher, "workflow_id", input.workflow_id.as_str());
    hash_field(
        &mut hasher,
        "workflow_version",
        input.workflow_version.as_str(),
    );
    hash_field(&mut hasher, "schema_version", input.schema_version.as_str());
    hash_field(
        &mut hasher,
        "workflow_hash",
        input.workflow_content_hash.as_str(),
    );
    hash_field(
        &mut hasher,
        "context_hash",
        input.resolved_execution_context_hash.as_str(),
    );
    for reference in input.definitions {
        hash_field(&mut hasher, "definition_kind", reference.kind.label());
        hash_field(&mut hasher, "definition_id", &reference.definition_id);
        hash_field(
            &mut hasher,
            "definition_version",
            reference.definition_version.as_deref().unwrap_or("none"),
        );
        hash_field(
            &mut hasher,
            "definition_schema",
            reference.schema_version.as_str(),
        );
        hash_field(
            &mut hasher,
            "definition_hash",
            reference.content_hash.as_str(),
        );
        hash_field(
            &mut hasher,
            "definition_step",
            reference.step_id.as_ref().map_or("none", StepId::as_str),
        );
    }
    let mut checkpoints = input
        .execution_posture
        .required_checkpoint_step_ids
        .iter()
        .map(StepId::as_str)
        .collect::<Vec<_>>();
    checkpoints.sort_unstable();
    for checkpoint in checkpoints {
        hash_field(&mut hasher, "checkpoint", checkpoint);
    }
    hash_field(
        &mut hasher,
        "hook_posture",
        reference_posture_label(input.execution_posture.hook_inputs),
    );
    hash_field(
        &mut hasher,
        "side_effect_posture",
        reference_posture_label(input.execution_posture.side_effect_inputs),
    );
    hash_field(
        &mut hasher,
        "report_posture",
        reference_posture_label(input.execution_posture.report_artifact_policy),
    );
    for handler in input.handlers {
        hash_field(&mut hasher, "handler_skill", handler.skill_id.as_str());
        hash_field(
            &mut hasher,
            "handler_version",
            handler.skill_version.as_str(),
        );
        hash_field(
            &mut hasher,
            "handler_posture",
            handler_posture_label(handler.posture),
        );
    }
    hash_field(&mut hasher, "created_at", &input.created_at.to_string());
    hash_field(&mut hasher, "created_by", input.created_by.as_str());
    hash_field(
        &mut hasher,
        "sensitivity",
        sensitivity_label(input.sensitivity),
    );
    hash_field(
        &mut hasher,
        "redaction_required",
        if input.redaction_required {
            "true"
        } else {
            "false"
        },
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn validate_manifest_collections(
    definitions: &[ImmutableRunBundleDefinitionReference],
    execution_posture: &ImmutableRunBundleExecutionPosture,
    handlers: &[ImmutableRunBundleHandlerReference],
) -> Result<(), WorkflowOsError> {
    if definitions.is_empty() || definitions.len() > REFERENCE_MAX_COUNT {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.definitions.invalid_count",
            "immutable run bundle definition count is invalid",
        ));
    }
    if definitions
        .iter()
        .filter(|item| item.kind == ImmutableRunBundleDefinitionKind::Workflow)
        .count()
        != 1
    {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.definitions.workflow_count",
            "immutable run bundle requires exactly one workflow reference",
        ));
    }
    let keys = definitions
        .iter()
        .map(|item| {
            (
                item.kind,
                item.definition_id.as_str(),
                item.definition_version.as_deref(),
                item.step_id.as_ref(),
            )
        })
        .collect::<BTreeSet<_>>();
    if keys.len() != definitions.len() {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.definitions.duplicate",
            "immutable run bundle definition references must be unique",
        ));
    }
    if handlers.len() > REFERENCE_MAX_COUNT {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.handlers.too_many",
            "immutable run bundle has too many handler references",
        ));
    }
    let handler_keys = handlers
        .iter()
        .map(|handler| (&handler.skill_id, &handler.skill_version))
        .collect::<BTreeSet<_>>();
    if handler_keys.len() != handlers.len() {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.handlers.duplicate",
            "immutable run bundle handler references must be unique",
        ));
    }
    let resolved_skill_keys = definitions
        .iter()
        .filter(|item| item.kind == ImmutableRunBundleDefinitionKind::Skill)
        .map(|item| {
            (
                item.definition_id.as_str(),
                item.definition_version.as_deref(),
            )
        })
        .collect::<BTreeSet<_>>();
    let handler_skill_keys = handlers
        .iter()
        .map(|handler| {
            (
                handler.skill_id.as_str(),
                Some(handler.skill_version.as_str()),
            )
        })
        .collect::<BTreeSet<_>>();
    if resolved_skill_keys != handler_skill_keys {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.handlers.skill_mismatch",
            "immutable run bundle handlers must match resolved skills",
        ));
    }
    execution_posture.validate()
}

fn validate_workflow_reference_alignment(
    definitions: &[ImmutableRunBundleDefinitionReference],
    workflow_id: &WorkflowId,
    workflow_version: &WorkflowVersion,
    schema_version: &SchemaVersion,
    workflow_content_hash: &SpecContentHash,
) -> Result<(), WorkflowOsError> {
    let Some(reference) = definitions
        .iter()
        .find(|item| item.kind == ImmutableRunBundleDefinitionKind::Workflow)
    else {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.definitions.workflow_count",
            "immutable run bundle requires exactly one workflow reference",
        ));
    };
    if reference.definition_id != workflow_id.as_str()
        || reference.definition_version.as_deref() != Some(workflow_version.as_str())
        || &reference.schema_version != schema_version
        || &reference.content_hash != workflow_content_hash
    {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.definitions.workflow_identity_mismatch",
            "immutable run bundle workflow reference does not match run identity",
        ));
    }
    Ok(())
}

fn canonicalize_definition_references(
    definitions: &mut Vec<ImmutableRunBundleDefinitionReference>,
) {
    let mut workflows = Vec::new();
    let mut skills = Vec::new();
    let mut policies = Vec::new();
    for reference in definitions.drain(..) {
        match reference.kind {
            ImmutableRunBundleDefinitionKind::Workflow => workflows.push(reference),
            ImmutableRunBundleDefinitionKind::Skill => skills.push(reference),
            ImmutableRunBundleDefinitionKind::Policy => policies.push(reference),
        }
    }
    policies.sort_by(|left, right| {
        (&left.definition_id, &left.definition_version)
            .cmp(&(&right.definition_id, &right.definition_version))
    });
    definitions.extend(workflows);
    definitions.extend(skills);
    definitions.extend(policies);
}

fn validate_bounded_identifier(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > ID_MAX_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.'))
    {
        return Err(WorkflowOsError::validation(
            "immutable_run_bundle.reference.id_invalid",
            "immutable run bundle definition id is invalid",
        ));
    }
    Ok(())
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update((label.len() as u64).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value.as_bytes());
}

const fn reference_posture_label(value: ImmutableRunBundleReferencePosture) -> &'static str {
    match value {
        ImmutableRunBundleReferencePosture::CommittedReference => "committed_reference",
        ImmutableRunBundleReferencePosture::PresentNotPreserved => "present_not_preserved",
        ImmutableRunBundleReferencePosture::NotSupplied => "not_supplied",
        ImmutableRunBundleReferencePosture::Unsupported => "unsupported",
    }
}
const fn handler_posture_label(value: ImmutableRunBundleHandlerPosture) -> &'static str {
    match value {
        ImmutableRunBundleHandlerPosture::DeclaredOnly => "declared_only",
        ImmutableRunBundleHandlerPosture::RegisteredUnattested => "registered_unattested",
        ImmutableRunBundleHandlerPosture::MockSelected => "mock_selected",
        ImmutableRunBundleHandlerPosture::Unavailable => "unavailable",
    }
}
const fn sensitivity_label(value: ImmutableRunBundleSensitivity) -> &'static str {
    match value {
        ImmutableRunBundleSensitivity::Internal => "internal",
        ImmutableRunBundleSensitivity::Confidential => "confidential",
        ImmutableRunBundleSensitivity::Restricted => "restricted",
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    fn workflow_reference() -> ImmutableRunBundleDefinitionReference {
        ImmutableRunBundleDefinitionReference::new(
            ImmutableRunBundleDefinitionKind::Workflow,
            "workflow/main",
            Some("v1".to_owned()),
            SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            SpecContentHash::from_text("workflow"),
            None,
        )
        .expect("workflow reference")
    }

    fn skill_reference() -> ImmutableRunBundleDefinitionReference {
        ImmutableRunBundleDefinitionReference::new(
            ImmutableRunBundleDefinitionKind::Skill,
            "skill/check",
            Some("v1".to_owned()),
            SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            SpecContentHash::from_text("skill"),
            Some(StepId::new("check").expect("step")),
        )
        .expect("skill reference")
    }

    fn manifest() -> ImmutableRunBundleManifest {
        ImmutableRunBundleManifest::new(
            ImmutableRunBundleId::new("bundle/run-1").expect("bundle id"),
            ImmutableRunBundleVersion::new("v1").expect("bundle version"),
            WorkflowRunId::new("run-1").expect("run id"),
            WorkflowId::new("workflow/main").expect("workflow id"),
            WorkflowVersion::new("v1").expect("workflow version"),
            SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            SpecContentHash::from_text("workflow"),
            SpecContentHash::from_text("resolved context"),
            vec![workflow_reference(), skill_reference()],
            ImmutableRunBundleExecutionPosture::new(
                vec![StepId::new("check").expect("step")],
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::NotSupplied,
                ImmutableRunBundleReferencePosture::CommittedReference,
            )
            .expect("execution posture"),
            vec![ImmutableRunBundleHandlerReference {
                skill_id: SkillId::new("skill/check").expect("skill"),
                skill_version: SkillVersion::new("v1").expect("skill version"),
                posture: ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            }],
            Timestamp::parse_rfc3339("2026-07-12T12:00:00Z").expect("timestamp"),
            ActorId::new("system/kernel").expect("actor"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect("manifest")
    }

    #[test]
    fn valid_manifest_has_deterministic_root() {
        assert_eq!(manifest().root_hash(), manifest().root_hash());
    }

    #[test]
    fn manifest_root_changes_with_definition_content() {
        let original = manifest();
        let mut changed = manifest();
        changed.definitions[1].content_hash = SpecContentHash::from_text("changed skill");
        changed.root_hash = compute_root_hash(&BundleHashInput {
            bundle_id: &changed.bundle_id,
            bundle_version: &changed.bundle_version,
            run_id: &changed.run_id,
            workflow_id: &changed.workflow_id,
            workflow_version: &changed.workflow_version,
            schema_version: &changed.schema_version,
            workflow_content_hash: &changed.workflow_content_hash,
            resolved_execution_context_hash: &changed.resolved_execution_context_hash,
            definitions: &changed.definitions,
            execution_posture: &changed.execution_posture,
            handlers: &changed.handlers,
            created_at: &changed.created_at,
            created_by: &changed.created_by,
            sensitivity: changed.sensitivity,
            redaction_required: changed.redaction_required,
        });
        assert_ne!(original.root_hash(), changed.root_hash());
    }

    #[test]
    fn serde_round_trip_validates_root() {
        let value = serde_json::to_string(&manifest()).expect("serialize");
        let decoded: ImmutableRunBundleManifest = serde_json::from_str(&value).expect("decode");
        assert_eq!(decoded, manifest());
    }

    #[test]
    fn tampered_serialized_root_fails_closed() {
        let mut value = serde_json::to_value(manifest()).expect("serialize");
        value["root_hash"] =
            serde_json::Value::String(SpecContentHash::from_text("tampered").to_string());
        let error = serde_json::from_value::<ImmutableRunBundleManifest>(value)
            .expect_err("tampered root rejected");
        assert!(!error.to_string().contains("tampered"));
    }

    #[test]
    fn exactly_one_workflow_reference_is_required() {
        let mut fixture = manifest();
        fixture.definitions.remove(0);
        assert_eq!(
            validate_manifest_collections(
                &fixture.definitions,
                &fixture.execution_posture,
                &fixture.handlers
            )
            .expect_err("workflow missing")
            .code(),
            "immutable_run_bundle.definitions.workflow_count"
        );
    }

    #[test]
    fn workflow_reference_must_match_manifest_identity() {
        let fixture = manifest();
        let error = validate_workflow_reference_alignment(
            &fixture.definitions,
            &WorkflowId::new("workflow/other").expect("workflow id"),
            &fixture.workflow_version,
            &fixture.schema_version,
            &fixture.workflow_content_hash,
        )
        .expect_err("workflow mismatch");
        assert_eq!(
            error.code(),
            "immutable_run_bundle.definitions.workflow_identity_mismatch"
        );
    }

    #[test]
    fn duplicate_definition_reference_is_rejected() {
        let mut fixture = manifest();
        fixture.definitions.push(skill_reference());
        assert_eq!(
            validate_manifest_collections(
                &fixture.definitions,
                &fixture.execution_posture,
                &fixture.handlers
            )
            .expect_err("duplicate rejected")
            .code(),
            "immutable_run_bundle.definitions.duplicate"
        );
    }

    #[test]
    fn definition_canonicalization_keeps_skills_ordered_and_sorts_policies() {
        let policy = |id: &str| {
            ImmutableRunBundleDefinitionReference::new(
                ImmutableRunBundleDefinitionKind::Policy,
                id,
                None,
                SchemaVersion::new("workflowos.dev/v0").expect("schema"),
                SpecContentHash::from_text(id),
                None,
            )
            .expect("policy")
        };
        let mut references = vec![
            policy("policy/z"),
            skill_reference(),
            workflow_reference(),
            policy("policy/a"),
        ];
        canonicalize_definition_references(&mut references);
        assert_eq!(
            references[0].kind(),
            ImmutableRunBundleDefinitionKind::Workflow
        );
        assert_eq!(
            references[1].kind(),
            ImmutableRunBundleDefinitionKind::Skill
        );
        assert_eq!(references[2].definition_id(), "policy/a");
        assert_eq!(references[3].definition_id(), "policy/z");
    }

    #[test]
    fn skill_reference_requires_step_and_policy_forbids_it() {
        assert!(ImmutableRunBundleDefinitionReference::new(
            ImmutableRunBundleDefinitionKind::Skill,
            "skill/check",
            Some("v1".to_owned()),
            SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            SpecContentHash::from_text("skill"),
            None,
        )
        .is_err());
        assert!(ImmutableRunBundleDefinitionReference::new(
            ImmutableRunBundleDefinitionKind::Policy,
            "policy/local",
            None,
            SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            SpecContentHash::from_text("policy"),
            Some(StepId::new("check").expect("step")),
        )
        .is_err());
    }

    #[test]
    fn duplicate_checkpoint_reference_is_rejected() {
        let mut fixture = manifest();
        fixture
            .execution_posture
            .required_checkpoint_step_ids
            .push(StepId::new("check").expect("step"));
        assert_eq!(
            fixture
                .execution_posture
                .validate()
                .expect_err("duplicate")
                .code(),
            "immutable_run_bundle.execution_posture.duplicate_checkpoint"
        );
    }

    #[test]
    fn invalid_serialized_execution_posture_fails_closed() {
        let value = serde_json::json!({
            "required_checkpoint_step_ids": ["check", "check"],
            "hook_inputs": "not_supplied",
            "side_effect_inputs": "not_supplied",
            "report_artifact_policy": "not_supplied"
        });
        let error = serde_json::from_value::<ImmutableRunBundleExecutionPosture>(value)
            .expect_err("invalid posture");
        assert_eq!(
            error.to_string(),
            "invalid immutable run bundle execution posture"
        );
    }

    #[test]
    fn duplicate_handler_reference_is_rejected() {
        let mut fixture = manifest();
        fixture.handlers.push(fixture.handlers[0].clone());
        assert_eq!(
            validate_manifest_collections(
                &fixture.definitions,
                &fixture.execution_posture,
                &fixture.handlers
            )
            .expect_err("duplicate handler")
            .code(),
            "immutable_run_bundle.handlers.duplicate"
        );
    }

    #[test]
    fn handlers_must_match_resolved_skills() {
        let mut fixture = manifest();
        fixture.handlers.clear();
        assert_eq!(
            validate_manifest_collections(
                &fixture.definitions,
                &fixture.execution_posture,
                &fixture.handlers
            )
            .expect_err("handler mismatch")
            .code(),
            "immutable_run_bundle.handlers.skill_mismatch"
        );
    }

    #[test]
    fn debug_redacts_identity_and_hashes() {
        let debug = format!("{:?}", manifest());
        assert!(!debug.contains("bundle/run-1"));
        assert!(!debug.contains("workflow/main"));
        assert!(!debug.contains(manifest().root_hash().as_str()));
        assert!(debug.contains("definition_count"));

        let definition_debug = format!("{:?}", skill_reference());
        assert!(!definition_debug.contains("skill/check"));
        assert!(!definition_debug.contains("check"));
        let handler_debug = format!("{:?}", manifest().handlers()[0]);
        assert!(!handler_debug.contains("skill/check"));
    }

    #[test]
    fn invalid_ids_fail_without_echoing_values() {
        let secret = "token=secret-value";
        let error = ImmutableRunBundleId::new(secret).expect_err("invalid id");
        assert!(!error.to_string().contains(secret));

        let reference_error = ImmutableRunBundleDefinitionReference::new(
            ImmutableRunBundleDefinitionKind::Policy,
            secret,
            None,
            SchemaVersion::new("workflowos.dev/v0").expect("schema"),
            SpecContentHash::from_text("policy"),
            None,
        )
        .expect_err("secret-like reference rejected");
        assert!(!reference_error.to_string().contains(secret));
    }

    #[test]
    fn handler_postures_are_explicitly_representable() {
        for posture in [
            ImmutableRunBundleHandlerPosture::DeclaredOnly,
            ImmutableRunBundleHandlerPosture::RegisteredUnattested,
            ImmutableRunBundleHandlerPosture::MockSelected,
            ImmutableRunBundleHandlerPosture::Unavailable,
        ] {
            assert!(!handler_posture_label(posture).is_empty());
        }
    }

    #[test]
    fn serialization_has_no_raw_payload_fields() {
        let serialized = serde_json::to_string(&manifest()).expect("serialize");
        for forbidden in [
            "raw_yaml",
            "provider_payload",
            "command_output",
            "credential",
        ] {
            assert!(!serialized.contains(forbidden));
        }
    }
}
