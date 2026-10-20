# Trusted-Host Workflow Completion Reconciliation Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The plan correctly keeps workflow completion in Core and keeps the foreground
application non-authoritative. Its proposed post-owner reconciliation point is
not restart-safe, however. The accepted supervisor commits the terminal
continuity attempt before returning the successful `SkillOutput`. A crash
after that commit and before the proposed workflow-event transaction leaves a
terminal successful attempt whose output reference existed only in lost
memory. The plan's suggested later reconciliation cannot reconstruct that
reference without fabricating evidence or rerunning already-completed work.

The plan also does not define the exact invocation lifecycle identity needed
to append a coherent `SkillInvocationSucceeded` or
`SkillInvocationFailed` event. The trusted-host path currently commits
dispatch and continuity projections but does not append the ordinary
`SkillInvocationRequested` and `SkillInvocationStarted` events. Appending only
a terminal invocation event after the fact would create an orphan lifecycle
record unless the plan deliberately defines and binds the complete mapping.

Implementation must not begin until both blockers are corrected.

## 2. Scope Verification

The planning phase stayed within its documentation-only scope. It added no
Rust behavior, workflow mutation, scheduler, automatic approval, profile,
provider, OpenShell, nested harness, public API, storage schema, write,
example, or release change.

The selected product boundary is correct:

- Core owns workflow lifecycle truth;
- continuity state owns attempt/window truth;
- the foreground application executes one opaque prepared operation; and
- bounded application output does not authorize a workflow transition.

## 3. Ownership Assessment

Keeping terminal reconciliation inside the existing Core-owned session path is
directionally correct. At that point Core still has the exact locator,
immutable bundle, executor binding, continuity result, and successful output.
The application must not receive a terminal-event builder or completion
capability.

The defect is not owner selection. It is transaction placement. The current
supervisor's `persist_outcome` commits continuity state before the owner returns
the output. A later transaction cannot make that earlier transition and the
workflow terminal sequence atomic.

## 4. Restart-Safety Blocker

The plan currently requires:

1. the accepted supervisor to record `Succeeded` or `TerminalFailure` in
   continuity state;
2. the owner to return an uncollapsed terminal result and optional
   `SkillOutput`; and
3. a later Core helper to append the workflow terminal event sequence.

For success, a process crash between steps 1 and 3 loses the only
`SkillOutput`. Durable continuity records the attempt as successful but does
not persist the bounded output reference or an equivalent terminal result
projection. Restart cannot:

- append an accurate `SkillInvocationSucceeded` event;
- validate required outputs;
- produce the same terminal event commitment;
- rerun the handler safely; or
- classify the run as completed without inventing evidence.

The plan's statement that explicit reconciliation retry can close this gap is
therefore false for successful outcomes.

### Required correction

The blocker fix must choose and fully specify one restart-safe design:

- **Preferred:** for the closed docs-check profile, replace the separate
  attempt-outcome write with one specialized SQLite transaction that atomically
  commits the authoritative attempt/window outcome and the complete workflow
  invocation/run event sequence while the exact in-memory result is present;
  or
- durably commit a bounded, integrity-bound terminal result projection in the
  same transaction as the attempt outcome, then prove a later workflow-event
  reconciliation can consume that projection exactly once without rerunning
  work.

The first option is smaller and avoids a new durable intermediate payload. If
selected, it must reuse the shared continuity semantics rather than duplicate
attempt/window transition logic.

The blocker fix may conclude that an internal SQLite schema change is required.
It must not promise a schema-free implementation before proving that existing
tables can represent exact replay and crash reconciliation.

## 5. Invocation-Lifecycle Blocker

The proposed success and failure sequences begin with a terminal invocation
event. The trusted-host path does not currently append the ordinary invocation
request and start events around executor entry. It also does not expose an
exact durable invocation ID in the proposed reconciliation input.

The blocker fix must define:

- the Core-derived invocation identity and where it is bound before dispatch;
- whether `SkillInvocationRequested` and `SkillInvocationStarted` are appended,
  and at which authoritative transactions;
- how those events relate to the existing dispatch-admission projection;
- the complete deterministic idempotency-key sequence;
- how replay proves one coherent invocation lifecycle; and
- whether report/audit consumers treat continuity dispatch as equivalent to,
  or distinct from, ordinary invocation lifecycle events.

