use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

use crate::{
    ActorId, Diagnostic, PolicyId, ProjectId, SchemaVersion, SkillDefinition, SpecContentHash,
    WorkflowDefinition, WorkflowOsError, WorkflowOsErrorKind,
};

/// The only schema version supported by the v0 foundation parser.
pub const SUPPORTED_SCHEMA_VERSION: &str = "workflowos.dev/v0";

/// Project metadata from `workflow-os.yml`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectManifest {
    /// Schema version for this manifest file.
    pub schema_version: SchemaVersion,
    /// Project identity and human-readable metadata.
    pub project: ProjectMetadata,
    /// Optional path overrides for the canonical project layout.
    #[serde(default)]
    pub layout: ProjectLayout,
    /// Optional environment/config overlay references.
    #[serde(default)]
    pub config: Vec<ConfigOverlay>,
}

/// Project identity and display metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectMetadata {
    /// Stable project identifier.
    pub id: ProjectId,
    /// Human-readable project name.
    pub name: String,
    /// Optional project description.
    #[serde(default)]
    pub description: Option<String>,
}

/// Canonical project layout paths.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectLayout {
    /// Directory containing `*.workflow.yml`.
    pub workflows: String,
    /// Directory containing `*.skill.yml`.
    pub skills: String,
    /// Directory containing `*.policy.yml`.
    pub policies: String,
    /// Directory containing `*.test.yml`.
    pub tests: String,
}

impl Default for ProjectLayout {
    fn default() -> Self {
        Self {
            workflows: "workflows".to_owned(),
            skills: "skills".to_owned(),
            policies: "policies".to_owned(),
            tests: "tests".to_owned(),
        }
    }
}

/// Reference to a non-secret environment/config overlay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigOverlay {
    /// Environment name, such as `dev` or `ci`.
    pub environment: String,
    /// Optional non-secret variables.
    #[serde(default)]
    pub vars: Vec<ConfigVar>,
}

/// Non-secret configuration variable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigVar {
    /// Variable name.
    pub name: String,
    /// Non-secret variable value.
    pub value: String,
}

/// Policy definition shell from `policies/*.policy.yml`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySpecDocument {
    /// Schema version for this policy spec.
    pub schema_version: SchemaVersion,
    /// Policy identifier.
    pub id: PolicyId,
    /// Human-readable policy name.
    pub name: String,
    /// Optional policy description.
    #[serde(default)]
    pub description: Option<String>,
    /// Policy rules parsed as shells only.
    #[serde(default)]
    pub rules: Vec<PolicyRuleShell>,
}

/// Minimal policy rule shell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyRuleShell {
    /// Rule identifier local to the policy.
    pub id: String,
    /// Rule effect.
    pub effect: String,
    /// Optional actor for the rule.
    #[serde(default)]
    pub actor: Option<ActorId>,
}

/// Test definition shell from `tests/*.test.yml`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestSpecDocument {
    /// Schema version for this test spec.
    pub schema_version: SchemaVersion,
    /// Test identifier.
    pub id: String,
    /// Human-readable test name.
    pub name: String,
    /// Referenced workflow, skill, or policy under test.
    pub target: SpecReference,
    /// Test assertion shells.
    #[serde(default)]
    pub assertions: Vec<TestAssertionShell>,
}

/// Minimal test assertion shell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestAssertionShell {
    /// Assertion identifier local to the test.
    pub id: String,
    /// Assertion description.
    pub description: String,
}

/// Reference to another spec by ID and optional version.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecReference {
    /// Referenced spec identifier.
    pub id: String,
    /// Optional version. Missing versions are resolved by documented reference rules.
    #[serde(default)]
    pub version: Option<String>,
}

/// Reference resolution strategy for v0 projects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceResolutionRules {
    /// Resolve only within the local project.
    LocalProjectOnly,
}

/// Environment reference accepted by v0 config overlays.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentRef {
    /// Environment name.
    pub name: String,
}

