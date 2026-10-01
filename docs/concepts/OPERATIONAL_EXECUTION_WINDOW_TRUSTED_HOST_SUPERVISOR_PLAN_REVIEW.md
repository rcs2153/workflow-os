# Operational Execution Window And Trusted-Host Supervisor Plan Review

## 1. Executive Verdict

**Plan accepted with incorporated blocker corrections; proceed to the
operational execution-window opening implementation only.**

The original draft chose the correct overall sequence but left three security
boundaries ambiguous: who may construct opening authority, whether opening
would mutate the accepted closed five-operation continuity contract, and what
exact operation binding survives restart. The reviewed plan now resolves all
three before implementation.

## 2. Scope Verification

The phase remained planning-only. It did not add an opening store API, SQLite
schema, projection event, supervisor, scheduler, executor redispatch,
automatic approval, provider mutation, OpenShell execution, nested harness
runtime, CLI, public schema, hosted runtime, Reasoning Lineage, or release
change.

## 3. Existing Boundary Assessment

The plan accurately identifies the accepted implementation boundary:

- yielded-window continuity is durable;
- wait transitions, directive consumption, attempt outcome, and ambiguous
  recovery are atomic;
- SQLite projects each accepted mutation into the event stream and snapshot;
- fresh-connection reconciliation distinguishes durable commit from absence;
  and
- no production path can lawfully create the initial operational window.

The test-only yielded-window bootstrap is correctly treated as conformance
fixture setup, not runtime opening evidence.

## 4. Original Planning Blockers

### 4.1 Caller-authored authority ambiguity

The first draft allowed “current gate-readiness and policy commitments” in the
opening request without stating who produced them. It also left open whether a
prior gate assessment was required. `AuthorizedExecutionGateAssessment` is
explicitly non-authoritative, so accepting its serialized value or arbitrary
commitments as permission would let the host manufacture opening posture.

The corrected plan requires Core to rehydrate durable state, recompute policy,
approval-presentation, evidence, check, proportional-governance, and current-
fact posture, and construct a private opening authorization only inside the
registered current-authority source's same-call use closure. No caller may
serialize or reconstruct this capability.

### 4.2 Closed-contract compatibility ambiguity

The accepted continuity contract intentionally contains five operations with
closed operation-kind, replay, codec, conformance, and SQLite assumptions.
Adding opening as a sixth enum variant would silently change the meaning of an
already accepted capability and require broad compatibility changes.

The corrected plan adds a separately versioned opening capability, record,
projection payload, and conformance surface. Existing V1, semantic V2, and the
five-operation suite remain unchanged.

### 4.3 Missing exact operation ownership

The existing authoritative window record carries governance and authority
commitments but does not by itself persist enough public action/resource
vocabulary to authorize arbitrary host callbacks after restart. The first
draft referred to a general local action and therefore risked widening one
window into ambient callback authority.

The corrected plan narrows the first operation to exactly
`invoke_current_step_skill` and requires a durable operation-binding
commitment plus exact window/attempt ownership links. Action substitution must
fail closed.

## 5. Opening Transaction Assessment

The corrected transaction boundary is appropriate:

- one current cursor and deterministic snapshot;
- one immutable run bundle;
- one current actor and exact operation scope;
- current policy, approval-presentation, evidence, check, proportional-
  governance, and authority facts;
- one window created directly in `executing` posture;
- one store-allocated first attempt in `started` posture;
- one operation record and receipt;
- one separately versioned bounded event and snapshot projection; and
- one private attempt-use capability returned only after known commit.

The first implementation deliberately omits a persisted
`assessment_required` intermediate state. That removes an unnecessary
schedulable gap and makes the opening transaction the only transition into
operational execution.

## 6. Commit Ambiguity And Recovery Assessment

The plan correctly returns no attempt capability after an ambiguous commit
acknowledgement. A fresh connection must reconcile the complete operation,
record, event, snapshot, and binding before the host takes another action.

A committed `started` attempt whose host disappears before reporting outcome
is conservatively ambiguous. It cannot be retried as known-not-started.
Recovery remains a separate Core decision, not a host inference.

## 7. Supervisor Boundary Assessment

