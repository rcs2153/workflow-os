# Trusted-Host Build-Input Discovery Core Model Report

## 1. Executive Summary

Workflow OS Core now contains a provider-neutral, non-authorizing request and
result model for future deterministic trusted-host static build-input
discovery. The model binds one exact repository snapshot, selection, metadata
source set, discovery policy, and contract version; validates bounded
payload-free graph facts; records every unresolved host obligation; and derives
deterministic canonical commitments.

The phase does not read repository files, parse Cargo metadata, run build code,
select a provider, register a handler, persist state, or grant execution
authority. Static graph completeness remains a distinct type and cannot be
used as complete live build-input closure.

## 2. Scope Completed

- Added one focused Core model module and crate exports.
- Added exact static discovery request binding and derived fingerprinting.
- Added closed contract-version and selection-posture vocabulary.
- Added static graph completeness distinct from live input completeness.
- Added the complete closed set of unresolved host obligations.
- Added bounded input-kind counts and unsupported-reason vocabulary.
- Added deterministic canonical result commitments.
- Added exact request/result substitution validation.
- Added validated serde and redaction-safe Debug behavior.
- Hardened the reused `BuildTimeInputKind` deserializer against unknown-value
  leakage.
- Added 18 focused model tests.

## 3. Scope Explicitly Not Completed

This phase did not add repository reads, filesystem traversal, TOML parsing,
Cargo metadata, Cargo execution, build scripts, procedural macros, compilers,
linkers, package code, provider code, shell commands, fixture discovery,
workspace discovery, object binding, provider selection, OpenShell integration,
handler registration, executor wiring, persistence, events, reports, artifacts,
CLI behavior, configuration, schemas, SDK changes, examples, dependencies,
credentials, provider mutations, writes, hosted expansion, or release changes.

## 4. Model Types Added

The module adds:

- `TrustedHostBuildInputDiscoveryContractVersion`;
- `TrustedHostBuildInputSelectionPosture`;
- `TrustedHostStaticGraphCompleteness`;
- `TrustedHostBuildInputHostObligationKind`;
- `TrustedHostBuildInputHostObligationPosture`;
- `TrustedHostBuildInputHostObligation`;
- `TrustedHostBuildInputUnsupportedReason`;
- `TrustedHostBuildInputDiscoveryRequest` and its definition; and
- `TrustedHostBuildInputDiscoveryResult` and its definition.

Existing `SpecContentHash`, `BuildTimeInputKind`,
`BuildTimeInputKindCount`, and `HostedExecutionReference` vocabulary is reused
where semantics are exact.

## 5. Request Binding And Canonicalization

The request commits:

- `StaticCargoGraphV1` contract semantics;
- exact repository-tree identity;
- explicit or conservative-all-potential-edge selection posture;
- package/feature/target/profile selection;
- the approved metadata source set; and
- the discovery policy and bounds.

Its request fingerprint is domain separated and derived from authoritative
fields. Serialization omits the derived fingerprint, and deserialization
recomputes it. The model contains no raw root path, branch name, working-tree
label, manifest data, or ambient current-directory state.

The result commits the exact request fingerprint, static completeness, bounded
canonical input-kind counts, all host obligations, stable unsupported reasons,
and an optional telemetry reference. Collection order cannot change the
commitment. A fixed vector protects the reviewed encoding.

## 6. Static Completeness Boundary

`TrustedHostStaticGraphCompleteness::Complete` means only that every approved
static source was resolved under the closed discovery contract. It is not
interchangeable with `BuildTimeInputCompletenessPosture::Complete`.

Every result, including a statically complete result, must contain exactly one
entry for each of these unresolved host obligations:

- toolchain executable;
- target specification;
- SDK or sysroot;
- linker or archiver;
- native compiler tools;
- dynamic libraries;
- registry and package sources;
- environment; and
- runtime/base identity.

Their only current postures are `Unavailable` and `Unsupported`. The model
cannot represent them as satisfied.

## 7. Validation Boundary

Construction rejects:

- oversized counts or collections;
- zero-count alternate encodings;
- duplicate input kinds;
- count-total mismatch;
- an empty graph claiming complete static posture;
- missing or duplicate host obligation kinds;
- reasons attached to complete posture;
- non-complete posture without a stable reason;
- duplicate or oversized reason collections;
- non-telemetry observation references; and
- request/result substitution.

Errors use the stable `trusted_host.build_input_discovery.*` family and contain
no rejected commitments, references, paths, payloads, or unknown values.

## 8. Redaction And Privacy

The model stores only closed enums, bounded counts, opaque references, and
SHA-256 commitments. It stores no file names, paths, manifests, lockfile
contents, package names or versions, source URLs, source or dependency
contents, environment names or values, tool locations, commands, outputs,
credentials, provider payloads, or logs.

