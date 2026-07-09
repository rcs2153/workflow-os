# GitHub PR Comment Provider Lookup Reconciliation Model Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The GitHub PR comment provider lookup/query reconciliation model/helper stays within the approved explicit injected-client boundary. It gives Workflow OS a bounded way to classify provider-side observations after event-proof ambiguity without treating provider lookup as durable Workflow OS event proof.

Recommended next phase: concrete injected GitHub PR comment lookup HTTP client planning.

## 2. Scope Verification

The phase stayed within model/helper scope.

Implemented scope:

- lookup observation, response, posture, next-action, request, client trait, and reconciliation result model;
- explicit `GitHubPullRequestCommentProviderLookupClient` boundary;
- caller-supplied auth carried into the validated request;
- deterministic lookup matching by exact provider reference or bounded managed marker;
- redaction-safe debug behavior;
- focused tests;
- roadmap, implementation-plan, and report updates.

No accidental implementation was found for:

- automatic provider lookup;
- hidden auth loading;
- GitHub comment creation, update, or deletion;
- automatic provider retries;
- workflow event append;
- workflow event repair;
- side-effect record mutation;
- manual state repair;
- report artifact writes;
- CLI behavior;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- broader write-capable adapter expansion;
- reasoning lineage;
- approval-presentation enforcement;
- release posture changes.

## 3. Model And Helper Assessment

The model is appropriately narrow for a recovery-adjacent provider observation boundary.

Verified model surfaces:

- `GitHubPullRequestCommentProviderLookupOutcome`;
- `GitHubPullRequestCommentProviderLookupObservation`;
- `GitHubPullRequestCommentProviderLookupResponse`;
- `GitHubPullRequestCommentProviderLookupReconciliationPosture`;
- `GitHubPullRequestCommentProviderLookupReconciliationNextAction`;
- `GitHubPullRequestCommentProviderLookupReconciliationInput`;
- `GitHubPullRequestCommentProviderLookupRequest`;
- `GitHubPullRequestCommentProviderLookupClient`;
- `GitHubPullRequestCommentProviderLookupReconciliationResult`;
- `reconcile_github_pr_comment_provider_lookup`.

The helper constructs a validated request, calls only the supplied injected client, validates the bounded response, classifies the observation, and returns a bounded reconciliation result. It does not read hidden global state, load auth, call GitHub directly, write local state, append events, or emit CLI output.

## 4. Matching Policy Assessment

The matching policy is conservative and aligned with the plan.

Accepted match signals:

- exact attempted-record target match before lookup;
- exact provider comment reference;
- exact bounded managed marker.

Rejected or absent match behavior:

- no raw comment body matching;
- no fuzzy text matching;
- no model interpretation of provider comments;
- no repository path inference;
- no matching on raw provider payloads, command output, parser payloads, source contents, or secrets.

Completed lookup responses with no observations produce `RemoteCommentAbsent`. Completed lookup responses with conflicting or non-deterministic observations produce `RemoteCommentAmbiguous` or fail closed if a required provider reference is missing.

## 5. Event-Proof And Artifact Boundary Assessment

The implementation preserves the source-of-truth boundary:

- provider lookup remains an observation;
- durable workflow event proof remains separate;
- successful remote observation does not append a workflow event;
- successful remote observation does not mutate a `SideEffectRecord`;
- successful remote observation does not write a report artifact;
- successful remote observation does not permit strict artifact write.

The result explicitly reports:

- `workflow_event_appended=false`;
- `side_effect_record_mutated=false`;
- `report_artifact_written=false`;
- `cli_output_emitted=false`;
- `artifact_write_blocked=true`;
- `artifact_write_may_proceed=false`.

This is the right posture before any future manual repair or artifact-write composition phase.

## 6. Validation And Error Handling Assessment

Validation is deterministic and fail-closed.

Verified behavior:

- attempted records must already be in `Attempted` lifecycle state;
- target identity must validate and match the attempted record;
- auth must be caller supplied and valid;
- provider references are bounded and validated;
- managed markers are bounded and validated;
- response observation count is bounded;
- non-completed lookup outcomes require stable provider error codes;
- redaction metadata is validated;
- client failures are mapped to a stable lookup-unavailable error;
- validation errors avoid raw target/path/token/provider payload leakage.

The helper maps unclassified injected-client errors to a stable non-leaking error. That avoids exposing raw provider payloads while still signaling that lookup did not return a bounded response.

## 7. Privacy And Redaction Assessment

The implementation is redaction-safe for the reviewed boundary.

Verified:

- `Debug` redacts side-effect IDs, idempotency keys, provider references, managed markers, auth values, and redaction metadata;
- result serialization is constructor-validated on deserialization;
- secret-like expected markers are rejected before lookup;
- secret-like observed provider references are rejected;
- client errors containing secret-like payloads are not propagated;
- raw provider responses, comment bodies, PR bodies, diffs, review threads, file contents, CI logs, command output, raw specs, parser payloads, authorization headers, and environment values are not stored by the helper.

The response and result may serialize validated redaction metadata, following the current core-model pattern. Because redaction metadata validation is enforced at construction/deserialization, this is acceptable for this phase.

## 8. Test Quality Assessment

The tests are strong for the approved slice.

Covered:

- remote comment observed by exact provider reference;
- remote comment observed by bounded managed marker;
- absent remote comment;
- ambiguous multiple matches;
- denied, unavailable, rate-limited, and untrusted lookup outcomes;
- target mismatch rejection before client invocation;
- secret-like marker rejection without leakage;
- unclassified client error mapping without leakage;
- debug and serialization non-leakage;
- serde round trip and invalid wire failure;
- artifact writes remain blocked after successful remote observation;
- no workflow event append, side-effect mutation, report artifact write, or CLI output.

Existing provider-write tests continue to cover adjacent provider-call, reconciliation, event append, report disclosure, artifact gate, and side-effect boundaries.

Non-blocking test follow-ups:

- add a focused test for completed lookup responses with non-matching observations producing `RemoteCommentAmbiguous`;
- add a focused test that a completed response with one marker match but missing provider reference fails closed;
- add a focused test for response observation-count bounds.

These are useful edge cases but do not block acceptance because the existing implementation path already validates the relevant shapes and the current tests cover the primary safety boundaries.

## 9. Documentation Review

Documentation is honest and scoped.

Verified docs state:

- the first explicit lookup reconciliation model/helper is implemented;
- automatic provider lookup is not implemented;
- hidden auth loading is not implemented;
- provider writes/retries are not implemented by this phase;
- workflow event append and repair are not implemented by this phase;
- side-effect record mutation is not implemented by this phase;
- manual state repair is not implemented;
- report artifact writes are not implemented;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, approval-presentation enforcement, and release posture changes remain unimplemented.

The report correctly recommends review before a concrete lookup HTTP client, CLI recovery surface, manual state repair helper, or artifact-write composition consumes this boundary.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add the three edge-case tests listed in the test quality assessment.
- Plan a concrete injected GitHub PR comment lookup HTTP client that returns this bounded response model.
- Keep any future concrete client explicit opt-in with caller-supplied auth and injected transport.
- Keep manual state repair, artifact-write composition, and CLI recovery commands separately planned and reviewed.

## 12. Recommended Next Phase

Recommended next phase: concrete injected GitHub PR comment lookup HTTP client planning.

Why: the model/helper now provides a reviewed local boundary, but no concrete lookup client exists. The next safe step is to plan the injected HTTP client surface without hidden auth, automatic lookup, event repair, state mutation, artifact writes, CLI behavior, schemas, examples, hosted behavior, or release posture changes.

## 13. Validation

Run during review:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
