use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

use crate::{
    ActorId, Diagnostic, ExplicitLocalCheckProfileId, GovernanceStrictnessProfile, PolicyId,
    ProjectId, SchemaVersion, SkillDefinition, SpecContentHash, WorkflowDefinition,
    WorkflowOsError, WorkflowOsErrorKind,
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
    /// Optional project-controlled authoritative governance activation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governance: Option<ProjectGovernanceConfiguration>,
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

/// Optional project-level governance configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectGovernanceConfiguration {
    /// Explicit activation of the closed authoritative local execution path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authoritative_execution: Option<AuthoritativeExecutionConfiguration>,
    /// Optional, subordinate provider-read declarations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_reads: Option<ProviderReadConfiguration>,
}

/// Closed provider-read declarations supported by the v0 project manifest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderReadConfiguration {
    /// Exact GitHub repository-metadata read declaration.
    pub github_repository_metadata: GitHubRepositoryMetadataReadConfiguration,
}

/// One exact canonical GitHub repository resource.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubRepositoryMetadataResource {
    owner: String,
    repository: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GitHubRepositoryMetadataResourceWire {
    owner: String,
    repository: String,
}

impl GitHubRepositoryMetadataResource {
    /// Creates one bounded lowercase GitHub repository resource.
    ///
    /// # Errors
    ///
    /// Returns a validation error when either component is malformed or not canonical.
    pub fn new(
        owner: impl Into<String>,
        repository: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            owner: owner.into(),
            repository: repository.into(),
        };
        value.validate()?;
        Ok(value)
    }

    /// Returns the canonical repository owner.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the canonical repository name.
    #[must_use]
    pub fn repository(&self) -> &str {
        &self.repository
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_github_resource_component("owner", &self.owner, 39, false)?;
        validate_github_resource_component("repository", &self.repository, 100, true)
    }
}

impl std::fmt::Debug for GitHubRepositoryMetadataResource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GitHubRepositoryMetadataResource")
            .field("owner", &"[REDACTED]")
            .field("repository", &"[REDACTED]")
            .finish()
    }
}

impl TryFrom<GitHubRepositoryMetadataResourceWire> for GitHubRepositoryMetadataResource {
    type Error = WorkflowOsError;

    fn try_from(value: GitHubRepositoryMetadataResourceWire) -> Result<Self, Self::Error> {
        Self::new(value.owner, value.repository)
    }
}

/// Exact, inactive project declaration for the first GitHub provider read.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "GitHubRepositoryMetadataReadConfigurationWire")]
pub struct GitHubRepositoryMetadataReadConfiguration {
    enabled: bool,
    endpoint: String,
    access_reference: String,
    resources: Vec<GitHubRepositoryMetadataResource>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GitHubRepositoryMetadataReadConfigurationWire {
    enabled: bool,
    endpoint: String,
    access_reference: String,
    resources: Vec<GitHubRepositoryMetadataResourceWire>,
}

impl GitHubRepositoryMetadataReadConfiguration {
    /// Fixed endpoint accepted by the first provider-read contract.
    pub const ENDPOINT: &'static str = "github.com";
    /// Fixed non-secret environment reference accepted by the contract.
    pub const ACCESS_REFERENCE: &'static str = "env/WORKFLOW_OS_GITHUB_TOKEN";
    const MAX_RESOURCES: usize = 32;

    /// Creates one validated, inactive provider-read declaration.
    ///
    /// # Errors
    ///
    /// Returns a validation error for disabled, unsupported, empty, excessive, or duplicate input.
    pub fn new(
        enabled: bool,
        endpoint: impl Into<String>,
        access_reference: impl Into<String>,
        resources: Vec<GitHubRepositoryMetadataResource>,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            enabled,
            endpoint: endpoint.into(),
            access_reference: access_reference.into(),
            resources,
        };
        value.validate()?;
        Ok(value)
    }

    /// Returns whether this exact declaration is enabled.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the fixed provider endpoint.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the non-secret access reference.
    #[must_use]
    pub fn access_reference(&self) -> &str {
        &self.access_reference
    }

