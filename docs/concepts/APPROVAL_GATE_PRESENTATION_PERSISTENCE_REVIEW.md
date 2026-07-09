# Approval Gate Presentation Persistence Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The approval-presentation persistence helper is a bounded local store slice. It
persists validated `ApprovalPresentationRecord` values, provides stable lookup
surfaces, preserves existing approval semantics, and does not introduce
executor enforcement. The next phase should plan opt-in approval-presentation
enforcement before any default approval behavior changes.

## 2. Scope Verification

The phase stayed within the approved persistence-helper scope.

No accidental implementation was found for:

- executor approval-presentation enforcement;
- default approval behavior changes;
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

## 3. Store API Assessment

`ApprovalPresentationRecordStore` is appropriately small and aligned with the
existing local state backend pattern. It exposes:

- `write_approval_presentation_record(...)`;
- `read_approval_presentation_record(...)`;
- `list_approval_presentation_records(...)`;
- `list_approval_presentation_records_for_approval(...)`.

The trait returns owned validated records and does not expose mutable internal
collections. The implementation is provided for both `LocalStateBackend` and
the in-memory test backend, which keeps future executor-adjacent code testable.

## 4. Persistence Boundary Assessment

The local backend stores records under an approval-presentation namespace and
stores a presentation-ID index for stable lookup. File names are derived from
validated encoded IDs rather than raw approval handoff text.

Duplicate presentation IDs are rejected. Conflicting workflow/run identity is
rejected on write for records in the same run. Reads fail closed when the ID
index or record cannot be deserialized or when the indexed run and record
identity disagree.

The helper does not append workflow events, update run snapshots, write approval
decisions, generate reports, create artifacts, execute runtime work, or mutate
approval projections.

## 5. Identity And Lookup Assessment

Lookup by presentation ID is stable and returns `None` when the presentation ID
has no stored index. Listing by run is deterministic because records are sorted
by presentation ID. Listing by run and approval ID validates the approval ID
input and filters existing records without echoing rejected values.

The approval-specific list path checks uniform run/workflow identity across the
records it inspects. The general list path validates that each stored record is
under the requested run ID. A future hardening pass can make the general list
path apply the same cross-record identity check as the approval-specific path.

## 6. Validation And Error-Handling Assessment

The implementation uses stable error codes for duplicate writes, failed writes,
corrupt reads, failed lists, and identity mismatches.

Validation and errors do not leak approval IDs, presentation IDs, raw handoff
text, local paths, corrupt payloads, command output, provider payloads, or
secret-like lookup values.

Evidence construction or executor approval enforcement is not present in this
phase, so there is no partial approval behavior to assess.

## 7. Privacy And Redaction Assessment

The store persists only already validated `ApprovalPresentationRecord` values.
The underlying model keeps handoff content bounded and validates redaction
metadata. Debug behavior remains redaction-safe at the model boundary.

No raw chats, screenshots, provider payloads, command output, CI logs, source
contents, spec contents, environment variable values, credentials,
authorization headers, private keys, or token-like values are introduced by the
persistence helper.

## 8. Default Behavior Assessment

Existing approval APIs remain unchanged. In particular:

- `LocalExecutor::decide_approval(...)` is unchanged;
- existing CLI approval behavior is unchanged;
- approval decisions do not require presentation proof by default;
- missing presentation records do not alter existing workflow execution.

This is the correct boundary for a persistence-only phase.

## 9. Test Quality Assessment

The focused tests cover:

- valid local write, read, and list behavior;
- listing by run and approval ID;
- duplicate presentation ID rejection without leakage;
- secret-like approval ID lookup rejection without leakage;
- corrupt stored record read failure without payload leakage;
- persistence does not mutate runtime events or approval projections;
- in-memory and local backend store contract coverage;
- existing approval-presentation model behavior.

The tests are meaningful for this phase. They do not overclaim executor
enforcement, CLI approval card behavior, WorkReport citations, or
high-assurance approval controls.

## 10. Documentation Review

Documentation correctly states:

- approval-presentation record persistence is implemented;
- executor approval-presentation enforcement is not implemented;
- default approval behavior is unchanged;
- automatic and hidden approval are not implemented;
- CLI mutation behavior is not implemented;
- workflow schema fields are not implemented;
- examples are not updated;
- high-assurance approval integration is not implemented;
- WorkReport citation/disclosure changes are not implemented;
- provider writes, side effects, hosted behavior, reasoning lineage, and release
  posture changes remain unsupported.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add a general-list corruption test for mixed workflow/schema identity under
  one run directory and consider applying the same cross-record identity check
  used by `list_approval_presentation_records_for_approval(...)`.
- Add tests for read behavior when the presentation-ID index points to a
  missing record file.
- Add tests for write rollback when ID-index creation succeeds but record write
  fails, if a small deterministic fixture can cover it without brittle filesystem
  permissions.
- Plan how opt-in enforcement should handle ambiguous multiple presentation
  records for one approval ID.

## 13. Recommended Next Phase

Recommended next phase: opt-in approval-presentation enforcement planning.

The persistence helper is ready for the next design decision: how an explicit
approval path should require matching presentation proof, how freshness/staleness
should be represented, and how failures should remain non-leaking without
changing existing approval behavior by default.

## 14. Validation Commands Run

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test approval_presentation` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 15. Dogfood Governance Summary

- Dogfood workflow ID: `dg/review`.
- Run ID: `run-1783589456544752000-2`.
- Approval ID: `approval/run-1783589456544752000-2/review-scope-approved`.
- Approval outcome: granted by the delegated maintainer.
- Event summary: completed with 39 events, including one approval request, one
  approval grant, eight policy decisions, six scheduled steps, six skill
  invocation requests, six skill invocation starts, six skill invocation
  successes, and terminal completion.
- Out-of-kernel work: repository file edits, validation commands, git
  operations, and PR operations remain performed by Codex/human tooling outside
  the kernel and are disclosed here.
