# GitHub PR Comment Live Sandbox Event-Proof Composition Blocker Fix Review

## 1. Executive Verdict

Blockers fixed; proceed to the next roadmap-selected runtime composition phase.

The bounded fix closes both durable proof-identity blockers identified by the
helper review. Equivalent callers now derive the same event idempotency key,
and event correlation must agree with the accepted SideEffect record and
transition before append. The helper remains explicit, local, opt-in, and
separate from provider transport and report artifact writes.

## 2. Scope Verification

The phase stayed within the approved blocker-fix scope. It changed event-proof
identity derivation, correlation validation, focused tests, and documentation.

It did not add provider calls, default or automatic writes, hidden auth
loading, CLI mutation behavior, report artifact writes, retry or repair,
schemas, examples, hosted behavior, broader adapters, reasoning lineage, or a
release-posture change.

## 3. Original Blockers

The original helper accepted a caller-selected event idempotency key. Distinct
callers could therefore race with different valid keys for the same accepted
SideEffect outcome.

It also accepted event correlation without binding it to the correlation
already carried by the accepted SideEffect record and transition event. That
could create durable proof whose envelope disagreed with the transition it
claimed to prove.

## 4. Deterministic Event Identity Assessment

The request no longer accepts an event idempotency key. The helper derives one
canonical SHA-256-backed key from the SideEffect ID, original provider-write
idempotency key, provider kind, stable target-kind label, classified provider
outcome, and completed or failed lifecycle state.

The derivation uses explicit stable vocabulary rather than `Debug` output.
Equivalent callers therefore reach the existing backend exact-key
idempotency boundary with the same key. Matching replay rehydrates current
state and returns `AlreadyPresent` without appending another proof event.

This resolves the caller-selected-key and equivalent-caller race blockers.

## 5. Correlation Binding Assessment

The helper validates the supplied expected correlation ID against both the
store-backed SideEffect record and the bounded transition event. A mismatch
returns stable
`github_pr_comment_live_sandbox_event_proof.identity_mismatch` posture before
append.

The workflow event envelope therefore cannot silently claim correlation that
disagrees with the accepted transition context.

## 6. Replay And Conflict Assessment

Canonical key derivation preserves matching replay semantics. Authoritative
run rehydration remains the source of truth for existing proof inspection.
Matching outcome proof returns `AlreadyPresent`; incompatible existing proof
returns bounded conflict posture; no provider call is repeated.

The existing event backend remains the atomic exact-key append boundary. The
fix does not introduce a second persistence mechanism or a caller-controlled
proof identity.

## 7. Privacy And Redaction Assessment

The fix does not persist or copy provider auth, authorization headers, raw
provider payloads, comment bodies, repository contents, CI logs, command
output, spec contents, environment values, browser/session state,
approval-presentation text, or secret-like values.

The canonical key is hashed. Event keys, correlation values, SideEffect
identifiers, targets, and actors remain redacted from request/result Debug
output. Validation failures use stable codes and bounded messages without raw
identity values.

## 8. Provider And Artifact Boundary Assessment

The helper continues to consume an already-composed live-sandbox result. It
does not call or recall the provider. It appends only completed or failed
SideEffect workflow event proof when explicitly enabled and eligible.

It does not write a WorkReport artifact or relax the separate artifact
event-proof gates. Fixture and dry-run outcomes remain ineligible.

## 9. Failure Semantics Assessment

Identity, eligibility, and append failures remain bounded and fail closed.
They do not fabricate proof, retry provider operations, repair state, or
change the accepted provider outcome.

One non-blocking ambiguity remains. If append succeeds and subsequent run
rehydration fails, the helper reports `Failed` with the pre-append run even
though proof may be durable. This conservative posture prevents downstream
artifact claims, but a future recovery phase should distinguish an unknown
append outcome from proof known to be absent.

## 10. Test Quality Assessment

Focused regression coverage proves:

- completed and failed provider outcomes append the expected proof events;
- disabled append policy appends nothing;
- canonical key derivation is stable across matching replay;
- replay leaves one completed proof event;
- correlation mismatch fails before append;
- non-terminal run posture blocks append;
- provider transport is not called again;
- no report artifact is written;
- Debug and error output do not leak bounded identities or payload markers.

The focused suite passes six tests. Existing workspace coverage continues to
exercise exact-key event idempotency and terminal SideEffect event rules.

Concurrent scheduling is not directly orchestrated by the focused test, but
the corrected callers now converge on the backend's existing atomic exact-key
boundary rather than presenting different keys.

## 11. Documentation Review

The blocker-fix report accurately describes canonical key derivation,
correlation binding, replay behavior, privacy posture, and remaining
limitations. The plan and roadmap continue to describe the helper as explicit
and opt-in rather than automatic provider or artifact behavior.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Model append-success/rehydration-failure as explicit ambiguous proof posture
  with bounded recovery guidance.
- Add composition-level fixture and dry-run ineligibility coverage.
- Keep report artifact gates in a separately reviewed composition path.
- Keep lookup, recovery, retry, and repair explicit until separately approved.

## 14. Recommended Next Phase

Select the next runtime composition phase from the current roadmap after this
review is merged. The selection should continue connecting accepted runtime
primitives rather than introducing a new semantic family.

The next phase must not make provider writes automatic, add hidden auth,
silently retry or repair, bypass event-proof or artifact gates, add hosted
behavior, or broaden release posture.

## 15. Validation And Governed Review Evidence

Required validation:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Governed review:

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783794223755363000-2`.
- Approval ID:
  `approval/run-1783794223755363000-2/review-scope-approved`.
- Approval outcome: granted through proof-enforced approval presentation.
- Approval presentation ID: `presentation/816919197b0385db`.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, eight policy decisions, six scheduled steps, six successful
  skill invocations, and one completed run; no retries or escalations.
- Approval-presentation enforcement: `proof_enforced`; the approval event trail
  exposes the matching presentation proof marker.
- Validation summary: formatting, clippy with warnings denied, the full
  workspace test suite, docs validation, and diff hygiene passed. Opt-in live
  provider tests remained skipped by their existing environment gates.
- Out-of-kernel work: Codex inspected implementation and tests, authored this
  review, ran validation, and will perform git and PR operations. The kernel
  governed scope and approval but did not execute shell commands, edit files,
  call providers, write artifacts, or perform git/PR actions.
