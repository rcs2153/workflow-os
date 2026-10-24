use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use sha2::{Digest, Sha256};

use super::{
    file_content_hash, hash_private_commitment_field, local_check_error,
    AuthoritativeLocalCheckHandler, LocalCheckCommandContract, LocalCheckCommandKind,
    LocalCheckProcessOutput, LocalCheckProcessRequest, LocalCheckProcessRunner, LocalCheckResult,
    LocalCheckSideEffectClass, LocalCheckWorkingDirectoryPolicy,
};
use crate::{
    SkillHandler, SkillInput, SkillOutput, SpecContentHash, WorkflowOsError, WorkflowOsErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum CargoClippyWritableRootRole {
    Cache,
    BuildOutput,
    Temporary,
}

impl CargoClippyWritableRootRole {
    const fn label(self) -> &'static [u8] {
        match self {
            Self::Cache => b"cache",
            Self::BuildOutput => b"build_output",
            Self::Temporary => b"temporary",
        }
    }
}

#[derive(Clone)]
struct CargoClippyWritableRoot {
    role: CargoClippyWritableRootRole,
    path: PathBuf,
    initial_commitment: SpecContentHash,
}

/// Internal inputs for the injected-only Cargo clippy profile proof.
///
/// This definition is deliberately private to Core. It is not a workflow,
/// CLI, schema, SDK, or production process-resolution contract.
#[derive(Clone)]
struct CargoClippyWorkspaceHandlerDefinition {
    cargo_executable: PathBuf,
    cargo_clippy_executable: PathBuf,
    clippy_driver_executable: PathBuf,
    rustc_executable: PathBuf,
    rustdoc_executable: PathBuf,
    additional_toolchain_executables: Vec<PathBuf>,
    toolchain_root: PathBuf,
    repository_root: PathBuf,
    dependency_cache_root: PathBuf,
    build_output_root: PathBuf,
    temporary_root: PathBuf,
    dependency_inputs: Vec<PathBuf>,
}

/// Closed Cargo clippy handler proved only through an injected runner.
///
/// No public or standard-process constructor exists. A later real-process path
/// requires a separate security review and implementation phase.
#[derive(Clone)]
#[allow(dead_code)]
struct CargoClippyWorkspaceLocalHandler {
    contract: LocalCheckCommandContract,
    cargo_executable: PathBuf,
    cargo_clippy_executable: PathBuf,
    clippy_driver_executable: PathBuf,
    rustc_executable: PathBuf,
    rustdoc_executable: PathBuf,
    additional_toolchain_executables: Vec<PathBuf>,
    toolchain_root: PathBuf,
    repository_root: PathBuf,
    dependency_cache_root: PathBuf,
    build_output_root: PathBuf,
    temporary_root: PathBuf,
    dependency_inputs: Vec<PathBuf>,
    writable_roots: Vec<CargoClippyWritableRoot>,
    toolchain_commitment: SpecContentHash,
    dependency_commitment: SpecContentHash,
    protected_source_commitment: SpecContentHash,
    execution_identity: SpecContentHash,
    process_runner: Arc<dyn LocalCheckProcessRunner>,
}

