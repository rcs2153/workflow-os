# GitHub PR Comment Provider Event-Proof Recovery Plan

Status: Planned. This follows the accepted [GitHub PR Comment Provider Report Artifact Event-Proof Gate Matrix Hardening Review](../concepts/GITHUB_PR_COMMENT_PROVIDER_REPORT_ARTIFACT_EVENT_PROOF_GATE_MATRIX_HARDENING_REVIEW.md). It plans the next bounded recovery layer for cases where provider/local reconciliation posture exists but durable workflow event proof is missing, stale, or ambiguous.

This plan does not implement recovery behavior, provider calls, provider lookup/query reconciliation, event append, report artifact writes, CLI behavior, schemas, examples, hosted behavior, approval-presentation enforcement, or release posture changes.

## 1. Executive Summary

Workflow OS now has strict opt-in report artifact gates that can reject GitHub pull request comment provider disclosures when durable workflow event proof is missing.

That is the correct safety posture, but it leaves an operator-facing question: what should happen after the gate rejects a report artifact write because provider/local state and event proof do not line up?

This plan defines a conservative event-proof recovery boundary. The first recovery implementation should classify recovery posture and produce bounded operator guidance from explicit local inputs. It should not query GitHub, repair state, append events, write artifacts, retry provider calls, or change workflow pass/fail semantics.

## 2. Goals

- Define a bounded recovery posture for missing or ambiguous provider event proof.
- Preserve workflow events as the durable event-proof source.
- Preserve provider disclosure as bounded posture, not durable event proof.
- Keep provider lookup/query reconciliation separate from local recovery planning.
- Give operators a safe next action when a strict report artifact gate fails.
- Prevent accidental duplicate provider writes after ambiguous provider/local states.
- Preserve in-memory reports and workflow run status when artifact gating fails.
- Keep errors and guidance stable, bounded, and redaction-safe.
- Prepare a small implementation phase for a local recovery classification helper.

## 3. Non-Goals

Do not implement or authorize:

- implementation in this planning phase;
- provider calls;
- GitHub lookup/query reconciliation;
- automatic retries;
- workflow event append;
- audit sink emission;
- observability emission;
- report artifact writes;
- automatic report generation;
- automatic report artifact writing;
- default executor behavior changes;
- CLI behavior;
- workflow schema changes;
- examples;
- hosted or distributed runtime behavior;
- broader write-capable adapters;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- approval-presentation enforcement;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- GitHub PR comment provider-call model and injected provider helper.
- Concrete GitHub PR comment provider client with explicit caller-supplied auth and injected transport.
- GitHub PR comment provider write reconciliation candidate model/helper.
- Executor-integrated provider-write request/result/helper.
- Provider write event append helper for eligible reconciled outcomes.
- Provider reconciliation disclosure projection into in-memory WorkReports.
- GitHub PR comment provider report artifact event-proof gate helper.
- Denied-posture matrix hardening for the strict event-proof gate.

Not implemented:

- operator recovery workflow for missing event proof;
- provider lookup/query reconciliation;
- automatic event repair;
- schema-declared provider artifact policy;
- automatic report artifact writes;
- CLI recovery commands.

## 5. Recovery Problem Statement

Strict artifact gates are intentionally fail-closed. They prevent a report artifact from becoming durable when it would overclaim provider-write proof.

The common failure shapes are:

- provider/local reconciliation indicates success or failure, but matching workflow event proof is missing;
- provider/local reconciliation is required or unavailable;
- provider response is ambiguous;
- provider succeeded but local lifecycle transition failed;
- provider failed but local lifecycle transition failed;
- local state is ambiguous;
- provider was not called even though a strict artifact path expected provider-write proof.

Without a recovery posture, the operator sees a correct denial but lacks a governed next action. The next layer should explain what is known, what is missing, and what must be reviewed before retry, repair, or artifact write.

## 6. Source-Of-Truth Boundaries

| Surface | Recovery Role |
| --- | --- |
| `GitHubPullRequestCommentProviderWriteReconciliationCandidate` | Bounded provider/local reconciliation posture. |
| `GitHubPullRequestCommentProviderWriteReportDisclosure` | Bounded report disclosure posture derived from explicit inputs. |
| `WorkflowRunEvent` | Durable event-proof source. |
| `SideEffectRecordStore` | Local side-effect lifecycle source. |
| `WorkReport` | In-memory or persisted governed handoff; not an event log. |
| `WorkReportArtifactRecord` | Artifact record that must not overclaim missing proof. |

