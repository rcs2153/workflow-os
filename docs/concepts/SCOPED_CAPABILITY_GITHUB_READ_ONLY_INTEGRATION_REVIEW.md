# Scoped Capability GitHub Read-Only Integration Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation gets the provider-call boundary, exact grant scope,
canonical repository identity, explicit failure domains, and receipt trust
posture right. It does not yet satisfy the accepted source-boundary contract.
The integration computes a private hash over filtered facts instead of reading
and validating a `CurrentAuthoritySourceSnapshot`, and it has no prior trusted
generation or watermark with which to detect source rollback.

No second operation, public API, executor wiring, or provider mutation should
reuse this integration pattern until those source guarantees are restored.

## 2. Scope Verification

The implementation stayed within the approved crate-private, opt-in,
read-only scope. It added no GitHub write, automatic adapter gating, executor
or workflow-schema wiring, CLI behavior, live-network acceptance requirement,
credential discovery, receipt persistence, hosted identity, nested harness
runtime, or release change.

Existing direct GitHub adapter APIs remain unchanged and are not represented
as capability-gated.

## 3. Provider Call Boundary

The helper preserves one auditable pre-call pipeline. Policy-precheck
provenance, canonical repository identity, source freshness, authority
resolution, exact selected-grant scope, projection, and adapter request
preconditions all execute before the injected client is called.

The adapter is invoked through
`GitHubReadOnlyAdapter::read_repository_metadata`; the helper does not bypass
the adapter by calling the client directly. Existing telemetry therefore
remains the provider-invocation source of truth.

## 4. Policy Assessment

Policy and capability authority remain independent:

- fixture mode accepts only an allowed fixture-test precheck;
- live read-only mode accepts only an allowed runtime-policy precheck;
- the helper does not synthesize policy success from a grant; and
- grants with independent policy, approval, evidence, or check prerequisites
  remain non-authorizing.

This boundary is accepted. Authentication of the supplied allowed policy
precheck remains an existing adapter-contract limitation rather than a new
guarantee of this integration.

## 5. Canonical Identity Assessment

Owner and repository segments are separately bounded, screened for
secret-like markers, and normalized to ASCII lowercase before resource,
request, and provider-call construction. The successful test proves that
mixed-case caller input results in one canonical `/repos/acme/widgets` call.

Errors and Debug output do not retain the original caller spelling.

## 6. Exact Authority Assessment

The integration resolves `github.read` for the exact actor, workflow, run,
step, harness, repository resource, sensitivity, and evaluation time. After
generic resolution, it rechecks that the selected grant itself contains the
exact run, step, and harness scope. Broader workflow authority is therefore
insufficient for this first operation.

Projection is derived from the fresh resolution inside the same call and
cannot be supplied as authority by the caller.

## 7. Source Snapshot Blocker

The accepted plan requires a complete, consistent, fresh source snapshot with
committed query coverage. The implementation instead filters the private
registered-source inventories and computes
`capability-use-current-authority-snapshot-v1` directly over those selected
records.

That custom hash is not a `CurrentAuthoritySourceSnapshot` and does not execute
the existing snapshot validations for:

- `CompleteForExactQuery` family and target coverage;
- registered versus observed consistency;
- bounded source read-window semantics;
- fact-family counts;
- watermark binding to the registered complete inventory; or
- snapshot-model validation and commitment consistency.

The source object is private, immutable, and validated when registered, so
this is not evidence that callers can currently mutate its inventories. It is
still a blocker because the integration report claims source-snapshot
guarantees that the actual operation path does not evaluate.

Required fix: construct and validate a dedicated capability-use snapshot
through the existing committed snapshot primitives, with exactly the
capability-grant and capability-availability families and one exact query.
The resulting validated snapshot commitment, not an ad hoc selected-record
hash, must bind the fact set and receipt.

## 8. Rollback Blocker

The plan also requires rollback or invalid-generation posture to fail before
the provider call. The helper commits the source's optional generation into a
hash but never compares it with a prior trusted generation or watermark. A
single newly supplied source instance cannot prove that its generation did not
move backward.

