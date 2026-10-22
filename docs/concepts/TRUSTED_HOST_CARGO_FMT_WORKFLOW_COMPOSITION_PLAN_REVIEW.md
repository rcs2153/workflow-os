# Trusted-Host Cargo Format Workflow Composition Plan Review

## 1. Executive Verdict

**Plan accepted after planning blocker corrections; proceed to the exact
three-step trusted-host no-write workflow implementation.**

The proposed topology is the smallest useful next runtime-composition proof:
it preserves the accepted project-validation and docs-check prefix, appends
the reviewed Cargo format profile, and leaves every existing one-step and
two-step identity unchanged. Review found two truth-binding gaps in the plan.
Both are corrected before implementation.

## 2. Scope Verification

The plan remains private, local, closed-profile, sequential, and no-write. It
does not authorize clippy, tests, builds, generic Cargo execution, arbitrary
commands, source-writing format mode, providers, credentials, external writes,
public CLI/schema/SDK surfaces, examples, hosted expansion, parallelism,
nested harnesses, automatic approval, or release changes.

The foreground loop is bounded to at most three operations for one explicit
run. It is not a scheduler, daemon, queue poller, or workflow-discovery path.

## 3. Topology And Compatibility Assessment

Appending Cargo format is the correct compatibility-preserving order:

```text
project validation -> docs check -> cargo format check
```

The first two steps remain the exact accepted two-step prefix. The plan adds a
closed three-profile registry, sibling request/admission boundary, and distinct
v4 report/artifact commitment. It does not rename or generalize the accepted
two-step types or migrate the v3 report identity.

A generic N-step abstraction is neither needed nor authorized. Limited private
helper extraction is acceptable only if observable one-step and two-step
events, errors, reports, artifacts, and replay remain unchanged.

## 4. Authority And Progression Assessment

Core remains authoritative for topology, current-step derivation, closed
profile selection, and successor scheduling. The host receives no caller-
selected step, profile, command, result, successor, event cursor, or report
identity.

Each successful non-terminal operation must atomically commit its bounded
result, matching invocation success, and exact successor schedule. Failure or
timeout commits the result, failed invocation, and failed run without a
successor. Re-admission derives the next lawful operation from durable history
rather than host memory.

The three-operation upper bound, fresh admission between steps, and stop-on-
unexpected-posture rules preserve the accepted continuation boundary.

## 5. Resolved Execution Identity Correction

The original plan implied that the immutable bundle alone binds the complete
Cargo toolchain and protected-source identity. The implementation does not use
that boundary. The bundle binds the exact workflow, handler posture, and
command declarations. The private resolved execution fingerprint is included
in the trusted-host executor commitment, which is durably recorded by the
authorized dispatch event.

For Cargo format, that resolved fingerprint includes the accepted formatter-
chain inventory and protected-source commitment. The corrected plan therefore
requires every attempted step to have exactly one matching dispatch, requires
the executor commitment to reproduce from the closed profile and immutable
step identity, and includes the dispatch event and commitment in v4 report
material. Command and bundle matches without the dispatch commitment are not
sufficient.

This closes the replay and report-integrity gap without serializing local paths,
toolchain inventory entries, source material, or environment values.

## 6. Pre-Result Failure Truth Correction

The accepted trusted-host executor maps a handler error to terminal failure
without constructing `LocalCheckResultRecorded`. That is the correct behavior
for formatter identity or protected-source failure before result acceptance,
but the original plan did not define how history and reporting remain honest
when no result exists.

The corrected plan accepts one narrow shape:

```text
step scheduled
  < matching authorized dispatch admitted
  < matching skill invocation failed
  < run failed
```

There is no result event and no successor. The v4 report binds the scheduled
step, dispatch commitment, failed invocation, and terminal event; states that
execution failed before a bounded result was accepted; and discloses later
steps as not attempted. It must not fabricate a result citation or infer a
specific error cause that durable history does not preserve.

## 7. History, Restart, And Replay Assessment

The legal schedule and result prefixes are exact and ordered. Every accepted
result binds its expected step, profile, command, immutable bundle, invocation,
and invocation-terminal event. Every attempted step additionally binds one
authorized dispatch commitment.

