# Proportional-Governance Selected Local Project Consumer CLI Adoption Plan

Status: Implemented. The selected fresh-run report adapter, its
existing-terminal reassessment provenance blocker fix, and the selected
approval adoption envelope are implemented and accepted by focused review.
The manifest-controlled CLI `run` and `approve` paths now consume those Core
compositions while preserving the existing public contract.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Proportional-Governance Selected Local Project Consumer Plan](proportional-governance-selected-local-project-consumer-plan.md)
- [Selected Consumer Implementation Report](../concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_CONSUMER_REPORT.md)
- [Selected Consumer Blocker-Fix Review](../concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_CONSUMER_BLOCKER_FIX_REVIEW.md)

## 1. Executive Summary

The selected local project-validation consumer is implemented and accepted in
Core. It owns the fixed current-runtime-fact source, selects fresh evaluation
time inside Core, reproduces current facts before approval mutation, preserves
separate aggregate-governance and workflow-step approvals, and closes granted
approval decisions through a trusted authority receipt and local WorkReport
artifact.

The existing manifest-controlled CLI path now uses the selected Core-owned
authoritative route and approval/report helpers. The adoption replaces its
internal composition without changing the public commands, activation
declaration, human output, JSON shape, run semantics, approval semantics, or
artifact obligation.

The adopted run path uses the selected fresh-run report-composition adapter for
quiet, visible, denied, existing-terminal, and approval-required outcomes. The
adopted approval path uses one selected Core envelope for both aggregate and
authored gates, projects the existing route labels from the Core-produced gate
kind, and closes terminal grants through the exact decision-time check
reference, authority receipt, integrity validation, and report artifact.

This implementation adds no new command, flag, declaration, schema, workflow
default, provider execution, or mutation capability.

## 2. Product Decision

CLI adoption remains declaration-controlled. The existing validated project
declaration is the only activation source:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

Users do not receive a new flag, command, mode, or default. Projects without
the declaration continue through ordinary execution. Projects with the exact
supported declaration continue to use the existing `run` and `approve`
commands, but Core supplies the selected source-backed assessment and closure.

The CLI may construct transport and local-store dependencies. It may not
construct current runtime facts, source registration, source identity,
evaluation time, governance disposition, disclosure requirement, or authority
receipt.

## 3. Goals

The implementation sequence must:

1. add one selected fresh-run report-composition adapter in Core;
2. execute the canonical project-validation check exactly once per route or
   current terminal reassessment;
3. derive the report reference from that actual same-call check result;
4. preserve terminal report generation and non-terminal deferral for every
   accepted route;
5. preserve existing local report-artifact persistence and proof-marker gates;
6. route declared CLI `run` through the selected consumer only after the
   adapter is reviewed;
7. route both aggregate-governance and separately authored workflow-step
   approvals created under the selected binding through the selected decision
   helper;
8. persist and validate trusted authority receipts only where the accepted
   approval-resume closure produces them;
9. preserve current human output, JSON fields, error posture, exit status,
   retry behavior, and durable event ordering;
10. keep ordinary execution and all existing public Core APIs available;
11. prove exact compatibility before removing duplicate CLI-only composition;
12. expose the exact selected local-check reference and bounded approval-gate
    kind from Core rather than reconstructing them in CLI; and
13. preserve workflow-derived high-assurance and approval-proof-marker artifact
    gates before any CLI cutover.

## 4. Strict Non-Goals

This adoption does not authorize:

- activation for undeclared projects;
- inferred activation from repository metadata or recommendations;
- a new CLI command or runtime flag;
- multi-step authoritative-governance expansion;
- arbitrary check profiles or command strings;
- caller-provided current facts, source registration, or evaluation time;
- automatic approval or model self-approval;
- provider execution or OpenShell integration;
- SideEffect execution or a new external mutation family;
- schemas, SDK changes, example changes, or scaffold-default changes;
- hosted or distributed behavior;
- enterprise stewardship or identity administration;
- reasoning lineage or nested harness execution; or
- release posture changes.

## 5. Current CLI Boundary

The declared authoritative CLI path currently:

- builds `LocalExecutionWithCoreOwnedAuthoritativeDocsCheckGovernanceRequest`;
- resolves the fixed project-validation profile;
- invokes
  `execute_with_core_owned_authoritative_explicit_local_check_profile_governance_report`;
- persists approval-presentation proof for approval routes;
- persists terminal report artifacts through
  `persist_authoritative_governance_report_artifact`;
- branches at approval time between aggregate-governance and authored-step
  approval helpers; and
