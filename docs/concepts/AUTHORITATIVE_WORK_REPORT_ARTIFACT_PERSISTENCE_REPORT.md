# Authoritative WorkReport Artifact Persistence Report

## 1. Executive Summary

Workflow OS now retains the validated terminal `WorkReport` produced by its
explicit and project-controlled authoritative proportional-governance paths.
The report is persisted through the existing local artifact store and existing
SideEffect, approval-linkage, high-assurance disclosure, and approval
proof-marker gates.

This closes a material quiet-success gap. Low-risk work may remain
non-interruptive, but its governed handoff is no longer discarded after CLI
output.

The implementation is local, additive, and project controlled. It does not
make artifacts universal, add a provider or sandbox, export report bodies, or
change ordinary executor behavior.

## 2. Scope Completed

- Added an explicit Core artifact-persistence composition for authoritative
  report results.
- Persisted terminal authoritative reports through reviewed artifact gates.
- Deferred report and artifact creation while a run waits for approval.
- Composed artifact persistence after proof-enforced authoritative approval
  and authored workflow approval completion.
- Added an explicit existing-terminal route for exact retry.
- Revalidated immutable run inputs and the current closed local check before
  retry reconciliation.
- Required exact durable governance-binding reproduction.
- Treated exactly equal existing artifacts as `already_persisted`.
- Failed closed on conflicting content at the same report identity.
- Reconciled concurrent equal create-only writes to one stored artifact.
- Added bounded human and JSON artifact posture.
- Added metadata-only artifact discovery to `workflow-os inspect`.
- Preserved ordinary undeclared execution without artifact persistence.

## 3. Scope Explicitly Not Completed

This phase did not add:

- artifacts for ordinary or undeclared runs;
- automatic report generation for every runtime path;
- provider execution or provider mutation expansion;
- OpenShell or another sandbox runtime;
- new SideEffect, approval, report, or artifact model families;
- hosted, remote, or shared artifact storage;
- report-body rendering, export, signing, notarization, or publication;
- post-terminal workflow events;
- terminal snapshot mutation;
- workflow schema or example changes;
- DLP, retention, access-control, or enterprise administration; or
- release posture changes.

## 4. Core And CLI Composition

`persist_authoritative_governance_report_artifact(...)` accepts an existing
authoritative run/report result, the exact workflow definition, local artifact
and SideEffect stores, and the existing approval proof-marker projection
store. It constructs a validated `WorkReportArtifactRecord`, derives
workflow-authored artifact policies, persists required projections, evaluates
all existing gates, and performs one create-only write.

The CLI invokes this composition only for the authoritative path. It reports:

- `persisted`;
- `already_persisted`;
- `deferred_non_terminal`;
- `report_unavailable`; or
- `persistence_failed`.

An authoritative artifact obligation failure returns a non-success operation
without rewriting the workflow's durable terminal result.

## 5. Deterministic Identity And Retry

The report ID remains derived from the run ID. The report generation timestamp
uses the immutable run bundle creation timestamp, correlation uses the stable
run-derived `correlation/<run-id>` value, and the generated actor remains the
bounded Workflow OS system actor.

On an existing terminal run, the authoritative route:

1. rehydrates the durable run;
2. requires terminal status;
3. validates the caller request against the stored immutable bundle;
4. reloads and validates the current project;
5. reruns the closed project-validation check;
6. recomputes the source-bound governance assessment;
7. requires exact equality with the durable binding;
8. regenerates the same validated report; and
9. reconciles only an exactly equal existing artifact.

The retry route does not execute workflow steps, deliver another disclosure,
append events, or mutate the snapshot.

## 6. Artifact Gate Summary

Every authoritative artifact write reuses existing enforcement for:

- artifact/run identity;
- cited SideEffect existence;
- SideEffect approval linkage;
- matching decisions for approved or denied SideEffects;
- workflow-authored high-assurance approval disclosure;
- approval proof-marker projection; and
- create-only local persistence.

