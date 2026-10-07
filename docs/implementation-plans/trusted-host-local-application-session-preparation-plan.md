# Trusted-Host Local Application Session Preparation Plan

Status: implemented as a private, feature-gated Core boundary. The production
pre-issuance blockers and the direct fresh-path proof blocker are fixed. Fresh
drop, pre-run cancellation, context substitution, opening-shape, and trusted-
time tests now prove the documented zero-write boundary. The focused proof
review is accepted. Local-host caller composition may proceed through a
separate [caller-composition
plan](trusted-host-local-application-caller-composition-plan.md); no package,
production caller, or operational application path is implemented here.

## 1. Executive Summary

The trusted-host local application SPI now has an accepted opaque one-shot
session, bounded outcomes, and a payload-free session failure boundary. The
next missing boundary is Core-owned preparation: converting explicit current
SQLite execution context into one opaque session and cooperative cancellation
handle without letting application code construct locators, capabilities, or
continuation state.

The first implementation should add one crate-private preparation helper and
direct Core tests only. Preparation performs read-only current-state and
binding validation, captures exact inputs into a one-shot session, and returns
the session with its cancellation handle. The session must revalidate current
authoritative state and use the existing atomic opening/redispatch path when
`run` is consumed. Preparation must not consume authority or open an attempt
early.

The implementation must also project cancellation request failures into the
same fixed application failure vocabulary before any production helper can
issue the handle.

This plan does not implement the helper, an application package, an
executable, a caller, discovery, scheduling, signal handling, provider
behavior, writes, schemas, SDK behavior, hosted parity, or release changes.

## 2. Goals

- Define the smallest Core-owned preparation boundary.
- Accept only explicit current inputs already available inside Core.
- Validate backend identity, workflow/run/step identity, immutable bundle,
  actor, window, invocation, skill input, and opening context coherently.
- Preserve read-only preparation and authoritative run-time revalidation.
- Return one opaque session and its paired cooperative cancellation handle.
- Keep all locator, authority, capability, and continuation construction
  private to Core.
- Project both run and cancellation failures to fixed payload-free vocabulary.
- Preserve zero discovery, zero ambient authority, and one-shot ownership.
- Define direct tests that prove stale, missing, ambiguous, or substituted
  inputs fail closed before session issuance.

## 3. Non-Goals

The planned implementation does not authorize:

- `workflow-local-host` or any other new package;
- an executable, command, SDK method, workflow field, schema, or configuration;
- a production caller or application-selected invocation source;
- queue polling, project scanning, startup discovery, or general scheduling;
- signal handlers, detached work, active-attempt interruption, or recovery;
- filesystem or PostgreSQL state translation;
- provider reads or mutations, OpenShell, nested harnesses, or broader writes;
- caller-created locators, capabilities, authority, trusted time, or
  continuation dispositions;
- default feature activation or a stable compatibility promise; or
- release posture changes.

## 4. Existing Composition Chain

The implementation must compose existing private Core boundaries rather than
duplicate them:

1. `SqliteStateBackend` owns the exact durable state instance.
2. `OperationalExecutionWindowOpeningUseInput` carries current authority,
   required-context bindings, trusted time, opening identity, retry limit, and
   redaction for a fresh window.
3. `TrustedHostOperationalEntryLocator` binds workflow, run, step, window,
   subject actor, and immutable run bundle.
4. `TrustedHostOperationalEntryInput` binds the backend, optional fresh
   opening, attempt executor, skill input, persistence identity, and
   redispatch identity provider.
5. `TrustedHostExplicitLocalProcessOwner` owns one foreground operation and
   cooperative cancellation state.
6. `TrustedHostLocalApplicationSession` projects the private owner into the
   one-shot application SPI.

Application code must receive only the opaque session, cancellation handle,
bounded outcome, and fixed failure vocabulary.

## 5. Preparation Boundary Decision

Add one feature-gated, crate-private helper conceptually shaped as:

```text
prepare_trusted_host_local_application_session(input)
  -> Result<TrustedHostLocalApplicationPreparedSession, WorkflowOsError>
```