Recovery must not treat provider references, report text, or operator notes as substitutes for workflow event proof.

## 7. Recovery Posture Taxonomy

The first implementation should classify recovery posture with stable vocabulary.

Recommended categories:

- `event_proof_present`: strict gate can proceed; no recovery needed.
- `event_proof_missing`: provider/local state is bounded, but matching workflow event proof is missing.
- `event_proof_mismatch`: supplied event proof does not match the expected run, side effect, lifecycle state, or provider disclosure.
- `provider_not_called`: no provider call proof exists for a path that required provider-write proof.
- `reconciliation_required`: provider/local state needs explicit operator or future helper resolution.
- `reconciliation_unavailable`: required reconciliation context was not supplied or could not be constructed.
- `provider_response_ambiguous`: provider outcome cannot safely prove success or failure.
- `local_transition_failed`: provider outcome exists, but local lifecycle transition failed.
- `local_state_ambiguous`: local lifecycle state cannot be trusted.
- `unsupported_posture`: supplied posture is outside the recovery helper's supported first slice.

These categories should be enum vocabulary or stable codes, not natural-language parsing.

## 8. Recommended First Implementation Target

Implement a small local recovery classification helper.

Preferred shape:

```text
classify_github_pr_comment_provider_event_proof_recovery(...)
```

or an equivalent explicit helper that accepts:

- one provider disclosure;
- optional matching workflow event proof reference;
- optional reconciliation candidate;
- expected run ID;
- expected side-effect ID;
- expected lifecycle posture;
- redaction metadata and sensitivity.

The helper should return a bounded recovery classification with:

- recovery posture;
- whether operator action is required;
- whether retry must remain blocked;
- whether artifact write may proceed;
- stable non-leaking reason code;
- bounded next action code;
- optional stable citations already supplied by the caller.

Avoid first:

- querying GitHub;
- appending missing events;
- mutating `SideEffectRecordStore`;
- writing report artifacts;
- integrating with CLI;
- changing default executor behavior.

## 9. Recovery Policy

Recommended v1 policy:

| Input State | Recovery Result |
| --- | --- |
| Provider/local completed or failed with matching event proof | `event_proof_present`; artifact gate may proceed. |
| Provider/local completed or failed with missing event proof | `event_proof_missing`; artifact write denied; operator action required if strict proof is required. |
| Provider/local completed or failed with mismatched event proof | `event_proof_mismatch`; artifact write denied; retry blocked. |
| Provider not called | `provider_not_called`; artifact write denied when provider proof was required. |
| Reconciliation required or unavailable | `reconciliation_required` or `reconciliation_unavailable`; artifact write denied; retry blocked until resolved. |
| Provider response ambiguous | `provider_response_ambiguous`; artifact write denied; retry blocked until provider lookup or operator review resolves ambiguity. |
| Provider outcome with local transition failure | `local_transition_failed`; artifact write denied; retry blocked to avoid duplicate provider mutation. |
| Local state ambiguous | `local_state_ambiguous`; artifact write denied; retry blocked. |

The helper should prefer denial plus bounded guidance over any automatic repair.

## 10. Operator Next-Action Vocabulary

The first helper should return stable next-action vocabulary suitable for future CLI/UI/work-report display.

Candidate next actions:

- `no_action_required`;
- `inspect_workflow_events`;
- `inspect_side_effect_record`;
- `inspect_reconciliation_candidate`;
- `manual_provider_lookup_required`;
- `manual_state_repair_required`;
- `retry_blocked_pending_reconciliation`;
- `artifact_write_blocked_pending_event_proof`;

These are guidance labels only. They do not authorize commands, provider calls, state repair, event append, or artifact writes.

## 11. Error Handling

Recovery classification failures should use stable non-leaking errors.

Candidate codes:

- `github_pr_comment_provider_event_proof_recovery.invalid_input`;
- `github_pr_comment_provider_event_proof_recovery.event_mismatch`;
- `github_pr_comment_provider_event_proof_recovery.reconciliation_invalid`;
- `github_pr_comment_provider_event_proof_recovery.unsupported_posture`;
- `github_pr_comment_provider_event_proof_recovery.redaction_invalid`.

