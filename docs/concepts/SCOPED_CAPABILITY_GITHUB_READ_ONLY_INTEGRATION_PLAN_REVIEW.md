# Scoped Capability GitHub Read-Only Integration Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The selected vertical slice is the correct next runtime-composition milestone:
one injected GitHub repository-metadata read, authorized from a fresh
Core-owned source and followed by bounded evidence. The plan also correctly
rejects fabricating a required-context obligation to reuse the current receipt.

Implementation must not begin yet. Five contract decisions remain unresolved
or unsafe as written: policy-precheck provenance, canonical GitHub repository
identity, the non-context source-query/fact-set shape, the receipt type, and
post-read receipt-failure semantics.

## 2. Scope Verification

The plan stays within the approved no-write integration boundary. It does not
authorize:

- GitHub comments, pull requests, merges, or another provider mutation;
- another GitHub read operation or adapter family;
- live GitHub access as acceptance evidence;
- credential discovery or connector installation;
- workflow schemas, runtime configuration, SDKs, CLI behavior, or examples;
- automatic grants, inferred authority, or model-decided authority;
- independent approval, evidence, or check evaluation;
- ambient tools, OpenShell, nested harness execution, or agent delegation;
- hosted identity, enterprise administration, cryptographic proof, or release
  posture changes.

## 3. Selected Operation Assessment

Selecting `GitHubReadOnlyAdapter::read_repository_metadata` is appropriate.
It has the smallest provider payload and sensitivity surface among the current
GitHub reads, uses an injected client, emits bounded normalized output, and
already requires `AdapterCapability::GitHubRead`.

Restricting the first slice to `github.repository.metadata` avoids accidentally
authorizing file contents, pull-request bodies, comments, diffs, or check data.

## 4. Source Boundary Assessment

The plan correctly identifies that caller-supplied grants, availability,
projections, receipts, and fact sets cannot be authoritative runtime inputs.
It also correctly rejects constructing a fake `RequiredContextContractBinding`
for a provider capability use.

The implementation direction must be made exact. Current
`CurrentAuthorityQuerySet`, `CurrentAuthorityFactSet`, and
`CurrentAuthoritySourceRequest` are structurally derived from required-context
requirements. They cannot truthfully represent this call.

Required fix: specify a dedicated crate-private capability-use query and fact
set for exactly one capability/resource pair. It may reuse source
registration, read-window, snapshot, completeness, freshness, consistency,
and commitment primitives, but it must not reuse context-only query types or
introduce a general public query framework in this phase.

## 5. Policy-Precheck Blocker

`AdapterRequest::validate_preconditions` requires
`AdapterPolicyPrecheck::Allowed` or `ApprovalGranted`. `Missing` fails with
`adapter.policy.required`. The plan says this slice does not evaluate policy,
but does not define where a truthful successful precheck comes from.

Capability authority is not a policy decision. The implementation must not:

- fabricate `runtime_allowed` provenance from a capability grant;
- use `fixture_test_allowed` in a live path;
- accept `Missing` and bypass adapter validation; or
- silently treat an independently conditioned grant as satisfying policy.

Required fix: make one already-evaluated `AdapterPolicyPrecheck` an explicit
input and preserve the existing adapter gate. For this slice:

- fixture operation mode accepts only `Allowed` with `FixtureTest` provenance;
- live-read-only mode accepts only `Allowed` with `RuntimePolicy` provenance;
- approval-granted, mock, local, and write-capable modes remain out of scope;
- the helper validates the precheck before source access and passes it unchanged
  into the adapter request; and
- the implementation report discloses that policy-precheck source
  authentication remains the existing adapter contract boundary, not a new
  guarantee of this capability-source slice.

This keeps capability authority additive to policy instead of replacing it.

## 6. Repository Identity Blocker

The proposed `github/<owner>/<repository>` resource is reasonable but is not
canonical while mixed-case aliases are allowed. GitHub owner and repository
lookups are case-insensitive, while `CapabilityResourceScope` equality is
case-sensitive. The same repository could otherwise receive multiple authority
identities.

Required fix:

- validate owner and repository as separate ASCII segments;
- enforce their reviewed maximum lengths and accepted characters;
- reject empty segments, separators, URL syntax, traversal, control text, and
  secret-like values;
- normalize both segments to lowercase before constructing
  `github/<owner>/<repository>`; and
- use the canonical lowercase values for source query, adapter metadata,
  adapter call, and operation commitment.

The original user-supplied spelling must not be retained in errors, Debug, or
receipt evidence.

## 7. Grant Scope Assessment

The plan requires exact actor, workflow, run, step, harness, resource, and
sensitivity binding. Existing resolution intentionally supports grants with
different specificity and chooses the most specific matching grant.

For this first adapter integration, the stricter posture is accepted: the
selected grant must itself contain the exact run and step and must match the
exact harness posture. A workflow-wide or run-wide grant may be valid model
vocabulary, but it is not sufficient for this first execution proof. The
implementation should resolve through the existing helper and then reject a
selected grant whose stored scope is broader than the call.

This is a local integration constraint, not a change to generic capability
resolution semantics.

## 8. Prerequisite Assessment

The plan correctly fails closed for grants carrying independent policy,
approval, evidence, or check requirements. It must not accept caller booleans
claiming those requirements passed.

