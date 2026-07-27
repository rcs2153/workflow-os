# Current Authority Same-Call Time-Of-Use Resolver Review

## 1. Executive Verdict

**Phase accepted; proceed to production current-authority source boundary
planning.**

The private test-only implementation proves the intended same-call
composition without exposing public readiness, dereferencing targets, or
creating runtime authority.

## 2. Scope Verification

The phase stayed within its approved boundary.

It did not add:

- public current-authority or readiness APIs;
- target or payload dereference;
- runtime, executor, approval-resume, retry, or report integration;
- persistence, events, audit records, receipts, or artifacts;
- providers, OpenShell, sandbox execution, SideEffects, or writes;
- schemas, SDKs, CLI, UI, examples, hosted behavior, enterprise
  administration, reasoning lineage, or release changes.

## 3. Trust-Boundary Assessment

The implementation does not accept public caller-constructed
`CurrentAuthorityFactSet` values as authoritative input.

Both complete sources and the resolver are private, compiled only under
`#[cfg(test)]`, and unavailable through the crate's public exports. The
authority source owns complete grant and availability inventories. The
reference source owns a complete context-reference inventory. Both derive and
commit their own canonical state.

This correctly proves composition without claiming a production source trust
model.

## 4. Same-Call Composition Assessment

The resolver executes the required sequence in one call:

1. validate immutable execution binding and exact contract;
2. query complete authority and reference sources;
3. invoke existing capability resolution for every exact requirement;
4. construct fresh projection candidates;
5. project by requested access level;
6. rerun exact required-context consumption; and
7. derive one payload-free `Ready | Blocked` assessment.

It does not accept prior capability resolutions, projections, consumption
results, or readiness leases.

The projection candidate observation time correctly comes from the current
reference inventory, while authority lifecycle and expiry are evaluated at the
single requested time of use.

## 5. Canonical Matching Assessment

The extracted crate-private `grant_matches_execution_scope` predicate is a
sound replacement for the previously duplicated source matcher.

It covers actor, workflow, optional run, optional step, and optional harness
scope. Production capability resolution still independently requires exact
capability and resource matching and remains authoritative for specificity,
lifecycle, expiry, revocation, sensitivity, and terminal posture.

No matching rule was weakened.

## 6. Readiness And Prerequisite Assessment

`Ready` remains narrowly defined as required-context satisfaction under the
exact immutable binding and fresh complete private facts.

Expired, revoked, unavailable, disconnected, or sensitivity-inadequate grants
do not project authority. Unresolved policy, approval, evidence, and check
prerequisites do not project authority. They block required obligations and
remain explicit non-blocking gaps for optional obligations.

This preserves accepted required-context semantics without turning IDs or
caller assertions into proof.

## 7. Determinism Assessment

The complete reference inventory:

- validates each reference;
- canonicalizes by typed target;
- rejects duplicate targets; and
- commits the full inventory with a versioned hash.

The final assessment commits binding, contract, exact query set, source
commitments, fact-set commitment, evaluation time, posture, reasons, and
consumption result. Reordered equivalent input produces the same fixed test
vector.

## 8. Privacy And Redaction Assessment

The implementation is payload-free.

It does not read or retain target contents, repository source, command output,
provider output, credentials, environment values, approval prose, policy
inputs, check output, sandbox data, or paths.

Debug output redacts timestamps and commitments. Errors use stable bounded
codes and do not include caller-supplied identities or references. Missing and
duplicate data fail closed without value leakage.

## 9. Test Quality Assessment

Fourteen focused tests cover:

- complete ready resolution;
- required approval and all prerequisite families;
- optional prerequisite disclosure;
- revoked and expired grants;
- sensitivity ceilings;
- unavailable optional references;
- disconnected capability availability;
- contract substitution;
- duplicate and missing references;
- deterministic ordering and hashing; and
- Debug and error non-leakage.

The full `workflow-core` library suite and complete workspace test suite pass.

## 10. Product And Evaluator Feedback Assessment

Fresh-pull evaluation describes Workflow OS as a coherent, honest local
governance kernel and identifies proportional governance and quiet success as
the next product priority.

This phase is aligned with that recommendation. It does not add user-visible
ceremony. It establishes a prerequisite for reducing ceremony safely:
low-friction decisions must rely on fresh Core-owned authority rather than
stale or caller-asserted state.

The evaluator's Node 24 integration-check failure and duplicate
missing-manifest diagnostic were already addressed in the accepted
fresh-pull evaluator UX and tooling fix. They do not create work for this
phase.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Define authenticated production source identity, freshness, snapshot or
  high-watermark, concurrency, retry, and operational failure semantics.
- Define independently trusted sources for policy, approval, evidence, and
  check prerequisites.
- Decide target-existence and reference-freshness semantics for a production
  reference source.
- Decide one-time-use or replay-prevention semantics before target
  dereference.
- Add future-dated reference-source and additional scope-substitution tests
  when the production source boundary is implemented.

## 13. Recommended Next Phase

Plan the production current-authority source boundary only.

Do not select or implement a runtime consumer yet. Do not expose public
readiness, dereference targets, integrate OpenShell, execute SideEffects, add
writes, or broaden provider mutation families.

## 14. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-core --lib same_call_resolver --quiet`: 14 passed.
- `cargo test -p workflow-core --lib --quiet`: 135 passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.

## 15. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785156669280162000-2`
- approval ID:
  `approval/run-1785156669280162000-2/review-scope-approved`
- presentation ID: `presentation/a0fa95aee17a9025`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- validation summary: formatting, warning-denied clippy, focused and full
  workspace tests, documentation checks, and diff integrity passed
- out-of-kernel work: source inspection, review judgment, documentation edits,
  and validation were performed by the delegated maintainer; the kernel
  governed scope and approval but did not inspect code, edit files, execute
  checks, or mutate git
