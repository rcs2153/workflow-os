# Authoritative WorkReport Artifact Persistence Plan

Status: Implemented. The explicit and project-controlled authoritative
proportional-governance paths now persist validated terminal `WorkReport`
artifacts through the existing local artifact store and governance gates.
Pending approvals still defer report and artifact creation, ordinary
undeclared execution remains unchanged, and exact terminal retries revalidate
the immutable bundle, rerun the closed check, reproduce the governance
binding, and reconcile only byte-equivalent validated artifacts.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Authoritative Quiet-Success CLI Preview Plan](authoritative-quiet-success-cli-preview-plan.md)
- [Report Artifact Plan](report-artifact-plan.md)
- [Report Artifact SideEffect Referential Integrity Plan](report-artifact-side-effect-referential-integrity-plan.md)
- [Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan](workflow-declared-high-assurance-artifact-requirement-executor-integration-plan.md)
- [Workflow-Declared Proof-Marker Artifact Executor Integration Plan](workflow-declared-proof-marker-artifact-executor-integration-plan.md)
- [Approval Resume Resolved-Context Integrity Plan](approval-resume-resolved-context-integrity-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)

## 1. Executive Summary

Workflow OS now has the pieces needed to make quiet successful work durable
without adding another approval or operator step:

- project-controlled authoritative governance activation;
- deterministic proportional-governance routing;
- same-call local-check evidence;
- terminal in-memory `WorkReport` generation;
- a validated local report-artifact store;
- SideEffect referential-integrity and approval-linkage gates;
- high-assurance approval-disclosure gates;
- approval proof-marker projection gates; and
- immutable run identity and approval-resume context checks.

The missing product boundary is composition. A terminal authoritative run
prints a report posture and report ID, but the report itself is discarded.
That weakens the quiet-success promise: low-risk work can proceed without
interruption, but its governed handoff is not durably inspectable.

The first implementation should make terminal report artifact persistence part
of the already explicit authoritative governance path. It should not add a
second opt-in flag. Ordinary undeclared `run` and `approve` behavior must remain
unchanged.

This implementation does not add provider execution, OpenShell integration,
new SideEffect families, hosted storage, report export, or broader execution
behavior.

## 2. Product Feedback Alignment

Fresh-pull evaluation now consistently describes Workflow OS as a credible and
honest local governance kernel. The remaining product pressure is not more
governance vocabulary. It is:

- less ceremony for low-risk work;
- complete evidence retention even when work remains quiet;
- durable artifact capture;
- machine-readable reporting; and
- clear inspection after execution.

This phase directly addresses those points. It does not turn Workflow OS into
an execution platform. The agent or handler still executes. Workflow OS
governs, records, validates, and retains the resulting handoff.

The reported Node 24 integration-helper failure and duplicate pre-scaffold
missing-manifest diagnostic are already fixed in
[Fresh-Pull Evaluator UX And Tooling Fix Report](../concepts/FRESH_PULL_EVALUATOR_UX_AND_TOOLING_FIX_REPORT.md).
They are regression coverage, not blockers for this phase.

## 3. Goals

- Persist one validated terminal `WorkReport` for the explicit authoritative
  local path.
- Reuse the existing `WorkReportArtifactRecord`,
  `WorkReportArtifactStore`, and governed artifact gates.
- Keep ordinary non-authoritative execution unchanged.
- Preserve workflow pass/fail/approval semantics.
- Preserve quiet success for eligible terminal work.
- Make persisted artifact posture visible in bounded human and JSON output.
- Make artifact identity and content deterministic across exact retry.
- Treat exact duplicate persistence as idempotent and conflicting duplicates
  as fail-closed integrity errors.
- Expose bounded report-artifact metadata through existing run inspection.
- Keep report content, local-check output, provider payloads, and secrets out of
  CLI status/error output.
- Retain explicit report-generation and artifact-write failure posture without
  rewriting durable run history.