Required fix: add the smallest Core-owned prior-source expectation needed to
compare the current snapshot with a trusted watermark or generation, or revise
the contract through focused review if first-use-without-history needs an
explicit posture. Merely hashing the current generation is not rollback
detection. Tests must prove rollback blocks with zero client calls.

## 9. Failure Semantics Assessment

The four outcome domains are truthful:

- `BlockedBeforeRead` means the adapter was not called;
- `ReadFailed` means the adapter call failed and no receipt exists;
- `Succeeded` carries the bounded outcome, projection, and receipt; and
- `ReadSucceededReceiptFailed` preserves the fact that the provider read
  completed, avoiding unsafe blind retry.

Debug output exposes only bounded posture and stable error classification.

## 10. Receipt Assessment

The separate `CapabilityUseAuthorityReceipt` avoids fabricating
required-context fields and leaves the existing `AuthorityReceipt` contract
unchanged. Trusted construction is crate-private and follows a successful
read. The receipt stores commitments rather than repository names or provider
payloads, and deserialization produces only an explicitly unverified claim.

The receipt remains local, unsigned, point-in-time, and evidence-only. It
cannot authorize a later operation. This boundary is accepted, subject to the
source-snapshot commitment fix above.

## 11. Privacy And Redaction Assessment

No raw GitHub response body, credential, header, environment value, repository
name, source fact, or grant payload is copied into receipt evidence or errors.
Input and proof types remain private and non-serializable. Debug output redacts
execution, resource, grant, source, and receipt identities.

Serialized receipt claims intentionally contain bounded execution and grant
identifiers but not repository plaintext or provider payloads. Their
unverified trust posture is explicit.

## 12. Test Quality Assessment

Focused tests prove the successful single-call path, case normalization,
policy-provenance rejection, broad-grant rejection, stale source rejection,
unavailable capability, prerequisite-bearing authority, sensitivity ceiling,
adapter failure, post-read receipt failure, unverified receipt
deserialization, and secret-like repository non-leakage.

Blocker-fix coverage must add:

- validated exact-query source snapshot construction;
- incomplete family or target coverage rejection;
- consistency or snapshot-commitment mismatch rejection where constructible;
- prior-generation or watermark rollback rejection with zero client calls;
- direct wrong run, step, and harness zero-call regressions; and
- owner/repository boundary and invalid-character cases.

The last two groups are defense-in-depth follow-ups; the missing validated
snapshot and rollback proof are blockers.

## 13. Blockers

1. Replace the ad hoc capability-use source snapshot hash with a validated
   exact-query source snapshot using existing committed snapshot invariants.
2. Introduce and enforce a truthful prior trusted generation/watermark posture
   so rollback can be detected before provider invocation.
3. Correct the implementation report after the fixes so it does not claim
   guarantees absent from the executable path.

## 14. Non-Blocking Follow-Ups

- Add direct integration regressions for each exact execution-identity
  mismatch.
- Add exhaustive owner/repository segment-boundary tests.
- Keep the helper crate-private until a later review considers executor or
  workflow-schema wiring.
- Continue to state that direct adapter APIs are not automatically
  capability-gated.

## 15. Recommended Next Phase

Execute a focused **scoped capability GitHub read-only integration blocker
fix**. Change only the source snapshot and rollback boundary, add the required
zero-call regressions, correct the implementation report, and rerun the full
workspace validation suite. Perform a short blocker-fix review before any
second provider operation or mutation.

## 16. Governed Phase Record

- workflow ID: `dg/review`
- run ID: `run-1792694512480264000-2`
- approval ID:
  `approval/run-1792694512480264000-2/review-scope-approved`
- presentation ID: `presentation/e9eff3419d713286`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- validation summary: `cargo fmt --all --check`, workspace Clippy with warnings
  denied, `cargo test --workspace`, `npm run check:docs`, and
  `git diff --check` passed
- kernel boundary: governance coordination only; source inspection, review
  authorship, validation, Git, pull-request, CI, and merge work remain outside
  the kernel
- out-of-kernel work: source inspection, review authorship, local validation,
  Git operations, pull-request publication, CI observation, and merge handling
