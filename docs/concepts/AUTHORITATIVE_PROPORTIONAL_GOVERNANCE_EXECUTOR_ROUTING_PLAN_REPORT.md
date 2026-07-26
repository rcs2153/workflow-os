# Authoritative Proportional-Governance Executor Routing Plan Report

## 1. Executive Summary

The next proportional-governance runtime-composition boundary is now planned.
The plan routes complete, source-bound assessments across quiet proceed,
visible proceed, approval-required, and denied outcomes without conflating
operator disclosure with human authorization.

This phase changed documentation only. No runtime routing behavior was added.
Focused review accepts the plan with implementation constraints in
[Authoritative Proportional-Governance Executor Routing Plan Review](AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_EXECUTOR_ROUTING_PLAN_REVIEW.md).

## 2. Scope Completed

- Defined the authoritative routing matrix.
- Preserved the accepted complete quiet `Proceed` path as the compatibility
  baseline.
- Defined visible disclosure as an explicit delivery obligation rather than an
  execution mode.
- Defined reuse of existing approval, presentation-proof, and resolved-context
  enforcement for approval-required outcomes.
- Defined fail-closed denial and incomplete-assessment behavior.
- Defined source-of-truth, ordering, privacy, error, test, and review
  requirements.
- Positioned optional OpenShell execution-provider work after authoritative
  routing and scoped capability authority.
- Corrected roadmap and quiet-success sequence status.

## 3. Scope Explicitly Not Completed

The phase did not add Rust, runtime routing, disclosure delivery, approval
creation, denial events, CLI/UI behavior, schemas, providers, OpenShell,
SideEffect execution, writes, report artifacts, automatic approvals, hosted
behavior, enterprise administration, reasoning lineage, or release changes.

## 4. Planning Decision

The plan treats proportional governance as independent execution and disclosure
obligations:

```text
Proceed + Quiet
Proceed + Visible
RequireApproval + Visible
Denied + Visible
```

Only complete, immutable-bundle-derived, source-bound assessments may enter the
router. Caller-selected posture and detached projections remain
non-authoritative.

## 5. Disclosure Boundary

The existing governance binding proves that visible disclosure is required. It
does not prove delivery.

The first future visible route should use one explicit injected local delivery
surface and retain a validated payload-free receipt before skill execution.
That preserves non-blocking human semantics without silently dropping a
mandatory disclosure.

## 6. Approval Boundary

The future approval route must reuse existing durable approval requests,
approval-presentation proof, decisions, resolved-context validation, and
executor resume. Planning review must determine whether the current
step-scoped approval model can truthfully represent an aggregate pre-execution
governance gate.

No automatic or model-selected approval is authorized.

## 7. Privacy And Security

The plan forbids raw source/spec contents, check output, commands, paths,
environment values, policy payloads, approval prose, provider payloads,
credentials, and tokens in routing, delivery receipts, errors, audit, or
report-ready references.

Visible delivery, approval presentation, and audit recording remain distinct
proofs.

## 8. Validation

Completed successfully:

- `npm run check:docs`;
- `git diff --check`; and
- governed event-trail inspection.

## 9. Governed Phase Record

- workflow: `dg/d`
- run: `run-1785032157063812000-2`
- approval:
  `approval/run-1785032157063812000-2/planning-approved`
- presentation: `presentation/890bc03190252cb3`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: source inspection, architecture reasoning, plan
  authoring, documentation validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute checks, create a WorkReport artifact, or perform git actions

## 10. Remaining Limitations

- No visible-disclosure delivery model exists yet.
- No aggregate proportional-governance approval route exists yet.
- No denial-specific runtime route is implemented.
- The accepted authoritative consumer remains explicit, local,
  `DocsCheck`-only, fresh-run-only, and quiet-`Proceed`-only.
- Retry, resume, cancellation, CLI/UI projection, schemas, providers,
  OpenShell, and writes remain deferred.

## 11. Recommended Next Phase

Proceed first to the smallest payload-free visible-disclosure delivery
prerequisite. Keep executor integration as the next separately reviewed phase.
