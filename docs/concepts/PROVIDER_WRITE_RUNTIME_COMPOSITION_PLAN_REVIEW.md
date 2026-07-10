# Provider-Write Runtime Composition Plan Review

## 1. Executive Verdict

Plan accepted; proceed to explicit provider-write runtime composition helper
implementation.

The plan is appropriately narrow. It connects already-reviewed provider-write,
SideEffect, approval-linkage, approval-presentation, event-proof, recovery, and
WorkReport disclosure primitives into one explicit runtime composition lane
without authorizing default executor writes or hidden provider behavior.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization was found for:

- automatic provider writes;
- default executor provider writes;
- hidden provider or auth loading;
- CLI mutation commands;
- workflow-declared provider-write configuration;
- runtime configuration;
- schema or SDK changes;
- examples;
- hosted or distributed runtime;
- generic side-effect execution expansion;
- write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

The plan explicitly frames the next implementation as an additive helper/service
with explicit caller inputs and injected provider behavior.

## 3. Boundary Assessment

The proposed boundary is correct for the current roadmap position. It does not
try to make provider writes a normal executor behavior. Instead, it proposes a
local, explicit, opt-in composition helper for the GitHub PR comment write lane.

That boundary is valuable because the repository now has many safe primitives
that are still too fragmented to prove a full write-adjacent path. A narrow
composition helper is the right next step because it exercises those primitives
together while preserving the project's safety posture.

The plan correctly requires:

- explicit request shape;
- injected provider implementation;
- explicit side-effect store;
- explicit approval-linkage policy;
- explicit approval-presentation policy;
- explicit reconciliation policy;
- explicit workflow event append policy;
- explicit report disclosure inputs;
- no hidden token discovery;
- no fabricated stable identifiers.

## 4. Composition Sequence Assessment

The proposed sequence is sound:

1. Validate provider-write request shape.
2. Validate or construct the proposed/attempted `SideEffectRecord`.
3. Validate approval-side-effect linkage.
4. Validate approval-presentation proof when required.
5. Invoke the injected provider only after pre-provider gates pass.
6. Transition attempted side-effect lifecycle through existing helpers.
7. Build reconciliation posture.
8. Append completed/failed SideEffect workflow events only when eligible.
9. Build bounded WorkReport disclosure posture.
10. Apply event-proof/report-artifact gates only when explicitly requested.
11. Derive recovery posture and next action for ambiguous states.

This order preserves the important invariant: provider invocation must be late,
explicit, and gated. The plan also correctly treats post-provider ambiguity as a
recovery posture rather than an automatic retry path.

## 5. Failure And Gate Clarity Assessment

The plan defines conservative fail-closed behavior before provider invocation
for invalid context, missing or invalid attempted records, approval linkage
failure, approval-presentation proof failure, policy denial, invalid idempotency
binding, and invalid provider target/auth shape.

The gate clarity vocabulary is appropriately bounded. The proposed result should
surface posture for preflight context, attempted record, approval linkage,
approval presentation, attempted lifecycle, provider call, provider response,
post-provider local transition, reconciliation, workflow event proof, report
disclosure, artifact event-proof gate, and operator recovery.

No blocker was found in the failure model. The implementation should be careful
not to duplicate large nested result surfaces if existing helper result types can
be referenced cleanly.

## 6. Report, Artifact, And Recovery Assessment

The plan correctly recommends in-memory report disclosure first and defers
artifact writing unless separately approved. That is the right conservative
choice. The report artifact lane already carries integrity obligations, and
combining it with first provider-write composition would create a larger review
surface than necessary.

Lookup and recovery are also correctly scoped as explicit. The plan does not
authorize automatic lookup, automatic repair, event appending for recovery, or
provider-write retry.

## 7. Privacy And Redaction Assessment

The privacy posture is sufficient for implementation planning. The plan forbids
storage or copying of:

- provider auth tokens;
- raw provider payloads;
- raw GitHub issue or PR bodies;
- raw command output;
- raw source contents;
- raw spec contents;
- environment variable values;
- approval-presentation payload text;
- secret-like values.

It also requires stable error codes and bounded, non-leaking messages. This is
especially important because provider-write composition will sit close to both
auth-bearing provider clients and human approval context.

## 8. Test Plan Assessment

The proposed test plan covers the important behavior:

- all pre-provider gates satisfied invokes injected provider exactly once;
- approval-side-effect linkage failure blocks provider calls;
- approval-presentation proof failure blocks provider calls;
- stale proof blocks provider calls;
- invalid attempted record blocks provider calls;
- provider success/failure transitions lifecycle where eligible;
- post-provider local transition failure creates ambiguous reconciliation and
  blocks retry;
- completed/failed workflow event append occurs only when eligible;
- missing event proof blocks artifact writing when configured;
- WorkReport disclosure remains bounded and non-leaking;
- lookup/recovery posture is explicit for ambiguous state;
- no hidden auth loading occurs;
- default executor behavior remains unchanged;
- no CLI output or filesystem artifacts are created unless explicitly scoped;
- Debug and serialization avoid token-like or payload-like values;
- existing provider-write, WorkReport, SideEffect, approval, and runtime tests
  continue to pass.

One non-blocking improvement: the implementation prompt should require a test
that proves provider invocation is skipped when any earlier gate fails, not just
that the final result reports failure.

## 9. Documentation Review

The plan states that provider-write runtime composition is planned, not
implemented. It also clearly keeps automatic writes, default executor behavior,
CLI mutation, schemas, examples, hosted runtime, generic adapters, reasoning
lineage, recursive agents, and release posture out of scope.

No documentation correction is required.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- In the implementation prompt, require explicit assertion that the injected
  provider is called exactly once only after all pre-provider gates pass.
- Keep report artifact writing out of the first implementation unless a later
  prompt explicitly approves it.
- Prefer reusing existing sub-result types over duplicating gate-detail payloads
  in the composition result.
- Add a short implementation report section that lists every existing primitive
  composed by the helper, so reviewers can confirm the phase connected
  primitives rather than recreating them.

## 12. Recommended Next Phase

Recommended next phase: explicit provider-write runtime composition helper
implementation.

The implementation should remain local, explicit, injected-provider only, and
in-memory first. It should not add automatic provider writes, default executor
integration, hidden auth loading, CLI mutation behavior, schemas, examples,
hosted behavior, generic write-capable adapters, reasoning lineage, report
artifact writing, or release posture changes.

## 13. Validation

- Dogfood workflow: `dg/review`
- Dogfood run: `run-1783724030968788000-2`
- Approval ID:
  `approval/run-1783724030968788000-2/review-scope-approved`
- Approval presentation: `presentation/66fbe0da684dca05`
- Approval presentation hash:
  `66fbe0da684dca0585dfdf8799040459efe7e204bda1cf5c35ba03e22583e505`
- Approval outcome: granted
- Dogfood run status: completed
- Superseded recovery note: an earlier recovered review run
  (`run-1783723658553936000-2`) failed after approval because the hidden
  proof-enforced approval path was invoked without mock local handlers. That run
  was not used as the review boundary; the clean run above was restarted and
  completed with proof enforcement.

Commands run:

- `npm run dogfood:benchmark -- phase-start ...`
- `workflow-os dogfood approval-presentation approve ...`
- `npm run check:docs`
- `git diff --check`
