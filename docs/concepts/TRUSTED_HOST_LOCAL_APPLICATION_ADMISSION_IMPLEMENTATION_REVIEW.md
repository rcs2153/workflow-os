# Trusted-Host Local Application Admission Implementation Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups. Proceed to bounded
application-boundary negative-path proof hardening before broader adoption.**

The implementation closes the intended runtime-composition gap without
moving authority construction into the application. Core owns admission,
immutable input resolution, policy and approval posture, explicit-zero
authority, executor commitment, and one-shot preparation. The unpublished
foreground process owns one SQLite lifetime and consumes only the opaque
prepared pair.

The implementation remains deliberately non-productized. It does not add a
released CLI path, discovery, scheduling, provider behavior, arbitrary
commands, broader profiles, automatic approval, or workflow completion.

## 2. Review Scope

This review assessed:

- Core-owned admission and provenance;
- fresh and existing-run behavior;
- explicit-zero current-authority proof;
- SQLite schema v7 immutable-bundle atomicity and upgrade posture;
- opaque prepared-pair ownership;
- replay and invocation-binding behavior;
- bounded output, errors, and privacy;
- operational-terminal versus workflow-terminal semantics;
- implementation and regression test quality; and
- documentation honesty and scope containment.

The review did not implement fixes or authorize a public CLI, additional
profiles, discovery, scheduling, provider calls, writes, hosted parity,
nested harnesses, OpenShell, schemas, examples, or release changes.

## 3. Scope Verification

The phase stayed within the accepted unpublished vertical slice. The only
application entry is the unpublished `workflow-local-host` binary. It accepts
explicit selection and lifecycle input, opens one SQLite backend, resolves the
closed docs-check profile, and delegates admission to Core.

No automatic runtime adoption, background process, state bridge, provider
mutation, arbitrary shell surface, approval grant, or supported operator
contract was introduced. The workflow remains `Running` after an operational
terminal result, so application return does not overclaim workflow completion.

## 4. Core Admission And Provenance Assessment

Core, rather than the foreground process, prepares the execution plan, loads
the validated project, evaluates pre-run policy, constructs and persists the
immutable bundle, binds the run and step, resolves the profile handler,
constructs `SkillInput`, derives the executor commitment, and prepares the
one-shot operation.

The caller cannot supply a handler, current-authority fact, capability,
execution binding, opening request, executor commitment, or prepared session.
Unsupported topology, adapters, side effects, hook input, and non-selected
skills fail before preparation.

Fresh admission publishes and validates the immutable bundle before
`RunCreated`. This ordering leaves immutable bundle storage as the first
durable record if later admission fails, but it does not authorize execution
and is consistent with the existing bundle-backed execution boundary. Exact
reuse remains create-only and fails closed on a different manifest.

## 5. Existing-Run And Replay Assessment

Existing runs are rehydrated from SQLite and accepted only in the reviewed
running or retrying states. Waiting approval and waiting external-condition
states return bounded wait outcomes; terminal runs return terminal replay;
other states return bounded blocked posture.

The path requires the durable immutable-bundle binding, exact stored bundle,
matching workflow and bundle identity, matching creator actor, matching
sensitivity and redaction posture, one immutable workflow step, the selected
skill identity, an existing operational opening, and the durable actor.

The existing path rebuilds `SkillInput` and the executor commitment, including
the supplied correlation ID and current command-contract fingerprint. The
private preparation boundary recomputes their invocation commitment and
compares it with the durable opening. A changed correlation, mapped input, or
handler contract therefore fails before execution rather than being accepted
as an exact replay.

The end-to-end test proves that exact replay does not append events or invoke
work again. Direct application-level substitution cases are not yet isolated
as dedicated tests and remain a non-blocking proof-depth follow-up.

## 6. Explicit-Zero Authority Assessment

The implementation uses the reviewed explicit-zero required-context
contract. It can register complete empty grant, availability, and governed-
context inventories only after the closed docs-check profile proves zero
requirements and binds that posture to the exact immutable execution.

Absence is not interpreted as authority. Non-empty, missing, stale,
incomplete, or conflicting requirements fail in the established contract,
source, or preparation boundaries. No authority-bearing material crosses to
the foreground process.

## 7. SQLite And Migration Assessment

SQLite schema v7 adds create-only immutable-run-bundle storage on the same
backend that owns run events and operational continuity. Publication is
transactional. Reads verify row identity, manifest root identity, canonical
definition and local-check payload decoding, and complete stored-bundle
validation.

The v6-to-v7 transition is explicit. Ordinary opening does not silently
upgrade an older database. The adapter reports the current schema through its
existing durable-state contract. This is an adapter-storage change, not a
workflow-specification schema change.

The implementation does not provide a single transaction spanning immutable
bundle publication and later run-event creation. That is an honest known
boundary, not an atomicity claim made by this phase.

