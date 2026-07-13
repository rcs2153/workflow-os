use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    parse_policy_spec_yaml, parse_skill_spec_yaml, parse_workflow_spec_yaml,
    ImmutableRunBundleDefinitionKind, ImmutableRunBundleDefinitionReference,
    ImmutableRunBundleSensitivity, ImmutableRunBundleVersion, PolicySpecDocument, SchemaVersion,
    SkillDefinition, SpecContentHash, StepId, WorkflowDefinition, WorkflowOsError,
};

const DEFINITION_RECORD_DOMAIN: &str = "workflow-os/immutable-run-bundle-definition/v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Versioned canonical model encoding used by immutable definition records.
pub enum ImmutableRunBundleDefinitionEncoding {
    /// Deterministic serde JSON over the validated v1 typed definition model.
    CanonicalJsonV1,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "definition", rename_all = "snake_case")]
/// Canonical validated definition content retained by an immutable run bundle record.
pub enum ImmutableRunBundleCanonicalDefinition {
    /// A canonical validated workflow definition.
    Workflow(WorkflowDefinition),
    /// A canonical validated skill definition.
    Skill(SkillDefinition),
    /// A canonical validated policy definition.
    Policy(PolicySpecDocument),
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "definition", rename_all = "snake_case")]
enum ImmutableRunBundleCanonicalDefinitionWire {
    Workflow(WorkflowDefinition),
    Skill(SkillDefinition),
    Policy(PolicySpecDocument),
}

impl ImmutableRunBundleCanonicalDefinitionWire {
    fn into_definition(self) -> ImmutableRunBundleCanonicalDefinition {
        match self {
            Self::Workflow(value) => ImmutableRunBundleCanonicalDefinition::Workflow(value),
            Self::Skill(value) => ImmutableRunBundleCanonicalDefinition::Skill(value),
            Self::Policy(value) => ImmutableRunBundleCanonicalDefinition::Policy(value),
        }
    }
}

impl ImmutableRunBundleCanonicalDefinition {
    #[must_use]
    /// Returns the definition kind.
    pub const fn kind(&self) -> ImmutableRunBundleDefinitionKind {
        match self {
            Self::Workflow(_) => ImmutableRunBundleDefinitionKind::Workflow,
            Self::Skill(_) => ImmutableRunBundleDefinitionKind::Skill,
            Self::Policy(_) => ImmutableRunBundleDefinitionKind::Policy,
        }
    }

    #[must_use]
    /// Returns the stable definition ID.
    pub fn definition_id(&self) -> &str {
        match self {
            Self::Workflow(value) => value.id.as_str(),
            Self::Skill(value) => value.id.as_str(),
            Self::Policy(value) => value.id.as_str(),
        }
    }

    #[must_use]
    /// Returns the definition version when the definition kind models one.
    pub fn definition_version(&self) -> Option<&str> {
        match self {
            Self::Workflow(value) => Some(value.version.as_str()),
            Self::Skill(value) => Some(value.version.as_str()),
            Self::Policy(_) => None,
        }
    }

    #[must_use]
    /// Returns the definition schema version.
    pub const fn schema_version(&self) -> &SchemaVersion {
        match self {
            Self::Workflow(value) => &value.schema_version,
            Self::Skill(value) => &value.schema_version,
            Self::Policy(value) => &value.schema_version,
        }
    }

    #[must_use]
    /// Returns the canonical workflow definition when this is a workflow record.
    pub const fn as_workflow(&self) -> Option<&WorkflowDefinition> {
        match self {
            Self::Workflow(value) => Some(value),
            Self::Skill(_) | Self::Policy(_) => None,
        }
    }

    #[must_use]
    /// Returns the canonical skill definition when this is a skill record.
    pub const fn as_skill(&self) -> Option<&SkillDefinition> {
        match self {
            Self::Skill(value) => Some(value),
            Self::Workflow(_) | Self::Policy(_) => None,
        }
    }

    #[must_use]
    /// Returns the canonical policy definition when this is a policy record.
    pub const fn as_policy(&self) -> Option<&PolicySpecDocument> {
        match self {
            Self::Policy(value) => Some(value),
            Self::Workflow(_) | Self::Skill(_) => None,
        }
    }
}