The exact Rust names may follow repository conventions, but the boundary must
remain internal until an explicit production source is separately planned.
The returned prepared value should expose only a consuming `into_parts` method
that yields:

- `TrustedHostLocalApplicationSession`; and
- `TrustedHostLocalApplicationCancellationHandle`.

The helper input may use private Core types. It must not create a public
caller-assembly API merely to make the model externally constructible.

Preparation errors remain inside Core because no application value is issued
on failure. Once a session or handle crosses the SPI, only
`TrustedHostLocalApplicationFailure` may cross back.

## 6. Explicit Input Contract

The private preparation input must contain or derive from exact values:

- one borrowed `SqliteStateBackend`;
- workflow, run, and step identity from one required-context execution
  binding;
- subject actor identity;
- immutable run bundle binding from the rehydrated run;
- one exact `SkillInput`;
- one borrowed `TrustedHostAttemptExecutor` and its binding commitment;
- fresh-opening authority context when no matching window exists;
- window identity and current continuation posture when a matching window
  exists;
- opening persistence operation/receipt identity;
- Core-owned redispatch and wake identity generation; and
- bounded retry/wake posture already defined by the existing local caller.

No string-only locator, copied snapshot, serialized capability, caller-authored
continuation disposition, or application-generated authority may be accepted.

## 7. Fresh And Existing Window Posture

Preparation must distinguish two current authoritative postures without
performing general discovery.

### 7.1 Fresh Opening

When the exact workflow/run/step scope has no continuity window:

- fresh-opening context is required;
- backend identity and every execution/opening binding must match;
- the rehydrated run must be running or retrying;
- the run must carry the expected immutable bundle;
- the skill invocation commitment must match the executor binding; and
- preparation captures the exact context but does not consume current
  authority or open the window.

### 7.2 Existing Window

When the exact scope has one continuity window:

- the window, actor, immutable bundle, and operation binding must match;
- current continuation posture must be supported;
- fresh-opening context must not be used as substitute authority; and
- the session remains bound to the current window identity.

Zero matching windows without valid fresh-opening context, more than one
matching window, or any mismatch fails closed before issuance.

## 8. Read-Only Preparation And Run-Time Revalidation

Preparation must not open a window, consume current authority, consume a
resume directive, allocate an attempt capability, append events, or invoke an
executor. Doing so before the application takes custody could leave durable
work admitted even when cancellation wins before entry.

Preparation instead performs read-only eligibility and coherence checks and
captures exact expected values. `run` remains the authority-consuming edge.
At that edge, the existing operational entry path must rehydrate current
state, recompute the invocation commitment, verify the captured bindings, and
atomically open or resume. Any change between preparation and run fails
closed through the bounded application failure projection.

This is deliberate defense against time-of-check/time-of-use drift rather
than a promise that a prepared session must later succeed.

## 9. Identity Provider Ownership

Application code must not inject redispatch or wake identity providers. The
prepared session runner must create or own the existing Core production
identity source internally.

The implementation may refactor private ownership or closure capture to make
the identity source live long enough for one consumed session. It must not
export random material, provider traits, or identity constructors through the
SPI.

Tests may inject deterministic identity sources only through `cfg(test)`
private paths.

## 10. Cancellation Failure Boundary

Before a preparation helper can issue a public cancellation handle,
`request_cancellation` must return `TrustedHostLocalApplicationFailure` or an
equally fixed payload-free cancellation failure type. It must not return
`WorkflowOsError`.

The projection must:

- use fixed stable codes;
- retain no original message, diagnostics, source, identifier, or payload;
- provide bounded Debug and Display;
- remain non-serializable; and
- prove secret-like private error text cannot cross the handle boundary.

Cancellation remains cooperative and idempotent. The projection does not add
revocation or active-attempt interruption.

## 11. Prepared Pair Contract

The prepared pair must be:

- opaque outside Core;
- non-cloneable;
- non-serializable and non-deserializable;
- Debug-redacted;
- bound to one backend instance and one exact invocation context;
- consumable into exactly one session and one handle; and
- impossible to reconstruct from identifiers after process loss.

