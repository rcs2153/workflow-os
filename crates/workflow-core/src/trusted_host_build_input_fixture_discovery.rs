//! Fixture-only parser and resolver for trusted-host Rust build metadata.
//!
//! This module reads a closed set of files beneath an explicitly supplied
//! fixture root. It does not run Cargo, source code, build scripts, procedural
//! macros, providers, or shell commands. Its output is non-authorizing.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{
    BuildTimeInputKind, BuildTimeInputKindCount, SpecContentHash,
    TrustedHostBuildInputDiscoveryContractVersion, TrustedHostBuildInputDiscoveryRequest,
    TrustedHostBuildInputDiscoveryRequestDefinition, TrustedHostBuildInputDiscoveryResult,
    TrustedHostBuildInputDiscoveryResultDefinition, TrustedHostBuildInputHostObligation,
    TrustedHostBuildInputHostObligationKind, TrustedHostBuildInputHostObligationPosture,
    TrustedHostBuildInputSelectionPosture, TrustedHostBuildInputUnsupportedReason,
    TrustedHostStaticGraphCompleteness, WorkflowOsError,
};

const PARSER_POLICY_VERSION: &str = "workflow-os/trusted-host-fixture-cargo-parser/v1";
const SOURCE_SET_DOMAIN: &str = "workflow-os/trusted-host-fixture-source-set/v1";
const GRAPH_DOMAIN: &str = "workflow-os/trusted-host-fixture-static-graph/v1";
const POLICY_DOMAIN: &str = "workflow-os/trusted-host-fixture-policy/v1";

const HARD_MAX_FILES: u32 = 512;
const HARD_MAX_BYTES_PER_FILE: u64 = 4 * 1024 * 1024;
const HARD_MAX_TOTAL_BYTES: u64 = 32 * 1024 * 1024;
const HARD_MAX_WORKSPACE_MEMBERS: u32 = 512;
const HARD_MAX_LOCAL_PATH_DEPENDENCIES: u32 = 512;
const HARD_MAX_LOCKED_PACKAGES: u32 = 16_384;
const HARD_MAX_PACKAGE_EDGES: u32 = 16_384;
const HARD_MAX_FEATURE_EXPRESSIONS: u32 = 4_096;
const HARD_MAX_GRAPH_DEPTH: u32 = 128;

/// Caller-supplied hard bounds for one fixture discovery operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_field_names)]
pub struct TrustedHostBuildInputFixtureDiscoveryLimits {
    max_files: u32,
    max_bytes_per_file: u64,
    max_total_bytes: u64,
    max_workspace_members: u32,
    max_local_path_dependencies: u32,
    max_locked_packages: u32,
    max_package_edges: u32,
    max_feature_expressions: u32,
    max_graph_depth: u32,
}

/// Authoritative values used to construct fixture discovery limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrustedHostBuildInputFixtureDiscoveryLimitsDefinition {
    /// Maximum approved metadata files.
    pub max_files: u32,
    /// Maximum bytes in one metadata file.
    pub max_bytes_per_file: u64,
    /// Maximum bytes across every metadata file.
    pub max_total_bytes: u64,
    /// Maximum exact workspace members.
    pub max_workspace_members: u32,
    /// Maximum local path dependencies.
    pub max_local_path_dependencies: u32,
    /// Maximum lockfile package records.
    pub max_locked_packages: u32,
    /// Maximum resolved package edges.
    pub max_package_edges: u32,
    /// Maximum feature expressions.
    pub max_feature_expressions: u32,
    /// Maximum package-graph traversal depth.
    pub max_graph_depth: u32,
}

impl TrustedHostBuildInputFixtureDiscoveryLimits {
    /// Creates validated fixture discovery limits.
    ///
    /// # Errors
    ///
    /// Rejects zero or implementation-exceeding limits with a stable error.
    pub fn new(
        definition: TrustedHostBuildInputFixtureDiscoveryLimitsDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.max_files == 0
            || definition.max_files > HARD_MAX_FILES
            || definition.max_bytes_per_file == 0
            || definition.max_bytes_per_file > HARD_MAX_BYTES_PER_FILE
            || definition.max_total_bytes < definition.max_bytes_per_file
            || definition.max_total_bytes > HARD_MAX_TOTAL_BYTES
            || definition.max_workspace_members == 0
            || definition.max_workspace_members > HARD_MAX_WORKSPACE_MEMBERS
            || definition.max_local_path_dependencies == 0
            || definition.max_local_path_dependencies > HARD_MAX_LOCAL_PATH_DEPENDENCIES
            || definition.max_locked_packages == 0
            || definition.max_locked_packages > HARD_MAX_LOCKED_PACKAGES
            || definition.max_package_edges == 0
            || definition.max_package_edges > HARD_MAX_PACKAGE_EDGES
            || definition.max_feature_expressions == 0
            || definition.max_feature_expressions > HARD_MAX_FEATURE_EXPRESSIONS
            || definition.max_graph_depth == 0
            || definition.max_graph_depth > HARD_MAX_GRAPH_DEPTH
        {
            return Err(fixture_error(
                "limits.invalid",
                "trusted-host fixture discovery limits are invalid",
            ));
        }
        Ok(Self {
            max_files: definition.max_files,
            max_bytes_per_file: definition.max_bytes_per_file,
            max_total_bytes: definition.max_total_bytes,
            max_workspace_members: definition.max_workspace_members,
            max_local_path_dependencies: definition.max_local_path_dependencies,
            max_locked_packages: definition.max_locked_packages,
            max_package_edges: definition.max_package_edges,
            max_feature_expressions: definition.max_feature_expressions,
            max_graph_depth: definition.max_graph_depth,
        })
    }

    /// Returns conservative standard fixture limits.
    #[must_use]
    pub const fn standard() -> Self {
        Self {
            max_files: 128,
            max_bytes_per_file: 1024 * 1024,
            max_total_bytes: 8 * 1024 * 1024,
            max_workspace_members: 256,
            max_local_path_dependencies: 256,
            max_locked_packages: 4_096,
            max_package_edges: 16_384,
            max_feature_expressions: 1_024,
            max_graph_depth: 64,
        }
    }
}