- prints stable human and JSON output from the existing result vocabulary.

That path is already manifest-controlled and fail-closed. Adoption must change
its internal Core composition, not redesign the command parser or activation
contract.

## 6. Selected Consumer Boundary

The accepted selected consumer provides:

- `route_selected_project_validation_governance`, which owns current fact
  construction and returns the existing authoritative route vocabulary; and
- `decide_selected_project_validation_approval_report_artifact`, which validates
  presentation proof, reruns current definitions and the canonical check for
  grants, preserves source-free denial, derives a trusted receipt, builds a
  receipt-citing WorkReport, and persists or reconciles the receipt and
  artifact.

The selected decision helper accepts explicit stores because local storage is
a product dependency, not hidden global state. It does not discover state
roots or mutate provider systems.

The missing adoption prerequisite is selected fresh-run report composition.
The route already retains the actual same-call check result, so Core can reuse
the accepted report consumer without a second check invocation.

## 7. Phase 1: Selected Fresh-Run Report Adapter

Add one additive Core request and function, using repository naming
conventions, equivalent in purpose to:

```text
LocalSelectedProjectValidationGovernanceReportRequest
execute_selected_project_validation_governance_report(...)
```

The request should contain only:

- `LocalSelectedProjectValidationGovernanceRequest`;
- existing `LocalExecutionReportInputs`; and
- existing `AuthoritativeDocsCheckReportReferenceInputs`.

The helper must call the selected route exactly once and reuse the existing
report-composition implementation with selected source-backed route options.
It must:

- reject missing run identity and duplicate report references before process
  use;
- derive exactly one `LocalCheckResultReference` from the actual route result;
- return deferred report posture for non-terminal approval routes;
- generate a WorkReport for quiet, visible, denied, or existing-terminal
  outcomes;
- retain route truth when reference or report generation fails; and
- keep all errors and Debug output bounded and non-leaking.

The result must expose the exact `LocalCheckResultReference` constructed from
the same-call canonical result. The CLI may print that reference after a later
cutover; it must not fabricate a replacement from a caller-authored ID.

This phase does not touch the CLI.

Implementation status: complete with the focused-review blocker fixed pending
review. Core now
exports `LocalSelectedProjectValidationGovernanceReportRequest` and
`execute_selected_project_validation_governance_report`. The adapter reuses
the selected source-backed route and existing report compositor, returns the
exact same-call `LocalCheckResultReference`, defers non-terminal reports, and
supports existing-terminal reassessment without rerunning workflow skills.
For terminal reassessment it now chooses a fresh Core-owned evaluation time,
constructs a truthful current snapshot, and compares the current source
registration, runtime-fact commitment/count, assessment aggregate, and
immutable assessment core with the durable original. Observation timestamps
may differ. The result preserves the original durable binding and event history
while exposing only the transient reassessment time as provenance, not a new
authority binding. Current semantic drift fails closed before report
regeneration, event append, or skill replay.

Focused review accepted the blocker fix. The approval adoption envelope may
now proceed as the next bounded implementation phase.

## 8. Phase 1B: Selected Approval Adoption Envelope

Before CLI approval adoption, add one bounded Core result projection around the
selected decision helper. It must preserve the accepted decision and artifact
closure while exposing:

- the exact decision-time `LocalCheckResultReference` for grants;
- a bounded gate kind distinguishing aggregate governance from an authored
  workflow-step approval;
- terminal report posture, including a truthful denial report without a
  decision-time check rerun;
- workflow-derived high-assurance disclosure and approval-proof-marker gate
  results; and
- the existing receipt, artifact, persistence, and retry-blocked posture.

Gate kind is output vocabulary only. Core derives it from the durable approval
request and uses one selected decision path for both gates. The CLI may use the
kind only to preserve its current route label; it may not select a different
approval implementation from that value.

The selected helper currently returns a transient authority receipt for a
successful aggregate grant that advances to a separate authored gate. That
receipt is not persisted and no report artifact is written while the run is
non-terminal. Adoption must preserve this distinction and must not describe a
transient receipt as durable authority evidence.

Implementation status: complete and accepted by focused review. Core exports
`decide_selected_project_validation_approval_envelope` and bounded input/result
vocabulary. The envelope derives aggregate-versus-authored gate kind from the
durable approval request, requires presentation proof before decision work,
reruns the canonical project-validation check exactly once for grants, and
builds the local-check reference from the Core-produced decision-time result.
Denials remain check-free and receipt-free while producing truthful terminal
reports and workflow-gated artifacts. A non-terminal aggregate grant returns a
transient receipt but persists no receipt, projection, report, or artifact.
Terminal closure derives high-assurance and proof-marker policy from the exact
immutable workflow, persists proof-marker projections when required, and
retains bounded artifact and retry posture. Existing public approval APIs and
CLI behavior remain unchanged.

