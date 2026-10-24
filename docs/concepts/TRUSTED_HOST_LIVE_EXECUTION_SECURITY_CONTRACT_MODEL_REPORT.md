# Trusted-Host Live-Execution Security Contract Core Model Report

## 1. Executive Summary

Workflow OS Core now contains the first provider-neutral, non-authorizing
security contract model for a future trusted-host live execution. The model
separates requirements from payload-free observations and compares them with a
pure assessment function. It starts no process, selects no provider, discovers
no host inputs, and grants no authority.

The implementation preserves the mandatory plan-review constraints: exact
provider-neutral attempt binding, explicit `NotStarted`, `Started`, and
`MayHaveStarted` posture, no fabricated runtime-image identity, bounded and
domain-separated commitments, and zero-event facts that are meaningful only
when the observation channel is proven.

## 2. Scope Completed

- Added one focused Core module and crate exports.
- Added validated requirements, observations, exact bindings, build-input
  manifests, typed root facts, process-tree facts, resource-budget facts, and
  transient-write facts.
- Added deterministic requirement fingerprints and a pure assessment.
- Added fail-closed validation, serde, redaction-safe Debug, and stable errors.
- Added focused behavioral and regression tests.

## 3. Scope Explicitly Not Completed

This phase did not add a process runner, shell, Cargo invocation, live smoke,
real input discovery, filesystem or network sandbox, provider selection,
OpenShell integration, workflow composition, handler registration, executor
authority, persistence, events, WorkReport integration, artifacts, CLI,
configuration, schemas, SDKs, examples, dependencies, provider mutations,
credentials, writes, hosted expansion, or release changes.

## 4. Model Types Added And Existing Types Reused

The module adds `TrustedHostLiveExecutionSecurityRequirements`,
`TrustedHostLiveExecutionSecurityObservations`,
`TrustedHostLiveExecutionSecurityAssessment`, and their closed supporting
vocabulary for exact binding, input closure, root roles, attempt-start posture,
process-tree terminality, resource budgets, transient writes, reasons, and
dispositions.

It reuses `SpecContentHash`, provider identity/version, hosted enforcement and
control posture, cleanup posture, hosted references, and the bounded hosted
observation summary. It does not create a second hosted terminal receipt or
reuse `HostedExecutionAttestation` where doing so would require a fabricated
runtime-image digest.

## 5. Requirement And Observation Separation

Requirements state what Core expects for one exact future operation.
Observations state bounded facts reported for one exact future attempt.
Constructing either value grants no authority and proves no satisfaction.

`assess_trusted_host_live_execution_security` is pure. It verifies exact
binding and produces only `Satisfied`, `DeniedBeforeStart`, or
`ReconciliationRequired` plus canonical stable reasons.

## 6. Coherence And Authority Boundaries

The model distinguishes affirmative non-start from started or possibly-started
execution. Only affirmative non-start may map failed requirements to
`DeniedBeforeStart`; every started or ambiguous attempt maps failure to
reconciliation.

Process-tree terminality requires parent exit, completed termination, and at
least one terminal event. Not-started observations require zero start and
terminal events. Policy stability requires both an affirmative stability fact
and zero policy-change events. Missing observation proof, allowed egress,
missing required probe evidence, incomplete roots or inputs, non-enforced
budgets, uncertain transient writes, and incomplete cleanup cannot satisfy.

A satisfied assessment is evidence for one exact attempt. It is not reusable
authority and cannot run, dispatch, authorize, register, or persist anything.

## 7. Validation And Stable Errors

Validated constructors enforce hard collection and count bounds, closed
reference kinds, canonical root ordering, exact role coverage, access and
invocation-scoping coherence, process-tree state coherence, and assessment
state coherence. Nested process-tree and assessment deserialization re-enter
those constructors and fail closed.

Errors use the stable `trusted_host.live_security.*` family and contain no
commitments, identifiers, paths, payloads, command output, provider output, or
secret-like values.

## 8. Redaction And Privacy

The model stores closed enums, bounded counts, validated stable references,
and SHA-256 commitments. It stores no file paths, source or dependency
contents, command text or output, environment values, credentials, network or
filesystem payloads, provider payloads, or sandbox logs. Custom Debug output
redacts provider identity, commitments, and references.

## 9. Test Coverage

Focused tests cover exact satisfaction without authority, incomplete input
reconciliation, affirmative pre-start denial, requirements and provider
selection substitution, duplicate roots, incoherent attempt posture, serde
round trip and bounds, all input and ambiguity vocabulary, invalid nested
process/assessment serde, policy-change and observation-channel failure,
required egress-probe failure, and canonical fingerprints independent of root
declaration order. They also cover canonical input-kind ordering, immutable-root
method semantics, and transient-write count/posture coherence.

Existing hosted-execution, trusted-host Cargo clippy, executor denial,
registration exclusion, and workspace tests remain part of phase validation.

## 10. Commands And Results

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- focused trusted-host live-security model tests: passed, 13 tests.
- existing hosted-execution attestation tests: passed, 4 tests.
- existing trusted-host Cargo clippy unit regressions: passed, 12 tests.
- Cargo clippy registration-exclusion regression: passed, 1 test.
- `cargo test --workspace`: passed.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 11. Remaining Live-Process Blockers

No code discovers complete Cargo build inputs, binds operating-system objects,
owns a complete process tree, enforces filesystem/network/resource policy,
proves transient-write exclusion, selects a provider, or composes this model
with a trusted-host attempt. OpenShell remains a candidate, not a dependency or
selected provider. Every real-process blocker remains open.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this model implementation.
Review exact binding, fingerprint canonicalization, hard bounds, impossible
state rejection, fail-closed serde, redaction, and the non-authorizing boundary
before planning any input-discovery or provider-composition work.

## Governed Implementation Record

- Workflow ID: `dg/implement`.
- Run ID: `run-1792812582916473000-2`.
- Approval ID:
  `approval/run-1792812582916473000-2/implementation-approved`.
- Approval-presentation ID: `presentation/011ac0ca300bf71b`.
- Approval-presentation content hash:
  `011ac0ca300bf71b2e4481600b81907bb6e16c49cef80cd6bfa4bf439945b3d3`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact model-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and the
  presentation marker is present in the event trail.
- Out-of-kernel work: the delegated maintainer edited model, test, and
  documentation files and ran validation. The kernel governed scope and
  approval but did not edit files, run checks, or perform git or PR operations.
