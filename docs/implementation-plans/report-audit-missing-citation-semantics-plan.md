# Report, Audit, And Missing-Citation Semantics Plan

Status: Planning only. This plan does not implement report generation changes, runtime events, audit events, artifact writing, persistence, CLI behavior, schemas, examples, reasoning lineage, side-effect modeling, writes, or release posture changes.

## 1. Executive Summary

Workflow OS now has WorkReport models, in-memory report generation helpers, executor-integrated report-bearing execution, and local report artifact APIs.

The next semantics question is how reports relate to audit/runtime events, how report-generation failures relate to workflow results, and when unavailable references should become explicit missing citations.

This plan preserves the current conservative direction: reports are derived governed handoff artifacts, not audit events; report-generation errors remain separate from workflow execution errors; absent optional references remain explicit section text until contract-driven citation slots exist.

## 2. Goals

- Clarify source-of-truth boundaries.
- Preserve workflow pass/fail semantics.
- Keep reports citeable without turning them into audit events.
- Prevent fake or fabricated citation IDs.
- Define when missing citations are appropriate.
- Prepare focused regression tests.

## 3. Non-Goals

This plan does not authorize:

- automatic report generation for every run;
- report-created runtime events;
- report-generation audit events;
- post-terminal event appends;
- automatic artifact writing from executor paths;
- CLI rendering or export;
- schema changes;
- workflow-declared report contracts;
- automatic citation discovery from stores;
- fabricated `EvidenceReference` or citation IDs;
- command-output evidence;
- approval evidence attachment;
- reasoning lineage;
- side-effect or write modeling.

## 4. Source-Of-Truth Boundaries

- Workflow events are the source of truth for run state.
- Audit events are operational governance records.
- EvidenceReference values are citation pointers.
- WorkReports are derived governed handoff artifacts.
- Report artifacts are stored report records, not event-log replacements.

Generated reports may cite audit events, workflow events, adapter telemetry, validation diagnostics, evidence references, local check results, policy decisions, and approval decisions by stable reference.

Reports should not become audit records, and audit records should not become narrative reports.

## 5. Report-Generation Failure Semantics

If execution fails before a run exists, the execution error should be returned unchanged.

If execution produces a run and report generation fails, the run should be preserved and report failure should be exposed separately. The report failure must not:

- mutate run status;
- append workflow events;
- emit audit events;
- create a project diagnostic;
- write artifacts automatically;
- change workflow pass/fail semantics.

## 6. Missing Reference Policy

Absent optional references should remain explicit section text such as `not available`, `none`, or `unsupported in this build`.

`WorkReportCitation::missing` should be reserved for a later phase where there is a known required citation slot with a stable target category and no fabricated ID.

The current helper paths should not create missing citations for absent optional references.

## 7. Model And API Options

Recommended now:

- document the current semantics;
- add focused regression tests;
- do not change public model shape.

Possible later:

- add a typed `MissingCitationReason`;
- add contract-enforced required citation slots;
- expose missing-citation records only when target category and requirement are stable.

Rejected now:

- placeholder citation targets;
- fabricated event or evidence IDs;
- report-generation audit events without separate planning.

## 8. Privacy And Error Rules

Report and citation errors must use stable codes and must not leak:

- raw provider payloads;
- raw command output;
- raw spec contents;
- raw parser payloads;
- environment values;
- paths, tokens, credentials, authorization headers, or private keys;
- unbounded report section text.

## 9. Test Plan

Future tests should cover:

- report-generation failure preserves workflow run and events;
- no report-generation workflow event is emitted;
- no report-generation audit event is emitted;
- no artifact is written automatically;
- absent evidence/audit/adapter references produce section text;
- absent validation/local-check references produce section text;
- terminal helper does not generate `missing=true` citations for absent optional references;
- supplied audit event IDs become report citations;
- report errors are redaction-safe.

## 10. Proposed Implementation Sequence

1. Add this semantics plan.
2. Add regression tests around current behavior.
3. Review.
4. Defer model changes until workflow-declared report contracts or required citation slots exist.

## 11. Final Recommendation

The next implementation slice should be a small semantics hardening pass: docs plus tests only, no public model changes unless tests reveal unavoidable ambiguity.
