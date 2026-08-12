# Proportional-Governance Selected Project-Validation Report Adapter Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to the selected approval adoption envelope.

The existing-terminal adapter now executes the canonical current check with a
fresh Core-owned evaluation time, compares the new V3 runtime-fact assessment
against the original durable binding on stable semantic commitments, and
returns the original durable binding without mutating run history. Current
semantic drift fails closed. No blocker remains from the original review.

## 2. Scope Verification

The fix stayed within the approved Core-only blocker scope. It added no CLI
cutover, approval-envelope behavior, schema field, provider mutation,
persistence, event mutation, reusable reassessment authority, hosted behavior,
or release change.

The shared legacy reassessment path retains byte-for-byte equality when it is
not using the current-runtime-fact source. Semantic comparison is limited to
the V3 source-backed path that requires truthful current temporal provenance.

## 3. Original Blocker Restatement

The prior adapter executed a new canonical check for an existing terminal run
but supplied the original runtime-fact snapshot evaluation time to the new
observation. Full binding equality was achieved by replaying time. That made
the new observation's temporal provenance false even though its facts were
current.

## 4. Fix Approach Assessment

The selected adapter now chooses `Timestamp::now_utc()` for every call. On an
existing terminal run, Core constructs a fresh source-backed assessment and
uses `GovernanceAssessmentBinding::validate_current_runtime_fact_binding` to
compare stable semantics rather than timestamps.

The approach is minimal and appropriately placed:

- Core still owns the timestamp and runtime-fact source;
- the canonical check still runs exactly once per adapter call;
- report construction still receives the exact current check reference;
- legacy non-source-backed reassessment behavior is unchanged; and
- no new persistence or authority record was introduced.

## 5. Semantic Comparison Assessment

Both bindings must use V3 current-runtime-fact vocabulary. Comparison requires
the same:

- workflow and run identity;
- immutable run-bundle binding;
- assessment algorithm and aggregate fingerprint;
- step count, execution disposition, disclosure, and completeness;
- source-registration commitment;
- runtime-fact commitment and fact count; and
- snapshot assessment aggregate fingerprint.

Observation and evaluation timestamps are intentionally excluded. Omitting
them is necessary for a truthful later reassessment and does not weaken the
fact, source, immutable-input, or assessment commitments.

## 6. Provenance And Durable-State Assessment

`reassessment_evaluated_at` exposes the fresh time only on the in-memory
existing-terminal route result. Debug output redacts it. The result continues
to expose the original durable governance binding, and the persisted run keeps
that same binding and event history.

The transient timestamp is not persisted, emitted as an event, or represented
as an authority receipt. It therefore cannot authorize later execution. This
is the correct boundary for a no-mutation report reconciliation path.

## 7. Failure And Compatibility Assessment

Current fact or check drift produces
`executor.governance_assessment_binding.reassessment_mismatch` before report
regeneration, event append, or skill replay. Errors remain stable and do not
expose fact values, paths, command output, or source identity.

The compatibility guard is important: callers without a current-runtime-fact
source still require exact durable binding equality. The fix therefore does
not silently broaden semantic matching across older V1/V2 or legacy routes.

## 8. Privacy And Redaction Assessment

The fix adds no raw payload, source content, command output, parser data,
provider data, environment value, credential, token, or path field. The fresh
timestamp is redacted in Debug output, and the serialized durable binding is
unchanged. No privacy or redaction blocker was found.

## 9. Test Quality Assessment

Focused tests now prove that:

- existing-terminal reassessment receives a time later than the original
  durable snapshot;
- the canonical check runs again while the workflow skill does not;
- the returned route exposes the original durable binding;
- the persisted run binding and event history remain unchanged; and
- changed current check posture fails closed without event or skill replay.

The complete workspace and CLI suites also protect legacy reassessment and
approval behavior. Direct selected-adapter tests for visible-disclosure and
denied terminal outcomes remain useful but non-blocking because the shared
route and compositor already cover those dispositions.

## 10. Documentation Assessment

The blocker-fix report, roadmap, original report fix-forward note, and CLI
adoption plan accurately distinguish current implementation from deferred
approval-envelope and CLI work. They do not claim durable reassessment
authority or runtime mutation.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add direct selected-adapter coverage for visible-disclosure and denied
  terminal outcomes when that adapter surface next changes.
- Keep transient reassessment provenance non-authorizing unless a separately
  designed durable audit requirement is approved.
- Preserve the legacy exact-equality branch while legacy callers remain.

## 13. Recommended Next Phase

Proceed to the selected approval adoption envelope. That phase should bind the
already selected project-validation route to the existing proof-enforced
approval path without cutting over the CLI or broadening provider, schema, or
persistence behavior.

## 14. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1786487950091526000-2`
- Approval: `approval/run-1786487950091526000-2/review-scope-approved`
- Presentation: `presentation/824549a06d9ef333`
- Approval outcome: granted by delegated maintainer with persisted
  presentation proof
- Presentation content hash:
  `824549a06d9ef333b0f7df988eb01aa5ec5adcddab26791719dd9814a819b265`
- Phase status: completed
- Event summary: 39 events, one approval, zero retries, and zero
  escalations
- Approval-presentation enforcement: proof enforced with one persisted
  presentation record and a present event marker

## 15. Validation

Validation results:

- focused selected-adapter tests: passed, 5 of 5;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- PR 462: passed all seven repository CI jobs before squash merge.

An independent `cargo test --workspace` review run compiled successfully and
reported no failures across every completed suite, including 216 Core unit
tests and the adapter, hook, approval-presentation, audit, capability,
current-authority, diagnostic, and durable-state integration suites. The run
was stopped after macOS held successive already-built test binaries in
`_dyld_start` for several minutes before their millisecond test executions.
The exact implementation tree had already completed the full workspace suite
before merge and then passed the authoritative merged PR CI matrix. This
environmental interruption is disclosed rather than represented as a second
completed local workspace run.

## 16. Out-Of-Kernel Disclosure

The kernel governed review scope, approval, and durable event history. Codex
inspected the merged implementation, tests, reports, and CI evidence; formed
the maintainer verdict; edited this review artifact and bounded roadmap status;
and ran repository validation outside the kernel. No Workflow OS runtime state
was edited by hand.
