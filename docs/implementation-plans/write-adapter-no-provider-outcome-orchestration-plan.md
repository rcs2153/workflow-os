# Write-Adapter No-Provider Outcome Orchestration Plan

Status: Implemented as a no-provider-call local helper in [Write-Adapter No-Provider Outcome Orchestration Report](../concepts/WRITE_ADAPTER_NO_PROVIDER_OUTCOME_ORCHESTRATION_REPORT.md). This plan follows the accepted [Write-Adapter Orchestration Helper Review](../concepts/WRITE_ADAPTER_ORCHESTRATION_HELPER_REVIEW.md). It defines the local composition slice after attempted-state orchestration: explicit completed/failed no-provider outcome orchestration for the GitHub PR comment candidate. It does not implement provider writes, runtime side-effect execution, CLI mutation behavior, workflow schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, auth material loading, or release posture changes.

## 1. Executive Summary

Workflow OS now has a reviewed no-provider-call helper that can persist a proposed GitHub PR comment `SideEffectRecord`, validate approval linkage, and transition the record to `Attempted`.

The next gap is still not a live provider call. The next gap is local outcome closure: how an explicit non-provider outcome reference should transition an already attempted record to `Completed` or `Failed`, expose reference-only event payloads, and preserve report/artifact citation obligations without claiming that GitHub was actually mutated.

This plan has been implemented only as an explicit local helper. Live provider-call outcome handling remains future work.

## 2. Goals

- Define explicit no-provider completed/failed outcome orchestration.
- Preserve `SideEffectRecordStore` as the source of truth for lifecycle state.
- Preserve `WorkflowRunEvent` append as a separate explicit caller/executor boundary.
- Require attempted state before completed or failed outcome closure.
- Use stable outcome references or stable failure reason codes only.
- Avoid raw provider responses, raw command output, raw logs, raw spec contents, and secrets.
- Keep outcome closure local, deterministic, and redaction-safe.
- Prepare for a later live provider-call plan without authorizing live calls.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- live GitHub pull request comment creation;
- any provider mutation;
- runtime side-effect execution;
- automatic executor writes;
- automatic completed/failed transitions from default executor paths;
- automatic workflow event append;
- report artifact writing;
- CLI mutation commands, rendering, or export;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- auth material loading;
- production credential management;
- OAuth app behavior or webhook ingestion;
- RBAC, IdP, enterprise stewardship, quorum approval, or revocation enforcement;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented and reviewed:

- adapter write preflight;
- GitHub PR comment request/response models;
- fixture/dry-run validation with no provider calls;
- proposed `SideEffectRecord` composition and persistence;
- proposed SideEffect event construction and explicit executor append input path;
- approval-side-effect linkage and store-backed approval linkage;
- pure and store-backed lifecycle transitions for attempted/completed/failed;
- explicit executor append path for attempted/completed/failed lifecycle events;
- WorkReport and report artifact SideEffect citation/integrity helpers;
- no-provider-call attempted orchestration helper.

Still missing:

- a no-provider helper that closes an attempted write candidate as completed or failed from explicit local outcome input;
- an orchestration result shape that reports proposed, attempted, and outcome records together;
- policy for when fixture/dry-run outcomes may be used as local validation outcomes without claiming provider success;
- explicit citation obligations for completed/failed local outcome closure.

## 5. Recommended First Implementation Target

Recommended first implementation: extend the GitHub PR comment no-provider orchestration lane with explicit outcome closure helpers.

Possible API shapes:

- `orchestrate_github_pr_comment_write_completed_without_provider_call(...)`;
- `orchestrate_github_pr_comment_write_failed_without_provider_call(...)`;
- or one input enum-based helper for `Completed` and `Failed`.

The first implementation should accept an already attempted record or store-backed attempted state. It should not perform proposed or attempted orchestration again unless a caller explicitly uses the existing attempted helper first.

## 6. Required Input Model

The future helper should accept explicit inputs only:

- caller-supplied `SideEffectRecordStore`;
- `SideEffectId` for an existing attempted record;
- expected workflow/run identity if needed for event payload validation;
- completed or failed outcome kind;
- transition timestamp;
- stable outcome reference for completed outcomes;
- stable failure reason code and optional failure reference for failed outcomes;
- additional stable references;
- evidence reference count;
- optional policy/approval/report citation references;
- sensitivity and redaction posture only where not already carried by the record.

It must not read hidden global state, load credentials, call providers, inspect GitHub, inspect local git, read files, invoke shell commands, or infer outcome from external systems.

## 7. Completed Outcome Rules

A completed no-provider outcome means: "a local non-provider validation outcome says this attempted candidate is complete for the current no-provider slice."

It must not mean: "GitHub was mutated."

Rules:

- prior stored record must exist and be `Attempted`;
- prior record must validate;
- capability must be `GitHubWrite`;
- target must remain the original GitHub PR comment target reference;
- outcome reference must be present, bounded, stable, and non-secret-like;
- outcome reference kind must distinguish fixture/dry-run/local validation from future provider references;
- transition must use `transition_side_effect_to_completed_in_store(...)`;
- returned event payload must remain unappended until an explicit caller appends it;
- result must state provider call was not performed.

## 8. Failed Outcome Rules

A failed no-provider outcome means: "a local non-provider validation outcome says this attempted candidate failed before any live provider mutation."

Rules:

- prior stored record must exist and be `Attempted`;
- prior record must validate;
- failure reason code must be stable and non-secret-like;
- optional failure reference must be bounded, stable, and non-secret-like;
- transition must use `transition_side_effect_to_failed_in_store(...)`;
- returned event payload must remain unappended until an explicit caller appends it;
- result must state provider call was not performed.

Initial failure reason vocabulary should include:

- `fixture.validation_failed`;
- `fixture.mismatch`;
- `dry_run.validation_failed`;
- `orchestration.precondition_failed`;
- `orchestration.outcome_reference_invalid`;
- `orchestration.unknown_failed`.

Provider failure reason codes remain future work.

## 9. Fixture And Dry-Run Outcome Policy

Fixture and dry-run results may support no-provider outcome closure only when the helper labels them honestly.

Allowed:

- fixture-validated completed local outcome;
- dry-run-validated completed local outcome;
- fixture/dry-run failed local validation outcome.

Forbidden:

- treating fixture success as provider success;
- creating provider comment references from fixture data;
- using fixture or dry-run responses to claim GitHub mutation;
- copying raw provider payloads, raw fixture bodies, or raw command output.

## 10. Event Boundary

The helper should return reference-only lifecycle event payloads from the existing transition helpers.

It must not:

- append workflow events;
- mutate `WorkflowRun`;
- emit audit events;
- emit observability events;
- create report artifacts;
- infer workflow success or failure.

If a future caller wants run-local event proof, it should use the explicit executor lifecycle append path already implemented and reviewed.

## 11. Report And Artifact Citation Boundary

The helper should return citation obligations or stable references, not artifacts.

Required references for a closed no-provider outcome:

- proposed `SideEffectRecord` ID if available to the caller;
- attempted `SideEffectRecord` ID;
- completed or failed `SideEffectRecord` ID;
- lifecycle transition event payload ID where available;
- policy decision reference;
- approval reference when authority required approval;
- outcome reference or failure reason code;
- evidence reference count.

Report artifact writing remains an explicit separate helper. The plan does not add artifact writes.

## 12. Error Handling

Errors must be structured and non-leaking.

Candidate stable codes:

- `github_pr_comment_write_outcome.record_missing`;
- `github_pr_comment_write_outcome.unsupported_lifecycle`;
- `github_pr_comment_write_outcome.outcome_reference_missing`;
- `github_pr_comment_write_outcome.failure_reason_missing`;
- `github_pr_comment_write_outcome.lifecycle_transition_failed`;
- `github_pr_comment_write_outcome.provider_call_not_supported`;
- `github_pr_comment_write_outcome.partial_state_detected`.

