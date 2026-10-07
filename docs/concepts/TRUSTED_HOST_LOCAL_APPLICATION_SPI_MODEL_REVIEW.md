# Trusted-Host Local Application SPI Model Review

## 1. Executive Verdict

**Needs blocker fix before session preparation or application composition.**

The feature boundary, opaque construction, one-shot ownership, cancellation
custody, fixed success outcomes, and scope containment are sound. The blocker
is the public failure surface: `TrustedHostLocalApplicationSession::run`
returns the complete `WorkflowOsError`, while the accepted topology permits
only a stable error code to cross the application SPI.

`WorkflowOsError` publicly exposes message text, diagnostics, Debug, Display,
and serde. The current implementation therefore cannot guarantee that a
future backend, preparation, or owner failure remains bounded and non-leaking
at the cross-crate boundary.

## 2. Scope Verification

The implementation stayed within the approved Core SPI model and visibility
slice.

It added one non-default feature, an opaque session, cancellation custody,
fixed outcome vocabulary, redacted Debug, compile-fail negative examples,
focused tests, and documentation.

It did not add a preparation helper, local-host package, executable, runtime
caller, discovery, polling, state bridge, hosted integration, provider
behavior, writes, schemas, SDK behavior, or release changes.

## 3. Feature Isolation Assessment

`trusted-host-application-spi` is absent from the default feature set, adds no
dependency, and is not enabled by another workspace package. The SPI module
and root re-exports are both feature-gated. Default CLI and hosted builds do
not activate the surface.

This is an appropriate packaging guard, not an authority boundary. The code
and documentation state that distinction correctly.

## 4. Construction And Authority Assessment

Production code has no constructor for either the session or cancellation
handle. Their fields are private, and the only constructors are compiled for
Core tests. Feature enablement therefore does not let an external caller
assemble an owner, locator, opening capability, attempt capability, or
continuation disposition.

The test-only owner wrapper proves the intended composition without creating
production ambient authority. A future preparation helper must keep owner
construction and outcome projection private to Core and return both opaque
values only after current-state validation succeeds.

## 5. One-Shot And Lifetime Assessment

The session owns a `FnOnce` runner and consumes `self` on `run`. It does not
implement Clone, Serialize, or Deserialize. The lifetime prevents the current
shape from outliving its borrowed Core dependencies.

The compile-fail examples directly protect clone and serde exclusion when the
feature is enabled. No restart reconstruction or identifier-based session
recreation surface exists.

## 6. Cancellation Assessment

The public handle wraps the existing cooperative timer cancellation handle.
It is not cloneable and does not expose its state. Requests are idempotent.
The feature-level owner test proves cancellation before entry causes no
executor entry, identity allocation, or snapshot mutation.

The SPI does not claim active-attempt interruption or revocation. Underlying
owner tests cover cancellation during waits and after admission, but those
semantics should be repeated through the prepared session pair when the
preparation helper exists.

## 7. Outcome Assessment

Successful owner results are reduced to fixed classifications:

- canceled before entry;
- entry stopped for await-condition, blocked, or terminal posture; and
- continuation stopped for canceled, blocked, terminal, unsupported wait, or
  exhausted wake budget.

The outcome does not expose workflow, run, step, actor, window, capability,
input, deadline, path, provider, credential, or raw state values. It also does
not reinterpret owner posture as workflow completion or failure.

The private owner-to-public mapping currently exists only in test builds.
That is acceptable while no production constructor exists. The blocker fix
or later preparation helper must move the mapping into private production
code; application code must never translate private owner outcomes.

## 8. Failure Boundary Blocker

The accepted topology requires the cross-crate boundary to expose fixed
outcomes or a stable `WorkflowOsError` code. Instead, `run` returns
`Result<TrustedHostLocalApplicationOutcome, WorkflowOsError>`.

`WorkflowOsError` contains and publicly exposes:

- a kind;
- an arbitrary string code;
- a human-readable message;
- structured diagnostics; and
- derived Debug, Serialize, and Deserialize behavior.

