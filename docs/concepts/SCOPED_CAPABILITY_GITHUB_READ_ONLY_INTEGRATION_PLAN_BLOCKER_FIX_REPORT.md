# Scoped Capability GitHub Read-Only Integration Plan Blocker Fix Report

## 1. Executive Summary

The five blockers from the focused maintainer and security review are fixed in
the integration plan. The selected operation and no-write boundary are
unchanged. No Rust or adapter runtime behavior was implemented.

## 2. Blockers Fixed

The plan now requires:

- an explicit already-evaluated adapter policy precheck with operation-mode-
  specific provenance rather than treating capability authority as policy;
- lowercase canonical GitHub owner/repository identity from separately bounded
  segments;
- dedicated crate-private non-context capability-use query and fact-set types;
- a separate `CapabilityUseAuthorityReceipt` V1 that leaves the existing
  required-context receipt unchanged; and
- an explicit `ReadSucceededReceiptFailed` posture that preserves completed
  provider-read truth and prevents blind retry.

## 3. Additional Decisions

The first integration requires the selected grant itself to contain the exact
run and step. Generic capability resolution semantics remain unchanged.

Fixture mode accepts only fixture-test policy provenance. Live-read-only mode
accepts only runtime-policy provenance. Other operation modes and approval-
granted posture remain outside this slice.

## 4. Scope Preserved

The fix adds no Rust, adapter invocation, provider write, live GitHub access,
credential handling, CLI, schema, SDK, automatic grant, hosted identity,
nested harness, or release change.

## 5. Validation

Required validation:

- `npm run check:docs`; and
- `git diff --check`.

Both checks passed. No Rust validation was required because the phase changed
documentation contracts only.

## 6. Recommended Next Phase

Perform a focused blocker-fix review. If accepted, proceed directly to the
complete no-write implementation milestone.

## 7. Dogfood Governance

- workflow: `dg/blocker`
- run ID: `run-1792691297499976000-2`
- approval ID: `approval/run-1792691297499976000-2/fix-approved`
- presentation ID: `presentation/d94eba4ab717151d`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- kernel boundary: governance coordination only; documentation edits,
  validation, Git, and pull-request work remain out of kernel
