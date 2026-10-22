# Executor Capability-Gated GitHub Read-Only Composition Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The crate-private SQLite-backed executor operation satisfies the mandatory
composition and ordering constraints from the accepted plan review. No path
was found that permits an unauthorized or repeated provider call. Public
adoption and a second provider operation remain unauthorized.

## 2. Scope Verification

The phase stayed within the approved private, opt-in, read-only scope. It added
one exact GitHub repository-metadata operation adjacent to
`LocalExecutor<SqliteStateBackend>`, minimal crate-private preparation and
telemetry reuse, focused tests, and phase documentation.

It did not add a public/default executor route, a second provider operation,
provider mutation, CLI or schema exposure, runtime configuration, ambient
credential discovery, report artifacts, OpenShell integration, hosted
identity, nested harness execution, reasoning lineage, or release changes.

## 3. Immutable Operation Selection Assessment

Core rehydrates the managed run and exact immutable bundle before selecting
the current scheduled step. The operation accepts only one non-`continue`
step whose immutable skill declares exactly:

- the symbolic GitHub read-only adapter;
- `external.read` and `adapter.invoke` capabilities;
- required, nonsensitive string inputs named `owner` and `repo`;
- one adapter requirement with `external.read`; and
- immutable literal mappings matching the requested owner and repository.

The private operation commitment binds the bundle root, operation code, step,
skill identity and version, and repository target. Caller input cannot switch
the selected provider operation or target without failing before provider
entry.

## 4. Run, Contract, And Cursor Assessment

Admission validates the required-context execution binding, exact contract
identity/version/content hash, running run identity, immutable bundle binding,
expected sequence/event cursor, and current correlation identity.

The continuation commitment additionally binds the resolved execution context,
immutable bundle, required-context binding, policy effects, capability set,
approval posture, approval sensitivity, and private operation binding.

After the one-time continuation claim is consumed, the shared continuation
helper validates the durable cursor again. The operation also performs an
explicit local cursor check before appending invocation events. Any cursor
change through that boundary produces zero provider calls and burns the claim,
which is the accepted fail-closed posture.

## 5. Policy And Approval Assessment

The immutable policy set is resolved inside Core. A required approval must
already be granted for the exact current step. The conservative policy engine
is evaluated with the actor, workflow, run, step, skill, autonomy, sensitivity,
adapter, capability, and correlation context, and the decision is durably
recorded before authority resolution.

The adapter precheck is constructed inside Core and is limited to fixture or
live read-only operation modes. No caller-supplied allowed precheck or approval
assertion crosses this boundary.

## 6. Current Authority And Source Continuity Assessment

The registered current-authority source derives the canonical GitHub resource,
reads the complete current fact inventory, validates freshness, completeness,
grant scope, availability, prerequisites, sensitivity, and step projection,
and prepares the exact request before provider entry.

Durable source continuity is reconciled before the continuation claim, as the
plan review explicitly requires. That record is source history, not operation
permission. A losing or stale consumer may leave a valid monotonic source
observation, but it cannot call the provider without winning the separate
cursor-bound continuation claim.

## 7. Provider Entry And Replay Assessment

The executor appends `SkillInvocationRequested` and
`SkillInvocationStarted` before entering the injected adapter. Exact concurrent
consumers share one durable claim; focused coverage proves one winner and one
provider call. A terminal replay fails before a second provider call.

The provider call is intentionally outside the SQLite transaction. Therefore
the implementation provides at-most-once dispatch with explicit ambiguity,
not fictional atomicity. A consumed claim without a start event and a started
invocation without a terminal event remain burned states requiring separately
reviewed recovery. The operation does not retry either posture automatically.

## 8. Event And Outcome Truthfulness

Provider success emits adapter telemetry before appending invocation success
and run completion. Provider failure appends invocation failure and run failure.
A successful provider read followed by receipt failure records telemetry and a
run failure without falsely claiming provider failure or invocation success.

