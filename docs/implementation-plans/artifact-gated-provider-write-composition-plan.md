# Artifact-Gated Provider-Write Composition Plan

Status: Implemented as the explicit in-memory helper
`compose_github_pr_comment_provider_write_with_artifact_gates(...)`. Default
executor writes, automatic artifact writes, CLI mutation behavior, schemas,
examples, hosted behavior, recovery automation, reasoning lineage, and release
posture changes remain unimplemented.

## 1. Executive Summary

Workflow OS now has an explicit in-memory runtime composition helper for the
GitHub PR comment provider-write lane. That helper composes local executor
execution, approval-presentation proof, injected provider invocation,
provider/local reconciliation, eligible workflow-event proof, and bounded
WorkReport disclosure.

Workflow OS also has explicit report artifact paths that can require
SideEffect referential integrity, approval-side-effect linkage, high-assurance
disclosure posture, and approval proof-marker projection coverage before an
artifact is written.

The explicit provider-write composition lane now composes with the explicit
report artifact gate lane through a caller-supplied in-memory helper. The
helper remains explicit and local. It does not implement artifact writing by
default, provider writes by default, CLI mutation behavior, schemas, examples,
hosted behavior, recovery automation, reasoning lineage, or release posture
changes.

## 2. Goals

- Define the smallest explicit artifact-gated provider-write composition
  boundary.
- Compose existing reviewed primitives instead of adding a new governance
  family.
- Preserve default `LocalExecutor::execute(...)` behavior.
- Keep provider writes explicit, injected, local, and caller opt-in.
- Keep report artifact writing explicit and caller opt-in.
- Require artifact gates before artifact write when the caller requests them.
- Preserve workflow pass/fail semantics when report or artifact gates fail
  after a run exists.
- Return bounded, stable, non-leaking errors.
- Keep all stores, provider clients, and policies caller supplied.
- Avoid hidden auth loading, hidden store roots, hidden retries, hidden
  recovery, and hidden runtime config.

## 3. Non-Goals

Do not implement in this phase:

- Rust code changes;
- automatic provider writes;
- default executor provider writes;
- automatic report generation;
- automatic report artifact writing;
- hidden provider or auth loading;
- hidden artifact, SideEffect, linkage, or proof-marker stores;
- provider lookup/recovery automation;
- CLI mutation commands or rendering;
- workflow schema changes;
- SDK changes;
- examples;
- hosted or distributed runtime behavior;
- write-capable Jira, CI, filesystem, HTTP, or arbitrary adapters;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed provider-write foundations include:

- `compose_github_pr_comment_provider_write_runtime(...)`;
- `GitHubPrCommentProviderWriteRuntimeCompositionRequest`;
- `GitHubPrCommentProviderWriteRuntimeCompositionResult`;
- proof-gated GitHub PR comment provider-write execution;
- injected provider-call execution;
- provider/local reconciliation;
- eligible completed/failed provider-write workflow-event append;
- bounded reconciliation disclosure for WorkReport side-effect posture.

Implemented and reviewed artifact-gate foundations include:

- explicit terminal report generation and report artifact model;
- explicit report artifact store;
- report artifact SideEffect referential integrity validation;
- store-backed approval-side-effect linkage validation;
- high-assurance artifact disclosure gates;
- approval proof-marker projection stores;
- store-backed report artifact approval proof-marker gates;
- helper-level report artifact write composition with governance gates;
- executor artifact proof-marker gate integration;
- workflow-declared proof-marker artifact requirement derivation.

The missing planning boundary is the bridge between the explicit provider-write
composition result and the explicit artifact-gated report write path.

## 5. Composition Problem

The provider-write composition helper proves that a caller can run one explicit
provider-write lane with approval-presentation proof and bounded disclosure.
The artifact gate helpers prove that a report artifact can be written only
after requested local evidence, SideEffect, approval-linkage, and proof-marker
checks pass.

If these paths remain separate forever, a caller can either:

- execute a provider write and inspect in-memory disclosure; or
- write a governed report artifact through artifact gates.

The next runtime-composition question is how a caller should explicitly request
both in one path without implying that provider writes or artifact writes are
automatic.

The bridge must not become a default executor path. It should be an explicit
composition helper or executor-adjacent API that callers choose when they
already have the required stores, proof records, report inputs, and provider
client.

## 6. Recommended First Implementation Boundary

The first implementation should add a new explicit helper-level composition
path rather than change default executor behavior.

Recommended posture:

- accepts a fully explicit provider-write composition request;
- accepts explicit report artifact write inputs or enough validated report
  artifact inputs to reuse existing artifact constructors;
