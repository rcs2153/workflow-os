# First-Run Governed WorkReport Artifact Plan

Status: Accepted after focused maintainer/security review. This plan implements
the lane selected in the [First-Run Governed WorkReport Artifact Queue
Selection Report](../concepts/FIRST_RUN_GOVERNED_WORK_REPORT_ARTIFACT_QUEUE_SELECTION_REPORT.md).
The review required the bounded analysis commitment to become the existing
immutable run bundle's resolved execution-context commitment before approval or
execution. Runtime implementation remains separately governed.

Related foundations:

- [First-Run Governed Ledger/Report Plan](first-run-governed-ledger-report-plan.md)
- [Authoritative WorkReport Artifact Persistence Plan](authoritative-work-report-artifact-persistence-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)
- [Report Artifact Write Integration Plan](report-artifact-write-integration-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)

## 1. Executive Summary

`workflow-os first-run` already performs useful, bounded repository posture
analysis. It validates the Workflow OS project, inspects safe repository
metadata, maps governance posture, and recommends review-only workflows. It is
intentionally non-mutating and must remain so.

The generated `local/first-run-governance` workflow currently demonstrates
approval and audit mechanics through a mock-only skill. It does not perform or
retain the real first-run analysis. The next vertical slice should replace that
gap with one explicit opt-in governed operation:

```text
workflow-os first-run --write-report-artifact
```

The explicit path should execute the existing generated workflow identity with
one Workflow OS-owned trusted local first-run analysis handler. It should compute the
bounded analysis before run creation, bind its versioned commitment into the
immutable run bundle, route through the project's proportional-governance and
approval posture, construct a truthful terminal `WorkReport` from an owned
same-call snapshot whose commitment matches that bundle, and persist it through
the existing local report-artifact store and governance gates.

This is not permission to make ordinary first-run mutating, execute repository
commands, call providers, inspect arbitrary source contents, add OpenShell, or
broaden external writes.

## 2. Architectural Decision

Do not write a standalone first-run report file.

`WorkReportArtifactRecord` is intentionally bound to a terminal
`WorkflowRun`. Bypassing that identity would create a second artifact contract
without run status, event history, approval proof, immutable definition
identity, or existing artifact gates.

Use the generated workflow and skill identities:

- workflow: `local/first-run-governance`;
- skill: `local/first-run-report`;
- step: `first-run-report`.

The explicit CLI path should register one Workflow OS-owned trusted local
handler for that exact skill ID and version. Repository observation remains a
trusted-host/CLI responsibility; Core remains responsible for immutable run
identity, routing, approvals, execution state, report validation, and artifact
gates. The handler performs only the same bounded analysis used by ordinary
`first-run`. It is not a shell handler, arbitrary local skill registry, mock
handler, provider adapter, or user-supplied executable.

The workflow remains project-authored and immutable-run-bound. Projects that
replace the generated workflow or skill with incompatible definitions must
fail validation rather than silently receiving the built-in handler.

## 3. User Experience

### Ordinary analysis

```sh
workflow-os first-run
workflow-os first-run --verbose
workflow-os --json first-run
```

These commands remain non-mutating. They do not create a run, approval,
runtime event, state directory, or report artifact.

### Explicit governed artifact path

```sh
workflow-os first-run --write-report-artifact
```

The command should:

1. load and validate the project;
2. validate the exact generated first-run workflow/skill contract;
3. compute one bounded analysis snapshot and immutable commitment;
4. create or resume a durable governed run bound to that commitment;
5. apply authoritative proportional-governance routing;
6. pause with a complete proof-enforced approval handoff when required;
7. on resume, recompute and verify the analysis commitment before execution;
8. construct a terminal `WorkReport` from the exact owned analysis snapshot;
9. persist the validated report artifact through existing gates; and
10. print bounded run, route, report, artifact, and inspection metadata.

Example bounded success output:

```text
mode: first_run_governed_report_artifact
status: completed
run_id: <bounded-id>
route: quiet_proceed
report_id: <bounded-id>
report_artifact_posture: persisted
analysis_commitment: <sha256>
inspect: workflow-os inspect <run-id>
```

The command must not print the report body by default.

## 4. Command Contract

Add one opt-in flag to `first-run`:

```text
--write-report-artifact
```

Compatibility rules:

- absent flag: current behavior is byte-for-byte compatible where practical;
- `--verbose`: allowed with the flag only if output remains bounded and does
  not print report contents;
- `--recommendation <id>`: incompatible with the flag and rejected with a
  stable usage error;
