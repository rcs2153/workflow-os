# Current Authority In-Memory Source Planning Report

## 1. Executive Summary

Workflow OS now has an accepted model-only current-authority fact-set
commitment, but its public constructor cannot prove that a caller supplied a
complete record set.

This planning phase defines the first Core-owned completeness boundary: a
private in-memory source for tests that owns a complete canonical inventory
before accepting an exact contract-derived query.

No source implementation, runtime authority decision, dereference, provider,
sandbox, SideEffect, or write was added.

## 2. Scope Completed

- Defined the source ownership and trust boundary.
- Defined complete canonical inventory construction.
- Defined exact contract-derived query execution.
- Defined grant and availability selection semantics.
- Defined source snapshot and fact-set binding.
- Defined deterministic failure and privacy behavior.
- Defined focused implementation and regression tests.
- Positioned the phase before a pure time-of-use resolver.

## 3. Scope Explicitly Not Completed

- No source code was implemented.
- No public source API or trait was added.
- No authority, permit, readiness, consumption, projection, or dereference
  result was added.
- No runtime or executor behavior changed.
- No persistence, events, audit receipts, artifacts, schemas, SDKs, CLI, UI,
  or examples were added.
- No providers, OpenShell integration, sandbox execution, SideEffects, or
  writes were added.
- No hosted behavior, enterprise identity, reasoning lineage, or release
  posture changed.

## 4. Recommended Model Boundary

The first implementation should add a private `#[cfg(test)]` in-memory source
inside `workflow-core`.

It should consume owned complete grant and availability inventories, commit
the whole canonical inventory independently of any query, derive the query set
from the exact required-context contract, and produce the existing
`CurrentAuthorityFactSet`.

It must not be re-exported or serialized.

## 5. Completeness Boundary

`CompleteForExactQuery` becomes meaningful in this test boundary only because:

- the source owns its full inventory before query execution;
- callers cannot supply filters, query hashes, snapshot hashes, or
  completeness posture;
- every matching grant candidate is retained;
- exact availability coverage is required; and
- the source snapshot commits records outside the selected query as well.

The output is still not production authority and exposes no readiness API.

## 6. Privacy And Security

The planned source stores only typed grants, payload-free availability
observations, timestamps, counts, and deterministic commitments.

It excludes raw contents, commands, outputs, provider data, credentials,
paths, sandbox data, policy payloads, approval prose, evidence payloads, and
check output. Stable errors must not include caller values.

Focused plan review corrected one boundary before implementation: grant
selection must apply the exact actor, workflow, run, step, and harness matching
predicate while retaining all scope-matching candidates regardless of
lifecycle, expiry, revocation, prerequisites, delegation, sensitivity, or
specificity.

## 7. Validation

- `npm run check:docs`: passed with the repository Node 20 toolchain.
- `git diff --check`: passed.

## 8. Dogfood Governance

- workflow: `dg/d`
- run ID: `run-1785148998199860000-2`
- approval ID:
  `approval/run-1785148998199860000-2/planning-approved`
- presentation ID: `presentation/1019e7b36e60424c`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events; 1 approval; 0 retries; 0 escalations; presentation
  proof enforced with one persisted presentation record and event marker
- out-of-kernel work: planning analysis, documentation edits, and validation
  were performed by the delegated maintainer; the kernel governed scope and
  approval but did not edit files, run documentation checks, or mutate git

## 9. Remaining Limitations

- The first source will prove completeness only over its owned test inventory.
- No production authority source is selected.
- Policy, approval, evidence, and check fact sources remain undefined.
- Freshness, retry, approval-resume, and concurrent snapshot semantics remain
  deferred.
- No consumer can interpret the fact set as readiness.

## 10. Recommended Next Phase

Perform a focused maintainer review of the plan, then implement the private
Core-owned in-memory source and its focused tests.
