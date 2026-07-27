# Required Context Contract Consumption Report

## 1. Executive Summary

Workflow OS now has a domain-neutral required-context contract model and a pure
helper that consumes exact governed context projections without treating a
declaration as authority or a stable reference as payload access.

The model binds canonical typed requirements to a harness contract ID, version,
and content hash. Required gaps block consumption, optional gaps remain
explicit, and undeclared projected context fails closed.

## 2. Scope Completed

- Added bounded `RequiredContextRequirementId`.
- Added exact required and optional obligation vocabulary.
- Added typed requirements carrying one stable target, exact access level, and
  known sensitivity ceiling.
- Added canonical `RequiredContextContractBinding` with deterministic content
  hashing over contract identity, version, and every requirement field.
- Added deterministic payload-free satisfaction and gap records.
- Added explicit `RequiredContextConsumptionContext` carrying the independently
  declared actor, workflow, run, step, harness, and evaluation time.
- Added `RequiredContextConsumptionResult` with satisfied or blocked posture.
- Added pure `consume_required_context`.
- Added read-only execution-context accessors to the existing governed context
  projection so exact source equality can be validated.
- Added validated serde, redaction-safe Debug, stable errors, and focused tests.

## 3. Scope Explicitly Not Completed

- No context, evidence, event, report, handoff, source, SideEffect, or artifact
  payload dereference.
- No repository or source inspection.
- No executor or runtime integration.
- No immutable-run-bundle consumption.
- No persistence, workflow events, audit records, or authority receipts.
- No schemas, SDKs, CLI behavior, UI, or examples.
- No connectors, provider invocation, OpenShell integration, process execution,
  network access, credential injection, SideEffect execution, or writes.
- No hosted administration, enterprise identity, reasoning lineage, or release
  posture changes.

## 4. Model And Helper Summary

`RequiredContextRequirement` uses an exact existing
`GovernedContextReferenceTarget`; there is no generic target string. It also
records one exact `GovernedContextAccessLevel`, required or optional obligation,
and a known sensitivity ceiling.

`RequiredContextContractBinding` is initially harness-specific. Its constructor
canonicalizes requirements and computes a `SpecContentHash` using fixed-width
length framing. Deserialization validates canonical order and recomputes the
hash.

The pure consumer accepts the immutable binding, one independently declared
execution context, and one or more projections, with at most one projection per
access level. Every projection must exactly match the declared actor, workflow,
run, step, harness, and evaluation time, and the context harness must match the
contract. The result retains the declared context and rechecks these equalities
during validation and deserialization.

## 5. Exact Consumption Semantics

- Contract and projection target sets must be exactly equal.
- Target and access-level matches are exact.
- `bounded_metadata` does not satisfy `reference_only`.
- Duplicate requirements, targets, access-level projections, or projected
  targets fail closed.
- Extra projected targets fail closed as ambient overexposure.
- Available authorized entries within both sensitivity ceilings satisfy a
  requirement.
- Required gaps produce `blocked`.
- Optional gaps remain explicit while the overall posture may remain
  `satisfied`.
- Serialized results retain the source contract and projections and recompute
  every derived satisfaction, gap, and posture during deserialization.

## 6. Authority Boundary

A contract describes required context; it does not issue a grant. Consumption
requires already validated capability-backed governed projections.

Availability does not imply authority. Approval does not manufacture authority.
A satisfied result is not a lease and does not authorize target dereference.
Any later access must re-resolve current authority, policy, approval,
evidence/check, availability, sensitivity, and immutable execution context at
time of use.

## 7. Privacy And Redaction

- The model stores stable typed references and bounded posture only.
- Debug output redacts contract identity, content hash, requirement IDs, target
  IDs, and source projection identities.
- Secret-like requirement IDs fail with static non-leaking errors.
- Unknown wire vocabulary fails with static errors that do not echo rejected
  values.
- Serialization has no field for raw provider payloads, command output, source
  contents, parser payloads, environment values, credentials, authorization
  headers, or private keys.

## 8. Test Coverage

Focused tests cover:

- exact required reference satisfaction;
- separate exact access-level projections;
- required unavailable context blocking;
- explicit non-blocking optional gaps;
- declaration without authority;
- rejection of extra projected context;
- rejection of overbroad access;
- rejection of projections from a different actor, workflow, run, step,
  harness, or evaluation time;
- requirement sensitivity ceilings;
- contract and result serde tampering;
- secret-like IDs and unknown wire values without leakage;
- payload-free Debug and serialization; and
- adjacent capability-authority and governed-context regression suites.

## 9. Commands And Results

- `cargo fmt --all`: passed.
- `cargo clippy -p workflow-core --test required_context -- -D warnings`:
  passed.
- `cargo test -p workflow-core --test required_context`: passed, 12 tests.
- `cargo test -p workflow-core --test capability_authority --test governed_context_access --test required_context`:
  passed, 71 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 10. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1785130420321493000-2`
- approval ID:
  `approval/run-1785130420321493000-2/implementation-approved`
- presentation ID: `presentation/980f061e29f1cdd4`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- out-of-kernel work: source and test edits plus validation commands were
  performed by the delegated maintainer; the kernel governed scope and
  approval but did not edit files, invoke tools, or mutate git

## 11. Product And Feedback Reconciliation

Fresh-pull evaluation describes current Workflow OS accurately as a coherent,
honest local governance kernel that remains more kernel than turnkey execution
platform. This phase strengthens that integrity boundary: required context is
explicit and enforceable without pretending references are payload access.

The evaluator's recommendation to reduce low-risk ceremony remains addressed by
the separate proportional-governance and quiet-success lane. Quiet presentation
may reduce interruption, but it cannot weaken a required-context failure.

OpenShell remains a promising optional containment provider after Workflow OS
resolves authority and required context. Workflow OS should not fork or absorb
that runtime surface unless upstream blocks essential lifecycle, effective
policy, result, and evidence hooks. This phase does not authorize integration.

## 12. Remaining Limitations

- The typed contract is not yet part of authored workflow or harness schemas.
- Existing name-only harness requirements remain compatibility vocabulary.
- The contract is not yet bound to an immutable run bundle.
- No time-of-use re-resolution or freshness policy exists.
- No audited dereference or authority receipt exists.
- Projection completeness remains relative to the caller-supplied candidate
  set.
- No runtime consumer prevents ambient workspace access yet.

## 13. Recommended Next Phase

Perform a focused maintainer review of the required-context execution-binding
blocker fix documented in
[Required Context Contract Consumption Blocker Fix Report](REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_BLOCKER_FIX_REPORT.md).

After acceptance, plan immutable-run-bundle binding and time-of-use
re-resolution before any context dereference, executor integration, sandbox
provider, or runtime consumer.