- `--json`: returns the same bounded posture fields as human output;
- `--mock-all-local-skills`: rejected for this path because mock output cannot
  become report evidence;
- explicit run ID support may be added only if it follows existing run-ID
  validation and does not permit identity reuse across changed inputs.

The exact flag name should be confirmed during implementation against CLI
conventions. Do not add aliases in the first slice.

## 5. Core-Owned Analysis Handler Boundary

Extract the current first-run analysis into one reusable function that returns
a validated bounded value rather than printing directly. A likely internal
shape is:

```text
FirstRunAnalysisSnapshot
```

The exact name may follow repository conventions. It should be a CLI-private
or crate-private composition type, not a new public schema family. It should
contain only the
already accepted bounded fields:

- scaffold presence;
- Git repository presence as a boolean;
- workflow, skill, policy, and test counts;
- governance field posture;
- ownership and escalation findings;
- spec-field coverage findings;
- safe repository metadata;
- proportional-governance assessments;
- workflow-discovery recommendations and next-action codes;
- bounded first-run recommendations;
- authoritative-execution declaration posture;
- all required WorkReport section inputs, incomplete-work disclosures,
  limitations, risks, and handoff notes; and
- a deterministic analysis commitment.

It must not contain:

- raw source contents;
- raw manifest or spec bytes;
- raw command, check, provider, or CI output;
- environment values;
- credentials or token-like values;
- private absolute paths;
- dynamically fetched network data; or
- model-generated narrative.

Ordinary first-run text and JSON output should consume this same value. The
trusted local workflow handler must also consume this same value. Do not maintain
two analysis implementations.

## 6. Deterministic Analysis Commitment

The snapshot needs one versioned deterministic commitment, for example:

```text
workflow-os/first-run-analysis/v1
```

The SHA-256 input should use labeled, length-delimited canonical fields in
stable order. It should include:

- manifest schema and project identity already available from validated
  definitions;
- workflow, skill, policy, and test IDs, versions where modeled, and accepted
  content hashes in canonical order;
- safe repository metadata codes and bounded values;
- governance, ownership, escalation, field-coverage, recommendation, and
  proportional-governance codes;
- generated first-run workflow and skill contract identity;
- disclosure, limitation, risk, and handoff codes or canonical bounded text;
  and
- analysis commitment version.

It must not depend on:

- map iteration order;
- absolute paths;
- wall-clock time;
- run, event, approval, or report IDs;
- serializer debug output;
- platform-specific path separators;
- environment values; or
- report generation time.

The commitment is an integrity value, not proof that repository commands,
tests, providers, or source analysis ran.

## 7. Same-Call And Time-Of-Use Rules

The handler must create one `FirstRunAnalysisSnapshot` and use that same owned
value to:

- produce the handler result;
- generate report sections and disclosures;
- return the analysis commitment; and
- construct the terminal report artifact.

Do not recompute the analysis between handler success and report construction.

Before run creation, compute one bounded snapshot and bind its commitment as
the immutable run bundle's resolved execution-context commitment. That makes
the exact posture under review part of run identity without storing raw source
or a second snapshot artifact.

Quiet and visible routes use that same owned snapshot through handler result,
report construction, and artifact persistence in the same call.

Approval-required routes persist only the run, immutable bundle commitment,
events, and approval presentation before pausing. On proof-enforced approval
resume, recompute one bounded snapshot, require its commitment to equal the
stored immutable run commitment, and use that exact owned value through
handler result, report construction, and artifact persistence. A mismatch
fails closed before handler success or artifact write.

Immediately before artifact construction, recompute only the commitment from
current bounded inputs and compare it with both the owned snapshot commitment
and immutable run commitment. This final read is a time-of-use guard, not a
replacement report source.

## 8. Workflow And Immutable-Run Binding

The run must use the existing immutable-run bundle path. The exact validated
workflow, skill, policy, approval, report-artifact requirements, and first-run
analysis commitment must be bound before execution. The analysis commitment
becomes the existing `resolved_execution_context_hash` supplied to immutable
bundle construction; do not add a second bundle hash or parallel snapshot
store. This field is already compared across approval resume and immutable
bundle validation.

Built-in handler eligibility requires all of the following:

- exact workflow ID `local/first-run-governance`;
- exact supported workflow version and a versioned allowlisted canonical
  contract fingerprint;
- exact step ID `first-run-report`;
- exact skill ID and supported version;
- expected no-provider/no-adapter requirement posture;
- no unrecognized step, input, output, policy, hook, or SideEffect widening;
  and
- accepted immutable bundle identity at execution time.

