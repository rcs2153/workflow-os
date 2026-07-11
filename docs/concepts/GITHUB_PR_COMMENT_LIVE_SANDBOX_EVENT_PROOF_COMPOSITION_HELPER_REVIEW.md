# GitHub PR Comment Live Sandbox Event-Proof Composition Helper Review

Fix-forward status: the blockers identified by this review are implemented in
[GitHub PR Comment Live Sandbox Event-Proof Composition Blocker Fix Report](GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_BLOCKER_FIX_REPORT.md).

## 1. Executive Verdict

Needs blocker fixes.

The helper stays within the approved explicit event-proof composition scope and
correctly avoids provider recall, automatic writes, artifact writes, hidden
auth, CLI behavior, schemas, examples, retry/repair, and hosted behavior.

The review found two related proof-identity blockers. The event idempotency key
is caller-selected rather than deterministically derived from the accepted
SideEffect outcome, and the caller-supplied event correlation ID is not bound
to the transition correlation context. Those gaps must be fixed before the
helper is accepted as durable workflow event proof.

## 2. Scope Verification

The implementation remained within the approved helper-only scope.

Implemented:

- an explicit event append policy;
- a bounded event-proof status model;
- an explicit request/result pair;
- authoritative run rehydration before proof inspection;
- completed/failed SideEffect event mapping;
- exact existing-outcome conflict detection;
- focused tests and documentation.

No accidental implementation was found for:

- default or automatic provider writes;
- provider recall from the event-proof helper;
- ordinary `LocalExecutor::execute(...)` behavior changes;
- hidden auth or runtime-config loading;
- CLI mutation commands;
- report artifact writes;
- lookup, recovery, retry, or repair mutation;
- workflow schema or SDK changes;
- examples;
- hosted or distributed behavior;
- broader adapter writes;
- reasoning lineage;
- release posture changes.

## 3. Helper API Assessment

The API is narrow and explicit. It accepts an already-composed
`GitHubPrCommentLiveSandboxRuntimeCompositionResult`, a caller-supplied run,
append policy, idempotency key, correlation ID, and actor. It returns the
original live-sandbox result, authoritative run posture after attempted append,
bounded status, and an optional stable error.

Reusing the accepted live-sandbox result is appropriate because that result
contains the classified provider response and store-backed lifecycle transition
without requiring another provider call or another auth source.

The request should not, however, remain authoritative for proof identity that
can be derived from that accepted result. Durable proof identity must be
deterministic across callers and restarts.

## 4. Authoritative Run Assessment

The helper correctly rehydrates the current run through the supplied executor
backend before terminal-state, identity, duplicate, or conflict checks.

This closes the stale-caller-run gap. Sequential retries see the latest event
stream, and a matching existing key and SideEffect outcome returns
`AlreadyPresent`. A different key for an already-present outcome returns
`Conflict` without appending another event.

The helper also correctly limits post-terminal append to completed/failed
SideEffect outcome events, which the runtime event model explicitly permits.

## 5. Event Mapping And Eligibility Assessment

Provider success maps to `SideEffectCompleted`; classified provider failure
maps to `SideEffectFailed`. Fixture and dry-run outcomes remain ineligible.

The helper validates:

- lifecycle agreement between response, transition record, and event payload;
- SideEffect identity agreement between transition record and event payload;
- workflow ID, workflow version, schema version, spec hash, and run ID against
  the authoritative run;
- presence of step, skill, and skill-version identity;
- explicit append policy;
- terminal run posture.

Those checks are appropriate and fail closed with bounded errors.

## 6. Deterministic Idempotency Blocker

Blocker: the request accepts any valid caller-supplied `IdempotencyKey` for the
first event append.

The pre-append scan prevents sequential duplicate outcome events, but it does
not make the event key deterministic. Two callers can rehydrate the same
pre-append run concurrently, supply different valid keys, and both reach the
append boundary. The backend deduplicates exact keys, not equivalent SideEffect
outcomes, so caller-selected keys leave a race that can create duplicate
durable proof.

Required fix:

- derive one canonical event idempotency key from stable accepted outcome
  identity, including at least SideEffect ID, original write idempotency key,
  provider kind, outcome/lifecycle posture, and any other existing canonical
  fields used by the provider-write event path; or
- validate a supplied key against that canonical derivation before append.

Tests must prove that independent callers derive the same key, matching replay
is idempotent, a mismatched supplied key fails before append if the field is
retained, and duplicate proof cannot be created through different caller keys.

## 7. Correlation Identity Blocker

