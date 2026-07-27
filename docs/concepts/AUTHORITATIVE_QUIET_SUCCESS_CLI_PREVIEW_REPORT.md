# Authoritative Quiet-Success CLI Preview Report

## 1. Executive Summary

The explicit authoritative quiet-success CLI preview is implemented.

`workflow-os run <workflow-id> --authoritative-governance` now composes the
accepted closed project-validation profile, immutable run bundle, Core-derived
proportional-governance route, bounded disclosure, approval presentation
proof, payload-free local-check reference, and in-memory terminal WorkReport.
The matching `approve ... --authoritative-governance` path resumes governed
approval gates without changing ordinary CLI behavior.

## 2. Scope Completed

- Added explicit authoritative flags to `run` and `approve`.
- Kept route selection inside the accepted Core dispatcher.
- Rendered bounded quiet, visible, approval-required, and denied outcomes.
- Persisted and rendered complete approval presentations.
- Revalidated the closed project-validation profile at approval time.
- Produced payload-free local-check references and in-memory terminal reports.
- Rejected missing, ambiguous, unsupported, or caller-selected check profiles
  before execution.
- Added bounded experimental JSON output.

## 3. Scope Explicitly Not Completed

- No default proportional-governance execution.
- No automatic approval.
- No arbitrary commands, shell strings, or repository command discovery.
- No runtime or workflow schema configuration.
- No report artifacts, report persistence, or report export.
- No provider integration, OpenShell integration, SideEffect execution, or
  writes.
- No scaffold, example, hosted-runtime, enterprise-control, or release change.

## 4. CLI Contract

The preview surfaces are:

```text
workflow-os run <workflow-id> --authoritative-governance
workflow-os approve <run-id> <approval-id> --authoritative-governance \
  --actor <actor> --reason <bounded-reason>
```

The flag selects one closed execution profile. It does not accept executable
paths, arguments, shell text, route choices, check outcomes, or approval
outcomes as ambient authority.

## 5. Route Behavior

- Quiet proceed returns concise completion, quiet disclosure, check-reference,
  report, and inspect posture.
- Visible proceed requires bounded disclosure delivery but no approval.
- Approval required persists and prints the complete approval handoff.
- Denial remains a terminal governed outcome with a stable CLI error code.
- Report-generation failure remains separate from durable workflow status.

## 6. Chained Approval Behavior

Aggregate proportional-governance approval and authored workflow or step
approval are distinct subjects.

Granting an aggregate approval does not satisfy a later authored gate. When a
later gate appears, the CLI:

1. leaves the run in `WaitingForApproval`;
2. persists a new presentation bound to the new approval ID;
3. renders the complete new handoff and exact next command; and
4. requires a separate proof-enforced decision.

This behavior was discovered by the focused CLI test and retained as a product
integrity rule.

## 7. Validation And Evidence Boundary

The resolved explicit profile exposes one narrow execute-once method for its
already fixed command contract. It cannot discover or accept a command.

Fresh routes cite the actual same-call result. Aggregate approval resume cites
the exact decision-time result from the accepted Core helper. A later authored
approval executes the same closed profile once, validates it before mutation,
constructs a payload-free result reference, and generates the report from the
terminal durable run.

## 8. Privacy And Redaction

Human and JSON output exclude raw check output, command transcripts, report
section bodies, paths, environment values, policy payloads, and provider data.
Errors use stable non-leaking codes. Debug behavior remains bounded through the
existing Core types.

## 9. Test Coverage

Focused CLI coverage includes:

- quiet, visible, approval-required, and denied routes;
- complete persisted approval handoff;
- separate aggregate and authored approval decisions;
- terminal in-memory report completion;
- closed-profile failure before run creation;
- bounded JSON output and payload non-leakage; and
- rejection of caller-selected command and route inputs.

Existing Core coverage remains authoritative for route derivation, immutable
bindings, approval reassessment, report generation, and disclosure delivery.

## 10. Validation Commands

All required checks passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- focused authoritative CLI suite: 7 passed

The workspace suite passed with only its explicitly opt-in live tests ignored.

## 11. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1785111273107714000-2`
- approval: `approval/run-1785111273107714000-2/implementation-approved`
- presentation: `presentation/1edffa6de5b6725e`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- focused validation: authoritative CLI tests passed
- out-of-kernel work: code and documentation edits, command execution, test
  interpretation, and report authoring

## 12. Remaining Limitations

- The preview supports only the fixed Workflow OS project-validation profile.
- It is explicit and experimental, not profile-controlled or automatic.
- In-memory WorkReports are not separately inspectable after process exit.
- No sandbox or provider execution substrate is integrated.
- CLI route output is intentionally bounded rather than a full operator UI.

## 13. Recommended Next Phase

Perform a phase-level maintainer review of the authoritative quiet-success CLI
preview. Do not broaden defaults, command families, providers, artifacts,
writes, schemas, or hosted behavior before that review.

## 14. Review Fix-Forward

Phase review found that the human approval renderer duplicated the persisted
presentation text. Although the values matched, later drift could have made the
displayed scope differ from the record used for proof enforcement.

The renderer now emits the persisted record's requested action, work summary,
scope, non-goals, touched surfaces, validation expectations, why-now context,
next action, presentation ID, and content hash directly. Focused regression
coverage verifies the new proof context.
