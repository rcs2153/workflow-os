# Trusted-Host Live-Execution Security Contract Core Model Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation is provider neutral, payload free, deterministic, and
strictly non-authorizing. Its explicit attempt-start posture, process-tree
coherence, bounded observations, stable reasons, and fail-closed substitution
checks materially advance the trusted-host security boundary without adding a
runner or live process.

Acceptance is blocked by three contract defects. First, the Core-owned
requirements value does not model most of the accepted security requirements;
those requirements are instead hard-coded inside the assessment function and
therefore absent from the requirements fingerprint. Second, root observations
do not reject reuse of one root commitment across incompatible typed roles.
Third, the pure assessment function is publicly exported even though the
accepted plan review required assessment orchestration to remain crate-private
until a concrete composition phase proves an external need.

These are narrow model-boundary fixes. They do not authorize discovery,
provider composition, OpenShell integration, a runner, live Cargo, persistence,
schemas, writes, or release changes.

## 2. Scope Verification

The implementation stayed within the approved model-only scope. It added one
Core model module, focused tests, public construction vocabulary, documentation,
and roadmap state. It did not add:

- a process runner, shell, subprocess, or Cargo invocation;
- input discovery, filesystem inspection, network enforcement, or sandboxing;
- provider selection, provider credentials, or OpenShell integration;
- workflow composition, executor dispatch, handler registration, or a public
  execution path;
- persistence, runtime events, reports, artifacts, CLI, config, schemas, SDKs,
  examples, dependencies, writes, or release changes.

The model remains descriptive and evaluative only.

## 3. Model And Reuse Assessment

The new vocabulary is appropriately provider neutral and reuses existing
hosted execution types only where their semantics match. In particular, the
implementation correctly avoids fabricating a runtime-image digest for a
trusted-host attempt and does not create a second terminal receipt hierarchy.

The closed build-input kinds, root roles, observation postures, attempt-start
posture, process-tree terminality, transient-write posture, reason codes, and
dispositions are coherent model concepts. Existing provider identity/version,
control posture, cleanup posture, references, observation summary, and
`SpecContentHash` remain the canonical shared vocabulary.

## 4. Requirement And Observation Separation

The implementation has distinct
`TrustedHostLiveExecutionSecurityRequirements` and
`TrustedHostLiveExecutionSecurityObservations` values, and construction of
either grants no authority. Observation validation proves bounded shape and
coherence rather than truth.

The separation is incomplete on the requirements side. The requirements value
currently stores only:

- exact attempt binding;
- requested policy hash;
- typed root requirements; and
- whether the denied-egress probe is required.

The accepted plan required the Core-owned contract to express the mandatory
enforcement mode, filesystem default-deny and exclusions, network zero-egress,
process ownership and terminality, resource-budget mandatory/optional posture,
transient-write prevention/observation, observation-channel completeness, and
cleanup obligations. The assessor currently treats these as fixed constants.
Consequently, two materially different accepted security contracts can have
the same requirements value and fingerprint, and the serialized contract does
not explain what the assessment actually requires.

The blocker fix must add closed requirement vocabulary for every
decision-relevant control or introduce one explicit versioned strict-profile
identity whose canonical semantics include all of them. Every such fact must
participate in the requirements fingerprint. The assessment must compare
observations against that accepted contract rather than undocumented constants.

## 5. Exact Binding And Fingerprint Assessment

The exact binding commits the immutable run bundle, command contract, attempt,
request, and provider selection. Assessment rejects binding or requirements
fingerprint substitution with stable, non-leaking codes. Canonical root sorting
ensures declaration order does not alter the requirements fingerprint.

The fingerprint encoding is versioned and length-delimited. It includes the
current binding, policy, probe, and root facts. However, because the missing
control requirements are not represented, the fingerprint is not yet a
commitment to the complete accepted security contract.

Focused tests prove root-order canonicalization and provider-selection
substitution. They do not prove fixed vectors or domain separation between all
caller-supplied commitment families. Add a fixed known vector for the complete
requirements contract and explicit change-sensitivity tests for every new
decision-relevant requirement.

## 6. Build-Time Input Assessment

