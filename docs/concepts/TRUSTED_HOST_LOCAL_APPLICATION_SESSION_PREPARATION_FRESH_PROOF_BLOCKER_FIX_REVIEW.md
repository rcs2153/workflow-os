# Trusted-Host Local Application Session Preparation Fresh Proof Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; proceed to local-host caller composition planning.**

The direct authority-backed fresh-path proof is now complete enough to accept
the private preparation boundary. Fresh drop and pre-run cancellation prove
zero authority use, zero opening state, unchanged continuity state, no workflow
event append, and zero executor entry. Fresh backend, actor, immutable-bundle,
invocation/executor, opening-shape, and trusted-time substitutions fail before
pair issuance with stable bounded errors.

The tests exposed two real pre-issuance gaps. The focused fix adds a read-only
semantic preflight for unusable opening shape and incompatible trusted-time
context. Atomic source/contract evaluation, opening-identity uniqueness, and
final time-of-use validation remain correctly deferred to consumed execution.

## 2. Review Scope

This review inspected:

- the authority-backed fresh preparation fixture;
- fresh prepared-pair drop and pre-run cancellation;
- fresh binding and context substitution failures;
- the read-only opening-shape and trusted-time preflight;
- preservation of consumed-run authority use and atomic store validation;
- error stability, privacy, and feature isolation;
- focused and workspace validation; and
- the implementation report, prior reviews, plan, and roadmap claims.

The review did not implement a host package, caller, discovery, scheduling,
provider behavior, writes, schemas, SDK behavior, hosted parity, nested
harnesses, OpenShell, public authority construction, or release changes.

## 3. Original Blocker Restatement

The prior blocker-fix review accepted the production correction for existing
windows but found the fresh proof incomplete. The private fresh fixture proved
read-only preparation and one consumed run, but did not directly prove that
dropping or canceling an unconsumed fresh pair was zero-write. It also did not
exercise fresh context substitutions at the newly introduced preparation
boundary.

That left local-host composition blocked because the application-facing
preparation contract was stronger than the direct proof supporting it.

## 4. Fresh Drop And Cancellation Assessment

The direct fresh drop test captures the continuity snapshot and workflow-event
count before preparation. Dropping the returned pair proves:

- zero executor calls;
- zero opening operations, attempts, projection bindings, and windows;
- unchanged continuity state; and
- no workflow-event append.

The direct pre-run cancellation test proves the same properties and returns
the bounded `CanceledBeforeEntry` outcome. Neither path consumes current
authority or enters the executor.

## 5. Fresh Substitution Assessment

Direct preparation tests now reject:

- a different SQLite backend instance;
- a substituted subject actor;
- a substituted immutable run bundle;
- a mismatched invocation or executor binding;
- a zero-attempt opening request; and
- trusted-time provenance that differs from the continuity instance.

Each test proves a stable fixed error, zero executor entry, and no opening
records. Backend substitution additionally proves neither database receives an
opening record. Immutable-bundle mismatch may fail first through run
eligibility, which is an acceptable fail-closed pre-issuance result.

## 6. Validation Boundary Assessment

Fresh preparation first reuses the exact structural binding validation for the
backend instance, workflow/run/step binding, actor, immutable bundle, window,
and invocation commitment. It then performs a read-only semantic preflight for
the opening shape and current trusted-time state.

The preflight mirrors the deterministic request-shape and trusted-time checks
performed by the atomic opening store. It rejects zero attempts, expiry that
does not follow observed trusted time, source/provenance/epoch mismatch,
ineligible or quarantined continuity time, and trusted-time regression.

The preflight intentionally does not claim to reserve authority or opening
identities. Source and contract freshness are evaluated when current authority
is consumed. Operation, receipt, window, and attempt uniqueness remains inside
the atomic store transaction. This avoids a racy read-only uniqueness promise.

## 7. Authority And Time-Of-Use Assessment

Preparation remains a read-only coherence check. Consuming the one-shot
session recomputes current preparation context, evaluates current authority,
and performs the opening transaction. The atomic path repeats opening-shape,
runtime-binding, trusted-time, active-window, identity, and idempotency checks.

Successful preparation therefore remains explicitly weaker than successful
execution. The fix does not move authority consumption earlier or weaken the
one-winner durable boundary.

## 8. Error And Privacy Assessment

- New errors use stable namespaced codes and fixed messages.
- Errors do not include actor, identifier, bundle, invocation, trusted-time,
  path, payload, credential, command output, or source content.
- Prepared values remain private, one-shot, non-cloneable, non-serializable,
  and Debug-redacted.
- The current-authority fixture remains crate-private and `cfg(test)` only.
- No production authority factory or broader application surface is exposed.

## 9. Regression And Test Quality Assessment

The focused application-preparation suite contains 16 passing tests. It now
covers valid fresh and existing preparation, one consumed run, fresh and
existing drop/cancellation behavior, stale preparation, binding substitutions,
opening-shape rejection, trusted-time rejection, bounded failures, and privacy.

The substitution tests assert zero opening-store rows and zero executor entry.
They do not each repeat the snapshot and workflow-event count assertions used
by the fresh drop and cancellation tests. This is a non-blocking test-quality
follow-up: the reviewed substitution paths return before any write-capable
operation, and the direct zero-write assertions cover the prepared wrapper's
drop and cancellation behavior.

## 10. Scope Verification

The fix stayed within the approved Core-only blocker scope. It did not add an
application package, executable, production caller, discovery, scheduling,
provider read or mutation, schema, CLI, SDK, hosted behavior, nested harness,
OpenShell integration, public authority factory, or release change.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Repeat explicit snapshot and workflow-event non-mutation assertions in each
  fresh substitution test when that fixture is next touched.
- Add an external-consumer feature fixture before a separate package depends
  on the SPI.
- Exercise during-wait and post-admission cancellation through a later real
  application boundary.
- Keep the prepared wrapper private until a concrete embedding package proves
  the smallest required exposure.
- Preserve the distinction between successful preparation and successful
  consumed execution.

## 13. Recommended Next Phase

Plan the first unpublished local-host caller composition around an injected
prepared pair. The plan must keep Core authoritative for preparation and
identity construction, add no ambient discovery or scheduling, and preserve
the private, feature-gated boundary until an external-consumer fixture proves
the required package interface.

Do not begin provider mutation broadening, OpenShell integration, nested
harnesses, public configuration, or general scheduling as part of that phase.

## 14. Validation

The reviewed implementation passed:

- feature-focused application-preparation tests: 16 passed;
- feature-focused clippy with warnings denied;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791386358892360000-2`
- approval: `approval/run-1791386358892360000-2/review-scope-approved`
- presentation: `presentation/84760634b2be05d7`
- presentation hash:
  `84760634b2be05d779b704677f0436dfab71d390b53dfa1d1a6f094497b01ece`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused fresh-proof blocker-fix maintainer/security
  review only
- phase status: completed
- event summary: recorded by governed phase close with one proof-enforced
  approval and no retry or escalation
- out-of-kernel work: code and documentation inspection, validation commands,
  review authoring, and later git or pull-request work
