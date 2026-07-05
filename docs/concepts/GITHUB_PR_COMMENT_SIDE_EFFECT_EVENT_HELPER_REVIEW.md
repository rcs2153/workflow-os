# GitHub PR Comment SideEffect Event Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The phase correctly adds a pure helper for constructing a reference-only `SideEffectProposed` workflow event payload from an already persisted proposed GitHub pull request comment `SideEffectRecord`. It stays inside the intended no-provider-call, no-event-append, no-audit-sink, no-report-artifact, no-CLI, no-schema, and no-release-posture-change boundary.

It is suitable to proceed to the next bounded phase: explicit executor append planning or implementation for the proposed event path, still without provider mutation.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- `GitHubPullRequestCommentSideEffectEventContext`;
- `compose_github_pr_comment_proposed_side_effect_event(...)`;
- `load_github_pr_comment_proposed_side_effect_event(...)`;
- read-only `SideEffectRecord` accessors needed for event construction;
- focused provider-write tests;
- roadmap, integration, implementation-plan, and phase-report documentation.

No accidental implementation found for:

- GitHub provider calls;
- GitHub pull request comment mutation;
- live sandbox write behavior;
- runtime side-effect execution;
- `SideEffectAttempted`, `SideEffectCompleted`, or `SideEffectFailed` behavior;
- automatic workflow event append;
- audit sink emission or dedicated audit storage;
- report artifact writes;
- automatic executor integration;
- CLI behavior;
- workflow schemas;
- examples;
- hosted or distributed behavior;
- reasoning lineage;
- autonomy expansion;
- release posture changes.

## 3. Helper API Assessment

The helper API is narrow and appropriate:

```text
compose_github_pr_comment_proposed_side_effect_event(
    record: &SideEffectRecord,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError>
```

The store-backed helper is similarly explicit:

```text
load_github_pr_comment_proposed_side_effect_event(
    store: &impl SideEffectRecordStore,
    side_effect_id: &SideEffectId,
    context: &GitHubPullRequestCommentSideEffectEventContext,
) -> Result<SideEffectWorkflowEvent, WorkflowOsError>
```

Both helpers accept explicit caller-supplied state rather than reading hidden global runtime state. They return a validated in-memory event payload and do not append that payload to a `WorkflowRun`.

## 4. Event Construction Assessment

The constructed event is appropriately reference-only.

The event carries:

- `SideEffectId`;
- lifecycle `Proposed`;
- optional step ID;
- optional skill ID and skill version;
- optional correlation ID;
- existing stable side-effect references;
- evidence reference count;
- zero outcome reference count;
- sensitivity;
- redaction metadata.

It does not copy the full `SideEffectRecord`, generated comment body, raw provider payloads, pull request bodies, diffs, command output, CI logs, file contents, spec contents, credentials, authorization headers, environment variable values, or provider references.

## 5. Validation Boundary Assessment

The helper fails closed unless:

- the record lifecycle is `Proposed`;
- the capability is `GitHubWrite`;
- the target is an adapter resource shaped like a GitHub pull request target;
- no outcome reference is present;
- workflow ID matches the expected context;
- workflow version matches the expected context;
- schema version matches the expected context;
- spec hash matches the expected context;
- run ID matches the expected context.

The event is then constructed through `SideEffectWorkflowEvent::new(...)`, keeping the existing SideEffect event validation boundary active.

No blocker found. One non-blocking follow-up is to centralize or strengthen the GitHub pull-request target classifier before executor append integration, so later phases do not spread ad hoc target-shape checks across multiple helpers.

## 6. Error Handling Assessment

Errors use stable, non-leaking codes:

- `github_pr_comment_side_effect_event.unsupported_lifecycle`;
- `github_pr_comment_side_effect_event.unsupported_capability`;
- `github_pr_comment_side_effect_event.unsupported_target`;
- `github_pr_comment_side_effect_event.outcome_not_supported`;
- `github_pr_comment_side_effect_event.identity_mismatch`;
- `github_pr_comment_side_effect_event.store_read_failed`;
- `github_pr_comment_side_effect_event.record_missing`;
- `github_pr_comment_side_effect_event.event.invalid`;
- `github_pr_comment_side_effect_event.reference_count.invalid`.

