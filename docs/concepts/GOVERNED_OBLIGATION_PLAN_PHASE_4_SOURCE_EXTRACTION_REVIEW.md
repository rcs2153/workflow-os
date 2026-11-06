# Governed Obligation Plan Phase 4 Source Extraction Review

## Status

**Changes requested. Executor composition remains unauthorized.**

The merged private extractor is deterministic, bounded, redaction-safe, and
correctly subordinate to the immutable bundle. Its workflow, skill, policy,
local-check, report-posture, bundle, run, step, and contract commitments are
derived from canonical validated records rather than mutable project input.

The review cannot yet accept the required-context closure as the exact
consumer-owned provenance boundary described by the accepted Phase 4 plan.

## Blocking Finding: Consumer-Owned Required-Context Posture Is Not Proven

`GovernedObligationCanonicalInputClosure::new` validates that each supplied
`RequiredContextExecutionBinding` is internally self-consistent and matches
the stored bundle, workflow, run, step, contract identity, contract content,
and bundle creation-time floor.

It does not compare the binding's actor, maximum sensitivity, or freshness to
an independently owned execution expectation. The existing public
`RequiredContextExecutionBinding::new` constructor accepts caller-selected
actor, sensitivity ceiling, and binding time. A different self-consistent
binding for the same bundle, step, and contract can therefore enter the
closure and produce a different but valid projection.

That is safe as a non-authorizing commitment, but it is not proof that the
binding belongs to the future runtime consumer. The accepted plan explicitly
requires wrong-actor, wrong-sensitivity, and stale-binding rejection and
forbids solving ownership through parallel caller-injected assertions.

The correction must do one of the following through a separately governed
plan amendment:

1. name an existing exact runtime consumer that already owns the expected
   actor, sensitivity posture, trusted time, stored bundle, and complete
   per-step required-context bindings, then derive the closure inside that
   boundary; or
2. narrow the source-extraction claim to immutable commitment only, preserve
   its non-authorizing posture, and keep runtime composition blocked until such
   a consumer-owned comparison exists.

The correction must not add a second resolver, accept caller-authored expected
values as authority, infer actor or sensitivity from unrelated bundle fields,
or treat the binding hash as proof that its semantic choices are correct.

## Blocking Finding: Required Adversarial Matrix Is Incomplete

The merged tests prove contract-content substitution, missing closure input,
explicit-none posture, deterministic output, redaction, and one
maximum-plus-one outer-bound rejection.

They do not yet prove the accepted plan's required matrix for:

- wrong actor;
- wrong sensitivity posture;
- stale or future binding time;
- wrong step;
- wrong run or bundle;
- duplicate binding;
- insertion-order independence with more than one step;
- acceptance at each exact maximum before maximum-plus-one rejection; or
- source and obligation cardinality independently.

Several of these cases should fail under existing checks today. The missing
tests still matter because this model is intended to become a runtime
precondition and its completeness claim must be reviewable without inference.
Actor, sensitivity, and freshness cases require the ownership correction
above before their expected result can be implemented honestly.

## Accepted Properties

The review accepts these parts of the merged implementation:

- `StoredImmutableRunBundle` is validated through manifest-to-record
  bijection and rejects unreferenced canonical records;
- exact workflow, referenced skill, and referenced policy records come from
  the immutable bundle rather than mutable YAML;
- typed policy effects are parsed through the existing Core vocabulary;
- local-check obligations come only from canonical declaration-set records;
- workflow-authored `ReportArtifactRequirements` are not mislabeled as a
  runtime `WorkReportContract`;
- explicit-none required context remains a canonical source without becoming
  an obligation;
- runtime `SideEffect` observations do not manufacture static obligations;
- the shared immutable-bundle cardinality constant is reused rather than
  copied;
- source and obligation allocations are preceded by checked cardinality
  calculation;
- the projection binds bundle, run, algorithm, required-context closure,
  source closure, inventory, and count without carrying authority or
  satisfaction; and
- Debug and stable error surfaces remain payload-safe.

## Validation Evidence

- exact-head CI for PR #773: 9 of 9 checks passed;
- local focused governed-obligation-plan tests: 26 passed;
- local warnings-denied Clippy for `workflow-core`: passed;
- local full `cargo test -p workflow-core`: passed, including 774 unit tests,
  all applicable integration binaries, and the crate doc test;
- source review traced immutable-store record validation, bundle construction,
  required-context contract construction, and execution-binding construction;
- no independent external assurance is claimed.

## Governed Review

- workflow: `dg/review`
- run: `run-1793976254292024000-2`
- approval: `approval/run-1793976254292024000-2/review-scope-approved`
- presentation: `presentation/6c33a0ac4968d1f4`
- approval outcome: granted with persisted presentation proof
- presentation enforcement: `proof_enforced`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations

Repository inspection, tests, documentation edits, Git, PR, and merge work are
performed outside the kernel and remain disclosed. The kernel coordinates the
governed phase; it does not simulate independent review or runtime execution.

## Next Required Gate

One separately governed correction plan must resolve required-context
execution ownership and enumerate the complete adversarial and cardinality
test matrix. No implementation correction, executor composition, persistence,
public API, provider work, release, or publication is authorized by this
review.
