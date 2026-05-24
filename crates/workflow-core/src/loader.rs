use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{
    canonical_yaml_content_hash, parse_policy_spec_yaml, parse_project_manifest_yaml,
    parse_skill_spec_yaml, parse_test_spec_yaml, parse_workflow_spec_yaml, Diagnostic,
    DiagnosticSeverity, PolicySpecDocument, ProjectManifest, SkillDefinition, SourceLocation,
    SpecContentHash, TestSpecDocument, WorkflowDefinition, WorkflowOsError,
};

const MANIFEST_FILE_NAME: &str = "workflow-os.yml";

/// Result of loading a Workflow OS project.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectLoadResult {
    /// Loaded project bundle when the manifest could be parsed.
    pub bundle: Option<ProjectBundle>,
    /// Accumulated loader diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

impl ProjectLoadResult {
    /// Returns true when any diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Error)
    }
}

/// A loaded Workflow OS project and its raw parsed definitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectBundle {
    /// Project root directory.
    pub root: PathBuf,
    /// Loaded project manifest.
    pub manifest: LoadedSpec<ProjectManifest>,
    /// Loaded workflow definitions.
    pub workflows: Vec<LoadedSpec<WorkflowDefinition>>,
    /// Loaded skill definitions.
    pub skills: Vec<LoadedSpec<SkillDefinition>>,
    /// Loaded policy definitions.
    pub policies: Vec<LoadedSpec<PolicySpecDocument>>,
    /// Loaded test definitions.
    pub tests: Vec<LoadedSpec<TestSpecDocument>>,
}

/// Parsed spec plus deterministic loader metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadedSpec<T> {
    /// File path the spec was loaded from.
    pub path: PathBuf,
    /// Canonical content hash for the loaded spec.
    pub content_hash: SpecContentHash,
    /// Parsed definition.
    pub definition: T,
}

/// Loads a Workflow OS project from a root directory without executing anything.
///
/// # Errors
///
/// This function does not return `Result`; all discovery and parse problems are
/// accumulated into diagnostics. `bundle` is `None` only when `workflow-os.yml`
/// cannot be found or parsed.
#[must_use]
pub fn load_project(root: impl AsRef<Path>) -> ProjectLoadResult {
    let root = root.as_ref().to_path_buf();
    let manifest_path = root.join(MANIFEST_FILE_NAME);
    let mut diagnostics = Vec::new();

    if !manifest_path.is_file() {
        diagnostics.push(
            Diagnostic::error(
                "loader.manifest_missing",
                format!("missing required {MANIFEST_FILE_NAME}"),
            )
            .with_source_location(SourceLocation::new(&manifest_path)),
        );
        return ProjectLoadResult {
            bundle: None,
            diagnostics,
        };
    }

    let Some(manifest) = load_spec_file(
        &manifest_path,
        parse_project_manifest_yaml,
        &mut diagnostics,
    ) else {
        return ProjectLoadResult {
            bundle: None,
            diagnostics,
        };
    };

    let layout = manifest.definition.layout.clone();

    let mut workflows = load_spec_directory(
        &root,
        &layout.workflows,
        ".workflow.yml",
        parse_workflow_spec_yaml,
        &mut diagnostics,
    );
    let mut skills = load_spec_directory(
        &root,
        &layout.skills,
        ".skill.yml",
        parse_skill_spec_yaml,
        &mut diagnostics,
    );
    let policies = load_spec_directory(
        &root,
        &layout.policies,
        ".policy.yml",
        parse_policy_spec_yaml,
        &mut diagnostics,
    );
    let tests = load_spec_directory(
        &root,
        &layout.tests,
        ".test.yml",
        parse_test_spec_yaml,
        &mut diagnostics,
    );

    attach_workflow_source_locations(&mut workflows);
    attach_skill_source_locations(&mut skills);
    report_duplicate_ids(
        "loader.duplicate_workflow_id",
        "workflow",
        workflows
            .iter()
            .map(|loaded| (loaded.definition.id.as_str(), loaded.path.as_path())),
        &mut diagnostics,
    );
    report_duplicate_ids(
        "loader.duplicate_skill_id",
        "skill",
        skills
            .iter()
            .map(|loaded| (loaded.definition.id.as_str(), loaded.path.as_path())),
        &mut diagnostics,
    );
    report_duplicate_ids(
        "loader.duplicate_policy_id",
        "policy",
        policies
            .iter()
            .map(|loaded| (loaded.definition.id.as_str(), loaded.path.as_path())),
        &mut diagnostics,
    );
    report_duplicate_ids(
        "loader.duplicate_test_id",
        "test",
        tests
            .iter()
            .map(|loaded| (loaded.definition.id.as_str(), loaded.path.as_path())),
        &mut diagnostics,
    );

    ProjectLoadResult {
        bundle: Some(ProjectBundle {
            root,
            manifest,
            workflows,
            skills,
            policies,
            tests,
        }),
        diagnostics,
    }
}

