# Trusted-Host Local Application Admission Implementation Report

## 1. Executive Summary

The trusted-host local application admission vertical slice is implemented.
An unpublished `workflow-local-host` foreground binary can open one SQLite
backend, submit explicit selection and lifecycle input, receive one Core-issued
opaque prepared operation, and synchronously execute the closed no-write
docs-check profile. The implementation closes the previously missing
application boundary without exposing authority-bearing construction inputs or
creating a supported CLI contract.

The slice remains deliberately narrow. It does not discover work, schedule in
the background, grant approval, accept caller-authored authority, execute
arbitrary commands, mutate providers, bridge state models, or claim production
local-host support.

## 2. Scope Completed

- Added a feature-gated Core admission request, bounded outcome vocabulary,
  and admission function for one exact trusted-host local operation.
- Added an unpublished foreground binary to the existing
  `workflow-local-host` package.
- Added owned private preparation inputs so Core can retain all authority and
  provenance while handing local host only the established opaque one-shot
  prepared pair.
- Added explicit fresh-run and exact existing-run admission paths.
- Bound workflow, run, step, actor, profile, immutable bundle, current
  authority, executor commitment, and validated skill input before execution.
- Added a dogfood-only one-step `dg/trusted-host-docs-check` workflow.
- Added a production-shaped integration test that invokes the actual binary.
- Added atomic SQLite immutable-run-bundle persistence and exact reads.

## 3. Scope Explicitly Not Completed

This phase does not add:

- the path to the released `workflow-os` CLI or SDKs;
- a stable local-host operator interface;
- automatic discovery, polling, scheduling, daemons, or process recovery;
- arbitrary local commands, ambient tools, live adapters, or provider calls;
- provider mutation or any broader write family;
- caller-authored authority, context facts, locators, executors, or skill
  input;
- automatic approval, approval bypass, or delegated-authority broadening;
- additional local-check profiles, parallelism, branching, nested harnesses,
  or OpenShell;
- hosted/PostgreSQL parity, workflow specification schema changes, examples,
  or release posture changes; or
- automatic workflow completion after the admitted operational attempt.

## 4. Admission API And Ownership Boundary

`TrustedHostLocalApplicationAdmissionRequest` accepts explicit project,
workflow, run, bundle, step, actor, correlation, and trusted-time inputs. The
caller selects the closed docs-check profile, but it cannot provide a handler,
command, authority fact, execution locator, executor commitment, or prepared
session.

`admit_trusted_host_local_application_operation` loads and validates the
project, accepts only the exact supported one-step topology, creates or
rehydrates the immutable run, evaluates current policy and approval posture,
binds the requested identity to durable state, resolves the handler and
validated `SkillInput`, establishes explicit-zero current authority, and
returns a bounded admission outcome. Only `Prepared` contains the opaque
one-shot prepared session. Waiting, terminal replay, blocked, and denied remain
lawful bounded observations rather than fabricated failures.

The foreground binary immediately converts `Prepared` into
`LocalHostPreparedOperation`. It owns process lifetime, the SQLite connection,
the resolved profile, and cancellation custody for the synchronous call. It
prints only closed posture strings and never prints handler output or Core
diagnostics.

## 5. SQLite Immutable-Run-Bundle Boundary

SQLite adapter schema v7 adds an `immutable_run_bundles` table and implements
`ImmutableRunBundleStore` directly on `SqliteStateBackend`. Bundle creation is
atomic and create-only. Reuse must match the exact bundle identity and
manifest; conflicting replacement fails closed. Exact reads reconstruct and
validate the canonical model.

The version-six-to-version-seven transition is explicit. Ordinary open does
not silently upgrade an older database. This preserves the one-backend
invariant for the admitted run and avoids a filesystem or PostgreSQL state
bridge.

This is an adapter-schema change only. It is not a workflow specification
schema change and does not expose a new public storage contract.

## 6. Explicit-Zero Current Authority

The admitted profile uses the reviewed explicit-zero required-context
contract. Core privately proves that the exact operation requires no
capability grants, availability facts, or governed-context references, then
registers complete empty inventories bound to the immutable operation.

Absence is not treated as proof. A non-empty, missing, stale, incomplete, or
conflicting requirement blocks admission. The application cannot supply or
amend the inventories.

## 7. Terminal And Replay Behavior

The first foreground path supports bounded prepared, waiting, terminal replay,
blocked, denied, and failure outcomes. Prepared operation outcomes are also
mapped to closed cancellation, entry-stop, and continuation-stop posture.