## 4. Non-Goals

This phase must not add:

- report persistence to ordinary undeclared executor paths;
- a second `--persist-report` adoption flag;
- provider calls or provider mutation expansion;
- OpenShell or another sandbox/runtime provider;
- automatic report artifacts for every Workflow OS run;
- a new report model, artifact model, store, or schema;
- arbitrary filesystem output outside the existing local state backend;
- hosted, remote, or shared artifact storage;
- report export, signing, notarization, or publication;
- full report-content rendering in `inspect`;
- post-terminal workflow events;
- workflow snapshot mutation after terminal state;
- new SideEffect or approval semantics;
- inferred approval proof;
- automatic approval or model self-approval;
- DLP, access-control, retention, or enterprise administration;
- examples or release-posture changes.

## 5. Existing Runtime Foundation

The implementation must compose existing reviewed surfaces rather than create a
parallel persistence path:

- `LocalExecutionWithAuthoritativeGovernanceReportResult` contains the selected
  route, terminal run, generated report posture, generated `WorkReport`, and
  bounded report error.
- `WorkReportArtifactRecord::new(...)` validates the report and derives matching
  artifact metadata.
- `LocalStateBackend` implements `WorkReportArtifactStore` and
  `SideEffectRecordStore`.
- `write_work_report_artifact_with_governance_gates(...)` validates:
  - artifact/run identity;
  - cited SideEffect existence when required;
  - SideEffect approval linkage;
  - high-assurance approval disclosure; and
  - store-backed approval proof-marker posture.
- Existing workflow declarations derive high-assurance and proof-marker
  requirements for report artifact paths.
- Existing proof-marker projection helpers can derive bounded durable
  projections from accepted approval events.

No implementation should bypass these gates by calling the store directly from
the CLI.

## 6. Activation Boundary

Artifact persistence should be active only when execution is already using the
authoritative governance path:

- explicit `--authoritative-governance`; or
- the accepted project-controlled authoritative execution declaration.

That path is already an opt-in governance contract. Requiring another
persistence flag would add avoidable ceremony and create a misleading state in
which a project requests authoritative governed reporting but silently discards
the report.

Ordinary `workflow-os run`, ordinary approval commands, mock skill demos,
first-run posture analysis, and undeclared projects must retain current
behavior.

## 7. Candidate Core Composition

Add the smallest additive Core composition around the existing authoritative
report result. Likely concepts include:

- `LocalExecutionWithAuthoritativeGovernanceArtifactResult`;
- `AuthoritativeGovernanceArtifactPosture`;
- one fresh-run composition helper; and
- one approval-decision/resume composition helper.

The result should retain:

- the existing authoritative route result;
- report posture;
- optional validated artifact record;
- artifact persistence posture;
- optional stable artifact error;
- approval proof-marker projection posture where evaluated; and
- the existing local-check result reference.

It must not expose report body text, local-check output, raw approval metadata,
paths, tokens, or provider payloads through `Debug`.

The exact type names should follow surrounding executor conventions. Do not
redesign the authoritative route or artifact APIs.

## 8. Composition Order

For terminal authoritative outcomes:

1. Complete the accepted authoritative route and durable workflow events.
2. Generate the terminal report from the same-call local-check result.
3. Construct and validate `WorkReportArtifactRecord`.
4. Derive workflow-authored artifact policies from the exact immutable run
   definitions.
5. Persist any required approval proof-marker audit projections from the
   durable run.
6. Evaluate SideEffect referential integrity and approval linkage.
7. Evaluate high-assurance disclosure requirements.
8. Evaluate approval proof-marker requirements.
9. Write through `WorkReportArtifactStore`.
10. Return bounded persistence posture without mutating the run.

For non-terminal approval-required outcomes:

- keep report generation deferred;
- write no artifact;
- preserve the complete approval presentation handoff; and
- perform artifact composition only after a terminal approval decision path
  generates a report.

