# DocsCheck Attestation Runtime Composition Review

## 1. Executive Verdict

Needs blocker fixes.

The helper composes the accepted primitives in the required order and preserves
its narrow in-memory scope, but accepted proof can still be attributed to
caller-selected step and skill identities that are not resolved from the
stored immutable run bundle.

## 2. Scope Verification

The phase stayed within its approved scope. It added one crate-internal,
explicit `DocsCheck` composition helper and did not add executor consumption,
default registration, automatic checks, persistence, events, evidence,
reports, artifacts, schemas, CLI behavior, providers, SideEffects, writes,
hosted behavior, or release changes.

The existing explicit `DocsCheckLocalHandler` behavior remains intact and now
shares only crate-internal request and runner operations with the helper.

## 3. Composition Boundary Assessment

The crate-private helper is an appropriate first boundary. It accepts explicit
inputs, uses the existing bounded process runner, returns an in-memory result,
and does not reconstruct authority from `SkillOutput` presentation text.

The helper-owned clock, internally constructed observation and candidate, and
crate-private verifier preserve the distinction between process output,
structured result, unverified commitment, and accepted proof.

## 4. Pre-Execution Ordering Assessment

The implementation validates manifest workflow/run identity and the command
requirement before sampling time. It creates
`ImmutableLocalCheckExecutionBinding` before building the request or invoking
the runner. Start and completion samples bracket runner invocation, and the
evaluation sample occurs only for an eligible result.

Clock, request, runner, result-construction, observation, candidate, and
verification failures return no partial outcome. Focused tests prove the
binding/start samples precede runner invocation and that backward time fails
before process execution.

## 5. Immutable Identity Assessment

Workflow and run identity are correctly compared with the validated stored
manifest. The verifier also derives the immutable run binding from that
manifest instead of trusting a caller-supplied bundle-root value.

Step and skill identity are not yet held to the same boundary. The helper
accepts `step_id`, `skill_id`, and `skill_version` from its caller, constructs a
self-consistent handler selection and execution binding from those values, and
then copies them into the observation and candidate. Neither the helper nor the
verifier proves that the selected step exists in the stored workflow record or
that the step resolves to the supplied skill identity and version in the stored
canonical records.

Consequently, a crate-internal caller can relabel a real `DocsCheck` execution
under a different step or skill identity and still receive accepted proof. The
stored bundle remains valid, but the proof attribution is not derived from it.
This violates the plan's exact stored-context requirement and blocks consumer
integration.

## 6. Result And Proof Semantics Assessment

Typed status eligibility is implemented correctly. Failed and timed-out results
outside the accepted status set return honest structured no-proof outcomes and
do not consume a verifier evaluation sample. Eligible results always invoke the
verifier, and verifier errors propagate rather than being reclassified as
ordinary check failure.

The accepted record remains constructor-private, read-only, payload-free, and
non-deserializable. Publicly recomputable fingerprints remain commitments, not
independent authenticity.

## 7. Privacy And Error Assessment

The helper reuses the existing environment, timeout, output-bounding,
redaction, and result-construction path. It does not add paths, executable
details, environment values, source content, raw output, credentials, or
provider payloads to the binding, observation, candidate, proof, Debug output,
or new errors.

New runtime errors use stable codes and static non-leaking messages. Input and
outcome Debug implementations redact identities, stored bundle details, clock,
result, and accepted proof.

## 8. Compatibility Assessment

`LocalSkillRegistry::new()` remains empty, existing executor methods do not call
the helper, and existing explicit DocsCheck invocation still produces the same
`SkillOutput` through the same process request and structured result path.

The implementation creates no state, event, evidence, report, artifact, CLI,
provider, or network integration.

## 9. Test Quality Assessment

Focused tests cover passed proof, failed/timed-out no-proof outcomes,
pre-execution ordering, manifest workflow/run mismatch, clock failure, runner
failure, verifier freshness failure, clock sample counts, and Debug
non-leakage. Existing verifier tests cover extensive command, result,
fingerprint, freshness, truncation, and stored-bundle substitution boundaries.

Missing blocker coverage:

- caller-supplied step identity that is absent from or mismatched with the
  stored workflow must fail before clock or runner use;
- caller-supplied skill ID or version that differs from the stored step
  resolution must fail before clock or runner use; and
- a consistent relabelling of step, skill, binding, observation, and candidate
  must not produce accepted proof.

All repository validation passed, including 265 local-executor tests and the
new focused composition tests. Passing tests do not remove the missing
immutable attribution invariant.

## 10. Blockers

1. Resolve the selected workflow step and skill identity/version from the
   validated stored canonical records, or compare the supplied identities
   against a Core-derived resolution from those records, before clock sampling
   or process execution.
2. Add regression tests for absent, mismatched, and consistently relabelled
   step/skill identities. These failures must be stable, non-leaking, and occur
   before runner invocation.

## 11. Non-Blocking Follow-Ups

- Freshness must be reevaluated at any later consumption boundary.
- Handler implementation provenance remains honestly
  `RegisteredUnattested`.
- The optional accepted-attestation outcome may later benefit from an explicit
  disposition enum.
- The dogfood close summary still cannot read beyond its bounded presentation
  record window, although this phase's approval was proof-enforced.

## 12. Governed Review

- workflow: `dg/review`
- run: `run-1784816876148756000-2`
- approval: `approval/run-1784816876148756000-2/review-scope-approved`
- presentation: `presentation/a3580f9bd1d363a7`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- implementation run: `run-1784644609560548000-2`
- implementation events: 39 events, one approval, zero retries, zero
  escalations
- kernel boundary: governance coordination only; review, repository inspection,
  documentation, and validation ran outside the kernel

## 13. Recommended Next Phase

Perform a focused immutable step/skill attribution blocker fix, followed by a
focused re-review.

Do not add executor consumption, automatic checks, persistence, events,
evidence, reports, artifacts, schemas, CLI behavior, additional command
families, providers, SideEffects, writes, hosted behavior, or release changes
until the fix is accepted.

## 14. Fix-Forward Status

The focused fix is implemented and documented in
[DocsCheck Attestation Runtime Composition Blocker Fix Report](DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_BLOCKER_FIX_REPORT.md).
This section does not erase the original blocker finding. Focused re-review is
required before consumer integration.
