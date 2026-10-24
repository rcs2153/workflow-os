#![allow(clippy::expect_used, clippy::panic)]

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    discover_trusted_host_build_inputs_from_fixture, TrustedHostBuildInputFixtureDiscoveryInput,
    TrustedHostBuildInputFixtureDiscoveryLimits,
    TrustedHostBuildInputFixtureDiscoveryLimitsDefinition,
};
use crate::{
    BuildTimeInputKind, SpecContentHash, TrustedHostBuildInputUnsupportedReason,
    TrustedHostStaticGraphCompleteness,
};

static FIXTURE_COUNTER: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new(label: &str) -> Self {
        let ordinal = FIXTURE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "workflow-os-trusted-host-fixture-{label}-{}-{ordinal}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale fixture");
        }
        fs::create_dir_all(&root).expect("create fixture root");
        Self { root }
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent");
        }
        fs::write(path, contents).expect("write fixture file");
    }

    fn discover(
        &self,
    ) -> Result<super::TrustedHostBuildInputFixtureDiscovery, crate::WorkflowOsError> {
        let input = TrustedHostBuildInputFixtureDiscoveryInput::new(
            &self.root,
            SpecContentHash::from_text("fixture-repository"),
            SpecContentHash::from_text("fixture-selection"),
            TrustedHostBuildInputFixtureDiscoveryLimits::standard(),
        );
        discover_trusted_host_build_inputs_from_fixture(&input)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write_supported_workspace(fixture: &Fixture) {
    fixture.write(
        "Cargo.toml",
        r#"
[workspace]
members = ["crates/app", "crates/macro"]
resolver = "2"
"#,
    );
    fixture.write(
        "crates/app/Cargo.toml",
        r#"
[package]
name = "fixture-app"
version = "0.1.0"
edition = "2021"
rust-version = "1.78"
build = "build.rs"

[lib]
path = "src/lib.rs"

[dependencies]
fixture-macro = { path = "../macro", version = "0.1.0" }

[build-dependencies]
fixture-macro = { path = "../macro", version = "0.1.0" }

[dev-dependencies]
fixture-macro = { path = "../macro", version = "0.1.0" }

[features]
default = ["fixture-macro/example"]
"#,
    );
    fixture.write(
        "crates/macro/Cargo.toml",
        r#"
[package]
name = "fixture-macro"
version = "0.1.0"
edition = "2021"
build = false

[lib]
path = "src/lib.rs"
proc-macro = true
"#,
    );
    fixture.write(
        "Cargo.lock",
        r#"
version = 4

[[package]]
name = "fixture-app"
version = "0.1.0"
dependencies = ["fixture-macro"]

[[package]]
name = "fixture-macro"
version = "0.1.0"
"#,
    );
}

fn write_single_package(fixture: &Fixture, dependency: &str, lock_dependency: &str) {
    fixture.write(
        "Cargo.toml",
        &format!(
            r#"
[package]
name = "fixture-app"
version = "0.1.0"
build = false

[lib]
path = "src/lib.rs"

{dependency}
"#
        ),
    );
    fixture.write(
        "Cargo.lock",
        &format!(
            r#"
version = 4

[[package]]
name = "fixture-app"
version = "0.1.0"
{lock_dependency}
"#
        ),
    );
}

fn assert_error_code(fixture: &Fixture, expected: &str) {
    let error = fixture.discover().expect_err("fixture must fail closed");
    assert_eq!(error.code(), expected);
}

fn count(
    discovery: &super::TrustedHostBuildInputFixtureDiscovery,
    kind: BuildTimeInputKind,
) -> u32 {
    discovery
        .result()
        .kind_counts()
        .iter()
        .find(|entry| entry.kind() == kind)
        .map_or(0, crate::BuildTimeInputKindCount::count)
}

#[test]
fn supported_virtual_workspace_resolves_local_edges_and_classifies_build_code() {
    let fixture = Fixture::new("supported-workspace");
    write_supported_workspace(&fixture);

    let discovery = fixture.discover().expect("supported fixture");

    assert_eq!(
        discovery.result().static_completeness(),
        TrustedHostStaticGraphCompleteness::Complete
    );
    assert_eq!(discovery.result().unsupported_reasons(), []);
    assert_eq!(count(&discovery, BuildTimeInputKind::RepositoryContent), 1);
    assert_eq!(
        count(&discovery, BuildTimeInputKind::ManifestOrConfiguration),
        4
    );
    assert_eq!(count(&discovery, BuildTimeInputKind::BuildScript), 1);
    assert_eq!(count(&discovery, BuildTimeInputKind::ProceduralMacro), 1);
    assert_eq!(discovery.result().total_count(), 7);
    discovery
        .result()
        .validate_for_request(discovery.request())
        .expect("request binding");
}

#[test]
fn fixture_discovery_is_deterministic_with_fixed_semantic_graph_vector() {
    let fixture = Fixture::new("deterministic");
    write_supported_workspace(&fixture);

    let first = fixture.discover().expect("first discovery");
    let second = fixture.discover().expect("second discovery");

    assert_eq!(first, second);
    assert_eq!(
        first.result().resolved_graph_commitment().as_str(),
        "a14d2f79e95bc7fcd0ee1c0e56993125cde7fc33773cbc47aa2d566395c25544"
    );
    assert_eq!(
        first.result().graph_commitment().as_str(),
        "d9a9b4b2b7bc605e66e45073fc4992682e200cca4eb61438a46c4e5488bb4b60"
    );
}

#[test]
fn external_locked_package_is_explicitly_incomplete() {
    let fixture = Fixture::new("external-package");
    write_single_package(
        &fixture,
        "[dependencies]\nserde = \"1\"",
        "dependencies = [\"serde 1.0.228 (registry+https://github.com/rust-lang/crates.io-index)\"]\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.228\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"bounded-checksum\"",
    );

    let discovery = fixture.discover().expect("bounded external package");

    assert_eq!(
        discovery.result().static_completeness(),
        TrustedHostStaticGraphCompleteness::Incomplete
    );
    assert_eq!(
        discovery.result().unsupported_reasons(),
        &[TrustedHostBuildInputUnsupportedReason::SelectionAmbiguous]
    );
    assert_eq!(count(&discovery, BuildTimeInputKind::DependencyPackage), 1);
}

#[test]
fn malformed_and_unknown_manifest_data_fail_without_leaking_values() {
    let malformed = Fixture::new("malformed");
    malformed.write("Cargo.toml", "secret-token-like-value = [");
    malformed.write("Cargo.lock", "version = 4");
    let malformed_error = malformed.discover().expect_err("malformed rejected");
    assert_eq!(
        malformed_error.code(),
        "trusted_host.build_input_fixture_discovery.metadata_invalid"
    );
    assert!(!malformed_error
        .to_string()
        .contains("secret-token-like-value"));

    let unknown = Fixture::new("unknown");
    unknown.write(
        "Cargo.toml",
        r#"
[package]
name = "fixture-app"
version = "0.1.0"
build = false
secret-token-like-field = "secret-token-like-value"
[lib]
path = "src/lib.rs"
"#,
    );
    unknown.write(
        "Cargo.lock",
        "version = 4\n[[package]]\nname = \"fixture-app\"\nversion = \"0.1.0\"",
    );
    let unknown_error = unknown.discover().expect_err("unknown rejected");
    assert_eq!(
        unknown_error.code(),
        "trusted_host.build_input_fixture_discovery.metadata_invalid"
    );
    assert!(!unknown_error.to_string().contains("secret-token-like"));
}

#[test]
fn root_escape_local_dependency_fails_closed() {
    let fixture = Fixture::new("root-escape");
    write_single_package(
        &fixture,
        "[dependencies]\noutside = { path = \"../../outside\", version = \"0.1.0\" }",
        "dependencies = [\"outside 0.1.0\"]",
    );
    assert_error_code(
        &fixture,
        "trusted_host.build_input_fixture_discovery.root_escape",
    );
}

#[cfg(unix)]
#[test]
fn symlinked_workspace_member_fails_closed() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new("symlink");
    fixture.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"member\"]\nresolver = \"2\"",
    );
    fixture.write("Cargo.lock", "version = 4");
    let external = Fixture::new("symlink-target");
    external.write(
        "Cargo.toml",
        "[package]\nname=\"member\"\nversion=\"0.1.0\"\nbuild=false\n[lib]\npath=\"src/lib.rs\"",
    );
    symlink(&external.root, fixture.root.join("member")).expect("create symlink");

    assert_error_code(
        &fixture,
        "trusted_host.build_input_fixture_discovery.symlink_unsupported",
    );
}

