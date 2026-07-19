# Runtime Proportional-Governance Assessment Binding Executor Integration Report

## 1. Executive Summary

Workflow OS now has one explicit opt-in local executor path that establishes a
validated proportional-governance assessment binding before workflow run
creation. The path derives the assessment from the exact stored immutable run
bundle plus explicit typed runtime facts, persists the payload-free binding
create-only, then projects it into the run event stream before validation and
execution start.

The integration records accepted posture; it does not yet enforce approval or
denial dispositions. Existing executor methods and default behavior are
unchanged.

## 2. Scope Completed

- Added explicit governance-assessment request and result types adjacent to the
  immutable-bundle executor API.
- Added `execute_with_governance_assessment_binding` as an additive local-only
  helper.
- Added create-only local persistence for one exact binding per run.
- Required the exact immutable bundle to exist before binding persistence.
- Validated an optional expected aggregate fingerprint before run creation.
- Emitted `GovernanceAssessmentBound` after `RunCreated` and before
  `RunValidated` and `RunStarted`.
- Returned the run, immutable bundle binding, and governance assessment binding
  as one in-memory result.

## 3. Integrity And Ordering Boundary

The helper first prepares and validates execution, persists or reopens the
immutable run bundle, assesses that stored bundle, validates any caller-supplied
expected fingerprint, and persists the resulting exact binding. Only then may
the executor append `RunCreated`.

Binding persistence is create-only. An exact pre-event duplicate can be
validated for interrupted setup, but a conflicting record fails closed. Once
run events exist, the path rejects reuse because retry/resume reassessment is
not implemented.

## 4. Runtime Semantics

The assessment remains review-and-record posture. The helper does not convert a
derived approval requirement into an approval pause and does not convert a
denied disposition into executor denial. That enforcement requires a separate
reviewed phase.

The existing `execute` and `execute_with_immutable_run_bundle` paths remain
unchanged. No global default opts into proportional-governance assessment.

## 5. Privacy And Error Handling

Request, result, store, and error boundaries retain the existing redaction-safe
model posture. Stable failures do not echo workflow IDs, run IDs, bundle IDs,
fingerprints, paths, definitions, or runtime facts. Stored bindings contain
typed references and hashes, not raw specs, source, evidence, check output,
provider payloads, command output, parser payloads, environment values, or
credentials.

## 6. Test Coverage

Focused tests prove:

- valid explicit execution persists and returns the exact binding;
- event ordering is `RunCreated`, `GovernanceAssessmentBound`,
  `RunValidated`, then `RunStarted`;
- the run snapshot retains the projected binding;
- expected fingerprint mismatch fails before any run event or skill call;
- no binding is persisted on fingerprint mismatch;
- an existing run is rejected until retry/resume reassessment exists;
- binding storage reopens across store instances and rejects duplicate writes;
- legacy executor behavior and immutable-bundle execution remain unchanged.

## 7. Scope Explicitly Not Added

No runtime disposition enforcement, retry/resume reassessment, trusted fact
freshness proof, default executor change, schema, CLI, UI, provider call,
provider write, automatic approval, enterprise administration, hosted runtime,
or persistence redesign was added.

## 8. Validation Commands

- Focused executor and immutable-governance tests: passed.
- Full formatting, lint, workspace test, and documentation validation are run
  before phase close and recorded in the governed phase report.

## 9. Remaining Limitations

- Runtime facts are explicit but do not yet carry trusted freshness references.
- Retry and approval-resume paths cannot reassess or prove an unchanged binding.
- Assessment execution disposition is not enforced.
- The integration is API-only and not exposed through workflow schema or CLI.
- The governed phase-close helper could not reread the accumulated approval
  presentation store after it reached 250 records. The proof-enforced approval
  command itself succeeded; close-summary store scaling needs separate kernel
  follow-up.

## 10. Recommended Next Phase

Perform a focused maintainer review of this executor integration. After
acceptance, implement retry/resume reassessment hardening before considering
default behavior or provider-mutation adoption.

## 11. Governed Phase Record

- Dogfood workflow: `dg/runtime-composition`
- Run ID: `run-1784501820708386000-2`
- Approval ID: `approval/run-1784501820708386000-2/composition-approved`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust implementation and tests, documentation updates,
  validation commands, diff inspection, and phase-report drafting
