# GitHub PR Comment Proposed SideEffectRecord Composition Plan

Status: Implemented as an in-memory helper. This plan follows the accepted fixture-backed GitHub pull request comment helper review. It defines the no-provider-call boundary for composing a validated proposed `SideEffectRecord` for a GitHub PR comment write candidate from existing request, preflight, and fixture/dry-run posture. The helper implementation is documented in [GitHub PR Comment Proposed SideEffectRecord Composition Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_COMPOSITION_HELPER_REPORT.md). Explicit proposed record persistence through `SideEffectRecordStore` is implemented in [GitHub PR Comment Proposed SideEffectRecord Persistence Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_PERSISTENCE_HELPER_REPORT.md), following [GitHub PR Comment Proposed SideEffectRecord Persistence Plan](github-pr-comment-side-effect-record-persistence-plan.md). This plan does not implement provider mutation, runtime side-effect execution, lifecycle transitions beyond proposed records, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 1. Executive Summary

Workflow OS now has:

- an adapter-neutral write preflight helper;
- a model-only GitHub pull request comment request/response boundary;
- a preflighted GitHub PR comment write helper;
- a fixture-only GitHub PR comment validation helper;
- a validated `SideEffectRecord` model and explicit local `SideEffectRecordStore`;
- an in-memory GitHub PR comment proposed `SideEffectRecord` composition helper.

The next safe step was not a live GitHub write. This plan defined the composition boundary for creating a proposed `SideEffectRecord` from an already validated GitHub PR comment write candidate, so the side-effect intent can become a governed record before any provider write is considered.

The first in-memory composition helper is implemented. Persistence remains deferred.

## 2. Goals

- Define how a future helper should compose a proposed `SideEffectRecord` for a GitHub PR comment write candidate.
- Require existing validated inputs rather than raw strings or raw provider payloads.
- Preserve preflight-before-side-effect-record composition.
- Preserve no-provider-call posture.
- Preserve no runtime side-effect execution.
- Preserve no workflow event or audit event append.
- Preserve no report artifact write.
- Ensure the proposed record carries stable identity, target, capability, authority, idempotency, references, redaction, sensitivity, and bounded summary.
- Prepare a small implementation prompt for a model/helper-only phase.

## 3. Non-Goals

This plan does not authorize:

- GitHub provider calls;
- pull request comment creation;
- live sandbox writes;
- proposed record persistence;
- provider auth handling;
- OAuth app behavior or webhook ingestion;
- runtime side-effect execution;
- attempted, completed, or failed SideEffect lifecycle transitions;
- workflow event appends;
- audit event emission;
- report artifact writes;
- automatic executor integration;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented baseline:

- `SideEffectRecord`, `SideEffectRecordDefinition`, lifecycle, authority, idempotency, target, reference, sensitivity, and redaction models.
- `SideEffectRecordStore` and local filesystem persistence for validated records.
- Store-backed discovery, WorkReport SideEffect citation, artifact referential integrity, and approval-linkage helpers.
- Adapter-neutral `AdapterWritePreflightRequest` and `AdapterWritePreflightDecision`.
- `GitHubPullRequestCommentWriteRequest`.
- `GitHubPullRequestCommentPreflightedWrite`.
- `GitHubPullRequestCommentFixture`.
- `validate_github_pr_comment_fixture_write(...)`.

Not implemented:

- GitHub provider mutation.
- Runtime side-effect attempt/completion/failure.
- Automatic proposed record persistence from GitHub write candidates.
- Workflow event/audit projection for this write candidate.
- CLI or schema surface.

## 5. Proposed Composition Boundary

The future helper should be explicit, local, and model-only:

```text
compose_github_pr_comment_proposed_side_effect_record(
    preflighted: &GitHubPullRequestCommentPreflightedWrite,
    fixture_response: Option<&GitHubPullRequestCommentWriteResponse>,
    input: GitHubPullRequestCommentSideEffectRecordInput,
) -> Result<SideEffectRecord, WorkflowOsError>
```

Exact names should follow repository conventions during implementation.

The helper should:

- accept a preflighted write value, not a raw request;
- optionally accept the fixture/dry-run response when the caller wants to cite fixture validation;
- accept explicit caller-supplied record context that is not already present on the write request;
- return a validated `SideEffectRecord` in `Proposed` lifecycle state;
- produce no store writes unless a later separately scoped helper explicitly persists the record;
- expose no provider-call, workflow-event, SideEffect-transition, report-artifact, file-write, or CLI authority.

## 6. Required Inputs

The helper should derive from `GitHubPullRequestCommentPreflightedWrite`:

- adapter ID;
- integration ID;
- correlation ID;
- workflow ID;
- workflow version;
- schema version;
- spec hash;
- run ID;
- optional step ID;
- actor;
- target repository and pull request reference;
- SideEffect ID;
- idempotency key;
- mode;
- preflight policy decision;
- preflight policy references;
- preflight approval references;
- request summary;
- sensitivity;
- redaction metadata.

The helper input should provide only the fields not safely derivable from the request/preflight:

- created timestamp;
- optional skill ID;
- optional skill version;
- optional additional stable references;
- optional bounded proposed-record summary override if needed;
- optional system actor only when actor is absent or the proposal is system-generated.

The helper must not accept:

- raw GitHub tokens;
- authorization headers;
- raw provider payloads;
- raw PR bodies;
- raw diffs;
- raw CI logs;
- raw command output;
- raw file contents;
- raw spec contents;
- environment variable values;
- unbounded prompts;
- secret-like values.

## 7. SideEffectRecord Mapping

The proposed record should map as follows:

| SideEffect field | Mapping |
| --- | --- |
| `side_effect_id` | Request SideEffect ID. |
| `lifecycle_state` | `Proposed`. |
| `target.kind` | `AdapterResource` or `ExternalResource`; prefer `AdapterResource` for first slice. |
| `target.reference` | Bounded GitHub PR target reference from `GitHubPullRequestCommentTarget::reference()`. |
| `capability` | `GitHubWrite`. |
| `authority.decision` | Derived from preflight policy posture: likely `AllowedByPolicy`, `RequiresApproval`, or denied/unsupported mapping if future denied-record composition is planned. First slice should target ready proposed records only. |
| `authority.policy_references` | Preflight policy references. |
| `authority.approval_references` | Preflight approval references. |
| `actor` | Request actor. |
| `system_actor` | Optional only when explicitly supplied and valid. |
| `workflow_id` | Request workflow ID. |
| `workflow_version` | Request workflow version. |
| `schema_version` | Request schema version. |
| `spec_hash` | Request spec hash. |
| `run_id` | Request run ID. |
| `step_id` | Request step ID. |
| `skill_id` / `skill_version` | Optional explicit input. |
| `adapter_id` | Request adapter ID. |
| `adapter_kind` | `GitHub`. |
| `integration_id` | Request integration ID. |
| `idempotency` | Request idempotency key, scoped to run for first slice. |
| `references` | Policy, approval, optional fixture response, and optional caller-supplied stable references. |
| `outcome_reference` | None for proposed records. |
| `created_at` | Explicit input timestamp. |
| `updated_at` | None for first proposed record. |
| `correlation_id` | Request correlation ID. |
| `summary` | Bounded request or explicit proposed-record summary. |
| `reason_codes` | Empty for ready proposed records. |
| `sensitivity` | Conservative maximum of request and helper input if applicable. |
| `redaction` | Validated request redaction metadata or explicitly supplied safe redaction metadata. |

## 8. Authority Mapping Policy

The first implementation should accept only ready preflight posture and produce proposed records that can later be attempted only by a separately reviewed runtime path.

Recommended first-slice mapping:

- preflight allowed without required approval -> `AllowedByPolicy`;
- preflight allowed with approval references -> `ApprovedByHuman` only if the preflight helper has already required and accepted those references;
- approval required but not granted -> do not compose an attemptable proposed record yet; fail closed or defer to denied/skipped planning;
- denied policy -> fail before proposed-record composition because `GitHubPullRequestCommentPreflightedWrite` cannot be constructed;
- unsupported capability -> fail before proposed-record composition.

Open question for implementation review: whether a future helper should also compose denied/skipped SideEffect records for rejected write candidates. That should not be included in the first implementation unless explicitly scoped.

## 9. Fixture Response Relationship

Fixture or dry-run response is useful as validation evidence for the proposed record, but it must not become proof of provider mutation.

Rules:

- `FixtureValidated` and `DryRunValidated` may be cited as internal validation references only if a stable reference vocabulary already exists or is explicitly provided.
- Do not store response summary as raw payload.
- Do not create provider comment references.
- Do not claim `Attempted`, `Completed`, or `Failed` lifecycle state from fixture validation.
- Do not use fixture response to bypass preflight, policy, approval, or idempotency requirements.

If the existing `SideEffectReferenceKind` vocabulary lacks a precise fixture-validation reference kind, the first implementation may use existing stable references supplied by the caller or omit fixture response citation.

## 10. Persistence Posture

The first implementation should return an in-memory validated `SideEffectRecord`.

