# Trusted-Host Local Application Preparation Source Plan

Status: planning only. The bounded `workflow-local-host` library and its
focused implementation review are complete. This document defines the next
non-operational Core-to-library bridge; it implements nothing.

## 1. Executive Summary

Core can privately prepare one opaque trusted-host local application session,
and the unpublished `workflow-local-host` package can consume that prepared
pair. No lawful cross-crate source currently connects those boundaries. Making
the private preparation function public would expose authority assembly.
Passing identifiers, locators, backend handles, executor internals, or current
authority into the local-host package would create a second execution truth.

The next implementation should add one opaque, Core-issued, one-shot
`TrustedHostLocalApplicationPreparationSource`. Core alone constructs the
source from an already-resolved private preparation input. The local-host
library may consume it, causing Core to perform the existing read-only
preparation and return only the existing prepared pair or bounded
application failure.

The first source slice remains non-operational. It adds no public resolver,
production issuer call site, embedding application, discovery, scheduling, or
automatic execution.

## 2. Goals

- Connect private Core preparation to the unpublished local-host composition
  library without exporting preparation inputs.
- Keep Core authoritative for current context, preparation, revalidation,
  authority use, state, and lifecycle truth.
- Make source custody one-shot, lifetime-bound, non-cloneable, non-serde, and
  Debug-redacted.
- Project preparation failure to the existing fixed, payload-free application
  failure vocabulary before it crosses the Core boundary.
- Preserve the prepared pair's unconstructible and one-shot properties.
- Prove that local-host can consume one opaque source without accepting
  locator, backend, authority, executor, or skill-binding parameters.
- Keep actual source issuance and operational adoption separate.

## 3. Non-Goals

The first implementation must not add:

- a public resolver or factory that accepts caller-authored identifiers;
- a production issuer call site or embedding application;
- a binary, CLI command, SDK method, workflow field, schema, or runtime config;
- run discovery, project scanning, queue polling, scheduling, redispatch
  selection, background tasks, or automatic invocation;
- public backend, locator, authority, current-context, executor, or skill-input
  assembly;
- provider reads or mutations, OpenShell, nested harnesses, or broader writes;
- signal installation, active-attempt interruption, process recovery, or
  hosted parity;
- automatic approval, inferred authority, or capability broadening; or
- release posture or stable compatibility promises.

## 4. Accepted Starting Boundary

Behind `trusted-host-application-spi`, Core currently provides:

- private `TrustedHostLocalApplicationPreparationInput` and fresh/existing
  posture variants;
- private `prepare_trusted_host_local_application_session`;
- public but unconstructible
  `TrustedHostLocalApplicationPreparedSession`;
- one-shot session, scoped cancellation handle, bounded outcomes, and bounded
  failures; and
- direct tests for read-only preparation, exact bindings, substitution,
  cancellation, staleness, and consumed execution.

The unpublished local-host package currently accepts only the prepared pair.
It cannot invoke preparation or construct authority.

## 5. Dependency And Ownership Constraint

`workflow-local-host` depends on `workflow-core`; Core must not depend back on
the local-host package. The source therefore belongs in Core and returns the
Core prepared pair. The local-host package consumes the source through a
concrete convenience method and wraps the resulting pair.

The source is not a service locator, scheduler, backend facade, or process
owner. It is one in-memory custody object for one already-resolved preparation
attempt.

## 6. Core Source Model

Add a feature-gated source conceptually shaped as:

```text
TrustedHostLocalApplicationPreparationSource<'a>
  - privately owns one TrustedHostLocalApplicationPreparationInput<'a>
  - has no public constructor
  - is non-cloneable and non-serializable
  - exposes consuming prepare(self)

prepare(self)
  -> Result<TrustedHostLocalApplicationPreparedSession<'a>,
            TrustedHostLocalApplicationFailure>
```

Exact names may follow repository conventions. `prepare` must call the
existing private preparation function once and map every private Core error to
the existing bounded application failure. No raw `WorkflowOsError` may cross
the source boundary.

The source must be `#[doc(hidden)]` or equivalently marked unstable and must
use fixed redacted Debug output.

## 7. Issuance Boundary

Core may add one crate-private constructor or issuance helper that accepts the
existing private preparation input by value and returns the opaque source.
That helper exists to prove ownership and source semantics inside Core tests.

The first implementation must not call the issuer from production code. This
is deliberate. An actual issuer call site must later prove where all of these
already-resolved facts come from:

- exact SQLite backend instance;
- workflow, run, step, window, and actor binding;
- immutable run bundle;
- fresh or existing continuation posture;
- current authority and required context for a fresh opening;
- exact attempt executor and invocation commitment;
- validated skill input; and
- opening persistence posture when required.

Until that call site is separately planned and reviewed, no external caller
can obtain a source.

## 8. Local-Host Consumption API

Add one concrete method to `workflow-local-host`, conceptually:

```text
LocalHostPreparedOperation::prepare(
  source: TrustedHostLocalApplicationPreparationSource
) -> Result<LocalHostPreparedOperation,
            TrustedHostLocalApplicationFailure>
```

The method consumes the source, asks Core to prepare once, and delegates to
the existing `from_prepared` constructor. It accepts no additional fields.
`from_prepared` may remain available as unstable composition vocabulary unless
the implementation review finds that narrowing it is required.

The package must not inspect, refresh, retry, log, persist, or reinterpret the
source or preparation failure.

## 9. Current-Authority Boundary

The source is not itself current authority. It owns one exact private request
to read and validate current authority through the existing preparation path.
Successful source issuance is weaker than successful preparation, and
successful preparation is weaker than successful consumed execution.