fn load_spec_directory<T>(
    root: &Path,
    relative_dir: &str,
    suffix: &str,
    parser: fn(&str) -> Result<T, WorkflowOsError>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<LoadedSpec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let directory = root.join(relative_dir);
    if !directory.is_dir() {
        diagnostics.push(
            Diagnostic::warning(
                "loader.directory_missing",
                format!("spec directory {relative_dir} does not exist"),
            )
            .with_source_location(SourceLocation::new(directory)),
        );
        return Vec::new();
    }

    let mut paths = Vec::new();
    match fs::read_dir(&directory) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file()
                            && path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .is_some_and(|name| name.ends_with(suffix))
                        {
                            paths.push(path);
                        }
                    }
                    Err(error) => diagnostics.push(io_diagnostic(
                        "loader.discovery_entry",
                        directory.clone(),
                        format!("failed to inspect directory entry: {error}"),
                    )),
                }
            }
        }
        Err(error) => {
            diagnostics.push(io_diagnostic(
                "loader.discovery",
                directory,
                format!("failed to discover spec files: {error}"),
            ));
            return Vec::new();
        }
    }

    paths.sort();
    paths
        .iter()
        .filter_map(|path| load_spec_file(path, parser, diagnostics))
        .collect()
}

fn load_spec_file<T>(
    path: &Path,
    parser: fn(&str) -> Result<T, WorkflowOsError>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<LoadedSpec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            diagnostics.push(io_diagnostic(
                "loader.read",
                path.to_path_buf(),
                format!("failed to read spec file: {error}"),
            ));
            return None;
        }
    };

    if let Err(error) = serde_yaml::from_str::<serde_yaml::Value>(&source) {
        diagnostics.push(yaml_parse_diagnostic(path, &error));
        return None;
    }

    match parser(&source) {
        Ok(definition) => match canonical_yaml_content_hash(&source) {
            Ok(content_hash) => Some(LoadedSpec {
                path: path.to_path_buf(),
                content_hash,
                definition,
            }),
            Err(error) => {
                diagnostics.extend(error_to_diagnostics(path, &error));
                None
            }
        },
        Err(error) => {
            diagnostics.extend(error_to_diagnostics(path, &error));
            None
        }
    }
}

fn yaml_parse_diagnostic(path: &Path, error: &serde_yaml::Error) -> Diagnostic {
    let mut location = SourceLocation::new(path);
    if let Some(yaml_location) = error.location() {
        if let Ok(line) = u32::try_from(yaml_location.line()) {
            location = location.with_line(line);
        }
        if let Ok(column) = u32::try_from(yaml_location.column()) {
            location = location.with_column(column);
        }
    }
    Diagnostic::error("yaml.parse", format!("failed to parse YAML: {error}"))
        .with_source_location(location)
}

fn error_to_diagnostics(path: &Path, error: &WorkflowOsError) -> Vec<Diagnostic> {
    if error.diagnostics().is_empty() {
        return vec![error_diagnostic(path, error.code(), error.message())];
    }

    error
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            let location = diagnostic
                .source_location()
                .cloned()
                .unwrap_or_else(|| default_source_location(path, diagnostic.code()));
            Diagnostic::new(
                diagnostic.severity(),
                diagnostic.code().to_owned(),
                diagnostic.message().to_owned(),
            )
            .with_source_location(location)
        })
        .collect()
}

fn error_diagnostic(path: &Path, code: &str, message: &str) -> Diagnostic {
    Diagnostic::error(code.to_owned(), message.to_owned())
        .with_source_location(default_source_location(path, code))
}

fn default_source_location(path: &Path, code: &str) -> SourceLocation {
    let location = SourceLocation::new(path);
    match code {
        "schema_version.missing" | "schema_version.unsupported" => {
            location.with_document_path("$.schema_version")
        }
        "spec.secret_disallowed" => location.with_document_path("$"),
        _ => location,
    }
}

fn io_diagnostic(code: &str, path: PathBuf, message: String) -> Diagnostic {
    Diagnostic::error(code, message).with_source_location(SourceLocation::new(path))
}

fn report_duplicate_ids<'a>(
    code: &'static str,
    kind: &'static str,
    ids_and_paths: impl Iterator<Item = (&'a str, &'a Path)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut seen: BTreeMap<&str, &Path> = BTreeMap::new();
    let mut reported = BTreeSet::new();

    for (id, path) in ids_and_paths {
        if let Some(first_path) = seen.get(id) {
            if reported.insert((id.to_owned(), path.to_path_buf())) {
                diagnostics.push(
                    Diagnostic::error(
                        code,
                        format!(
                            "duplicate {kind} id {id}; first declared in {}",
                            first_path.display()
                        ),
                    )
                    .with_source_location(SourceLocation::new(path).with_document_path("$.id")),
                );
            }
        } else {
            seen.insert(id, path);
        }
    }
}

fn attach_workflow_source_locations(workflows: &mut [LoadedSpec<WorkflowDefinition>]) {
    for loaded in workflows {
        loaded.definition.source_location = Some(SourceLocation::new(&loaded.path));
    }
}

fn attach_skill_source_locations(skills: &mut [LoadedSpec<SkillDefinition>]) {
    for loaded in skills {
        loaded.definition.source_location = Some(SourceLocation::new(&loaded.path));
    }
}