Quiet execution affects interruption posture. It does not weaken artifact
integrity requirements.

## 7. CLI And Inspection

Completed quiet success now reports the persisted report identity and existing
inspect command. Verbose and JSON modes retain bounded route, report,
artifact, and local-check reference posture.

`workflow-os inspect <run-id>` lists validated artifact metadata including
report ID, run ID, terminal status, generation timestamp, sensitivity, and
validation posture. It does not render report sections, redaction reasons,
command output, provider payloads, or local state-root paths.

## 8. Privacy And Redaction

The composition stores only validated `WorkReport` content. Errors and debug
surfaces use stable codes and bounded posture.

It does not copy or print raw provider payloads, command or CI logs, source or
spec contents, parser payloads, environment values, credentials, tokens,
private keys, local-check stdout/stderr, or secret-like approval metadata.

The artifact store remains a sensitive local preview store. Encryption,
retention, shared access control, and regulated-data suitability are not
claimed.

## 9. Test Coverage

Focused coverage proves:

- terminal quiet completion persists one artifact;
- terminal visible completion persists one artifact;
- terminal denial persists its generated artifact before returning the bounded
  denial result;
- approval-required work writes no artifact before completion;
- proof-enforced approval resume persists the artifact;
- the persisted and rendered approval presentation explicitly authorizes only
  the exact governed terminal WorkReport artifact;
- completed-run retry returns `already_persisted`;
- approval-resume retry returns `already_persisted`;
- retry appends no workflow events;
- concurrent equal writers produce one persisted and one reconciled result;
- conflicting content at the same identity fails closed;
- quiet and verbose output remain bounded;
- JSON exposes stable artifact posture;
- inspect exposes metadata without report body content;
- ordinary runs do not persist artifacts;
- missing authoritative profiles fail before state creation; and
- error output does not echo conflicting report content.

## 10. Validation

Focused validation completed during implementation:

- `cargo fmt --all`: passed;
- `cargo check -p workflow-core`: passed;
- `cargo check -p workflow-cli`: passed;
- authoritative CLI test group: passed, 9 tests;
- completed-run artifact retry test: passed;
- approval-resume artifact retry test: passed; and
- concurrent duplicate/conflict Core test: passed.

Full phase-close validation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check`: passed, including documentation, dogfood helper,
  integration-helper, TypeScript SDK, and schema/example contracts;
- `npm run check:integrations`: passed under Node 20 and Node 24; and
- `git diff --check`: passed.

## 11. Remaining Limitations

- Full report-body inspection and export are not implemented.
- Artifact retention, encryption, and shared access controls are not
  implemented.
- Artifact persistence remains limited to the authoritative local path.
- Failed, canceled, and denied artifact behavior remains limited to terminal
  routes where the existing authoritative report consumer produces a report.
- No provider, sandbox, or external execution substrate is introduced.
- Existing runtime facts still rely on the reviewed explicit authoritative
  input boundary and closed local-check profile.

## 12. Recommended Next Phase

The initial maintainer review found approval-presentation and JSON naming
blockers. The focused blocker fix corrected both, and the separate
blocker-fix review accepted the complete authoritative WorkReport artifact
persistence phase.

After publication, return to the roadmap's runtime authority and capability
sequence rather than broadening provider mutations.

## 13. Blocker Fix

The initial maintainer review found that the authoritative approval
presentation still excluded report artifacts and persistence even though
successful approval completion now persists the exact governed terminal
WorkReport artifact. The focused fix:

- makes that exact local artifact obligation part of approved scope;
- keeps arbitrary artifacts, report export/publication, hosted persistence,
  provider writes, and scope expansion outside approval;
- includes approval proof-marker projection state and the governed terminal
  artifact in expected touched surfaces;
- aligns JSON with the planned `report_artifact_posture` and
  `report_artifact_error_code` fields; and
- adds presentation, visible-route, denied-route, and exact-key regressions.

The fix does not broaden execution or persistence beyond the original phase.