The implementation must not append a report-created event after terminal state.

## 9. Deterministic Report Identity And Retry

The current authoritative report ID is stable for a run, but report
`generated_at` and correlation inputs are constructed from current wall-clock
and caller state. That is acceptable for in-memory output but is not sufficient
for create-only durable artifact retry.

Before persistence, the authoritative report must derive retry-stable identity
inputs from durable run history:

- `report_id` remains derived from the validated run ID;
- `generated_at` should use one documented durable event timestamp, preferably
  the terminal event timestamp;
- correlation should use the durable terminal event correlation ID, or the
  durable run-creation correlation ID under one documented fallback;
- generated actor should use a stable bounded system actor or the selected
  durable terminal actor under one documented rule; and
- workflow identity, version, schema, spec hash, terminal status, and run ID
  continue to come from the terminal run.

Exact retry behavior:

- if no artifact exists, write it;
- if an exactly equal validated artifact already exists, return
  `already_persisted` success;
- if the same run/report identity resolves to different artifact content,
  return a stable conflict error;
- after a concurrent duplicate-write rejection, re-read and apply the same
  exact-equality rule;
- never overwrite or repair a conflicting artifact.

Tests must prove that fresh completion, completed-run retry, approval-resume
retry, and concurrent duplicate handling are deterministic.

## 10. Terminal Status Behavior

The composition should preserve current report-generation support:

- completed: persist the generated terminal report;
- failed: persist when the authoritative consumer generated a valid report;
- canceled: persist when that route is currently supported by the
  authoritative report consumer;
- denied: persist the terminal denial report when generated;
- waiting for approval: defer report and artifact;
- other non-terminal states: defer and disclose.

This phase must not introduce new terminal statuses or change runtime transition
rules.

## 11. Artifact Governance Gates

Every write must pass the strictest combination of caller-independent and
workflow-authored requirements.

Required posture:

- validate artifact/run identity;
- require every cited SideEffect record when the derived policy requires it;
- require approval references for `RequiresApproval` SideEffects when declared;
- require matching decisions for approved/denied SideEffects when declared;
- enforce high-assurance disclosure requirements;
- enforce proof-marker projection requirements;
- never infer a missing projection from an approval ID alone;
- never weaken a workflow requirement because the selected governance route was
  quiet; and
- never fabricate missing citations or evidence.

Quiet execution changes interruption posture, not artifact integrity.

## 12. Failure Semantics

Workflow execution and artifact persistence remain separate outcomes.

If report generation or artifact persistence fails after a terminal run exists:

- do not change the terminal workflow status;
- do not append workflow events;
- do not mutate the snapshot;
- return a non-success CLI result for the authoritative operation;
- print a stable non-leaking error code and inspect command;
- do not print quiet-success wording;
- retain the in-memory report when generation succeeded but persistence failed;
- do not claim an artifact exists; and
- permit an exact retry to complete persistence later.

This is not a retroactive workflow failure. It is a failed governed handoff
obligation attached to an otherwise unchanged terminal run.

## 13. CLI And Quiet-Success UX

The concise terminal quiet-success shape should remain bounded:

```text
status: completed
governance: quiet_proceed
report: persisted report/<run-id>
inspect: workflow-os inspect <run-id>
```

Exact wording may follow existing CLI style, but it must distinguish:

- `persisted`;
- `already_persisted`;
- `deferred_non_terminal`;
- `generation_failed`; and
- `persistence_failed`.

`--verbose` should retain existing route, disclosure, report, and local-check
detail and add bounded artifact gate posture.

JSON should add stable fields rather than remove or rename current fields:

- `report_artifact_posture`;
- `report_artifact_id` when present;
- `report_artifact_error_code` when present; and
- bounded gate summaries where useful.

No report body should be printed by default.

## 14. Inspect Boundary

`workflow-os inspect <run-id>` should expose bounded artifact metadata from the
existing local store:

- whether terminal report artifacts exist;
- report IDs;
- terminal status;
- generation timestamp;
- sensitivity;
- validation posture; and
- artifact count.

Inspect must not:

- render report section text;
- print redaction reasons;
- expose local state-root paths;
- read raw provider payloads or command output;
- mutate state; or
- imply artifact validity when deserialization or identity validation fails.

Corrupt artifacts must fail closed with stable non-leaking diagnostics.

## 15. Privacy And Redaction

The composition inherits the existing `WorkReport` and artifact validation
boundaries.

It must not store or print:

- raw provider payloads;
- raw CI or command logs;
- raw Jira or GitHub bodies;
- raw source or spec contents;
- parser payloads;
- environment variable values;
- credentials, authorization headers, tokens, or private keys;
- unbounded operator notes;
- local-check stdout/stderr; or
- secret-like approval presentation metadata.

Artifact and projection stores remain local preview stores. This phase does not
claim encryption at rest, retention policy, multi-user access control, or
regulated-data suitability.

## 16. Test Plan

Future implementation tests must cover:

1. terminal quiet completion persists one validated artifact;
2. terminal visible completion persists one validated artifact;
3. terminal denial persists its generated report when supported;
4. approval-required fresh run writes no artifact;
5. proof-enforced approval completion persists one artifact;
6. exact completed-run retry returns `already_persisted`;
7. exact approval-resume retry returns `already_persisted`;
8. conflicting duplicate artifact fails closed;
9. concurrent duplicate resolution accepts only exact equality;
10. generated timestamp and correlation are durable-run-derived;
11. workflow-authored high-assurance requirements are enforced;
12. workflow-authored proof-marker requirements are enforced;
13. missing/stale/mismatched proof projection blocks the write;
14. missing cited SideEffect blocks when required;
15. approval-linkage mismatch blocks when required;
16. artifact failure does not alter workflow status or event history;
17. report failure writes no artifact;
18. quiet output remains concise after successful persistence;
19. failure output is visible and non-leaking;
20. JSON adds stable artifact posture;
21. inspect lists bounded validated metadata;
22. inspect fails safely on corrupt artifact;
23. ordinary undeclared run/approve behavior remains unchanged;
24. no post-terminal event is appended;
25. no provider is called;
26. no report content or forbidden payload marker appears in output;
27. existing WorkReport, artifact, approval, SideEffect, executor, CLI, and
    integration tests pass; and
28. Node 20 and Node 24 integration-tooling regression coverage remains green.

## 17. Proposed Implementation Sequence

1. Add deterministic durable-run-derived authoritative report identity inputs.
2. Add an additive Core result/composition wrapper around the existing
   authoritative report result and governed artifact-write gates.
3. Compose the fresh terminal authoritative route.
4. Compose proof-enforced approval decision/resume terminal routes.
5. Add exact-idempotency and conflict handling.
6. Add bounded CLI human and JSON artifact posture.
7. Add bounded read-only artifact metadata to `inspect`.
8. Run a focused maintainer review.
9. Run one governed dogfood phase through quiet completion and one through
   approval resume, then inspect both artifacts.

Do not broaden provider, sandbox, capability, or workflow families in this
sequence.

## 18. Validation

The implementation phase should run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused artifact/executor/CLI tests;
- `cargo test --workspace`;
- `npm run check`;
- `npm run check:docs`;
- `npm run check:integrations` under the supported Node 20 baseline;
- the maintained Node 24 integration regression check; and
- `git diff --check`.

The governed phase closeout must disclose any skipped check and all work
performed outside the kernel.

## 19. Final Recommendation

Proceed next with **authoritative WorkReport artifact persistence, local and
project-controlled only**.

The first implementation should compose existing reviewed primitives into one
runtime path. It should not add OpenShell, provider expansion, a new artifact
model, another user-facing opt-in flag, hosted persistence, report export, or
broader workflow defaults.
