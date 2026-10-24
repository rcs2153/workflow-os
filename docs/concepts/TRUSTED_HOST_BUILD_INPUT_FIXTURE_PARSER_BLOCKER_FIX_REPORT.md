# Trusted-Host Build-Input Fixture Parser Blocker Fix Report

## 1. Executive Summary

The four blockers from the fixture-parser maintainer/security review are fixed
within the accepted fixture-only, process-free boundary. The parser now
root-checks every accepted explicit Cargo path used by the graph, enforces one
operation-wide feature-expression budget, binds canonical dependency feature
semantics into the resolved graph commitment, and fails closed for broken
optional metadata symlinks.

This phase does not inspect the live Workflow OS workspace, execute Cargo or
build code, observe host inputs, admit runtime execution, select a provider,
persist discovery, expose schemas or CLI behavior, or authorize writes.

## 2. Blockers Fixed

1. Explicit library paths, explicit build-script paths, workspace members and
   exclusions, and local path dependencies are normalized relative to their
   owning manifest and rejected if normalization escapes the fixture root.
2. `max_feature_expressions` is one checked budget for the complete discovery
   operation, including feature names, canonical top-level feature entries,
   and canonical dependency feature lists.
3. Manifest dependency edges now bind `optional`, effective
   `default-features`, and sorted/deduplicated feature selections. Top-level
   feature entries are also sorted and deduplicated before commitment.
4. Optional toolchain and Cargo configuration discovery uses
   `symlink_metadata`. Only an actual not-found result means absence; a broken
   or live symlink is present and then rejected by the existing bounded reader.

## 3. Implementation Approach

The fix reuses the existing lexical normalization and fixture reader rather
than adding another filesystem authority. `normalize_relative_join` is now the
single root-containment gate for accepted explicit path-bearing Cargo fields.
The parser carries one checked feature counter through manifest traversal.
Canonical feature facts are retained in private parser records and included in
the already-existing payload-free semantic graph commitment.

No raw path, feature string, package identity, manifest body, parser payload,
or filesystem error is added to public output or error text.

## 4. Path And Metadata Boundary

Parent components may normalize within the fixture root, but any path that
would escape it fails with the stable
`trusted_host.build_input_fixture_discovery.root_escape` code. This applies to
explicit target and build-script paths as well as graph-selection paths.

Optional metadata presence is no longer inferred with `Path::exists`, which
can hide broken symlinks. Present symlinks reach the same fail-closed reader as
required metadata and produce the stable non-leaking symlink error.

## 5. Bounds And Commitment Boundary

The caller's feature-expression limit is applied once across the complete
operation. Checked integer conversion and addition fail closed. Canonical
sorting and deduplication make set-like feature declarations independent of
incidental TOML array order while preserving commitment sensitivity to
dependency optional/default/feature semantics.

The fixed semantic graph vector is
`381bb0687a930fc071af924aafb64a525dbefbd0770c04ad34bcb9b96b8434a3`.
The fixed outer result vector is
`a2a3f26d4f7a3325f35dd66843ea332c03f35fd4cee177606562634e3d64f803`.

## 6. Privacy And Error Posture

All new failure paths use existing stable, generic fixture-discovery errors.
Errors do not include paths, feature names, dependency declarations, parser
payloads, source contents, command output, credentials, tokens, or provider
data. Debug and serialization boundaries are unchanged and payload-free.

## 7. Test Coverage

Focused regressions cover:

- root-escaping explicit library paths;
- root-escaping explicit build-script paths;
- root-escaping workspace exclusions;
- broken optional toolchain symlinks;
- broken optional Cargo-config symlinks;
- aggregate overflow across multiple manifests and dependency feature lists;
- canonical order-independent feature semantics; and
- commitment sensitivity to dependency optional/default/feature changes.

The trusted-host focused suite passes 39 tests. Existing workspace validation
remains part of the required phase-close matrix.

## 8. Commands Run And Results

- `cargo test -p workflow-core trusted_host_build_input`: passed, 39 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo metadata --locked --format-version 1`: passed.
- CI-pinned RustSec audit at advisory revision
  `309ad29d8fe448bf986019e05d47b9e0e29a2218`: passed with one allowed
  yanked-dependency warning and no vulnerability finding.
- `npm audit --audit-level=moderate`: passed with zero vulnerabilities.
- `npm run check`: passed under the repository Node 20 toolchain.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Scope Explicitly Not Completed

- Live workspace proof or Git object binding.
- Immutable filesystem object identity.
- Cargo, compiler, linker, build-script, procedural-macro, provider, or shell
  execution.
- Host toolchain, environment, filesystem, network, process, resource, or
  transient-write observation.
- Runtime admission, handlers, persistence, workflow events, reports,
  artifacts, schemas, SDKs, CLI behavior, onboarding, examples, provider
  mutations, credentials, authority broadening, or release changes.

## 10. Remaining Known Limitations

- The parser supports a strict Cargo subset rather than complete Cargo
  semantics.
- Caller-supplied repository and selection commitments are not independently
  proved by this fixture helper.
- The existing before/after metadata check does not establish immutable object
  identity.
- Direct graph-cycle and deterministic metadata-change regressions remain
  non-blocking follow-ups before live workspace proof.
- External packages remain incomplete because fixture lock metadata does not
  prove executable build-time behavior.

## 11. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Do not begin
live Workflow OS workspace proof, host observation, runtime admission,
providers, persistence, schemas, CLI behavior, examples, writes, authority
broadening, or release changes before acceptance.

## 12. Governed Phase Record

- Workflow ID: `dg/blocker`.
- Run ID: `run-1792829031904993000-2`.
- Approval ID: `approval/run-1792829031904993000-2/fix-approved`.
- Approval-presentation ID: `presentation/a2cace1499ae62a0`.
- Approval-presentation content hash:
  `a2cace1499ae62a040c34e586001c5f55892ae8face78219cfc5a2b9d90c1991`.
- Approval outcome: granted with persisted presentation proof under standing
  delegated-maintainer authority for the exact blocker-fix scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations.
- Approval-presentation enforcement: proof enforced; one matching presentation
  record and the persisted presentation marker were present in the event trail.
- Out-of-kernel work: the delegated maintainer edited source, tests, and docs
  and ran validation. The kernel governed scope and approval but did not edit
  files, parse fixtures, run validation, or perform git or pull-request work.