The helper does not include raw SideEffect IDs, run IDs, target references, summaries, spec hashes, provider references, redaction metadata values, or secret-like values in error messages. Store read failures are mapped to a stable GitHub-specific code rather than forwarding raw store details.

## 7. Privacy And Redaction Assessment

The phase preserves the existing redaction posture.

The helper does not store, construct, copy, or emit:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw pull request bodies;
- raw diffs;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompts;
- secret-like values.

`GitHubPullRequestCommentSideEffectEventContext` has redaction-safe `Debug` behavior for spec hash and run identity. The returned `SideEffectWorkflowEvent` uses the existing redaction-safe event model.

## 8. Relationship To Workflow Events, Audit, And Reports

The phase correctly separates event payload construction from event acceptance.

It does not append the proposed event to a workflow event stream. It does not project an audit event. It does not mutate a `WorkReport` or write a report artifact.

This keeps the important source-of-truth boundary intact:

- `SideEffectRecord` remains durable proposed write intent.
- `SideEffectWorkflowEvent` remains an in-memory event payload until explicitly accepted by a runtime append path.
- `AuditEvent` remains a bounded projection of accepted workflow events, not of arbitrary helper output.

## 9. Relationship To Write Adapter Readiness

This is a useful write-readiness increment. A future GitHub PR comment write path needs durable proposed write intent and event/audit traceability before any provider mutation is considered.

This phase still does not authorize live GitHub writes. The next work should remain event/audit/report-composition oriented and should not skip to provider mutation.

## 10. Test Quality Assessment

Tests cover:

- composing a proposed event from a persisted proposed record;
- store-backed loading by stable `SideEffectId`;
- missing store record failure without leaking the missing ID;
- non-proposed lifecycle rejection;
- workflow/run identity mismatch rejection without leaking IDs or target details;
- Debug and serialization non-leakage for raw provider/spec/comment markers;
- existing provider-write request, fixture, response, composition, and persistence behavior.

The tests are appropriately focused for the helper-only phase.

Missing or shallow coverage:

- no direct test mutates capability to prove `unsupported_capability`;
- no direct test mutates target kind/shape to prove `unsupported_target`;
- no direct test adds an outcome reference to prove `outcome_not_supported`;
- no direct test forces `store_read_failed` with a failing store.

These are non-blocking because the implemented branches are simple and the broader SideEffect model tests already cover event validation, but they are good regression candidates before executor append integration.

## 11. Documentation Review

Documentation now states:

- the pure proposed-event construction helper is implemented;
- workflow event append behavior is not implemented by the helper;
- audit sink emission is not implemented;
- runtime side-effect execution is not implemented;
- provider mutation and live sandbox writes are not implemented;
- report artifact writes are not implemented;
- automatic executor integration is not implemented;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, autonomy expansion, and release posture changes remain unimplemented.

The docs preserve the distinction between persisted proposed intent and accepted runtime history.

## 12. Validation Review

The implementation report records successful validation:

- `cargo fmt --all`;
- `cargo test -p workflow-core --test provider_write`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

This review reran the required validation commands after creating the review document.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add direct regression tests for unsupported capability, unsupported target, outcome reference rejection, and store read failure mapping before or during executor append integration.
- Centralize or strengthen GitHub pull-request target classification before multiple runtime paths depend on it.
- Decide in the next phase whether event append should require loading from `SideEffectRecordStore` every time or accept an already-loaded validated record from a caller-controlled transaction boundary.

## 15. Recommended Next Phase

Recommended next phase: explicit executor append planning or implementation for the GitHub PR comment proposed SideEffect event path, still no provider mutation.

Rationale: the system can now compose and persist proposed GitHub PR comment write intent, then construct a valid proposed workflow event payload from that durable record. The next runtime-composition gap is an explicit opt-in append path that accepts this proposed event into workflow history and allows existing bounded audit projection to operate, without attempting or completing a provider write.
