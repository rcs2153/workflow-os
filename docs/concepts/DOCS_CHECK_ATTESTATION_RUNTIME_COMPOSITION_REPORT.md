# DocsCheck Attestation Runtime Composition Report

## 1. Executive Summary

Workflow OS now has one explicit crate-internal helper that executes the
existing bounded `DocsCheck` process path and returns an in-memory structured
result plus an accepted independent attestation when the typed requirement is
satisfied.

The helper freezes immutable run, command, handler, effective policy, and
invocation context before the process starts. Core owns observation time,
result construction, candidate construction, eligibility, and verifier
invocation. No executor default, persistence, event, evidence, report, artifact,
schema, CLI, provider, SideEffect, write, hosted, or release behavior changed.

## 2. Scope Completed

- Added `execute_docs_check_with_attestation` as a crate-internal composition
  boundary.
- Added an explicit borrowed input and read-only in-memory outcome.
- Added a crate-private injectable clock with four Core-owned time samples.
- Reused the existing `DocsCheckLocalHandler` request, runner, redaction, and
  structured-result path.
- Created the immutable execution binding before invoking the process runner.
- Derived the Core observation and unverified candidate from one process output.
- Invoked the accepted verifier only for typed eligible statuses.
- Returned failed and timed-out structured results without fake proof.

## 3. Scope Explicitly Not Completed

- automatic or default local check execution;
- executor, registry-default, workflow-schema, or CLI integration;
- persistence, cache reuse, events, audit projection, evidence, reports, or
  artifacts;
- proportional-governance, approval, authority, or capability consumption;
- additional command families, providers, SideEffects, or writes;
- stronger handler binary or host provenance;
- hosted, distributed, cryptographic, hardware-backed, or remote attestation;
- examples, SDKs, migrations, or release posture changes.

## 4. Helper API Summary

`DocsCheckAttestationExecutionInput` accepts a validated stored immutable run
bundle, independent-check requirement, explicit `DocsCheckLocalHandler`, exact
workflow/run/step/invocation/idempotency/result/attestation identities, and a
crate-private clock. Skill identity is resolved from the selected canonical
stored workflow step and exact stored skill record rather than accepted from
the caller.

It does not accept process output, observation, candidate, binding timestamps,
evaluation timestamps, raw command text, environment values, or proof claims.
The outcome exposes only the validated `LocalCheckResult` and an optional
read-only `AcceptedLocalCheckAttestation`.

## 5. Ordering And Authority Boundary

The helper validates stored-manifest workflow/run identity and command
requirements before sampling time. It then creates the immutable execution
binding, builds the bounded request, samples process start, invokes the runner,
samples completion, constructs the result and observation, constructs the
candidate, evaluates typed status eligibility, and, only when eligible, samples
evaluation time and invokes the verifier.

Impossible clock ordering, identity mismatch, request failure, runner failure,
redaction failure, candidate inconsistency, or verifier rejection returns a
stable non-leaking error and no partial outcome.

## 6. Result And Proof Semantics

A passed result accepted by the requirement must pass the verifier before the
outcome contains proof. Failed or timed-out results that are not accepted by the
requirement return their structured result and no proof without interpreting a
verifier error as ordinary failure.

Every verifier error after typed eligibility propagates. Publicly recomputable
fingerprints remain commitments, not authenticity. The accepted assurance is
still honestly limited to `KernelObservedLocalProcess` with a
`RegisteredUnattested` handler.

## 7. Compatibility

The existing explicit `DocsCheckLocalHandler` still implements `SkillHandler`
with the same request, process, result, and `SkillOutput` behavior. It now
delegates to shared crate-internal request and runner operations so runtime
composition does not duplicate environment, timeout, output-bounding, or
redaction logic.

`LocalSkillRegistry::new()` remains unchanged and empty. Existing executor
behavior does not call the new helper.

## 8. Privacy And Redaction

The binding, observation, candidate, and accepted proof remain payload-free.
They contain no executable path, repository path, npm-cache path, arguments,
environment values, source contents, raw stdout/stderr, credentials, tokens, or
provider payloads.

The result retains only existing bounded redacted summaries. Input and outcome
Debug implementations redact identities, paths, fingerprints, results, proof,
and clock details. New errors contain stable codes and bounded static messages.

## 9. Test Coverage

Focused tests prove:

- passed `DocsCheck` produces a structured result and accepted proof;
- binding creation and process-start sampling precede runner invocation;
- the clock is sampled exactly four times for accepted proof;
- failed and timed-out checks return structured no-proof outcomes with no
  verifier-evaluation clock sample;
- stored-manifest identity mismatch fails before clock or runner use;
- backward time fails before process execution;
- runner failures return no partial outcome;
- an eligible stale proof propagates the verifier failure; and
- Debug output does not expose supplied identities.

Existing verifier regression coverage independently protects command, handler,
policy, bundle, invocation, idempotency, result, duration, truncation,
freshness, substitution, and non-leakage boundaries.

## 10. Governed Phase

- workflow: `dg/runtime-composition`
- run: `run-1784644609560548000-2`
- approval: `approval/run-1784644609560548000-2/composition-approved`
- presentation: `presentation/2c4bfeba5030306e`
- approval outcome: granted by delegated maintainer through proof enforcement
- event summary: 39 events, one approval, zero retries, zero escalations
- kernel boundary: governance coordination only; implementation, tests,
  documentation, and validation ran outside the kernel

## 11. Validation

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`

All commands passed at phase close.

## 12. Remaining Limitations

- the helper is crate-internal and has no executor consumer;
- accepted proof is in memory only and is not evented, persisted, cited, or
  reused;
- freshness must be reevaluated by any future consumer;
- local runner and registered handler implementation provenance remain
  unattested;
- only canonical `DocsCheck` is composed; and
- the known dogfood phase-close presentation-record read cap remains separate.

## 13. Recommended Next Phase

Phase-level review found an immutable step/skill attribution blocker. The fix is
now implemented in
[DocsCheck Attestation Runtime Composition Blocker Fix Report](DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_BLOCKER_FIX_REPORT.md),
and accepted by
[DocsCheck Attestation Runtime Composition Blocker Fix Review](DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_BLOCKER_FIX_REVIEW.md).
Consumer integration planning is next. Do not add executor consumption,
automatic checks, persistence, events, evidence, reports, artifacts, schemas,
CLI, additional command families, providers, SideEffects, writes, hosted
behavior, or release changes without separately governed scope.
