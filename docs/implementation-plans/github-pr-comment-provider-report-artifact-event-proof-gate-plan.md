# GitHub PR Comment Provider Report Artifact Event-Proof Gate Plan

## 1. Executive Summary

GitHub PR comment provider reconciliation disclosure can now be composed into in-memory WorkReports.

The next question is how explicit report artifact paths should handle that disclosure when a caller requires durable event proof. This plan defines a narrow future gate that can deny report artifact writes when provider/local reconciliation exists but workflow event proof is missing.

This plan does not implement anything. It does not authorize provider calls, workflow event appends, report artifact writes, retries, auth loading, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, broader writes, or release posture changes.

## 2. Goals

- Preserve workflow events as the durable event-proof source.
- Preserve provider disclosure as bounded posture, not event proof.
- Allow explicit artifact paths to require event proof before writing report artifacts.
- Keep provider/local reconciliation distinct from workflow event proof.
- Preserve workflow pass/fail semantics when artifact gate failure occurs after a run exists.
- Return structured non-leaking artifact gate errors.
- Avoid raw provider payloads, comment bodies, PR bodies, diffs, command output, raw specs, credentials, paths, and token-like values.
- Prepare a small implementation prompt for an explicit helper or gate option.

## 3. Non-Goals

Do not implement:

- implementation in this planning phase;
- provider calls;
- GitHub comment creation;
- provider lookup/query reconciliation;
- automatic retries;
- workflow event appends;
- audit sink emission;
- observability emission;
- automatic report generation;
- automatic report artifact writing;
- default executor behavior changes;
- CLI behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- broader write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- provider write request/response model;
- provider preflight and side-effect record composition;
- attempted/completed/failed side-effect lifecycle transition helpers;
- injected provider-call trait/input model;
- concrete GitHub PR comment provider client with explicit auth and injected transport;
- provider write reconciliation candidate model/helper;
- executor-integrated provider-write request/result/helper;
- provider write event append helper for eligible reconciled outcomes;
- bounded provider reconciliation disclosure helper;
- in-memory WorkReport provider disclosure composition;
- report artifact store and generic side-effect integrity helpers;
- GitHub PR comment report artifact citation helper;
- provider-candidate report artifact integration helper.

Not implemented:

- strict report artifact gate policy based directly on provider disclosure posture;
- operator recovery workflow for missing event proof;
- provider lookup/query reconciliation;
- default executor automatic report/artifact behavior;
- live write adapter readiness.

## 5. Source-Of-Truth Policy

The gate must preserve these boundaries:

| Surface | Role |
| --- | --- |
| `GitHubPullRequestCommentProviderWriteReportDisclosure` | Bounded provider/local/event-proof posture disclosure. |
| `WorkflowRunEvent` | Durable event-proof source. |
| `SideEffectRecordStore` | Local side-effect lifecycle source. |
| `WorkReport` | Governed handoff artifact, not an event log. |
| `WorkReportArtifactRecord` | Durable local report artifact record, separate from workflow state. |

Provider/local agreement must never be upgraded into durable event proof. If disclosure says event proof is missing, a strict artifact gate should deny artifact write when event proof is required.

## 6. Recommended First Implementation Target

Implement a narrow explicit helper or gate option for report artifact write paths.

Preferred shape:

- Add an explicit provider disclosure gate input to the GitHub PR comment report artifact integration path, or a small standalone helper used immediately before artifact write.
- Accept one or more precomputed `GitHubPullRequestCommentProviderWriteReportDisclosure` values.
- Accept an explicit policy flag such as `require_provider_event_proof`.
- Allow artifact write only when every supplied provider disclosure has a posture with workflow event proof present.
- Return a structured report artifact error when event proof is required and missing.
- Preserve the run and in-memory report when artifact write is denied.

Avoid first:

- changing default executor behavior;
- changing `execute_with_report`;
- adding schema-declared artifact policy;
- creating new workflow events;
- calling providers or querying GitHub;
- writing artifacts automatically for every run.

## 7. Gate Policy

The first strict policy should be opt-in.

Candidate policy fields:

- `require_provider_event_proof: bool`;
- `allow_provider_not_called: bool`;
- `allow_missing_disclosures_when_no_provider_write_requested: bool`.

Recommended v1 default for an explicit gate helper:

- require provider event proof when provider disclosures are supplied and artifact policy requests strict event proof;
- allow no provider disclosures only when the caller did not request provider disclosure gating;
- deny missing-event postures when strict event proof is required;
- deny ambiguous, transition-failed, local-state-ambiguous, reconciliation-required, and reconciliation-unavailable postures when strict event proof is required;
- preserve permissive behavior for existing artifact paths unless the new gate is explicitly enabled.