Generating an invocation ID only during terminal reconciliation or appending a
terminal invocation event without a bound lifecycle is not acceptable.

## 6. Terminal Mapping Assessment

The proposed mappings are otherwise appropriately conservative:

- successful authoritative attempt plus valid output may complete the one-step
  workflow;
- terminal failure may fail it with fixed non-leaking metadata; and
- retryable, yielded, ambiguous, stale, mismatched, or incomplete posture must
  not terminalize it.

The blocker fix should retain those rules. It should also state explicitly
whether output validation happens before the atomic transition and how a
validation failure becomes a typed authoritative attempt result without
creating disagreement between continuity and workflow state.

## 7. Atomicity And Replay Assessment

The desired all-or-nothing workflow terminal sequence is correct but
insufficient by itself. Atomicity must begin at the terminal continuity
outcome, not after it.

The corrected design must prove:

- one transaction owns the attempt/window terminal transition and every
  workflow event whose facts depend on the in-memory result; or
- a durable integrity-bound intermediate result makes the later transition
  fully reconstructable;
- exact replay returns existing committed truth;
- conflicting replay fails closed;
- ambiguous commit uses a fresh connection and accepts only the complete exact
  transaction result; and
- no crash state requires handler reinvocation or output fabrication.

## 8. Privacy And Redaction Assessment

The plan's privacy posture is strong. It stores only bounded output references
and fixed failure metadata, keeps bindings private and Debug-redacted, and
prohibits raw command output, paths, environment values, payloads, credentials,
and secrets.

Any durable intermediate-result option must preserve those limits and bind a
safe output reference, not raw output. Serialization and corruption errors
must remain value-free.

## 9. Test Plan Assessment

The proposed tests cover success, failure, replay, contention, stale and
substituted identity, unsupported outcomes, privacy, and existing admission
behavior. The blocker fix must add explicit tests for:

- crash after executor return but before any terminal transaction;
- crash or injected failure at each transaction phase;
- restart after a committed success with no in-memory `SkillOutput`;
- complete invocation lifecycle identity and ordering;
- no orphan terminal invocation event;
- no terminal continuity state with an unreconstructable workflow result; and
- output-validation failure consistency across continuity and workflow state.

## 10. Blockers

1. **Terminal continuity and workflow completion are not atomic or durably
   bridgeable.** Successful output is lost across the proposed crash window.
2. **Invocation lifecycle identity and ordering are unspecified.** The plan
   cannot yet append a coherent invocation terminal event.

Both blockers are specific, security-relevant, and actionable.

## 11. Non-Blocking Follow-Ups

- Decide how committed workflow events drive audit and observability delivery
  without making external sinks part of transaction success.
- Preserve the disclosed final executable-validation-to-process-entry race for
  a later descriptor- or sandbox-attested launch design.
- Keep retry and ambiguity semantics outside this first closed profile unless
  separately planned.

## 12. Recommended Next Phase

Perform a documentation-only planning blocker fix that:

1. moves atomic ownership to the point where the terminal attempt outcome and
   in-memory result are both available;
2. specifies the exact invocation lifecycle identity and event ordering;
3. defines restart and ambiguous-commit behavior without output fabrication;
4. revises tests and storage assumptions accordingly; and
5. receives focused re-review before Rust implementation.

Do not implement runtime completion, add a schema, or broaden trusted-host
scope during the blocker-fix phase.

## 13. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1791505330084210000-2`
- Approval: `approval/run-1791505330084210000-2/review-scope-approved`
- Presentation proof: `presentation/fc3eff04758de0e0`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations
- Validation: `npm run check:docs` and `git diff --check` passed
- Approval-presentation enforcement: proof enforced with matching persisted
  presentation and event marker

Repository inspection, review reasoning, documentation edits, and validation
were performed by the delegated maintainer outside the kernel. The kernel
governed scope, approval, sequencing, and durable review evidence.

No runtime handler, report artifact, provider call, repository write, or other
kernel execution was simulated. Shell inspection, review authoring,
validation, and later git/PR operations are out-of-kernel maintainer work.
