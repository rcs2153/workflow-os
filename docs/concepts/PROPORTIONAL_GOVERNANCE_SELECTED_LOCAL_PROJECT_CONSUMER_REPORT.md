# Proportional-Governance Selected Local Project Consumer Report

## 1. Executive Summary

The first additive selected-consumer composition API is implemented in
`workflow-core`. It composes the explicitly activated, one-step local
project-validation route with the Core-owned current-fact source, proof-enforced
approval decisions, fresh same-call project validation, trusted authority
receipts, terminal WorkReports, and local report-artifact persistence.

The implementation is local, explicit, opt-in, and store-injected. Existing
executor APIs and CLI behavior are unchanged. CLI adoption remains a separate
phase after focused review.

## 2. Scope Completed

- Added `LocalSelectedProjectValidationGovernanceRequest` and
  `route_selected_project_validation_governance`.
- Added `LocalSelectedProjectValidationArtifactDecisionInput` and
  `decide_selected_project_validation_approval_report_artifact`.
- Kept runtime-fact source identity, registration, facts, and same-call check
  ownership inside Core.
- Added accepted V3 current-runtime-fact commitment support to shared approval,
  disclosure, and authoritative route validation.
- Rebuilt the current immutable project bundle before each granted decision and
  rejected relevant-definition changes before mutation.
- Reused the accepted authority-receipt, WorkReport, referential-integrity,
  selected artifact-gate, and persistence composition.
- Preserved separate aggregate-governance and workflow step approvals when the
  selected workflow declares both.

## 3. Scope Explicitly Not Completed

- No CLI adoption or output changes.
- No executor default changes or automatic activation.
- No generic caller-provided authoritative fact source.
- No multi-step authoritative-governance expansion.
- No provider execution, OpenShell integration, SideEffect execution, or new
  mutation family.
- No schemas, examples, hosted behavior, enterprise administration, or release
  posture changes.

## 4. API Summary

The route API accepts the existing closed Core-owned project-validation request
plus an explicit evaluation timestamp. It executes the canonical selected check,
constructs the fixed Core source snapshot, persists the V3 governance binding,
and returns the existing quiet, visible, approval, or denied route vocabulary.

The decision API accepts explicit approval-presentation proof, the exact
selected execution identity, explicit report inputs, and injected local stores.
Proof validation occurs before project reload, check execution, or source
observation. A grant rebuilds and compares the current immutable bundle, reruns
the canonical check, reproduces the durable assessment core and source
registration, and delegates to the accepted receipt and artifact closure. A
denial invokes neither the decision-time check nor source.

## 5. Approval And Artifact Behavior

The implementation does not collapse distinct gates. When proportional
governance requires an aggregate approval and the workflow step also declares
an approval policy, the first proven grant resumes into the step approval and
does not write a terminal artifact. A second proven grant reruns current facts,
executes the step, produces the trusted receipt and receipt-citing WorkReport,
and persists the report artifact.

This preserves workflow semantics and avoids treating approval of aggregate
governance posture as implicit approval of a separately declared step action.

## 6. Validation And Failure Boundaries

- Missing or invalid presentation proof fails before recheck or mutation.
- Relevant current definition changes fail exact immutable-bundle comparison.
- Failed decision-time project validation cannot become satisfied evidence.
- Changed assessment core or source registration fails reassessment.
- Denial remains source-free and write-free after valid presentation proof.
- Post-decision report and persistence behavior continues to use the accepted
  generic closure and preserves truthful workflow state.
- Errors and Debug output remain bounded and do not expose facts, paths,
  identifiers, command output, report text, environment values, or credentials.

## 7. Test Coverage

Focused selected-consumer tests cover:

- V3 route binding and approval pause;
- complete two-gate grant through receipt-citing artifact persistence;
- source-free denial with no artifact writes;
- missing presentation proof before recheck and event mutation;
- relevant policy-definition invalidation before recheck; and
- failed decision-time validation before approval mutation or writes.

Existing bridge, generic approval-artifact closure, report failure, persistence
failure, retry, authoritative route, approval, WorkReport, SideEffect, adapter,
runtime, and CLI suites remain the broader regression boundary.

## 8. Privacy And Redaction

The public request and result Debug representations redact identity, report,
source, and evaluation details. The Core-owned source stores only validated
typed facts and payload-free bindings. No raw check output, source content,
path, environment value, token, provider payload, or credential is copied into
the authority receipt or WorkReport.

## 9. Commands Run

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed with the
  repository toolchain and an isolated target directory.
- Focused selected-consumer local-executor tests: 5 passed.
- `cargo test --workspace`: passed with the repository toolchain and an
  isolated target directory; opt-in live tests remained ignored as designed.
- `npm run check:docs`: passed under the repository Node 20 toolchain.
- `git diff --check`: passed.

## 10. Governed Phase Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1786440952567626000-2`.
- Approval ID:
  `approval/run-1786440952567626000-2/implementation-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/9465cb736a97f52c`.
- Terminal status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, no retries, and no
  escalations.
- Approval-presentation enforcement: proof enforced with the presentation
  marker present in the durable event trail.

Repository edits, shell commands, validation commands, and report authoring
were performed by the maintainer outside the kernel under this governed phase.
The kernel coordinated scope and approval and recorded the phase trail; it did
not edit files, execute checks, mutate git state, push a branch, or open a pull
request. Opt-in live integration tests were skipped by the workspace suite as
designed and were not simulated.

## 11. Remaining Limitations

- The API is not wired into CLI behavior.
- The selected workflow may require two distinct approvals; no gate is silently
  collapsed.
- The API supports the closed one-step project-validation profile only.
- Runtime-fact snapshots remain call-local evidence metadata, not reusable
  authority.
- Report and persistence failure/retry behavior is inherited from the accepted
  generic closure and has not been widened.

## 12. Recommended Next Phase

Perform a focused maintainer review of the selected-consumer composition API.
If accepted, plan CLI adoption as a separate compatibility-sensitive phase.
