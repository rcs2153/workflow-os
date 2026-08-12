# Proportional-Governance Legacy Core API Compatibility Review

## 1. Executive Verdict

Compatibility cleanup is required before additional proportional-governance
consumer broadening.

The selected project-validation `run` and `approve` path is now the canonical
product path. The older public `DocsCheck` route family remains useful as
crate-internal composition and regression coverage, but its public request
surface accepts caller-authored runtime facts and can operate without the
validated project activation that the selected path requires. Keeping that
family publicly re-exported preserves an alternate authority model after the
CLI cutover.

## 2. Review Scope

This review inspected:

- public exports from `workflow-core`;
- request and route construction in `executor.rs`;
- all repository call sites outside the implementation module;
- focused executor tests;
- the accepted selected-consumer and CLI-adoption plans, reports, and reviews;
- current compatibility and release posture.

The review did not delete APIs, change runtime or CLI behavior, add schemas,
call providers, integrate OpenShell, execute SideEffects, add writes, broaden
hosted behavior, or change release posture.

## 3. Current Public Surface

The crate root still publicly re-exports the earlier explicit-fact route and
report functions, including:

- `route_authoritative_docs_check_governance`;
- `route_authoritative_explicit_local_check_profile_governance`;
- `execute_with_authoritative_docs_check_governance`;
- `execute_with_authoritative_docs_check_visible_governance`;
- `execute_with_authoritative_docs_check_approval_governance`;
- `execute_with_authoritative_docs_check_denied_governance`;
- `execute_with_authoritative_docs_check_governance_report`; and
- `execute_with_authoritative_explicit_local_check_profile_governance_report`.

Their public request type,
`LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest`, accepts a vector
of caller-authored `StepGovernanceRuntimeFacts`. It also allows
`project_authoritative_execution` to be absent. Tests intentionally exercise
that historical explicit-fact shape by supplying authority, evidence/check,
SideEffect, execution-disposition, and disclosure facts.

## 4. Selected Consumer Boundary

The selected project-validation consumer is materially stronger rather than a
renamed convenience API:

- validated project activation selects the closed profile;
- immutable run input binds that activation;
- Core owns the fixed runtime-fact source;
- Core selects fresh evaluation time for every decision call;
- callers cannot preclassify authority;
- the canonical local check is executed and cited in the same call;
- approval presentation proof precedes grant reassessment; and
- terminal closure preserves authority receipt and report-artifact gates.

The CLI now consumes this selected route and approval envelope. Ordinary
undeclared workflows remain on the ordinary executor rather than the older
explicit-fact authoritative surface.

## 5. Authority Assessment

The older APIs bind supplied facts into a deterministic assessment and retain
valuable route-specific enforcement. They do not let callers select a route
enum directly. However, binding caller-authored facts proves consistency, not
current authority.

When project activation is absent, a caller can supply `Sufficient` authority
and other route-driving facts. That was an explicit experimental embedding
contract while the source-backed path was being built. It is not the accepted
product authority boundary now that Core owns the selected source.

The old family therefore must not remain a public authority-bearing product
surface merely for compatibility. Approval and deterministic assessment do
not turn a caller assertion into current authority.

## 6. Call-Site And Compatibility Assessment

Repository call-site inspection found:

- production CLI use only of the selected project-validation run/report and
  approval-envelope functions;
- no other production crate consumer of the older public route functions;
- focused `workflow-core` tests as the only Rust call sites outside
  `executor.rs`; and
- historical plans and reports that document the earlier experimental APIs.

The repository is still `0.2.0-preview.1`, and the affected APIs are explicitly
experimental. Removing public re-exports is therefore an acceptable preview
compatibility correction if it includes migration notes and preserves the
canonical selected path. Historical phase records should not be rewritten.

## 7. Required Cleanup Boundary

The next implementation should:

1. remove the older explicit-fact route and report functions and request types
   from the crate-root public export surface;
2. make those implementation primitives crate-private where the selected
   consumer still needs them;
3. keep exact route enforcement and focused regression coverage inside Core;
4. retain selected project-validation functions and types as the supported
   public local consumer;
5. retain ordinary undeclared executor behavior;
6. document the preview migration boundary; and
7. prove that no CLI, event, artifact, approval, retry, privacy, or ordinary
   workflow behavior changes.

The implementation should not delete useful private route consumers merely to
reduce line count. The problem is public authority provenance, not internal
defense-in-depth.

## 8. Failure And Privacy Assessment

The cleanup itself should not add a runtime fallback or new failure path.
Unsupported external callers should receive a compile-time API removal rather
than a runtime route that accepts weaker authority provenance.

Existing stable runtime errors, bounded Debug output, redaction behavior, and
non-leaking report/approval failures remain unchanged for supported paths.

## 9. Test Requirements

The implementation phase must prove:

- the old functions and request types are not re-exported publicly;
- internal route-specific quiet, visible, approval, and denial enforcement
  remains covered;
- the selected run/report and approval-envelope APIs remain public;
- declared quiet, visible, approval-required, denied, retry, report, artifact,
  and approval paths remain behaviorally compatible;
- ordinary undeclared workflows remain unchanged;
- caller-preclassified authority cannot enter the selected consumer;
- current CLI human and JSON contracts remain unchanged; and
- workspace, docs, integration, schema, and SDK checks pass where applicable.

## 10. Blockers

None for the bounded cleanup implementation.

## 11. Non-Blocking Follow-Ups

- A future stable public embedding API should accept registered current-fact
  sources or scoped capability grants, not arbitrary authority facts.
- Additional local-check profile families require separate source,
  attestation, and compatibility review.
- OpenShell remains an optional execution substrate and must not become an
  authority source.

## 12. Recommended Next Phase

Implement the legacy Core API public-surface retirement as one bounded preview
compatibility phase. Preserve the selected project-validation consumer, the
private route machinery, and all current CLI behavior.

Do not broaden providers, OpenShell, SideEffect execution, writes, schemas,
hosted behavior, or proportional-governance defaults in that phase.

## 13. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1786538493217671000-2`
- Approval: `approval/run-1786538493217671000-2/review-scope-approved`
- Presentation: `presentation/af707a9538b1244c`
- Approval outcome: granted by the delegated maintainer with persisted
  presentation proof
- Phase status: completed
- Out-of-kernel work: source inspection, compatibility judgment,
  documentation editing, validation, and later Git/PR work
- Kernel boundary: the kernel governed scope, approval, and durable events; it
  did not inspect code, edit files, run checks, or perform Git actions

## 14. Validation

Required review validation:

- `npm run check:docs`;
- `git diff --check`; and
- repository-wide call-site inspection for the affected public exports.

## 15. Fix-Forward Status

The bounded retirement was implemented and accepted in
[Proportional-Governance Legacy Core API Retirement Report](PROPORTIONAL_GOVERNANCE_LEGACY_CORE_API_RETIREMENT_REPORT.md)
and
[Proportional-Governance Legacy Core API Retirement Review](PROPORTIONAL_GOVERNANCE_LEGACY_CORE_API_RETIREMENT_REVIEW.md).
This note preserves the pre-implementation findings above while recording
that the alternate public authority-bearing surface is no longer present.