The build-input manifest is payload free, bounded to ten closed kinds and
16,384 aggregate entries, rejects duplicate kinds and count disagreement, and
canonicalizes kind order. `Complete` cannot be paired with zero inputs or an
unavailable/unsupported discovery method.

The model correctly treats completeness as a claim rather than proof. The
optional telemetry reference follows the accepted plan. File names, paths,
package names, source contents, environment values, and raw inventories are
absent.

No build-input blocker remains inside this model slice. A reviewed discovery
implementation remains mandatory before any producer may claim complete live
input closure.

## 7. Root-Binding Assessment

Requirements cover exactly the six typed roles, enforce read-only/read-execute
versus writable access classes, and require invocation-scoped creation for all
writable roots. Observations are canonicalized by role and reject duplicate
roles, unavailable methods cannot claim proven binding or continuity, and
immutable-projection requirements require the matching observation method.

The blocker is cross-role commitment aliasing. The same commitment may be used
for repository input and build output, or for any other incompatible role
combination, while each observation independently claims a different access
class. The assessor then accepts the set. That defeats the model's typed-root
separation because one object can simultaneously stand in for protected input
and writable output.

The blocker fix must reject duplicate root commitments across roles unless a
future contract explicitly models and validates a safe alias class. The first
slice should require six distinct commitments and add regressions for protected
input/write-root aliasing and write-root/write-root aliasing.

## 8. Attempt Start, Process Tree, And Cleanup Assessment

`NotStarted`, `Started`, and `MayHaveStarted` are explicit and correctly drive
safe denial versus reconciliation. Nested deserialization re-enters the
constructor. Not-started observations require no parent exit, no process tree,
no cancellation, and no completed termination. Started terminal observations
require parent exit and completed termination. Possibly-started observations
remain non-terminal or uncertain and cannot claim completed termination.

Observation construction also reconciles process-start and terminal-event
counts. Assessment requires complete-tree terminality and completed cleanup;
parent success alone is insufficient. This satisfies the accepted ambiguity
boundary for the model slice.

## 9. Filesystem, Network, Budget, And Transient-Write Assessment

The observation model represents enforcement mode, filesystem/process/network
control posture, default-deny facts, host-home and credential-store exclusion,
ambient-temp exclusion, policy stability, observation-channel certainty,
bounded security counts, all six resource-budget controls, transient-write
posture, output quiescence, retained-artifact commitment, and cleanup.

Assessment fails closed for audit mode, unavailable controls, allowed network
events, missing required probe evidence, policy drift, incomplete observation,
unenforced budgets, uncertain transient writes, mutable outputs, or incomplete
cleanup. Zero events cannot satisfy when the observation channel is not proven.

These comparisons are conservative, but their governing requirements must be
moved into the accepted contract as described in section 4. The denied-egress
probe remains correctly separate from ordinary zero-egress execution.

## 10. Authority And Public Surface Assessment

The implementation states clearly that a satisfied assessment is evidence for
one exact attempt, not a capability, approval, policy decision, dispatch grant,
retry grant, registration, credential, or write authority. The model performs
no I/O.

The accepted plan review required assessment orchestration to remain
crate-private until a concrete later composition phase proved an external need.
`assess_trusted_host_live_execution_security` is currently re-exported from the
crate root. This creates a public compatibility surface before its requirements
contract is complete and before an approved composition consumer exists.

The blocker fix must make the assessment function crate-private. Public model
construction vocabulary may remain only where focused external tests genuinely
need it; otherwise narrow exports further. A later reviewed composition phase
may deliberately expose the minimum required surface.

## 11. Privacy, Serde, And Error Assessment

Custom Debug implementations redact provider identity, commitments, and stable
references. Errors use static `trusted_host.live_security.*` codes and do not
echo rejected values. The model stores no paths, raw inputs, command output,
environment values, credentials, denied endpoints, provider payloads, or logs.

Validated outer types re-enter constructors during deserialization, including
the nested process-tree, transient-write, input-manifest, root-observation, and
assessment shapes. Unknown enum values fail. Impossible process and assessment
states fail without leaking the rejected value.

As a hardening follow-up, add serialized tamper tests for every derived or
canonical field in the complete requirements shape. The requirements
fingerprint should be recomputed from authoritative fields and either omitted
from accepted input or compared explicitly when present; its wire behavior
must not be ambiguous.

