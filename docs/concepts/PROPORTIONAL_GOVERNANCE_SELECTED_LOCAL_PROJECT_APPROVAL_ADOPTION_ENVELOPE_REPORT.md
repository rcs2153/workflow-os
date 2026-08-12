# Proportional-Governance Selected Local Project Approval Adoption Envelope Report

## 1. Executive Summary

Core now provides one explicit selected project-validation approval envelope
for both aggregate-governance and authored workflow-step approvals. The
envelope preserves approval-presentation proof, current-fact reassessment,
exact local-check evidence, authority-receipt posture, terminal WorkReport
generation, workflow-derived report-artifact gates, and truthful denial
closure without changing any CLI path or existing public approval default.

## 2. Scope Completed

The phase adds:

- `LocalSelectedProjectValidationApprovalEnvelopeInput`;
- `LocalSelectedProjectValidationApprovalEnvelopeResult`;
- `LocalSelectedProjectValidationApprovalGateKind`;
- `LocalSelectedProjectValidationArtifactGateResult`; and
- `decide_selected_project_validation_approval_envelope`.

The API is additive, explicit, local, and store-injected. Core derives gate
kind and artifact policy rather than accepting either as caller-selected
authority.

## 3. Approval And Check Behavior

Presentation proof is preflighted before decision work. A grant reruns the
canonical selected project-validation check exactly once with a fresh
Core-owned evaluation time and reconstructs resume state through the scoped
report-artifact-capable validation boundary. The returned
`LocalCheckResultReference` uses the exact Core-produced decision-time result
identifier; caller-supplied result identifiers are ignored.

Denials invoke neither the current-fact source nor the project-validation
check. They append the truthful denial outcome, generate a terminal report,
and proceed through the workflow-derived artifact gates without fabricating a
receipt or new check evidence.

## 4. Multi-Gate Posture

Core inspects the durable approval request to distinguish:

- aggregate proportional-governance approval; and
- authored workflow-step approval.

An aggregate grant that advances to an authored gate returns a transient
authority receipt in memory but persists no receipt, proof-marker projection,
WorkReport, or report artifact. Terminal authored grant or either terminal
denial performs report and artifact closure. Approval of one gate never
implies approval of the other.

## 5. Artifact And Persistence Boundary

Terminal closure recovers the exact workflow from the immutable run bundle and
derives high-assurance disclosure and approval-proof-marker requirements from
that definition. Where required, it persists approval proof-marker projections
before the existing governed artifact write. Terminal grants persist and
validate their trusted authority receipt before artifact closure. Denials
remain receipt-free.

Exact duplicate and ambiguous persistence behavior continues to use the
existing artifact and receipt helpers. The new envelope exposes bounded
posture; it does not invent a parallel store or retry authority.

## 6. Validation Boundary

The selected path carries one explicit
`ProjectValidationCapability::ReportArtifactCapable` posture through project
load, resume-plan reconstruction, and immutable-bundle reconstruction. This
capability is scoped to the selected workflow and approval-proof-marker
support. Existing executor and approval APIs continue to validate with the
default capability.

## 7. Privacy And Error Posture

Input and result `Debug` implementations expose only bounded posture and redact
approval, execution, report, and local-check reference inputs. Errors retain
stable bounded codes and do not include raw check output, report text, paths,
runtime facts, credentials, tokens, provider payloads, or caller-supplied
reference content.

## 8. Test Coverage

Focused tests cover:

- aggregate grant followed by a distinct authored grant;
- non-terminal report/artifact deferral and transient receipt posture;
- exact Core-produced check-reference identity;
- terminal authored grant with proof-marker projection and artifact gates;
- aggregate denial without check or skill execution;
- authored denial without a second check or skill execution;
- truthful receipt-free denial report artifacts; and
- redaction-safe input and result `Debug` output.

Existing selected route, report adapter, executor, receipt, WorkReport,
SideEffect, proof-marker, and artifact helpers remain exercised by the broader
workspace suite.

## 9. Scope Explicitly Not Completed

This phase does not implement:

- CLI `run` or `approve` adoption;
- new commands, flags, JSON fields, or human output;
- workflow schemas or SDK changes;
- provider or SideEffect execution;
- automatic approval or reusable approval authority;
- hidden store discovery or runtime configuration;
- hosted or distributed behavior;
- examples or scaffold-default changes;
- reasoning lineage or nested harness execution; or
- release posture changes.

## 10. Governed Phase Record

- Workflow: `dg/implement`
- Run: `run-1786494315100898000-2`
- Approval: `approval/run-1786494315100898000-2/implementation-approved`
- Presentation: `presentation/2cd78dbf6c5ace70`
- Approval outcome: granted by delegated maintainer with persisted presentation
  proof
- Presentation content hash:
  `2cd78dbf6c5ace705ef68da495f0a24cf726aef5e91f6dab52f804f260613902`
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, no retries, and no escalations
- Approval-presentation enforcement: proof enforced with the presentation
  marker present in the approval event trail

## 11. Validation Commands

Required validation passed:

- `cargo test -p workflow-core --test local_executor selected_approval_envelope --no-fail-fast`
  (3 passed)
- `cargo test -p workflow-core --test local_executor selected_project_validation_composition --no-fail-fast`
  (5 passed)
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `git diff --check`
- `npm run dogfood:benchmark -- phase-close run-1786494315100898000-2 --phase implementation`

## 12. Remaining Limitations

The envelope is not yet called by the CLI. Public output compatibility and the
combined declared `run`/`approve` cutover remain unproven until the separately
scoped adoption phase. The transient aggregate receipt remains intentionally
in-memory and is not durable evidence.

## 13. Recommended Next Phase

Perform a focused maintainer review of this selected approval adoption
envelope. If accepted, implement the declared CLI `run` and `approve` adoption
together as the compatibility-sensitive next phase.

## 14. Out-Of-Kernel Disclosure

The kernel governed phase scope, approval, and durable event history. Codex
implemented the Rust changes, tests, and documentation and ran repository
validation outside the kernel. No Workflow OS runtime state was edited by
hand.