## 8. Ownership, Privacy, And Error Assessment

The prepared pair remains one-shot, non-cloneable, non-serializable, and
Debug-redacted. The application owns one synchronous process call and scoped
cooperative-cancellation custody. It cannot reconstruct preparation authority.

Application failures retain only a fixed category. Standard output and error
use closed posture strings and do not expose paths, identifiers, commands,
arguments, environment values, policy text, payloads, outputs, credentials,
or Core diagnostics. Request and prepared-outcome Debug implementations redact
their binding material.

The current timestamps are explicitly supplied by the foreground owner and
validated through the existing opening and authority boundaries. This is a
bounded trusted-host assumption, not proof from a remote or cryptographic
time source.

## 9. Workflow Semantics Assessment

The application does not grant approval, reinterpret policy, append workflow
completion, create a report artifact, or mutate provider state. Approval
remains waiting, policy denial remains denied, unsupported state remains
blocked, and operational terminal remains distinct from workflow terminal.

This distinction is both documented and exercised: the foreground proof
observes `operation.terminal` while the durable workflow status remains
`Running`.

## 10. Test Quality Assessment

The test suite provides strong lower-level coverage for immutable bundle
validation, explicit-zero authority, SQLite opening and continuity,
cancellation, one-shot ownership, drift, substitution, bounded failures, and
privacy. The production-shaped binary test proves one successful fresh
admission, durable state, exact immutable bundle recovery, and idempotent
replay through the real unpublished process.

The following application-boundary cases are not independently exercised:

- approval wait;
- policy denial;
- unsupported workflow topology;
- changed actor, correlation, command contract, bundle, or selected step;
- incompatible SQLite schema and unhealthy state;
- cancellation before entry; and
- output and Debug non-leakage under failing inputs.

These gaps do not block the single unpublished no-write slice because the
owning lower-level boundaries are covered and the application exposes no
broader authority. They should be closed before another profile, public
surface, scheduler, or operational adoption is considered.

The implementation report also discloses one pre-existing intermittent
time-window test and one root-target integration hang. Both passed on clean
rerun or the already-established isolated target. The review does not treat
those reruns as evidence that the underlying flake and target-contention
follow-ups are resolved.

## 11. Documentation Review

The plan, roadmap, and implementation report accurately describe the first
slice as unpublished, local, synchronous, no-write, and non-operational. They
state that no discovery, scheduling, approval broadening, provider behavior,
public CLI contract, hosted parity, nested harness, OpenShell integration, or
workflow completion was added.

The end-of-phase report records validation, the intermittent observations,
the governed phase identity, and the exact remaining limitations without
inflating application readiness.

## 12. Blockers

None for acceptance of the implemented unpublished docs-check admission
slice.

Broader adoption remains blocked on separate planning, review, and proof. In
particular, this review does not authorize another profile, automatic
discovery, background scheduling, a released CLI path, provider mutation, or
workflow completion.

## 13. Non-Blocking Follow-Ups

- Add application-level negative tests for approval wait, policy denial,
  unsupported topology, actor and invocation substitution, SQLite
  incompatibility, cancellation, and non-leaking failure output.
- Preserve the immutable-bundle-before-run durability posture explicitly in
  future recovery or cleanup design.
- Investigate the disclosed time-window flake independently if it recurs.
- Keep integration validation on an isolated Cargo target while root-target
  contention remains observable.
- Require a separately reviewed owner for workflow completion after an
  operational terminal result.

## 14. Recommended Next Phase

Implement bounded application-boundary negative-path proof hardening for the
existing unpublished docs-check slice, then perform focused review.

Do not add another profile, public configuration, discovery, scheduling,
provider calls, writes, automatic approval, workflow completion, hosted
parity, nested harnesses, OpenShell, schemas, examples, or release changes.

## 15. Validation

The implementation validation reviewed here completed successfully:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace` on the complete rerun;
- `npm run check:docs`;
- `npm run check:integrations` with the established isolated Cargo target;
- dogfood project validation; and
- `git diff --check`.

This review adds documentation only. Its own documentation and diff checks
are recorded when the governed review phase closes.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1791450133744165000-2`
- approval:
  `approval/run-1791450133744165000-2/review-scope-approved`
- presentation: `presentation/fafcddc0011e47a4`
- approval outcome: granted under standing delegated-maintainer authority
  through persisted presentation proof
- approved boundary: focused implementation and security review only
- phase status: completed
- event summary: 39 ordered events, one approval, no retry or escalation;
  approval-presentation proof was enforced
- validation summary: `npm run check:docs` and `git diff --check` passed;
  implementation validation remained unchanged
- out-of-kernel work: source, tests, Cargo, plan, roadmap, and implementation
  report inspection; review authoring; and documentation validation
- report posture: no runtime WorkReport artifact was generated or persisted