#[allow(dead_code)]
impl CargoClippyWorkspaceLocalHandler {
    #[allow(clippy::too_many_lines)]
    fn new_with_process_runner(
        contract: LocalCheckCommandContract,
        definition: &CargoClippyWorkspaceHandlerDefinition,
        process_runner: Arc<dyn LocalCheckProcessRunner>,
    ) -> Result<Self, WorkflowOsError> {
        contract.validate()?;
        if contract != LocalCheckCommandContract::cargo_clippy_workspace_model_only()? {
            return Err(clippy_error(
                WorkflowOsErrorKind::Validation,
                "local_check.profile.clippy.contract_non_canonical",
                "Cargo clippy handler requires its complete canonical contract",
            ));
        }
        if contract.command_kind() != LocalCheckCommandKind::CargoClippyWorkspace
            || contract.working_directory_policy()
                != LocalCheckWorkingDirectoryPolicy::RepositoryRoot
            || contract.side_effect_class() != LocalCheckSideEffectClass::BuildOrCacheWrites
        {
            return Err(clippy_error(
                WorkflowOsErrorKind::Validation,
                "local_check.profile.clippy.contract_posture_invalid",
                "Cargo clippy handler requires its closed model-only posture",
            ));
        }

        let repository_root = canonical_unlinked_directory(
            &definition.repository_root,
            "local_check.profile.clippy.repository_root_invalid",
            "Cargo clippy handler requires a valid repository root",
        )?;
        if !repository_root.join("Cargo.toml").is_file()
            || !repository_root.join("Cargo.lock").is_file()
        {
            return Err(clippy_error(
                WorkflowOsErrorKind::Validation,
                "local_check.profile.clippy.repository_root_invalid",
                "Cargo clippy handler requires a locked Cargo workspace root",
            ));
        }
        let toolchain_root = canonical_unlinked_directory(
            &definition.toolchain_root,
            "local_check.profile.clippy.toolchain_root_invalid",
            "Cargo clippy handler requires a valid toolchain root",
        )?;
        let dependency_cache_root = canonical_unlinked_directory(
            &definition.dependency_cache_root,
            "local_check.profile.clippy.cache_root_invalid",
            "Cargo clippy handler requires a valid dependency cache root",
        )?;
        let build_output_root = canonical_unlinked_directory(
            &definition.build_output_root,
            "local_check.profile.clippy.build_root_invalid",
            "Cargo clippy handler requires a valid build output root",
        )?;
        let temporary_root = canonical_unlinked_directory(
            &definition.temporary_root,
            "local_check.profile.clippy.temp_root_invalid",
            "Cargo clippy handler requires a valid temporary root",
        )?;

        validate_disjoint_roots(&[
            &repository_root,
            &toolchain_root,
            &dependency_cache_root,
            &build_output_root,
            &temporary_root,
        ])?;

        let cargo_executable =
            canonical_clippy_executable(&definition.cargo_executable, &toolchain_root, "cargo")?;
        let cargo_clippy_executable = canonical_clippy_executable(
            &definition.cargo_clippy_executable,
            &toolchain_root,
            "cargo-clippy",
        )?;
        let clippy_driver_executable = canonical_clippy_executable(
            &definition.clippy_driver_executable,
            &toolchain_root,
            "clippy-driver",
        )?;
        let rustc_executable =
            canonical_clippy_executable(&definition.rustc_executable, &toolchain_root, "rustc")?;
        let rustdoc_executable = canonical_clippy_executable(
            &definition.rustdoc_executable,
            &toolchain_root,
            "rustdoc",
        )?;
        if definition.additional_toolchain_executables.is_empty() {
            return Err(clippy_identity_unavailable());
        }
        let mut additional_toolchain_executables = definition
            .additional_toolchain_executables
            .iter()
            .map(|path| canonical_additional_executable(path, &toolchain_root))
            .collect::<Result<Vec<_>, _>>()?;
        additional_toolchain_executables.sort();
        additional_toolchain_executables.dedup();
        if additional_toolchain_executables.len()
            != definition.additional_toolchain_executables.len()
        {
            return Err(clippy_identity_unavailable());
        }

        let mut dependency_inputs = definition
            .dependency_inputs
            .iter()
            .map(|path| canonical_regular_file_in_root(path, &dependency_cache_root))
            .collect::<Result<Vec<_>, _>>()?;
        dependency_inputs.sort();
        dependency_inputs.dedup();
        if dependency_inputs.is_empty()
            || dependency_inputs.len() != definition.dependency_inputs.len()
        {
            return Err(clippy_dependency_unavailable());
        }

        let writable_roots = vec![
            writable_root(
                CargoClippyWritableRootRole::Cache,
                dependency_cache_root.clone(),
            )?,
            writable_root(
                CargoClippyWritableRootRole::BuildOutput,
                build_output_root.clone(),
            )?,
            writable_root(
                CargoClippyWritableRootRole::Temporary,
                temporary_root.clone(),
            )?,
        ];
        validate_writable_root_roles(&writable_roots)?;

        let toolchain_commitment = toolchain_commitment(
            &toolchain_root,
            &cargo_executable,
            &cargo_clippy_executable,
            &clippy_driver_executable,
            &rustc_executable,
            &rustdoc_executable,
            &additional_toolchain_executables,
        )?;
        let dependency_commitment =
            dependency_commitment(&dependency_cache_root, &dependency_inputs)?;
        let protected_source_commitment = protected_repository_commitment(&repository_root)?;
        let execution_identity = execution_identity(
            &contract,
            &toolchain_commitment,
            &dependency_commitment,
            &protected_source_commitment,
            &writable_roots,
        );

        Ok(Self {
            contract,
            cargo_executable,
            cargo_clippy_executable,
            clippy_driver_executable,
            rustc_executable,
            rustdoc_executable,
            additional_toolchain_executables,
            toolchain_root,
            repository_root,
            dependency_cache_root,
            build_output_root,
            temporary_root,
            dependency_inputs,
            writable_roots,
            toolchain_commitment,
            dependency_commitment,
            protected_source_commitment,
            execution_identity,
            process_runner,
        })
    }

    fn build_process_request(&self) -> Result<LocalCheckProcessRequest, WorkflowOsError> {
        self.validate_current_identity()?;
        LocalCheckProcessRequest::new(
            self.cargo_executable.clone(),
            self.contract.arguments().to_vec(),
            self.repository_root.clone(),
            self.environment()?,
            Duration::from_secs(u64::from(self.contract.timeout_seconds())),
        )
    }

    fn run_process(
        &self,
        request: &LocalCheckProcessRequest,
    ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
        self.validate_current_identity()?;
        let output = self.process_runner.run(request);
        self.validate_current_identity()?;
        output
    }

    fn execute(&self) -> Result<LocalCheckResult, WorkflowOsError> {
        let request = self.build_process_request()?;
        let output = self.run_process(&request)?;
        LocalCheckResult::from_process_output(&self.contract, &output)
    }

    fn environment(&self) -> Result<BTreeMap<String, String>, WorkflowOsError> {
        let mut path_directories = BTreeSet::new();
        for executable in self.all_toolchain_executables() {
            let parent = executable
                .parent()
                .ok_or_else(clippy_identity_unavailable)?;
            path_directories.insert(parent.to_path_buf());
        }
        let path = std::env::join_paths(path_directories).map_err(|_| {
            clippy_error(
                WorkflowOsErrorKind::Security,
                "local_check.profile.clippy.environment_invalid",
                "Cargo clippy environment is invalid",
            )
        })?;
        let roots = self.roots_by_role()?;
        let mut environment = BTreeMap::new();
        environment.insert(
            "CARGO_HOME".to_owned(),
            self.dependency_cache_root.to_string_lossy().into_owned(),
        );
        environment.insert("CARGO_NET_OFFLINE".to_owned(), "true".to_owned());
        environment.insert(
            "CARGO_TARGET_DIR".to_owned(),
            roots
                .get(&CargoClippyWritableRootRole::BuildOutput)
                .ok_or_else(clippy_root_roles_invalid)?
                .to_string_lossy()
                .into_owned(),
        );
        environment.insert("PATH".to_owned(), path.to_string_lossy().into_owned());
        environment.insert(
            "RUSTC".to_owned(),
            self.rustc_executable.to_string_lossy().into_owned(),
        );
        environment.insert(
            "RUSTDOC".to_owned(),
            self.rustdoc_executable.to_string_lossy().into_owned(),
        );
        environment.insert(
            "RUSTUP_HOME".to_owned(),
            self.toolchain_root.to_string_lossy().into_owned(),
        );
        environment.insert(
            "TMPDIR".to_owned(),
            roots
                .get(&CargoClippyWritableRootRole::Temporary)
                .ok_or_else(clippy_root_roles_invalid)?
                .to_string_lossy()
                .into_owned(),
        );
        let allowed = self
            .contract
            .allowed_environment_variables()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let actual = environment.keys().map(String::as_str).collect::<Vec<_>>();
        if actual != allowed {
            return Err(clippy_error(
                WorkflowOsErrorKind::Security,
                "local_check.profile.clippy.environment_invalid",
                "Cargo clippy environment does not match its closed contract",
            ));
        }
        Ok(environment)
    }

