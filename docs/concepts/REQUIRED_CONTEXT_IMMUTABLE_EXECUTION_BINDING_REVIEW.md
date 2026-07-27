# Required Context Immutable Execution Binding Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation provides a narrow, deterministic, payload-free commitment
between a validated stored immutable run bundle and one exact
required-context contract consumption scope. It does not overstate that
commitment as current authority, a dereference lease, or runtime execution.

## 2. Scope Verification

The phase stayed within its approved model-only scope.

Implemented:

- a versioned required-context execution-binding model;
- construction from a validated `StoredImmutableRunBundle`;
- exact workflow, run, step, actor, contract, sensitivity, and time binding;
- fixed-width framed deterministic hashing;
- fail-closed validation and deserialization;
- redaction-safe Debug behavior;
- focused tests; and
- honest roadmap and phase documentation.

Not introduced:

- context-target or source-payload dereference;
- current capability, grant, or availability resolution;
- executor or runtime integration;
- persistence, events, audit projection, authority receipts, or artifacts;
- workflow, immutable-bundle, schema, SDK, CLI, UI, or example changes;
- providers, connectors, OpenShell, sandbox execution, SideEffects, or writes;
- hosted administration, reasoning lineage, or release changes.

## 3. Model Assessment

`RequiredContextExecutionBinding` is appropriately minimal. It commits:

- binding algorithm version;
- immutable bundle identity, version, and root hash;
- workflow, run, and step identity;
- actor identity;
- harness contract identity, version, and content hash;
- maximum sensitivity;
- binding time; and
- deterministic binding hash.

The model stores no context target payload. Private fields and read-only
accessors preserve the aggregate validation boundary.

## 4. Immutable Provenance Assessment

Construction starts from `StoredImmutableRunBundle`, which is produced by the
validated create-only bundle store boundary. The constructor then locates the
single canonical workflow record and independently verifies its ID, version,
schema, and source content hash against the stored manifest.

The requested step must exist in that frozen workflow. Workflow and run
identity are derived from the manifest rather than accepted as independent
caller assertions.

This is the correct provenance boundary for a pre-consumption commitment. It
does not imply that the harness contract is itself a canonical immutable-bundle
record; the exact contract ID, version, and content hash are committed
separately and that limitation is documented.

## 5. Validation Assessment

Validation fails closed for:

- a missing or duplicate canonical workflow record;
- workflow record and manifest disagreement;
- a step absent from the immutable workflow;
- unknown sensitivity;
- a binding time before immutable bundle creation;
- an unknown binding version; and
- any serialized-field substitution that invalidates the binding hash.

Errors use stable `required_context.execution_binding.*` codes and fixed
messages without rejected identities, hashes, timestamps, paths, payloads, or
secret-like values.

The sensitivity value is intentionally a future-consumer ceiling, not a claim
that every contract requirement is consumable. A future time-of-use helper
must block any required target whose sensitivity exceeds that ceiling.

## 6. Commitment And Authority Assessment

The model correctly remains a commitment, not authority.

It does not prove:

- a scoped grant exists or remains active;
- a target is currently available;
- policy, approval, evidence, or checks remain satisfied;
- the caller supplied the complete current authority fact set;
- a payload may be dereferenced; or
- any work was executed.

A deserialized binding proves only internal wire consistency. It must not be
accepted as authoritative merely because its public deterministic hash
validates. A future consumer must compare it to the validated stored immutable
bundle and re-resolve the complete current authority fact set in the same
consumption call.

## 7. Hash And Serde Assessment

The hash is domain separated by a versioned algorithm identifier and uses
fixed-width length framing for every label and value. It covers every retained
field except the derived hash itself.

Valid values round trip through serde. Unknown versions and internally
inconsistent wire values fail closed with bounded errors.

The implementation is suitable for an internal preview model. Before schema,
SDK, persistence, or cross-version compatibility exposure, add a fixed known
hash vector and an explicit framing-collision regression test so the algorithm
cannot drift unnoticed.

## 8. Privacy And Redaction Assessment

The serialized model contains only typed identities, hashes, an enumerated
sensitivity ceiling, and a timestamp. It has no fields for raw source,
provider, parser, command, environment, credential, log, artifact, or
context-target payloads.

Debug output redacts workflow, run, step, actor, harness, timestamp, contract
hash, and binding hash. The retained immutable bundle binding has its own
redaction-safe Debug implementation. Deserialization errors do not echo
rejected wire values.

## 9. Test Quality Assessment

Focused tests cover:

- deterministic valid construction;
- immutable identity derivation;
- exact contract identity and hash;
- absent step;
- unknown sensitivity;
- binding time before bundle creation;
- bundle, contract, actor, and sensitivity substitution;
- serde round trip;
- serialized tampering;
- non-leaking secret-like rejection;
- Debug non-leakage; and
- payload-free serialized shape.

Adjacent required-context and immutable-bundle builder/store tests pass.

Non-blocking test gaps:

- no fixed known-vector assertion for the v1 binding hash;
- no explicit ambiguous-framing collision regression; and
- no future-consumer test yet proves revalidation against the store because
  runtime consumption is correctly out of scope.

## 10. Documentation Review

The roadmap, immutable-run/time-of-use plan, contract-consumption plan,
capability-projection plan, and phase report accurately state:

- the immutable binding core model is implemented;
- the binding is payload-free and non-authoritative;
- current authority re-resolution and complete-set discovery remain open;
- dereference and executor integration are not implemented;
- persistence, events, schemas, CLI behavior, providers, OpenShell, sandbox
  execution, SideEffects, and writes remain unsupported by this phase.

No documentation overclaims current runtime capability.

## 11. Validation

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test required_context_execution_binding --test required_context --test immutable_run_bundle_store --test immutable_run_bundle_builder`:
  passed, 44 tests.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed before this review.
- `git diff --check`: passed before this review.

## 12. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785141057148044000-2`
- approval ID:
  `approval/run-1785141057148044000-2/review-scope-approved`
- presentation ID: `presentation/fdfac8914a77c0f1`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- out-of-kernel work: source, tests, plans, and report inspection plus this
  review were performed by the delegated maintainer; the kernel governed scope
  and approval but did not edit files, invoke tools, or mutate git

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Add a fixed v1 known-vector and framing-collision regression before
  compatibility exposure.
- Keep deserialized bindings non-authoritative until a consumer compares them
  with the validated stored bundle.
- Decide whether canonical harness contract records belong in a future
  immutable-bundle taxonomy phase.
- Define the complete current grant and availability fact-set source before
  implementing authoritative time-of-use readiness.

## 15. Recommended Next Phase

Proceed to **required-context current authority fact-set planning**.

That phase should define the complete validated source of current grants,
revocation, expiry, availability, accepted policy decisions, approvals,
evidence, checks, sensitivity, and SideEffect constraints needed by a future
same-call time-of-use helper.

Do not implement dereference, runtime consumption, providers, OpenShell,
sandbox execution, persistence, schemas, CLI behavior, SideEffects, writes,
hosted behavior, reasoning lineage, or release changes.
