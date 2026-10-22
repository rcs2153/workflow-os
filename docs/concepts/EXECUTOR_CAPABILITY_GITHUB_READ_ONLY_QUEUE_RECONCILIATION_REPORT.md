# Executor Capability-Gated GitHub Read-Only Queue Reconciliation Report

## 1. Executive Summary

The Active Delivery Queue has been reconciled after acceptance of Core-owned
durable capability-source continuity. The next executable runtime-composition
lane is one crate-private, opt-in executor operation for the already accepted
GitHub repository-metadata read.

This selection closes no runtime gap by assertion. It defines the exact plan
that must be reviewed before implementation.

## 2. Decision

Selected next phase:

**Focused maintainer/security review of the Executor Capability-Gated GitHub
Read-Only Composition Plan.**

After plan acceptance, the first implementation should compose one existing
read-only operation into a durable executor boundary. It must not add another
provider operation or any mutation.

## 3. Why This Selection

The repository already contains accepted foundations for:

- immutable run-bundle binding;
- current run and cursor rehydration;
- cursor-bound one-time continuation;
- registered current capability authority;
- step-scoped capability projection;
- GitHub read-only repository metadata;
- payload-free capability-use receipts;
- Core-owned durable source continuity;
- adapter telemetry and EvidenceReference; and
- WorkReport construction.

The remaining gap is composition. The private provider helper is proven but is
not reached from an executor path. Selecting another model family would widen
the docs/runtime gap called out by external review.

## 4. Alternatives Not Selected

- **Second provider operation:** rejected because one existing operation still
  lacks executor composition.
- **Provider mutation broadening:** rejected because a read-only composition is
  the safer proof of the boundary.
- **Public API or CLI exposure:** rejected until the private operation is
  reviewed in a real executor path.
- **Generic adapter gating:** rejected because it would generalize beyond the
  evidence available from one operation.
- **OpenShell or nested harness execution:** deferred; neither is required to
  prove this local governance boundary.
- **More model-only authority vocabulary:** not selected because accepted
  models already cover the chosen operation.

## 5. Key Architectural Finding

The generic `SkillHandler` cannot by itself establish this operation's full
authority context. Its input does not independently prove actor, workflow,
step, harness, immutable bundle, event cursor, current source, or selected
grant. Encoding those facts as mapped strings would convert caller input into
authority.

The plan therefore selects a closed Core operation adjacent to the executor,
with explicit rehydration and validation before provider entry.

## 6. Scope Boundary

The planning phase adds documentation only. It does not add Rust code, change
runtime behavior, introduce a public API, enable live GitHub access, add a
provider operation, mutate external state, write report artifacts, change
schemas, or alter release posture.

## 7. Governed Planning Record

- workflow ID: `dg/d`
- run ID: `run-1792701682492394000-2`
- approval ID:
  `approval/run-1792701682492394000-2/planning-approved`
- presentation ID: `presentation/6a35e827d730990d`
- presentation content hash:
  `6a35e827d730990d812f7cc1c63c4ca7c26f9e3bd033bf13fc37ee877a57f0d1`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  approval-presentation proof was persisted and enforced
- kernel boundary: governance coordination only; repository inspection,
  architecture analysis, documentation, validation, Git, pull-request, CI, and
  merge work remain outside the kernel

## 8. Validation

Required planning validation:

- `npm run check:docs`: passed
- `git diff --check`: passed
- governed phase inspection and close: passed; run terminal as `Completed`

No Rust implementation changed, so Rust formatting, Clippy, and test suites are
deferred to the implementation phase rather than represented as planning proof.

## 9. Recommended Next Phase

Perform focused maintainer/security review of
[Executor Capability-Gated GitHub Read-Only Composition Plan](../implementation-plans/executor-capability-github-read-only-composition-plan.md).

Do not implement the operation until that review resolves the event-vocabulary,
operation-selection, evidence-projection, and transaction-boundary questions.