Errors must not include raw targets, GitHub URLs, comment bodies, provider payloads, command output, CI logs, file paths, snippets, auth material, headers, private keys, token-like values, or secret-like strings.

## 13. Privacy And Redaction

The implementation must stay reference-first:

- no raw provider payloads;
- no raw GitHub bodies or diffs;
- no raw command output;
- no raw CI logs;
- no raw parser output;
- no raw spec contents;
- no environment values;
- no credentials;
- no authorization headers;
- no private keys;
- no token-like values.

Debug output should show lifecycle posture and bounded counts, not raw IDs or user-supplied summaries.

## 14. Test Plan

Future implementation tests should cover:

- completed no-provider outcome transitions an attempted store record to `Completed`;
- failed no-provider outcome transitions an attempted store record to `Failed`;
- missing attempted record fails closed;
- proposed record cannot transition directly to completed/failed through the outcome helper;
- denied/skipped records cannot transition to completed/failed;
- completed outcome requires stable outcome reference;
- failed outcome requires stable reason code or failure reference;
- fixture success is not represented as provider success;
- provider references are rejected in no-provider outcome mode;
- lifecycle event payload is returned but not appended;
- no provider calls occur;
- no workflow events, audit events, report artifacts, filesystem writes, or CLI output are emitted;
- Debug and errors do not leak raw payloads or secret-like values;
- existing provider-write, side-effect, executor, WorkReport, and report artifact tests still pass;
- `cargo test --workspace` and `npm run check:docs` pass.

## 15. Proposed Implementation Sequence

Recommended small phases:

1. Implement completed/failed no-provider outcome input/result types and helper.
2. Reuse existing store-backed completed/failed lifecycle transition helpers.
3. Return reference-only event payloads and citation obligations without appending events.
4. Add focused tests for completed, failed, invalid prior state, invalid outcome, non-leakage, and no provider call.
5. Review.
6. Only after review, plan whether no-provider outcome orchestration should compose with report/artifact gates.
7. Only after that, reconsider live sandbox provider-call planning.

## 16. Deferred Work

Deferred until separately planned and reviewed:

- live provider calls;
- sandbox smoke writes;
- production credential posture;
- provider failure classification;
- provider retry and duplicate-prevention behavior;
- automatic executor orchestration;
- automatic workflow event append;
- automatic report artifact writing;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted/distributed runtime;
- RBAC/IdP/enterprise stewardship;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy.

## 17. Open Questions

- Should completed and failed outcome helpers be separate functions or one enum-driven helper?
- Should fixture/dry-run completion use a distinct `SideEffectOutcomeReferenceKind` value before provider references exist?
- Should the helper require proposed and attempted event proof, or only return outcome event payloads?
- Should outcome orchestration results become future WorkReport citation targets?
- How should partial store/event mismatch be disclosed when callers append returned events later?
- What is the smallest safe boundary before live sandbox provider-call planning becomes useful?

## 18. Final Recommendation

Proceed next to **completed/failed no-provider outcome orchestration implementation**.

The first implementation should close attempted records as completed or failed from explicit local no-provider outcome inputs only. It must not call providers, load credentials, append workflow events, write report artifacts, add CLI behavior, add schemas, update examples, add hosted behavior, implement reasoning lineage, enable recursive agents or agent swarms, enable Level 3/4 autonomy, or change release posture.

## 19. Dogfood Governance

This planning phase is governed by:

- workflow: `dg/d`;
- run: `run-1783275800112959000-2`;
- approval: `approval/run-1783275800112959000-2/planning-approved`;
- approval actor: `user/delegated-maintainer`;
- approved scope: planning document and small status links only;
- strict non-goals: no provider writes, runtime mutation, CLI, schemas, examples, hosted behavior, lineage, autonomy, auth loading, or code implementation;
- approval outcome: granted by `user/delegated-maintainer`;
- event summary: 39 events total; 1 approval; 0 retries; 0 escalations;
- validation summary: `npm run check:docs` passed;
- out-of-kernel work: repository edits, docs validation commands, and documentation updates were performed by Codex outside the kernel execution layer and disclosed here.