## 12. Test Quality Assessment

The 13 focused tests prove meaningful behavior rather than construction alone:

- exact satisfaction without authority;
- incomplete-input reconciliation;
- affirmative pre-start denial;
- requirements and provider-selection substitution;
- duplicate root roles and attempt-state incoherence;
- bounded serde and redacted Debug;
- complete vocabulary coverage;
- nested process/assessment serde failure;
- policy drift and observation-channel failure;
- required probe failure;
- canonical root-order fingerprints;
- canonical input kinds and immutable-projection semantics; and
- transient-write count/posture coherence.

Missing blocker regressions are:

- every decision-relevant requirement changing the contract fingerprint;
- fixed-vector fingerprint stability for the complete requirements contract;
- duplicate root commitment rejection across incompatible roles;
- proof that assessment orchestration is absent from the public crate API; and
- tampered serialized complete requirements failing closed or being
  unambiguously reconstructed.

Existing hosted, executor-denial, profile-registration, provider-write,
workspace, Node, integration, and docs suites all pass.

## 13. Documentation Assessment

The plan and implementation report accurately preserve the model-only,
provider-neutral, non-authorizing boundary and disclose all real-process
blockers. The report correctly says OpenShell is only a candidate.

The report overstates requirement/observation separation because most security
requirements are not represented in the requirements value. It also describes
the assessment function without disclosing that it is publicly exported
against the plan-review constraint. The blocker-fix report must correct these
claims without erasing this review.

## 14. Blockers

1. Model every decision-relevant security requirement in the accepted
   requirements contract, or bind one closed versioned strict-profile identity
   whose semantics include all of them. Include every requirement in the
   canonical fingerprint and compare observations against that contract.
2. Reject duplicate root commitments across typed roles and add aliasing
   regressions, especially protected-input versus writable-output reuse.
3. Remove the public crate-root export of
   `assess_trusted_host_live_execution_security` until a separately reviewed
   composition consumer proves that public assessment orchestration is needed.
4. Add focused fingerprint, public-surface, requirements-serde, and root-alias
   tests, then correct the implementation report's overstatements.

## 15. Non-Blocking Follow-Ups

- Keep complete build-input discovery, operating-system containment,
  process-tree ownership, transient-write observation, and independent
  effective-control evidence as blockers to live execution.
- Decide in the blocker fix whether explicit requirement fields or one closed
  strict-profile identity is the smaller durable contract. Do not add generic
  policy extensibility.
- Keep hosted receipt composition separate until the model passes review.
- Do not add provider-specific fields merely to compute provider-selection
  commitments.

## 16. Recommended Next Phase

Perform one narrow **trusted-host live-execution security contract model
blocker fix**, followed by a focused blocker-fix review.

The fix is limited to complete requirements binding and fingerprinting, root
commitment uniqueness, public-surface narrowing, focused regressions, and
documentation accuracy. It must not add discovery, provider composition,
OpenShell integration, a runner, live Cargo, persistence, schemas, writes, or
release changes.

## 17. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- focused trusted-host live-security model tests: passed, 13 tests.
- `cargo test --workspace`: passed.
- `npm run check`: passed under the repository Node 20 toolchain.
- `npm run check:integrations`: passed under the repository Node 20 toolchain.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 18. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792814890186110000-2`.
- Approval ID:
  `approval/run-1792814890186110000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/ae09f55b2dd66b06`.
- Approval-presentation content hash:
  `ae09f55b2dd66b069815ce73f045ec5164c8b6e48cdc76212f0b3161c3e64f72`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, zero retries, and zero
  escalations. Approval presentation enforcement was proof-enforced and the
  presentation marker is present in the event trail.
- Validation summary: the final implementation matrix remained green;
  `npm run check:docs` and `git diff --check` also passed after the review and
  roadmap edits.
- Out-of-kernel work: the delegated maintainer inspected the accepted plan,
  plan review, implementation report, model implementation, exports, and tests;
  authored this review and roadmap update; and ran validation. The kernel
  governed scope and approval but did not edit files, run checks, or perform
  git or PR operations.