    fn all_toolchain_executables(&self) -> Vec<&Path> {
        let mut values = vec![
            self.cargo_executable.as_path(),
            self.cargo_clippy_executable.as_path(),
            self.clippy_driver_executable.as_path(),
            self.rustc_executable.as_path(),
            self.rustdoc_executable.as_path(),
        ];
        values.extend(
            self.additional_toolchain_executables
                .iter()
                .map(PathBuf::as_path),
        );
        values
    }

    fn roots_by_role(
        &self,
    ) -> Result<BTreeMap<CargoClippyWritableRootRole, &Path>, WorkflowOsError> {
        validate_writable_root_roles(&self.writable_roots)?;
        Ok(self
            .writable_roots
            .iter()
            .map(|root| (root.role, root.path.as_path()))
            .collect())
    }

    fn validate_current_identity(&self) -> Result<(), WorkflowOsError> {
        validate_writable_root_roles(&self.writable_roots)?;
        let current_repository = validate_current_root(&self.repository_root)?;
        let current_toolchain_root = validate_current_root(&self.toolchain_root)?;
        let current_dependency_cache = validate_current_root(&self.dependency_cache_root)?;
        let current_build_output = validate_current_root(&self.build_output_root)?;
        let current_temporary = validate_current_root(&self.temporary_root)?;
        if self.writable_roots[0].path != self.dependency_cache_root
            || self.writable_roots[1].path != self.build_output_root
            || self.writable_roots[2].path != self.temporary_root
        {
            return Err(clippy_root_roles_invalid());
        }
        validate_disjoint_roots(&[
            &current_repository,
            &current_toolchain_root,
            &current_dependency_cache,
            &current_build_output,
            &current_temporary,
        ])?;
        let current_toolchain = toolchain_commitment(
            &self.toolchain_root,
            &self.cargo_executable,
            &self.cargo_clippy_executable,
            &self.clippy_driver_executable,
            &self.rustc_executable,
            &self.rustdoc_executable,
            &self.additional_toolchain_executables,
        )?;
        if current_toolchain != self.toolchain_commitment {
            return Err(clippy_identity_changed());
        }
        let current_dependencies =
            dependency_commitment(&self.dependency_cache_root, &self.dependency_inputs)?;
        if current_dependencies != self.dependency_commitment {
            return Err(clippy_dependency_changed());
        }
        let current_source = protected_repository_commitment(&self.repository_root)?;
        if current_source != self.protected_source_commitment {
            return Err(super::protected_state_changed_error());
        }
        if execution_identity(
            &self.contract,
            &current_toolchain,
            &current_dependencies,
            &current_source,
            &self.writable_roots,
        ) != self.execution_identity
        {
            return Err(clippy_identity_changed());
        }
        Ok(())
    }
}

impl fmt::Debug for CargoClippyWorkspaceLocalHandler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CargoClippyWorkspaceLocalHandler")
            .field("command_kind", &self.contract.command_kind())
            .field("toolchain", &"[REDACTED]")
            .field("repository", &"[REDACTED]")
            .field("dependencies", &"[REDACTED]")
            .field("writable_root_count", &self.writable_roots.len())
            .field("execution_identity", &"[REDACTED]")
            .field("process_runner", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl SkillHandler for CargoClippyWorkspaceLocalHandler {
    fn invoke(&self, _input: SkillInput) -> Result<SkillOutput, WorkflowOsError> {
        Ok(self.execute()?.to_skill_output())
    }
}

impl AuthoritativeLocalCheckHandler for CargoClippyWorkspaceLocalHandler {
    fn contract(&self) -> &LocalCheckCommandContract {
        &self.contract
    }

    fn skill_id(&self) -> &'static str {
        "local/check-cargo-clippy"
    }

    fn skill_version(&self) -> &'static str {
        "v0"
    }

    #[cfg(feature = "trusted-host-application-spi")]
    fn executable_content_hash(&self) -> &SpecContentHash {
        &self.execution_identity
    }

    fn build_process_request(&self) -> Result<LocalCheckProcessRequest, WorkflowOsError> {
        self.build_process_request()
    }

    fn run_process(
        &self,
        request: &LocalCheckProcessRequest,
    ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
        self.run_process(request)
    }
}

fn writable_root(
    role: CargoClippyWritableRootRole,
    path: PathBuf,
) -> Result<CargoClippyWritableRoot, WorkflowOsError> {
    Ok(CargoClippyWritableRoot {
        role,
        initial_commitment: directory_commitment(&path, b"writable-root-initial/v1")?,
        path,
    })
}

fn validate_writable_root_roles(roots: &[CargoClippyWritableRoot]) -> Result<(), WorkflowOsError> {
    let expected = [
        CargoClippyWritableRootRole::Cache,
        CargoClippyWritableRootRole::BuildOutput,
        CargoClippyWritableRootRole::Temporary,
    ];
    if roots.len() != expected.len()
        || roots.iter().map(|root| root.role).collect::<Vec<_>>() != expected
    {
        return Err(clippy_root_roles_invalid());
    }
    Ok(())
}

fn validate_disjoint_roots(roots: &[&Path]) -> Result<(), WorkflowOsError> {
    for (index, left) in roots.iter().enumerate() {
        for right in roots.iter().skip(index + 1) {
            if left.starts_with(right) || right.starts_with(left) {
                return Err(clippy_error(
                    WorkflowOsErrorKind::Security,
                    "local_check.profile.clippy.root_overlap",
                    "Cargo clippy roots must be mutually disjoint",
                ));
            }
        }
    }
    Ok(())
}