The existing one-step event vocabulary is truthful for this private slice.
The implementation does not claim that unrelated work completed, and it
rejects a workflow whose step could continue into additional work.

## 9. Telemetry, Receipt, And Evidence Assessment

The operation reuses existing adapter audit and observability projections with
the exact immutable step and skill context. It returns the trusted,
payload-free `CapabilityUseAuthorityReceipt` separately from telemetry.

It correctly does not fabricate an `EvidenceReference`: the current evidence
taxonomy has no truthful capability-use receipt kind. Receipt-specific report
or evidence vocabulary requires separate planning.

## 10. Privacy And Error Assessment

Composition errors use stable
`executor.github_repository_metadata.*` codes and bounded messages. Debug
output redacts provider results and exposes only run posture and stable error
classification. Workflow events, continuity state, and receipt commitments do
not copy provider bodies, credentials, headers, source inventories, policy
payloads, repository paths, command output, or raw configuration.

Focused tests place forbidden markers in provider success and failure payloads,
then verify that Debug and SQLite files do not contain them. Existing
authority-source tests cover secret-like target rejection and non-leaking
failure paths.

## 11. Test Quality Assessment

Focused coverage proves:

- exact immutable operation success with one provider call;
- ordered request, start, success, and completion events;
- one audit and one observability projection;
- payload-free receipt validation;
- terminal replay rejection;
- target substitution rejection before provider entry;
- provider failure truthfulness and non-leakage;
- concurrent exact-consumer at-most-once behavior; and
- post-claim cursor mismatch rejection before provider entry.

The full workspace test suite, Clippy, formatting, and documentation checks
remain green. Existing lower-layer suites cover stale, incomplete, revoked,
unavailable, prerequisite-bearing, rollback, generation, watermark, query,
registration, and continuation replay postures.

## 12. Blockers And Non-Blocking Follow-Ups

Blockers: none.

Non-blocking follow-ups:

- add a forced receipt-construction failure seam if receipt production gains a
  configurable or fallible implementation boundary;
- add process-level contention coverage before any cross-process adoption;
- add direct composition restart tests for claim-only and invocation-started
  ambiguous states before implementing recovery;
- preserve the accepted source-continuity-before-claim ordering and disclose
  that losing consumers may still record monotonic source history;
- keep the operation crate-private until a separately reviewed selection and
  configuration boundary exists; and
- do not add receipt evidence vocabulary by reusing an inaccurate existing
  evidence kind.

## 13. Recommended Next Phase

Do not broaden to a second provider operation yet. Complete the dependency-
tracked dedicated draft-pull-request live sandbox proof when its
least-privilege credential is available. Otherwise reconcile the active queue
before selecting another runtime slice.

Any future public adoption of this read must separately define authored
operation selection, configuration, credential posture, recovery semantics,
and user-facing disclosure. OpenShell remains an optional future execution
provider, not an authority source.

## 14. Governed Review Record

- workflow: `dg/review`
- run ID: `run-1792704647702221000-2`
- approval ID:
  `approval/run-1792704647702221000-2/review-scope-approved`
- approval presentation ID: `presentation/44f55db4ec9f696e`
- approval presentation content hash:
  `44f55db4ec9f696e2c73ebdeac022ae0e887a14b900e4bf6f7a9aa5399e6a230`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- approval-presentation enforcement: persisted proof enforced
- review status: accepted and governed phase completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, no retries, and no escalations
- validation summary: focused composition tests, formatting, documentation,
  and diff checks passed during review; the unchanged implementation commit's
  immediately preceding workspace Clippy and full workspace test runs passed
- out-of-kernel work: the delegated maintainer inspected the implementation,
  tests, plans, prior reports, and validation evidence and authored this review;
  shell validation, documentation edits, Git operations, pull-request actions,
  and CI/merge inspection remained outside the kernel; no required check was
  skipped and no missing kernel coverage was simulated