Errors must not include provider payloads, comment bodies, PR numbers, repository paths, URLs with private identity, command output, raw IDs when sensitive, tokens, credentials, raw specs, parser payloads, or secret-like values.

## 12. Workflow Semantics

Recovery classification must not change workflow pass/fail status.

It must not:

- mutate `WorkflowRun`;
- append workflow events;
- emit audit events;
- emit observability events;
- call providers;
- mutate side-effect records;
- write report artifacts;
- create filesystem artifacts;
- expose CLI output.

The helper is an input-to-classification function only.

## 13. Privacy And Redaction

Recovery must be reference-first and posture-first.

Allowed:

- stable posture codes;
- stable event IDs supplied by caller;
- stable side-effect IDs supplied by caller, when already part of local model;
- bounded reason and next-action codes;
- redaction metadata after validation.

Forbidden:

- raw GitHub responses;
- comment bodies;
- PR bodies, diffs, review threads, or file contents;
- authorization headers;
- tokens or credentials;
- environment values;
- CI logs;
- command output;
- parser payloads;
- raw spec contents;
- unbounded operator notes.

Debug and serialization must remain redaction-safe.

## 14. Relationship To Provider Lookup

Provider lookup/query reconciliation is a separate future phase.

This recovery helper may say `manual_provider_lookup_required`, but it must not perform that lookup. A later plan can define an explicit provider lookup helper with caller-supplied auth and injected transport, bounded response classification, and no automatic retry.

## 15. Relationship To Artifact Gates

The strict artifact gate should remain the enforcement boundary for artifact writes.

The recovery helper should sit beside it:

- gate denies unsafe artifact write;
- recovery helper explains the bounded reason and next action;
- caller may include recovery posture in an in-memory report or operator handoff;
- no artifact is written until the strict gate passes or a separately approved policy allows disclosure-only artifacts.

## 16. Test Plan

Future implementation tests should cover:

- event-proof-present posture allows recovery classification with no action required;
- missing event proof returns `event_proof_missing`;
- mismatched event proof returns `event_proof_mismatch`;
- provider-not-called returns `provider_not_called`;
- reconciliation-required returns `reconciliation_required`;
- reconciliation-unavailable returns `reconciliation_unavailable`;
- ambiguous provider response returns `provider_response_ambiguous`;
- local transition failure returns `local_transition_failed`;
- local-state ambiguity returns `local_state_ambiguous`;
- retry remains blocked for ambiguous or split-brain states;
- artifact write remains blocked for missing or mismatched event proof;
- helper does not call providers;
- helper does not append events;
- helper does not mutate side-effect records;
- helper does not write report artifacts;
- errors use stable codes and do not leak secret-like markers;
- Debug and serialization do not leak forbidden payloads;
- existing provider-write, event append, report disclosure, artifact gate, executor, WorkReport, and docs tests still pass.

## 17. Proposed Implementation Sequence

1. Add local recovery posture and next-action vocabulary.
2. Add explicit recovery classification input/result types.
3. Add recovery classifier helper with redaction-safe validation.
4. Add focused tests for supported postures, blocked retry, blocked artifact write, and non-mutation behavior.
5. Create an implementation report.
6. Review before any provider lookup, event repair, artifact write composition, CLI, schema, or example work.

## 18. Deferred Work

- Provider lookup/query reconciliation.
- Automatic event repair.
- Manual state repair helper.
- CLI recovery display or command.
- Workflow schema-declared provider artifact policy.
- Automatic executor behavior.
- Report artifact writes from recovery path.
- Examples.
- Hosted/distributed runtime.
- Broader write-capable adapters.
- Reasoning lineage.
- Approval-presentation enforcement.
- Release posture changes.

## 19. Final Recommendation

Proceed next to a small implementation phase: **GitHub PR comment provider event-proof recovery model/helper, local classification only**.

Do not build provider lookup, automatic repair, workflow event append, artifact write composition, CLI behavior, schemas, examples, hosted behavior, broader writes, reasoning lineage, approval-presentation enforcement, or release posture changes in that phase.