impl fmt::Debug for ImmutableRunBundleCanonicalDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleCanonicalDefinition")
            .field("kind", &self.kind())
            .field("definition_id", &"[REDACTED]")
            .field(
                "definition_version_present",
                &self.definition_version().is_some(),
            )
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
/// Canonical validated definition record referenced by an immutable run bundle.
pub struct ImmutableRunBundleDefinitionRecord {
    record_version: ImmutableRunBundleVersion,
    encoding: ImmutableRunBundleDefinitionEncoding,
    source_content_hash: SpecContentHash,
    canonical_definition: ImmutableRunBundleCanonicalDefinition,
    sensitivity: ImmutableRunBundleSensitivity,
    redaction_required: bool,
    canonical_record_hash: SpecContentHash,
}

#[derive(Deserialize)]
struct ImmutableRunBundleDefinitionRecordWire {
    record_version: ImmutableRunBundleVersion,
    encoding: ImmutableRunBundleDefinitionEncoding,
    source_content_hash: SpecContentHash,
    canonical_definition: ImmutableRunBundleCanonicalDefinitionWire,
    sensitivity: ImmutableRunBundleSensitivity,
    redaction_required: bool,
    canonical_record_hash: SpecContentHash,
}

impl ImmutableRunBundleDefinitionRecord {
    /// Creates a canonical workflow definition record.
    ///
    /// # Errors
    ///
    /// Returns a bounded error when the workflow is not parser-valid, contains source-derived
    /// fields, or its parser content hash does not match the supplied source hash.
    pub fn from_workflow(
        record_version: ImmutableRunBundleVersion,
        definition: WorkflowDefinition,
        source_content_hash: SpecContentHash,
        sensitivity: ImmutableRunBundleSensitivity,
        redaction_required: bool,
    ) -> Result<Self, WorkflowOsError> {
        if definition.spec_content_hash.as_ref() != Some(&source_content_hash) {
            return Err(record_validation_error(
                "immutable_run_bundle.definition_record.workflow_hash_mismatch",
                "workflow definition source hash does not match parsed content hash",
            ));
        }
        let canonical = canonicalize_workflow(definition)?;
        Self::build(
            record_version,
            source_content_hash,
            ImmutableRunBundleCanonicalDefinition::Workflow(canonical),
            sensitivity,
            redaction_required,
        )
    }

    /// Creates a canonical skill definition record.
    ///
    /// # Errors
    ///
    /// Returns a bounded error when the skill cannot be serialized and reparsed through the
    /// validated project parser.
    pub fn from_skill(
        record_version: ImmutableRunBundleVersion,
        definition: SkillDefinition,
        source_content_hash: SpecContentHash,
        sensitivity: ImmutableRunBundleSensitivity,
        redaction_required: bool,
    ) -> Result<Self, WorkflowOsError> {
        let canonical = canonicalize_skill(definition)?;
        Self::build(
            record_version,
            source_content_hash,
            ImmutableRunBundleCanonicalDefinition::Skill(canonical),
            sensitivity,
            redaction_required,
        )
    }

    /// Creates a canonical policy definition record.
    ///
    /// # Errors
    ///
    /// Returns a bounded error when the policy cannot be serialized and reparsed through the
    /// validated project parser.
    pub fn from_policy(
        record_version: ImmutableRunBundleVersion,
        definition: &PolicySpecDocument,
        source_content_hash: SpecContentHash,
        sensitivity: ImmutableRunBundleSensitivity,
        redaction_required: bool,
    ) -> Result<Self, WorkflowOsError> {
        let canonical = canonicalize_policy(definition)?;
        Self::build(
            record_version,
            source_content_hash,
            ImmutableRunBundleCanonicalDefinition::Policy(canonical),
            sensitivity,
            redaction_required,
        )
    }

    fn build(
        record_version: ImmutableRunBundleVersion,
        source_content_hash: SpecContentHash,
        canonical_definition: ImmutableRunBundleCanonicalDefinition,
        sensitivity: ImmutableRunBundleSensitivity,
        redaction_required: bool,
    ) -> Result<Self, WorkflowOsError> {
        let canonical_bytes = canonical_definition_bytes(&canonical_definition)?;
        let encoding = ImmutableRunBundleDefinitionEncoding::CanonicalJsonV1;
        let canonical_record_hash = compute_definition_record_hash(
            &record_version,
            encoding,
            &source_content_hash,
            &canonical_definition,
            &canonical_bytes,
            sensitivity,
            redaction_required,
        );
        Ok(Self {
            record_version,
            encoding,
            source_content_hash,
            canonical_definition,
            sensitivity,
            redaction_required,
            canonical_record_hash,
        })
    }

