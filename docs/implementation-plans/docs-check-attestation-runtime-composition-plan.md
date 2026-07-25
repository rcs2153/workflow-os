# DocsCheck Attestation Runtime Composition Plan

Status: Implemented as one explicit, crate-internal, in-memory `DocsCheck`
composition helper. The implementation is documented in
[DocsCheck Attestation Runtime Composition Report](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_REPORT.md).
Phase-level review found an immutable attribution blocker: workflow/run identity
is derived from the stored manifest, but step and skill identity are still
caller-selected rather than resolved from the stored canonical records. See
[DocsCheck Attestation Runtime Composition Review](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_REVIEW.md).
The focused fix removes caller-supplied skill identity and derives the selected
step's skill ID/version from the validated stored canonical records. See
[DocsCheck Attestation Runtime Composition Blocker Fix Report](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_BLOCKER_FIX_REPORT.md).
Focused re-review accepts the fix in
[DocsCheck Attestation Runtime Composition Blocker Fix Review](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_BLOCKER_FIX_REVIEW.md).
Consumer integration planning may proceed, but integration remains
unimplemented. The first consumer boundary is defined in
[DocsCheck Attestation Consumer Integration Plan](docs-check-attestation-consumer-integration-plan.md).
Planning blockers are fixed. Implementation inspection found and the focused
verifier fix closes a prerequisite blocker: execution-binding
workflow and run identity must match the validated stored manifest. See
[DocsCheck Attestation Runtime Composition Blocker Report](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_BLOCKER_REPORT.md).
The fix is documented in
[Independent Local Check Attestation Stored Manifest Identity Blocker Fix Report](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_STORED_MANIFEST_IDENTITY_BLOCKER_FIX_REPORT.md).
Focused review accepts the fix in
[Independent Local Check Attestation Stored Manifest Identity Blocker Fix Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_STORED_MANIFEST_IDENTITY_BLOCKER_FIX_REVIEW.md).
Runtime composition resumed after focused review accepted that fix.
Observation time is sampled by the
composition helper through an injected Core-owned clock rather than supplied
as caller facts, and honest no-proof status handling uses typed requirement
eligibility before verifier invocation. See
[DocsCheck Attestation Runtime Composition Plan Review](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_PLAN_REVIEW.md).
The fix is documented in
[DocsCheck Attestation Runtime Composition Plan Blocker Fix Report](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_PLAN_BLOCKER_FIX_REPORT.md).
Focused re-review accepts the corrected plan in
[DocsCheck Attestation Runtime Composition Plan Blocker Fix Review](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_PLAN_BLOCKER_FIX_REVIEW.md).
This plan originally authorized runtime composition; its approved helper is now
implemented without executor or default registration changes.
The immutable local-check execution binding and pure independent attestation
verifier are implemented and accepted. The next implementation is one explicit,
in-memory, opt-in `DocsCheck` composition helper only.

## 1. Executive Summary

Workflow OS can freeze a local-check command, handler selection, effective
policy, and immutable run context before execution. It can also independently
verify a Core-owned observation against the resulting structured check result.
Those primitives are not yet connected to a real local check execution path.

The first runtime-composition slice should add one explicit helper for the
existing `DocsCheck` command. The helper must create the immutable execution
binding before process launch, execute through the existing bounded process
runner, derive the structured result and crate-private observation from the
same Core-owned process output, construct the unverified candidate, and invoke
the accepted verifier. It returns an in-memory outcome and changes no executor
default.

This plan does not implement the helper, automatic checks, persistence, events,
evidence, reports, artifacts, schemas, CLI behavior, providers, SideEffects,
writes, hosted execution, or release changes.

## 2. Goals

- Compose the accepted attestation primitives around one real `DocsCheck` run.
- Freeze all decision-relevant execution context before process launch.
- Keep process observation and attestation authority inside Core.
- Return the structured check result without parsing `SkillOutput` text.
- Return accepted proof only when the requirement and observed result satisfy
  the verifier.
