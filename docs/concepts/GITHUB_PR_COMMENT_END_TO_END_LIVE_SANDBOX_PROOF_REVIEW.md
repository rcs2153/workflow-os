# GitHub PR Comment End-To-End Live Sandbox Proof Review

## 1. Executive Verdict

Live transport and event proof accepted; expansion readiness needs one blocker
fix.

The merged proof establishes that the concrete injected GitHub HTTP provider,
store-backed SideEffect completion, and durable workflow event-proof path work
together against a real non-production pull request. It does not yet establish
that the live write authority was derived from a real, validated
approval-presentation and approval-decision chain. Another mutation or adapter
must not be authorized from this proof until that linkage is composed and
tested.

## 2. Scope Verification

The phase stayed within the approved opt-in integration scope.

Implemented:

- one ignored-by-default integration test;
- one test-local injected HTTP transport;
- explicit environment-supplied target and auth;
- one provider call to a draft pull request;
- store-backed SideEffect completion;
- durable completed event proof;
- focused documentation and a phase report.

No default writes, production target, hidden auth loading, CLI mutation,
automatic retry/repair, report artifact write, schema change, broader adapter,
hosted behavior, reasoning lineage, higher autonomy, or release change was
introduced.

## 3. External Effect Assessment

The external effect is bounded and verified. Draft PR #318 received exactly one
comment with non-production language. The GitHub connector confirmed the
comment reference, and the ignored live test passed.

The test transport reduced the raw response to status and comment ID before it
entered Workflow OS models. The provider outcome was classified as success.

## 4. SideEffect Lifecycle Assessment

The test persisted a proposed SideEffect, transitioned it to `Attempted`,
invoked the provider after sandbox readiness allowed the call, and verified the
store-backed transition to `Completed`.

Target, capability, run, workflow, step, skill, integration, idempotency, and
correlation identity were preserved through the accepted model boundaries.

## 5. Durable Event-Proof Assessment

The accepted event-proof helper appended one `SideEffectCompleted` event after
the provider/local transition succeeded. The test rehydrated the run and
verified one completed proof event. Event-proof composition did not recall the
provider or write an artifact.

Canonical key derivation and correlation binding remain covered by focused
non-live regressions. The live proof did not expose an idempotency or correlation
failure.

## 6. Approval Authority Linkage Blocker

Blocker: the integration harness does not derive live write authority from an
actual approval-presentation proof and approval decision.

The harness creates a SideEffect authority with `ApprovedByHuman` and a stable
approval reference, then supplies sandbox readiness posture
`LinkedAndApproved`. Those values are validated shapes, but they are synthetic
test inputs. The governed `dg/runtime-composition` approval authorizes the
development phase; it is not consumed by the test as the approval record that
authorizes the GitHub write.

The proof therefore demonstrates provider and event composition after claimed
authority, not the complete authority-to-effect chain.

Required fix before expansion readiness:

- construct or load a real approval-presentation record and proof marker;
- record a matching approval decision through the existing proof-enforced path;
- validate approval/SideEffect linkage through existing store-backed helpers;
- derive sandbox approval readiness from that validated linkage rather than a
  caller-selected enum alone;
- prove mismatch, missing proof, stale proof, and wrong approval identity block
  before provider invocation;
- preserve the explicit test-only target/auth boundary and avoid another live
  provider call for negative cases.

## 7. Restart And Replay Assessment

The local event path rehydrates after append, and existing tests cover matching
event replay and canonical idempotency. The live test itself intentionally calls
GitHub once and does not replay the provider operation.

That is the correct safety posture because GitHub issue comments do not expose
the kernel idempotency key as a provider-enforced idempotency primitive. Restart
and replay validation should therefore occur after the provider call using
persisted local state and lookup/recovery posture, never by blindly repeating
the external write.

The append-success/rehydration-failure ambiguity remains non-blocking and was
not observed.

## 8. Report Disclosure Assessment

The phase report honestly records provider call, SideEffect transition, event
proof, external effect, skipped artifact behavior, validation, and
out-of-kernel work.

No runtime WorkReport artifact was generated. That is consistent with phase
scope, but the phase should not be described as proving automatic terminal
report or artifact composition.

## 9. Privacy And Redaction Assessment

The token was injected explicitly into the test process and was not printed or
committed. Production request Debug remains redacted. Raw provider response
payloads do not enter the durable models or event. The event stores bounded
identity/reference posture rather than comment content or auth material.

No leakage was found in the test, report, Git diff, or GitHub PR body.

## 10. Test Quality Assessment

Strong coverage:

- explicit opt-in and ignored default posture;
- real provider success;
- completed SideEffect transition;
- one durable completed event;
- no provider recall during event proof;
- no report artifact write;
- focused canonical-key, correlation, failure, disabled-policy, replay, and
  non-terminal regressions;
- full workspace non-regression.

Missing blocker coverage:

- real approval-presentation proof to approval decision to SideEffect linkage;
- missing/stale/mismatched proof blocking before provider invocation;
- derivation of `LinkedAndApproved` readiness from validated linkage.

## 11. Documentation Review

The phase report accurately distinguishes the live integration harness from
default runtime behavior. It does not claim CLI writes, hidden auth, retries,
artifacts, production readiness, broader adapters, or hosted operation.

The roadmap must distinguish “live provider/event proof passed” from “complete
governed authority chain accepted.”

## 12. Blockers

1. Compose and test proof-enforced approval presentation and approval decision
   linkage into SideEffect authority and sandbox readiness before provider
   invocation.

## 13. Non-Blocking Follow-Ups

- Preserve post-provider restart/recovery testing without repeating the write.
- Model append-success/rehydration-failure as explicit ambiguous proof posture.
- Keep WorkReport artifact composition separately gated.
- Keep the live integration test ignored and explicit.

## 14. Recommended Next Phase

Recommended next phase: live sandbox approval-authority linkage blocker fix
planning and implementation.

The phase should reuse existing approval-presentation, proof-marker,
high-assurance approval, approval-side-effect linkage, sandbox readiness, and
provider composition primitives. It must not add another provider call for
negative tests, broaden adapters, load hidden auth, enable default writes,
retry or repair automatically, add CLI mutation behavior, or change release
posture.

## 15. Validation And Governed Review Evidence

Required validation:

```sh
cargo test -p workflow-core --test local_executor live_sandbox
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Governed review:

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783797069329229000-2`.
- Approval ID:
  `approval/run-1783797069329229000-2/review-scope-approved`.
- Approval presentation ID: `presentation/1bd094c503d477f6`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Event summary: 39 ordered governance events, including one approval request,
  one approval grant, eight policy decisions, six scheduled steps, six
  successful skill invocations, and one completed run; no retries or
  escalations.
- Approval-presentation enforcement: `proof_enforced`; the approval event trail
  exposes the matching presentation proof marker.
- Validation summary: focused live-sandbox tests, formatting, clippy with
  warnings denied, the full workspace suite, docs validation, and diff hygiene
  passed. The real live test was not repeated during review; its one external
  effect and passing result were inspected from merged phase evidence and PR
  #318.
- Out-of-kernel work: Codex inspected merged code, tests, the phase report, and
  GitHub proof evidence; authored this review; ran validation; and will perform
  git/PR operations. The kernel governed scope and approval but did not call
  GitHub, execute commands, edit files, or perform git/PR actions.