Do not persist automatically.

If persistence is added in a later phase, it should be an explicit helper:

```text
compose_and_persist_github_pr_comment_proposed_side_effect_record(
    store: &impl SideEffectRecordStore,
    ...
) -> Result<SideEffectRecord, WorkflowOsError>
```

That later helper must:

- use an explicit caller-supplied store;
- write only a validated proposed record;
- not mutate workflow run state;
- not append workflow events;
- not emit audit events;
- not create report artifacts;
- preserve duplicate ID and immutable run identity checks from `SideEffectRecordStore`.

## 11. Workflow Event And Audit Posture

This composition helper must not append workflow events or emit audit events.

Later event/audit integration should decide:

- whether a proposed record must exist before a `SideEffectProposed` workflow event is appended;
- whether a `SideEffectProposed` workflow event can be appended from the composed record;
- whether audit projection should cite the record, event, or both;
- how replay handles missing or corrupt records.

Until then, the composed record is a validated object, not runtime history.

## 12. Error Handling

Errors must use stable, non-leaking codes.

Candidate codes:

- `github_pr_comment_side_effect_record.preflight.not_ready`;
- `github_pr_comment_side_effect_record.mode.unsupported`;
- `github_pr_comment_side_effect_record.authority.unsupported`;
- `github_pr_comment_side_effect_record.target.invalid`;
- `github_pr_comment_side_effect_record.reference.invalid`;
- `github_pr_comment_side_effect_record.record.invalid`.

Errors must not include:

- owner/repository names;
- pull request body or diff contents;
- comment body;
- raw fixture response text;
- SideEffect IDs;
- idempotency keys;
- provider payloads;
- command output;
- parser payloads;
- spec contents;
- redaction metadata;
- secret-like values.

## 13. Privacy And Redaction

The composition helper must preserve:

- reference-only target posture;
- bounded summaries only;
- no raw provider payloads;
- no raw PR descriptions;
- no raw diffs;
- no raw CI logs;
- no command output;
- no provider auth values;
- no raw source file contents;
- no raw spec contents;
- no unbounded prompt text;
- no secret-like metadata.

`Debug` output for any new input or composition result wrapper must redact target internals, SideEffect ID, idempotency key, run identity, spec hash, comment body, summaries, references, and redaction metadata.

## 14. Test Plan

Future implementation tests should cover:

- valid preflighted fixture write composes a proposed `SideEffectRecord`;
- valid preflighted dry-run write composes a proposed `SideEffectRecord`;
- raw write request is not accepted by the helper API;
- live sandbox mode remains unsupported;
- record lifecycle is `Proposed`;
- capability is `GitHubWrite`;
- target reference is bounded and redaction-safe;
- authority references preserve preflight policy references;
- approval references preserve preflight approval references where applicable;
- idempotency binding uses the request idempotency key;
- workflow/run/spec identity is preserved;
- adapter/integration identity is preserved;
- fixture response does not create provider references;
- no attempted/completed/failed lifecycle state is produced;
- no store write occurs;
- no workflow event or audit event is appended;
- no report artifact is written;
- secret-like summaries, references, and redaction metadata fail without leakage;
- debug output does not leak target, comment body, SideEffect ID, idempotency key, run ID, fixture reference, or redaction metadata;
- existing provider write tests still pass;
- existing side-effect tests still pass;
- docs check passes.

## 15. Proposed Implementation Sequence

Recommended small future phases:

1. Add an in-memory composition helper and input type that returns a validated proposed `SideEffectRecord`.
2. Add focused provider-write and side-effect tests for mapping, non-execution posture, and redaction.
3. Review the composition helper.
4. Plan explicit optional persistence through `SideEffectRecordStore`.
5. Review persistence before any live sandbox write planning.

Do not combine these phases with live provider mutation.

## 16. Open Questions

- Should the first composition helper map approval-bearing ready preflight posture to `ApprovedByHuman` or `RequiresApproval` with approval references?
- Should denied/skipped SideEffect record composition be a separate helper?
- Should fixture response validation be cited through a dedicated reference kind, or only through caller-supplied stable references?
- Should proposed record persistence be required before any live sandbox write, or can a live sandbox plan require only an in-memory proposed record plus later persistence?
- Should a future `SideEffectProposed` workflow event require the persisted record to exist first?

## 17. Final Recommendation

The next implementation phase should be:

```text
GitHub PR comment proposed SideEffectRecord composition helper, in-memory only.
```

It must not implement provider calls, live writes, provider auth, lifecycle attempts/completions/failures, workflow events, audit events, report artifacts, CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.