/// Explicit input for one fixture-only discovery operation.
pub struct TrustedHostBuildInputFixtureDiscoveryInput<'a> {
    fixture_root: &'a Path,
    repository_tree_commitment: SpecContentHash,
    selection_commitment: SpecContentHash,
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
}

impl<'a> TrustedHostBuildInputFixtureDiscoveryInput<'a> {
    /// Creates one explicit fixture discovery input.
    #[must_use]
    pub const fn new(
        fixture_root: &'a Path,
        repository_tree_commitment: SpecContentHash,
        selection_commitment: SpecContentHash,
        limits: TrustedHostBuildInputFixtureDiscoveryLimits,
    ) -> Self {
        Self {
            fixture_root,
            repository_tree_commitment,
            selection_commitment,
            limits,
        }
    }
}

impl fmt::Debug for TrustedHostBuildInputFixtureDiscoveryInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostBuildInputFixtureDiscoveryInput")
            .field("fixture_root", &"[REDACTED]")
            .field("repository_tree_commitment", &"[REDACTED]")
            .field("selection_commitment", &"[REDACTED]")
            .field("limits", &self.limits)
            .finish()
    }
}

/// Non-authorizing request and result produced from one fixture root.
#[derive(Clone, Eq, PartialEq)]
pub struct TrustedHostBuildInputFixtureDiscovery {
    request: TrustedHostBuildInputDiscoveryRequest,
    result: TrustedHostBuildInputDiscoveryResult,
}

impl TrustedHostBuildInputFixtureDiscovery {
    /// Returns the exact generated discovery request.
    #[must_use]
    pub const fn request(&self) -> &TrustedHostBuildInputDiscoveryRequest {
        &self.request
    }

    /// Returns the validated payload-free discovery result.
    #[must_use]
    pub const fn result(&self) -> &TrustedHostBuildInputDiscoveryResult {
        &self.result
    }

    /// Separates the generated request and result.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        TrustedHostBuildInputDiscoveryRequest,
        TrustedHostBuildInputDiscoveryResult,
    ) {
        (self.request, self.result)
    }
}

impl fmt::Debug for TrustedHostBuildInputFixtureDiscovery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostBuildInputFixtureDiscovery")
            .field("request", &self.request)
            .field("result", &self.result)
            .finish()
    }
}

/// Parses and resolves one bounded Cargo fixture without executing build code.
///
/// # Errors
///
/// Fails closed for missing, malformed, changing, unsupported, ambiguous,
/// symlinked, root-escaping, or over-limit metadata. Errors contain only stable
/// codes and generic messages.
pub fn discover_trusted_host_build_inputs_from_fixture(
    input: &TrustedHostBuildInputFixtureDiscoveryInput<'_>,
) -> Result<TrustedHostBuildInputFixtureDiscovery, WorkflowOsError> {
    let mut reader = FixtureReader::new(input.fixture_root, input.limits)?;
    reject_nested_repository(&reader, Path::new("."))?;

    let root_manifest_path = PathBuf::from("Cargo.toml");
    let root_manifest = parse_manifest(&reader.read(&root_manifest_path)?)?;
    let manifest_records = resolve_manifests(&mut reader, &root_manifest_path, root_manifest)?;

    let lockfile = parse_lockfile(&reader.read(Path::new("Cargo.lock"))?)?;
    validate_lockfile_bounds(&lockfile, input.limits)?;
    read_optional_metadata(&mut reader)?;

    let resolved = resolve_graph(&manifest_records, &lockfile, input.limits)?;
    let metadata_source_set_commitment = reader.source_set_commitment();
    let discovery_policy_commitment = discovery_policy_commitment(input.limits);
    let request = TrustedHostBuildInputDiscoveryRequest::new(
        TrustedHostBuildInputDiscoveryRequestDefinition {
            version: TrustedHostBuildInputDiscoveryContractVersion::StaticCargoGraphV1,
            repository_tree_commitment: input.repository_tree_commitment.clone(),
            selection_posture: TrustedHostBuildInputSelectionPosture::ConservativeAllPotentialEdges,
            selection_commitment: input.selection_commitment.clone(),
            metadata_source_set_commitment,
            discovery_policy_commitment,
        },
    );

    let mut kind_counts = vec![
        BuildTimeInputKindCount::new(BuildTimeInputKind::RepositoryContent, 1)?,
        BuildTimeInputKindCount::new(
            BuildTimeInputKind::ManifestOrConfiguration,
            reader.file_count,
        )?,
    ];
    if resolved.external_package_count > 0 {
        kind_counts.push(BuildTimeInputKindCount::new(
            BuildTimeInputKind::DependencyPackage,
            resolved.external_package_count,
        )?);
    }
    if resolved.build_script_count > 0 {
        kind_counts.push(BuildTimeInputKindCount::new(
            BuildTimeInputKind::BuildScript,
            resolved.build_script_count,
        )?);
    }
    if resolved.procedural_macro_count > 0 {
        kind_counts.push(BuildTimeInputKindCount::new(
            BuildTimeInputKind::ProceduralMacro,
            resolved.procedural_macro_count,
        )?);
    }
    let total_count = kind_counts.iter().try_fold(0_u32, |total, entry| {
        total.checked_add(entry.count()).ok_or_else(|| {
            fixture_error(
                "bound_exceeded",
                "trusted-host fixture discovery bound was exceeded",
            )
        })
    })?;

    let (static_completeness, unsupported_reasons) = if resolved.external_package_count == 0 {
        (TrustedHostStaticGraphCompleteness::Complete, Vec::new())
    } else {
        (
            TrustedHostStaticGraphCompleteness::Incomplete,
            vec![TrustedHostBuildInputUnsupportedReason::SelectionAmbiguous],
        )
    };
    let result = TrustedHostBuildInputDiscoveryResult::new(
        TrustedHostBuildInputDiscoveryResultDefinition {
            request_fingerprint: request.request_fingerprint().clone(),
            resolved_graph_commitment: resolved.graph_commitment,
            static_completeness,
            total_count,
            kind_counts,
            host_obligations: host_obligations(),
            unsupported_reasons,
            observation_reference: None,
        },
    )?;
    result.validate_for_request(&request)?;
    Ok(TrustedHostBuildInputFixtureDiscovery { request, result })
}