fn canonical_unlinked_directory(
    path: &Path,
    code: &'static str,
    message: &'static str,
) -> Result<PathBuf, WorkflowOsError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| clippy_error(WorkflowOsErrorKind::Validation, code, message))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(clippy_error(WorkflowOsErrorKind::Validation, code, message));
    }
    path.canonicalize()
        .map_err(|_| clippy_error(WorkflowOsErrorKind::Validation, code, message))
}

fn validate_current_root(expected: &Path) -> Result<PathBuf, WorkflowOsError> {
    let metadata = fs::symlink_metadata(expected).map_err(|_| clippy_root_identity_changed())?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(clippy_root_identity_changed());
    }
    let current = expected
        .canonicalize()
        .map_err(|_| clippy_root_identity_changed())?;
    if current != expected {
        return Err(clippy_root_identity_changed());
    }
    Ok(current)
}

fn canonical_clippy_executable(
    path: &Path,
    root: &Path,
    expected_name: &str,
) -> Result<PathBuf, WorkflowOsError> {
    if path.file_name().and_then(|name| name.to_str()) != Some(expected_name) {
        return Err(clippy_identity_unavailable());
    }
    canonical_additional_executable(path, root)
}

fn canonical_additional_executable(path: &Path, root: &Path) -> Result<PathBuf, WorkflowOsError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| clippy_identity_unavailable())?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(clippy_identity_unavailable());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o111 == 0 {
            return Err(clippy_identity_unavailable());
        }
    }
    let canonical = path
        .canonicalize()
        .map_err(|_| clippy_identity_unavailable())?;
    if !canonical.starts_with(root) {
        return Err(clippy_identity_unavailable());
    }
    Ok(canonical)
}

fn canonical_regular_file_in_root(path: &Path, root: &Path) -> Result<PathBuf, WorkflowOsError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| clippy_dependency_unavailable())?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(clippy_dependency_unavailable());
    }
    let canonical = path
        .canonicalize()
        .map_err(|_| clippy_dependency_unavailable())?;
    if !canonical.starts_with(root) {
        return Err(clippy_dependency_unavailable());
    }
    Ok(canonical)
}

fn toolchain_commitment(
    root: &Path,
    cargo: &Path,
    cargo_clippy: &Path,
    clippy_driver: &Path,
    rustc: &Path,
    rustdoc: &Path,
    additional: &[PathBuf],
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_private_commitment_field(
        &mut hasher,
        b"domain",
        b"workflow-os/cargo-clippy-toolchain/v1",
    );
    for (label, path) in [
        (b"cargo".as_slice(), cargo),
        (b"cargo_clippy".as_slice(), cargo_clippy),
        (b"clippy_driver".as_slice(), clippy_driver),
        (b"rustc".as_slice(), rustc),
        (b"rustdoc".as_slice(), rustdoc),
    ] {
        commit_input_file(&mut hasher, root, label, path, clippy_identity_unavailable)?;
    }
    for path in additional {
        commit_input_file(
            &mut hasher,
            root,
            b"additional_tool",
            path,
            clippy_identity_unavailable,
        )?;
    }
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn dependency_commitment(
    root: &Path,
    dependencies: &[PathBuf],
) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_private_commitment_field(
        &mut hasher,
        b"domain",
        b"workflow-os/cargo-clippy-dependencies/v1",
    );
    for path in dependencies {
        commit_input_file(
            &mut hasher,
            root,
            b"dependency",
            path,
            clippy_dependency_unavailable,
        )?;
    }
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn commit_input_file(
    hasher: &mut Sha256,
    root: &Path,
    label: &[u8],
    path: &Path,
    unavailable: fn() -> WorkflowOsError,
) -> Result<(), WorkflowOsError> {
    let relative = path.strip_prefix(root).map_err(|_| unavailable())?;
    let metadata = fs::symlink_metadata(path).map_err(|_| unavailable())?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(unavailable());
    }
    hash_private_commitment_field(hasher, b"kind", label);
    hash_private_commitment_field(
        hasher,
        b"relative_path",
        relative.as_os_str().as_encoded_bytes(),
    );
    let digest = file_content_hash(
        path,
        "local_check.profile.clippy.input_unavailable",
        "Cargo clippy input identity is unavailable",
    )?;
    hash_private_commitment_field(hasher, b"content", digest.as_str().as_bytes());
    Ok(())
}

fn execution_identity(
    contract: &LocalCheckCommandContract,
    toolchain: &SpecContentHash,
    dependencies: &SpecContentHash,
    source: &SpecContentHash,
    writable_roots: &[CargoClippyWritableRoot],
) -> SpecContentHash {
    let mut hasher = Sha256::new();
    hash_private_commitment_field(
        &mut hasher,
        b"domain",
        b"workflow-os/cargo-clippy-resolved-execution/v1",
    );
    hash_private_commitment_field(
        &mut hasher,
        b"command_id",
        contract.command_id().as_str().as_bytes(),
    );
    hash_private_commitment_field(&mut hasher, b"toolchain", toolchain.as_str().as_bytes());
    hash_private_commitment_field(
        &mut hasher,
        b"dependencies",
        dependencies.as_str().as_bytes(),
    );
    hash_private_commitment_field(&mut hasher, b"source", source.as_str().as_bytes());
    for root in writable_roots {
        hash_private_commitment_field(&mut hasher, b"root_role", root.role.label());
        hash_private_commitment_field(
            &mut hasher,
            b"root_path",
            root.path.as_os_str().as_encoded_bytes(),
        );
        hash_private_commitment_field(
            &mut hasher,
            b"root_initial",
            root.initial_commitment.as_str().as_bytes(),
        );
    }
    SpecContentHash::from_bytes(hasher.finalize())
}

