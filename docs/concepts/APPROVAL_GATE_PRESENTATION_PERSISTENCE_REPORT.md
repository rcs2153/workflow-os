# Approval Gate Presentation Persistence Report

## 1. Executive Summary

Workflow OS now has a local persistence helper for
`ApprovalPresentationRecord` values.

The helper can write, read, and list validated approval-presentation proof
records without mutating workflow events, approval projections, executor state,
CLI behavior, schemas, examples, or runtime approval semantics.

This is a persistence-only phase. It does not enforce presentation proof before
approval decisions.

## 2. Scope Completed

- Added `ApprovalPresentationRecordStore`.
- Implemented the store for `LocalStateBackend`.
- Implemented the store for the in-memory test backend.
- Added local state directories for approval-presentation records and
  presentation-ID indexes.
- Added lookup by `ApprovalPresentationId`.
- Added listing by `WorkflowRunId`.
- Added listing by `WorkflowRunId` and approval ID.
- Added duplicate write protection.
- Added corrupt read failure handling.
- Added focused tests for persistence behavior and non-mutation.
- Exported the store trait and approval ID lookup validator.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- executor approval-presentation enforcement;
- changes to default approval behavior;
- automatic approval;
- hidden approval;
- CLI mutation behavior;
- workflow schema fields;
- examples;
- high-assurance approval integration;
- WorkReport citation or disclosure changes;
- provider writes;
- side effects;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Helper API Summary

`ApprovalPresentationRecordStore` provides:

- `write_approval_presentation_record(...)`;
- `read_approval_presentation_record(...)`;
- `list_approval_presentation_records(...)`;
- `list_approval_presentation_records_for_approval(...)`.

The local backend stores validated records under an approval-presentation state
namespace and stores a presentation-ID index for stable lookup. Records are
returned as owned validated values.

## 5. Persistence Boundary Summary

The helper persists only validated `ApprovalPresentationRecord` values. It
rejects duplicate presentation IDs and rejects conflicting workflow/run identity
among records for the same run.

Read paths fail closed when stored JSON cannot be deserialized or does not match
the requested presentation/run identity.

Persistence does not append workflow events, update run snapshots, write
approval decisions, generate reports, create artifacts, or execute runtime work.

## 6. Redaction And Privacy Summary

The persistence layer reuses the redaction-safe model boundary. It does not
store raw chats, screenshots, provider payloads, command output, CI logs, source
contents, spec contents, environment variables, credentials, authorization
headers, private keys, or token-like values.

Errors use stable codes and avoid echoing approval IDs, presentation IDs, raw
handoff text, local paths, corrupt payloads, or secret-like lookup inputs.

## 7. Test Coverage Summary

Tests cover:

- valid local write, read, and list behavior;
- listing by run and approval ID;
- duplicate presentation ID rejection without leakage;
- secret-like approval ID lookup rejection without leakage;
- corrupt stored record read failure without payload leakage;
- persistence does not mutate runtime events or approval projections;
- in-memory and local backend store contract coverage;
- existing approval-presentation model tests.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test approval_presentation` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Dogfood Governance Summary

- Dogfood workflow ID: `dg/implement`.
- Run ID: `run-1783586951447296000-2`.
- Approval ID: `approval/run-1783586951447296000-2/implementation-approved`.
- Approval outcome: granted by the delegated maintainer.
- Event summary: completed with 39 events, including one approval request,
  one approval grant, eight policy decisions, six scheduled steps, six skill
  invocation requests, six skill invocation starts, six skill invocation
  successes, and terminal completion.
- Out-of-kernel work: repository edits, shell validation commands, formatting,
  tests, documentation checks, git operations, and PR operations remain
  performed by Codex/human tooling outside the kernel and are disclosed here.

## 10. Remaining Known Limitations

- Approval decisions do not yet require presentation proof.
- No opt-in executor-adjacent enforcement path exists yet.
- No freshness or staleness policy is enforced.
- No WorkReport citation or disclosure integration exists.
- No CLI approval card or approval-presentation command exists.
- Dogfood runner output is not yet automatically persisted as presentation
  proof.

## 11. Recommended Next Phase

Recommended next phase: approval gate presentation persistence review.

The review should verify store scope, non-mutation, redaction safety, tests, and
preservation of default approval behavior before any opt-in enforcement path is
implemented.
