# Governed Spec-Field Phase Schema-Scope Blocker Fix Report

## 1. Executive Summary

Dogfooding found that the governed phase runner applied one hard-coded
`approval_does_not_allow` string to every phase. That string prohibited schema
changes even for `dg/spec-field-operationalization`, making the dedicated
workflow unable to authorize its declared work.

The runner now emits phase-aware non-scope. Ordinary phases still prohibit
schema changes. The spec-field phase excludes only schema changes outside its
explicitly approved scope.

## 2. Blocker Fixed

The rejected implementation handoff attempted to authorize a defaulted
`StepDefinition` field while also saying approval did not allow schema changes.
The delegated maintainer denied that gate rather than treating the field as a
non-schema Rust change.

## 3. Behavior Added

- `phaseApprovalNonScope(phaseName)` owns deterministic phase-specific output.
- All existing phases retain the conservative default.
- `spec-field-operationalization` receives one narrow exception phrase.
- Structured and copy-safe approval handoffs use the same resolved non-scope.
- Caller-supplied strict non-goals remain independent and required for live
  material phase starts.

## 4. Behavior Not Added

No runtime approval semantics, automatic approvals, Workflow OS spec model,
schema artifact, executor behavior, persistence, SideEffect, write, provider,
hosted, or release behavior changed.

## 5. Tests

Focused tests prove ordinary phases still prohibit schema changes and the
dedicated spec-field phase permits only explicitly approved schema scope in its
rendered handoff.

## 6. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784967967364729000-2`
- approval: `approval/run-1784967967364729000-2/fix-approved`
- presentation: `presentation/c4b9409f7462ddea`
- outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; edits and validation run
  outside the kernel

## 7. Recommended Next Phase

Restart typed local-check declaration implementation under
`dg/spec-field-operationalization` with the exact serialized field scope and
non-goals presented for approval.
