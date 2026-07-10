# Provider-Write Runtime Composition Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The helper provides the intended explicit, local, in-memory runtime composition
boundary for the GitHub PR comment provider-write lane. It composes reviewed
executor, approval-presentation, provider-call, reconciliation, workflow-event
proof, and WorkReport disclosure primitives without making provider writes
automatic or widening write support.

## 2. Scope Verification

The phase stayed within approved helper scope.

No accidental implementation was found for:

- automatic provider writes;
- default executor provider writes;
- hidden provider or auth loading;
- runtime config;
- CLI mutation commands;
- workflow schema changes;
- SDK changes;
- examples;
- report artifact writing;
- provider lookup/recovery automation;
- write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters;
- reasoning lineage;
- hosted or distributed runtime;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

The helper is exported from `workflow-core`, but remains an explicit opt-in API.
`LocalExecutor::execute(...)` is unchanged.

## 3. Helper API Assessment

The implemented API is appropriately narrow:

- `GitHubPrCommentProviderWriteRuntimeCompositionRequest`;
- `GitHubPrCommentProviderWriteRuntimeCompositionResult`;
- `GitHubPrCommentProviderWriteRuntimeCompositionParts`;
- `compose_github_pr_comment_provider_write_runtime(...)`.

The request wraps
`LocalExecutionWithGitHubPrCommentProviderWritePresentationGateRequest`, which
keeps the caller responsible for explicit execution inputs, attempted
side-effect/provider-call inputs, reconciliation metadata, and
approval-presentation policy.

The result exposes the existing provider-write result and a bounded report
disclosure projection. This is preferable to duplicating all sub-result fields
because it keeps the composition helper as an assembly boundary rather than a
new provider-write model family.

## 4. Composition Assessment

The implementation delegates to
`execute_with_github_pr_comment_provider_write_presentation_gate(...)`.

That composition path covers:

- local workflow execution;
- terminal-run requirement;
- approval-presentation proof enforcement;
- injected provider invocation only after gates pass;
- provider/local lifecycle transition;
- reconciliation;
- eligible completed/failed workflow event append;
- bounded report disclosure projection.

This is the right first implementation slice. It composes existing primitives
without inventing new provider behavior.

## 5. Gate And Failure Assessment

The helper preserves conservative gate behavior:

- missing approval-presentation proof blocks provider invocation;
- provider is not called when the proof gate fails;
- provider-write errors remain structured and non-leaking;
- execution errors before a run exists are returned unchanged;
- report artifact posture remains false;
- default executor behavior remains unchanged.

The result exposes bounded gate clarity through the existing
`GitHubPullRequestCommentProviderWriteGateClarity`.

One limitation is intentional: approval-side-effect linkage and attempted-record
validity are enforced through the nested existing provider-write inputs and
helpers rather than through separate fields on the new composition request.
That keeps the first helper small and avoids parallel validation logic.

## 6. Report, Artifact, And Recovery Assessment

The helper correctly returns in-memory report disclosure only.

It does not:

- generate a full `WorkReport`;
- write a report artifact;
- perform lookup/recovery;
- retry provider writes;
- append recovery events;
- repair provider/local ambiguity.

This is aligned with the reviewed plan. Artifact and recovery composition should
remain separate future phases.

## 7. Privacy And Redaction Assessment

The helper does not store or copy:

- provider auth tokens;
- raw provider payloads;
- raw GitHub issue or PR bodies;
- raw command output;
- raw source contents;
- raw spec contents;
- environment variable values;
- approval-presentation payload text;
- secret-like values.

Request and result `Debug` implementations are bounded. The focused tests check
that token-like values, comment body text, provider references, and side-effect
IDs do not appear in the new request/result debug surfaces.

## 8. Test Quality Assessment

Focused tests cover:

- successful composition after approval-presentation proof;
- injected provider called exactly once after gates pass;
- workflow event proof appended on successful provider/local completion;
- bounded report disclosure for success;
- missing proof blocking provider invocation;
- blocked provider-call gate posture;
- stable non-leaking error code for missing proof;
- no report artifact writes;
- request/result Debug non-leakage.

The broader existing provider-write suite still covers lower-level invalid
attempted records, provider-call gate failures, provider success/failure
transitions, reconciliation ambiguity, event append failures, and disclosure
posture.

No blocker test gap was found. A non-blocking future improvement would be a
composition-level test that fails an attempted-record/provider-call gate through
the wrapper, mainly as smoke coverage that those lower-level failures remain
visible at the composition boundary.

## 9. Documentation Review

Documentation now states that:

- the first explicit in-memory runtime composition helper is implemented;
- automatic provider writes are not implemented;
- default executor provider writes are not implemented;
- hidden auth loading is not implemented;
- CLI mutation commands are not implemented;
- workflow schema changes are not implemented;
- examples are not updated;
- report artifacts are not written by this helper;
- lookup/recovery automation is not implemented;
- broader write-capable adapters are not implemented;
- reasoning lineage and hosted runtime remain out of scope.

The phase report accurately records the helper API, validation posture, dogfood
run, and remaining limitations.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a composition-level smoke test for a lower-level provider-call gate
  failure, such as invalid attempted record or disabled provider call.
- Keep artifact-gated provider-write composition separate from this helper.
- Keep lookup/recovery composition separate and explicit.
- Continue avoiding default executor provider-write behavior until the
  approval, artifact, recovery, and operator UX surfaces are reviewed together.

## 12. Recommended Next Phase

Recommended next phase: artifact-gated provider-write composition planning.

The explicit provider-write composition helper now proves the in-memory
runtime path. The next useful planning step is not broader provider writes; it
is deciding whether and how this explicit path should compose with report
artifact event-proof gates without making artifact writing or provider writes
default.

## 13. Validation

Dogfood governance:

- workflow: `dg/review`;
- run: `run-1783727337968080000-2`;
- approval: `approval/run-1783727337968080000-2/review-scope-approved`;
- presentation: `presentation/33020b1c790ce4ae`;
- presentation hash:
  `33020b1c790ce4ae9ccc4369c3b1c757337cbe41c4b923643e0a0b28713abca4`;
- approval outcome: granted;
- run status: completed.

Commands:

- `npm run dogfood:benchmark -- phase-start --phase review ...` - passed;
- `workflow-os dogfood approval-presentation approve ...` - passed;
- `npm run check:docs` - required for this review phase.
