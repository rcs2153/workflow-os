# Trusted-Host Local Application Preparation Source Plan Blocker Fix Report

## 1. Executive Summary

The preparation-source planning blocker cannot be fixed by choosing another
opaque wrapper. Workflow OS has no current application process that owns the
SQLite trusted-host run-admission moment from which a lawful source could be
issued. The standalone source plan is therefore superseded. The next phase is
local-host application admission planning, followed by review, before any
source implementation.

## 2. Blocker Restatement

The reviewed plan proposed a Core-owned, one-shot preparation source with a
test-only issuer and no production caller. That would add an unused production
abstraction while leaving the runtime-composition gap unchanged.

The blocker fix had to identify:

- one process that owns the SQLite backend and foreground lifecycle;
- one Core call site that already owns exact current run, step, actor, bundle,
  authority, executor, and skill-input bindings; and
- one end-to-end path that passes the issued value into `workflow-local-host`
  without discovery or caller-authored authority.

## 3. Repository Findings

### 3.1 Core owns preparation but not application lifecycle

`prepare_trusted_host_local_application_session` already validates the exact
operational locator, fresh or existing posture, executor commitment, skill
input, immutable run bundle, actor, and current SQLite state. It creates the
prepared pair read-only and revalidates again when the session is consumed.

Core cannot, by itself, own foreground process shutdown, cancellation-handle
custody, or invocation timing.

### 3.2 The local-host package owns consumption but not admission

`workflow-local-host` accepts one unconstructible prepared pair and owns one
synchronous call plus scoped cancellation control. It does not own project
loading, run creation, runnable-step selection, policy or approval evaluation,
SQLite backend creation, executor registry, or validated `SkillInput`
construction.

### 3.3 Current applications are ineligible

The CLI's runtime is filesystem-backed. Its SQLite use is limited to explicit
migration and health operations. Adopting the source there would bridge state
models or create a new product command before the SQLite runtime contract is
ready.

The hosted worker is PostgreSQL-backed and relies on hosted claim, lease, and
fencing semantics. Calling a SQLite-only source there would bypass those
contracts.

### 3.4 Existing SQLite scheduling is downstream of admission

The private redispatch, timer, process-owner, and operational-entry functions
already own exact backend, executor, and skill-input bindings once an
operation has been selected. They do not define which application starts the
run, chooses the exact runnable step, or supplies the initial current-authority
opening. They therefore cannot serve as a production source issuer without an
upstream admission owner.

## 4. Decision

Do not implement `TrustedHostLocalApplicationPreparationSource` as a
standalone type or add a test-only production issuer.

Select `workflow-local-host` as the future foreground process owner, but do
not describe the existing library as an application yet. A future admission
plan must define the smallest production-shaped local application boundary
that:

1. owns one `SqliteStateBackend` instance for its foreground lifetime;
2. receives one exact runnable operation from a Core-owned admission result,
   not from copied identifiers or durable-state scanning;
3. obtains the exact reviewed attempt executor and validated `SkillInput`
   from the same admitted operation;
4. causes Core to construct fresh or existing preparation posture from current
   authoritative state;
5. immediately converts that admitted operation into the existing prepared
   pair and `LocalHostPreparedOperation`; and
6. retains cancellation custody until the synchronous operation returns.

The source may be introduced only inside that end-to-end slice. If the
admission design cannot produce these facts without a public authority factory
or hidden discovery, implementation must remain blocked.

## 5. Required Next Plan

Create a trusted-host local application admission plan that answers:

- What starts the local SQLite run and owns its backend lifecycle?
- Which Core transition identifies the exact runnable step?
- How are policy, approval, immutable bundle, actor, executor, and skill input
  bound before preparation?
- Which opaque admission result crosses into the local-host process without
  reusable authority or caller-authored locators?
- Where are fresh opening and existing continuation paths separated?
- How does one production-shaped integration test exercise admission through
  operation return without a public test factory?
- What operator surface is intentionally absent from the first slice?

The plan must not simply rename the proposed preparation source or introduce a
new opaque token without a production owner.

## 6. Preserved Requirements

Any later source remains:

- one-shot, non-cloneable, non-serde, lifetime-bound, and Debug-redacted;
- weaker than successful preparation and consumed execution;
- zero-write on construction, failed preparation, and drop;
- incapable of carrying caller-authored authority;
- mapped to existing bounded application failures before crossing Core; and
- unavailable to CLI, hosted, SDK, schema, and default Core consumers unless
  separately reviewed.

## 7. Scope Explicitly Not Completed

This blocker fix does not add:

- Rust implementation or a new crate;
- a source, issuer, resolver, application, binary, or command;
- run discovery, scanning, polling, scheduling broadening, or detached work;
- CLI or hosted adoption;
- provider access, mutations, OpenShell, or nested harnesses;
- schemas, SDKs, runtime configuration, or release changes; or
- automatic approval or authority broadening.

## 8. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

No Rust validation is required because this phase changes documentation only.

## 9. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1791389705340761000-2`
- approval: `approval/run-1791389705340761000-2/fix-approved`
- presentation: `presentation/cb2e89a2945b544d`
- presentation hash:
  `cb2e89a2945b544da23ca636e9f874022f354c45502013e9b94955e5e42eb634`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: documentation-only preparation-source blocker fix
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: documentation and diff checks passed; Rust checks were
  not run because the approved phase changed documentation only
- out-of-kernel work: architecture and code inspection, documentation edits,
  validation commands, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted

## 10. Recommended Next Phase

Plan the trusted-host local application admission vertical slice, then perform
a focused maintainer/security review. Do not return to source implementation
until that plan identifies a real production owner and one end-to-end path.
