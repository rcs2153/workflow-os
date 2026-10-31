# First-Run Quiet-Success Artifact Composition Plan Report

## Summary

The selected quiet-success artifact lane is now specified as one bounded
composition plan. The key decision is to preserve the existing
`local/first-run-governance` approval/audit demo and add a second exact generated
artifact workflow, low-sensitivity artifact skill, and observe-and-report policy
for real bounded analysis.

The plan does not treat explicit invocation as approval. Core remains the only
route authority and may return quiet proceed, visible proceed, approval
required, or denial from complete current facts.

## Planned Boundary

- new starts use a separate exact generated contract;
- explicit local-write request authority is one-use, CLI-private, immutable-
  context-bound, and cannot satisfy an approval minimum;
- legacy in-flight runs retain their immutable approval-gated contract;
- missing new scaffold files require an additive upgrade before run creation;
- quiet and visible routes fabricate no approval records or proof markers;
- approval escalation reuses persisted presentation proof and fresh
  reassessment;
- local artifact writes remain create-only and separately disclosed from
  external SideEffects; and
- replay and recovery preserve the route that authorized terminal execution.

## Explicitly Deferred

No runtime, schema, SDK, provider, network, OpenShell, hosted, enterprise,
nested-harness, release, or broader-profile behavior is implemented by this
planning phase.

## Governed Phase

- workflow: `dg/d`;
- run: `run-1793438127349702000-2`;
- approval: `approval/run-1793438127349702000-2/planning-approved`;
- presentation: `presentation/befe74a67cc8acd2`;
- decision: granted under delegated-maintainer authority; and
- kernel boundary: scope, approval proof, and durable phase history only.

Repository analysis and documentation edits were performed outside the kernel
under the approved scope.

## Validation

Validation passed:

- pinned Node 20 `npm run check:docs`;
- `git diff --check`; and
- governed phase close with `Completed`, 39 events, one proof-enforced
  approval, zero retries, and zero escalations.

Rust checks are intentionally deferred because this phase changes planning
documentation only. No runtime handler, check, report artifact, or approval was
simulated by the planning workflow.

## Next Step

Perform focused maintainer/security review of the plan. Implementation remains
blocked until that review accepts the contract split, authority boundary,
compatibility posture, and acceptance matrix.