    #[must_use]
    /// Returns the record model version.
    pub const fn record_version(&self) -> &ImmutableRunBundleVersion {
        &self.record_version
    }

    #[must_use]
    /// Returns the explicit canonical model encoding.
    pub const fn encoding(&self) -> ImmutableRunBundleDefinitionEncoding {
        self.encoding
    }

    #[must_use]
    /// Returns the definition kind.
    pub const fn kind(&self) -> ImmutableRunBundleDefinitionKind {
        self.canonical_definition.kind()
    }

    #[must_use]
    /// Returns the stable definition ID.
    pub fn definition_id(&self) -> &str {
        self.canonical_definition.definition_id()
    }

    #[must_use]
    /// Returns the definition version when modeled.
    pub fn definition_version(&self) -> Option<&str> {
        self.canonical_definition.definition_version()
    }

    #[must_use]
    /// Returns the definition schema version.
    pub const fn schema_version(&self) -> &SchemaVersion {
        self.canonical_definition.schema_version()
    }

    #[must_use]
    /// Returns the source canonical-content hash supplied by the validated loader boundary.
    pub const fn source_content_hash(&self) -> &SpecContentHash {
        &self.source_content_hash
    }

    #[must_use]
    /// Returns the canonical validated definition content.
    pub const fn canonical_definition(&self) -> &ImmutableRunBundleCanonicalDefinition {
        &self.canonical_definition
    }

    #[must_use]
    /// Returns the record sensitivity posture.
    pub const fn sensitivity(&self) -> ImmutableRunBundleSensitivity {
        self.sensitivity
    }

    #[must_use]
    /// Returns whether downstream handling requires redaction.
    pub const fn redaction_required(&self) -> bool {
        self.redaction_required
    }

    #[must_use]
    /// Returns the deterministic canonical record hash.
    pub const fn canonical_record_hash(&self) -> &SpecContentHash {
        &self.canonical_record_hash
    }

    /// Builds a manifest reference to this record.
    ///
    /// # Errors
    ///
    /// Returns a bounded error when the supplied step posture is incompatible with the record
    /// kind.
    pub fn definition_reference(
        &self,
        step_id: Option<StepId>,
    ) -> Result<ImmutableRunBundleDefinitionReference, WorkflowOsError> {
        ImmutableRunBundleDefinitionReference::new(
            self.kind(),
            self.definition_id(),
            self.definition_version().map(str::to_owned),
            self.schema_version().clone(),
            self.source_content_hash.clone(),
            step_id,
        )
    }
}

impl<'de> Deserialize<'de> for ImmutableRunBundleDefinitionRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ImmutableRunBundleDefinitionRecordWire::deserialize(deserializer)?;
        let supplied_definition = wire.canonical_definition.into_definition();
        let supplied_hash = wire.canonical_record_hash;
        let canonical_definition = canonicalize_definition(supplied_definition.clone())
            .map_err(|_| serde::de::Error::custom("invalid immutable run bundle definition"))?;
        if supplied_definition != canonical_definition {
            return Err(serde::de::Error::custom(
                "non-canonical immutable run bundle definition",
            ));
        }
        let record = Self::build(
            wire.record_version,
            wire.source_content_hash,
            canonical_definition,
            wire.sensitivity,
            wire.redaction_required,
        )
        .map_err(|_| serde::de::Error::custom("invalid immutable run bundle definition record"))?;
        if record.encoding != wire.encoding {
            return Err(serde::de::Error::custom(
                "unsupported immutable run bundle definition encoding",
            ));
        }
        if record.canonical_record_hash != supplied_hash {
            return Err(serde::de::Error::custom(
                "invalid immutable run bundle definition record hash",
            ));
        }
        Ok(record)
    }
}

impl fmt::Debug for ImmutableRunBundleDefinitionRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleDefinitionRecord")
            .field("record_version", &self.record_version)
            .field("encoding", &self.encoding)
            .field("kind", &self.kind())
            .field("definition_identity", &"[REDACTED]")
            .field("source_content_hash", &"[REDACTED]")
            .field("canonical_record_hash", &"[REDACTED]")
            .field("sensitivity", &self.sensitivity)
            .field("redaction_required", &self.redaction_required)
            .finish_non_exhaustive()
    }
}