Dropping an unrun prepared session performs no write and makes no workflow
lifecycle claim.

## 12. Failure Semantics

Preparation fails closed and returns no pair when:

- the backend cannot be read coherently;
- the run is missing or not eligible;
- immutable bundle identity is missing or mismatched;
- workflow, run, step, actor, window, invocation, or skill identity differs;
- current authority or required context is missing for a fresh opening;
- continuity state is absent when an existing window is required;
- more than one matching window exists;
- trusted-time or opening context is stale or unsupported; or
- the intended session cannot be constructed without caller-authored
  authority.

Preparation errors must use stable private Core codes and avoid sensitive
values. No partial session or handle may escape.

After issuance, run-time staleness or authority loss returns only bounded
application failure. It must not silently refresh, broaden authority, rebuild
from copied identifiers, or reinterpret failure as workflow completion.

## 13. Security And Privacy

- Validate object identity where the same backend instance is required.
- Derive locators from validated Core context rather than caller strings.
- Keep opening capability, attempt capability, locator, identity providers,
  and continuation state private.
- Redact Debug for input, prepared pair, session, and handle.
- Store no raw provider payload, command output, credentials, or source text.
- Do not serialize session, pair, handle, locator, or capability.
- Preserve exact immutable-run and invocation commitments.
- Treat feature visibility as packaging only, never as authority.
- Add no unsafe code or dependency.

## 14. Test Plan

The implementation phase must prove:

- valid fresh context prepares exactly one opaque pair without writes;
- valid existing-window context prepares exactly one opaque pair without
  consuming a resume directive;
- dropping the pair creates no state or event change;
- cancellation before `run` remains zero owner entry and zero executor entry;
- `run` revalidates and rejects state changed after preparation;
- backend substitution is rejected;
- workflow, run, step, actor, window, immutable bundle, invocation, executor,
  and skill-input mismatches are rejected;
- missing fresh-opening context and ambiguous windows are rejected;
- application callers cannot construct, clone, or serialize the pair;
- application callers cannot inject identity providers;
- cancellation request failure is payload-free and non-leaking;
- run failure remains payload-free and non-leaking;
- exactly one owner/executor entry is possible;
- default builds do not expose the SPI;
- explicit-feature builds expose only the reviewed values; and
- CLI, hosted, filesystem, PostgreSQL, provider, and default workspace
  behavior remain unchanged.

## 15. Candidate Implementation Sequence

1. Project cancellation request failure to bounded application vocabulary.
2. Add a private preparation input and opaque prepared-pair type behind the
   existing feature.
3. Add read-only preparation validation using current SQLite state and exact
   bindings.
4. Capture a one-shot runner that creates Core-owned identity providers and
   performs authoritative run-time revalidation/opening.
5. Add direct fresh, existing, stale, substitution, cancellation, and privacy
   tests.
6. Run a focused maintainer/security review.
7. Only after acceptance, implement the unpublished local-host library with
   an injected prepared pair.

The first implementation may complete steps 1 through 5 together because
they form one security boundary. It must not create the local-host package or
any operational caller.

## 16. Open Questions For Review

- Should the prepared pair itself be public behind the feature, or should
  Core expose only its two opaque parts?
- Can fresh and existing-window preparation share one private input without
  permitting ambiguous optional fields?
- Which existing current-authority source can eventually call preparation
  without reconstructing context?
- Should bounded cancellation failure reuse the session failure type or use a
  smaller dedicated enum?
- What compile-time external-consumer fixture best proves default absence and
  explicit-feature presence before the local-host crate is added?

## 17. Validation

The implementation is validated with feature-focused clippy and tests in
addition to the ordinary repository formatting, workspace clippy, workspace
test, documentation, and diff-hygiene checks.

## 18. Final Recommendation

The private preparation boundary and fresh proof are accepted. Proceed through
the separate caller-composition plan with the bounded unpublished local-host
library and prepared-pair visibility slice only. Do not add an executable,
production preparation source, operational caller, discovery, provider
behavior, writes, schemas, SDK behavior, hosted parity, or release change.
