# Independent Local Check Attestation Verifier Plan Report

## 1. Executive Summary

Planning now defines the first authority boundary that may convert an explicitly
unverified local check candidate into accepted independent proof. Focused review
found that the original plan lacked a pre-execution immutable command and
handler commitment. The fix now requires a separate content-addressed execution
binding before verifier implementation.

No verifier, check execution, runtime integration, persistence, event, schema,
CLI, provider, SideEffect, or write behavior is implemented.

## 2. Scope Completed

- Defined trusted verifier inputs and source-of-truth boundaries.
- Chose crate-private observation and verifier visibility.
- Defined command-contract fingerprint requirements.
- Defined exact immutable-run, command, handler, invocation, result, policy,
  temporal, freshness, and candidate-binding checks.
- Defined a distinct non-constructible accepted record.
- Defined stable non-leaking error families.
- Defined focused future tests and implementation sequencing.
- Reconciled proportional-governance and capability-authority boundaries.
- Defined the prerequisite immutable local-check execution binding and its
  honest `KernelObservedLocalProcess` assurance boundary.

## 3. Scope Explicitly Not Completed

- verifier implementation;
- local process or check execution;
- automatic handler registration;
- persistence, cache reuse, events, audit projection, evidence, or reports;
- executor, approval, or artifact enforcement;
- schemas, SDKs, CLI, UI, or examples;
- provider access, side effects, writes, hosted behavior, or release changes.

## 4. Key Architecture Decision

Core must first create an immutable local-check execution binding before
observation. That separate binding references the immutable run bundle and
commits the complete command contract, Core-derived registered-handler selection
metadata and posture, and effective execution policy. It does not pretend that current
run bundles already contain local-check command definitions.

The later verifier must not be public over a publicly constructible observation.
Both the kernel observation constructor and verifier should be crate-private.
The accepted result may be public and read-only, but must have no public
constructor and no deserialization path in the first phase.

This prevents callers from manufacturing the authority that the verifier is
intended to establish.

## 5. Validation Boundary Summary

The planned verifier recomputes and validates:

- requirement fingerprint;
- stored immutable bundle integrity and exact run binding;
- canonical command-contract fingerprint;
- handler registration and identity posture;
- workflow/run/step, invocation, idempotency, and result identity;
- accepted status, exit, duration, timeout, truncation, and policy posture;
- temporal ordering and evaluation-time freshness;
- complete unverified candidate binding fingerprint.

Failure returns no partial accepted record.

## 6. Privacy Summary

The verifier and accepted record remain payload-free. Raw output, arguments,
paths, source contents, environment values, credentials, provider payloads, and
free-form claims are excluded. Debug and errors must remain bounded and
non-leaking.

## 7. Relationship To Current Roadmap

This phase follows the merged and accepted independent attestation core model.
It supports the broader goal of proving real checks before expanding provider
mutations. It also supplies a future typed check fact for proportional
governance without granting capability or SideEffect authority.

## 8. Governed Phase

- workflow: `dg/d`
- run: `run-1784512170493900000-2`
- approval: `approval/run-1784512170493900000-2/planning-approved`
- presentation: `presentation/74dcebc833f4965a`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: planning coordination only; repository edits and validation
  were performed outside the kernel

## 9. Validation

- `npm run check:docs`
- `git diff --check`

## 10. Remaining Limitations

- No accepted proof exists yet.
- No public compile-fail privacy test tooling is selected.
- The immutable execution binding and verifier remain unimplemented.
- Registered-unattested handlers do not establish implementation integrity.
- Persistence and time-of-use freshness remain separate future boundaries.
- The dogfood phase-close presentation-record list-cap defect remains open.

## 11. Recommended Next Phase

Repeat focused maintainer review of the corrected plan. If accepted, implement
the immutable local-check execution binding core model only. The pure verifier
follows only after that model passes review. Do not integrate the executor or
broaden provider writes first.
