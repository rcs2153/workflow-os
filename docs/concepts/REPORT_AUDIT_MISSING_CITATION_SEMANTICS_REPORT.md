# Report, Audit, And Missing-Citation Semantics Report

## 1. Executive Summary

The report/audit/missing-citation semantics hardening phase is complete.

This phase kept the implementation narrow: it added regression coverage and documentation for the current conservative semantics. Reports remain derived governed handoff artifacts, not workflow events or audit events. Report-generation failures remain separate from workflow execution results. Absent optional references remain explicit section text rather than fabricated missing citations.

No public model shape changes were introduced.

## 2. Scope Completed

- Confirmed report-generation failure preserves the completed workflow run and event history.
- Added regression coverage that report-generation failure does not emit extra report audit or observability signals.
- Added regression coverage that supplied audit event IDs become report citations.
- Added regression coverage that absent optional references remain section text and do not create `missing=true` citations.
- Updated roadmap and planning docs to state the implemented semantics.
- Preserved the current local, explicit, in-memory report posture.

## 3. Scope Explicitly Not Completed

This phase did not implement:

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
- side-effect or write modeling;
- release posture changes.

## 4. Source-Of-Truth Boundary Summary

The phase preserves the existing boundary:

- workflow events remain the source of truth for run state;
- audit events remain operational governance records;
- `EvidenceReference` values remain citation pointers;
- `WorkReport` values remain derived governed handoff artifacts;
- report artifacts remain stored report records, not event-log replacements.

Reports may cite audit events, workflow events, adapter telemetry, validation diagnostics, evidence references, local check results, policy decisions, approval decisions, and typed handoffs by stable reference. Reports do not become audit records, and audit records do not become narrative reports.

## 5. Missing-Citation Policy Summary

Absent optional references continue to be represented as explicit bounded section text such as `none`, `not available`, or `unsupported in this build`.

`WorkReportCitation::missing` remains reserved for a later contract-driven phase where a required citation slot exists, the target category is stable, and no fabricated ID should be created.

## 6. Audit And Observability Summary

Report-generation failure remains separate from workflow execution. When execution produces a run and report generation fails:

- the run status is preserved;
- workflow events are preserved;
- no report artifact is written automatically;
- no report-specific audit event is emitted;
- no report-specific observability event is emitted;
- the structured report-generation error is exposed separately.

## 7. Test Coverage Summary

Focused regression tests now cover:

- absent optional references remain section text;
- absent optional references do not generate missing citations;
- supplied audit event IDs become `AuditEvent` report citations;
- report-generation failure preserves the run;
- report-generation failure preserves runtime events;
- report-generation failure does not emit extra report audit or observability records;
- report-generation errors remain redaction-safe.

Existing executor report tests continue to cover successful completed and failed reports, non-terminal report errors, typed handoff citation forwarding, artifact non-writing, and debug redaction.

## 8. Commands Run And Results

- `cargo test -p workflow-core --test local_executor execute_with_report` - passed.
- `cargo test -p workflow-core --test local_executor report_generation_failure_emits_no_report_audit_or_observability_events` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- Missing-citation records are not contract-enforced yet.
- Workflow-declared report contracts are not implemented.
- Report-generation audit events are intentionally not implemented.
- Automatic report artifact writing from executor paths is not implemented.
- Approval/cancellation report-bearing executor methods are not implemented.
- Side-effect boundary modeling and writes remain unsupported.

## 10. Recommended Next Phase

Recommended next phase: report/audit/missing-citation semantics review.

The review should verify that the hardening stayed within docs-and-tests scope, that no public model changes were introduced, and that the semantics remain suitable before side-effect boundary work or any contract-driven missing-citation model is considered.