If a project customizes the generated definitions beyond an allowlisted exact
contract, the command fails with a bounded `unsupported_contract` error and
does not fall back to mock execution. Partial semantic matching is forbidden:
unknown fields or changed approvals, policies, inputs, outputs, hooks, steps,
SideEffects, or report requirements must not receive Core execution authority.
New scaffold versions require a reviewed new fingerprint entry.

## 9. Proportional Governance And Approval

The explicit path should reuse the accepted authoritative governance router.
It must not invent a first-run-specific approval system.

- `quiet_capture`: proceed without interruption and retain the full event and
  report artifact trail;
- visible disclosure delivery: proceed while returning the bounded disclosure
  posture already modeled by the authoritative route;
- `blocking_approval`: stop at the existing proof-enforced approval boundary,
  emit the complete handoff, and include the bounded analysis commitment rather
  than raw analysis content;
- denial: fail closed and retain the durable denial trail; and
- unknown or unsupported route: fail closed.

Visible disclosure remains delivery posture, not independent execution
authority. The implementation should use the current accepted route model
rather than reopening that product decision.

## 10. WorkReport Construction

The report must be a real terminal `WorkReport`, not the existing report-ready
context and not a mock-skill summary.

Use:

- workflow/run/spec identity from the terminal immutable-bound run;
- `report/<run-id>` as the deterministic report ID;
- terminal event timestamp as `generated_at`;
- durable terminal correlation ID, with the documented run-creation fallback;
- a bounded Core system actor for `generated_by`;
- existing v1 contract identity;
- the exact owned first-run snapshot for sections and disclosures; and
- existing approval references/proof-marker citations when the route required
  approval.

Truthful section posture:

- Work performed: bounded first-run governance analysis only.
- Evidence considered: safe metadata and project validation posture only;
  never claim command or provider evidence.
- Decisions made: recommendation and route codes, explicitly review-only.
- Policy gates: only gates actually evaluated by the governed run.
- Approvals: actual durable request/decision posture only.
- Validation/checks: project validation passed; repository checks skipped.
- Side effects: local runtime state and report artifact write disclosed; no
  external SideEffect occurred.
- Incomplete work: commands, tests, providers, source inspection, workflow
  promotion, and external writes remain unperformed.
- Limitations and risks: retain current honest first-run posture.
- Handoff: inspect the run/artifact and review recommendations.

The analysis commitment should appear in one bounded report summary and CLI
metadata. Do not fabricate an `EvidenceReference` or validation record solely
to carry the hash.

## 11. Artifact Persistence And Governance Gates

Persist through the existing authoritative artifact composition and local
`WorkReportArtifactStore`.

Required behavior:

1. terminal run exists;
2. same-call snapshot and time-of-use commitment match;
3. terminal report validates;
4. `WorkReportArtifactRecord` validates against the run/report identity;
5. workflow-declared high-assurance and proof-marker requirements derive from
   the immutable definition;
6. approval projection/gates pass when applicable;
7. SideEffect gates see no external SideEffect citations unless future scoped
   work adds them; and
8. create-only artifact write succeeds or reconciles exact equality.

The local state and report artifact writes are real local side effects and must
be disclosed in command output and report content. They are not provider or
external writes.

## 12. Idempotency, Replay, Conflict, And Interruption

### Exact completed retry

When the supplied run ID is already terminal and the expected artifact exists:

- load and validate the immutable run and artifact;
- verify workflow/skill contract and report/run identity;
- return `already_persisted` without rerunning analysis; and
- print the existing bounded metadata.

### Duplicate concurrent write

After a duplicate-write rejection, re-read the artifact. Return idempotent
success only when the validated artifact is exactly equal. Never overwrite.

### Identity reuse

Reusing a run ID with different immutable inputs, analysis commitment, report
identity, or artifact content fails closed.

### Crash before terminal run

Normal executor recovery rules apply. No report artifact exists yet.

### Crash after terminal run but before artifact persistence

Recompute one bounded snapshot and require its commitment to equal the
immutable run commitment. If equal, reconstruct the deterministic terminal
report from durable run events plus that exact owned snapshot, then run normal
artifact gates and create-only reconciliation. If unequal or unavailable, fail
closed with `terminal_artifact_recovery_drift` and preserve the run.

This recovery is not silent reconstruction from unrelated current state. The
immutable commitment is the authority boundary, and every field that can
affect report content must participate in it. The implementation must include a
test that mutates each bounded input family and proves recovery rejection.

### Artifact gate or store failure

Do not rewrite workflow pass/fail history. Return bounded artifact failure
posture and preserve the terminal run for inspection.

## 13. Error Model