    /// Returns exact allowed resources.
    #[must_use]
    pub fn resources(&self) -> &[GitHubRepositoryMetadataResource] {
        &self.resources
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if !self.enabled {
            return Err(WorkflowOsError::validation(
                "project.governance.provider_reads.github_repository_metadata.disabled",
                "GitHub repository metadata read declaration must be explicitly enabled",
            ));
        }
        if self.endpoint != Self::ENDPOINT {
            return Err(WorkflowOsError::validation(
                "project.governance.provider_reads.github_repository_metadata.endpoint_unsupported",
                "GitHub repository metadata read endpoint is unsupported",
            ));
        }
        if self.access_reference != Self::ACCESS_REFERENCE {
            return Err(WorkflowOsError::validation(
                "project.governance.provider_reads.github_repository_metadata.access_reference_unsupported",
                "GitHub repository metadata access reference is unsupported",
            ));
        }
        if self.resources.is_empty() || self.resources.len() > Self::MAX_RESOURCES {
            return Err(WorkflowOsError::validation(
                "project.governance.provider_reads.github_repository_metadata.resources_invalid",
                "GitHub repository metadata resources must use the supported bounded count",
            ));
        }
        let unique = self
            .resources
            .iter()
            .map(|resource| (resource.owner(), resource.repository()))
            .collect::<std::collections::BTreeSet<_>>();
        if unique.len() != self.resources.len() {
            return Err(WorkflowOsError::validation(
                "project.governance.provider_reads.github_repository_metadata.resource_duplicate",
                "GitHub repository metadata resources must be unique",
            ));
        }
        Ok(())
    }
}

impl std::fmt::Debug for GitHubRepositoryMetadataReadConfiguration {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GitHubRepositoryMetadataReadConfiguration")
            .field("enabled", &self.enabled)
            .field("endpoint", &self.endpoint)
            .field("access_reference", &"[REDACTED]")
            .field("resource_count", &self.resources.len())
            .finish()
    }
}

impl TryFrom<GitHubRepositoryMetadataReadConfigurationWire>
    for GitHubRepositoryMetadataReadConfiguration
{
    type Error = WorkflowOsError;

    fn try_from(value: GitHubRepositoryMetadataReadConfigurationWire) -> Result<Self, Self::Error> {
        Self::new(
            value.enabled,
            value.endpoint,
            value.access_reference,
            value
                .resources
                .into_iter()
                .map(GitHubRepositoryMetadataResource::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )
    }
}

fn validate_github_resource_component(
    label: &'static str,
    value: &str,
    max_bytes: usize,
    allow_dot_and_underscore: bool,
) -> Result<(), WorkflowOsError> {
    let characters_valid = value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'-'
            || (allow_dot_and_underscore && matches!(byte, b'.' | b'_'))
    });
    if value.is_empty()
        || value.len() > max_bytes
        || !characters_valid
        || !value
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        || !value
            .as_bytes()
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
    {
        return Err(WorkflowOsError::validation(
            format!("project.governance.provider_reads.github_repository_metadata.{label}_invalid"),
            format!("GitHub repository metadata {label} is invalid"),
        ));
    }
    Ok(())
}

/// Validated project declaration for authoritative local execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "AuthoritativeExecutionConfigurationWire")]
pub struct AuthoritativeExecutionConfiguration {
    profile: GovernanceStrictnessProfile,
    local_check_profile: ExplicitLocalCheckProfileId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoritativeExecutionConfigurationWire {
    profile: GovernanceStrictnessProfile,
    local_check_profile: ExplicitLocalCheckProfileId,
}

impl AuthoritativeExecutionConfiguration {
    /// Creates the only authoritative execution combination supported by v0.
    ///
    /// # Errors
    ///
    /// Returns a stable validation error when the supplied strictness profile
    /// and local-check profile are not an implemented activation combination.
    pub fn new(
        profile: GovernanceStrictnessProfile,
        local_check_profile: ExplicitLocalCheckProfileId,
    ) -> Result<Self, WorkflowOsError> {
        if profile != GovernanceStrictnessProfile::ObserveAndReport
            || local_check_profile != ExplicitLocalCheckProfileId::WorkflowOsProjectValidation
        {
            return Err(WorkflowOsError::validation(
                "project.governance.authoritative_execution.unsupported",
                "authoritative execution configuration is unsupported",
            ));
        }
        Ok(Self {
            profile,
            local_check_profile,
        })
    }

    /// Returns the declared minimum governance strictness profile.
    #[must_use]
    pub const fn profile(self) -> GovernanceStrictnessProfile {
        self.profile
    }

