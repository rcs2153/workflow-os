# Trusted-Host Local Application Admission Negative-Path Hardening Review

## 1. Executive Verdict

**Needs blocker fixes.**

The application-boundary tests materially improve proof depth and correctly
exercise approval wait, unsupported topology, identity substitution,
incompatible state, and bounded output through the real unpublished binary.
However, the resolved-execution commitment does not yet prove the exact
executable identity it claims to bind.

## 2. Scope Verification

The implementation remains inside the accepted unpublished docs-check slice.
It adds no profile, public CLI, discovery, scheduling, provider behavior,
write, automatic approval, workflow completion, hosted parity, nested harness,
OpenShell, schema, example, or release change.

The review itself adds documentation only.

## 3. Application Proof Assessment

The new tests correctly establish that:

- a valid approval requirement produces a durable waiting run and no
  operational opening;
- unsupported multi-step topology fails before run creation;
- changed actor and correlation bindings fail without appending events;
- a different executable path is rejected on replay;
- incompatible SQLite input returns a bounded state error without path
  disclosure; and
- the existing success and exact replay behavior remains intact.

The decision not to fabricate policy-denial and pre-entry-cancellation cases at
the binary boundary is correct. The application exposes no injected policy
engine or cancellation input. Existing lower-level tests remain the honest
proofs for those postures.

## 4. Resolved Execution Commitment Assessment

The implementation adds a separate private commitment over the resolved
process request and combines it with the logical command-contract fingerprint.
Separating logical command identity from local resolution identity is the
correct architecture. Environment iteration is deterministic because the
request stores a `BTreeMap`, and only the final digest crosses the admission
boundary.

The current encoding is not sufficient for an exact security commitment:

1. The executable is committed by path only. Replacing executable contents at
   the same path does not change the commitment. A restarted existing-window
   admission could therefore accept a different executable under the durable
   opening identity.
2. Paths are converted with `to_string_lossy`. Distinct paths containing
   different non-UTF-8 byte sequences can collapse to the same replacement
   text and therefore the same commitment.
3. The application test changes the executable path, so it does not detect a
   same-path executable replacement.

The commitment must bind a deterministic executable content identity and use
an exact byte representation for local path material. Resolution should also
fail safely if executable identity cannot be read or changes while the
commitment is being established.

## 5. Replay And State Assessment

Actor and correlation substitutions are rejected before new events. The
unsupported-topology proof confirms no run is created. The SQLite failure
proof confirms the fixed public category remains independent of a local path.

The existing exact replay proof remains valid for an unchanged executable. It
does not establish exact replay after same-path executable replacement, which
is the blocker described above.

## 6. Privacy And Error Assessment

The new tests demonstrate that secret-like actor text, changed correlation
text, alternate executable names, and database paths are absent from output.
Debug and application output remain bounded.

The blocker fix must preserve this posture. Executable bytes, local paths,
environment values, and content digests must not be exposed in Debug, errors,
reports, or serialized public model data.

## 7. Test Quality Assessment

The focused six-test suite is useful and production-shaped. Full workspace,
clippy, documentation, and integration validation passed.

Required blocker regression coverage:

- replace executable contents at the same path and prove replay fails before
  execution or new events;
- prove exact unchanged executable replay remains idempotent;
- prove exact path-byte framing is deterministic without lossy conversion;
- prove unreadable or unstable executable identity fails closed without path,
  bytes, or digest leakage; and
- retain all current application negative-path tests.

## 8. Blockers

The trusted-host executor commitment must bind the exact executable content
identity rather than only its path, and it must not derive security identity
through lossy path conversion.

Broader trusted-host adoption remains blocked until this is fixed and reviewed.

## 9. Non-Blocking Follow-Ups

- Preserve the current honest lower-level ownership of policy-denial and
  cancellation proofs unless an application seam is deliberately introduced.
- Consider whether repository working-set identity needs a separately planned
  immutable input boundary before crash/restart re-entry can execute a local
  check against mutable source state.

## 10. Recommended Next Phase

Implement a focused blocker fix that introduces an exact, non-lossy resolved
execution commitment with executable content identity and same-path
replacement regression tests. Then perform a focused blocker-fix review.

Do not broaden profiles, public runtime surfaces, scheduling, providers,
writes, approvals, workflow completion, hosted behavior, nested harnesses,
OpenShell, schemas, examples, or release posture.

## 11. Validation

The reviewed implementation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `npm run check:integrations`; and
- `git diff --check`.

## 12. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1791482689703116000-2`
- Approval: `approval/run-1791482689703116000-2/review-scope-approved`
- Presentation proof: `presentation/b189ae9d818b8e16`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations

Repository inspection, documentation editing, validation, and git/PR work were
performed by the delegated maintainer outside the kernel. The kernel governed
scope, approval, sequencing, and durable review evidence.

## 13. Fix-Forward Note

The blocker was accepted into a separately governed `dg/blocker` phase. The
fix replaces lossy path text with exact encoded path bytes, commits a streaming
executable content hash, revalidates that identity before process-request
construction, and adds same-path replacement plus non-UTF-8 distinction
regressions. This note does not erase the original finding. Acceptance remains
subject to the focused blocker-fix review.