Current trusted-host paths generally use bounded error text, and skill-handler
failure is reduced before reaching this surface. That implementation detail is
not a durable cross-crate guarantee. Backend errors and future preparation
paths can evolve independently, while the public return type permits their
messages and diagnostics to cross the SPI unchanged.

The fix must introduce a fixed, payload-free application failure vocabulary or
equivalent stable-code wrapper inside the feature boundary. Core must map
private errors to that vocabulary before returning from `run`. The public
failure must not retain the original message, diagnostics, source, path,
identifier, payload, or serialized error object.

## 9. Privacy And Debug Assessment

Session and cancellation Debug output contains only redacted markers. Outcome
Debug contains fixed enum names. No raw state, payload, dependency, or unsafe
code was added.

Privacy is accepted for values and successful outcomes. Failure privacy is not
accepted until the broad `WorkflowOsError` return is replaced or safely
projected.

## 10. Test Quality Assessment

Existing focused coverage proves:

- one-shot runner consumption;
- bounded successful outcomes;
- redacted session and handle Debug output;
- idempotent cancellation;
- zero-write pre-entry cancellation through the real private owner; and
- compile-time clone and serde rejection.

The blocker fix needs regression tests that inject an internal error carrying
secret-like message and diagnostic text, then prove public Debug, Display, and
any serialization surface expose only fixed vocabulary. It should also prove
deterministic mapping for every accepted internal error kind.

Non-blocking follow-ups:

- add an external-consumer feature-matrix fixture before another crate depends
  on the SPI; and
- repeat during-wait and post-admission cancellation tests through the
  prepared session pair.

## 11. Documentation Review

The roadmap, topology plan, and implementation report accurately state that
the SPI is feature-gated, construction-restricted, and not operationally
adopted. They do not overclaim an application package, source, command,
discovery, bridge, provider behavior, or write support.

The implementation report describes the current return as a stable Core error
but does not establish a bounded cross-crate failure contract. This review
records that gap without rewriting the original report.

## 12. Blockers

One blocker:

- replace or project the public `WorkflowOsError` return into a fixed,
  payload-free, redaction-safe application failure boundary before adding a
  production preparation helper or application package.

## 13. Non-Blocking Follow-Ups

- Add external-consumer compile coverage for default-feature absence and
  explicit-feature presence before cross-crate adoption.
- Keep private owner-to-public outcome mapping in Core production code.
- Test cancellation during waits and after admission through the future
  prepared session pair.
- Continue describing the future local-host library as a composition boundary
  until a real process embeds it.

## 14. Recommended Next Phase

Implement the trusted-host local application SPI bounded-failure blocker fix.

The phase should add only a fixed application failure type and private mapping
from internal errors, update `run` to return that type, add non-leakage and
mapping tests, and update the report and roadmap. It must not add the
preparation helper, local-host package, executable, caller, discovery, state
bridge, hosted parity, provider behavior, writes, schemas, SDK behavior, or
release changes.

## 15. Validation

- repository and feature-activation inspection: passed;
- public construction-path inspection: passed;
- private owner and cancellation-path inspection: passed;
- feature-focused clippy, unit, private-owner wrapping, and compile-fail doc
  tests: passed;
- `cargo fmt --all --check`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791379085603544000-2`
- approval: `approval/run-1791379085603544000-2/review-scope-approved`
- presentation: `presentation/9f9259d80ac01ea6`
- presentation hash:
  `9f9259d80ac01ea65aa86c0232d7892aa83dadd6cd04bb1878a647822629d06f`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused SPI maintainer and security review only
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval presentation enforcement: proof enforced with one persisted record
  and a matching approval-event marker
- validation summary: feature-focused Rust checks, formatting, docs checks, and
  diff checks passed
- out-of-kernel work: source, feature graph, tests, plans, and reports were
  inspected and this review was authored outside the kernel

## 17. Fix-Forward Status

The blocker is now implemented for review. The public session returns the
fixed `TrustedHostLocalApplicationFailure` vocabulary, and private Core error
messages, diagnostics, sources, identifiers, and payloads are discarded before
the result crosses the feature boundary. This note does not erase the original
finding or constitute acceptance of the fix.
