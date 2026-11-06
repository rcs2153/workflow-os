# Governed Obligation Plan Phase 4 Source Extraction Report

## Status

The first private Phase 4 implementation slice is complete. Core now derives a
run-bound static obligation inventory from one store-validated immutable bundle
plus one exact required-context contract and execution binding for every
workflow step.

The implementation remains crate-private, non-serializing, non-persistent,
non-authorizing, and disconnected from executor control flow.

## Canonical Input Closure

`GovernedObligationCanonicalInputClosure` accepts only:

- one `StoredImmutableRunBundle` already validated by the immutable store; and
- exactly one `RequiredContextContractBinding` and matching
  `RequiredContextExecutionBinding` for every immutable workflow step.

Construction fails closed for missing or duplicate step posture, bundle/run/
workflow mismatch, contract identity or content mismatch, unknown steps,
invalid execution bindings, or bindings older than the immutable bundle.
Canonical explicit-none required context is retained as source posture and is
not converted into a requirement.

## Deterministic Source Extraction

The private extractor inventories supported static sources from:

- immutable workflow steps and exact resolved skills;
- canonical policy records parsed through the existing typed `PolicyEffect`
  parser;
- exact required-context contract and execution-binding pairs;
- canonical local-check declaration-set records; and
- workflow-authored `ReportArtifactRequirements`.

The private source vocabulary now names `ReportArtifactRequirements`
accurately instead of mislabeling that authored declaration as a runtime
`WorkReportContract`. Non-default authored report artifact requirements remain
invalid under current project validation because their runtime enforcement is
not implemented; this slice preserves the canonical default source posture and
does not bypass that fail-closed product boundary.

Workflow-level approval requirements remain unsupported by the closed Phase 2
source vocabulary and fail closed rather than being forced into a step source.
Runtime `SideEffect` observations remain absent.

## Bounds And Projection

The extractor reuses the immutable bundle's crate-private reference bound. It
does not copy the numeric limit. Source and obligation counts exceeding that
shared bound fail before plan construction.

`GovernedObligationPlanProjectionBinding` commits:

- the existing immutable run-bundle binding;
- the exact run ID;
- the complete sorted required-context closure;
- the derivation algorithm;
- source-closure and inventory commitments; and
- the obligation count.

It contains no execution disposition, disclosure, completeness, current facts,
authority, approval, evidence, satisfaction, or runtime decision. Debug output
redacts all protected identities and commitments.

## Tests

Focused tests prove:

- policy, required-context, local-check, and supported report posture are
  derived from canonical stored inputs;
- identical closure inputs produce identical plan and projection commitments;
- projection and closure Debug output do not reveal run IDs or commitments;
- missing and substituted required-context bindings fail closed; and
- the shared immutable-bundle cardinality maximum rejects maximum-plus-one
  required-context input before plan construction; and
- explicit-none required context remains a source without becoming an
  obligation.

Existing Phase 2 and Phase 3 model, explanation, fixed-vector, differential,
omission, duplicate, unsupported, redaction, and `SideEffect` non-inference
tests remain passing.

## Validation

- `cargo fmt --all --check`: passed after applying canonical formatting.
- focused governed-obligation-plan tests: passed, 26 tests.
- warnings-denied Clippy for `workflow-core`, all targets and features: passed.
- full `cargo test -p workflow-core`: passed, including 774 unit tests, all
  applicable integration binaries, and the crate doc test. Opt-in live tests
  remained ignored by their declared environment gates.
- `npm run check:docs`: required before commit.
- `git diff --check`: required before commit.
- broader workspace checks remain delegated to exact-head CI.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1793970455759286000-2`
- approval: `approval/run-1793970455759286000-2/implementation-approved`
- presentation: `presentation/1bccfa2941eb8c43`
- approval outcome: granted with persisted presentation proof
- phase status: `Completed`
- presentation enforcement: `proof_enforced`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Workflow OS governed phase scope and approval. Source edits, tests, shell
commands, documentation, Git, PR, and merge work occur outside the kernel and
remain disclosed. No executor consumption, persistence, provider action,
release, publication, or independent assurance is simulated.

## Next Required Gate

Focused maintainer/security review of the exact implementation and tests is
required next. Executor composition remains unauthorized.

Only after that review accepts extraction completeness, shared bounds,
required-context provenance, projection commitments, redaction, and the
unsupported report/approval boundaries may a separately governed plan
amendment attempt to name an exact runtime consumer.