#[test]
fn nested_repository_and_nonempty_cargo_configuration_fail_closed() {
    let nested = Fixture::new("nested-repository");
    write_supported_workspace(&nested);
    nested.write("crates/app/.git/HEAD", "ref: refs/heads/main");
    assert_error_code(
        &nested,
        "trusted_host.build_input_fixture_discovery.nested_repository_unsupported",
    );

    let config = Fixture::new("cargo-config");
    write_single_package(&config, "", "");
    config.write(".cargo/config.toml", "[net]\noffline = true");
    assert_error_code(
        &config,
        "trusted_host.build_input_fixture_discovery.cargo_configuration_unsupported",
    );
}

#[test]
fn ambiguous_and_inherited_selection_surfaces_fail_closed() {
    let target = Fixture::new("target-dependency");
    write_single_package(
        &target,
        "[target.'cfg(unix)'.dependencies]\nserde = \"1\"",
        "",
    );
    assert_error_code(
        &target,
        "trusted_host.build_input_fixture_discovery.selection_ambiguous",
    );

    let inherited = Fixture::new("workspace-inheritance");
    inherited.write(
        "Cargo.toml",
        r#"
[workspace]
members = ["member"]
[workspace.package]
version = "0.1.0"
"#,
    );
    inherited.write("Cargo.lock", "version = 4");
    assert_error_code(
        &inherited,
        "trusted_host.build_input_fixture_discovery.workspace_inheritance_unsupported",
    );
}