Restart after the first or second committed success runs only the remaining
steps. Terminal replay runs no process and appends no event. Duplicate,
reordered, skipped, contradictory, substituted, or successor-after-failure
history fails closed before further execution or report composition.

## 8. WorkReport And Artifact Assessment

A distinct v4 commitment is required because the topology and durable binding
set differ from the accepted two-step v3 contract. Completed reports require
three passing result citations plus three matching dispatch bindings. Failed
reports cite only accepted results while binding every attempted dispatch and
invocation terminal.

The create-only private SQLite artifact path remains appropriate. Exact
duplicates may reconcile as already persisted; conflicting or unreadable
artifacts fail closed. Report or artifact failure cannot rewrite terminal run
truth, rerun a check, or append lifecycle events.

## 9. Cargo Integrity And Privacy Assessment

The composition reuses the reviewed Cargo profile rather than redefining its
toolchain, root, environment, timeout, mutation-detection, or error contracts.
The plan correctly retains the limitations that final-state commitments do not
prevent transient restored writes and disabled-network contract posture is not
operating-system egress isolation.

Errors, Debug, events, reports, artifacts, and host output remain payload-free
and may not expose paths, stdout, stderr, command transcripts, source/spec
contents, environment values, Cargo configuration, credentials, or token-like
material.

## 10. Test Plan Assessment

The corrected matrix covers exact three-step success; failure at every step;
Cargo non-zero, timeout, and pre-result integrity behavior; dispatch-
commitment substitution; atomic progression; restart at both continuation
boundaries; terminal replay; causal-history corruption; v4 report truth;
create-only artifact recovery; privacy; and one-step/two-step compatibility.

The entry-point tests must exercise the unpublished foreground binary, not
only private helpers. Complete engineering-standard validation and the opt-in
real bundled-toolchain smoke remain required before implementation acceptance.

## 11. Planning Blockers Found And Corrected

### B1. Resolved Cargo identity was not explicitly bound into v4 history

The plan now requires exact authorized dispatch and executor-commitment
validation for every attempted step and includes those bindings in v4 report
material.

### B2. Pre-result integrity failure had no honest report contract

The plan now defines the exact no-result failed-invocation shape and forbids
fabricated result citations or unsupported claims about the normalized cause.

## 12. Remaining Limitations

- The workflow remains private and unpublished.
- There is no operating-system process, filesystem, or network sandbox.
- Persistent final-state comparison cannot detect a transient restored write.
- The toolchain identity is self-committed rather than independently attested.
- The implementation remains an exact three-step proof, not a generic workflow
  engine extension.

These are explicit limitations, not blockers for the bounded implementation.

## 13. Non-Blocking Follow-Ups

- Consider shared private validation helpers only after exact compatibility is
  demonstrated by regression tests.
- Keep process containment in the separately governed sandbox/runtime lane.
- Require another plan and review before adding clippy, tests, builds, a fourth
  profile, or a caller-authored topology.

## 14. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 15. Governed Review Record

- Workflow ID: `dg/review`
- Run ID: `run-1792681037171049000-2`
- Approval ID:
  `approval/run-1792681037171049000-2/review-scope-approved`
- Presentation ID: `presentation/e2a637e8ede8abb0`
- Presentation content hash:
  `e2a637e8ede8abb0d7edaa1371ccc019563d20273481a1a1eb13711dd6c8b14b`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted review-only scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, and zero escalations
- Approval presentation enforcement: proof enforced with one persisted
  presentation record and the approval-event proof marker present
- Out-of-kernel work: the delegated maintainer inspected the plan, accepted
  runtime identity, dispatch, supervisor, report, and test boundaries;
  corrected planning blockers; wrote this review; and ran validation outside
  the kernel. The kernel governed scope and approval but did not perform those
  actions.

## 16. Recommended Next Phase

Implement the exact three-step trusted-host no-write workflow as corrected.
Start with the closed three-profile registry, exact topology/history validator,
and dispatch-commitment binding. Then add bounded progression, the private v4
report/artifact path, foreground entry-point tests, restart/replay tests, and
complete regression validation.

Do not add clippy, tests, builds, arbitrary commands, providers, external
writes, public surfaces, hosted expansion, parallelism, nested harnesses, or
release changes.
