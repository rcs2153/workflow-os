# Authoritative Local-Check Reassessment Binding Report

## 1. Executive Summary

Workflow OS now has one crate-private, in-memory composition path that binds an
authoritative same-call local-check fact to proportional-governance
reassessment.

The helper validates deterministic immutable-bundle and runtime-fact context
before any clock or local process use, rejects caller-selected check posture
for the selected step, invokes the accepted local-check composition path,
injects only the Core-derived evidence/check posture, and returns one private
bound-assessment value.

The implementation is not wired into the executor and does not activate
automatic checks or quiet success.

## 2. Scope Completed

- Factored immutable-bundle workflow, skill, policy, and exact runtime-fact
  resolution into one pure crate-private preflight.
- Added one crate-private authoritative reassessment input.
- Added selected-step rejection for caller-supplied evidence/check posture.
- Reused the accepted authoritative `DocsCheck` same-call composition helper.
- Verified the produced candidate-set identity against the preflighted
  candidate.
- Replaced only the selected step's absent evidence/check axis through a
  Core-private helper.
- Reused immutable-bundle proportional-governance assessment.
- Added one private bound-assessment value that owns:
  - the authoritative aggregate local-check fact;
  - the complete immutable-bundle assessment set; and
  - the versioned binding fingerprint.
- Returned bounded local-check results without exposing the raw fact or
  assessment set as independent authority.

## 3. Scope Explicitly Not Completed

This phase did not add:

- executor integration or an executor checkpoint;
- default, background, parallel, or automatic local checks;
- runtime quiet-success activation or visible-disclosure presentation;
- workflow-state mutation, persistence, events, evidence, reports, or
  artifacts;
- schemas, SDK, CLI, UI, onboarding, or examples;
- providers, OpenShell, SideEffects, external writes, or network access;
- hosted or distributed behavior;
- reasoning lineage; or
- release changes.

## 4. Implementation Boundary

The private helper accepts:

- the existing authoritative local-check composition input;
- the active governance profile; and
- exact runtime facts for every immutable workflow step.

It does not accept:

- a caller-selected evidence/check posture for the selected step;
- a detached aggregate fact;
- imported leaf contributions;
- a structural-coverage candidate; or
- a prior binding.

The outcome contains bounded local-check results and one private bound value.
There is no public or serialized model and no accessor that returns the raw
assessment set as reusable authority.

## 5. Preflight And Failure Boundary

Before any clock or process use, the helper requires:

- exact immutable workflow, skill, and policy resolution;
- exact runtime-fact count and step membership;
- selected-step membership;
- absence of selected-step caller check posture;
- canonical selected-step local-check declarations; and
- a complete, unambiguous execution set for those declarations.

If preflight fails, no check starts.

If local-check execution fails, no reassessment or bound value is returned. If
pure reassessment fails after checks complete, no bound value is returned. The
implementation does not claim rollback of a completed non-source-writing local
check.

## 6. Binding Identity

The v1 binding fingerprint commits with length-framed fields to:

- algorithm domain;
- immutable bundle ID, version, and root;
- workflow, run, and selected-step identity;
- aggregate local-check fact algorithm and fingerprint;
- candidate-set and structural-coverage fingerprints;
- selected workload-assessment algorithm and input fingerprint; and
- complete assessment-set algorithm and aggregate fingerprint.

Equal complete inputs produce equal identity. A different authoritative fact
or non-check governance axis changes the binding.

## 7. Governance Semantics

The authoritative check fact controls one axis only.

Focused binding tests and the existing accepted component tests collectively
prove:

- `Satisfied` can preserve quiet proceed behavior;
- `OptionalUnavailable` preserves visible disclosure;
- `RequiredUnavailable` preserves denial;
- an executed failed check preserves denial; and
- satisfied checks cannot weaken unavailable authority.

The helper remains review-only. It does not enforce the decision.

## 8. Privacy And Redaction

Errors use stable codes and static messages. The new `Debug` implementations
expose only bounded algorithm, posture, and count metadata.

They do not expose:

- workflow, run, step, check, invocation, or handler identifiers;
- fingerprints;
- commands, arguments, paths, or working directories;
- stdout, stderr, source, spec, environment, or provider payloads;
- credentials, authorization headers, private keys, or tokens; or
- report text.

## 9. Test Coverage

Focused tests cover:

- successful authoritative fact-bound reassessment;
- caller-selected posture rejection before clock/process use;
- invalid runtime-fact shape before clock/process use;
- same posture with different authoritative fact identity;
- non-check governance-axis invalidation;
- monotonic authority behavior;
- optional and required unavailable semantics;
- post-check reassessment failure with no bound value;
- length-framing ambiguity resistance;
- stable v1 known vector; and
- `Debug` non-leakage.

Existing local-check composition tests continue to cover complete execution-set
preflight, command-contract matching, required/optional coverage, execution
failure, freshness, and proof posture.

## 10. Governed Implementation Record

- workflow: `dg/implement`
- run: `run-1785020454006594000-2`
- approval:
  `approval/run-1785020454006594000-2/implementation-approved`
- presentation: `presentation/1d023264b79ec4d3`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, including one approval request, one approval grant,
  eight policy decisions, six successful skill invocations, no retries, and no
  escalations
- out-of-kernel work: Rust implementation, tests, documentation, shell
  validation, and git/PR work
- missing coverage: the kernel coordinated governance only; it did not execute
  checks, edit the repository, generate a WorkReport artifact, or perform git
  actions

## 11. Validation

Completed:

- `cargo check -p workflow-core`: passed;
- focused `workflow-core` local-check runtime tests: 34 passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 12. Remaining Limitations

- The binding is crate-private and in memory only.
- No executor consumes it.
- It is not durable freshness or replay protection.
- Other steps' runtime check facts remain explicit under the existing model.
- Quiet-success product behavior is unchanged.

## 13. Recommended Next Phase

Perform a phase-level maintainer review.

Only after acceptance should Workflow OS plan one opt-in executor consumer of
the private bound value. Keep automatic checks, persistence, providers,
OpenShell, SideEffects, writes, schemas, hosted behavior, and release changes
separate.