Add stable `cli.first_run.artifact.*` errors only where existing error codes do
not fit. Candidate categories:

- `unsupported_contract`;
- `mock_mode_rejected`;
- `analysis_failed`;
- `analysis_drift`;
- `approval_required` through existing posture, not an error;
- `report_generation_failed`;
- `artifact_persistence_failed`;
- `terminal_artifact_recovery_drift`; and
- `identity_conflict`.

Errors must not contain raw report content, recommendations, paths, source
contents, spec bytes, command output, environment values, credentials, or
provider payloads.

## 14. CLI Output And Inspection

Human and JSON output should expose only:

- mode and status;
- route and execution disposition;
- run ID;
- approval ID and complete approval-presentation handoff when waiting;
- report posture and report ID;
- artifact posture;
- analysis commitment;
- bounded error code when present; and
- `workflow-os inspect <run-id>` next action.

`inspect` should continue to list artifact metadata through the existing path.
Do not add default full report rendering or export in this slice.

## 15. Generated Mock Demo Relationship

The generated workflow identity becomes the real governed first-run artifact
contract only through the explicit `first-run --write-report-artifact` path.

The generic command remains an approval/audit demonstration:

```sh
workflow-os --mock-all-local-skills run local/first-run-governance
```

Its mock output must not be persisted as first-run analysis evidence or a
first-run WorkReport artifact. CLI and docs should clearly distinguish:

- ordinary `first-run`: real non-mutating posture analysis;
- explicit artifact path: real governed analysis plus durable local artifact;
  and
- generic mock run: optional approval/audit mechanics demonstration.

Do not deprecate the demo in this slice. Revisit it only after external user
testing confirms the real path is understandable.

## 16. Test Plan

Focused Core/CLI tests must prove:

- ordinary first-run remains non-mutating;
- explicit flag creates a real governed run and never selects mock handlers;
- exact generated workflow/skill contract is required;
- customized or widened contract fails closed;
- quiet route completes and persists one valid artifact;
- visible route preserves disclosure posture and persists one artifact;
- approval route writes no artifact before approval;
- proof-enforced approval resumes, analyzes once, and persists one artifact;
- denial writes no success artifact and retains denial events;
- owned snapshot is reused for report construction;
- pre-write metadata/manifest drift fails before artifact creation;
- exact completed retry returns the equal artifact without re-analysis;
- concurrent duplicate equality returns idempotent success;
- conflicting duplicate fails without overwrite;
- terminal run with missing artifact reconstructs only when the recomputed
  snapshot matches the immutable commitment;
- terminal artifact recovery fails when any bounded input family drifts;
- skipped checks, absent evidence, and no external SideEffects stay explicit;
- mock results cannot enter report citations or sections as evidence;
- no repository commands, providers, network access, arbitrary source reads,
  or external writes occur;
- JSON/human errors and `Debug` output remain bounded and non-leaking;
- existing first-run, mock demo, WorkReport, artifact, approval, immutable-run,
  proportional-governance, and inspect tests continue to pass.

Disposable external-repository acceptance tests should cover:

1. scaffold a fixture with existing unmanaged `AGENTS.md` content;
2. verify ordinary first-run writes nothing;
3. run the explicit quiet-capture path and inspect its artifact;
4. run an approval-required fixture and confirm no artifact before approval;
5. approve with presentation proof and inspect the artifact/event trail;
6. mutate bounded metadata during a controlled pre-write hook and confirm
   drift failure; and
7. retry the completed run and prove no duplicate artifact or analysis.

## 17. Implementation Sequence

Implement as one reviewed vertical slice, with internal commits as needed:

1. extract reusable bounded first-run analysis and canonical commitment;
2. add exact generated-contract validation and trusted local handler registration;
3. add explicit CLI flag and authoritative route integration;
4. compose terminal WorkReport and governed artifact persistence;
5. implement exact replay, conflict, and crash-gap behavior;
6. add bounded human/JSON output and inspect verification;
7. add focused and disposable-repository tests;
8. update CLI/user docs and implementation report; and
9. perform focused maintainer/security review before merge.

Do not split the work into additional model-only roadmap phases unless review
finds a concrete invariant that cannot be safely implemented in this vertical
slice.

## 18. Review Questions

Focused review must answer:

1. **Built-in authority:** accepted only with exact versioned allowlisted
   contract fingerprints. ID/version checks or partial semantic matching alone
   are insufficient.
2. **Approval timing:** revised. The bounded analysis commitment is computed
   and immutable-bound before approval; resume recomputes and must match before
   execution.