fn canonicalize_definition(
    definition: ImmutableRunBundleCanonicalDefinition,
) -> Result<ImmutableRunBundleCanonicalDefinition, WorkflowOsError> {
    match definition {
        ImmutableRunBundleCanonicalDefinition::Workflow(value) => {
            canonicalize_workflow(value).map(ImmutableRunBundleCanonicalDefinition::Workflow)
        }
        ImmutableRunBundleCanonicalDefinition::Skill(value) => {
            canonicalize_skill(value).map(ImmutableRunBundleCanonicalDefinition::Skill)
        }
        ImmutableRunBundleCanonicalDefinition::Policy(value) => {
            canonicalize_policy(&value).map(ImmutableRunBundleCanonicalDefinition::Policy)
        }
    }
}

fn canonicalize_workflow(
    mut definition: WorkflowDefinition,
) -> Result<WorkflowDefinition, WorkflowOsError> {
    clear_workflow_derived_fields(&mut definition);
    let encoded = serialize_definition_for_validation(&definition)?;
    let mut canonical = parse_workflow_spec_yaml(&encoded).map_err(|_| {
        record_validation_error(
            "immutable_run_bundle.definition_record.workflow_invalid",
            "workflow definition is not valid canonical model content",
        )
    })?;
    clear_workflow_derived_fields(&mut canonical);
    Ok(canonical)
}

fn clear_workflow_derived_fields(definition: &mut WorkflowDefinition) {
    definition.spec_content_hash = None;
    definition.source_location = None;
    for step in &mut definition.steps {
        step.source_location = None;
    }
}

fn canonicalize_skill(mut definition: SkillDefinition) -> Result<SkillDefinition, WorkflowOsError> {
    definition.source_location = None;
    let encoded = serialize_definition_for_validation(&definition)?;
    let mut canonical = parse_skill_spec_yaml(&encoded).map_err(|_| {
        record_validation_error(
            "immutable_run_bundle.definition_record.skill_invalid",
            "skill definition is not valid canonical model content",
        )
    })?;
    canonical.source_location = None;
    Ok(canonical)
}

fn canonicalize_policy(
    definition: &PolicySpecDocument,
) -> Result<PolicySpecDocument, WorkflowOsError> {
    let encoded = serialize_definition_for_validation(&definition)?;
    parse_policy_spec_yaml(&encoded).map_err(|_| {
        record_validation_error(
            "immutable_run_bundle.definition_record.policy_invalid",
            "policy definition is not valid canonical model content",
        )
    })
}

fn serialize_definition_for_validation<T: Serialize>(
    definition: &T,
) -> Result<String, WorkflowOsError> {
    serde_json::to_string(definition).map_err(|_| {
        record_validation_error(
            "immutable_run_bundle.definition_record.serialization_failed",
            "definition could not be canonicalized",
        )
    })
}

fn canonical_definition_bytes(
    definition: &ImmutableRunBundleCanonicalDefinition,
) -> Result<Vec<u8>, WorkflowOsError> {
    serde_json::to_vec(definition).map_err(|_| {
        record_validation_error(
            "immutable_run_bundle.definition_record.serialization_failed",
            "definition could not be canonicalized",
        )
    })
}

fn compute_definition_record_hash(
    record_version: &ImmutableRunBundleVersion,
    encoding: ImmutableRunBundleDefinitionEncoding,
    source_content_hash: &SpecContentHash,
    definition: &ImmutableRunBundleCanonicalDefinition,
    canonical_bytes: &[u8],
    sensitivity: ImmutableRunBundleSensitivity,
    redaction_required: bool,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_bytes(&mut hasher, b"domain", DEFINITION_RECORD_DOMAIN.as_bytes());
    hash_bytes(
        &mut hasher,
        b"record_version",
        record_version.as_str().as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"encoding",
        definition_encoding_label(encoding).as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"kind",
        definition_kind_label(definition.kind()).as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"definition_id",
        definition.definition_id().as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"definition_version",
        definition.definition_version().unwrap_or("none").as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"schema_version",
        definition.schema_version().as_str().as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"source_content_hash",
        source_content_hash.as_str().as_bytes(),
    );
    hash_bytes(&mut hasher, b"canonical_definition", canonical_bytes);
    hash_bytes(
        &mut hasher,
        b"sensitivity",
        sensitivity_label(sensitivity).as_bytes(),
    );
    hash_bytes(
        &mut hasher,
        b"redaction_required",
        if redaction_required {
            b"true"
        } else {
            b"false"
        },
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn hash_bytes(hasher: &mut Sha256, label: &[u8], value: &[u8]) {
    hasher.update((label.len() as u64).to_be_bytes());
    hasher.update(label);
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

const fn definition_kind_label(kind: ImmutableRunBundleDefinitionKind) -> &'static str {
    match kind {
        ImmutableRunBundleDefinitionKind::Workflow => "workflow",
        ImmutableRunBundleDefinitionKind::Skill => "skill",
        ImmutableRunBundleDefinitionKind::Policy => "policy",
    }
}

const fn definition_encoding_label(encoding: ImmutableRunBundleDefinitionEncoding) -> &'static str {
    match encoding {
        ImmutableRunBundleDefinitionEncoding::CanonicalJsonV1 => "canonical_json_v1",
    }
}

const fn sensitivity_label(sensitivity: ImmutableRunBundleSensitivity) -> &'static str {
    match sensitivity {
        ImmutableRunBundleSensitivity::Internal => "internal",
        ImmutableRunBundleSensitivity::Confidential => "confidential",
        ImmutableRunBundleSensitivity::Restricted => "restricted",
    }
}

