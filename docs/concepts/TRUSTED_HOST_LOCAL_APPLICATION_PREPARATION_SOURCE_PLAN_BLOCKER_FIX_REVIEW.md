# Trusted-Host Local Application Preparation Source Plan Blocker Fix Review

## 1. Executive Verdict

**Blocker fix accepted; proceed to trusted-host local application admission
planning.**

The corrected plan no longer authorizes an unreachable preparation-source
implementation. It accurately identifies the missing runtime boundary: no
current process owns both the SQLite trusted-host admission context and the
foreground lifecycle needed to issue and consume a prepared operation.

The next plan must select that admission path before any source code is added.
This unresolved design is deliberate next-phase work, not a remaining blocker
to accepting the correction.

## 2. Review Scope

This review assessed:

- whether the original standalone implementation is fully withdrawn;
- whether repository ownership findings match current code and package
  topology;
- whether the future owner and Core issuance prerequisites are stated without
  overclaiming an application that does not exist;
- authority, dependency, privacy, failure, and testing boundaries;
- roadmap sequencing; and
- documentation honesty.

It did not implement a source, admission path, application, runtime behavior,
CLI or hosted integration, provider behavior, schema, or release change.

## 3. Original Blocker Resolution

The original review found that a source with only a test issuer would be an
unused production abstraction. The blocker fix resolves that finding by:

1. superseding the standalone implementation plan;
2. removing the old source implementation sequence and test-only issuer;
3. documenting that no current application owns the issuance point;
4. requiring admission ownership before source reconsideration; and
5. requiring the first accepted implementation to prove an end-to-end
   admission-to-operation path.

The correction does not hide the missing call site behind a trait, provider,
resolver, callback, or renamed opaque value.

## 4. Repository Findings Assessment

The ownership findings are accurate:

- Core owns SQLite state, authority, preparation, redispatch, timer, and
  consumed-execution revalidation but not process lifetime.
- `workflow-local-host` owns only one prepared operation and cancellation
  control; it does not admit runs or steps.
- CLI execution remains filesystem-backed and cannot adopt this SQLite path
  without a separately reviewed runtime surface.
- Hosted execution remains PostgreSQL-backed and cannot adopt the SQLite path
  without violating lease and fencing ownership.

The private operational-entry and scheduling functions are downstream of the
missing admission decision. Their possession of backend, executor, and skill
input does not make them a production issuer because no application calls them
from a lawful admitted run.

## 5. Future Owner Assessment

The plan correctly names a **future local-host application boundary** as the
foreground owner while refusing to equate that future application with the
current library package.

This is appropriately precise for the blocker fix. The next plan must still
identify:

- the application entry point;
- SQLite backend lifecycle;
- the Core run/step admission transition;
- executor and validated input provenance;
- fresh versus existing posture derivation; and
- the exact value crossing from Core into local host.

Implementation remains blocked until those questions have a reviewed answer.

## 6. Authority And Dependency Assessment

The corrected plan preserves the right dependency direction:
`workflow-local-host` may depend on Core, while Core must not depend on the
local-host package.

It also preserves the critical authority rule: caller-selected identifiers,
backend handles, copied state, event data, or externally assembled locators
cannot become execution authority. Core must derive the admitted operation
from current state and bind the exact actor, immutable bundle, executor, skill
input, policy, approval, and opening posture before crossing the application
boundary.

## 7. Failure And Privacy Assessment

The preserved requirements remain adequate:

- source construction, failed preparation, and drop are zero-write;
- source, pair, and operation remain one-shot and non-serde;
- Debug output remains redacted;
- Core failures become fixed payload-free application failures before crossing
  the boundary;
- state drift fails closed; and
- no reusable authority or caller-authored context crosses the SPI.

No raw identifiers, paths, payloads, credentials, provider values, or Core
diagnostics are authorized at the application boundary.

## 8. Test Strategy Assessment

The required future end-to-end test is stronger than the rejected source-only
tests. It must use production ownership and constructors from local admission
through synchronous operation return, while proving:

- pre-admission and preparation failures are zero-write;
- drop is zero-write;
- stale or substituted state fails closed;
- executor admission is at most once;
- no source or operation can be reused;
- failures remain bounded; and
- no public test authority factory is introduced.

A compile-only or package-private source fixture will not satisfy this
requirement.

## 9. Scope Verification

The blocker fix stayed within documentation-only scope. It did not add Rust
code, packages, commands, runtime configuration, discovery, scheduling,
provider access, mutations, schemas, automatic approvals, hosted behavior, or
release claims.

The roadmap now points to admission planning rather than source
implementation.

## 10. Remaining Blockers

None for accepting and merging the planning correction.

Implementation of a preparation source remains blocked until the local-host
application admission plan is reviewed and accepted.

## 11. Non-Blocking Follow-Ups

- The admission plan should decide whether a distinct source type is needed or
  whether Core should return the prepared pair directly.
- The admission plan should avoid turning exact-operation selection into
  general run discovery or scheduling.
- The admission plan should state whether the first application boundary is
  unpublished permanently or only during hardening.
- Process-loss observation and operator surfaces should remain separately
  reviewed.

## 12. Recommended Next Phase

Create the trusted-host local application admission plan. The plan must be
reviewed before implementation and must define one production-shaped path
from Core admission to local-host operation return.

Do not implement the standalone source, add a public authority factory, bridge
CLI filesystem state, call SQLite from hosted execution, or broaden into
discovery, scheduling, provider behavior, OpenShell, nested harnesses, schemas,
or release changes.

## 13. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

Rust validation was not required because the reviewed correction changes
documentation only.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1791390022932304000-2`
- approval: `approval/run-1791390022932304000-2/review-scope-approved`
- presentation: `presentation/861608e531804017`
- presentation hash:
  `861608e531804017cac441fcbd9098c477a020d6dc9ac6d3e35d8de55ea6b0b9`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused planning blocker-fix review only
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: documentation and diff checks passed; Rust checks were
  not run because the reviewed change is documentation only
- out-of-kernel work: source and architecture inspection, review authoring,
  validation commands, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
