# Authorized Execution Continuity Semantic V2 Review

## 1. Executive Verdict

**Needs blocker fixes. Do not proceed to the SQLite V2 continuity backend.**

The additive semantic amendment preserves the narrow backend and runtime
scope, but focused review found four correctness and compatibility blockers in
the shared semantic oracle. Historical security-rejection replay is coupled to
mutable current trusted-time state, terminal windows can be classified as
blocked, exact-target validation does not verify every authoritative record's
embedded identity, and V1 wire behavior changed despite the compatibility
claim.

These must be fixed and independently re-reviewed before SQLite copies the
semantics. The phase still does not accept durable continuity, automatic
executor resume, runtime scheduling, approval automation, provider mutation,
or hosted execution.

## 2. Scope Verification

The phase stayed within the approved semantic-amendment scope. It added:

- one additive public V2 contract and fixed V2 version;
- one explicit local-live-state-only support scope;
- private committed-success and committed-security-rejection dispositions;
- private epoch-bound trusted-time and live-instance eligibility semantics;
- consume-by-value, non-cloneable authority for directive consumption;
- private capability-free operation reconciliation;
- exact wait identity and authoritative continuation classification;
- focused public contract and private reference tests; and
- implementation and roadmap documentation.

It did not add SQLite tables or transactions, backend migration, runtime
events, executor or supervisor integration, automatic approval, external
effects, provider calls, CLI behavior, workflow-schema fields, SDK behavior,
hosted operation, nested harness execution, or release-posture changes.

## 3. Public Contract And Compatibility Assessment

The V1 contract remains source compatible at the named-type and constructor
level. Its exhaustive
`AuthorizedExecutionContinuityStateContractVersion` still contains only `V1`,
and the V2 declaration is a separate type rather than a new variant in that
enum, so exhaustive downstream matches do not break.

The wire-compatibility claim is not yet true. The V1 constructor now sorts
operation entries, changing the serialized order previously preserved from a
valid caller input. Custom V1 deserialization also rejects unknown contract and
entry fields that the prior derived/wire behavior accepted. The blocker fix
must either preserve the accepted V1 behavior or explicitly approve and
document a contract break with migration guidance; this review does not
authorize the latter.

`AuthorizedExecutionContinuityStateContractV2` accepts only a complete,
duplicate-free set of supported operation families under
`LocalLiveStateOnly`. Mixed or partial support fails closed. Custom enum and
contract deserialization uses fixed bounded errors and rejects unknown fields
without echoing caller-controlled values.

The public V2 declaration is capability vocabulary, not an implementation
claim. No production backend implements or returns it. Local filesystem,
SQLite, and PostgreSQL retain their V1 unsupported declarations.

## 4. Trusted-Time And Security-Rejection Assessment

The reference store owns the clock source, expected provenance commitment,
and fixed epoch. Mutation requests cannot supply authoritative time. New
operations require coherent source, provenance, epoch, global watermark,
window watermark, and expiry checks.

Regression, incompatible provenance, epoch mismatch, and expiry can commit a
bounded security-rejection disposition together with the exact trusted-time
transition and receipt. Exact replay resolves the committed operation before
observing time again. Rejection validation recomputes its commitment and
checks the legal prior-to-resulting security transition.

The current authoritative-row check is too strong. It requires the mutable
current trusted-time and window snapshots to equal the historical rejection's
resulting snapshots. After a valid expiry rejection, a later valid operation
on another window can advance the global trusted-time watermark and revision.
Exact replay of the earlier operation then reports corrupt state, and read-only
reconciliation reports unreadable state. Historical replay must validate the
committed transition and prove that current state is a legal successor, not
require current global state to remain frozen forever.

After the historical replay blocker is fixed and re-reviewed, these semantics
may become sufficient for the SQLite implementation to copy. They do not
provide an external rollback-resistant epoch anchor or certify arbitrary
backup restore.

## 5. Authority, Atomicity, And Reconciliation Assessment

Directive consumption takes an owning request containing a private,
non-cloneable authority-use capability. Crossing the mutation boundary burns
that capability regardless of success, rollback, or ambiguity. Exact replay
returns the committed bounded result without issuing another attempt
capability.

Successful directive consumption intends to bind one exact window, directive, yield
generation, attempt, cursor, subject, authority commitment, request
commitment, receipt, and resulting state. Attempt outcome recording requires
the private attempt capability produced by that exact successful consume.
Ambiguous recovery remains capability-free but can only classify an existing
started attempt under exact expected bindings.

The replay validator retrieves authoritative rows by map key but does not
consistently compare each row's embedded `window_id`, `attempt_id`,
`generation_id`, `directive_id`, or wait identity with that key and the
committed result. A corrupted embedded identity can therefore survive some
successful-result validation paths. The reference oracle must reject every
key/record/result identity mismatch before it can define the SQLite ownership
contract.

Read-only reconciliation accepts only operation ID, expected request
commitment, and expected receipt ID. It either returns the validated committed
disposition, confirms absence, or reports unreadable state. It cannot recreate
authority or mutate state.

The transaction oracle retains one-winner and exact-replay behavior for all
five operation families. Its fault points distinguish pre-commit rollback,
commit ambiguity, and durable committed state. These properties remain a
reference specification rather than a production durability claim.

## 6. Continuation Classification Assessment

The private reader defines one kernel-owned classification vocabulary:

- `ResumeNow` only for a live eligible, trusted, unexpired yielded window with
  no unsatisfied or terminal wait;
- `AwaitCondition` only for an exact active yield with an unsatisfied typed
  wait;
- `Blocked` for quarantine, restore uncertainty, stale or unavailable trusted
  time, executing or recovery-required state, malformed ownership, or a
  terminalized wait; and
