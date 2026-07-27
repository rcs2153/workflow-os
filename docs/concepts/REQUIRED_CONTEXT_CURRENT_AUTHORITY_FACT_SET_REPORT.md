# Required Context Current Authority Fact-Set Report

## 1. Executive Summary

Workflow OS now has a model-only, payload-free commitment for the current
authority facts supplied for one exact required-context contract.

The model derives every query from the contract, commits source snapshot
posture, canonicalizes grant and availability inventories, rejects duplicates
and out-of-query records, and detects serialized substitution.

It does not return an authority or readiness decision. Claimed completeness is
not trusted until a future Core-owned source independently proves it.

## 2. Scope Completed

- Added `CurrentAuthorityFactSetVersion`.
- Added `AuthorityFactSourceKind`.
- Added `AuthorityFactCompletenessPosture`.
- Added `CurrentAuthorityQuery` and `CurrentAuthorityQuerySet`.
- Added `AuthorityFactSourceBinding`.
- Added `CurrentAuthorityFactSetInput`.
- Added `CurrentAuthorityFactSet`.
- Exposed exact capability-resource derivation on typed context targets.
- Derived the complete canonical query set from the exact contract.
- Bound the fact set to the immutable execution-binding hash.
- Retained canonical candidate grants and availability observations.
- Rejected duplicate and out-of-query records.
- Required exact availability coverage for a claimed complete query set.
- Added deterministic domain-separated hashes, fail-closed serde, redacted
  Debug, and focused tests.

## 3. Scope Explicitly Not Completed

This phase did not add:

- an `authorize`, `permit`, `ready`, or `dereference` API;
- trusted source completeness;
- a source/store implementation;
- current policy, approval, evidence, or check acceptance;
- time-of-use resolution;
- runtime or executor integration;
- persistence, events, receipts, artifacts, schemas, SDKs, CLI, UI, or
  examples;
- providers, OpenShell, sandbox execution, SideEffects, or writes;
- hosted behavior, reasoning lineage, or release changes.

## 4. Model Boundary

Each query retains:

- requirement ID;
- typed target;
- access level;
- obligation;
- maximum sensitivity;
- derived capability; and
- derived resource.

The source commitment retains:

- source kind;
- snapshot hash;
- observation time;
- claimed completeness;
- query-set hash;
- grant and availability counts; and
- records hash.

The aggregate commits the immutable execution-binding hash, complete query set,
source commitment, evaluation time, grants, availability records, and fact-set
hash.

## 5. Completeness Posture

`CompleteForExactQuery` is model vocabulary and a committed claim only.
Arbitrary public construction or deserialization does not make it trusted
authority.

When that posture is present, the model requires one unambiguous availability
record for every exact query and rejects extra, duplicate, or missing records.
A future Core-owned source must still prove that its snapshot actually owns
the complete relevant inventory before any runtime consumer may rely on it.

## 6. Validation And Privacy

Validation fails closed for:

- execution-binding and contract hash mismatch;
- invalid temporal ordering;
- empty, unordered, duplicate, or incorrectly derived queries;
- duplicate grants;
- duplicate, noncanonical, missing, or out-of-query availability;
- noncanonical grant ordering;
- out-of-query grants;
- source counts, query hash, records hash, or observation mismatch; and
- aggregate hash mismatch.

Errors use stable `current_authority.fact_set.*` codes without caller values.
Debug output exposes versions, postures, and counts while redacting identities,
hashes, resources, and timestamps.

No raw source, target, provider, policy-input, approval, evidence, check,
command, parser, environment, credential, log, path, or sandbox payload is
stored.

## 7. Tests

Focused tests cover:

- complete query derivation;
- valid fact-set construction and serde round trip;
- missing availability under claimed completeness;
- duplicate availability;
- noncanonical serialized record ordering;
- fixed v1 aggregate hash stability;
- fixed-width framing separation;
- wire tampering without value leakage;
- Debug non-leakage; and
- payload-free serialized shape.

Adjacent required-context contract and immutable execution-binding tests also
pass.

## 8. Validation

- `cargo fmt --all --check`: passed after formatting.
- `cargo test -p workflow-core --test current_authority_fact_set --test required_context_execution_binding --test required_context`:
  passed, 21 tests.
- `cargo test -p workflow-core --test current_authority_fact_set`: passed,
  5 tests after canonical wire-order hardening.
- `cargo clippy -p workflow-core --all-targets -- -D warnings`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1785141714253971000-2`
- approval ID:
  `approval/run-1785141714253971000-2/implementation-approved`
- presentation ID: `presentation/59aaff0f0a525f90`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: `Completed`
- event summary: 39 events; 1 approval; 0 retries; 0 escalations; presentation
  proof enforced with one persisted presentation record and event marker
- out-of-kernel work: code, tests, docs, formatting, and validation commands
  were performed by the delegated maintainer; the kernel governed scope and
  approval but did not edit files, run cargo, or mutate git

## 10. Remaining Limitations

- Source completeness is claimed, not independently proven.
- No Core-owned complete inventory source exists.
- No accepted policy, approval, evidence, or check fact wrappers are composed.
- No freshness policy is enforced beyond timestamp ordering.
- No current authority or time-of-use readiness result exists.
- The hash currently commits canonical serde bytes; compatibility exposure
  remains deferred. A fixed v1 vector and framing regression now protect the
  internal preview algorithm from accidental drift.

## 11. Recommended Next Phase

Focused maintainer review accepted the phase after adding the planned hash
regressions. Implement one Core-owned in-memory completeness-capable source for
tests only next. Continue to defer time-of-use readiness, dereference,
runtime integration, persistence, providers, OpenShell, sandbox execution,
SideEffects, writes, hosted behavior, and release changes.
