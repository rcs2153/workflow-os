# GitHub PR Comment Live Sandbox Approval Authority Linkage Blocker Fix Report

## 1. Executive Summary

The live-sandbox approval-authority blocker is fixed in the explicit local
composition path.

The new helper does not trust caller-selected `LinkedAndApproved` readiness.
Before the injected provider is reachable, it validates a matching terminal
workflow run, resolves the attempted SideEffect's stable approval reference,
validates the matching approval decision through the existing
approval-presentation proof policy, and validates the persisted SideEffect
approval linkage. Only then does it derive the allowed sandbox posture and
delegate to the accepted live-sandbox helper.

The focused review found one follow-up blocker: caller-supplied run memory had
not been bound to durable executor state. The helper now rehydrates the run
through the executor backend, requires exact equality with the supplied run,
and uses the durable run for all authority checks.

The re-review then identified that the first rehydration call also projected a
snapshot. The final fix uses the backend's read-only `rehydrate_run(...)`
boundary and verifies workflow snapshot and event content are unchanged across
successful composition.

## 2. Blocker Fixed

The earlier live proof supplied a structurally valid `ApprovedByHuman`
SideEffect and caller-selected `LinkedAndApproved` readiness. That proved the
provider, lifecycle, and event path after claimed authority, but not the full
authority-to-effect chain.

The provider can no longer be reached through the new proof-bound composition
unless the claimed approval is present in the matching run, was decided through
the proof-enforced presentation path, and is linked to the persisted attempted
SideEffect.

## 3. Implementation Approach

Added:

- `GitHubPrCommentLiveSandboxApprovalAuthorityCompositionRequest`;
- `GitHubPrCommentLiveSandboxApprovalAuthorityCompositionResult`;
- `compose_github_pr_comment_live_sandbox_runtime_with_approval_authority(...)`;
- a record-oriented reuse boundary for the existing provider-write approval
  presentation gate.

The helper accepts explicit executor, store, provider, terminal run,
presentation policy, target proof, readiness inputs, and provider-call inputs.
It introduces no hidden state or runtime configuration.

## 4. Validation Boundary

The helper fails closed unless:

- the executor can rehydrate the durable run;
- the supplied run exactly equals the durable rehydrated run;
- the run is terminal;
- workflow, version, schema, spec hash, and run identity match the attempted
  SideEffect;
- the SideEffect cites a stable approval-decision reference;
- the approval request and decision exist in the run;
- the approval presentation proof satisfies the supplied proof policy;
- the persisted SideEffect record exists and its approval linkage validates.

After those checks, the helper sets approval-required posture and derives
`LinkedAndApproved`. The caller's original approval-readiness enum is not used
as authority.

## 5. Provider And Runtime Boundary

This remains an additive, explicit, in-memory and local composition helper. It
does not make provider writes automatic, load auth, add CLI mutation, append
events itself, write artifacts, retry or repair, broaden adapters, add schemas,
or change release posture.

The lower-level accepted live-sandbox helper remains available for compatibility.
The ignored real-provider proof test now uses the proof-bound helper, so any
future explicit live run must exercise the complete authority chain. This phase
did not repeat the external provider call.

## 6. Privacy And Error Handling

The request and result Debug implementations expose bounded status and counts,
not approval IDs, presentation IDs, SideEffect IDs, provider auth, comment
content, targets, paths, or raw payloads. Existing proof and linkage failures
retain stable, non-leaking error codes. Provider orchestration errors preserve
their bounded structured code and message.

## 7. Test Coverage

Focused tests prove:

- a real proof-enforced approval and persisted linkage allow one injected
  provider call;
- a caller-supplied missing approval posture is replaced by derived authority;
- missing approval-presentation proof blocks before provider invocation;
- stale proof blocks before provider invocation;
- wrong approval identity blocks before provider invocation;
- missing persisted SideEffect linkage blocks before provider invocation;
- a non-proof presentation policy blocks before provider invocation;
- a caller run that differs from durable state blocks before provider
  invocation;
- missing durable run state blocks before provider invocation;
- successful authority validation leaves workflow snapshot and event state
  unchanged;
- request/result Debug output does not expose secret or stable authority IDs;
- the ignored live proof harness compiles against the proof-bound path.

Existing provider-write, live-sandbox, approval, SideEffect, executor, report,
adapter, validation, and runtime tests remain in the workspace suite.

## 8. Commands And Results

The phase runs:

```sh
cargo test -p workflow-core --test local_executor live_sandbox_approval_authority
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

The ignored real-provider test is compile-checked by the workspace suite but is
not executed, avoiding another external comment.

## 9. Governed Phase Evidence

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783797882614468000-2`.
- Approval ID: `approval/run-1783797882614468000-2/fix-approved`.
- Approval presentation ID: `presentation/d6d617b00a602eb5`.
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer.
- Phase status: completed.
- Event summary: 39 ordered events, including one approval request, one
  approval grant, eight policy decisions, six scheduled steps, six successful
  skill invocations, and one completed run; no retries or escalations.
- Validation summary: six focused approval-authority tests, formatting, clippy
  with warnings denied, the complete workspace suite, docs validation, and diff
  hygiene passed. The ignored live-provider test was not executed.
- Out-of-kernel work: Codex edited code/tests/docs, ran commands, and will
  perform git and PR operations. The kernel governed scope and approval but did
  not call GitHub, execute commands, edit files, or perform git actions.
- Runner note: the first phase-close invocation used a descriptive phase label
  unsupported by the repo-local runner and exited without state mutation; the
  corrected `blocker` close completed and produced the summary above.

Durable-run follow-up fix:

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783801523525540000-2`.
- Approval ID: `approval/run-1783801523525540000-2/fix-approved`.
- Approval presentation ID: `presentation/62088cb3955bf3aa`.
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer.
- Event summary: 39 ordered events with one approval request, one grant, eight
  policy decisions, six scheduled steps, six successful skill invocations, and
  one completed run; no retries or escalations.
- Validation summary: eight focused authority tests, formatting, clippy with
  warnings denied, the complete workspace suite, docs validation, and diff
  hygiene passed. The ignored live-provider test was not executed.

Read-only durable-run correction:

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783802979838593000-2`.
- Approval ID: `approval/run-1783802979838593000-2/fix-approved`.
- Approval presentation ID: `presentation/9fdd9b8ec42b842b`.
- Approval outcome: granted through the proof-enforced path by the delegated
  maintainer.
- Event summary: 39 ordered events with one approval request, one grant, eight
  policy decisions, six scheduled steps, six successful skill invocations, and
  one completed run; no retries or escalations.
- Validation summary: eight focused authority tests, formatting, clippy with
  warnings denied, the complete workspace suite, docs validation, and diff
  hygiene passed. The ignored live-provider test was not executed.

## 10. Remaining Limitations

- The proof-bound helper is explicit and not a default executor path.
- Auth remains caller supplied.
- No additional provider mutation was exercised.
- Event proof and report artifact composition remain separately explicit.
- Ambiguous provider outcome recovery remains evidence-driven future work.
- Production targets, hidden auth, automatic retries, hosted execution, and
  broader provider support remain unsupported.

## 11. Recommended Next Phase

Recommended next phase: focused re-review of the durable-run authority binding
fix.

After acceptance, implement the already-planned proportional-governance core
decision model before broadening approval defaults, workflow automation, or
provider mutations.
