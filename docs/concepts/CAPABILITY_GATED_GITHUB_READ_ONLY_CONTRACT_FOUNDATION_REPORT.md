# Capability-Gated GitHub Read-Only Contract Foundation Report

Status: Implemented and accepted after focused maintainer/security review.

## Summary

The first phase of capability-gated GitHub read-only public adoption is
implemented as a dark contract. Workflow OS can now parse and validate one
exact authored adapter operation and one subordinate project-level GitHub
repository-metadata declaration, then commit that declaration into an
authoritative immutable run bundle.

This phase does not make the operation executable. It does not read access
material, derive authority, call GitHub, alter ordinary CLI routing, or add
recovery behavior.

## Implemented Scope

- Added bounded `AdapterOperationId` vocabulary and optional
  `adapter_requirements[].operation` support.
- Recognized only the exact v0 contract:
  `symbolic/github-read-only` + `github.repository.metadata` +
  `external.read`, with no integration identifier.
- Added the closed `governance.provider_reads.github_repository_metadata`
  project declaration with exact endpoint, access reference, and bounded
  canonical resource allowlist.
- Required the existing supported `governance.authoritative_execution`
  declaration whenever provider reads are declared.
- Added Rust, JSON Schema, and TypeScript SDK parity and validation.
- Bound the validated provider-read declaration into authoritative immutable
  run-bundle serialization and root hashing.
- Preserved old projects, skills without an operation, and stored bundles that
  predate the provider-read field.

## Security And Privacy Posture

- The declaration contains an access *reference*, never credential material.
- Debug output discloses only declaration presence and bounded counts; it does
  not print repository targets or the access reference.
- Invalid endpoint, access reference, resource form, duplicate resource, or
  mismatched operation contract fails closed.
- Declaration is not authority. No capability grant, current availability,
  provider dispatch, or network access is added.
- Immutable hashing includes the endpoint, access reference, and sorted exact
  resources so declaration drift changes the run commitment.

## Validation

Focused validation covers:

- valid declaration parsing and schema-compatible shape;
- authoritative-execution prerequisite enforcement;
- unsupported endpoint, access reference, duplicate, and noncanonical resource
  rejection;
- exact operation-contract acceptance and unsupported/mismatched rejection;
- immutable root changes when provider constraints change;
- JSON round-trip and legacy bundle compatibility; and
- TypeScript generation and validation parity.

Completed validation:

- `cargo fmt --all -- --check`: passed;
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed;
- focused project-spec, semantic-validation, and immutable-bundle tests: 61
  passed;
- `cargo test -p workflow-core`: passed, including 573 library tests, all Core
  integration suites, and doc tests;
- `cargo test --workspace --all-targets`: passed, including CLI, Core, hosted,
  local-host, examples, and external-consumer tests; opt-in live tests remained
  ignored as designed;
- `npm run check`: passed, including docs, dogfood helper, integration helper,
  TypeScript typecheck/lint/tests, and contract parity;
- `git diff --check`: passed.

## Explicit Non-Goals

- public runtime or CLI selection;
- environment access or transient credential delivery;
- Core-owned current-authority derivation;
- provider dispatch, GitHub requests, or live smoke tests;
- atomic operational-window, attempt, terminal, or recovery behavior;
- a second provider operation or any provider mutation; and
- release, tag, or package publication.

## Governed Phase Record

- Workflow: `dg/implement`
- Run: `run-1793445726652710000-2`
- Approval: `approval/run-1793445726652710000-2/implementation-approved`
- Presentation: `presentation/d09b4b3a18ecf481`
- Presentation hash:
  `d09b4b3a18ecf4815d900abc603ea306d8f0320cefec500a4508978ff2ce84d6`
- Approval outcome: granted under delegated maintainer authority with persisted
  presentation proof.

The Workflow OS kernel governed scope, approval, and phase state. Codex used
the local editor, Rust/Node toolchains, and Git as the execution layer. No
provider call or external write occurred during implementation.

## Review Outcome

Focused maintainer/security review found one blocking contract-closure defect:
an exact operation could be selected by a skill whose adapter tuple was valid
but whose capability or input contract was unrelated. The implementation now
requires the complete accepted skill shape in Rust and the TypeScript SDK:
exact capabilities, exact required nonsensitive `owner` and `repo` inputs, no
input examples, and no additional adapter requirement. Negative coverage
proves extra capability and incomplete input shapes fail closed.

The corrected implementation is accepted. It remains a dark contract: no
provider selection, access-material handling, authority derivation, network
request, or CLI behavior is enabled. See the [focused implementation
review](CAPABILITY_GATED_GITHUB_READ_ONLY_CONTRACT_FOUNDATION_REVIEW.md).

## Next Phase

Proceed to Phase 2: Core-private authority derivation, separate transient-access
resolution, and the complete operational-window, attempt, dispatch, terminal,
and ambiguity lifecycle. Phase 2 must remain non-public and no-write.