- accepts caller-supplied artifact, SideEffect, approval-linkage, and
  proof-marker projection stores;
- accepts caller-supplied artifact gate policies;
- invokes the provider-write composition helper first;
- writes no artifact unless provider-write composition reaches a caller-
  approved terminal posture and artifact gates pass;
- returns the provider-write composition result plus bounded artifact gate/write
  posture;
- writes no artifact on any strict gate failure;
- does not retry provider writes or perform recovery.

Prefer helper-level composition first. Executor integration can follow after
review if the helper proves the gate order and result posture.

## 7. Candidate API Shape

Names should follow repository conventions at implementation time. Candidate
shape:

```rust
pub struct GitHubPrCommentProviderWriteArtifactGatedCompositionRequest<'a> {
    pub provider_write:
        GitHubPrCommentProviderWriteRuntimeCompositionRequest<'a>,
    pub artifact_inputs:
        GitHubPrCommentProviderWriteArtifactGateInputs<'a>,
}

pub struct GitHubPrCommentProviderWriteArtifactGateInputs<'a> {
    pub artifact_store: &'a dyn WorkReportArtifactStore,
    pub side_effect_store: &'a dyn SideEffectRecordStore,
    pub approval_linkage_store: &'a LocalApprovalSideEffectLinkageStore,
    pub approval_proof_marker_projection_store:
        &'a LocalApprovalProofMarkerAuditProjectionStore,
    pub side_effect_integrity_policy:
        WorkReportArtifactSideEffectIntegrityPolicy,
    pub approval_linkage_policy:
        WorkReportArtifactApprovalLinkagePolicy,
    pub high_assurance_disclosure_policy:
        WorkReportArtifactHighAssuranceDisclosurePolicy,
    pub approval_proof_marker_policy:
        WorkReportArtifactApprovalProofMarkerGatePolicy,
}

pub fn compose_github_pr_comment_provider_write_with_artifact_gates(
    request: GitHubPrCommentProviderWriteArtifactGatedCompositionRequest<'_>,
) -> Result<GitHubPrCommentProviderWriteArtifactGatedCompositionResult, WorkflowOsError>
```

If current store abstractions use concrete types rather than traits, the
implementation should use the existing concrete names. Do not introduce broad
trait abstractions only for this bridge.

## 8. Gate Sequence

Use deterministic fail-closed ordering:

1. Validate the explicit provider-write composition request.
2. Run `compose_github_pr_comment_provider_write_runtime(...)`.
3. Stop before artifact construction/write if provider-write composition did
   not reach an artifact-eligible terminal posture.
4. Build or accept a validated `WorkReport`/`WorkReportArtifactRecord` through
   existing constructors.
5. Verify artifact/report/run identity against the terminal run returned by the
   provider-write composition path.
6. Validate SideEffect referential integrity.
7. Validate approval-side-effect linkage.
8. Validate high-assurance disclosure posture when required.
9. Validate approval proof-marker projection posture from the caller-supplied
   store when required.
10. Write the artifact only after all requested strict gates pass.

The provider must never be invoked after artifact gates; provider invocation is
already part of the provider-write composition path. Artifact gates must guard
artifact writing, not the original provider call.

## 9. Artifact Eligibility

Artifact writing should be eligible only when the provider-write composition
result has a bounded posture that is safe to disclose.

Recommended first eligibility:

- workflow execution reached a terminal completed/failed/canceled run supported
  by the existing provider-write helper;
- provider-call posture is complete enough for bounded reconciliation
  disclosure;
- reconciliation posture is explicit, even if ambiguous;
- event-proof posture is explicit, even if event append was not eligible;
- WorkReport disclosure projection is available and validated.

Do not treat artifact eligibility as provider success only. Failed provider
writes may still require artifact disclosure if the failure is bounded and
reviewable. Ambiguous provider/local states may be artifact-eligible only if the
artifact clearly discloses ambiguity and recommended operator action.

## 10. Failure Semantics

Provider-write failure before a run exists should still return the existing
structured execution/provider error.

After a run exists:

- report generation failure must not change workflow pass/fail semantics;
- artifact gate failure must not change workflow pass/fail semantics;
- artifact write failure must not change workflow pass/fail semantics;
- no workflow events should be appended because an artifact gate failed;
- no provider retry should occur because an artifact gate failed;
- no recovery lookup or repair should occur automatically;
- no partial artifact should be written when strict gates fail.

The result should carry the provider-write result and either:

- `Some(artifact_record/write_result)` when artifact writing succeeds; or
- `None` plus a bounded artifact error/gate posture when artifact writing is
  blocked or fails after provider-write completion.