## 9. Phase 2: Declared `run` Adoption

After focused review of Phase 1, update only the already-declared authoritative
CLI branch:

1. load and validate the project as today;
2. resolve the supported declaration and immutable execution inputs as today;
3. construct `LocalSelectedProjectValidationGovernanceRequest` around the
   existing fact-free execution request;
4. call the selected report adapter;
5. persist approval-presentation proof for approval-required outcomes as
   today;
6. persist the terminal report artifact through the same workflow-derived
   high-assurance and approval-proof-marker gates used today;
7. print the existing human or JSON result without adding fields; and
8. preserve the existing denied and failed-run exit behavior.

The CLI must not precompute route selection or disclosure. It may supply the
existing visible-delivery handler only; Core consumes it conditionally after
the actual selected assessment requires visible delivery.

An existing terminal run retry must revalidate the immutable bundle and
selected current facts, regenerate the same report identity, and reconcile an
exact artifact duplicate without executing the workflow again.

## 10. Phase 3: Declared `approve` Adoption

For a waiting run created under the selected V3 governance binding:

1. rehydrate the run and exact immutable activation;
2. resolve the requested approval without selecting an implementation from its
   semantic type in CLI;
3. construct the existing proof-enforced approval request;
4. open the receipt store at
   `<state-root>/governance-decision-authority-receipts`, reuse the existing
   state backend for report artifacts and SideEffect records, and reuse
   `<state-root>/audit-projections/approval-proof-markers` for proof-marker
   projections;
5. call `decide_selected_project_validation_approval_report_artifact` for a
   grant or denial;
6. print the existing decision, run, approval-handoff, exact local-check
   reference, route label, report, and artifact posture; and
7. preserve truthful workflow status when report or persistence closure fails.

The selected Core helper, not a CLI branch selecting behavior from
`governance_approval_binding`, must preserve the distinction between the
aggregate gate and a separately authored step gate. A first grant may
legitimately return `WaitingForApproval` at the authored gate with a transient,
unpersisted receipt but no report artifact. A later grant reruns current facts
and may close the terminal run with a persisted receipt-citing artifact.
Neither gate implies approval of the other.

Denial requires valid presentation proof but remains check-free, source-free,
and receipt-free. The adoption envelope must compose a truthful terminal denial
report from durable run and prior selected-assessment references and pass it
through the existing workflow-derived artifact gates without rerunning the
project check or fabricating evidence. This preserves the current CLI terminal
artifact obligation. No CLI fallback may rerun the project check after a
selected denial.

## 11. Store And Artifact Posture

CLI adoption creates explicit local stores from the deterministic paths named
in Phase 3. Store construction must not inspect environment variables, user
home directories, or hidden runtime configuration.

Receipt records are evidence-only and point-in-time. They are not reusable
authority. Exact duplicate writes may reconcile idempotently; conflicts,
missing records, corrupt records, or ambiguous writes fail closed with existing
stable error families.

Quiet or visible terminal routes do not fabricate approval authority receipts.
They retain the existing source-bound governance assessment and local-check
evidence in the WorkReport. Approval-resume routes cite the trusted receipt
produced by the selected decision closure.

The selected closure must derive artifact policy from the immutable workflow
definition and persist proof-marker projections through the same explicit
projection store as the current CLI path. A hard-coded caller policy is not
equivalent and must fail focused compatibility review.

## 12. Output And Compatibility Contract

Adoption must preserve the current public command surface:

```text
workflow-os run <workflow-id> [--run-id <run-id>] [--verbose]
workflow-os approve <run-id> <approval-id> [--actor <actor>] [--reason <reason>] [--deny]
```

Required compatibility includes:

- route labels;
- run and approval identity fields;
- approval-handoff content and presentation proof;
- quiet-success summary behavior;
- visible-disclosure output;
- report and artifact posture fields;
- existing JSON keys and absence of payload fields;
- exit success for waiting approvals and accepted denials where currently
  defined;
- stable error-code families and non-leaking messages; and
- ordinary behavior for projects without authoritative activation.

New receipt or selected-source details must remain report evidence unless a
separate public-output phase is approved. This phase does not add them to CLI
output.

## 13. Failure Ordering