- `Terminal` for closed, expired, revoked, or superseded windows.

The implementation checks eligibility, clock availability, regression, and
expiry before matching the persisted window state. A committed `Expired`
window necessarily has an observation at or after expiry, so it cannot reach
the `Terminal` branch. `Closed`, `Revoked`, and `Superseded` can likewise
become `Blocked` after expiry or when trusted time is unavailable. Terminal
classification must derive from validated persisted terminal state without
requiring fresh live execution eligibility.

An executing window is correctly blocked because no live attempt lease yet
proves whether an executor still owns it. That conservative behavior is not a
substitute for correct terminal-state classification.

## 7. Privacy And Error Assessment

The V2 models and private state contain bounded identifiers, hashes,
timestamps, revisions, enums, and stable references. They do not store prompts,
reasoning text, source contents, command output, provider payloads,
environment-variable values, credentials, authorization headers, private
keys, or tokens.

Private identifiers and trusted-time records use redaction-safe `Debug`
implementations. Public malformed-wire errors and private operation errors use
stable generic messages and do not echo rejected values. Bearer capabilities
are private, non-serializable, and non-cloneable.

## 8. Test Quality Assessment

Public tests protect V1 construction, canonical ordering, serde compatibility,
V2 round trip, complete supported operation coverage, scope validation,
mixed-support rejection, and bounded deserialization errors.

Private reference tests cover:

- one-winner directive consumption and yield registration;
- consume-by-value authority and capability-free exact replay;
- exact wait identity and wake binding;
- committed regression, provenance, epoch, and expiry rejection;
- rejection-transition and commitment validation;
- exact successful-result target and ownership validation;
- receipt and trusted-time corruption;
- restart trust-root mismatch and restore quarantine;
- runnable, genuine-wait, blocked, executing, expired, and terminal
  continuation classification; and
- pre-commit, during-commit, and after-commit fault posture for every operation
  family.

Focused review exposed missing regressions for:

- replay of a historical security rejection after legitimate global
  trusted-time advancement;
- every persisted terminal state with expired, unavailable, or quarantined
  current time;
- authoritative map-key versus embedded-record identity corruption; and
- exact V1 serialization order and previously accepted unknown-field wire
  behavior.

After the blockers are fixed, the remaining test architecture work is to extract these
scenarios into one backend-parametric conformance harness and run them against
both reference and SQLite implementations before SQLite advertises V2 support.

## 9. Documentation Assessment

The roadmap and SQLite plan correctly distinguish the
implemented semantic V2 baseline from unimplemented durable and runtime
behavior. The implementation report overstates V1 wire compatibility, exact
target validation, historical replay, and terminal classification. This review
is the fix-forward source of truth until the implementation and report are
corrected.

The required sequence remains: fix and accept shared semantics, then
implement and review SQLite V2, then add event/state projection, and only then
prove one local injected-supervisor vertical slice.

No document claims that Workflow OS currently resumes an external executor,
keeps a live attempt lease, persists V2 continuity state, or automatically
approves a gate.

## 10. Blockers

The shared semantic V2 phase has four blockers:

1. Historical security-rejection replay must remain valid after legitimate
   current trusted-time and unrelated-window advancement while still failing
   closed on illegal predecessor/successor state.
2. Persisted terminal windows must classify as `Terminal` without requiring a
   fresh live clock or an unexpired execution window.
3. Successful-result validation must verify every authoritative record's map
   key, embedded identity, ownership, and committed result identity.
4. V1 wire compatibility must be preserved or an explicit breaking-contract
   decision and migration must be separately approved.

The following remain blockers for operational authorized-execution
continuity:

- no durable backend implements V2;
- no explicit SQLite V1-to-V2 atomic upgrade exists;
- no backend-parametric conformance harness exists;
- no runtime event/state projection opens or advances authoritative windows;
- no host supervisor consumes `ResumeNow`; and
- no live attempt lease distinguishes active execution from orphaned work.

## 11. Non-Blocking Follow-Ups

- Keep the V2 support declaration additive until a deliberate public contract
  migration is justified.
- Preserve fixed malformed-wire messages if V2 becomes schema-exposed later.
- Keep arbitrary restore support out of scope until an external epoch anchor
  can prove rollback resistance.
- Do not treat state-operation exactly-once behavior as exactly-once external
  execution.

## 12. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786835945299494000-2`;
- approval: `approval/run-1786835945299494000-2/review-scope-approved`;
- presentation: `presentation/3a88ab6c1213f4c5`;
- presentation hash:
  `3a88ab6c1213f4c54ba38bf37e496e456091d8bc9ebd4e2d3c06df77111ea54e`;
- approval outcome: granted by the delegated maintainer after the complete
  handoff;
- phase status: completed;
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six successful skill invocations, no retries,
  and no escalations;
- approval-presentation enforcement: proof enforced, with the presentation
  marker present in the approval event trail; and
- out-of-kernel work: source inspection, independent review, documentation,
  validation, git operations, and PR operations were performed by the external
  executor. The kernel did not edit files, run checks, schedule an agent, or
  implement backend support.

## 13. Validation

- focused semantic unit tests: passed;
- public V1/V2 contract tests: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check`: passed;
- `npm run check:integrations`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- independent focused security and compatibility reviews: both returned
  `BLOCK` with the four blockers recorded above.

## 14. Recommended Next Phase

Implement a focused **Authorized Execution Continuity semantic V2 blocker
fix** and then repeat this maintainer/security review.

The fix must address only historical rejection replay, terminal disposition,
exact authoritative identity validation, V1 compatibility, and focused
regressions. SQLite schema and transactions, runtime events, executor
integration, host scheduling, automatic approval, provider mutation,
CLI/schema exposure, hosted operation, and nested harness execution remain
deferred.
