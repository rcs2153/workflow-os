# Governance Without Brittle Orchestration Report

## 1. Executive summary

The roadmap and user-facing guidance now clarify that Workflow OS is not a rigid graph-control framework for agents. Workflow OS governs the work around probabilistic execution: steps, gates, approvals, evidence obligations, side-effect disclosure, validation, handoffs, auditability, and final reporting.

## 2. Scope completed

- Added a roadmap section titled `Governance Without Brittle Orchestration`.
- Updated the agent harness quickstart to explain that agents keep speed and flexibility while Workflow OS makes work inspectable.
- Updated the Governed Work Pattern concept to distinguish work governance from brittle agent-edge orchestration.
- Connected evidence capture and final reports to workflow evolution and catalog stewardship.

## 3. Scope explicitly not completed

- No runtime behavior changes.
- No new workflow schema fields.
- No automatic evidence capture implementation.
- No automatic workflow generation or catalog mutation.
- No write-capable adapters.
- No hosted collaboration registry.
- No recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Product thesis summary

Workflow OS should not ask users to trade automation speed for governance. Agents should run with the kernel. The kernel should block only at meaningful governance boundaries such as missing approval, denied policy, unsafe side effect, failed validation, missing required evidence, unsupported authority, or required report closure.

## 5. Evidence posture

The roadmap now states that evidence should be gathered from existing run events, validation diagnostics, adapter telemetry, local checks, side-effect records, reports, and explicit citations where implemented. Evidence gathering should not interrupt every agent action.

## 6. Dogfood governance used

- Workflow: `dg/implement`
- Initial generated run: `run-1782061751262547000-2`
- Initial result: waiting for approval; approval attempt exposed an idempotency-key length limitation with generated run IDs.
- Completed governed run: `run/gbo`
- Approval: `approval/run/gbo/implementation-approved`
- Final status: completed.

## 7. Commands run and results

- `npm run check:docs` passed.

## 8. Remaining limitations

- The thesis is now documented, but automatic evidence gathering remains implemented only through existing bounded surfaces.
- Workflow recommendation and catalog governance remain future roadmap capabilities.
- The generated-run-id idempotency limitation should be considered for a future blocker-fix or hardening phase.

## 9. Recommended next phase

Dogfood Workflow Suite Phase 2: add `dg/runtime-composition`, `dg/blocker`, and `dg/release`, then use the suite to govern the next implementation, review, and PR handoff.
