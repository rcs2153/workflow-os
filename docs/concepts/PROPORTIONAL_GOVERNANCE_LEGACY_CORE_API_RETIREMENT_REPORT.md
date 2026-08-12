# Proportional-Governance Legacy Core API Retirement Report

## 1. Executive Summary

The preview public caller-fact authoritative API has been retired. External
embedders can no longer construct the old request containing caller-authored
`StepGovernanceRuntimeFacts` or invoke its route, report, visible, approval,
denial, or approval-report entry points.

The selected project-validation API and the fact-free Core-owned API remain
public. Both continue to use the same private route enforcement machinery.

## 2. Scope Completed

- Removed the legacy caller-fact functions and request types from the crate
  root.
- Made the underlying explicit-fact request and route bridge crate-private.
- Removed wrappers used only by the retired external surface.
- Migrated external regression coverage to the retained Core-owned and
  selected suites.
- Preserved current-authority internal composition by importing the private
  bridge from the executor module.
- Updated the roadmap and compatibility record.

## 3. Scope Explicitly Not Completed

This phase did not change selected-path behavior, ordinary execution, CLI
output, schemas, providers, OpenShell, SideEffect execution, writes, hosted
behavior, release posture, or proportional-governance defaults.

## 4. Public API Boundary

Removed public entry points include the earlier
`route_authoritative_docs_check_governance`,
`route_authoritative_explicit_local_check_profile_governance`, direct
docs-check execution variants, caller-fact report variants, and caller-fact
approval-report variants.

The migration target is the selected project-validation family:

- `route_selected_project_validation_governance`;
- `execute_selected_project_validation_governance_report`;
- `decide_selected_project_validation_approval_envelope`; and
- `decide_selected_project_validation_approval_report_artifact`.

The fact-free Core-owned route remains available for explicit embedding where
the caller supplies validated project activation rather than authority facts.

## 5. Enforcement Preservation

Core still performs immutable-bundle binding, same-call check execution,
current authority binding, proportional-governance assessment, disclosure
delivery, approval pause/resume, denial, drift rejection, report composition,
and artifact gates. Retirement changes who may supply route-driving facts; it
does not weaken the route machinery.

## 6. Compatibility Posture

This is an intentional preview compatibility correction in
`0.2.0-preview.1`. Repository inspection found no production consumer of the
removed functions. Historical phase documents remain unchanged as records of
the APIs that existed when those phases were completed.

## 7. Test Coverage

The retained integration suite covers:

- Core-owned quiet and visible routes;
- missing disclosure capability;
- Core-owned approval reassessment;
- invalid multi-step shape rejection;
- same-call check failure;
- selected terminal and non-terminal report behavior;
- terminal reassessment and drift rejection;
- report preflight and redaction;
- selected approval grants, denials, envelopes, and artifact decisions; and
- ordinary executor behavior across the workspace suite.

Historical tests that constructed the retired external request were removed
with that request. Equivalent supported-path coverage remains on the
Core-owned or selected project-validation APIs; the phase did not retain a
test-only public compatibility surface.

## 8. Privacy And Failure Posture

No payload, error, serialization, or Debug behavior changed. Unsupported
external use now fails at compile time rather than reaching a runtime path
that accepts weaker authority provenance.

## 9. Governed Implementation Record

- Workflow: `dg/implement`
- Run: `run-1786538708341183000-2`
- Approval: `approval/run-1786538708341183000-2/implementation-approved`
- Presentation: `presentation/1f8aa4898eba90cb`
- Approval outcome: granted by delegated maintainer through proof-enforced
  approval
- Kernel boundary: the kernel governed scope, approval, and events; code
  inspection, edits, checks, and Git work remained outside the kernel

## 10. Validation

The final local matrix passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test -p workflow-core --test local_executor` with 338 passed and 1
  ignored opt-in live test;
- `cargo test --workspace`, including all Core, CLI, hosted, adapter,
  provider-write, report, evidence, state, OpenShell, schema, and SDK-facing
  Rust suites;
- `npm run check:docs`;
- `npm run check:integrations`;
- `npm run check`, including 32 dogfood-helper tests, 3 integration-helper
  tests, 11 TypeScript SDK tests, TypeScript typecheck/lint, and contract
  checks; and
- `git diff --check`.

Repository-wide export and call-site inspection also confirmed that the
retired request and route family are absent from the crate root and have no
remaining production callers.

## 11. Remaining Limitations

- The selected consumer currently covers the closed local project-validation
  profile rather than arbitrary check families.
- A future stable embedding API must consume registered current sources or
  scoped capability grants, not caller-authored authority facts.
- This phase does not make proportional governance a generic runtime default.

## 12. Recommended Next Phase

Run a focused maintainer review of this retirement. If accepted, resume
runtime composition from the roadmap rather than adding another compatibility
layer.