Debug redacts all commitments and references. Derived fingerprints are omitted
from serialization and recomputed. Unknown contract versions, host-obligation
kinds, postures, reasons, and reused build-input kinds fail with
value-independent errors.

## 9. Test Coverage

The 18 focused tests cover:

- a valid complete static graph with unresolved host obligations;
- deterministic request fingerprints and selection sensitivity;
- order-independent result canonicalization;
- fixed-vector graph commitment;
- serde round trips with derived-field omission and reconstruction;
- exact request binding and substitution rejection;
- complete/non-complete reason coherence;
- complete host-obligation coverage and duplicate rejection;
- duplicate, zero, mismatched, and empty count rejection;
- observation-reference validation;
- redaction-safe Debug;
- non-leaking unknown contract, nested obligation, and input-kind values;
- invalid result serde re-entering validation;
- closed posture and reason vocabulary; and
- pure construction without filesystem, process, provider, or state backend.

## 10. Validation

Completed before report finalization:

- `cargo fmt --all --check`: passed.
- focused model tests: passed, 18 tests.
- focused `workflow-core` clippy with warnings denied: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Limitations

- No parser or filesystem observer exists.
- No repository object-binding mechanism is selected.
- Cargo feature, target, profile, implicit-target, and implicit-build-script
  semantics are not implemented.
- No registry, Git source, path source, checksum, or source-replacement
  normalization exists.
- No fixture or Workflow OS workspace discovery proof exists.
- No host toolchain, sysroot, linker, dynamic-library, environment, or runtime
  observer exists.
- No provider can consume this model.
- A valid result remains caller-constructed evidence and is not executable
  authority.

## 12. Recommended Next Phase

Perform a focused maintainer and security review of the Core static-discovery
model. The review should verify exact binding, canonical encoding, strict
static/live completeness separation, complete host obligations, bounds,
serde/error privacy, public API minimality, and absence of executable
authority.

Do not begin parser or workspace discovery implementation before that review.

## 13. Governed Implementation Record

- Workflow ID: `dg/implement`.
- Run ID: `run-1792819622675797000-2`.
- Approval ID:
  `approval/run-1792819622675797000-2/implementation-approved`.
- Approval-presentation ID: `presentation/dcf7c33d8e95f83f`.
- Approval-presentation content hash:
  `dcf7c33d8e95f83f2b9cf09ddf314c73e8155a8d46b17a82d83ccbf5d8713e10`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact model-only implementation scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request and grant, six
  scheduled steps with six successful skill invocations, eight policy
  decisions, and terminal run completion; zero retries and zero escalations.
- Approval-presentation enforcement: proof enforced, with one matching
  presentation record and the presentation marker present in the approval
  event trail.
- Validation summary: formatting, focused tests, focused clippy, full workspace
  clippy and tests, aggregate repository checks, integration contracts,
  documentation checks, and diff hygiene all passed.
- Out-of-kernel work: the delegated maintainer implemented and tested the Core
  model, updated documentation and roadmap state, ran validation, and performs
  git and pull-request operations. The kernel governed scope and approval but
  did not edit files, run commands, or operate git/GitHub.

## 14. CI Concurrency Blocker Fix

The first two CI Rust executions each failed a different existing
trusted-host concurrency test after the static-discovery model tests passed.
Both failures preserved the security invariant: exactly one caller entered the
executor and the aggregate executor-entry count remained one. The assertions
failed because they omitted two lawful fail-closed loser observations already
proved by the adjacent competing-owner contract:

- `trusted_host_redispatch.directive_missing`, when the winner consumes the
  only available directive before the loser reads it; and
- `trusted_host_redispatch.active_yield_missing`, when terminal reconciliation
  removes the active yield before the loser reads it.

The focused blocker fix adds only those two exact stable codes to the closed
loser sets for the competing local-timer and explicit-reinvocation composition
tests. It retains the one-call, one-entry, and one-winner assertions and adds
bounded result diagnostics for future assertion failures. Production behavior
is unchanged.

Focused stress passed across 20 competing local-timer races and 10 concurrent
explicit-reinvocation races. The complete validation matrix then passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

The fix was governed by `dg/blocker` run
`run-1792821803893758000-2`, approval
`approval/run-1792821803893758000-2/fix-approved`, and persisted
approval-presentation proof `presentation/c6c9f5c8837cd0c0`. The delegated
maintainer granted the exact bounded fix scope; no retry-only workaround,
assertion weakening, ignored test, production runtime change, or discovery
model expansion was authorized. The run completed with 39 events, one approval,
zero retries, zero escalations, one matching presentation record, and the
presentation marker present in the approval event trail. Repository edits,
validation, git, pull-request, and CI operations remained out-of-kernel work.
