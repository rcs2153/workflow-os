# GitHub Adapter Posture

Workflow OS implements a GitHub read-only adapter in Phase 2. Write-capable GitHub behavior remains future work and must not be implied by the read-only adapter.

The GitHub adapter implements generic adapter contracts rather than special-casing workflow state. It must not make Workflow OS a GitHub automation tool.

The first future write candidate is GitHub pull request comment. Workflow OS now has a model-only request/response boundary for that candidate, preflight composition is implemented as model/helper-only in [GitHub PR Comment Preflight Composition Plan](../implementation-plans/github-pr-comment-preflight-composition-plan.md), fixture-backed adapter validation is implemented as a no-provider-call helper in [GitHub PR Comment Fixture Adapter Plan](../implementation-plans/github-pr-comment-fixture-adapter-plan.md), in-memory proposed `SideEffectRecord` composition is implemented in [GitHub PR Comment Proposed SideEffectRecord Composition Plan](../implementation-plans/github-pr-comment-side-effect-record-composition-plan.md), explicit proposed record persistence through `SideEffectRecordStore` is implemented in [GitHub PR Comment Proposed SideEffectRecord Persistence Plan](../implementation-plans/github-pr-comment-side-effect-record-persistence-plan.md), workflow event/audit projection for persisted proposed records is planned in [GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan](../implementation-plans/github-pr-comment-side-effect-event-audit-projection-plan.md), the first pure proposed-event construction helper is implemented in [GitHub PR Comment SideEffect Event Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_HELPER_REPORT.md), explicit append planning for feeding persisted proposed records into the existing executor SideEffect event input path is documented in [GitHub PR Comment Proposed SideEffect Event Append Plan](../implementation-plans/github-pr-comment-side-effect-event-append-plan.md), and the first bridge helper is implemented in [GitHub PR Comment SideEffect Event Append Helper Report](../concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_HELPER_REPORT.md). No GitHub provider write call, runtime write execution, CLI write command, schema support, or live sandbox write is implemented.

## Read-Only Scope

The implemented read-only adapter supports:

- repository metadata
- default branch
- file contents metadata and reference summaries by path/ref
- pull request metadata
- pull request diff summary
- pull request changed files
- pull request comments as read-only data
- check run status summaries

Live GitHub mode is opt-in. Fixture tests run without credentials.

## Future Write Requirements

A GitHub adapter must:

- declare read and write capabilities explicitly
- require policy allow or approval before writes
- use idempotency keys for write operations
- store references to repositories, issues, pull requests, commits, or checks rather than raw sensitive payloads
- classify authentication, permission, rate limit, not found, validation, transient, and unknown failures
- report outcomes through core runtime interfaces

## Deferred Behavior

No branch creation, commits, pull request creation, pull request comment provider call, review requests, label changes, merges, PR closure, check reruns, workflow dispatch, OAuth flows, or webhook handling are implemented.