struct FixtureReader {
    canonical_root: PathBuf,
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
    file_count: u32,
    total_bytes: u64,
    source_commitments: BTreeMap<PathBuf, SpecContentHash>,
}

impl FixtureReader {
    fn new(
        root: &Path,
        limits: TrustedHostBuildInputFixtureDiscoveryLimits,
    ) -> Result<Self, WorkflowOsError> {
        let root_metadata = fs::symlink_metadata(root).map_err(|_| {
            fixture_error(
                "root.invalid",
                "trusted-host fixture discovery root is invalid",
            )
        })?;
        if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
            return Err(fixture_error(
                "root.invalid",
                "trusted-host fixture discovery root is invalid",
            ));
        }
        let canonical_root = root.canonicalize().map_err(|_| {
            fixture_error(
                "root.invalid",
                "trusted-host fixture discovery root is invalid",
            )
        })?;
        Ok(Self {
            canonical_root,
            limits,
            file_count: 0,
            total_bytes: 0,
            source_commitments: BTreeMap::new(),
        })
    }

    fn read(&mut self, relative: &Path) -> Result<Vec<u8>, WorkflowOsError> {
        validate_relative_path(relative)?;
        if let Some(existing) = self.source_commitments.get(relative) {
            let _ = existing;
            return Err(fixture_error(
                "metadata.duplicate",
                "trusted-host fixture metadata source was read more than once",
            ));
        }
        reject_symlink_components(&self.canonical_root, relative)?;
        let candidate = self.canonical_root.join(relative);
        let canonical = candidate.canonicalize().map_err(|_| {
            fixture_error(
                "metadata.missing",
                "trusted-host fixture metadata is missing",
            )
        })?;
        if !canonical.starts_with(&self.canonical_root) {
            return Err(fixture_error(
                "root_escape",
                "trusted-host fixture metadata escapes the approved root",
            ));
        }
        let mut file = File::open(&canonical).map_err(|_| {
            fixture_error(
                "metadata.unreadable",
                "trusted-host fixture metadata could not be read",
            )
        })?;
        let before = file.metadata().map_err(|_| {
            fixture_error(
                "metadata.unreadable",
                "trusted-host fixture metadata could not be read",
            )
        })?;
        if !before.is_file() || before.len() > self.limits.max_bytes_per_file {
            return Err(fixture_error(
                "bound_exceeded",
                "trusted-host fixture discovery bound was exceeded",
            ));
        }
        let mut bytes = Vec::new();
        file.by_ref()
            .take(self.limits.max_bytes_per_file.saturating_add(1))
            .read_to_end(&mut bytes)
            .map_err(|_| {
                fixture_error(
                    "metadata.unreadable",
                    "trusted-host fixture metadata could not be read",
                )
            })?;
        if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > self.limits.max_bytes_per_file {
            return Err(fixture_error(
                "bound_exceeded",
                "trusted-host fixture discovery bound was exceeded",
            ));
        }
        let after = file.metadata().map_err(|_| {
            fixture_error(
                "metadata.unreadable",
                "trusted-host fixture metadata could not be read",
            )
        })?;
        if metadata_changed(&before, &after) {
            return Err(fixture_error(
                "metadata_changed",
                "trusted-host fixture metadata changed during discovery",
            ));
        }
        self.file_count = self.file_count.checked_add(1).ok_or_else(|| {
            fixture_error(
                "bound_exceeded",
                "trusted-host fixture discovery bound was exceeded",
            )
        })?;
        self.total_bytes = self
            .total_bytes
            .checked_add(u64::try_from(bytes.len()).unwrap_or(u64::MAX))
            .ok_or_else(|| {
                fixture_error(
                    "bound_exceeded",
                    "trusted-host fixture discovery bound was exceeded",
                )
            })?;
        if self.file_count > self.limits.max_files || self.total_bytes > self.limits.max_total_bytes
        {
            return Err(fixture_error(
                "bound_exceeded",
                "trusted-host fixture discovery bound was exceeded",
            ));
        }
        self.source_commitments
            .insert(relative.to_path_buf(), SpecContentHash::from_bytes(&bytes));
        Ok(bytes)
    }

    fn source_set_commitment(&self) -> SpecContentHash {
        let mut hasher = Sha256::new();
        hash_field(&mut hasher, "domain", SOURCE_SET_DOMAIN);
        for (relative, commitment) in &self.source_commitments {
            hash_field(
                &mut hasher,
                "relative_path",
                relative.to_string_lossy().as_ref(),
            );
            hash_field(&mut hasher, "content", commitment.as_str());
        }
        SpecContentHash::from_bytes(hasher.finalize())
    }
}

fn metadata_changed(before: &Metadata, after: &Metadata) -> bool {
    before.len() != after.len()
        || before.modified().ok() != after.modified().ok()
        || before.is_file() != after.is_file()
}

fn validate_relative_path(path: &Path) -> Result<(), WorkflowOsError> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(fixture_error(
            "path.invalid",
            "trusted-host fixture metadata path is invalid",
        ));
    }
    Ok(())
}