The implementation must preserve this order:

1. argument and project validation;
2. activation and closed-profile validation;
3. immutable bundle and complete check preflight;
4. canonical check execution exactly once;
5. Core-owned source observation and governance assessment;
6. durable binding before run execution;
7. presentation proof before every approval decision;
8. grant-side immutable reload and current-fact reassessment before mutation;
9. truthful run and approval mutation;
10. trusted receipt and WorkReport construction; and
11. receipt integrity and selected artifact gates before artifact persistence.

Pre-decision failures must not append approval events. Post-decision report or
persistence failures must not rewrite the truthful workflow result. Ambiguous
artifact outcomes must block blind retry until reconciled.

## 14. Migration And Rollback

Do not execute both old and selected paths in production to compare them: that
would rerun local checks and could duplicate workflow effects. Equivalence is
proved in deterministic tests over the same fixtures and expected event/state
projections.

The old public Core APIs remain available during adoption. The CLI switch is
limited to the already-activated declaration and should be one reviewable
change after the selected report adapter is accepted. If compatibility tests
fail, the CLI remains on the accepted old path; no runtime fallback should
silently choose between authority models after execution begins.

Duplicate CLI-only helper code may be retired only after a separate cleanup
review proves no other caller depends on it.

## 15. Test Plan

### Core adapter tests

- quiet, visible, approval-required, denied, and existing-terminal routes;
- one canonical check invocation per call;
- exact selected source-backed V3 binding;
- terminal report generation and non-terminal deferral;
- stable local-check reference derived from the actual result;
- duplicate or invalid reference preflight;
- report failure retaining route and run truth;
- relevant-definition invalidation and unrelated-definition stability;
- no caller-authored source registration, facts, or evaluation time; and
- redaction-safe Debug and errors.

### Core approval-envelope tests

- exact decision-time local-check reference for each grant;
- aggregate and authored gate-kind projection without divergent execution;
- transient aggregate receipt is not persisted for a non-terminal run;
- terminal approval denial produces a report artifact without check or source
  invocation;
- workflow-derived high-assurance disclosure policy parity;
- approval-proof-marker projection and artifact-gate parity; and
- no fabricated reference, receipt, report evidence, or gate kind.

### CLI adoption tests

- declared quiet, visible, approval-required, denied, and failed-check runs;
- undeclared ordinary execution unchanged;
- unsupported declaration failure before state mutation;
- aggregate approval followed by distinct authored-step approval;
- grant and denial with persisted presentation proof;
- missing, stale, mismatched, and ambiguous proof before mutation;
- decision-time failed check or changed definition before mutation;
- receipt persistence and exact duplicate reconciliation;
- report-artifact success, report failure, persistence failure, and ambiguous
  write posture;
- retry without duplicate workflow execution or duplicate artifact conflict;
- byte-for-byte human output fixtures where maintained;
- exact JSON key and value compatibility;
- no raw facts, source identity, paths, command output, report text,
  environment values, provider payloads, or credentials; and
- all existing CLI, executor, WorkReport, receipt, SideEffect, adapter,
  runtime, and state-backend tests.

## 16. Documentation And Validation

Each implementation phase must update the roadmap, its implementation report,
and focused review. Public user guides should change only during actual CLI
adoption, not during the Core prerequisite.

Required validation for implementation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused Core and CLI compatibility tests;
- `cargo test --workspace`;
- `npm run check:docs`;
- schema/SDK and integration checks only if touched by separately approved
  scope; and
- `git diff --check`.

## 17. Resolved Review Decisions

- Receipt records use
  `<state-root>/governance-decision-authority-receipts`; artifacts and
  SideEffect records continue to use the existing state backend; proof-marker
  projections retain their existing deterministic path.
- The selected report adapter should reuse the private generic report
  composition with a narrow selected route option; it must not duplicate
  report construction.
- Existing CLI semantic output assertions are compatibility requirements, and
  exact JSON keys and route labels are frozen for this adoption.
- Current declared initial denial persists a terminal report artifact. Selected
  approval denial must also preserve terminal report/artifact posture without
  decision-time check execution before CLI cutover.
- The selected approval result needs an adoption envelope because the current
  result omits the exact check reference, bounded gate kind, and
  workflow-derived proof-marker artifact-gate projection required by CLI.

## 18. Final Recommendation

The already-declared `run` and `approve` paths are adopted together and no
longer select divergent CLI-side approval implementations. Conduct a focused
maintainer review before broadening the selected consumer or retiring the old
public Core compatibility APIs.
