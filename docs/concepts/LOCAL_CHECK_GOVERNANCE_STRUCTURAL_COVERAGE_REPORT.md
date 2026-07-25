# Local Check Governance Structural Coverage Report

## 1. Executive Summary

Workflow OS now has a crate-private candidate model and pure deterministic
evaluator for structural coverage of local-check governance obligations.

The result proves exactness only relative to an explicitly supplied candidate
set whose declaration provenance remains unresolved. It is not an aggregate
evidence/check fact and carries no authority to reassess, execute, approve,
persist, or write.

## 2. Scope Completed

- Added private required and optional local-check obligation levels.
- Added private contribution postures for satisfied, failed, required-
  unavailable, and optional-unavailable outcomes.
- Added immutable candidate-set binding across bundle, workflow, run, and step
  identity.
- Added deterministic obligation-set and structural-coverage fingerprints.
- Bound every contribution to the exact candidate-set fingerprint.
- Added exact duplicate, unexpected, and level-mismatch rejection.
- Added fail-closed complete-coverage evaluation independent of input order.
- Added a private same-call adapter for the existing `DocsCheck` governance
  contribution.
- Added redaction-safe Debug behavior and focused unit coverage.

## 3. Scope Explicitly Not Completed

- authoritative declaration-source derivation;
- aggregate `GovernanceWorkloadEvidenceCheckPosture` conversion;
- proportional-governance reassessment;
- executor checkpoints or automatic checks;
- persistence, replay, events, evidence records, reports, or artifacts;
- workflow, policy, profile, project, CLI, SDK, or example schemas;
- provider calls, SideEffects, writes, hosted behavior, or release changes.

## 4. Model Summary

The private model represents:

- one candidate set bound to immutable bundle, workflow, run, and step
  identity;
- one required or optional obligation for each exact requirement fingerprint;
- zero or one contribution for each expected obligation; and
- one bounded structural result containing disposition, counts, and redacted
  fingerprints.

No new type is publicly exported or serialized. Declaration provenance is not
caller-asserted or labeled authoritative; the model remains structurally
useful and semantically unresolved by construction.

## 5. Structural Evaluation

Evaluation rejects duplicate expected obligations, duplicate contributions,
unexpected contributions, and requirement-level mismatches. Missing required
coverage becomes `RequiredUnavailable`; missing optional coverage becomes
`OptionalUnavailable`; an executed optional failure remains `Failed`.

The strict precedence is:

```text
Failed
  > RequiredUnavailable
  > OptionalUnavailable
  > Satisfied
```

An empty supplied candidate set is structurally vacuous but still has
unresolved source authority. It cannot be treated as proof that no canonical
obligations exist.

## 6. DocsCheck Adapter

The private adapter consumes the current same-call, requirement-scoped
`DocsCheck` contribution and maps it into the candidate vocabulary. The
candidate obligation identity must match exactly. No `EvidenceReference`,
aggregate workload fact, persisted contribution, or imported proof is created.

This adapter remains private and non-authoritative. A future authoritative
path must derive requirement level and complete obligation membership from
canonical declarations frozen into the immutable run bundle.

## 7. Privacy And Redaction

The model stores no raw command output, source content, file path, environment
value, credential, token, provider payload, or evidence payload. Debug output
redacts binding identities and fingerprints and exposes only bounded posture
and count information. Validation errors use stable static messages.

## 8. Test Coverage

Focused tests prove:

- complete candidate coverage is structurally satisfied;
- missing required and optional coverage remain distinct;
- optional executed failure is not weakened;
- a failed contribution cannot be masked;
- duplicate expected and supplied identities fail closed;
- unexpected and level-mismatched contributions fail closed;
- input ordering does not change the result or fingerprint;
- bundle, workflow, run, step, and obligation substitution changes identity;
- a contribution from another candidate-set binding fails closed;
- an empty unresolved candidate remains non-authoritative; and
- Debug and errors do not leak identity values.

The existing runtime contribution test also adapts a real same-call
`DocsCheck` contribution into the private candidate vocabulary.

## 9. Governed Phase

- workflow: `dg/implement`
- run: `run-1784960052258189000-2`
- approval: `approval/run-1784960052258189000-2/implementation-approved`
- presentation: `presentation/3c5514645df52b58`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; code edits, tests,
  documentation, and validation ran outside the kernel

## 10. Validation

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test -p workflow-core --lib local_check_attestation` - passed, 34
  tests;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed; and
- `git diff --check` - passed.

## 11. Remaining Limitations

- no canonical evidence/check declaration schema exists;
- no authoritative obligation set is frozen into the immutable run bundle;
- no aggregate evidence/check workload fact can be produced;
- no proportional-governance reassessment consumes the result;
- no executor checkpoint consumes the result; and
- the first private adapter supports only the accepted `DocsCheck` leaf.

## 12. Recommended Next Phase

Perform phase-level maintainer review of this private structural slice. If
accepted, plan canonical evidence/check declaration fields and immutable-run-
bundle derivation before any aggregate workload conversion or executor
checkpoint.