fn record_validation_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::canonical_yaml_content_hash;

    const WORKFLOW_YAML: &str = r"
schema_version: workflowos.dev/v0
id: workflow/main
version: v1
name: Main workflow
steps:
  - id: check
    skill_ref:
      id: skill/check
      version: v1
";

    const SKILL_YAML: &str = r"
schema_version: workflowos.dev/v0
id: skill/check
version: v1
name: Check skill
";

    const POLICY_YAML: &str = r"
schema_version: workflowos.dev/v0
id: policy/local
name: Local policy
rules:
  - id: allow-local
    effect: allow_local_skill
";

    fn workflow_record() -> ImmutableRunBundleDefinitionRecord {
        let definition = parse_workflow_spec_yaml(WORKFLOW_YAML).expect("workflow");
        ImmutableRunBundleDefinitionRecord::from_workflow(
            ImmutableRunBundleVersion::new("v1").expect("version"),
            definition,
            canonical_yaml_content_hash(WORKFLOW_YAML).expect("hash"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect("record")
    }

    fn skill_record() -> ImmutableRunBundleDefinitionRecord {
        ImmutableRunBundleDefinitionRecord::from_skill(
            ImmutableRunBundleVersion::new("v1").expect("version"),
            parse_skill_spec_yaml(SKILL_YAML).expect("skill"),
            canonical_yaml_content_hash(SKILL_YAML).expect("hash"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect("record")
    }

    fn policy_record() -> ImmutableRunBundleDefinitionRecord {
        ImmutableRunBundleDefinitionRecord::from_policy(
            ImmutableRunBundleVersion::new("v1").expect("version"),
            &parse_policy_spec_yaml(POLICY_YAML).expect("policy"),
            canonical_yaml_content_hash(POLICY_YAML).expect("hash"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect("record")
    }

    #[test]
    fn all_definition_kinds_construct_and_are_inspectable() {
        let workflow = workflow_record();
        let skill = skill_record();
        let policy = policy_record();
        assert_eq!(workflow.kind(), ImmutableRunBundleDefinitionKind::Workflow);
        assert_eq!(skill.kind(), ImmutableRunBundleDefinitionKind::Skill);
        assert_eq!(policy.kind(), ImmutableRunBundleDefinitionKind::Policy);
        assert!(workflow.canonical_definition().as_workflow().is_some());
        assert!(skill.canonical_definition().as_skill().is_some());
        assert!(policy.canonical_definition().as_policy().is_some());
        assert_eq!(
            workflow.encoding(),
            ImmutableRunBundleDefinitionEncoding::CanonicalJsonV1
        );
    }

    #[test]
    fn canonical_record_hash_is_deterministic_and_content_sensitive() {
        let first = workflow_record();
        let second = workflow_record();
        assert_eq!(
            first.canonical_record_hash(),
            second.canonical_record_hash()
        );

        let changed_yaml = WORKFLOW_YAML.replace("Main workflow", "Changed workflow");
        let changed = ImmutableRunBundleDefinitionRecord::from_workflow(
            ImmutableRunBundleVersion::new("v1").expect("version"),
            parse_workflow_spec_yaml(&changed_yaml).expect("workflow"),
            canonical_yaml_content_hash(&changed_yaml).expect("hash"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect("record");
        assert_ne!(
            first.canonical_record_hash(),
            changed.canonical_record_hash()
        );
    }

    #[test]
    fn workflow_source_hash_must_match_parser_hash() {
        let error = ImmutableRunBundleDefinitionRecord::from_workflow(
            ImmutableRunBundleVersion::new("v1").expect("version"),
            parse_workflow_spec_yaml(WORKFLOW_YAML).expect("workflow"),
            SpecContentHash::from_text("different"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect_err("mismatch");
        assert_eq!(
            error.code(),
            "immutable_run_bundle.definition_record.workflow_hash_mismatch"
        );
    }

    #[test]
    fn manually_constructed_secret_like_content_is_rejected_without_leakage() {
        let mut definition = parse_workflow_spec_yaml(WORKFLOW_YAML).expect("workflow");
        let secret = "token:private-value";
        definition.display_name = secret.to_owned();
        let error = ImmutableRunBundleDefinitionRecord::from_workflow(
            ImmutableRunBundleVersion::new("v1").expect("version"),
            definition,
            canonical_yaml_content_hash(WORKFLOW_YAML).expect("hash"),
            ImmutableRunBundleSensitivity::Internal,
            true,
        )
        .expect_err("secret-like model content rejected");
        assert_eq!(
            error.code(),
            "immutable_run_bundle.definition_record.workflow_invalid"
        );
        assert!(!error.to_string().contains(secret));
    }

    #[test]
    fn source_locations_and_derived_hash_are_not_stored_in_canonical_workflow() {
        let record = workflow_record();
        let definition = record
            .canonical_definition()
            .as_workflow()
            .expect("workflow");
        assert!(definition.source_location.is_none());
        assert!(definition.spec_content_hash.is_none());
        assert!(definition
            .steps
            .iter()
            .all(|step| step.source_location.is_none()));
    }

    #[test]
    fn record_builds_manifest_reference_without_recreating_content() {
        let record = skill_record();
        let reference = record
            .definition_reference(Some(StepId::new("check").expect("step")))
            .expect("reference");
        assert_eq!(reference.kind(), ImmutableRunBundleDefinitionKind::Skill);
        assert_eq!(reference.content_hash(), record.source_content_hash());
    }

    #[test]
    fn serde_round_trip_revalidates_canonical_content_and_hash() {
        for record in [workflow_record(), skill_record(), policy_record()] {
            let serialized = serde_json::to_string(&record).expect("serialize");
            let decoded: ImmutableRunBundleDefinitionRecord =
                serde_json::from_str(&serialized).expect("deserialize");
            assert_eq!(decoded, record);
        }
    }

    #[test]
    fn tampered_record_hash_fails_closed() {
        let mut value = serde_json::to_value(workflow_record()).expect("serialize");
        value["canonical_record_hash"] =
            serde_json::Value::String(SpecContentHash::from_text("tampered").to_string());
        let error = serde_json::from_value::<ImmutableRunBundleDefinitionRecord>(value)
            .expect_err("tamper rejected");
        assert!(!error.to_string().contains("tampered"));
    }

    #[test]
    fn non_canonical_derived_fields_fail_closed_on_deserialization() {
        let mut value = serde_json::to_value(workflow_record()).expect("serialize");
        value["canonical_definition"]["definition"]["spec_content_hash"] =
            serde_json::Value::String(SpecContentHash::from_text("injected").to_string());
        let error = serde_json::from_value::<ImmutableRunBundleDefinitionRecord>(value)
            .expect_err("derived field rejected");
        assert_eq!(
            error.to_string(),
            "non-canonical immutable run bundle definition"
        );
    }

    #[test]
    fn debug_redacts_definition_content_identity_and_hashes() {
        let record = workflow_record();
        let debug = format!("{record:?}");
        assert!(!debug.contains("workflow/main"));
        assert!(!debug.contains("Main workflow"));
        assert!(!debug.contains(record.source_content_hash().as_str()));
        assert!(!debug.contains(record.canonical_record_hash().as_str()));
    }

    #[test]
    fn serialization_excludes_raw_source_and_forbidden_payload_fields() {
        let serialized = serde_json::to_string(&workflow_record()).expect("serialize");
        for forbidden in [
            "raw_yaml",
            "source_location",
            "provider_payload",
            "command_output",
            "credential",
            "authorization_header",
        ] {
            assert!(!serialized.contains(forbidden));
        }
    }
}
