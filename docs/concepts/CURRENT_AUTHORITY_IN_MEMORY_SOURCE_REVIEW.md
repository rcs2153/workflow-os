# Current Authority In-Memory Source Review

## 1. Executive Verdict

**Phase accepted; proceed to pure same-call time-of-use resolver planning.**

The implementation establishes the intended private test trust boundary. One
Core-owned source commits its complete fixture inventory before query
selection, derives the exact query set inside Core, filters grants by the
bound actor and immutable execution scope, requires exact availability
coverage, and constructs the existing payload-free fact set.

No public source, runtime readiness, dereference, provider, sandbox, or write
authority was introduced.

## 2. Scope Verification

The phase stayed within the approved source-only scope:

- the source is a private `#[cfg(test)]` child module;
- production code only declares the test module;
- no crate export, trait, wire type, or compatibility surface was added;
- no runtime or executor consumer can call the source;
- no persistence, event, receipt, artifact, schema, SDK, CLI, UI, example,
  provider, sandbox, SideEffect, write, hosted, or release behavior changed.

## 3. Trust Boundary Assessment

The source constructor consumes owned complete grant and availability
inventories. It validates and canonicalizes them, rejects duplicate identities
and temporally inconsistent records, and computes the source snapshot hash
before accepting an exact query.

The query operation accepts only the immutable execution binding, exact
required-context contract, and evaluation time. Callers cannot supply selected
records, query hashes, source hashes, or completeness posture.

`CompleteForExactQuery` remains meaningful only over the source-owned test
fixture. The implementation does not present that fixture as a production
authority root.

## 4. Query And Selection Assessment

The query set is derived from the exact contract inside Core.

Grant selection requires:

- exact subject actor;
- exact workflow;
- compatible optional run, step, and harness scope;
- exact capability; and
- exact resource.

Every grant satisfying that identity boundary remains available to the future
resolver. Revoked, expired, prerequisite-bearing, delegated,
sensitivity-limited, and lower-specificity candidates are not silently
discarded.

Unrelated actors and execution scopes remain committed by the source snapshot
but are not copied into the returned fact set.

## 5. Availability Assessment

The source requires exactly one availability record for every derived query.
Missing availability fails closed, and duplicate availability keys are
rejected at source construction.

Disconnected, unsupported, and unknown postures remain explicit complete
facts. They do not become readiness.

## 6. Determinism Assessment

The inventory hash binds:

- the private v1 source version;
- source observation time;
- all canonical grants; and
- all canonical availability records.

Input order does not change the hash. Records outside the selected query do
change it. The fixed v1 vector and framed-hash regression protect the current
canonical representation.

## 7. Privacy And Error Assessment

The model contains typed grant records and payload-free availability
observations only.

Debug output redacts timestamps, hashes, identities, resources, and records.
Errors use stable `current_authority.source.*` codes and do not include caller
values. Tests cover secret-like identifiers and unavailable-source failure
without leakage.

No raw target data, source content, command output, provider payload,
credential, path, policy input, approval prose, evidence payload, check output,
or sandbox data is stored.

## 8. Test Assessment

Focused tests cover:

- complete exact-query construction;
- exact actor and execution-scope filtering;
- revoked, expired, and lower-specificity candidate retention;
- complete zero-grant posture;
- canonical inventory order;
- commitment to out-of-query records;
- missing and duplicate source facts;
- explicit unavailable and unknown availability;
- temporal consistency;
- contract substitution;
- Debug and error non-leakage;
- fixed v1 inventory hash; and
- collision-resistant framing.

The full workspace suite also passed, including existing approval, capability,
required-context, immutable-run, report, provider, SideEffect, local-check,
adapter, and runtime suites.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- The future resolver and any future production source should share one
  canonical grant-to-execution matching predicate. Duplicating that predicate
  would risk source/resolver semantic drift.
- A production source still needs authenticated source identity, freshness,
  snapshot or high-watermark semantics, concurrency, and operational failure
  behavior.
- Policy, approval, evidence, and check facts require independently trusted
  sources rather than caller booleans.
- The private fixture source should remain unexported until a real consumer
  proves the smallest useful source abstraction.

## 11. Recommended Next Phase

Plan the pure same-call time-of-use resolver.

The resolver should consume one exact execution binding, contract, and
source-produced fact set in the same call; evaluate grant lifecycle,
specificity, sensitivity, prerequisites, availability, and freshness; and
return a bounded ready-or-blocked result without dereferencing target data or
integrating with the executor.

## 12. Validation

- `cargo fmt --all --check`: passed.
- focused source unit tests: passed, 9 tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed before the review document was added.

## 13. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785152153919489000-2`
- approval ID:
  `approval/run-1785152153919489000-2/review-scope-approved`
- presentation ID: `presentation/0349ec2cdff656c3`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events; 1 approval; 0 retries; 0 escalations;
  presentation proof enforced with one persisted presentation record and event
  marker
- out-of-kernel work: source inspection, review writing, documentation
  updates, and validation were performed by the delegated maintainer; the
  kernel governed scope and approval but did not edit files, run checks, or
  mutate git
