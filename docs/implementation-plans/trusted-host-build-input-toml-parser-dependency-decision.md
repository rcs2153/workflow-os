# Trusted-Host Build-Input TOML Parser Dependency Decision

Status: accepted dependency decision; implementation remains separately
governed.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Trusted-Host Build-Input Discovery Proof Plan](trusted-host-build-input-discovery-proof-plan.md)
- [Trusted-Host Build-Input Discovery Core Model Blocker Fix Review](../concepts/TRUSTED_HOST_BUILD_INPUT_DISCOVERY_MODEL_BLOCKER_FIX_REVIEW.md)

## 1. Decision

Use the Rust `toml` crate at exact version `0.8.23` for the first fixture-only
trusted-host build-input parser. Add it to `workflow-core` with default
features disabled and only the `parse` feature enabled:

```toml
toml = { version = "=0.8.23", default-features = false, features = ["parse"] }
```

The implementation phase must commit the resulting lockfile changes and review
the resolved dependency graph before accepting the addition. It must not enable
the `display`, `preserve_order`, or `unbounded` features.

This document selects a dependency. It does not add the dependency, implement
parsing, authorize filesystem discovery, or widen the trusted-host execution
boundary.

## 2. Why A Dependency Is Required

Cargo manifests, lockfiles, toolchain files, and Cargo configuration are TOML
documents with syntax and semantic edge cases that should not be reimplemented
inside Workflow OS. A hand-written parser would create a larger correctness and
security surface than a narrowly configured, reviewed parser.

The workspace currently has no TOML parser. Existing YAML and JSON dependencies
cannot parse Cargo metadata without an unsafe translation layer or an ad hoc
parser. The accepted discovery plan therefore requires an explicit dependency
decision before parser implementation.

## 3. Compatibility

`toml` `0.8.23` declares:

- Rust version `1.66`, below the Workflow OS workspace floor of Rust `1.78`;
- Rust edition 2021, matching the workspace edition;
- `MIT OR Apache-2.0`, compatible with Workflow OS's Apache-2.0 license; and
- serde-compatible structured deserialization.

The current `toml` 1.x line is not selected because its declared Rust floor is
higher than the Workflow OS Rust `1.78` compatibility contract. The exact
`0.8.23` pin prevents a future incompatible major or minor parser change from
entering this security-sensitive boundary without review.

Authoritative package metadata:

- <https://docs.rs/crate/toml/0.8.23>
- <https://docs.rs/crate/toml/0.8.23/source/Cargo.toml>
- <https://docs.rs/crate/toml/0.8.23/features>

## 4. Feature And Dependency Surface

The `parse` feature provides structured TOML decoding through the crate's
serde-compatible API. Disabling default features excludes formatting output.
The selected configuration does not need insertion-order preservation or the
feature that disables recursion limits.

The published package metadata identifies these direct parser-side components:

- `serde`;
- `serde_spanned`;
- `toml_datetime`; and
- optional `toml_edit`, activated by `parse` with its own default features
  disabled and its `serde` and `parse` support enabled.

The implementation phase must inspect the actual locked transitive graph,
license posture, duplicate versions, and advisory results. Any unexpected
native dependency, build script, incompatible license, advisory, or material
graph expansion is a blocker requiring a new decision.

## 5. Alternatives Considered

### Hand-written TOML parsing

Rejected. Cargo metadata is not a safe line-oriented or string-splitting
format. Reimplementing TOML would weaken malformed-input handling, duplicate
declaration behavior, and semantic fidelity.

### `toml_edit` as the direct public API

Rejected for this phase. The first parser reads immutable fixture metadata and
does not preserve formatting or edit documents. Directly exposing a
format-preserving syntax tree would add an unnecessary representation surface.
`toml_edit` remains a transitive implementation detail of the selected parser.

### Current `toml` 1.x

Rejected while Workflow OS supports Rust `1.78`. The current line's Rust floor
does not satisfy that workspace contract.

### Running Cargo to obtain metadata

Rejected. Cargo execution can resolve environment, configuration, registries,
plugins, build scripts, and other ambient inputs. The first proof is explicitly
static, fixture-only, and no-process.

## 6. Security Requirements For Implementation

Using a mature parser does not make the discovery result trustworthy by
itself. The implementation must still:

- enforce file byte limits before parsing;
- enforce collection, graph, and traversal-depth limits;
- keep parser recursion limits enabled;
- deserialize into narrow internal structures rather than retaining arbitrary
  payload trees;
- reject or explicitly classify graph-affecting unsupported fields;
- reject symlinks, root escape, metadata drift, malformed inputs, and ambiguous
  Cargo semantics;
- avoid echoing raw paths, values, or parser excerpts in errors;
- canonicalize only validated semantic values; and
- treat the parser as syntax infrastructure, not as an authority or execution
  provider.

## 7. Validation And Acceptance

The dependency may be added only in the separately governed fixture-parser
implementation phase. That phase must run at least:

- focused parser and security tests;
- `cargo metadata --locked --format-version 1`;
- `cargo audit`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

The dependency decision must be revisited if the fixture parser cannot remain
within the accepted static, root-bounded, payload-free discovery claim.

## 8. Explicit Non-Authorization

This decision does not authorize:

- live repository discovery;
- arbitrary source-file reads;
- Cargo, compiler, linker, build-script, procedural-macro, provider, or shell
  execution;
- network access, registry access, package downloads, or cache mutation;
- persistence, events, runtime wiring, automatic handler registration,
  schemas, CLI behavior, SDK changes, writes, or authority grants;
- provider selection or sandbox composition; or
- hosted or release-posture changes.

## 9. Governed Phase Report

- Dogfood workflow: `dg/d`
- Run ID: `run-1792826636286148000-2`
- Approval ID:
  `approval/run-1792826636286148000-2/planning-approved`
- Approval outcome: granted with persisted, proof-enforced presentation
- Event summary: 39 events, one approval, zero retries, zero escalations
- Validation summary: `npm run check:docs` and `git diff --check` passed;
  compatibility and license claims were checked against the workspace manifest
  and the official `toml` `0.8.23` package metadata
- Out-of-kernel work: official package metadata research, documentation edits,
  Git commands, and validation commands were performed by the delegated
  maintainer outside the kernel; the kernel coordinated governance and recorded
  the approval trail
- Missing coverage: no dependency was added and no lockfile, resolved
  transitive graph, advisory result, parser behavior, filesystem behavior, or
  runtime behavior was tested in this planning phase
