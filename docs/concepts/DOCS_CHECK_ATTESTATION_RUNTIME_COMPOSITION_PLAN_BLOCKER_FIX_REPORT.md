# DocsCheck Attestation Runtime Composition Plan Blocker Fix Report

## 1. Executive Summary

The two planning blockers are fixed. The future composition helper now owns all
binding and observation time through one injected crate-private clock, and it
uses typed requirement-status eligibility before verifier invocation.

This is a documentation-only planning fix. Runtime composition remains
unimplemented.

## 2. Clock Authority Fix

The input no longer accepts binding creation, observation start, observation
completion, or verifier evaluation timestamps. The helper receives one narrow
Core-owned clock and samples it:

1. for immutable execution-binding creation;
2. immediately before process-runner invocation;
3. immediately after process-runner completion; and
4. immediately before verifier evaluation.

The runner and public callers cannot supply or override those facts. Clock
errors or impossible ordering fail closed without partial proof.

## 3. Typed Eligibility Fix

The helper checks the observed result status against
`LocalCheckAttestationRequirement::accepted_statuses()` before invoking the
verifier.

- Ineligible status returns the honest structured result and no proof.
- Eligible status invokes the verifier.
- Every verifier error for an eligible status propagates as an integrity
  failure.

The helper does not inspect verifier error strings or codes and cannot downgrade
an integrity failure into an ordinary check outcome.

## 4. Scope Preservation

No Rust implementation, process execution, executor behavior, default
registration, persistence, events, evidence, reports, artifacts, schemas, CLI,
providers, SideEffects, writes, hosted behavior, or release posture changed.

## 5. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784557795698162000-2`
- approval: `approval/run-1784557795698162000-2/fix-approved`
- presentation: `presentation/31d18c08e7983e76`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; documentation and validation
  ran outside the kernel

## 6. Validation

- `npm run check:docs` - passed.
- `git diff --check` - passed.
- governed phase close - completed with 39 events, one approval, zero retries,
  and zero escalations.

The close helper again reported the known 250-record presentation-read cap.
The exact presentation proof was persisted and used for approval; no approval
was inferred or bypassed.

## 7. Recommended Next Phase

Perform a focused re-review. Do not implement the helper unless the corrected
plan is accepted.