/// Parses a project manifest from YAML.
///
/// # Errors
///
/// Returns an error for invalid YAML, missing schema version, unsupported schema version,
/// malformed identifiers, unknown fields, or secret-looking values in spec fields.
pub fn parse_project_manifest_yaml(source: &str) -> Result<ProjectManifest, WorkflowOsError> {
    parse_yaml_document(source)
}

/// Parses a workflow spec from YAML.
///
/// # Errors
///
/// Returns an error for invalid YAML, missing schema version, unsupported schema version,
/// malformed identifiers, unknown fields, or secret-looking values in spec fields.
pub fn parse_workflow_spec_yaml(source: &str) -> Result<WorkflowDefinition, WorkflowOsError> {
    let mut workflow: WorkflowDefinition = parse_yaml_document(source)?;
    workflow.spec_content_hash = Some(canonical_yaml_content_hash(source)?);
    Ok(workflow)
}

/// Parses a skill spec from YAML.
///
/// # Errors
///
/// Returns an error for invalid YAML, missing schema version, unsupported schema version,
/// malformed identifiers, unknown fields, or secret-looking values in spec fields.
pub fn parse_skill_spec_yaml(source: &str) -> Result<SkillDefinition, WorkflowOsError> {
    parse_yaml_document(source)
}

/// Parses a policy spec from YAML.
///
/// # Errors
///
/// Returns an error for invalid YAML, missing schema version, unsupported schema version,
/// malformed identifiers, unknown fields, or secret-looking values in spec fields.
pub fn parse_policy_spec_yaml(source: &str) -> Result<PolicySpecDocument, WorkflowOsError> {
    parse_yaml_document(source)
}

/// Parses a test spec from YAML.
///
/// # Errors
///
/// Returns an error for invalid YAML, missing schema version, unsupported schema version,
/// malformed identifiers, unknown fields, or secret-looking values in spec fields.
pub fn parse_test_spec_yaml(source: &str) -> Result<TestSpecDocument, WorkflowOsError> {
    parse_yaml_document(source)
}

/// Computes a stable content hash over canonicalized YAML content.
///
/// # Errors
///
/// Returns an error when the YAML cannot be parsed or canonicalized.
pub fn canonical_yaml_content_hash(source: &str) -> Result<SpecContentHash, WorkflowOsError> {
    let yaml = parse_yaml_value(source)?;
    ensure_supported_schema_version(&yaml)?;
    reject_secret_values(&yaml)?;
    Ok(SpecContentHash::from_text(&canonical_json(&yaml)?))
}

fn parse_yaml_document<T>(source: &str) -> Result<T, WorkflowOsError>
where
    T: for<'de> Deserialize<'de>,
{
    let yaml = parse_yaml_value(source)?;
    ensure_supported_schema_version(&yaml)?;
    reject_secret_values(&yaml)?;
    serde_yaml::from_value(yaml).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::Parse,
            "spec.parse",
            format!("failed to parse spec document: {error}"),
        )
        .with_diagnostic(Diagnostic::error("spec.parse", error.to_string()))
    })
}

fn parse_yaml_value(source: &str) -> Result<YamlValue, WorkflowOsError> {
    serde_yaml::from_str(source).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::Parse,
            "yaml.parse",
            format!("failed to parse YAML: {error}"),
        )
    })
}

fn ensure_supported_schema_version(value: &YamlValue) -> Result<(), WorkflowOsError> {
    let schema_version = value
        .as_mapping()
        .and_then(|mapping| mapping.get(YamlValue::String("schema_version".to_owned())))
        .and_then(YamlValue::as_str)
        .ok_or_else(|| {
            WorkflowOsError::validation(
                "schema_version.missing",
                "spec document must declare schema_version",
            )
        })?;

    if schema_version != SUPPORTED_SCHEMA_VERSION {
        return Err(WorkflowOsError::validation(
            "schema_version.unsupported",
            format!(
                "unsupported schema_version {schema_version}; expected {SUPPORTED_SCHEMA_VERSION}"
            ),
        ));
    }

    Ok(())
}

