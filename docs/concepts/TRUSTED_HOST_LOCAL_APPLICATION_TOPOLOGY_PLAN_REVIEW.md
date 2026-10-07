# Trusted-Host Local Application Topology Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking follow-ups; proceed to the Core SPI model and
visibility slice only.**

The plan chooses a coherent boundary: Core remains authoritative, the future
local-host package remains unpublished and SQLite-specific, and no existing
CLI or hosted path is made to bridge incompatible state models. The proposed
opaque session contract prevents application code from assembling private
locators or capabilities.

The central qualification is terminology. A library package can compose
lifecycle behavior but cannot itself own a process. Until a real embedding
application is selected, the planned package must be described as an
unpublished local-host composition boundary, not an operational application
owner.

## 2. Scope Verification

The plan stayed within documentation and architecture planning. It does not
authorize runtime code in the planning phase, a command, discovery, a state
bridge, hosted parity, signal handling, provider behavior, writes, schemas, or
release changes.

## 3. Application Ownership Assessment

The rejection of current application sites is correct:

- the CLI owns a foreground lifetime but uses filesystem-backed execution and
  does not possess the trusted-host SQLite binding;
- the hosted worker owns process lifecycle but uses PostgreSQL leases and
  fencing; and
- Core owns authority and continuity but is not a process.

An unpublished local-host package is the least misleading place to develop
the composition. It must not be counted as operational adoption until an
actual binary or embedding application owns invocation and shutdown.

## 4. Backend Isolation Assessment

The plan correctly prohibits filesystem-to-SQLite and PostgreSQL-to-SQLite
translation. It also avoids silently enabling the owner from `run`, `approve`,
or `next-action`. This preserves one durable truth per execution path and
prevents SQLite scheduling from bypassing hosted claim and lease semantics.

## 5. SPI Visibility And Authority Assessment

The non-default feature is a useful packaging guard, not a security boundary.
The plan states that distinction explicitly and relies on opaque constructors,
current-state validation, one-shot values, and Core-issued sessions for actual
authority protection.

The first implementation must not export private locator, opening capability,
attempt capability, or continuation disposition constructors. Enabling the
feature must not itself authorize execution.

## 6. Session Issuance Assessment

The planned session is appropriately non-cloneable, non-serde, redacted, and
bound to one backend, immutable run, step, actor, window, invocation, and
authority context. Core rehydration and validation remain mandatory before
issuance.

The plan honestly identifies that no production source currently provides all
required inputs. The model/visibility slice may represent and test the
boundary, but it must not invent a fixture-backed or caller-assembled source
and describe that as adoption.

## 7. Cancellation And Failure Assessment

The plan preserves the accepted semantics:

- zero-write cancellation before entry;
- cooperative wake during supported timer waits;
- no revocation or interruption after executor admission; and
- synchronous waiting for admitted work to return.

It also correctly defers signals, detached work, process-loss recovery, and
restart ownership. Bounded owner outcomes remain separate from workflow
lifecycle claims.

## 8. Privacy Assessment

The planned session and outcomes exclude serialization and redact identity,
binding, input, path, provider, and credential values. Stable errors and fixed
outcome classifications are sufficient for the first slice. No raw state or
authority-bearing value should cross the SPI.

## 9. Implementation Sequence Assessment

Starting with the Core SPI model and visibility slice is acceptable because it
establishes the cross-crate authority boundary before adding another package.
That phase must remain more than vocabulary: compile-time feature posture,
opaque construction, one-shot ownership, non-serde behavior, and redacted
Debug must be directly tested.

The later preparation helper, local-host package, and operational source remain
separate review gates. No phase may claim a running host before the explicit
source and embedding process are both accepted.

## 10. Test Plan Assessment

The planned tests cover feature isolation, stale/substituted bindings,
one-shot construction, cancellation custody, active-attempt waiting,
competing owners, bounded outcomes, backend non-regression, and absence of
default activation. This is sufficient for phased implementation.

Compile-fail or feature-matrix coverage should be used where practical to
prove that the SPI is absent from ordinary builds and that session values
cannot be cloned or serialized.

## 11. Blockers

None for the Core SPI model and visibility slice.

Operational adoption remains blocked until a real in-process source and an
actual embedding application are planned, implemented, and reviewed.

## 12. Non-Blocking Follow-Ups

- Describe `workflow-local-host` as a composition library until a process
  actually embeds it.
- Prefer compile-time negative tests for feature absence and non-clone/non-serde
  guarantees.
- Decide whether the first implementation needs a public feature-gated module
  at all, or can keep the model inside Core until the application package is
  introduced.
- Do not split pure vocabulary from its construction/privacy tests.

## 13. Recommended Next Phase

Implement the Core SPI model and visibility slice only. Add the non-default
feature posture, opaque one-shot session and bounded outcome model, redacted
Debug, and focused tests. Do not add the local-host package, preparation
helper, binary, command, operational source, or runtime adoption yet.

## 14. Validation

- `npm run check:docs`: passed
- `git diff --check`: passed
- focused trusted-host competing-timer contention test: passed 25 consecutive
  local runs after one non-reproducing CI failure on the documentation-only
  branch

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791377240614268000-2`
- approval: `approval/run-1791377240614268000-2/review-scope-approved`
- presentation: `presentation/4261e6568799b11d`
- presentation hash:
  `4261e6568799b11d5e90a07bf4eca14713749534765c3002be4df1eaf90ba6bc`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused topology and security review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations;
  approval-presentation proof enforced with one persisted presentation record
- out-of-kernel work: repository and plan inspection, review authoring,
  validation, and later git or pull-request work
