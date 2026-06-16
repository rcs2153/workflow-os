# Report, Audit, And Missing-Citation Semantics Review

## 1. Executive Verdict

Phase accepted; proceed to governed multi-step workflow execution ADR and planning.

The hardening phase stayed within the approved docs-and-tests boundary. It clarified report/audit/missing-citation semantics without changing public model shape, runtime report-generation behavior, persistence, CLI output, schemas, examples, reasoning lineage, side-effect modeling, writes, or release posture.

## 2. Scope Verification

The phase remained scoped to regression tests and documentation.

No accidental implementation was introduced for:

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

## 3. Semantics Assessment

The phase correctly preserves the current source-of-truth boundary:

- workflow events remain the source of truth for run state;
- audit events remain operational governance records;
- `EvidenceReference` values remain citation pointers;
- `WorkReport` values remain derived governed handoff artifacts;
- report artifacts remain stored report records, not event-log replacements.

The updated docs correctly state that generated reports may cite audit events without becoming audit events themselves.

## 4. Missing-Citation Assessment

The current policy is appropriate for the implemented model:

- absent optional references remain explicit bounded section text;
- optional missing references do not create `missing=true` citations;
- `WorkReportCitation::missing` remains reserved for a later contract-driven phase with required citation slots.

This avoids fake evidence and avoids overloading the citation model before workflow-declared report contracts exist.

## 5. Audit And Observability Assessment

The phase adds focused coverage that report-generation failure after a run exists:

- preserves run status;
- preserves workflow events;
- does not write report artifacts automatically;
- does not emit extra report audit records;
- does not emit report-specific observability records;
- exposes the report-generation error separately.

The observability regression checks that all observability events continue to trace to existing workflow events, which is the right boundary for this phase.

## 6. Test Quality Assessment

The added tests are focused and behavioral:

- absent optional refs produce not-available section text;
- absent optional refs do not create missing citations;
- supplied audit event IDs become `AuditEvent` citations;
- report-generation failure preserves workflow run/events;
- report-generation failure does not add report-specific audit/observability records.

Existing executor report tests still cover completed, failed, non-terminal, typed handoff, artifact non-writing, and redaction-safe result behavior.

## 7. Documentation Review

Documentation now states:

- report/audit/missing-citation semantics are hardened;
- reports are derived handoff artifacts, not audit events;
- report-generation failures are separate from workflow execution errors;
- absent optional references remain section text;
- automatic report generation, automatic artifact writing, CLI rendering, schemas, examples, reasoning lineage, side effects, and writes remain unimplemented.

The previous tension in the terminal report generation plan around `WorkReportCitation::missing` was corrected.

## 8. Validation

Validated by the implementation phase:

- `cargo test -p workflow-core --test local_executor execute_with_report` - passed.
- `cargo test -p workflow-core --test local_executor report_generation_failure_emits_no_report_audit_or_observability_events` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Blockers

No blockers remain for the report/audit/missing-citation semantics hardening phase.

## 10. Non-Blocking Follow-Ups

- Revisit `WorkReportCitation::missing` only when workflow-declared report contracts or required citation slots exist.
- Keep report-generation audit events deferred unless a separate ADR explains why they are operationally necessary.
- Keep automatic artifact writing from executor paths deferred.

## 11. P0 Pivot Note

Kernel dogfooding produced a higher-priority blocker: governed multi-step workflows.

The current local executor still supports exactly one step. That prevents Workflow OS from governing realistic work at scale, where value comes from a sequence of governed checks, validations, handoffs, and terminal reporting. This should now supersede lower-priority report polish.

## 12. Recommended Next Phase

Recommended next phase: governed multi-step workflow execution ADR and implementation planning.

The next phase should define deterministic sequential multi-step execution before nested harness execution, write-capable adapters, side-effect modeling, broad report automation, or reasoning lineage implementation.
