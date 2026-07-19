# Runtime Proportional-Governance Assessment Binding Report

## 1. Executive Summary

Workflow OS now has a validated payload-free model for binding one accepted
immutable-bundle proportional-governance assessment set to workflow/run and
bundle identity. An additive idempotent event can retain that binding in a run
snapshot before validation and project bounded posture into the generic audit
record.

This phase adds model and event vocabulary only. It does not establish the
binding before `RunCreated`, automatically emit the event, persist volatile
runtime facts, or enforce the assessment in an executor.

## 2. Scope Completed

- Added `GovernanceAssessmentBindingVersion`.
- Added `GovernanceAssessmentSetAlgorithm` and reused it in aggregate
  fingerprint derivation.
- Added validated `GovernanceAssessmentBinding` construction from one stored
  immutable bundle and its accepted assessment set.
- Added additive `GovernanceAssessmentBound` workflow-event vocabulary.
- Added optional backward-readable snapshot retention.
- Added bounded generic audit projection and reference-only redaction posture.
- Added stable non-leaking validation and runtime error codes.
- Added focused model, serde, runtime-event, legacy-compatibility, audit, and
  privacy tests.

## 3. Scope Explicitly Not Completed

No executor establishes or emits the binding. No binding is persisted before
`RunCreated`. No execution, approval, denial, retry, or resume behavior consumes
the model. No runtime-fact freshness proof, schema, CLI, UI, provider call,
provider write, new mutation family, automatic approval, enterprise
administration, or default runtime behavior was added.

## 4. Binding Model

The binding records:

- binding-model version;
- assessment-set algorithm;
- workflow and run identity;
- immutable run-bundle ID, version, and integrity root;
- aggregate assessment-set fingerprint;
- bounded nonzero step count;
- strictest execution disposition;
- strictest disclosure requirement;
- aggregate deterministic fact completeness.

Construction validates exact bundle/set identity and derives aggregate posture
from the complete ordered assessment set. Serialized input fails closed for an
empty or oversized set and for blocking or denied execution paired with quiet
disclosure.

The assessment set retains the exact `ImmutableRunBundleBinding` used during
derivation. Binding construction requires exact equality with the supplied
stored bundle, preventing a fingerprint derived in one valid store from being
paired with a different bundle root that reuses the same workflow/run IDs.

## 5. Event And Snapshot Boundary

`GovernanceAssessmentBound` requires an idempotency key, exact run/workflow and
immutable-bundle identity, and a run that remains in `Created`. Duplicate
bindings fail closed. The accepted binding is retained in
`WorkflowRunSnapshot`; the field is optional and serde-defaulted so legacy
events and snapshots remain readable.

The event records an already-established binding. It does not itself prove that
assessment inputs were fresh or durably committed before the run was created.

## 6. Audit And Privacy Boundary

The generic audit projection exposes only execution disposition, disclosure
requirement, completeness, and step count. It does not copy workflow/run IDs,
bundle identifiers, roots, fingerprints, definitions, runtime facts, evidence,
check output, provider payloads, command output, parser payloads, source
content, paths, environment values, credentials, or tokens into decision
context.

`Debug` redacts workflow identity, run identity, bundle identity/root, and the
aggregate fingerprint. Custom enum and binding deserialization use fixed errors
that do not echo unknown caller-supplied values.

## 7. Test Coverage

Focused tests prove:

- valid accepted sets derive exact deterministic bindings;
- binding serde round trips;
- cross-store same-run/different-bundle construction fails closed;
- zero step count and contradictory blocking/quiet posture fail closed;
- identifiers and fingerprints do not leak through `Debug` or errors;
- binding events retain the model before validation;
- idempotency, exact identity, and single-binding invariants are enforced;
- legacy snapshots remain readable without the optional field;
- audit projection is bounded and reference-only;
- existing immutable-bundle reassessment, runtime-event, and audit tests pass.

## 8. Validation Commands

- Focused assessment-binding, runtime-event, and audit tests: passed.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed; opt-in live integration tests remained
  intentionally ignored.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Remaining Limitations

- Runtime facts have no trusted freshness proof.
- The executor does not establish the binding before `RunCreated`.
- No executor emits the event or consumes its posture.
- Retry and approval resume do not reassess proportional governance.
- Visible disclosure has no CLI or UI presentation path from this event.
- The serialized binding remains sensitive operational metadata.

## 10. Recommended Next Phase

Perform a focused maintainer review of the assessment-binding model and event
vocabulary. If accepted, implement one explicit opt-in executor path that
establishes the accepted binding durably before `RunCreated`, then emits the
bounded event projection without changing existing executor defaults.

Initial review found an exact bundle/set integrity blocker. The narrow fix is
complete and focused re-review accepts it. The next bounded phase is one
explicit opt-in executor binding path before `RunCreated`.

## 11. Governed Phase Record

- Dogfood workflow: `dg/implement`
- Run ID: `run-1784188951586830000-2`
- Approval ID:
  `approval/run-1784188951586830000-2/implementation-approved`
- Approval outcome: granted with persisted presentation proof
- Out-of-kernel work: Rust and documentation edits, focused test execution,
  validation commands, diff inspection, and report drafting
