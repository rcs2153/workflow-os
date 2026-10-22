# Trusted-Host Cargo Format Workflow Composition Plan Report

## 1. Executive Summary

Planning is complete for composing the accepted Cargo format profile into one
private exact three-step trusted-host no-write workflow. The plan appends Cargo
format after the accepted project-validation and docs-check prefix, preserves
all existing one-step and two-step contracts, and defines a separate v4
terminal report and artifact path.

No runtime behavior was implemented in this phase.

## 2. Scope Completed

This phase:

- inspected the accepted two-step registry, admission, progression, history,
  replay, report, artifact, foreground-host, and test boundaries;
- inspected the accepted Cargo profile identity, root, mutation, result, and
  privacy boundaries;
- selected one exact compatibility-preserving three-step topology;
- defined the closed three-profile registry and host inputs;
- defined Core-owned atomic progression and causal-history requirements;
- defined bounded restart and terminal replay behavior;
- selected a distinct private v4 report commitment and artifact posture;
- defined failure disclosure, privacy, tests, and implementation sequence; and
- advanced the active roadmap to focused plan review.

## 3. Scope Explicitly Not Completed

The phase did not add Rust behavior, another check profile, arbitrary Cargo
commands, clippy, tests, builds, source-writing format, providers, external
writes, public CLI behavior, schemas, SDK changes, examples, hosted behavior,
parallelism, nested harnesses, automatic approvals, or release changes.

## 4. Architecture Decision

The plan chooses an additive exact three-step sibling rather than changing the
accepted two-step API. The new topology is:

```text
project validation -> docs check -> cargo format check
```

Appending Cargo format retains the complete accepted two-step prefix. The
existing v3 report contract remains unchanged; the three-step path receives a
distinct v4 commitment and contract identity.

## 5. Runtime And Integrity Boundary

Core remains authoritative for topology, current-step selection, profile
selection, immutable bundle identity, progression, and terminal truth. The
host resolves exactly three closed profiles and may execute at most three
opaque prepared operations, re-entering Core after each committed
continuation.

The accepted Cargo formatter-chain inventory, protected-tree exclusion-policy
identity, root-disjointness, pre/post integrity checks, and non-leaking result
posture remain requirements of the composed path.

## 6. Report And Replay Decision

The proposed v4 report binds the exact run, immutable bundle, ordered topology,
attempted profiles and commands, concrete results, matching invocation events,
and terminal event. Completed reports require three passing results. Failed
reports cite only attempted checks and explicitly disclose later checks that
were not attempted.

Fresh replay must never rerun a committed check or append events. A missing
exact artifact may be recreated through the existing create-only gate;
conflicting or unreadable artifacts fail closed.

## 7. Test Posture

The future implementation must prove ordered success, failure at every step,
restart after both continuation boundaries, exact terminal replay, causal
history corruption rejection, profile/result/bundle substitution rejection,
v4 report integrity, create-only artifact recovery, non-leakage, and complete
one-step and two-step compatibility.

## 8. Governed Planning Record

- Workflow ID: `dg/d`
- Run ID: `run-1792679968261360000-2`
- Approval ID:
  `approval/run-1792679968261360000-2/planning-approved`
- Presentation ID: `presentation/40e34758fc89a0f8`
- Approval outcome: granted under delegated-maintainer authority for the exact
  persisted documentation-only planning scope
- Phase status: `Completed`
- Event summary: 39 events, one approval, zero retries, and zero escalations
- Approval presentation enforcement: proof enforced with one persisted
  presentation record and the approval-event proof marker present

Repository inspection, documentation authoring, validation, and later git/PR
operations are out-of-kernel maintainer work under the approved scope. The
kernel governs scope and approval but does not perform those actions.

## 9. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Manual scope review confirmed that no implementation or broader execution
  authority is claimed.
- Governed phase close confirmed terminal `Completed` status.

No Rust or TypeScript implementation checks were required because this phase
changes documentation only. Missing runtime composition remains documented
rather than simulated.

## 10. Remaining Limitations

- The three-step workflow is planned, not implemented.
- The Cargo profile remains available only through its accepted private
  one-step path.
- The accepted two-step workflow remains the largest composed trusted-host
  no-write proof.
- Trusted-host execution has no operating-system process, filesystem, or
  network sandbox.
- Final-state source comparison cannot detect transient restored writes.
- Reports and artifacts remain private and have no released inspection or
  export surface.

## 11. Recommended Next Phase

Perform focused maintainer/security review of the composition plan. Do not
implement the three-step path until the review accepts topology order,
compatibility, immutable identity, atomic progression, causal-history
validation, Cargo integrity treatment, v4 reporting, replay, privacy, and the
test matrix.
