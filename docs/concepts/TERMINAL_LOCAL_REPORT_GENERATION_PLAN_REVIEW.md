# Terminal Local Report Generation Plan Review

Review date: 2026-06-05

## 1. Executive Verdict

Plan accepted with non-blocking follow-ups.

The terminal local report generation plan defines a conservative, terminal-only, local, deterministic, in-memory generation boundary. It preserves current workflow semantics, avoids persistence and CLI exposure, uses existing `WorkReport` constructors, and keeps report artifacts, schemas, examples, reasoning lineage, side-effect modeling, writes, and release posture changes out of scope.

There are no planning blockers before a scoped implementation of an in-memory terminal local report generation service/helper.

## 2. Scope Verification

The plan stayed within planning-only scope.

It does not authorize:

- runtime report generation implementation in this phase;
- report artifacts;
- persistence;
- CLI behavior;
- example updates;
- workflow spec schema changes;
- reasoning lineage implementation;
- side-effect boundary;
- writes;
- approval evidence attachment;
- release posture changes.

The plan explicitly recommends a future implementation phase and keeps the current task as documentation-only planning.

## 3. Generation Boundary Assessment

The plan defines a conservative v1 generation boundary.

Verified:

- terminal-only generation is recommended;
- completed, failed, canceled, escalated, and blocked report statuses are considered;
- the current runtime distinction is documented: only completed, failed, and canceled are terminal under `WorkflowRunStatus::is_terminal()`;
- escalated is currently non-terminal in the runtime projection;
- blocked exists as report vocabulary but not as a runtime terminal state;
- mid-run reports are deferred;
- generation is local and deterministic;
- generation does not imply persistence or CLI rendering;
- generation does not change workflow semantics prematurely.

The recommended boundary is appropriate: an internal helper/service that consumes an already terminal `WorkflowRun` and returns an in-memory `WorkReport`.

## 4. Report Contract Source Assessment

The plan considers:

- built-in/default contract;
- explicitly declared workflow contract;
- runtime configuration;
- test-only contract.

It correctly defers workflow schema changes and recommends the smallest first source: a built-in/default v1 contract supplied directly to the generation helper/service. This is small, testable, and keeps future workflow-declared contracts possible.

Non-blocking clarification for implementation: because there is no current runtime report config surface, the first helper should likely accept the contract explicitly rather than inventing runtime config.

## 5. Runtime Integration Assessment

The plan accounts for the current runtime shape:

- `LocalExecutor::execute` returns `WorkflowRun`;
- `WorkflowRun` contains snapshot and event history;
- `WorkflowRunSnapshot` carries immutable run identity, status, skill invocations, approval requests, retries, escalations, cancellation, and failure state;
- runtime events carry event IDs, sequence numbers, timestamps, immutable identity, actor/correlation metadata where relevant, and state transition details;
- validation diagnostics exist before execution through loader/validator surfaces;
- adapter telemetry can be emitted from controlled read-only fixture handlers and stored through adapter telemetry stores;
- policy decisions are represented in events and durable pre-run policy audit records;
- approval requests and decisions are modeled, while approval evidence attachment remains unimplemented.

The smallest future integration point is appropriate: add an internal report-generation helper/service that accepts terminal run state and explicit context, returns an in-memory report, and does not append events, write files, persist records, or expose CLI behavior.

## 6. Input Boundary Assessment

Allowed inputs are appropriately limited to stable identity values, bounded summaries, and references:

- workflow identity;
- workflow version;
- schema version;
- spec hash;
- run ID;
- terminal status;
- generated timestamp;
- generated actor/system actor;
- correlation ID;
- validation diagnostics;
- `EvidenceReference` IDs;
- audit event IDs;
- adapter telemetry record IDs or stable references;
- policy decision IDs or policy event IDs;
- approval decision IDs where available;
- bounded handoff notes where available.

Forbidden inputs remain excluded:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira/GitHub bodies;
- raw spec contents;
- raw parser payloads;
- environment variable values;
- credentials, secrets, and tokens.

The input boundary is suitable for a first in-memory implementation.

## 7. Citation Policy Assessment

The plan defines a useful citation policy:

- citations come from existing stable IDs or bounded stable references;
- mandatory identity and core sections are distinguished from best-effort citations;
- missing citations are explicit through `WorkReportCitation::missing`;
- citation summaries must be bounded and redacted;
- `EvidenceReference` citations use existing IDs without recreating evidence;
- validation diagnostics should be cited by stable `ValidationReferenceId` where available;
- adapter telemetry should use stable references until a dedicated telemetry ID exists;
- approval/policy citation vocabulary is allowed only where stable IDs exist;
- citation failure must not fabricate evidence.

This is aligned with the EvidenceReference source-of-truth boundary.

## 8. Section Population Assessment

The plan defines population rules for all v1 sections:

- work performed;
- evidence considered;
- decisions made;
- policy gates evaluated;
- approvals;
- validation and quality checks;
- side effects;
- incomplete or deferred work;
- known limitations;
- risks;
- operator handoff notes.

For each section, the plan identifies likely data source, required status, empty/none/skipped/not-available behavior, and forbidden copied payloads. It also requires all v1 sections to be present, which matches the current `WorkReport` model validation.

The section policy is appropriately conservative. It allows explicit “none” or “not available” text instead of pretending citations or data exist.

## 9. Workflow Semantics And Failure Behavior Assessment

The plan is explicit about failure behavior:

- report generation should not change workflow pass/fail semantics;
- report generation should not mutate terminal run status;
- report generation should not append post-terminal workflow events;
- report generation failure should not retroactively fail a successfully completed workflow;
- construction failure should return a structured report-generation error from the helper/service;
- report construction errors should not become misleading user project diagnostics;
- errors must remain stable and non-leaking.

This is the correct v1 posture. It avoids treating report generation as part of the workflow outcome before the runtime semantics are designed.

## 10. Storage And Artifact Posture Assessment

The storage/artifact boundary is clear.

Verified:

- no persistence is authorized;
- no filesystem artifacts are authorized;
- no CLI rendering is authorized;
- no export command is authorized;
- no schema exposure is authorized;
- later artifact-phase requirements are deferred and called out.

The plan correctly separates in-memory report construction from artifact generation.

## 11. Privacy/Redaction Assessment

The plan preserves the WorkReport privacy posture.

Verified:

- report generation must use `WorkReport` constructors;
- report generation must use validated `WorkReportCitation` constructors;
- WorkReport redaction metadata validation is respected;
- summaries are bounded;
- raw payloads are forbidden;
- secret-like metadata is forbidden;
- file paths are treated conservatively;
- Debug/serialization safety is preserved;
- reports are treated as sensitive even when citations are read-only.

This is aligned with the WorkReport redaction metadata blocker fix.

## 12. Contract Enforcement Assessment

The plan scopes contract enforcement appropriately.

Verified:

- v1 generation should use a built-in/default contract;
- generated reports should validate against current model requirements;
- missing required sections should fail construction;
- missing optional citations should become explicit missing citations or incomplete-work disclosures;
- future workflow-declared contracts are deferred;
- schema exposure remains deferred.

Non-blocking clarification for implementation: the helper should define what “validates against `WorkReportContract::v1`” means beyond the current report model’s required-section validation, since the model does not yet enforce a specific contract instance.

## 13. Test Plan Assessment

The future test plan covers the important behaviors:

- terminal completed run report;
- failed, canceled, escalated, and blocked statuses where runtime context supports them;
- use of model constructors;
- workflow result semantic preservation;
- explicit missing citations;
- invalid report construction failure;
- no raw provider/spec/command/parser payload copying;
- validation diagnostic citation by stable reference;
- adapter telemetry citation by stable reference;
- `EvidenceReference` ID citation without recreation;
- side-effects section as none/skipped/unsupported;
- Debug and serialization non-leakage;
- no persistence or CLI artifacts;
- existing runtime tests.

Non-blocking additions:

- add tests proving no post-terminal workflow event is appended;
- add tests proving the helper can be used without a `StateBackend` write;
- add tests for unavailable validation diagnostics and unavailable adapter telemetry producing explicit not-available/missing content rather than empty silence.

## 14. Documentation Review

Documentation is aligned.

Verified docs state:

- terminal local report generation is planned, not implemented;
- `WorkReport` and `WorkReportContract` models exist;
- runtime report generation is not implemented;
- report artifacts are not implemented;
- persistence is not implemented;
- CLI rendering is not implemented;
- examples are not updated;
- workflow spec schema changes are not implemented;
- reasoning lineage is not implemented;
- side-effect boundary is not implemented;
- writes remain unsupported.

No dangerous public-facing overclaim was found.

## 15. Planning Blockers

None.

## 16. Non-Blocking Follow-Ups

- In the implementation prompt, prefer an explicit helper/service input contract over a new runtime config surface.
- Define what built-in/default contract enforcement means beyond current model validation.
- Add tests proving report generation does not append post-terminal events.
- Add tests proving report generation creates no persistence, files, or CLI output.
- Clarify whether escalated and blocked reports are only model-vocabulary tests until runtime terminal status support changes.
- Decide later whether adapter telemetry needs a dedicated stable ID before persistence or schema exposure.

## 17. Recommended Next Phase

Recommended next phase: terminal local report generation service/helper implementation, in-memory only.

The plan is precise enough to drive a small implementation. The next phase should create a local in-memory helper/service that returns `WorkReport` values from terminal run inputs and explicit context. It must not add persistence, CLI rendering, examples, schemas, reasoning lineage, side-effect modeling, writes, approval evidence attachment, report artifacts, or release posture changes.

## Validation

| Command | Result |
| --- | --- |
| `npm run check:docs` | Passed |
