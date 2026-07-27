# Profile-Controlled Authoritative Governance Activation Report

## 1. Executive Summary

Workflow OS now supports one typed, optional project declaration that activates
the existing authoritative local-governance path without requiring a repeated
CLI flag. The implementation is complete for the closed v0 combination:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

Projects without the declaration retain ordinary execution behavior. The
explicit `--authoritative-governance` flag remains an experimental
compatibility path.

## 2. Scope Completed

- Added typed Rust project-governance and authoritative-execution
  configuration.
- Added synchronized JSON Schema and TypeScript SDK contracts.
- Added stable fail-closed parsing and semantic validation.
- Routed `run` through the existing authoritative path when the declaration is
  present.
- Routed matching waiting runs through proof-enforced authoritative `approve`
  without requiring the compatibility flag.
- Bound the exact validated declaration and canonical project-manifest content
  identity into the immutable run execution posture.
- Re-derived and compared that identity during approval resume.
- Added bounded first-run verbose and JSON disclosure.
- Preserved the existing Core route selector, local-check profile, approval
  presentation proof, reassessment, and in-memory report path.

## 3. Scope Explicitly Not Completed

This phase did not add:

- scaffold defaults or inferred activation;
- arbitrary local commands or ambient executable discovery;
- additional local-check or governance-profile families;
- automatic or model self-approval;
- report persistence or report artifacts;
- provider execution, OpenShell integration, network access, or credentials;
- SideEffect execution or provider writes;
- enterprise RBAC, IdP, stewardship, or hosted controls;
- workflow generation or promotion;
- reasoning lineage, nested harness execution, recursive agents, or agent
  swarms; or
- release posture changes.

## 4. Public Contract

`ProjectManifest` now accepts optional `governance` configuration. The first
closed `authoritative_execution` contract requires both:

- `profile: observe_and_report`
- `local_check_profile: workflow_os_project_validation`

Unknown fields, incomplete objects, unsupported values, and unsupported
combinations fail closed. The declaration contains no command, provider,
credential, or precomputed route.

## 5. Runtime Behavior

On `run`, the CLI validates the project and selects the existing authoritative
execution path when the supported declaration is present. Core still chooses
quiet proceed, visible proceed, approval required, or denial from validated
facts.

On `approve`, the CLI identifies declaration-bound runs from the durable
immutable bundle. It reloads the current validated project and invokes the
existing proof-enforced reassessment and report-completion path without
requiring the compatibility flag.

## 6. Immutable Run And Resume Integrity

The immutable execution posture stores a bounded authoritative activation
containing:

- the validated closed configuration; and
- the canonical project-manifest content hash.

The activation participates in the immutable bundle root hash. Approval resume
re-derives the activation from the current validated project and requires exact
posture equality. Removing the declaration, changing it, or changing any
project-manifest content invalidates the resume request before gated work can
continue.

Debug output discloses only that activation is present and redacts the manifest
hash.

## 7. Compatibility

Ordinary projects remain on the existing execution path. Existing explicit
`run --authoritative-governance` and
`approve --authoritative-governance` behavior remains available. The new
serialized immutable-posture field is optional for backward deserialization of
older bundles.

## 8. Privacy And Error Handling

Validation and runtime failures use stable bounded codes. They do not include
raw manifest content, absolute paths, command output, environment values,
credentials, tokens, provider payloads, or approval reasons.

First-run output exposes only known profile/check labels and declared,
supported, and enforced booleans.

## 9. Test Coverage

Focused tests cover:

- valid project declaration parsing;
- incomplete and unsupported declaration rejection;
- TypeScript SDK emission;
- schema synchronization through repository checks;
- no-flag quiet authoritative execution;
- no-flag approval-required execution and proof-enforced resume;
- removed declaration failure on resume;
- unchanged declaration with changed project-manifest identity failure on
  resume;
- first-run verbose and JSON disclosure;
- existing explicit authoritative CLI behavior; and
- ordinary execution compatibility through the workspace suite.

## 10. Validation Commands And Results

Completed while implementing:

- `cargo check --workspace --all-targets`: passed.
- focused project parser tests: passed, 16 tests.
- focused declaration activation CLI tests: passed, 3 tests.
- focused project-manifest identity drift CLI test: passed, 1 test.
- TypeScript SDK tests: passed, 11 tests.
- existing focused authoritative CLI tests: passed, 7 tests.
- focused first-run disclosure test: passed, 1 test.

Full workspace formatting, clippy, tests, documentation checks, and integration
checks then completed:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed after correcting one undeclared-path
  immutable-hash compatibility regression found by the first full run.
- `npm run check:docs`: passed.
- `npm run check:integrations` under Node 20: passed.
- `git diff --check`: passed.

## 11. Workflow Semantics

The declaration is an activation and minimum posture input, not a route
override. It cannot suppress policy, approval, evidence, check, sensitivity,
authority, or SideEffect facts. Report generation failure and governance
denial retain their existing semantics.

## 12. Remaining Limitations

- Only one closed profile/check combination is supported.
- The local check remains the canonical Workflow OS project-validation
  profile.
- Reports remain in memory on this path.
- Project files can declare the local minimum; enterprise-controlled
  stewardship and tightening sources remain future work.
- The explicit compatibility flag remains available.
- No provider or sandbox execution substrate is connected.

## 13. Recommended Next Phase

Phase-level maintainer review is complete and accepts the implementation with
non-blocking follow-ups. See the
[Profile-Controlled Authoritative Governance Activation Review](PROFILE_CONTROLLED_AUTHORITATIVE_GOVERNANCE_ACTIVATION_REVIEW.md).

After acceptance, resume the roadmap sequence rather than broadening provider
mutations. OpenShell, if later pursued, should remain an optional execution
provider behind Workflow OS governance and should receive a separate contract
and threat-model phase.

## 14. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1785117747770274000-2`
- approval: `approval/run-1785117747770274000-2/implementation-approved`
- presentation: `presentation/02d5e27bed6ed12a`
- approval outcome: granted by delegated maintainer through presentation-proof
  enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- validation summary: formatting, clippy, workspace tests, docs, Node 20
  integration checks, and diff hygiene passed
- skipped checks: opt-in live adapter and provider smoke tests remained skipped
  by their existing environment-gated contracts
- report posture: this implementation report is persisted in the repository;
  no runtime WorkReport artifact was generated
- out-of-kernel work: code and documentation inspection, editing, test
  execution, and report authoring
- kernel boundary: the kernel governed scope and approval; it did not edit
  files, run checks, or perform git and PR actions