#[test]
fn implicit_target_and_build_script_fail_closed() {
    let target = Fixture::new("implicit-target");
    target.write(
        "Cargo.toml",
        "[package]\nname=\"fixture-app\"\nversion=\"0.1.0\"\nbuild=false",
    );
    target.write(
        "Cargo.lock",
        "version=4\n[[package]]\nname=\"fixture-app\"\nversion=\"0.1.0\"",
    );
    assert_error_code(
        &target,
        "trusted_host.build_input_fixture_discovery.implicit_target_unsupported",
    );

    let build = Fixture::new("implicit-build-script");
    build.write(
        "Cargo.toml",
        "[package]\nname=\"fixture-app\"\nversion=\"0.1.0\"\n[lib]\npath=\"src/lib.rs\"",
    );
    build.write(
        "Cargo.lock",
        "version=4\n[[package]]\nname=\"fixture-app\"\nversion=\"0.1.0\"",
    );
    build.write("build.rs", "secret-provider-payload");
    assert_error_code(
        &build,
        "trusted_host.build_input_fixture_discovery.implicit_build_script_unsupported",
    );
}

#[test]
fn hard_file_bound_and_inconsistent_lockfile_fail_closed() {
    let bounded = Fixture::new("bounded");
    write_single_package(&bounded, "", "");
    let limits = TrustedHostBuildInputFixtureDiscoveryLimits::new(
        TrustedHostBuildInputFixtureDiscoveryLimitsDefinition {
            max_files: 1,
            max_bytes_per_file: 1024,
            max_total_bytes: 1024,
            max_workspace_members: 1,
            max_local_path_dependencies: 1,
            max_locked_packages: 1,
            max_package_edges: 1,
            max_feature_expressions: 1,
            max_graph_depth: 1,
        },
    )
    .expect("bounded limits");
    let input = TrustedHostBuildInputFixtureDiscoveryInput::new(
        &bounded.root,
        SpecContentHash::from_text("repository"),
        SpecContentHash::from_text("selection"),
        limits,
    );
    let error =
        discover_trusted_host_build_inputs_from_fixture(&input).expect_err("file bound rejected");
    assert_eq!(
        error.code(),
        "trusted_host.build_input_fixture_discovery.bound_exceeded"
    );

    let inconsistent = Fixture::new("inconsistent-lock");
    write_single_package(
        &inconsistent,
        "[dependencies]\nmissing = \"1\"",
        "dependencies = [\"missing 1.0.0\"]",
    );
    assert_error_code(
        &inconsistent,
        "trusted_host.build_input_fixture_discovery.lockfile_inconsistent",
    );
}

#[test]
fn debug_and_serialized_models_do_not_expose_fixture_paths_or_contents() {
    let fixture = Fixture::new("privacy");
    write_supported_workspace(&fixture);
    let secret_path_marker = fixture.root.to_string_lossy().into_owned();
    let input = TrustedHostBuildInputFixtureDiscoveryInput::new(
        &fixture.root,
        SpecContentHash::from_text("secret-tree-marker"),
        SpecContentHash::from_text("secret-selection-marker"),
        TrustedHostBuildInputFixtureDiscoveryLimits::standard(),
    );
    let input_debug = format!("{input:?}");
    assert!(!input_debug.contains(&secret_path_marker));
    assert!(!input_debug.contains("secret-tree-marker"));
    assert!(!input_debug.contains("secret-selection-marker"));

    let discovery =
        discover_trusted_host_build_inputs_from_fixture(&input).expect("privacy fixture discovery");
    let debug = format!("{discovery:?}");
    let serialized = serde_json::to_string(&(discovery.request(), discovery.result()))
        .expect("serialize payload-free models");
    for forbidden in [
        secret_path_marker.as_str(),
        "[workspace]",
        "fixture-macro/example",
        "src/lib.rs",
        "build.rs",
    ] {
        assert!(!debug.contains(forbidden));
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn invalid_limits_are_rejected_without_caller_values() {
    let error = TrustedHostBuildInputFixtureDiscoveryLimits::new(
        TrustedHostBuildInputFixtureDiscoveryLimitsDefinition {
            max_files: 0,
            max_bytes_per_file: 1,
            max_total_bytes: 1,
            max_workspace_members: 1,
            max_local_path_dependencies: 1,
            max_locked_packages: 1,
            max_package_edges: 1,
            max_feature_expressions: 1,
            max_graph_depth: 1,
        },
    )
    .expect_err("zero file limit rejected");
    assert_eq!(
        error.code(),
        "trusted_host.build_input_fixture_discovery.limits.invalid"
    );
    assert!(!error.to_string().contains('0'));
}
