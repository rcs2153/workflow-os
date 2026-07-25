# Local Check Governance Structural Coverage Blocker Fix Review

## 1. Executive Verdict

Blocker fixed; proceed to canonical local-check declaration and immutable-run-
bundle derivation planning.

## 2. Original Blocker Restatement

The original candidate accepted an opaque leaf obligation fingerprint together
with independently supplied bundle metadata. The private adapter could verify
fingerprint membership but could not prove that the candidate's visible bundle
and step binding matched the context already hashed into that leaf.

That allowed construction-time relabeling before a contribution became bound
to the candidate set.

## 3. Fix Assessment

The fix removes opaque leaf fingerprints from candidate declaration input.
Candidate input now carries an exact requirement fingerprint and requirement
level. Candidate construction derives the `DocsCheck` obligation identity from
its own bundle ID, bundle version, bundle root, step ID, and requirement
fingerprint using the same domain-separated function as runtime leaf creation.

The adapter compares the runtime leaf to this derived identity before creating
a candidate-bound contribution. The evaluator separately requires the exact
candidate-set fingerprint. Together, these checks close both construction-time
relabeling and later cross-set reuse.

## 4. Binding Integrity Assessment

Changing any of the following changes expected obligation identity before
adaptation:

- bundle ID;
- bundle version;
- bundle root;
- step ID; or
- requirement fingerprint.

Workflow and run identity remain committed by the candidate-set fingerprint.
Because the current runtime leaf identity intentionally commits to immutable
bundle and step context, the adapter no longer trusts a caller-provided opaque
hash as evidence of that context.

## 5. Regression Assessment

Focused regression coverage proves:

- a contribution already bound to candidate A fails against candidate B; and
- a runtime leaf produced under bundle A fails adaptation into a candidate
  relabeled as bundle B even when the requirement is otherwise identical.

All prior structural semantics remain unchanged: complete coverage,
required/optional omission, executed failure, precedence, duplicate and
unexpected rejection, ordering, empty unresolved candidates, and redaction.

## 6. Scope And Authority Assessment

The fix remains crate-private and non-serialized. It creates no canonical
declaration authority, aggregate workload posture, proportional-governance
reassessment, executor checkpoint, persistence, schema, CLI behavior,
provider call, SideEffect, or write.

Structural satisfaction remains non-authoritative because declaration source
provenance is still unresolved.

## 7. Privacy Assessment

The shared identity derivation stores no raw payload. Debug and errors do not
expose bundle identity, requirement identity, obligation identity, command
output, source path, environment values, credentials, tokens, or provider
payloads.

## 8. Validation

- `cargo fmt --all --check` - passed;
- `cargo clippy --workspace --all-targets -- -D warnings` - passed;
- `cargo test -p workflow-core --lib local_check_attestation` - passed, 34
  tests;
- `cargo test --workspace` - passed;
- `npm run check:docs` - passed before this review document; and
- `git diff --check` - passed before this review document.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Derive requirement level from future canonical frozen declarations rather
  than adapter caller input.
- Keep `Unknown` as future authoritative aggregate vocabulary, not a v1
  structural candidate disposition.
- Preserve one shared identity derivation implementation as additional local-
  check families are considered.

## 11. Governed Re-Review

- workflow: `dg/review`
- run: `run-1784966509498880000-2`
- approval: `approval/run-1784966509498880000-2/review-scope-approved`
- presentation: `presentation/50354b6f0135a706`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; inspection, documentation,
  and validation ran outside the kernel

## 12. Recommended Next Phase

Plan canonical local-check evidence/check declaration fields and their
deterministic derivation into the immutable run bundle. Do not implement
aggregate workload conversion, reassessment, executor checkpoints, or broader
check families until that source-authority boundary is reviewed.