## 8. Posture Mapping

Recommended strict gate behavior:

| Disclosure posture | Strict event-proof gate |
| --- | --- |
| `ProviderNotCalled` | Deny when provider write proof was required; allow only when provider write was not required. |
| `ProviderSucceededLocalCompletedEventAppended` | Allow. |
| `ProviderSucceededLocalCompletedEventMissing` | Deny. |
| `ProviderFailedLocalFailedEventAppended` | Allow when failure artifact disclosure is permitted. |
| `ProviderFailedLocalFailedEventMissing` | Deny. |
| `ProviderResponseAmbiguous` | Deny. |
| `ProviderSucceededLocalTransitionFailed` | Deny. |
| `ProviderFailedLocalTransitionFailed` | Deny. |
| `LocalStateAmbiguous` | Deny. |
| `ReconciliationRequired` | Deny. |
| `ReconciliationUnavailable` | Deny. |

Failure postures with event proof should remain allowed only if the artifact policy is about disclosure correctness rather than write success. A separate caller policy may decide whether failed provider outcomes should still produce artifacts.

## 9. Error Handling

Gate failures should be artifact/report errors, not user project diagnostics.

Recommended error codes:

- `github_pr_comment_provider_artifact_gate.event_proof_missing`;
- `github_pr_comment_provider_artifact_gate.reconciliation_required`;
- `github_pr_comment_provider_artifact_gate.provider_not_called`;
- `github_pr_comment_provider_artifact_gate.unsupported_posture`;
- `github_pr_comment_provider_artifact_gate.disclosure_required`.

Errors must not leak provider payloads, comment bodies, repository identity, PR numbers, paths, raw IDs where sensitive, tokens, raw specs, command output, parser payloads, or secret-like values.

## 10. Workflow Semantics

Artifact gate failure after a workflow run exists must not change workflow pass/fail status.

The future implementation should return:

- run preserved;
- in-memory report preserved when already generated;
- no artifact record written;
- structured artifact/report generation error;
- no workflow event appended;
- no audit or observability event emitted unless separately scoped.

## 11. Privacy And Redaction

The gate must use the existing bounded provider disclosure model.

It must not store, serialize, Debug-print, or copy:

- raw provider payloads;
- GitHub comment bodies;
- GitHub PR bodies;
- diffs or file contents;
- review thread bodies;
- CI logs;
- command output;
- parser payloads;
- raw specs;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

## 12. Test Plan

Future implementation tests should cover:

- strict gate allows `ProviderSucceededLocalCompletedEventAppended`;
- strict gate rejects `ProviderSucceededLocalCompletedEventMissing`;
- strict gate allows or rejects `ProviderFailedLocalFailedEventAppended` according to explicit policy;
- strict gate rejects `ProviderFailedLocalFailedEventMissing`;
- strict gate rejects ambiguous and transition-failed postures;
- strict gate rejects provider-not-called when provider proof is required;
- permissive artifact path remains unchanged when gate is not enabled;
- gate failure preserves run and in-memory report;
- gate failure writes no report artifact;
- gate failure appends no workflow event;
- gate failure emits no provider call;
- errors use stable codes and do not leak secret-like markers;
- existing report artifact, provider disclosure, side-effect integrity, approval-linkage, executor, and WorkReport tests continue to pass;
- docs check passes.

## 13. Proposed Implementation Sequence

1. Add a small provider disclosure artifact gate helper or explicit integration input.
2. Wire it only into the explicit GitHub PR comment report artifact integration path.
3. Preserve existing artifact paths when the gate is not enabled.
4. Add focused tests for allow/deny postures and non-mutation behavior.
5. Run full Rust and docs validation.
6. Create an implementation report.
7. Perform maintainer review before any broader executor/default behavior.

## 14. Deferred Work

- Provider lookup/query reconciliation.
- Operator recovery workflow for missing event proof.
- Workflow schema-declared provider artifact policy.
- Automatic executor report/artifact behavior.
- CLI rendering or commands.
- Examples.
- Hosted/distributed runtime.
- Broader write-capable adapters.
- Reasoning lineage.
- Release posture changes.

## 15. Final Recommendation

Proceed next to a small implementation phase: explicit GitHub PR comment provider report artifact event-proof gate helper, opt-in only.

Do not build default executor behavior, provider calls, event appends, artifact auto-writes, CLI behavior, schemas, examples, hosted behavior, broader writes, reasoning lineage, or release posture changes in that phase.