Blocker: the helper uses the caller-supplied correlation ID for the workflow
event envelope but does not compare it with the correlation context already
carried by the accepted transition record/event.

This can produce a durable event whose envelope correlation disagrees with the
SideEffect transition it claims to prove. That weakens audit traceability and
violates the plan's required run/workflow/step/spec/correlation identity
binding.

Required fix:

- derive correlation context from the accepted transition when available; or
- reject caller-supplied correlation that does not match the transition;
- add matching and mismatching correlation tests with stable non-leaking
  errors.

## 8. Privacy And Redaction Assessment

The privacy boundary is acceptable.

The helper does not copy provider auth, authorization headers, raw provider
payloads, comment bodies, repository contents, CI logs, command output, spec
contents, environment values, browser/session state, approval-presentation
text, or secret-like values.

Request/result Debug output redacts run identity, event key, correlation ID,
actor, target, and SideEffect identifiers. Errors use stable codes and bounded
messages. Focused tests cover token-like, comment-body, target, and SideEffect
identifier non-leakage.

## 9. Failure Semantics Assessment

Eligibility and identity failures return bounded posture without provider
recall, retry, repair, artifact writes, or fabricated proof.

One non-blocking ambiguity remains: append and rehydrate are sequential. If
append succeeds but rehydration fails, the result reports `Failed` and returns
the pre-append run even though the event may already be durable. The posture is
conservative for artifact gating, but a future result model should distinguish
`append outcome unknown` from `event definitely absent` and guide operator
rehydration/recovery.

## 10. Test Quality Assessment

Focused tests prove:

- provider success appends one completed event;
- classified provider failure appends one failed event;
- provider transport is not called again;
- disabled policy appends nothing;
- a stale caller run is rehydrated;
- matching replay returns `AlreadyPresent`;
- a second key for an existing outcome returns `Conflict`;
- non-terminal run posture blocks append;
- Debug and error output are non-leaking;
- no report artifact is written.

Missing blocker coverage:

- deterministic key derivation across independent callers;
- mismatched canonical key rejection if caller input remains;
- correlation match and mismatch behavior;
- concurrent duplicate prevention at the backend append boundary.

Non-blocking additional coverage:

- append-success/rehydration-failure ambiguity;
- canceled-run outcome append posture;
- ineligible fixture/dry-run result at the composition layer.

## 11. Documentation Review

The roadmap, plan, and phase report correctly describe the helper as explicit,
opt-in, local, and separate from provider transport and artifacts. They do not
claim default writes, CLI mutation behavior, hidden auth loading, schemas,
examples, hosted behavior, broad adapters, reasoning lineage, or release
posture changes.

The phase report should be amended by the blocker-fix phase to state that the
event key and correlation context are canonically bound after the fix.

## 12. Blockers

1. Derive or validate one canonical event idempotency key from the accepted
   SideEffect/provider outcome before append so equivalent concurrent callers
   cannot use different keys.
2. Bind workflow event correlation context to the accepted SideEffect
   transition and reject mismatches before append.

## 13. Non-Blocking Follow-Ups

- Model append-success/rehydration-failure as explicit ambiguous proof posture
  with operator recovery guidance.
- Add composition-level fixture/dry-run ineligibility coverage.
- Keep report artifact gates in a separate reviewed composition path.
- Keep provider lookup/recovery explicit and non-mutating until separately
  approved.

## 14. Recommended Next Phase

Recommended next phase: live sandbox event-proof composition blocker fix.

The fix should be limited to canonical event-key binding, correlation binding,
focused regression tests, and honest documentation updates. It must not call
the provider, enable default writes, add CLI behavior, write artifacts, add
schemas or examples, implement retry/repair, broaden adapters, add hosted
behavior, implement reasoning lineage, or change release posture.

## 15. Validation

Required review validation:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Results:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 16. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783789645091631000-2`.
- Approval ID:
  `approval/run-1783789645091631000-2/review-scope-approved`.
- Approval outcome: granted through proof-enforced approval presentation.
- Approval presentation ID: `presentation/1556002d13277a36`.
- Event summary: 39 ordered events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one completed run; no retries or escalations.
- Validation summary: all required review validation commands passed.
- Out-of-kernel work: Codex performed source inspection, review analysis,
  documentation edits, validation, and git/PR operations. The kernel governed
  phase scope and approval but did not inspect code, execute shell commands,
  edit files, or perform git/PR actions.
- Report posture: this review is documentation; no runtime `WorkReport`
  artifact is generated or persisted.