fn reject_symlink_components(root: &Path, relative: &Path) -> Result<(), WorkflowOsError> {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        let Component::Normal(segment) = component else {
            return Err(fixture_error(
                "path.invalid",
                "trusted-host fixture metadata path is invalid",
            ));
        };
        current.push(segment);
        let metadata = fs::symlink_metadata(&current).map_err(|_| {
            fixture_error(
                "metadata.missing",
                "trusted-host fixture metadata is missing",
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(fixture_error(
                "symlink_unsupported",
                "trusted-host fixture symlinks are unsupported",
            ));
        }
    }
    Ok(())
}

fn reject_nested_repository(
    reader: &FixtureReader,
    relative: &Path,
) -> Result<(), WorkflowOsError> {
    let directory = if relative == Path::new(".") {
        reader.canonical_root.clone()
    } else {
        reader.canonical_root.join(relative)
    };
    let nested = directory.join(".git");
    if relative != Path::new(".") && fs::symlink_metadata(nested).is_ok() {
        return Err(fixture_error(
            "nested_repository_unsupported",
            "trusted-host fixture nested repositories are unsupported",
        ));
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct CargoManifest {
    package: Option<CargoPackage>,
    workspace: Option<CargoWorkspace>,
    lib: Option<CargoLib>,
    #[serde(default)]
    dependencies: BTreeMap<String, CargoDependency>,
    #[serde(default)]
    build_dependencies: BTreeMap<String, CargoDependency>,
    #[serde(default)]
    dev_dependencies: BTreeMap<String, CargoDependency>,
    #[serde(default)]
    features: BTreeMap<String, Vec<String>>,
    target: Option<toml::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct CargoPackage {
    name: String,
    version: String,
    edition: Option<String>,
    rust_version: Option<String>,
    build: Option<CargoBuildDeclaration>,
    publish: Option<toml::Value>,
    description: Option<String>,
    license: Option<String>,
    repository: Option<String>,
    homepage: Option<String>,
    readme: Option<toml::Value>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CargoBuildDeclaration {
    Enabled(bool),
    Path(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct CargoWorkspace {
    #[serde(default)]
    members: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
    resolver: Option<String>,
    package: Option<toml::Value>,
    dependencies: Option<toml::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct CargoLib {
    path: String,
    name: Option<String>,
    #[serde(default)]
    proc_macro: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CargoDependency {
    Version(String),
    Detailed(CargoDependencyDetail),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct CargoDependencyDetail {
    version: Option<String>,
    path: Option<String>,
    package: Option<String>,
    #[serde(default)]
    optional: bool,
    default_features: Option<bool>,
    #[serde(default)]
    features: Vec<String>,
    workspace: Option<bool>,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    rev: Option<String>,
    registry: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CargoLockfile {
    version: u32,
    #[serde(default)]
    package: Vec<CargoLockPackage>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct CargoLockPackage {
    name: String,
    version: String,
    source: Option<String>,
    checksum: Option<String>,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RustToolchainFile {
    toolchain: RustToolchain,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RustToolchain {
    channel: String,
    #[serde(default)]
    components: Vec<String>,
    #[serde(default)]
    targets: Vec<String>,
    profile: Option<String>,
}

fn parse_manifest(bytes: &[u8]) -> Result<CargoManifest, WorkflowOsError> {
    let text = std::str::from_utf8(bytes).map_err(|_| metadata_invalid())?;
    let manifest: CargoManifest = toml::from_str(text).map_err(|_| metadata_invalid())?;
    validate_manifest_shape(&manifest)?;
    Ok(manifest)
}

fn parse_lockfile(bytes: &[u8]) -> Result<CargoLockfile, WorkflowOsError> {
    let text = std::str::from_utf8(bytes).map_err(|_| metadata_invalid())?;
    let lockfile: CargoLockfile = toml::from_str(text).map_err(|_| metadata_invalid())?;
    if !matches!(lockfile.version, 3 | 4) {
        return Err(fixture_error(
            "lockfile.unsupported",
            "trusted-host fixture lockfile version is unsupported",
        ));
    }
    Ok(lockfile)
}

fn validate_manifest_shape(manifest: &CargoManifest) -> Result<(), WorkflowOsError> {
    if manifest.target.is_some() {
        return Err(fixture_error(
            "selection_ambiguous",
            "trusted-host fixture selection is ambiguous",
        ));
    }
    if manifest
        .workspace
        .as_ref()
        .is_some_and(|workspace| workspace.package.is_some() || workspace.dependencies.is_some())
    {
        return Err(fixture_error(
            "workspace_inheritance_unsupported",
            "trusted-host fixture workspace inheritance is unsupported",
        ));
    }
    for dependency in manifest
        .dependencies
        .values()
        .chain(manifest.build_dependencies.values())
        .chain(manifest.dev_dependencies.values())
    {
        validate_dependency(dependency)?;
    }
    if manifest.package.is_some() && manifest.lib.is_none() {
        return Err(fixture_error(
            "implicit_target_unsupported",
            "trusted-host fixture implicit targets are unsupported",
        ));
    }
    Ok(())
}

fn validate_dependency(dependency: &CargoDependency) -> Result<(), WorkflowOsError> {
    if let CargoDependency::Detailed(detail) = dependency {
        if detail.workspace == Some(true) {
            return Err(fixture_error(
                "workspace_inheritance_unsupported",
                "trusted-host fixture workspace inheritance is unsupported",
            ));
        }
        if detail.git.is_some()
            || detail.branch.is_some()
            || detail.tag.is_some()
            || detail.rev.is_some()
            || detail.registry.is_some()
        {
            return Err(fixture_error(
                "source_replacement_unsupported",
                "trusted-host fixture package source is unsupported",
            ));
        }
    }
    Ok(())
}

#[derive(Clone)]
struct ManifestRecord {
    relative_path: PathBuf,
    package_name: String,
    package_version: String,
    dependencies: Vec<ManifestDependency>,
    explicit_build_script: bool,
    procedural_macro: bool,
    features: Vec<ManifestFeature>,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum DependencyRole {
    Normal,
    Build,
    Development,
}

#[derive(Clone)]
struct ManifestDependency {
    package_name: String,
    local_manifest_path: Option<PathBuf>,
    role: DependencyRole,
    optional: bool,
    default_features: bool,
    features: Vec<String>,
}

#[derive(Clone)]
struct ManifestFeature {
    name: String,
    expressions: Vec<String>,
}

#[allow(clippy::too_many_lines)]
fn resolve_manifests(
    reader: &mut FixtureReader,
    root_path: &Path,
    root_manifest: CargoManifest,
) -> Result<Vec<ManifestRecord>, WorkflowOsError> {
    let mut pending = VecDeque::new();
    let mut parsed = BTreeMap::new();
    parsed.insert(root_path.to_path_buf(), root_manifest);
    pending.push_back(root_path.to_path_buf());

    if let Some(workspace) = parsed
        .get(root_path)
        .and_then(|manifest| manifest.workspace.as_ref())
    {
        if usize_to_u32(workspace.members.len())? > reader.limits.max_workspace_members {
            return Err(bound_exceeded());
        }
        let excludes = workspace
            .exclude
            .iter()
            .map(|path| validated_directory_manifest_path(path))
            .collect::<Result<BTreeSet<_>, _>>()?;
        for member in &workspace.members {
            let member_manifest = validated_directory_manifest_path(member)?;
            if !excludes.contains(&member_manifest) {
                pending.push_back(member_manifest);
            }
        }
    }

    let mut records = Vec::new();
    let mut visited = BTreeSet::new();
    let mut local_dependency_count = 0_u32;
    let mut feature_expression_count = 0_u32;
    while let Some(relative_path) = pending.pop_front() {
        if !visited.insert(relative_path.clone()) {
            continue;
        }
        if usize_to_u32(visited.len())?
            > reader
                .limits
                .max_workspace_members
                .saturating_add(reader.limits.max_local_path_dependencies)
        {
            return Err(bound_exceeded());
        }
        let manifest = if let Some(manifest) = parsed.remove(&relative_path) {
            manifest
        } else {
            let parent = relative_path.parent().unwrap_or_else(|| Path::new("."));
            reject_nested_repository(reader, parent)?;
            parse_manifest(&reader.read(&relative_path)?)?
        };
        let Some(package) = manifest.package.as_ref() else {
            if relative_path == root_path && manifest.workspace.is_some() {
                continue;
            }
            return Err(fixture_error(
                "package.missing",
                "trusted-host fixture package declaration is missing",
            ));
        };
        validate_bounded_identifier(&package.name)?;
        validate_bounded_identifier(&package.version)?;
        if let Some(edition) = &package.edition {
            validate_bounded_value(edition)?;
        }
        if let Some(rust_version) = &package.rust_version {
            validate_bounded_value(rust_version)?;
        }
        let lib = manifest.lib.as_ref().ok_or_else(|| {
            fixture_error(
                "implicit_target_unsupported",
                "trusted-host fixture implicit targets are unsupported",
            )
        })?;
        let package_directory = relative_path.parent().unwrap_or_else(|| Path::new("."));
        let _ = normalize_relative_join(package_directory, &lib.path)?;
        if let Some(name) = &lib.name {
            validate_bounded_identifier(name)?;
        }
        validate_package_metadata(&manifest, package)?;

        let explicit_build_script = match &package.build {
            Some(CargoBuildDeclaration::Enabled(false)) => false,
            Some(CargoBuildDeclaration::Enabled(true)) => true,
            Some(CargoBuildDeclaration::Path(path)) => {
                let _ = normalize_relative_join(package_directory, path)?;
                true
            }
            None => {
                let implicit = reader
                    .canonical_root
                    .join(package_directory)
                    .join("build.rs");
                if fs::symlink_metadata(implicit).is_ok() {
                    return Err(fixture_error(
                        "implicit_build_script_unsupported",
                        "trusted-host fixture implicit build scripts are unsupported",
                    ));
                }
                false
            }
        };

        let mut dependencies = Vec::new();
        collect_dependencies(
            &manifest.dependencies,
            DependencyRole::Normal,
            package_directory,
            &mut dependencies,
            &mut pending,
            &mut local_dependency_count,
            reader.limits,
        )?;
        collect_dependencies(
            &manifest.build_dependencies,
            DependencyRole::Build,
            package_directory,
            &mut dependencies,
            &mut pending,
            &mut local_dependency_count,
            reader.limits,
        )?;
        collect_dependencies(
            &manifest.dev_dependencies,
            DependencyRole::Development,
            package_directory,
            &mut dependencies,
            &mut pending,
            &mut local_dependency_count,
            reader.limits,
        )?;

        let mut features = Vec::with_capacity(manifest.features.len());
        for (name, entries) in &manifest.features {
            validate_bounded_identifier(name)?;
            add_feature_expressions(&mut feature_expression_count, 1, reader.limits)?;
            let mut expressions = entries.clone();
            expressions.sort();
            expressions.dedup();
            add_feature_expressions(
                &mut feature_expression_count,
                expressions.len(),
                reader.limits,
            )?;
            for expression in &expressions {
                validate_bounded_value(expression)?;
            }
            features.push(ManifestFeature {
                name: name.clone(),
                expressions,
            });
        }
        for dependency in &dependencies {
            add_feature_expressions(
                &mut feature_expression_count,
                dependency.features.len(),
                reader.limits,
            )?;
        }

        records.push(ManifestRecord {
            relative_path,
            package_name: package.name.clone(),
            package_version: package.version.clone(),
            dependencies,
            explicit_build_script,
            procedural_macro: lib.proc_macro,
            features,
        });
    }
    records.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(records)
}

fn validate_package_metadata(
    manifest: &CargoManifest,
    package: &CargoPackage,
) -> Result<(), WorkflowOsError> {
    let _ = (&package.publish, &package.readme);
    for value in [
        package.description.as_ref(),
        package.license.as_ref(),
        package.repository.as_ref(),
        package.homepage.as_ref(),
        manifest
            .workspace
            .as_ref()
            .and_then(|value| value.resolver.as_ref()),
    ]
    .into_iter()
    .flatten()
    {
        validate_bounded_value(value)?;
    }
    Ok(())
}

fn collect_dependencies(
    dependencies: &BTreeMap<String, CargoDependency>,
    role: DependencyRole,
    package_directory: &Path,
    output: &mut Vec<ManifestDependency>,
    pending: &mut VecDeque<PathBuf>,
    local_dependency_count: &mut u32,
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
) -> Result<(), WorkflowOsError> {
    for (alias, dependency) in dependencies {
        validate_bounded_identifier(alias)?;
        let (package_name, local_manifest_path, optional, default_features, features) =
            match dependency {
                CargoDependency::Version(version) => {
                    validate_bounded_value(version)?;
                    (alias.clone(), None, false, true, Vec::new())
                }
                CargoDependency::Detailed(detail) => {
                    if let Some(version) = &detail.version {
                        validate_bounded_value(version)?;
                    }
                    let mut features = detail.features.clone();
                    features.sort();
                    features.dedup();
                    for feature in &features {
                        validate_bounded_value(feature)?;
                    }
                    let package_name = detail.package.clone().unwrap_or_else(|| alias.clone());
                    validate_bounded_identifier(&package_name)?;
                    let local_manifest_path = if let Some(path) = &detail.path {
                        validate_relative_text_path(path)?;
                        *local_dependency_count = local_dependency_count
                            .checked_add(1)
                            .ok_or_else(bound_exceeded)?;
                        if *local_dependency_count > limits.max_local_path_dependencies {
                            return Err(bound_exceeded());
                        }
                        let manifest_path =
                            normalize_relative_join(package_directory, path)?.join("Cargo.toml");
                        pending.push_back(manifest_path.clone());
                        Some(manifest_path)
                    } else {
                        None
                    };
                    (
                        package_name,
                        local_manifest_path,
                        detail.optional,
                        detail.default_features.unwrap_or(true),
                        features,
                    )
                }
            };
        output.push(ManifestDependency {
            package_name,
            local_manifest_path,
            role,
            optional,
            default_features,
            features,
        });
    }
    Ok(())
}

fn validated_directory_manifest_path(value: &str) -> Result<PathBuf, WorkflowOsError> {
    if value.contains('*') || value.contains('?') || value.contains('[') {
        return Err(fixture_error(
            "workspace_glob_unsupported",
            "trusted-host fixture workspace globs are unsupported",
        ));
    }
    Ok(normalize_relative_join(Path::new("."), value)?.join("Cargo.toml"))
}

fn add_feature_expressions(
    count: &mut u32,
    additional: usize,
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
) -> Result<(), WorkflowOsError> {
    *count = count
        .checked_add(usize_to_u32(additional)?)
        .ok_or_else(bound_exceeded)?;
    if *count > limits.max_feature_expressions {
        return Err(bound_exceeded());
    }
    Ok(())
}

fn normalize_relative_join(base: &Path, value: &str) -> Result<PathBuf, WorkflowOsError> {
    validate_relative_text_path(value)?;
    let mut output = PathBuf::new();
    for component in base.join(value).components() {
        match component {
            Component::Normal(value) => output.push(value),
            Component::CurDir => {}
            Component::ParentDir => {
                if !output.pop() {
                    return Err(fixture_error(
                        "root_escape",
                        "trusted-host fixture metadata escapes the approved root",
                    ));
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(fixture_error(
                    "root_escape",
                    "trusted-host fixture metadata escapes the approved root",
                ));
            }
        }
    }
    if output.as_os_str().is_empty() {
        return Err(fixture_error(
            "path.invalid",
            "trusted-host fixture metadata path is invalid",
        ));
    }
    Ok(output)
}

fn validate_relative_text_path(value: &str) -> Result<(), WorkflowOsError> {
    if value.len() > 512 || value.is_empty() || Path::new(value).is_absolute() {
        return Err(fixture_error(
            "path.invalid",
            "trusted-host fixture metadata path is invalid",
        ));
    }
    Ok(())
}

fn validate_bounded_identifier(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
        return Err(metadata_invalid());
    }
    Ok(())
}

fn validate_bounded_value(value: &str) -> Result<(), WorkflowOsError> {
    if value.len() > 512 || value.chars().any(char::is_control) {
        return Err(metadata_invalid());
    }
    Ok(())
}

fn validate_lockfile_bounds(
    lockfile: &CargoLockfile,
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
) -> Result<(), WorkflowOsError> {
    if usize_to_u32(lockfile.package.len())? > limits.max_locked_packages {
        return Err(bound_exceeded());
    }
    let edge_count = lockfile.package.iter().try_fold(0_u32, |total, package| {
        total
            .checked_add(usize_to_u32(package.dependencies.len())?)
            .ok_or_else(bound_exceeded)
    })?;
    if edge_count > limits.max_package_edges {
        return Err(bound_exceeded());
    }
    Ok(())
}

fn read_optional_metadata(reader: &mut FixtureReader) -> Result<(), WorkflowOsError> {
    let toolchain_paths = [
        Path::new("rust-toolchain"),
        Path::new("rust-toolchain.toml"),
    ];
    let present_toolchains = toolchain_paths
        .iter()
        .map(|path| {
            optional_metadata_present(&reader.canonical_root, path).map(|present| (*path, present))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter_map(|(path, present)| present.then_some(path))
        .collect::<Vec<_>>();
    if present_toolchains.len() > 1 {
        return Err(metadata_invalid());
    }
    if let Some(path) = present_toolchains.first() {
        let bytes = reader.read(path)?;
        let text = std::str::from_utf8(&bytes).map_err(|_| metadata_invalid())?;
        let toolchain: RustToolchainFile = toml::from_str(text).map_err(|_| metadata_invalid())?;
        validate_bounded_value(&toolchain.toolchain.channel)?;
        if let Some(profile) = &toolchain.toolchain.profile {
            validate_bounded_value(profile)?;
        }
        for value in toolchain
            .toolchain
            .components
            .iter()
            .chain(&toolchain.toolchain.targets)
        {
            validate_bounded_value(value)?;
        }
    }

    let config_paths = [Path::new(".cargo/config"), Path::new(".cargo/config.toml")];
    let present_configs = config_paths
        .iter()
        .map(|path| {
            optional_metadata_present(&reader.canonical_root, path).map(|present| (*path, present))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter_map(|(path, present)| present.then_some(path))
        .collect::<Vec<_>>();
    if present_configs.len() > 1 {
        return Err(metadata_invalid());
    }
    if let Some(path) = present_configs.first() {
        let bytes = reader.read(path)?;
        let text = std::str::from_utf8(&bytes).map_err(|_| metadata_invalid())?;
        let values: BTreeMap<String, toml::Value> =
            toml::from_str(text).map_err(|_| metadata_invalid())?;
        if !values.is_empty() {
            return Err(fixture_error(
                "cargo_configuration_unsupported",
                "trusted-host fixture Cargo configuration is unsupported",
            ));
        }
    }
    Ok(())
}

fn optional_metadata_present(root: &Path, relative: &Path) -> Result<bool, WorkflowOsError> {
    match fs::symlink_metadata(root.join(relative)) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(fixture_error(
            "metadata.unreadable",
            "trusted-host fixture metadata could not be read",
        )),
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct PackageKey {
    name: String,
    version: String,
    source: Option<String>,
}

struct ResolvedGraph {
    graph_commitment: SpecContentHash,
    external_package_count: u32,
    build_script_count: u32,
    procedural_macro_count: u32,
}

#[allow(clippy::too_many_lines)]
fn resolve_graph(
    manifests: &[ManifestRecord],
    lockfile: &CargoLockfile,
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
) -> Result<ResolvedGraph, WorkflowOsError> {
    let mut packages = BTreeMap::<PackageKey, CargoLockPackage>::new();
    for package in &lockfile.package {
        validate_bounded_identifier(&package.name)?;
        validate_bounded_identifier(&package.version)?;
        if let Some(source) = &package.source {
            validate_bounded_value(source)?;
        }
        if let Some(checksum) = &package.checksum {
            validate_bounded_value(checksum)?;
        }
        let key = PackageKey {
            name: package.name.clone(),
            version: package.version.clone(),
            source: package.source.clone(),
        };
        if packages.insert(key, package.clone()).is_some() {
            return Err(lockfile_inconsistent());
        }
    }

    let mut local_keys = BTreeMap::<PathBuf, PackageKey>::new();
    for manifest in manifests {
        let matches = packages
            .keys()
            .filter(|key| {
                key.name == manifest.package_name
                    && key.version == manifest.package_version
                    && key.source.is_none()
            })
            .cloned()
            .collect::<Vec<_>>();
        let [key] = matches.as_slice() else {
            return Err(lockfile_inconsistent());
        };
        local_keys.insert(manifest.relative_path.clone(), key.clone());
    }

    let mut reachable = BTreeSet::new();
    let mut pending = VecDeque::new();
    for key in local_keys.values() {
        pending.push_back((key.clone(), 0_u32));
    }
    let mut edges = BTreeSet::<(
        PackageKey,
        PackageKey,
        &'static str,
        bool,
        bool,
        Vec<String>,
    )>::new();
    while let Some((key, depth)) = pending.pop_front() {
        if depth > limits.max_graph_depth {
            return Err(bound_exceeded());
        }
        if !reachable.insert(key.clone()) {
            continue;
        }
        let package = packages.get(&key).ok_or_else(lockfile_inconsistent)?;
        for dependency in &package.dependencies {
            let target = resolve_lock_dependency(dependency, &packages)?;
            edges.insert((
                key.clone(),
                target.clone(),
                "locked",
                false,
                true,
                Vec::new(),
            ));
            pending.push_back((target, depth.saturating_add(1)));
        }
    }
    if usize_to_u32(edges.len())? > limits.max_package_edges {
        return Err(bound_exceeded());
    }

    for manifest in manifests {
        let source = local_keys
            .get(&manifest.relative_path)
            .ok_or_else(lockfile_inconsistent)?;
        let lock_targets = edges
            .iter()
            .filter(|(from, _, _, _, _, _)| from == source)
            .map(|(_, target, _, _, _, _)| target.clone())
            .collect::<Vec<_>>();
        for dependency in &manifest.dependencies {
            let matching = lock_targets
                .iter()
                .filter(|target| target.name == dependency.package_name)
                .cloned()
                .collect::<Vec<_>>();
            let [target] = matching.as_slice() else {
                return Err(lockfile_inconsistent());
            };
            if let Some(local_manifest_path) = &dependency.local_manifest_path {
                let expected = local_keys
                    .get(local_manifest_path)
                    .ok_or_else(lockfile_inconsistent)?;
                if *target != *expected {
                    return Err(lockfile_inconsistent());
                }
            }
            edges.insert((
                source.clone(),
                target.clone(),
                role_tag(dependency.role),
                dependency.optional,
                dependency.default_features,
                dependency.features.clone(),
            ));
        }
    }

    let local_key_set = local_keys.values().cloned().collect::<BTreeSet<_>>();
    let external_package_count = usize_to_u32(
        reachable
            .iter()
            .filter(|key| !local_key_set.contains(*key))
            .count(),
    )?;
    let build_script_count = usize_to_u32(
        manifests
            .iter()
            .filter(|manifest| manifest.explicit_build_script)
            .count(),
    )?;
    let procedural_macro_count = usize_to_u32(
        manifests
            .iter()
            .filter(|manifest| manifest.procedural_macro)
            .count(),
    )?;

    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", GRAPH_DOMAIN);
    hash_field(&mut hasher, "policy_version", PARSER_POLICY_VERSION);
    for key in &reachable {
        hash_package_key(&mut hasher, "package", key);
        if let Some(package) = packages.get(key) {
            hash_field(
                &mut hasher,
                "checksum",
                package.checksum.as_deref().unwrap_or("local"),
            );
        }
    }
    for (source, target, role, optional, default_features, features) in &edges {
        hash_package_key(&mut hasher, "edge_source", source);
        hash_package_key(&mut hasher, "edge_target", target);
        hash_field(&mut hasher, "edge_role", role);
        hash_field(
            &mut hasher,
            "edge_optional",
            if *optional { "true" } else { "false" },
        );
        hash_field(
            &mut hasher,
            "edge_default_features",
            if *default_features { "true" } else { "false" },
        );
        for feature in features {
            hash_field(&mut hasher, "edge_feature", feature);
        }
    }
    for manifest in manifests {
        hash_field(
            &mut hasher,
            "manifest_path",
            manifest.relative_path.to_string_lossy().as_ref(),
        );
        hash_field(
            &mut hasher,
            "build_script",
            if manifest.explicit_build_script {
                "true"
            } else {
                "false"
            },
        );
        hash_field(
            &mut hasher,
            "procedural_macro",
            if manifest.procedural_macro {
                "true"
            } else {
                "false"
            },
        );
        for feature in &manifest.features {
            hash_field(&mut hasher, "feature_name", &feature.name);
            for expression in &feature.expressions {
                hash_field(&mut hasher, "feature_expression", expression);
            }
        }
    }

    Ok(ResolvedGraph {
        graph_commitment: SpecContentHash::from_bytes(hasher.finalize()),
        external_package_count,
        build_script_count,
        procedural_macro_count,
    })
}

fn resolve_lock_dependency(
    dependency: &str,
    packages: &BTreeMap<PackageKey, CargoLockPackage>,
) -> Result<PackageKey, WorkflowOsError> {
    validate_bounded_value(dependency)?;
    let mut parts = dependency.split_whitespace();
    let name = parts.next().ok_or_else(lockfile_inconsistent)?;
    let second = parts.next();
    let remaining = parts.collect::<Vec<_>>().join(" ");
    let (version, source) = match second {
        None => (None, None),
        Some(value) if value.starts_with('(') => (None, parse_parenthesized_source(value)?),
        Some(value) => {
            let source = if remaining.is_empty() {
                None
            } else {
                parse_parenthesized_source(&remaining)?
            };
            (Some(value), source)
        }
    };
    let matches = packages
        .keys()
        .filter(|key| {
            key.name == name
                && version.map_or(true, |expected| key.version == expected)
                && source
                    .as_deref()
                    .map_or(true, |expected| key.source.as_deref() == Some(expected))
        })
        .cloned()
        .collect::<Vec<_>>();
    let [key] = matches.as_slice() else {
        return Err(lockfile_inconsistent());
    };
    Ok(key.clone())
}

fn parse_parenthesized_source(value: &str) -> Result<Option<String>, WorkflowOsError> {
    if !value.starts_with('(') || !value.ends_with(')') || value.len() < 3 {
        return Err(lockfile_inconsistent());
    }
    Ok(Some(value[1..value.len() - 1].to_owned()))
}

fn role_tag(role: DependencyRole) -> &'static str {
    match role {
        DependencyRole::Normal => "normal",
        DependencyRole::Build => "build",
        DependencyRole::Development => "development",
    }
}

fn hash_package_key(hasher: &mut Sha256, label: &str, key: &PackageKey) {
    hash_field(hasher, label, &key.name);
    hash_field(hasher, "version", &key.version);
    hash_field(hasher, "source", key.source.as_deref().unwrap_or("local"));
}

fn discovery_policy_commitment(
    limits: TrustedHostBuildInputFixtureDiscoveryLimits,
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", POLICY_DOMAIN);
    hash_field(&mut hasher, "parser_policy", PARSER_POLICY_VERSION);
    hash_field(&mut hasher, "max_files", &limits.max_files.to_string());
    hash_field(
        &mut hasher,
        "max_bytes_per_file",
        &limits.max_bytes_per_file.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_total_bytes",
        &limits.max_total_bytes.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_workspace_members",
        &limits.max_workspace_members.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_local_path_dependencies",
        &limits.max_local_path_dependencies.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_locked_packages",
        &limits.max_locked_packages.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_package_edges",
        &limits.max_package_edges.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_feature_expressions",
        &limits.max_feature_expressions.to_string(),
    );
    hash_field(
        &mut hasher,
        "max_graph_depth",
        &limits.max_graph_depth.to_string(),
    );
    SpecContentHash::from_bytes(hasher.finalize())
}

fn host_obligations() -> Vec<TrustedHostBuildInputHostObligation> {
    use TrustedHostBuildInputHostObligationKind as Kind;
    [
        Kind::ToolchainExecutable,
        Kind::TargetSpecification,
        Kind::SdkOrSysroot,
        Kind::LinkerOrArchiver,
        Kind::NativeCompilerTools,
        Kind::DynamicLibraries,
        Kind::RegistryAndPackageSources,
        Kind::Environment,
        Kind::RuntimeBase,
    ]
    .into_iter()
    .map(|kind| {
        TrustedHostBuildInputHostObligation::new(
            kind,
            TrustedHostBuildInputHostObligationPosture::Unavailable,
        )
    })
    .collect()
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &str) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value.as_bytes());
}

fn usize_to_u32(value: usize) -> Result<u32, WorkflowOsError> {
    u32::try_from(value).map_err(|_| bound_exceeded())
}

fn metadata_invalid() -> WorkflowOsError {
    fixture_error(
        "metadata_invalid",
        "trusted-host fixture metadata is invalid",
    )
}

fn bound_exceeded() -> WorkflowOsError {
    fixture_error(
        "bound_exceeded",
        "trusted-host fixture discovery bound was exceeded",
    )
}

fn lockfile_inconsistent() -> WorkflowOsError {
    fixture_error(
        "lockfile_inconsistent",
        "trusted-host fixture lockfile is inconsistent",
    )
}

fn fixture_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("trusted_host.build_input_fixture_discovery.{suffix}"),
        message,
    )
}

#[cfg(test)]
#[path = "trusted_host_build_input_fixture_discovery_tests.rs"]
mod tests;