    /// Returns the declared closed local-check profile.
    #[must_use]
    pub const fn local_check_profile(self) -> ExplicitLocalCheckProfileId {
        self.local_check_profile
    }
}

impl TryFrom<AuthoritativeExecutionConfigurationWire> for AuthoritativeExecutionConfiguration {
    type Error = WorkflowOsError;

    fn try_from(value: AuthoritativeExecutionConfigurationWire) -> Result<Self, Self::Error> {
        Self::new(value.profile, value.local_check_profile)
    }
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
    let yaml = parse_yaml_value(source)?;
    ensure_supported_schema_version(&yaml)?;
    reject_secret_values(&yaml)?;
    validate_project_governance_yaml(&yaml)?;
    deserialize_yaml_document(yaml)
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
    deserialize_yaml_document(yaml)
}

fn deserialize_yaml_document<T>(yaml: YamlValue) -> Result<T, WorkflowOsError>
where
    T: for<'de> Deserialize<'de>,
{
    serde_yaml::from_value(yaml).map_err(|error| {
        WorkflowOsError::new(
            WorkflowOsErrorKind::Parse,
            "spec.parse",
            format!("failed to parse spec document: {error}"),
        )
        .with_diagnostic(Diagnostic::error("spec.parse", error.to_string()))
    })
}

fn validate_project_governance_yaml(value: &YamlValue) -> Result<(), WorkflowOsError> {
    let Some(governance) = value
        .as_mapping()
        .and_then(|mapping| mapping.get(YamlValue::String("governance".to_owned())))
    else {
        return Ok(());
    };
    let governance = governance.as_mapping();
    let authoritative_execution = governance
        .and_then(|mapping| mapping.get(YamlValue::String("authoritative_execution".to_owned())));
    let provider_reads =
        governance.and_then(|mapping| mapping.get(YamlValue::String("provider_reads".to_owned())));
    if provider_reads.is_some() && authoritative_execution.is_none() {
        return Err(WorkflowOsError::validation(
            "project.governance.provider_reads.authoritative_execution_required",
            "provider reads require the supported authoritative execution declaration",
        ));
    }
    let Some(authoritative_execution) = authoritative_execution else {
        return Ok(());
    };
    let Some(configuration) = authoritative_execution.as_mapping() else {
        return Ok(());
    };
    let profile = configuration
        .get(YamlValue::String("profile".to_owned()))
        .and_then(YamlValue::as_str)
        .ok_or_else(|| {
            WorkflowOsError::validation(
                "project.governance.authoritative_execution.profile_missing",
                "authoritative execution configuration requires a governance profile",
            )
        })?;
    let local_check_profile = configuration
        .get(YamlValue::String("local_check_profile".to_owned()))
        .and_then(YamlValue::as_str)
        .ok_or_else(|| {
            WorkflowOsError::validation(
                "project.governance.authoritative_execution.local_check_profile_missing",
                "authoritative execution configuration requires a local-check profile",
            )
        })?;
    if profile != GovernanceStrictnessProfile::ObserveAndReport.label() {
        return Err(WorkflowOsError::validation(
            "project.governance.authoritative_execution.profile_unsupported",
            "authoritative execution governance profile is unsupported",
        ));
    }
    if local_check_profile != "workflow_os_project_validation" {
        return Err(WorkflowOsError::validation(
            "project.governance.authoritative_execution.local_check_profile_unsupported",
            "authoritative execution local-check profile is unsupported",
        ));
    }
    if let Some(provider_reads) = provider_reads {
        let github_repository_metadata = provider_reads
            .as_mapping()
            .and_then(|mapping| {
                mapping.get(YamlValue::String("github_repository_metadata".to_owned()))
            })
            .ok_or_else(|| {
                WorkflowOsError::validation(
                    "project.governance.provider_reads.github_repository_metadata.missing",
                    "provider reads require the GitHub repository metadata declaration",
                )
            })?;
        let wire: GitHubRepositoryMetadataReadConfigurationWire =
            serde_yaml::from_value(github_repository_metadata.clone()).map_err(|error| {
                WorkflowOsError::new(
                    WorkflowOsErrorKind::Parse,
                    "project.governance.provider_reads.parse",
                    "failed to parse provider-read declaration",
                )
                .with_diagnostic(Diagnostic::error(
                    "project.governance.provider_reads.parse",
                    error.to_string(),
                ))
            })?;
        GitHubRepositoryMetadataReadConfiguration::try_from(wire)?;
    }
    Ok(())
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