The explicit adapter policy precheck required by Section 5 is separate from a
grant's declared independent prerequisites. A grant that declares a policy
prerequisite remains `RequiresIndependentEvaluation` even when the adapter
request contains an allowed precheck, because this slice does not bind that
decision to the grant requirement.

## 9. Failure Ordering Assessment

The proposed ordering is otherwise correct:

1. input, identity, operation mode, and policy-precheck validation;
2. canonical repository identity;
3. source registration and exact query;
4. complete fresh source read;
5. fact-set validation;
6. capability resolution and exact-grant-scope check;
7. step projection;
8. adapter request validation;
9. one adapter call;
10. outcome commitment; and
11. receipt issuance.

Every pre-call failure must prove zero client invocations. Adapter failure must
produce no receipt or synthetic successful telemetry.

## 10. Post-Read Receipt-Failure Blocker

A provider read can succeed before receipt construction fails. Returning a
plain `Err` loses the completed-operation fact and can invite an unsafe retry.

Required fix: define an explicit outcome posture with at least:

- `Succeeded { outcome, projection, receipt }`;
- `ReadSucceededReceiptFailed { outcome, projection, error }`;
- `BlockedBeforeRead { ... }`; and
- `ReadFailed { ... }`.

The exact names may follow repository conventions, but a completed provider
read must remain visible and distinguishable from a pre-call block or adapter
failure. The error must remain stable and non-leaking.

## 11. Receipt Compatibility Blocker

The current `AuthorityReceipt` V1 record requires required-context fields that
do not exist for this operation. Adding fake fields is invalid, while altering
the record risks changing existing bytes and trust semantics.

Required fix: choose a separate versioned `CapabilityUseAuthorityReceipt` V1
type rather than changing or wrapping the existing required-context receipt.
The new type must:

- be issued only from a private successful-use proof;
- bind exact execution identity, capability, canonical resource commitment,
  selected grant, source snapshot, fact set, assessment, operation, bounded
  outcome commitment, and issuance time;
- be local, unsigned, evidence-only, and point-in-time;
- expose trusted construction only inside Core;
- deserialize only into an explicitly unverified claim type; and
- leave existing `AuthorityReceipt` V1 fixture bytes and APIs unchanged.

A generalized receipt envelope can be considered after at least two truthful
operation-specific receipt models exist.

## 12. Adapter And Telemetry Assessment

The helper must use `AdapterRequest` and
`GitHubReadOnlyAdapter::read_repository_metadata`; direct client invocation is
not acceptable. Existing adapter invocation and observability records remain
the telemetry source of truth.

The action built for this path must truthfully include the GitHub read
capability expected by the adapter contract. Existing generic
`AdapterAction::read` vocabulary must not create a mismatch between
`ExternalRead` and `GitHubRead`; the implementation should construct and test
the exact action contract used by current GitHub fixtures.

## 13. Privacy And Error Assessment

The plan's payload-free posture is appropriate. New runtime inputs and private
proofs should not derive serialization. Debug and errors must not expose
repository names, source records, grants, response bodies, URLs, credentials,
headers, environment values, paths, or token-like markers.

The successful receipt should commit to canonical resource and bounded outcome
rather than storing provider strings. Existing adapter telemetry may retain its
already-reviewed bounded summary and references; the receipt must not copy
them.

## 14. Test Plan Assessment

The proposed tests are broad enough after adding explicit regressions for:

- operation-mode/policy-precheck provenance mismatch;
- mixed-case repository alias canonicalization;
- owner/repository segment bounds and invalid characters;
- exact selected-grant run/step scope;
- context-query fabrication absence;
- existing `AuthorityReceipt` V1 known bytes unchanged;
- capability-use receipt unverified deserialization; and
- read-success/receipt-failure non-retry posture.

Network-free injected clients remain the correct required validation path.

## 15. Planning Blockers

1. Specify truthful policy-precheck input and provenance constraints.
2. Define lowercase canonical GitHub repository identity and segment rules.
3. Require a dedicated crate-private non-context capability-use query/fact set.
4. Choose a separate `CapabilityUseAuthorityReceipt` V1 type.
5. Define an explicit completed-read/receipt-failure outcome.

These are documentation-level blockers with concrete resolutions. They do not
invalidate the selected vertical slice.

## 16. Non-Blocking Guardrails

- Keep the helper crate-private.
- Keep required tests fixture-backed and network-free.
- Do not generalize the first operation into a public adapter-authority
  framework.
- Do not persist the new receipt in this phase.
- Do not broaden policy, approval, evidence, or check composition.
- Do not alter existing direct adapter APIs or imply they are automatically
  capability-gated.

## 17. Recommended Next Phase

Execute a focused **planning blocker fix** that updates the plan with the five
decisions above, then perform a short blocker-fix review. If accepted, proceed
directly to the complete no-write implementation milestone rather than adding
another planning cycle.

## 18. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1792690261568422000-2`
- approval ID:
  `approval/run-1792690261568422000-2/review-scope-approved`
- presentation ID: `presentation/ea150109644714cd`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- validation: `npm run check:docs` and `git diff --check` passed
- kernel boundary: governance coordination only; source inspection, review
  writing, validation, Git, and pull-request work remain out of kernel