The one-shot injected supervisor is correctly deferred until opening receives
its own focused implementation review. Its responsibilities are appropriately
limited to:

- obtaining a private attempt capability from Core;
- invoking one exact injected executor once;
- reporting a bounded result through Core; and
- returning the current reconciled disposition.

The supervisor cannot decide gate readiness, create authority, approve work,
fabricate completion, satisfy waits, retry a started attempt, or create a model
turn. Repeated scheduling remains out of scope.

## 8. Yield, Wait, And Completion Assessment

The plan preserves the key P0 invariant:

- an ordinary turn boundary is a durable yield, not a wait;
- a genuine wait names exact typed conditions;
- `ResumeNow` is a disposition, not bearer authority;
- every resume uses fresh source-backed authority and one current directive;
  and
- only an existing real terminal runtime event can terminate the workflow.

An assistant final response, host delivery acknowledgement, or successful
callback cannot create terminal workflow state.

## 9. Backend And Migration Assessment

SQLite is the correct first and only eligible backend for the proof. The
opening capability must be advertised independently of semantic V2 support.
Filesystem and PostgreSQL remain explicit zero-write unsupported paths.

The corrected plan requires a new private versioned opening record/table
rather than widening the closed operation-kind check. Because no production
API could previously open operational windows, migration must prove the
pre-opening state needed for safe upgrade or fail closed. It cannot synthesize
authority for legacy fixture-shaped records.

## 10. Privacy Assessment

The plan keeps prompts, transcripts, hidden reasoning, source, specs, command
output, parser/provider/sandbox payloads, environment values, credentials,
paths, approval reasons, evidence bodies, and reconstructable authority out of
requests, records, events, snapshots, errors, and Debug output.

Only bounded IDs, enums, revisions, timestamps, counts, sensitivity/redaction
posture, and domain-separated commitments are allowed.

## 11. Test Plan Assessment

The corrected test plan covers the high-risk cases:

- caller-authored readiness and authority rejection;
- exact operation-binding substitution;
- stale cursor, immutable bundle, actor, scope, policy, evidence, check,
  approval, expiry, and revocation rejection;
- concurrent openers with one winner;
- event-cursor contention;
- before/during/after commit faults and fresh-connection reconciliation;
- no capability after rejection or ambiguity;
- started-attempt crash recovery;
- bounded injected outcomes;
- turn-boundary yield and one lawful resume;
- no false completion;
- unsupported backend zero-write behavior; and
- privacy and regression coverage.

## 12. Remaining Blockers

None for beginning the first opening-only implementation phase.

The implementation must not include the injected supervisor until the opening
contract, SQLite transaction, projection, reconciliation, migration, and tests
pass focused maintainer/security review.

## 13. Non-Blocking Follow-Ups

- Select the narrowest existing registered current-authority producer for the
  first proof during implementation.
- Choose between a no-side-effect test skill and an existing deterministic
  read-only local check handler only after the opening boundary is accepted.
- Keep future model-executor orientation context payload-free and separately
  reviewed.

## 14. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- governed review close: passed with proof-enforced approval presentation and
  terminal `Completed` status.

## 15. Governed Review Record

- workflow: `dg/review`;
- run: `run-1790833880067773000-2`;
- approval: `approval/run-1790833880067773000-2/review-scope-approved`;
- presentation: `presentation/709b857ee1989728`;
- presentation hash:
  `709b857ee1989728e7155b5d51bdf078d0648b1efc108753dbe4c65f53bea722`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented; and
- governed status: completed with 39 events, 1 approval, 0 retries, and 0
  escalations; and
- out-of-kernel work: source inspection, security analysis, plan correction,
  review authoring, and validation were performed by the external executor.
  The kernel did not edit files, run checks, open a window, invoke a
  supervisor, mutate a provider, or create a model turn.

## 16. Recommended Next Phase

Implement the **operational execution-window opening capability only**:

1. separately versioned Core model and private authorization;
2. reference conformance;
3. SQLite schema/migration and atomic opening transaction;
4. separately versioned runtime event/snapshot projection;
5. fresh-connection reconciliation; and
6. focused maintainer/security review.

Do not implement the trusted-host supervisor in the same phase.