- Preserve explicit opt-in registration and existing executor semantics.
- Keep errors stable, bounded, deterministic, and non-leaking.
- Establish the smallest production-shaped path for later evidence and
  proportional-governance integration.

## 3. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- automatic, ambient, or default local check execution;
- changes to `LocalExecutor::execute` or existing skill invocation semantics;
- workflow-declared check configuration or runtime configuration;
- check-result, attestation, or execution-binding persistence;
- workflow events, audit projection, evidence, WorkReport, or artifact changes;
- approval, proportional-governance, capability, or authority enforcement;
- schemas, SDKs, CLI commands, UI, examples, or migration behavior;
- arbitrary commands, Cargo checks, provider checks, or additional check kinds;
- network-enabled execution, provider access, SideEffects, or writes;
- stronger handler implementation provenance than `RegisteredUnattested`;
- remote, cryptographic, hardware-backed, hosted, or distributed attestation;
- release posture changes.

## 4. Current Runtime Boundary

The existing `DocsCheckLocalHandler` is explicitly constructed and is never
registered by default. It validates the canonical `DocsCheck` contract, creates
a bounded process request with repository-root working directory, sanitized
minimal environment, disabled network posture, timeout, redaction, and
no-source-writes classification, then invokes an injected
`LocalCheckProcessRunner`.

The handler currently converts `LocalCheckProcessOutput` into a validated
`LocalCheckResult` and immediately converts that result to `SkillOutput`. The
executor records the output reference, but the structured result is no longer
available as a trusted runtime input. Parsing the output reference or value map
back into proof would make presentation data an authority source and is not
acceptable.

The attestation verifier is crate-private and accepts a validated
`StoredImmutableRunBundle`, a pre-execution
`ImmutableLocalCheckExecutionBinding`, an exact command contract, a
crate-private observation, a structured result, an unverified candidate, and
an evaluation time. No current runtime path owns all of those inputs.

## 5. Integration Decision

Add an additive internal helper, likely
`execute_docs_check_with_attestation`, in a local-check-attestation runtime
module. Do not alter the general skill registry or executor path in the first
slice.

The helper should accept only explicit inputs and injected dependencies. It
must not read hidden global state, discover a command, select a handler from an
ambient registry, obtain wall-clock time internally, load mutable workflow
definitions, or access a state backend.

The helper is the authority boundary for this one composition path. Public
callers may supply validated identity and policy inputs, but they must not
supply an observation or claim that a process ran.

## 6. Proposed Input Contract

Use one borrowed input structure, likely
`DocsCheckAttestationExecutionInput<'a>`, containing:

- validated `StoredImmutableRunBundle`;
- validated `LocalCheckAttestationRequirement`;
- exact canonical `LocalCheckCommandContract`;
- workflow, run, step, and skill identity;
- Core-selected `SkillInvocationId` and `IdempotencyKey`;
- explicit handler selection definition and registration mode;
- effective local-check execution policy definition;
- bounded creator/system actor;
- one injected crate-private Core-owned clock/time source;
- Core-selected `LocalCheckResultId`;
- explicit npm executable, repository root, and optional npm cache path needed
  by the existing `DocsCheck` request builder; and
- injected `LocalCheckProcessRunner`.

The input must not contain process output, binding creation time, observation
timestamps, verifier evaluation time, a caller-created observation, an accepted
attestation, raw command text, arbitrary environment values, or a caller
assertion that the check passed.

The first implementation should define a narrow crate-private clock trait,
likely `LocalCheckObservationClock`, whose only operation returns a validated
`Timestamp` or a stable non-leaking `WorkflowOsError`. Tests use a scripted
clock; the production-shaped helper may use an explicitly injected system-clock
implementation. The helper, not a public caller, owns every sample.

## 7. Pre-Execution Ordering

The implementation must enforce this order:

1. validate the stored immutable run bundle;
2. validate the exact `DocsCheck` contract and all supplied identities;
3. derive the canonical command-contract fingerprint;
4. derive the explicit registered-handler selection and effective policy;
5. sample the injected Core-owned clock for binding creation time;
6. create and validate `ImmutableLocalCheckExecutionBinding`;
7. construct the bounded process request from the exact bound contract;
8. sample the clock immediately before invoking the runner;
9. invoke the injected process runner;
10. sample the clock immediately after runner completion;
11. derive `LocalCheckResult` from the returned process output;
12. derive `KernelObservedLocalCheck` from the same output, result, binding, and
    helper-owned clock samples;
13. construct the unverified candidate from those exact facts;
14. test typed status eligibility against
    `requirement.accepted_statuses()`;
15. if eligible, sample the clock for verifier evaluation and invoke the pure
    verifier; and
16. return the in-memory composition outcome.

No process may start before step 5 succeeds. No accepted proof may exist before
the process output and structured result are available.

## 8. Execution And Observation Ownership

The existing request construction and bounded output handling should be
factored into the smallest reusable crate-internal operation needed by both the
current handler and the new helper. Do not duplicate redaction, environment,
timeout, or result-construction logic.

The helper owns the injected clock and samples it in deterministic order. It
must reject a clock error or impossible ordering without invoking later stages.
The runner cannot supply, override, or relabel binding, start, completion, or
evaluation time.

`LocalCheckProcessRunner` remains an injected execution boundary. Its output is
not independently trusted merely because it implements the trait. Core owns the
translation from that output into:

- validated `LocalCheckResult`;
- exit-code posture;
- timeout and truncation facts;
- bounded duration;
- observation timestamps; and
- the crate-private observation.

The first assurance remains honestly named `KernelObservedLocalProcess`. It
does not attest runner binary provenance, operating-system integrity, command
source integrity, remote independence, or third-party execution.

## 9. Candidate And Verification Composition

The helper must construct the unverified candidate internally from the exact
execution binding, result, invocation, and observation context. It must not
accept a caller-supplied candidate for this runtime path.

The candidate remains explicitly unverified until the crate-private verifier
returns `AcceptedLocalCheckAttestation`. Matching fingerprints alone are not
proof. The helper must pass the validated stored bundle itself so the verifier
derives the immutable binding from the complete manifest.

The verifier evaluation timestamp is explicit. Later consumers must still
reevaluate freshness at time of use; successful creation does not make proof
permanently fresh.

## 10. Outcome And Failure Semantics

Add one read-only in-memory outcome, likely
`DocsCheckAttestationExecutionOutcome`, containing:

- the validated `LocalCheckResult`; and
- `Option<AcceptedLocalCheckAttestation>`.

A result whose typed status appears in
`LocalCheckAttestationRequirement::accepted_statuses()` proceeds to verifier
invocation and returns `Some(accepted)` only if verification succeeds. A
completed process whose typed status is not accepted returns the structured
result with no accepted proof and does not invoke the verifier. This preserves
the distinction between execution evidence and requirement satisfaction
without fabricating an internal failure.

Invalid binding, observation inconsistency, candidate mismatch, stale proof,
redaction failure, unsafe request construction, or process-runner internal
failure returns `WorkflowOsError` and no outcome. Stable error codes must not
echo IDs, paths, command text, output, environment values, hashes, tokens, or
payloads.

Every verifier error after typed eligibility succeeds is an integrity failure
and must propagate. The helper must not inspect verifier error strings or codes
to produce a no-proof outcome. No verifier error may be downgraded to ordinary
check failure.

## 11. Executor And Default Compatibility

The first implementation must not:

- register `DocsCheckLocalHandler` by default;
- change `LocalSkillRegistry::new()`;
- change existing `LocalExecutor::execute` behavior;
- append additional workflow events;
- create or modify runtime state;
- write report artifacts;
- execute from workflow schema fields; or
- expose a new CLI surface.

Existing explicit `DocsCheckLocalHandler` behavior remains compatible. The new
helper is invoked directly in focused tests and by future reviewed composition
only.

## 12. Privacy And Redaction

The helper must preserve the existing local-check privacy boundary:

- bounded redacted stdout/stderr summaries only;
- no raw output in the execution binding, observation, candidate, or accepted
  proof;