3. **Mixed state:** accepted only when the same owned snapshot feeds handler,
   report, and artifact paths, with a final commitment comparison before write.
4. **Crash gap:** revised. Deterministic recovery is allowed only when a fresh
   bounded snapshot exactly matches the immutable commitment.
5. **Exact retry:** accepted. Existing artifacts are loaded and validated
   without rerunning analysis; missing artifacts follow commitment-gated
   recovery.
6. **Side-effect truth:** accepted when local runtime/artifact writes are
   disclosed separately from external SideEffects.
7. **Report truth:** accepted with the explicit section policy and mock-output
   prohibition in this plan.
8. **Contract reuse:** accepted. The design must reuse immutable-run,
   approval-presentation, proportional-governance, WorkReport, artifact-store,
   and inspect paths rather than create parallel persistence.

Review outcome: accepted with the revisions above. No unresolved critical
finding remains in the planning boundary. Implementation review must verify
that the commitment covers every report-affecting bounded input and that the
allowlisted generated-contract check is exact.

## 19. Strict Non-Goals

This plan does not authorize:

- default first-run mutation;
- automatic run, approval, or artifact creation;
- arbitrary source inspection;
- shell, Cargo, npm, test, lint, or build execution;
- provider reads or writes;
- network access or credentials;
- OpenShell integration or sandbox claims;
- new external SideEffect families;
- report publication or export;
- hosted/shared storage;
- schema or SDK exposure;
- recursive agents, agent swarms, or nested harness execution;
- enterprise administration; or
- release-posture changes.

## 20. Acceptance Criteria

The implementation is acceptable only when:

- ordinary first-run remains non-mutating;
- the explicit path creates a real governed run, not a synthetic artifact;
- only the exact accepted generated contract can receive the trusted local
  handler;
- a versioned analysis commitment is immutable-bound before approval or
  execution;
- approval and proportional-governance routes remain authoritative;
- one owned same-call analysis produces the report and commitment;
- drift fails before artifact write;
- report content is truthful and mock-free;
- one validated create-only artifact is persisted through existing gates;
- exact retry is idempotent without re-analysis;
- identity/content conflicts fail closed;
- missing-artifact recovery occurs only under immutable commitment equality;
- output and errors are bounded and non-leaking;
- disposable real-repository tests pass; and
- focused maintainer/security review accepts the implementation.

## 21. Reviewed Implementation Surfaces

Focused review checked the plan against these existing source contracts:

- [`first_run_command` and `FirstRunReportReadyContext`](../../crates/workflow-cli/src/main.rs)
  for current non-mutating analysis and output;
- [`SkillHandler` and `LocalSkillRegistry`](../../crates/workflow-core/src/executor.rs)
  for explicit non-default trusted local handler registration;
- [`ImmutableRunBundleBuildRequest`](../../crates/workflow-core/src/immutable_run_bundle_builder.rs)
  for the existing resolved execution-context commitment;
- [`WorkReport`, terminal report generation, and governed artifact
  composition](../../crates/workflow-core/src/work_report.rs);
- [`WorkReportArtifactStore`](../../crates/workflow-core/src/state.rs); and
- existing authoritative report artifact CLI composition in
  [`workflow-cli`](../../crates/workflow-cli/src/main.rs).

The review found no need for a public schema, SDK contract, parallel artifact
store, or new general-purpose execution primitive.

## 22. Governance And Validation

Planning phase:

- workflow ID: `dg/d`;
- run ID: `run-1793432595292380000-2`;
- approval ID: `approval/run-1793432595292380000-2/planning-approved`;
- presentation ID: `presentation/8b9909f4d705de10`;
- outcome: granted under standing delegated-maintainer authority; and
- close: 39 events, one proof-enforced approval, zero retries, zero
  escalations.

Focused review phase:

- workflow ID: `dg/review`;
- run ID: `run-1793432707067659000-2`;
- approval ID: `approval/run-1793432707067659000-2/review-scope-approved`;
- presentation ID: `presentation/95e88e7bb3464cac`;
- outcome: review authorized under standing delegated-maintainer authority;
  and
- close: 39 events, one proof-enforced approval, zero retries, zero
  escalations.

Review disposition: accepted with required revisions recorded in Section 18.

Validation:

- pinned Node 20 `npm run check:docs`: passed;
- `git diff --check`: passed;
- source links: passed through the documentation checker; and
- Rust formatting, linting, and tests: intentionally deferred because this
  milestone changes planning documentation only.

Out-of-kernel work includes repository inspection, architecture/security
analysis, documentation edits, and validation commands. The kernel coordinated
approval and durable phase history; it did not perform those actions.