fn protected_repository_commitment(root: &Path) -> Result<SpecContentHash, WorkflowOsError> {
    directory_commitment(root, b"cargo-clippy-protected-repository/v1")
}

fn directory_commitment(root: &Path, domain: &[u8]) -> Result<SpecContentHash, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hash_private_commitment_field(&mut hasher, b"domain", domain);
    commit_directory(root, root, &mut hasher)?;
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn commit_directory(
    root: &Path,
    directory: &Path,
    hasher: &mut Sha256,
) -> Result<(), WorkflowOsError> {
    let mut entries = fs::read_dir(directory)
        .map_err(|_| super::protected_state_unavailable_error())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| super::protected_state_unavailable_error())?;
    entries.sort_by(|left, right| {
        left.file_name()
            .as_encoded_bytes()
            .cmp(right.file_name().as_encoded_bytes())
    });
    for entry in entries {
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|_| super::protected_state_unavailable_error())?;
        if protected_repository_excluded(relative) {
            continue;
        }
        let metadata =
            fs::symlink_metadata(&path).map_err(|_| super::protected_state_unavailable_error())?;
        if metadata.file_type().is_symlink() {
            return Err(clippy_error(
                WorkflowOsErrorKind::Security,
                "local_check.profile.clippy.symlink_unsupported",
                "Cargo clippy identity inventories cannot contain symbolic links",
            ));
        }
        if metadata.is_dir() {
            hash_private_commitment_field(
                hasher,
                b"directory",
                relative.as_os_str().as_encoded_bytes(),
            );
            commit_directory(root, &path, hasher)?;
        } else if metadata.is_file() {
            hash_private_commitment_field(hasher, b"file", relative.as_os_str().as_encoded_bytes());
            let digest = file_content_hash(
                &path,
                "local_check.profile.clippy.input_unavailable",
                "Cargo clippy input identity is unavailable",
            )?;
            hash_private_commitment_field(hasher, b"content", digest.as_str().as_bytes());
        } else {
            return Err(super::protected_state_unavailable_error());
        }
    }
    Ok(())
}

fn protected_repository_excluded(relative: &Path) -> bool {
    [".git", ".workflow-os", "dogfood/.workflow-os"]
        .iter()
        .map(Path::new)
        .any(|excluded| relative == excluded || relative.starts_with(excluded))
}

fn clippy_error(
    kind: WorkflowOsErrorKind,
    code: &'static str,
    message: &'static str,
) -> WorkflowOsError {
    local_check_error(kind, code, message)
}

fn clippy_identity_unavailable() -> WorkflowOsError {
    clippy_error(
        WorkflowOsErrorKind::Security,
        "local_check.profile.clippy.toolchain_identity_unavailable",
        "Cargo clippy toolchain identity is unavailable",
    )
}

fn clippy_identity_changed() -> WorkflowOsError {
    clippy_error(
        WorkflowOsErrorKind::Security,
        "local_check.profile.clippy.toolchain_identity_changed",
        "Cargo clippy toolchain identity changed after resolution",
    )
}

fn clippy_dependency_unavailable() -> WorkflowOsError {
    clippy_error(
        WorkflowOsErrorKind::Security,
        "local_check.profile.clippy.dependency_identity_unavailable",
        "Cargo clippy dependency identity is unavailable",
    )
}

fn clippy_dependency_changed() -> WorkflowOsError {
    clippy_error(
        WorkflowOsErrorKind::Security,
        "local_check.profile.clippy.dependency_identity_changed",
        "Cargo clippy dependency identity changed after resolution",
    )
}

fn clippy_root_roles_invalid() -> WorkflowOsError {
    clippy_error(
        WorkflowOsErrorKind::Security,
        "local_check.profile.clippy.writable_root_roles_invalid",
        "Cargo clippy writable root roles are invalid",
    )
}

