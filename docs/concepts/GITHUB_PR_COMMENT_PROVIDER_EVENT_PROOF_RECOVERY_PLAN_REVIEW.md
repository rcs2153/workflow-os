# GitHub PR Comment Provider Event-Proof Recovery Plan Review

## 1. Executive Verdict

Plan accepted; proceed to provider event-proof recovery model/helper implementation.

The plan is appropriately bounded and reviewable. It addresses the operator gap left by strict report artifact event-proof gates without weakening those gates or introducing provider calls, automatic repair, workflow event append, report artifact writes, CLI behavior, schemas, examples, hosted behavior, approval-presentation enforcement, or release posture changes.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization was found for:

- implementation in the planning phase;
- provider calls;
- GitHub lookup or query reconciliation;
- automatic retries;
- workflow event append;
- audit or observability emission;
- report artifact writes;
- automatic report generation;
- default executor behavior changes;
- CLI behavior;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- broader write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- approval-presentation enforcement;
- release posture changes.

## 3. Recovery Boundary Assessment

The recovery boundary is conservative and appropriate.

The plan identifies the right problem: strict artifact gates can correctly deny artifact writes when event proof is missing or ambiguous, but operators still need bounded next-action guidance. The proposed first implementation is a local classifier that accepts explicit inputs and returns posture plus next-action vocabulary. That keeps recovery guidance useful without turning it into an implicit repair mechanism.

The plan correctly rejects first-phase provider lookup, automatic event repair, side-effect record mutation, report artifact writes, and CLI display. That matters because recovery is adjacent to enforcement, not a replacement for enforcement.

## 4. Source-Of-Truth Assessment

The plan preserves source-of-truth boundaries.

It treats `WorkflowRunEvent` as the durable event-proof source and does not allow provider references, report text, operator notes, or reconciliation disclosure to stand in for workflow event proof. It also correctly separates provider/local reconciliation candidates from durable event proof, and it keeps `WorkReport` and `WorkReportArtifactRecord` as handoff/artifact surfaces rather than event logs.

This is the right architecture for the current write-readiness lane. Provider/local agreement can inform posture, but artifact writes must not overclaim evidence unless event proof exists.

## 5. Taxonomy And Next-Action Vocabulary Assessment

The recovery posture taxonomy is specific enough for a first implementation.

The proposed categories cover the important failure shapes:

- proof present;
- proof missing;
- proof mismatch;
- provider not called;
- reconciliation required or unavailable;
- ambiguous provider response;
- local transition failure;
- local-state ambiguity;
- unsupported posture.

The next-action vocabulary is also appropriately bounded. Labels such as `inspect_workflow_events`, `manual_provider_lookup_required`, and `artifact_write_blocked_pending_event_proof` guide an operator without authorizing commands, provider calls, state repair, event append, or artifact writes.

One non-blocking follow-up is to ensure the implementation names distinguish "artifact write may proceed" from "no recovery action required." Those can be correlated, but they should not become the same state.

## 6. Workflow Semantics Assessment

The plan preserves workflow semantics.

The helper is specified as an input-to-classification function only. It must not mutate `WorkflowRun`, append workflow events, emit audit or observability events, call providers, mutate side-effect records, write report artifacts, create filesystem artifacts, or expose CLI output.

That is the right boundary. Recovery classification failure should not change workflow pass/fail status or silently authorize retry.

## 7. Privacy And Redaction Assessment

The privacy posture is strong.

The plan allows stable posture codes, stable caller-supplied references, bounded reason and next-action codes, and validated redaction metadata. It forbids raw GitHub responses, comment bodies, PR bodies, diffs, review threads, file contents, authorization headers, tokens, credentials, environment values, CI logs, command output, parser payloads, raw specs, and unbounded operator notes.

The implementation should preserve this by using stable validation errors and redaction-safe `Debug` and serde behavior. No blocker was found in the plan.

## 8. Error-Handling Assessment

The proposed error boundary is appropriate.

Stable candidate error codes are listed, and the plan explicitly forbids leaking raw metadata, provider payloads, comment bodies, repository paths, URLs with private identity, command output, raw IDs when sensitive, tokens, credentials, raw specs, parser payloads, or secret-like values.

The future implementation should keep invalid-input failures distinct from supported recovery postures. A malformed input should be a validation error; a valid but unsafe provider/local posture should be a successful recovery classification that blocks retry or artifact write as appropriate.

## 9. Test Plan Assessment

The future test plan is sufficient for the first implementation.

It covers every proposed recovery posture, retry blocking, artifact-write blocking, non-provider-call behavior, non-event-append behavior, non-mutation behavior, non-artifact-write behavior, stable non-leaking errors, redaction-safe debug/serialization, and existing provider-write/report/artifact-gate regressions.

Non-blocking follow-up: include at least one test proving event-proof-present classification does not itself write the artifact. The gate may proceed, but the recovery helper should not perform the write.

## 10. Documentation Review

The documentation is honest about implemented and deferred behavior.

The plan and phase report both state that recovery is planned, not implemented. They also state that provider lookup, automatic repair, event append, report artifact writes, CLI behavior, schemas, examples, hosted behavior, approval-presentation enforcement, and release posture changes remain unimplemented.

No dangerous false claims were found.

## 11. Planning Blockers

No planning blockers.

## 12. Non-Blocking Follow-Ups

- Keep "artifact write may proceed" distinct from "no recovery action required" in the implementation model.
- Add a future test proving event-proof-present recovery classification does not itself write report artifacts.
- Keep provider lookup/query reconciliation as a separate plan before any implementation attempts to inspect GitHub state.

## 13. Recommended Next Phase

Recommended next phase: provider event-proof recovery model/helper implementation, local classification only.

Implement only the local model/helper slice described by the plan: recovery posture vocabulary, next-action vocabulary, explicit input/result types, validation, redaction-safe behavior, and focused tests. Do not implement provider lookup, automatic repair, event append, artifact write composition, CLI behavior, schemas, examples, hosted behavior, broader writes, reasoning lineage, approval-presentation enforcement, or release posture changes.

## 14. Validation

- `npm run check:docs`: required for this review update.

## 15. Governed Dogfood

- Workflow: `dg/review`.
- Run: `run-1783311707429643000-2`.
- Approval: `approval/run-1783311707429643000-2/review-scope-approved`.
- Approval outcome: granted under delegated maintainer authority after the complete approval handoff block was surfaced.