fn reject_secret_values(value: &YamlValue) -> Result<(), WorkflowOsError> {
    let mut path = Vec::new();
    reject_secret_values_at(value, &mut path)
}

fn reject_secret_values_at(
    value: &YamlValue,
    path: &mut Vec<String>,
) -> Result<(), WorkflowOsError> {
    match value {
        YamlValue::Mapping(mapping) => {
            for (key, nested) in mapping {
                let key_text = yaml_key_to_string(key)?;
                path.push(key_text.clone());
                if looks_like_secret_key(&key_text) {
                    return Err(secret_error(path));
                }
                reject_secret_values_at(nested, path)?;
                path.pop();
            }
        }
        YamlValue::Sequence(sequence) => {
            for (index, nested) in sequence.iter().enumerate() {
                path.push(index.to_string());
                reject_secret_values_at(nested, path)?;
                path.pop();
            }
        }
        YamlValue::String(text)
            if looks_like_inline_secret(text) || looks_like_secret_key(text) =>
        {
            return Err(secret_error(path));
        }
        _ => {}
    }
    Ok(())
}

fn secret_error(path: &[String]) -> WorkflowOsError {
    let document_path = format_document_path(path);
    WorkflowOsError::security(
        "spec.secret_disallowed",
        format!("spec field {document_path} appears to contain a secret"),
    )
    .with_diagnostic(Diagnostic::error(
        "spec.secret_disallowed",
        format!("secrets must not be stored in specs at {document_path}"),
    ))
}

fn looks_like_secret_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "secret" | "secrets" | "password" | "token" | "api_key" | "apikey" | "private_key"
    )
}

fn looks_like_inline_secret(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    normalized.starts_with("secret:")
        || normalized.starts_with("token:")
        || normalized.starts_with("password:")
}

fn canonical_json(value: &YamlValue) -> Result<String, WorkflowOsError> {
    let json = yaml_to_json(value)?;
    serde_json::to_string(&json).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::Internal,
            "canonical_json.serialize",
            format!("failed to serialize canonical JSON: {error}"),
        )
    })
}

fn yaml_to_json(value: &YamlValue) -> Result<JsonValue, WorkflowOsError> {
    match value {
        YamlValue::Null => Ok(JsonValue::Null),
        YamlValue::Bool(value) => Ok(JsonValue::Bool(*value)),
        YamlValue::Number(value) => serde_json::to_value(value).map_err(|error| {
            WorkflowOsError::new(
                WorkflowOsErrorKind::Parse,
                "yaml.number",
                format!("failed to convert YAML number: {error}"),
            )
        }),
        YamlValue::String(value) => Ok(JsonValue::String(value.clone())),
        YamlValue::Sequence(sequence) => sequence
            .iter()
            .map(yaml_to_json)
            .collect::<Result<Vec<_>, _>>()
            .map(JsonValue::Array),
        YamlValue::Mapping(mapping) => {
            let mut object = BTreeMap::new();
            for (key, nested) in mapping {
                object.insert(yaml_key_to_string(key)?, yaml_to_json(nested)?);
            }
            serde_json::to_value(object).map_err(|error| {
                WorkflowOsError::new(
                    WorkflowOsErrorKind::Internal,
                    "canonical_json.object",
                    format!("failed to convert YAML object: {error}"),
                )
            })
        }
        YamlValue::Tagged(tagged) => yaml_to_json(&tagged.value),
    }
}

fn yaml_key_to_string(value: &YamlValue) -> Result<String, WorkflowOsError> {
    value
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| WorkflowOsError::validation("yaml.key", "YAML mapping keys must be strings"))
}

fn format_document_path(path: &[String]) -> String {
    if path.is_empty() {
        return "$".to_owned();
    }

    let mut output = String::from("$");
    for segment in path {
        if segment.chars().all(|character| character.is_ascii_digit()) {
            output.push('[');
            output.push_str(segment);
            output.push(']');
        } else {
            output.push('.');
            output.push_str(segment);
        }
    }
    output
}