fn clippy_root_identity_changed() -> WorkflowOsError {
    clippy_error(
        WorkflowOsErrorKind::Security,
        "local_check.profile.clippy.root_identity_changed",
        "Cargo clippy root identity changed after resolution",
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::expect_used)]
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    use super::*;
    use crate::{
        ExplicitLocalCheckProfileId, ExplicitLocalCheckProfileSelection, LocalCheckResultStatus,
    };

    static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(1);

    #[derive(Debug)]
    struct FakeRunner {
        output: LocalCheckProcessOutput,
        request: Mutex<Option<LocalCheckProcessRequest>>,
        mutation: Option<(PathBuf, &'static [u8])>,
    }

    impl FakeRunner {
        fn success() -> Self {
            Self {
                output: LocalCheckProcessOutput::completed(
                    Some(0),
                    true,
                    11,
                    Vec::new(),
                    Vec::new(),
                ),
                request: Mutex::new(None),
                mutation: None,
            }
        }

        fn with_output(output: LocalCheckProcessOutput) -> Self {
            Self {
                output,
                request: Mutex::new(None),
                mutation: None,
            }
        }

        fn mutating(path: PathBuf) -> Self {
            Self {
                mutation: Some((path, b"changed\n")),
                ..Self::success()
            }
        }
    }

    impl LocalCheckProcessRunner for FakeRunner {
        fn run(
            &self,
            request: &LocalCheckProcessRequest,
        ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
            *self.request.lock().expect("request lock") = Some(request.clone());
            if let Some((path, content)) = &self.mutation {
                fs::write(path, content).expect("fixture mutation");
            }
            Ok(self.output.clone())
        }
    }

    #[cfg(unix)]
    #[derive(Debug)]
    struct RootReplacingRunner {
        root: PathBuf,
    }

    #[cfg(unix)]
    impl LocalCheckProcessRunner for RootReplacingRunner {
        fn run(
            &self,
            _request: &LocalCheckProcessRequest,
        ) -> Result<LocalCheckProcessOutput, WorkflowOsError> {
            replace_root_with_symlink(&self.root);
            Ok(LocalCheckProcessOutput::completed(
                Some(0),
                true,
                11,
                Vec::new(),
                Vec::new(),
            ))
        }
    }

    struct Fixture {
        root: PathBuf,
        repository: PathBuf,
        toolchain: PathBuf,
        cache: PathBuf,
        target: PathBuf,
        temp: PathBuf,
        cargo: PathBuf,
        cargo_clippy: PathBuf,
        clippy_driver: PathBuf,
        rustc: PathBuf,
        rustdoc: PathBuf,
        platform_tool: PathBuf,
        dependency: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "workflow-os-cargo-clippy-profile-{}-{}",
                std::process::id(),
                FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            let repository = root.join("repository");
            let toolchain = root.join("toolchain");
            let cache = root.join("cargo-cache");
            let target = root.join("cargo-target");
            let temp = root.join("cargo-temp");
            let bin = toolchain.join("bin");
            fs::create_dir_all(repository.join("src")).expect("repository fixture");
            fs::create_dir_all(&bin).expect("toolchain fixture");
            fs::create_dir_all(cache.join("registry/src/package")).expect("cache fixture");
            fs::create_dir_all(&target).expect("target fixture");
            fs::create_dir_all(&temp).expect("temp fixture");
            fs::write(
                repository.join("Cargo.toml"),
                "[workspace]\nmembers = [\".\"]\n[package]\nname = \"fixture\"\nversion = \"0.0.0\"\n",
            )
            .expect("manifest fixture");
            fs::write(repository.join("Cargo.lock"), "version = 4\n").expect("lock fixture");
            fs::write(repository.join("build.rs"), "fn main() {}\n").expect("build fixture");
            fs::write(
                repository.join("src/lib.rs"),
                "pub fn value() -> u8 { 1 }\n",
            )
            .expect("source fixture");

            let cargo = bin.join("cargo");
            let cargo_clippy = bin.join("cargo-clippy");
            let clippy_driver = bin.join("clippy-driver");
            let rustc = bin.join("rustc");
            let rustdoc = bin.join("rustdoc");
            let platform_tool = bin.join("cc");
            for executable in [
                &cargo,
                &cargo_clippy,
                &clippy_driver,
                &rustc,
                &rustdoc,
                &platform_tool,
            ] {
                write_executable(executable);
            }
            let dependency = cache.join("registry/src/package/lib.rs");
            fs::write(&dependency, "pub fn dependency() {}\n").expect("dependency fixture");

            Self {
                root,
                repository,
                toolchain,
                cache,
                target,
                temp,
                cargo,
                cargo_clippy,
                clippy_driver,
                rustc,
                rustdoc,
                platform_tool,
                dependency,
            }
        }

        fn definition(&self) -> CargoClippyWorkspaceHandlerDefinition {
            CargoClippyWorkspaceHandlerDefinition {
                cargo_executable: self.cargo.clone(),
                cargo_clippy_executable: self.cargo_clippy.clone(),
                clippy_driver_executable: self.clippy_driver.clone(),
                rustc_executable: self.rustc.clone(),
                rustdoc_executable: self.rustdoc.clone(),
                additional_toolchain_executables: vec![self.platform_tool.clone()],
                toolchain_root: self.toolchain.clone(),
                repository_root: self.repository.clone(),
                dependency_cache_root: self.cache.clone(),
                dependency_inputs: vec![self.dependency.clone()],
                build_output_root: self.target.clone(),
                temporary_root: self.temp.clone(),
            }
        }

        fn handler(
            &self,
            runner: Arc<dyn LocalCheckProcessRunner>,
        ) -> Result<CargoClippyWorkspaceLocalHandler, WorkflowOsError> {
            CargoClippyWorkspaceLocalHandler::new_with_process_runner(
                LocalCheckCommandContract::cargo_clippy_workspace_model_only()?,
                &self.definition(),
                runner,
            )
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn write_executable(path: &Path) {
        fs::write(path, "#!/bin/sh\nexit 0\n").expect("executable fixture");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mut permissions = fs::metadata(path).expect("metadata").permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(path, permissions).expect("permissions");
        }
    }

    #[cfg(unix)]
    fn replace_root_with_symlink(path: &Path) {
        use std::os::unix::fs::symlink;

        let replacement = path.with_extension("resolved-root");
        fs::rename(path, &replacement).expect("move original root");
        symlink(&replacement, path).expect("replace root with symlink");
    }

    #[test]
    fn canonical_contract_and_profile_vocabulary_are_exact_and_non_executing() {
        let contract = LocalCheckCommandContract::cargo_clippy_workspace_model_only()
            .expect("canonical contract");
        let selection = ExplicitLocalCheckProfileSelection::cargo_clippy_workspace();

        assert_eq!(
            selection.profile_id(),
            ExplicitLocalCheckProfileId::CargoClippyWorkspace
        );
        assert_eq!(
            contract.command_kind(),
            LocalCheckCommandKind::CargoClippyWorkspace
        );
        assert_eq!(
            contract.arguments(),
            [
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings"
            ]
        );
        assert_eq!(
            contract.side_effect_boundary().allowed_effects(),
            [
                super::super::LocalCheckSideEffectKind::SourceReadOnly,
                super::super::LocalCheckSideEffectKind::CacheWriteOnly,
                super::super::LocalCheckSideEffectKind::BuildOutputWrite,
                super::super::LocalCheckSideEffectKind::TempWriteOnly,
            ]
        );
    }

    #[test]
    fn injected_handler_builds_exact_offline_request_and_returns_payload_free_result() {
        let fixture = Fixture::new();
        let runner = Arc::new(FakeRunner::success());
        let handler = fixture
            .handler(Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>)
            .expect("handler resolves");

        assert!(runner.request.lock().expect("request lock").is_none());
        let result = handler.execute().expect("injected result");
        let request = runner
            .request
            .lock()
            .expect("request lock")
            .clone()
            .expect("request captured");

        assert_eq!(result.status(), LocalCheckResultStatus::Passed);
        assert_eq!(
            request.arguments(),
            [
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings"
            ]
        );
        assert_eq!(
            request
                .environment()
                .get("CARGO_NET_OFFLINE")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(request.environment().len(), 8);
        assert_eq!(
            request.environment().get("CARGO_HOME").map(String::as_str),
            Some(handler.dependency_cache_root.to_string_lossy().as_ref())
        );
        assert_eq!(
            request
                .environment()
                .get("CARGO_TARGET_DIR")
                .map(String::as_str),
            Some(handler.writable_roots[1].path.to_string_lossy().as_ref())
        );
        assert_eq!(
            request.environment().get("RUSTUP_HOME").map(String::as_str),
            Some(handler.toolchain_root.to_string_lossy().as_ref())
        );
        assert_eq!(
            request.environment().get("TMPDIR").map(String::as_str),
            Some(handler.writable_roots[2].path.to_string_lossy().as_ref())
        );
        assert_eq!(
            request.environment().get("RUSTC").map(String::as_str),
            Some(handler.rustc_executable.to_string_lossy().as_ref())
        );
        assert_eq!(
            request.environment().get("RUSTDOC").map(String::as_str),
            Some(handler.rustdoc_executable.to_string_lossy().as_ref())
        );
        assert!(!request.environment().contains_key("HOME"));
        assert!(!request.environment().contains_key("HTTP_PROXY"));
        assert_eq!(handler.skill_id(), "local/check-cargo-clippy");
        assert_eq!(handler.skill_version(), "v0");
        let serialized = serde_json::to_string(&result).expect("result serializes");
        assert!(!serialized.contains(&fixture.root.to_string_lossy().into_owned()));
        assert!(!format!("{handler:?}").contains(&fixture.root.to_string_lossy().into_owned()));
    }

    #[test]
    fn injected_handler_maps_nonzero_and_timeout_without_command_output_evidence() {
        for (output, expected) in [
            (
                LocalCheckProcessOutput::completed(Some(9), false, 7, Vec::new(), Vec::new()),
                LocalCheckResultStatus::Failed,
            ),
            (
                LocalCheckProcessOutput::timed_out(900_000, Vec::new(), Vec::new()),
                LocalCheckResultStatus::TimedOut,
            ),
        ] {
            let fixture = Fixture::new();
            let handler = fixture
                .handler(Arc::new(FakeRunner::with_output(output)))
                .expect("handler resolves");
            let result = handler.execute().expect("bounded result");
            assert_eq!(result.status(), expected);
            assert!(result.stdout_summary().is_empty());
            assert!(result.stderr_summary().is_empty());
        }
    }

    #[test]
    fn invalid_or_incomplete_identity_fails_before_runner_entry() {
        let fixture = Fixture::new();
        let runner = Arc::new(FakeRunner::success());
        let mut definition = fixture.definition();
        definition.additional_toolchain_executables.clear();
        let error = CargoClippyWorkspaceLocalHandler::new_with_process_runner(
            LocalCheckCommandContract::cargo_clippy_workspace_model_only()
                .expect("canonical contract"),
            &definition,
            Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
        )
        .expect_err("incomplete identity fails");

        assert_eq!(
            error.code(),
            "local_check.profile.clippy.toolchain_identity_unavailable"
        );
        assert!(runner.request.lock().expect("request lock").is_none());
        assert!(!error
            .to_string()
            .contains(&fixture.root.to_string_lossy().into_owned()));
    }

    #[test]
    fn empty_and_duplicate_toolchain_and_dependency_inventories_fail_closed() {
        let fixture = Fixture::new();
        let runner = Arc::new(FakeRunner::success());

        let mut empty_dependencies = fixture.definition();
        empty_dependencies.dependency_inputs.clear();
        let error = CargoClippyWorkspaceLocalHandler::new_with_process_runner(
            LocalCheckCommandContract::cargo_clippy_workspace_model_only()
                .expect("canonical contract"),
            &empty_dependencies,
            Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
        )
        .expect_err("empty dependency inventory fails");
        assert_eq!(
            error.code(),
            "local_check.profile.clippy.dependency_identity_unavailable"
        );

        let mut duplicate_dependencies = fixture.definition();
        duplicate_dependencies
            .dependency_inputs
            .push(fixture.dependency.clone());
        let error = CargoClippyWorkspaceLocalHandler::new_with_process_runner(
            LocalCheckCommandContract::cargo_clippy_workspace_model_only()
                .expect("canonical contract"),
            &duplicate_dependencies,
            Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
        )
        .expect_err("duplicate dependency inventory fails");
        assert_eq!(
            error.code(),
            "local_check.profile.clippy.dependency_identity_unavailable"
        );

        let mut duplicate_tools = fixture.definition();
        duplicate_tools
            .additional_toolchain_executables
            .push(fixture.platform_tool.clone());
        let error = CargoClippyWorkspaceLocalHandler::new_with_process_runner(
            LocalCheckCommandContract::cargo_clippy_workspace_model_only()
                .expect("canonical contract"),
            &duplicate_tools,
            Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
        )
        .expect_err("duplicate toolchain inventory fails");
        assert_eq!(
            error.code(),
            "local_check.profile.clippy.toolchain_identity_unavailable"
        );
        assert!(runner.request.lock().expect("request lock").is_none());
    }

    #[test]
    fn root_overlap_and_role_substitution_fail_closed() {
        let fixture = Fixture::new();
        let runner = Arc::new(FakeRunner::success());
        let mut definition = fixture.definition();
        definition.temporary_root = fixture.target.clone();
        let error = CargoClippyWorkspaceLocalHandler::new_with_process_runner(
            LocalCheckCommandContract::cargo_clippy_workspace_model_only()
                .expect("canonical contract"),
            &definition,
            Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
        )
        .expect_err("overlap fails");
        assert_eq!(error.code(), "local_check.profile.clippy.root_overlap");

        let mut handler = fixture
            .handler(Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>)
            .expect("handler resolves");
        handler.writable_roots.swap(0, 1);
        let error = handler
            .build_process_request()
            .expect_err("role substitution fails");
        assert_eq!(
            error.code(),
            "local_check.profile.clippy.writable_root_roles_invalid"
        );
        assert!(runner.request.lock().expect("request lock").is_none());
    }

    #[test]
    fn nested_root_overlap_fails_closed_in_both_directions() {
        for build_contains_temp in [true, false] {
            let fixture = Fixture::new();
            let runner = Arc::new(FakeRunner::success());
            let mut definition = fixture.definition();
            if build_contains_temp {
                definition.temporary_root = fixture.target.join("nested-temp");
                fs::create_dir_all(&definition.temporary_root).expect("nested temp fixture");
            } else {
                definition.build_output_root = fixture.temp.join("nested-build");
                fs::create_dir_all(&definition.build_output_root).expect("nested build fixture");
            }
            let error = CargoClippyWorkspaceLocalHandler::new_with_process_runner(
                LocalCheckCommandContract::cargo_clippy_workspace_model_only()
                    .expect("canonical contract"),
                &definition,
                Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>,
            )
            .expect_err("nested roots fail");
            assert_eq!(error.code(), "local_check.profile.clippy.root_overlap");
            assert!(runner.request.lock().expect("request lock").is_none());
        }
    }

    #[cfg(unix)]
    #[test]
    fn every_root_replacement_fails_before_runner_entry() {
        for root_kind in ["repository", "toolchain", "cache", "build", "temp"] {
            let fixture = Fixture::new();
            let runner = Arc::new(FakeRunner::success());
            let handler = fixture
                .handler(Arc::clone(&runner) as Arc<dyn LocalCheckProcessRunner>)
                .expect("handler resolves");
            let root = match root_kind {
                "repository" => fixture.repository.clone(),
                "toolchain" => fixture.toolchain.clone(),
                "cache" => fixture.cache.clone(),
                "build" => fixture.target.clone(),
                "temp" => fixture.temp.clone(),
                _ => unreachable!(),
            };
            replace_root_with_symlink(&root);

            let error = handler
                .build_process_request()
                .expect_err("replaced root fails before runner entry");
            assert_eq!(
                error.code(),
                "local_check.profile.clippy.root_identity_changed"
            );
            assert!(!error
                .to_string()
                .contains(&fixture.root.to_string_lossy().into_owned()));
            assert!(runner.request.lock().expect("request lock").is_none());
        }
    }

    #[cfg(unix)]
    #[test]
    fn every_root_replacement_fails_after_runner_entry() {
        for root_kind in ["repository", "toolchain", "cache", "build", "temp"] {
            let fixture = Fixture::new();
            let root = match root_kind {
                "repository" => fixture.repository.clone(),
                "toolchain" => fixture.toolchain.clone(),
                "cache" => fixture.cache.clone(),
                "build" => fixture.target.clone(),
                "temp" => fixture.temp.clone(),
                _ => unreachable!(),
            };
            let handler = fixture
                .handler(Arc::new(RootReplacingRunner { root }))
                .expect("handler resolves");

            let error = handler
                .execute()
                .expect_err("replaced root fails after runner entry");
            assert_eq!(
                error.code(),
                "local_check.profile.clippy.root_identity_changed"
            );
            assert!(!error
                .to_string()
                .contains(&fixture.root.to_string_lossy().into_owned()));
        }
    }

    #[test]
    fn source_toolchain_and_dependency_mutation_fail_closed_after_runner_entry() {
        for target in [
            "source",
            "manifest",
            "lockfile",
            "build_script",
            "toolchain",
            "platform_tool",
            "dependency",
        ] {
            let fixture = Fixture::new();
            let path = match target {
                "source" => fixture.repository.join("src/lib.rs"),
                "manifest" => fixture.repository.join("Cargo.toml"),
                "lockfile" => fixture.repository.join("Cargo.lock"),
                "build_script" => fixture.repository.join("build.rs"),
                "toolchain" => fixture.clippy_driver.clone(),
                "platform_tool" => fixture.platform_tool.clone(),
                "dependency" => fixture.dependency.clone(),
                _ => unreachable!(),
            };
            let handler = fixture
                .handler(Arc::new(FakeRunner::mutating(path)))
                .expect("handler resolves");
            let error = handler.execute().expect_err("identity drift fails closed");
            assert!(matches!(
                error.code(),
                "local_check.profile.protected_state_changed"
                    | "local_check.profile.clippy.toolchain_identity_changed"
                    | "local_check.profile.clippy.dependency_identity_changed"
            ));
            assert!(!error
                .to_string()
                .contains(&fixture.root.to_string_lossy().into_owned()));
        }
    }

    #[test]
    fn declared_cache_build_and_temp_mutations_are_accepted() {
        for path_kind in ["cache", "build", "temp"] {
            let fixture = Fixture::new();
            let path = match path_kind {
                "cache" => fixture.cache.join("new-cache-state"),
                "build" => fixture.target.join("new-build-state"),
                "temp" => fixture.temp.join("new-temp-state"),
                _ => unreachable!(),
            };
            let handler = fixture
                .handler(Arc::new(FakeRunner::mutating(path)))
                .expect("handler resolves");
            let result = handler
                .execute()
                .expect("declared writable root accepts mutation");
            assert_eq!(result.status(), LocalCheckResultStatus::Passed);
        }
    }

    #[test]
    fn public_production_resolvers_reject_clippy_selection() {
        let fixture = Fixture::new();
        let runner = Arc::new(FakeRunner::success());
        let error = ExplicitLocalCheckProfileSelection::cargo_clippy_workspace()
            .resolve_with_process_runner(
                fixture.cargo.clone(),
                fixture.repository.clone(),
                runner as Arc<dyn LocalCheckProcessRunner>,
            )
            .expect_err("generic production resolver rejects clippy");
        assert_eq!(error.code(), "local_check.profile.resolver_mismatch");
    }
}