## 11. Result And Debug Posture

The result may expose bounded counts and posture fields, such as:

- provider write gate state;
- reconciliation state;
- report disclosure state;
- artifact gate state;
- artifact write state;
- SideEffect citation count;
- approval-linkage count;
- approval proof-marker citation count;
- projected proof-marker count;
- missing projection count.

Do not expose raw IDs, provider references, comment bodies, report text,
approval reasons, presentation content hashes, local paths, command output, or
secret-like values through `Debug`, error messages, or serialized helper
results.

## 12. Store And Persistence Boundary

All stores must be caller supplied.

The helper must not:

- infer state roots;
- create artifact stores;
- create SideEffect stores;
- create approval-linkage stores;
- create proof-marker projection stores;
- persist proof-marker projections;
- persist SideEffect records outside the existing provider-write helper path;
- append workflow events outside the existing provider-write helper path;
- emit audit/observability records;
- mutate workflow state;
- write files other than the explicitly requested report artifact write.

## 13. Privacy And Redaction

The composition path must not store or copy:

- provider auth tokens;
- raw provider payloads;
- raw GitHub issue, PR, or comment bodies;
- raw command output;
- raw CI logs;
- raw source contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- approval-presentation text;
- approval reasons;
- credentials, authorization headers, private keys, tokens, or secret-like
  values.

Errors must use stable codes and avoid paths, snippets, payloads, and raw
identifiers. Artifact gate posture should remain count-based where possible.

## 14. Contract With Existing Paths

Do not change existing methods or helpers:

- `LocalExecutor::execute(...)`;
- `LocalExecutor::execute_with_report(...)`;
- `execute_with_report_artifact_and_side_effect_gates(...)`;
- `compose_github_pr_comment_provider_write_runtime(...)`;
- provider lookup/recovery helpers.

The new path should be additive. Existing tests for executor, report artifact,
provider-write, SideEffect, approval proof-marker, and WorkReport behavior
should continue to pass without modification except for imports if required by
new tests.

## 15. Test Plan

Future implementation tests should cover:

- successful provider-write composition plus artifact write when all gates pass;
- provider-write gate failure blocks provider invocation and writes no artifact;
- provider-write completion with missing strict proof-marker projection writes
  no artifact;
- SideEffect referential integrity failure writes no artifact;
- approval-side-effect linkage failure writes no artifact;
- high-assurance disclosure failure writes no artifact;
- artifact/run identity mismatch writes no artifact;
- artifact store failure returns bounded artifact error without retrying
  provider write;
- provider failure posture can still produce artifact disclosure when bounded
  and eligible;
- ambiguous reconciliation posture is explicit and does not trigger automatic
  lookup/recovery;
- absent artifact gate inputs are not supported by this helper; callers should
  use the existing in-memory provider-write composition helper instead;
- default executor behavior remains unchanged;
- existing report artifact paths remain unchanged;
- `Debug` and serialization do not leak provider payloads, comment text,
  approval reasons, local paths, report text, IDs, hashes, command output, or
  secret-like values.

## 16. Proposed Implementation Sequence

1. Add helper-level request/result types for artifact-gated provider-write
   composition.
2. Delegate provider-write behavior to
   `compose_github_pr_comment_provider_write_runtime(...)`.
3. Define artifact eligibility from bounded provider-write composition posture.
4. Reuse existing WorkReport/artifact constructors and governance-gated artifact
   write helpers.
5. Add focused tests for success, fail-closed gate order, no artifact on
   failure, and non-leaking output.
6. Review the helper phase.
7. Only after review, decide whether an executor-adjacent API is warranted.

## 17. Deferred Work

- Executor-adjacent artifact-gated provider-write API.
- Automatic provider writes.
- Automatic report artifact writing.
- Hidden auth loading.
- Workflow-declared provider-write configuration.
- Workflow schema or SDK vocabulary.
- CLI mutation commands.
- Examples.
- Provider lookup/recovery automation.
- Broader write-capable adapters.
- Hosted/distributed runtime.
- Reasoning lineage.
- Release posture changes.

## 18. Final Recommendation

Proceed next with **artifact-gated provider-write composition helper
implementation**, helper-level and explicit only.

The implementation should compose the accepted in-memory provider-write runtime
composition helper with existing governed report artifact write gates. It must
not change executor defaults, make provider writes automatic, infer stores or
auth, add CLI behavior, add schemas/examples, perform lookup/recovery
automation, broaden adapter writes, implement reasoning lineage, or change
release posture.