Preparation remains read-only. Session consumption remains the authoritative
revalidation and authority-use edge. State or authority drift after source
issuance or preparation must fail closed rather than refresh or broaden the
request.

## 10. Failure And Atomicity

- Source construction performs no state write or authority use.
- Dropping an unconsumed source performs no work and makes no lifecycle claim.
- Consuming a source invokes preparation at most once.
- Preparation failure returns no pair and only a bounded application failure.
- Dropping a successfully prepared but unrun operation remains zero-write.
- No source or pair can be reconstructed from identifiers after process loss.
- The package must not retry failed preparation automatically.

## 11. Privacy And Redaction

- Source Debug exposes only a fixed type name and `[REDACTED]` binding marker.
- Source, input, pair, operation, and cancellation values remain non-serde.
- No workflow identity, locator, authority fact, state payload, path, command
  output, provider value, credential, or raw Core error crosses the boundary.
- Compile errors, Display, and Debug tests must use secret-like fixtures and
  prove non-leakage.

## 12. Feature And Compatibility Posture

The source is available only with the existing non-default SPI feature. Core,
CLI, and hosted default package graphs must remain unchanged. The local-host
package intentionally activates the feature. Workspace feature unification
remains expected.

The source is unstable application SPI, not a public product contract. No
schema, CLI, SDK, or semver compatibility promise is added.

## 13. Test Plan

### 13.1 Core tests

- source has no public construction path;
- source creation is zero-write;
- source drop is zero-write;
- source consumption invokes preparation once;
- private preparation errors map exhaustively to bounded failures;
- no pair escapes on failure;
- source Debug is redacted;
- source cannot be cloned, serialized, or deserialized; and
- existing fresh/existing, substitution, staleness, cancellation, and
  consumed-execution proofs still pass.

### 13.2 Local-host tests

- one opaque source can produce one operation through a private test seam;
- preparation failure propagates unchanged and creates no operation;
- no automatic retry occurs;
- source and operation cannot be consumed twice; and
- the public method accepts no caller-authored binding or authority fields.

### 13.3 External and package checks

- default Core consumers cannot name the source without the feature;
- an explicit-feature consumer can type-check source-to-operation composition
  without constructing a source;
- CLI and hosted package graphs do not request the feature;
- no production call site invokes source issuance or source consumption; and
- workspace tests intentionally cover the feature-bearing package.

## 14. Validation Commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- workspace rustdoc with warnings denied
- package-specific feature graph checks for Core, CLI, hosted, and local-host
- `npm run check:docs`
- `git diff --check`

## 15. First Implementation Scope

The first implementation should contain only:

1. the opaque Core source and crate-private issuer;
2. exhaustive bounded error projection through the source;
3. one local-host `prepare` composition method;
4. Core, package-private, external, and compile-fail tests;
5. honest roadmap and plan updates; and
6. an implementation report.

It must stop before adding any production issuer or caller.

## 16. Subsequent Sequence

1. Implement the non-operational source bridge.
2. Perform a focused maintainer/security review.
3. Plan one actual Core-owned issuer call site from already-resolved current
   authority and execution context.
4. Implement and review that issuer without discovery or scheduling.
5. Plan one embedding application boundary that receives the opaque source and
   invokes the local-host package explicitly.
6. Only after those reviews, consider process control, signal handling, or
   operational adoption.

Provider mutation broadening, OpenShell, nested harnesses, hosted parity, and
public configuration remain separate roadmap decisions.

## 17. Alternatives Rejected

- **Make private preparation public:** exposes authority assembly and private
  Core errors.
- **Pass private preparation fields to local-host:** creates caller-authored
  execution truth and leaks Core internals.
- **Add a public resolver taking IDs:** becomes discovery and permits arbitrary
  target selection before an authority source exists.
- **Have Core depend on local-host:** creates a dependency cycle and reverses
  ownership.
- **Add a public test factory:** makes forgeable production-looking authority
  for test convenience.
- **Adopt the source in CLI or hosted now:** selects the wrong lifecycle and
  backend semantics before an embedding boundary is designed.
- **Combine source, issuer, and application in one phase:** hides the most
  security-sensitive authority transition inside a large operational change.

## 18. Open Questions For Review

- Should `from_prepared` remain visible after source composition exists, or be
  narrowed in a later breaking SPI cleanup?
- Should the Core source's consuming method be named `prepare`, `into_prepared`,
  or use a sealed helper to keep the ordinary API surface smaller?
- Is a crate-private issuer wrapper meaningful enough, or should source
  construction occur directly at the later accepted issuer call site?
- Which checked-in negative fixture best proves default consumers cannot name
  the source without relying only on manual Cargo graph inspection?
- Can lifetime borrowing remain sufficient without accidental `Send`, `Sync`,
  or `'static` claims?

## 19. Final Recommendation

Proceed next with a focused maintainer/security review of this plan. If
accepted, implement only the opaque non-operational source bridge and tests.
Do not add a production issuer, embedding caller, discovery, scheduling,
provider behavior, OpenShell, nested harnesses, hosted adoption, or release
change.

## 20. Governed Planning Record

- workflow: `dg/d`
- run: `run-1791389192666100000-2`
- approval: `approval/run-1791389192666100000-2/planning-approved`
- presentation: `presentation/b75989c297190ce9`
- presentation hash:
  `b75989c297190ce98ff79ecf7ae2717c4262a23ebf37b841af46142609ae75a8`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: preparation-source planning and roadmap update only
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture and source inspection, plan and roadmap
  authoring, documentation validation, and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
