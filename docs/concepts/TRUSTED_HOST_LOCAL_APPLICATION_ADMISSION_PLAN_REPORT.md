# Trusted-Host Local Application Admission Plan Report

## 1. Executive Summary

The trusted-host local application admission boundary is now planned. The
plan selects one unpublished foreground owner in `workflow-local-host`, one
SQLite backend lifecycle, one Core-owned admission transition, and one closed
no-write docs-check profile for the first end-to-end slice.

The plan closes the design gap identified by the preparation-source review:
it does not add a source with only a test issuer, bridge the filesystem CLI,
or call SQLite from hosted execution. Core remains authoritative for every
identity, policy, approval, immutable-input, current-authority, executor,
skill-input, opening, and preparation decision. Local host receives only the
existing opaque prepared pair and consumes it synchronously.

## 2. Scope Completed

- Inspected current CLI, Core, SQLite, executor, handler, SPI, and local-host
  ownership boundaries.
- Confirmed that generic `LocalExecutor<SqliteStateBackend>` compatibility is
  insufficient for trusted-host operational admission.
- Selected the future foreground owner and SQLite lifecycle.
- Defined the Core admission transition and exact authority boundary.
- Defined fresh and existing run behavior.
- Selected the explicit docs-check profile as the only first-slice handler.
- Defined executor and `SkillInput` provenance.
- Defined the opaque cross-crate handoff, bounded failures, privacy posture,
  workflow semantics, implementation sequence, and end-to-end tests.

## 3. Scope Explicitly Not Completed

This phase did not add Rust code, a binary, a command, a source, an issuer, an
application, a backend, a handler, runtime configuration, discovery,
scheduling, provider access, provider mutation, OpenShell, nested harnesses,
schemas, hosted parity, or release changes.

The released CLI remains filesystem-backed for normal execution. The hosted
worker remains PostgreSQL-backed. No operational trusted-host path exists yet.

## 4. Architecture Decision

The future path is:

```text
unpublished workflow-local-host foreground entry
  -> one explicit admission request
  -> Core project/run/policy/approval/current-authority admission
  -> Core-derived closed-profile executor and validated SkillInput
  -> Core fresh/existing preparation
  -> opaque prepared session pair
  -> LocalHostPreparedOperation
  -> synchronous one-shot execution
  -> bounded application posture
```

The application request is selection input, not authority. Core derives all
authority-bearing facts from validated project definitions and current SQLite
state in the admission call.

## 5. Foreground Owner

The plan selects an unpublished foreground entry in `workflow-local-host`.
Unlike the current library wrapper, that entry will own one SQLite backend and
one resolved closed handler for the complete synchronous operation lifetime.

The focused review must select the concrete executable form. It cannot accept
a library function with no real process caller or a test-only issuer as the
first implementation.

## 6. Admission Boundary

Core admission must load and validate the project, establish or rehydrate the
immutable run, evaluate policy and approval posture, select exactly one
runnable step, resolve current authority and required context, derive the
closed handler and validated input, choose fresh or existing posture, and
prepare the opaque pair.

The application cannot provide a locator, opening input, executor,
`SkillInput`, capability, current-authority source, or preparation source.

## 7. Handler Decision

The first slice is restricted to the existing explicit docs-check profile. It
is already modeled as a closed no-write command contract and handler. Arbitrary
commands, ambient discovery, mocks, adapters, provider calls, and additional
profiles remain out of scope.

## 8. Validation And Test Posture

The later implementation must prove fresh and existing admission, approval
wait, policy denial, terminal replay, cancellation, state drift, at-most-once
executor admission, bounded privacy, and the actual foreground path. Direct
private preparation calls and test-only factories do not satisfy acceptance.

This planning phase requires documentation and diff validation only.

## 9. Remaining Limitations

- No operational application exists yet.
- The executable form of the unpublished foreground entry requires focused
  review.
- The first slice supports only one explicit no-write profile.
- Process restart, background scheduling, run discovery, released CLI
  adoption, hosted parity, and additional handlers remain deferred.
- The path makes no production service-availability claim.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of the admission plan. If
accepted, implement the complete Core-admission-to-local-host-operation slice
as one phase. Do not split out another standalone source or issuer.

## 11. Governed Phase Record

- workflow: `dg/d`
- run: `run-1791390471334764000-2`
- approval: `approval/run-1791390471334764000-2/planning-approved`
- presentation: `presentation/7ed0bbf766b192ed`
- presentation hash:
  `7ed0bbf766b192edb2c8519bab0fe0d5c24bf56c68e91ddffb6413dd64d28797`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: documentation-only trusted-host local application
  admission planning
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: `npm run check:docs` and `git diff --check` passed; Rust
  checks were not run because the approved phase changed documentation only
- out-of-kernel work: repository architecture inspection, plan and report
  authoring, documentation validation, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
