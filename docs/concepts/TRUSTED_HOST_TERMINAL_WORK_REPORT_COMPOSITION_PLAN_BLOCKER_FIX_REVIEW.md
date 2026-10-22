# Trusted-Host Terminal WorkReport Composition Plan Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to the bounded in-memory implementation.**

The corrected plan selects one authoritative replay source for every concrete
trusted-host local-check result: a Core-authored, payload-free
`LocalCheckResultRecorded` workflow event committed atomically with invocation
and run terminalization. The limitation-only replay path is removed.

## 2. Scope Verification

The fix remains planning-only. It does not authorize runtime code, raw result
persistence, WorkReport persistence, artifacts, released CLI behavior, another
profile, arbitrary commands, scheduling, providers, writes, hosted expansion,
schemas, examples, or release changes.

The planned event is durable execution evidence needed to reconcile an already
accepted local-check result. It is not a persisted WorkReport or report
artifact.

## 3. Event Ownership Assessment

Core alone derives and appends the result event from the actual same-call
`LocalCheckResult`. The host cannot supply a result, status, citation, output
reference, or report claim. Exact replay trusts only durable state and never an
output-reference string or caller assertion.

This ownership matches the existing trusted-host boundary.

## 4. Minimum Binding Assessment

The planned event binds the required facts:

- validated result identity and status;
- run, workflow, version, step, and invocation attempt;
- selected profile and canonical command;
- immutable bundle root;
- bounded output reference when one exists;
- invocation terminal event relationship; and
- Core-provided result timestamp.

These fields are sufficient to reject run, bundle, step, profile, command,
attempt, result, status, and terminal-event substitution.

## 5. Atomicity Assessment

The result event, matching invocation terminal event, and run terminal event
must use one state-backend compare-and-append operation. Concrete result truth
therefore cannot commit partially. This is the correct lifecycle boundary for
fresh composition and crash recovery.

An internal handler error without a concrete result emits no result event and
uses explicit unavailable posture. It cannot fabricate a check outcome.

## 6. Replay And Recovery Assessment

Exact replay requires one matching durable result event and reconstructs the
same validated reference without process execution or event append. Missing,
duplicate, mismatched, or conflicting result events fail closed. Crash after
terminal commit but before in-memory report construction can rehydrate the
same result binding and derive the same report identity.

The previous ambiguity between a durable record and a known limitation is
removed.

## 7. Privacy Assessment

The event explicitly excludes stdout, stderr, summaries, command transcripts,
paths, executable fingerprints, environment values, source/spec contents, and
payloads. The permitted contents are validated identifiers, status vocabulary,
immutable commitments, linkage, and timestamps.

The fix does not solve replay by storing raw `LocalCheckResult` data.

## 8. Test Assessment

The corrected plan adds direct tests for:

- atomic lifecycle commit;
- post-terminal pre-report crash recovery;
- passing, failed, and timed-out reconstruction;
- missing, duplicate, ID-conflicting, status-conflicting, and
  binding-conflicting replay; and
- durable-event non-leakage.

The original report identity, section, workflow semantics, substitution,
privacy, no-artifact, and full regression coverage remains required.

## 9. Remaining Non-Blocking Decisions

- Select the exact terminal event timestamp used as `generated_at`.
- Select the existing approval reference projection without approval reason
  text.
- Return fixed generated or generation-failed posture through the opaque host
  result.
- Select the repository's existing domain-separated digest helper for report
  identity.

These decisions fit inside implementation and do not weaken the reviewed
boundary.

## 10. Blockers

None.

## 11. Recommended Next Phase

Implement the trusted-host terminal WorkReport composition vertical slice,
in-memory only. Perform focused maintainer/security review before considering
report persistence, artifacts, released surfaces, another profile, scheduling,
provider access, or writes.

## 12. Governed Review Evidence

The re-review ran under:

- workflow ID: `dg/review`
- run ID: `run-1792664199251286000-2`
- approval ID:
  `approval/run-1792664199251286000-2/review-scope-approved`
- approval outcome: granted by `user/delegated-maintainer`
- presentation proof: `presentation/196d48a94ea8af28`

`npm run check:docs` and `git diff --check` passed. The terminal governed run
contains 39 events, one approval request and grant, zero retries, and zero
escalations. Approval presentation was proof-enforced and its event marker is
present.

Repository reads, review writing, shell validation, and git inspection occurred
outside the kernel under the approved re-review scope. Rust and TypeScript
checks were skipped because no runtime code changed. The kernel did not edit
files, commit, push, open a pull request, generate a WorkReport, or create an
artifact.
