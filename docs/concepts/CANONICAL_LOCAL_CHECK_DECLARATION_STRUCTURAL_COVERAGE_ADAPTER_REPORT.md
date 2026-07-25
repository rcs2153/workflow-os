# Canonical Local-Check Declaration Structural-Coverage Adapter Report

## 1. Executive Summary

Workflow OS now has a crate-private authoritative adapter from a validated
stored immutable run bundle to the existing local-check structural-coverage
candidate. The adapter removes caller-supplied declaration facts from this
boundary: workflow identity, run identity, bundle binding, step identity,
requirement fingerprints, and requirement levels are derived from validated
stored records.

This phase does not convert structural coverage into aggregate governance
posture and does not enforce an executor checkpoint.

## 2. Scope Completed

- Added explicit `Unresolved` and `CanonicalStoredBundle` declaration-source
  posture.
- Added a private adapter accepting `StoredImmutableRunBundle` and `StepId`.
- Required complete one-reference and one-record coverage for every workflow
  step.
- Derived obligation definitions from canonical declaration records.
- Preserved authoritative empty declaration sets.
- Added source posture to candidate and coverage fingerprints and redaction-safe
  `Debug` output.
- Added stable fail-closed errors for missing, incomplete, duplicate,
  mismatched, and unknown-step sources.
- Added a pre-deduplication cardinality check that rejects two distinct skill
  references bound to one workflow step.

## 3. Scope Explicitly Not Completed

This phase does not add:

- aggregate evidence/check posture;
- proportional-governance reassessment;
- executor or runtime gates;
- automatic or default local-check execution;
- local-check handlers or command inference;
- public APIs, schemas, or CLI behavior;
- workflow events or persistence changes;
- evidence or WorkReport generation;
- provider adapters, OpenShell integration, SideEffect execution, or writes;
- hosted/distributed behavior or release-posture changes.

## 4. Adapter Boundary

The adapter requires a `StoredImmutableRunBundle`, which is only produced after
the create-only bundle store has resolved and validated the manifest and every
referenced canonical record. It then independently verifies complete step
coverage and reference-to-record alignment before constructing a candidate.

The existing caller-supplied constructor remains available only as unresolved
structural vocabulary. Its candidate fingerprint cannot collide with an
authoritative stored candidate because source posture participates in hashing.

## 5. Authoritative Empty Sets

A canonical record containing no declarations means the frozen workflow step
has no local-check obligations. It produces an authoritative empty candidate
and vacuously satisfied structural coverage.

A legacy bundle with no declaration-set references is different. It has no
authoritative declaration source and fails closed.

## 6. Validation And Error Boundary

The adapter verifies:

- the requested step is a resolved workflow step;
- declaration references cover every resolved workflow step exactly once;
- resolved records cover every resolved workflow step exactly once;
- every reference resolves to exactly one matching content-addressed record;
- record workflow, workflow version, and immutable-bundle version match the
  stored manifest; and
- requirement levels map only from the validated canonical enum.

Errors use stable
`local_check_attestation.structural_coverage.authoritative_*` codes and contain
no caller-supplied identity or payload values.

## 7. Privacy And Redaction

The adapter reads only validated bounded identities, fingerprints, and
requirement levels already present in the immutable bundle. It does not read or
copy command text, arguments, working directories, environment values, raw
output, source contents, provider payloads, credentials, or evidence bodies.
Candidate and result `Debug` output discloses source posture and counts while
redacting bindings and fingerprints.

## 8. Tests Added

Focused tests cover:

- authoritative populated declaration adaptation;
- required obligation derivation;
- canonical authoritative empty sets;
- distinct authoritative and unresolved fingerprints;
- legacy bundle rejection;
- unknown-step rejection;
- stable error codes and error non-leakage;
- source-posture propagation into structural results; and
- ambiguous step binding rejection before expected-step deduplication; and
- regression of the existing unresolved structural-coverage behavior.

The normal-path tests construct, persist, and reload real enriched and legacy
immutable bundles. One focused defense-in-depth regression constructs a
manifest-valid stored-bundle fixture with two distinct skill identities bound
to one step, then proves the adapter fails closed before candidate
construction.

## 9. Governed Phase Record

- workflow: `dg/implement`
- run: `run-1785001236688645000-2`
- approval:
  `approval/run-1785001236688645000-2/implementation-approved`
- presentation: `presentation/3cbd8b0844d8bd1f`
- approval outcome: granted by delegated maintainer through proof enforcement
- implementation run status: completed
- kernel boundary: governance coordination only; source edits and validation
  occurred outside the kernel

## 10. Validation

Validation completed:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- focused structural-coverage library tests: passed, 15 tests.

## 11. Remaining Limitations

- No aggregate workload posture consumes the authoritative result.
- No proportional-governance decision is reassessed from coverage.
- No executor path invokes the adapter.
- No check is run and no result attestation is created by this adapter.
- Legacy immutable bundles remain inspectable but cannot claim authoritative
  local-check coverage.

## 12. Recommended Next Phase

The focused ambiguous-step blocker is implemented and governed re-review
accepts the authoritative adapter. Aggregate governance posture remains a
separate future phase.

## 13. Blocker Fix Record

The blocker fix:

- counts step-scoped skill references before set construction;
- compares that count with the unique expected-step cardinality;
- returns
  `local_check_attestation.structural_coverage.authoritative_source_duplicate`
  when distinct skill identities share one step; and
- adds a focused regression that reaches the adapter through a
  `StoredImmutableRunBundle` fixture.

The original phase review remains unchanged above its eventual fix-forward
note so the finding and its disposition stay auditable.

The blocker fix was governed through:

- workflow: `dg/blocker`;
- run: `run-1785004078133718000-2`;
- approval:
  `approval/run-1785004078133718000-2/fix-approved`;
- presentation: `presentation/fdf51d741bb83022`;
- approval outcome: granted by delegated maintainer through proof enforcement;
  and
- event summary: 39 events, one approval, zero retries, zero escalations.