- no command arguments, executable paths, repository paths, npm cache paths,
  environment values, source contents, credentials, or provider payloads in
  Debug output or errors;
- content-addressed policy and command commitments rather than copied payloads;
- redaction failure fails closed before proof creation; and
- accepted proof remains payload-free and non-deserializable.

## 13. Test Plan

The future implementation must prove:

1. one explicit passed `DocsCheck` produces a structured result and accepted
   attestation;
2. the immutable execution binding exists before the runner is invoked;
3. command, handler, effective policy, and stored bundle commitments match the
   accepted proof;
4. the exact stored immutable bundle is required;
5. Core derives the observation and candidate without public caller authority;
6. the helper never parses `SkillOutput` or an output-reference string;
7. a failed check returns the failed structured result and no accepted proof;
8. a timed-out check returns the timed-out structured result and no accepted
   proof;
9. a process-runner internal error returns no partial result or proof;
10. redaction failure returns no partial result or proof;
11. the injected clock is sampled for binding creation, process start, process
    completion, and evaluation in that exact order;
12. callers cannot supply or override observation timestamps;
13. stale, future-dated, inconsistent, or clock-error timestamps fail closed;
14. changed command, handler, policy, bundle, invocation, idempotency, result,
    timeout, duration, exit, or truncation context fails closed;
15. a mock/caller/unavailable handler posture cannot produce accepted proof;
16. ineligible typed status returns no proof without invoking the verifier;
17. eligible typed status always invokes the verifier and every verifier error
    propagates;
18. Debug and errors do not leak paths, arguments, output, environment values,
    IDs, hashes, credentials, or secret-like values;
19. no state backend, event log, artifact store, evidence store, provider, or
    network access is required;
20. `LocalSkillRegistry::new()` remains empty;
21. existing explicit DocsCheck executor tests remain unchanged and green;
22. existing immutable bundle, execution binding, attestation, executor,
    report, proportional-governance, capability, SideEffect, and workspace tests
    remain green; and
23. full formatting, clippy, workspace tests, docs, and diff checks pass.

## 14. Proposed Implementation Sequence

1. Add the crate-private clock boundary and scripted-clock focused tests.
2. Add the crate-internal reusable DocsCheck request/result operation without
   changing current handler behavior.
3. Add the explicit runtime-composition input and read-only outcome.
4. Create the immutable execution binding before invoking the runner.
5. Derive result, observation, candidate, typed eligibility, and accepted proof
   from one execution.
6. Add focused success, honest no-proof, mismatch, ordering, and privacy tests.
7. Run full repository validation.
8. Perform a phase-level maintainer review before any consumer integration.

Keep this as one small `workflow-core` implementation phase.

## 15. Deferred Work

- executor invocation of the composition helper;
- default registration and automatic checks;
- check requirement inference or workflow-declared configuration;
- attestation persistence, cache reuse, events, and audit projection;
- EvidenceReference and WorkReport citations;
- report artifact gates;
- proportional-governance fact consumption;
- approval, capability, authority, or policy enforcement;
- schemas, SDK, CLI, UI, examples, and migration behavior;
- other command families, remote runners, providers, SideEffects, and writes;
- stronger handler provenance and cryptographic assurance.

## 16. Open Questions

- Should honest failed/timed-out results later use an explicit outcome
  disposition enum instead of `Option<AcceptedLocalCheckAttestation>`?
- Should the first crate-private clock return `Timestamp` directly or
  `Result<Timestamp, WorkflowOsError>` to preserve explicit clock failure?
- Should the existing `DocsCheckLocalHandler` later delegate to the same
  request/result operation while retaining its current `SkillOutput` contract?
- What future event should persist an accepted proof reference without storing
  the proof payload or raw output?
- At what boundary should freshness be reevaluated before proportional
  governance treats the proof as a satisfied check?

## 17. Final Recommendation

Perform a focused immutable step/skill attribution blocker-fix review before
adding a consumer integration.

Do not change executor defaults or add persistence, events, evidence, reports,
artifacts, schemas, CLI, additional check families, providers, SideEffects,
writes, hosted behavior, or release changes.
