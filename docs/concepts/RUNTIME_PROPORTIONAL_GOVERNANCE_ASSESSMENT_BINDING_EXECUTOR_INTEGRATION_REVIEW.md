# Runtime Proportional-Governance Assessment Binding Executor Integration Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups. Proceed to retry/resume
proportional-governance reassessment hardening.

## 2. Scope Verification

The phase stayed within the approved additive local executor boundary. It added
one explicit request/result path, create-only binding persistence, event
projection, focused tests, and honest documentation.

It did not change existing executor defaults or add runtime disposition
enforcement, retry/resume reassessment, trusted fact freshness, schemas, CLI,
UI, provider calls, provider writes, automatic approvals, enterprise controls,
hosted behavior, or unrelated model families.

## 3. API Assessment

`LocalExecutionWithGovernanceAssessmentRequest` composes the existing immutable
bundle request with explicit profile, exact per-step runtime facts, and an
optional expected aggregate fingerprint. It does not read hidden global state.

`execute_with_governance_assessment_binding` is narrow and opt-in. Its result
returns the run, immutable bundle binding, and exact assessment binding through
read-only accessors and owned decomposition. Existing execution APIs remain
unchanged.

## 4. Pre-Run Integrity Assessment

The helper establishes the source-of-truth order correctly:

1. prepare and validate the execution plan;
2. build, validate, and persist or reopen the immutable run bundle;
3. assess the exact stored bundle using explicit runtime facts;
4. validate any expected aggregate fingerprint;
5. construct and persist the exact binding create-only;
6. append `RunCreated`;
7. append `GovernanceAssessmentBound`;
8. append validation and start events;
9. execute workflow steps.

This means the binding exists durably before the run event stream begins. The
event projects an already-established binding and does not substitute for that
persistence boundary.

## 5. Store Assessment

The binding store uses one encoded run-addressed record and create-only writes.
It requires the exact immutable bundle to exist and compares the complete typed
bundle binding, not only workflow/run identity. Reads revalidate storage
identity and bundle equality.

The executor tolerates an exact existing pre-event binding only by reading and
comparing it. A conflict fails closed. Once run events exist, the path rejects
reuse until retry/resume reassessment has been designed.

## 6. Runtime And Compatibility Assessment

The phase is intentionally record-only. Approval-required and denied assessment
dispositions do not yet alter execution. The API documentation, roadmap, plan,
and report state that limitation directly.

Legacy `execute` and `execute_with_immutable_run_bundle` behavior remains
unchanged. Existing run identities remain backward compatible because the
governance binding stays in its existing optional snapshot field.

## 7. Privacy And Error Assessment

Debug implementations expose counts and typed posture while relying on
redaction-safe binding Debug implementations. Stable errors do not echo IDs,
hashes, paths, definitions, runtime facts, or payloads. Persistence stores only
the validated payload-free binding.

Fingerprint mismatch fails before binding persistence, run events, or skill
invocation. A pre-existing run fails before reassessment or mutation.

## 8. Test Quality Assessment

Focused tests prove successful execution, binding persistence and reopening,
snapshot projection, event ordering, fingerprint mismatch failure before
`RunCreated`, no skill call on failure, no binding on mismatch, duplicate
create-only behavior, and rejection of existing runs.

The full workspace suite preserves executor, immutable bundle, approval,
capability, SideEffect, evidence, report, adapter, provider-write, and CLI
behavior.

Two useful cases remain non-blocking:

- directly exercise exact pre-event replay after an interrupted setup;
- corrupt a stored binding record and assert stable non-leaking read failure.

Both belong naturally with retry/resume persistence hardening.

## 9. Documentation Assessment

The roadmap and plan correctly distinguish implemented binding establishment
from unimplemented disposition enforcement, fact freshness, retry/resume
reassessment, schema, CLI, UI, and default behavior. The phase report accurately
describes commands executed outside the kernel and the local-only posture.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add exact interrupted pre-event replay coverage.
- Add corrupt binding-record read and non-leakage coverage.
- Define current-fact freshness and expected-fingerprint requirements for retry
  and approval resume.
- Keep runtime disposition enforcement separate from reassessment integrity.

## 12. Recommended Next Phase

Implement retry/resume proportional-governance reassessment hardening as one
explicit opt-in local path. It must prove the immutable bundle and current facts
still produce the accepted binding before new execution or resume events.

Do not broaden default execution, schemas, CLI, UI, provider mutations, or
automatic approvals in that phase.

## 13. Validation

- Focused executor and immutable-governance tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live tests remained intentionally
  ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 14. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784504118630768000-2`
- Approval ID: `approval/run-1784504118630768000-2/review-scope-approved`
- Presentation ID: `presentation/4d0b4cc104663946`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: code and test inspection, phase-review drafting,
  validation review, and scope assessment