An operational terminal result is not workflow completion. The end-to-end
proof intentionally observes the workflow run as `Running` after the
docs-check operational window reaches terminal posture. A later reviewed
composition must own workflow completion. Replaying the exact request returns
the same bounded operational posture without duplicating durable events or
invoking the handler again.

## 8. Privacy And Security Posture

- Debug for the admission request and prepared outcome is redacted.
- Application errors are fixed codes and omit paths, identifiers, policy text,
  commands, arguments, environment values, payloads, outputs, credentials, and
  Core diagnostics.
- The opaque prepared pair is one-shot, non-cloneable, and non-serializable.
- Authority-bearing inputs remain private to Core.
- Unknown arguments, unsupported profiles, invalid timestamps, incompatible
  SQLite state, unsupported workflow topology, and binding mismatches fail
  closed.
- The integration proof uses a fixed local executable that exits successfully;
  it performs no network access or provider operation.

## 9. Test Coverage

Focused coverage proves:

- the explicit-zero contract preserves legacy non-empty hashes and wire shape;
- zero projections are accepted only for an explicit-zero contract;
- complete empty current-authority inventories resolve only for the exact
  binding;
- SQLite v7 initializes, persists, exactly reads, rejects replacement, and
  requires explicit upgrade from earlier schemas;
- the actual foreground binary admits and executes the dogfood docs-check
  operation;
- the durable run, continuity projection, event history, and immutable bundle
  are present after execution;
- exact replay duplicates neither events nor work; and
- established local-host cancellation, drift, one-shot ownership, bounded
  failure, and privacy tests remain active.

The first end-to-end test does not separately exercise approval wait, policy
denial, or every substitution at the binary boundary. Those postures are
covered by existing lower-level executor, preparation, policy, and ownership
tests and remain explicit review targets before any broader adoption.

## 10. Workflow Semantics

The helper does not grant approval, reinterpret policy, or change existing run
status semantics. Approval remains a wait, denial remains denial, unsupported
state remains blocked, and application return does not complete the workflow.
No post-terminal workflow event, report artifact, provider call, or released
CLI output is introduced.

## 11. Commands And Results

The implementation was validated with:

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed on the complete rerun.
- `npm run check:docs`: passed.
- `npm run check:integrations`: passed with the already-validated temporary
  Cargo target.
- `workflow-os --project-dir dogfood/workflow-os-self-governance validate`:
  passed with the established experimental-lifecycle warnings.
- `git diff --check`: passed.

Focused checks completed during implementation include the Core feature build,
the 17-test SQLite backend suite, and the foreground admission integration
test.

The first workspace test invocation encountered one intermittent failure in
the pre-existing
`explicit_time_window_reinvocation_enters_existing_window_once` test. The same
test passed immediately in isolation and passed in the complete workspace
rerun. No failure occurred in the new admission test. The first integrations
invocation also stopped making progress while using the repository root Cargo
target; it was terminated and the unchanged integration script passed against
the already-validated temporary Cargo target. These observations are disclosed
rather than treated as product correctness proof or silently omitted.

## 12. Remaining Known Limitations

- The binary interface is intentionally unpublished and unstable.
- Only one explicit docs-check workflow/profile is accepted.
- Current authority is limited to exact explicit-zero obligations.
- There is no discovery, queue, background scheduler, process-loss recovery,
  or persistent supervisor.
- The operational terminal result does not complete the workflow.
- Broader binary-level negative-path coverage is still desirable.
- No provider, write-capable, hosted, nested-harness, or OpenShell behavior is
  enabled.

## 13. Governed Phase Record

- Workflow: `dg/implement`
- Run: `run-1791406308048230000-2`
- Approval: `approval/run-1791406308048230000-2/implementation-approved`
- Presentation proof: `presentation/24b718aa3caad8f4`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations

No `.workflow-os` runtime state is part of the implementation change set.

## 14. Recommended Next Phase

Perform a focused implementation/security review. It should assess Core-owned
provenance, explicit-zero authority proof, SQLite v7 bundle atomicity and
upgrade behavior, prepared-pair opacity, replay/idempotency, bounded
application output, operational-terminal versus workflow-terminal semantics,
and whether the test depth is sufficient for this unpublished first slice.

Do not broaden the profile family, adopt the released CLI, add discovery or
scheduling, mutate providers, or claim production local-host support before
that review is accepted.
