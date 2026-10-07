# Trusted-Host Local Application Admission Plan Blocker Fix Report

## 1. Executive Summary

The trusted-host local application admission plan now selects an exact
foreground process owner, a fail-closed current-authority source for the
obligation-free docs-check slice, and a typed admission result that preserves
lawful workflow posture outside the error channel.

The first future implementation is an unpublished `workflow-local-host` binary
target. Core remains the only admission authority. The application supplies
selection and lifecycle input, while Core derives every run, step, immutable,
policy, approval, authority, handler, executor, `SkillInput`, opening, and
preparation binding.

## 2. Blockers Fixed

### 2.1 Foreground entry

The plan no longer leaves binary-versus-wrapper ownership open. The owner is
the unpublished `workflow-local-host` binary target. It owns `main`, process
exit, one SQLite backend instance, one closed handler lifetime, and scoped
cancellation. The end-to-end proof must invoke the binary.

### 2.2 Current-authority provenance

Repository inspection confirmed that no production code currently registers
`RegisteredInMemoryCurrentAuthoritySource`; existing registration calls are
test-only.

The selected first slice does not invent grants or accept caller-authored
inventories. Core may register complete empty inventories only after proving
from the immutable required-context contract and closed profile that the exact
operation requires zero capability grants, zero availability facts, and zero
governed-context references. Any non-empty, missing, stale, incomplete,
optional-but-policy-significant, or conflicting requirement blocks admission.

This is a narrow no-authority projection, not a general authority source.
Additional profiles require a separately reviewed source.

### 2.3 Admission result

The plan now separates:

- prepared operation;
- waiting posture;
- terminal replay;
- blocked posture;
- denial; and
- application failure.

Only the prepared variant carries the existing opaque pair. Lawful workflow
postures do not become errors, while invalid inputs, unsupported behavior,
security failure, invalid state, and internal failure retain fixed bounded
failure codes.

## 3. SQLite Lifecycle

The binary opens or creates one database through `SqliteStateBackend::open`,
validates schema and health before admission, and retains the same backend
through operation return. It performs no migration, repair, fallback, or
state-model translation. Backend failure stops before admission.

## 4. Authority Boundary

The application may supply project root, database path, workflow selection,
optional run selection, actor, correlation, closed-profile choice, and bounded
time observations. These are requests, not proof.

Core alone constructs the current-authority source, exact execution binding,
required-context contract, handler, executor commitment, `SkillInput`,
operational locator, opening posture, persistence identities, and prepared
pair. None of those values crosses into or can be amended by the application.

## 5. Waiting And Resume Posture

A run that needs approval or evidence returns a bounded waiting outcome before
preparation. A separately governed operation must change durable state. The
foreground binary may then be explicitly invoked again with the same run ID.
It does not poll, wait interactively, or grant approval.

## 6. Proof Additions

The test plan now directly requires:

- invocation of the actual binary target;
- accepted empty inventories only for an obligation-free contract;
- rejection of stale, incomplete, conflicting, and non-empty authority
  requirements;
- backend open, schema, and health failure before admission;
- typed waiting, terminal, blocked, denied, and failure outcomes; and
- unchanged filesystem CLI and PostgreSQL hosted behavior.

## 7. Scope Explicitly Not Completed

This documentation-only fix does not add Rust, a binary, command, source,
issuer, runtime configuration, released CLI behavior, hosted behavior,
discovery, scheduling, provider access, provider mutation, OpenShell, nested
harnesses, schemas, examples, or release changes.

## 8. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Limitations

- The operational path is not implemented.
- The no-authority projection supports only an exact obligation-free closed
  profile.
- The exact event projection for lawful pre-preparation waits remains an open
  review question.
- Process restart, background continuation, released CLI adoption, additional
  profiles, and general authority sources remain deferred.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. If accepted,
implement the complete binary-to-Core-admission-to-local-host-operation path as
one vertical slice. Do not implement a standalone source or broaden the
authority projection beyond the obligation-free docs-check profile.

## 11. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1791390796486929000-2`
- approval: `approval/run-1791390796486929000-2/fix-approved`
- presentation: `presentation/49fa64871343459d`
- presentation hash:
  `49fa64871343459d0982f0e56b1fdff807f1a4600697388628648f8baf959505`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: documentation-only admission planning blocker fix
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: documentation and diff checks passed; Rust checks were
  not run because the approved phase changed documentation only
- out-of-kernel work: source inspection, plan correction, report authoring,
  validation commands, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
