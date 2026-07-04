# GitHub PR Comment Write Boundary Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The GitHub pull request comment write boundary is appropriately model-only. It adds narrow request, response, target, mode, outcome, and preflight-definition helper vocabulary for a future GitHub PR comment write candidate without calling GitHub, mutating provider state, appending events, transitioning SideEffect lifecycle state, adding CLI behavior, adding schemas, updating examples, or changing release posture.

The implementation is safe to use as a validated planning and fixture boundary. It should not be treated as write execution.

## 2. Scope Verification

The phase stayed within approved model-only scope.

No accidental implementation was found for:

- GitHub provider calls;
- pull request comment creation;
- write-capable adapter execution;
- fixture-backed adapter execution;
- live sandbox write smoke;
- runtime side-effect execution;
- SideEffect attempted/completed/failed lifecycle transitions;
- automatic workflow event appends;
- automatic report generation or artifact writing;
- CLI write commands or flags;
- workflow schema fields;
- examples;
- hosted or distributed runtime behavior;
- production credential management;
- OAuth app behavior or webhook ingestion;
- RBAC, IdP, SSO, SCIM, enterprise admin controls, or quorum approval;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

The explicit no-authority helpers on request/response are important: provider calls, workflow event appends, and SideEffect lifecycle transitions remain disallowed by this model.

## 3. Model Assessment

The model is narrow, domain-specific, and appropriate for a first write candidate.

Implemented vocabulary covers:

- `GitHubPullRequestCommentTarget`;
- `GitHubPullRequestCommentWriteMode`;
- `GitHubPullRequestCommentWriteOutcome`;
- `GitHubPullRequestCommentWriteRequestDefinition`;
- `GitHubPullRequestCommentWriteRequest`;
- `GitHubPullRequestCommentWriteResponseDefinition`;
- `GitHubPullRequestCommentWriteResponse`;
- `GitHubPullRequestCommentPreflightDefinitionInput`;
- `github_pr_comment_preflight_definition(...)`.

The boundary is not a generic provider write framework and does not overgeneralize beyond the approved first candidate. That restraint is correct.

## 4. Request Boundary Assessment

The request captures the expected governed write context:

- adapter and integration identity;
- correlation ID;
- workflow ID and version;
- schema version;
- spec hash;
- run ID and optional step ID;
- actor;
- bounded GitHub target;
- bounded comment body;
- bounded summary;
- proposed SideEffect ID;
- idempotency key;
- write mode;
- matching adapter write preflight request;
- sensitivity;
- redaction metadata.

Validation rejects unsafe target shapes, zero pull request numbers, empty or oversized comment/summary values, secret-like strings, forbidden raw payload markers, and unsafe redaction metadata.

The request also validates that the embedded preflight request matches the GitHub PR comment capability, target reference, SideEffect ID, and idempotency key. This is the right model-level alignment check for this phase.

## 5. Response Boundary Assessment

The response model is explicit about fixture, dry-run, and future provider outcomes.

Validation correctly prevents fixture and dry-run responses from carrying provider references or provider errors. Future provider success requires a provider reference, and future provider failure requires a bounded provider error code.

This creates a useful response envelope for fixture and later sandbox work without implying that provider execution already exists.

## 6. Preflight Alignment Assessment

The helper builds a matching `AdapterWritePreflightRequestDefinition` for the GitHub PR comment target, SideEffect ID, idempotency key, policy references, approval references, summary, sensitivity, and redaction metadata.

The model validates preflight alignment, but does not execute `preflight_adapter_write(...)`. That is correct for this phase. The next runtime-facing work should compose this request model with actual preflight execution before any fixture-backed or live provider path.

## 7. SideEffect And Idempotency Assessment

The model requires a proposed SideEffect ID and idempotency key, then validates that both match preflight state.

It does not require a persisted proposed `SideEffectRecord`, does not transition SideEffect lifecycle state, and does not append workflow events. That preserves the approved boundary.

Before any real write, a later phase must require SideEffect record linkage, preflight execution, approval linkage where required, idempotency replay behavior, and audit/event projection.

## 8. Privacy And Redaction Assessment

The implementation is redaction-aware:

- target values are bounded and not URL/path shaped;
- comment body and summary reject secret-like values and raw payload markers;
- redaction metadata field names and reasons are bounded and checked;
- provider references and provider error codes are bounded and checked;
- Debug redacts comment body, summary, actor, run IDs, step IDs, target owner/repository, SideEffect ID, idempotency key, preflight details, provider reference, and redaction metadata.

No raw provider payloads, raw command output, parser payloads, spec contents, credentials, authorization headers, private keys, token-like values, or environment values are introduced.

One non-blocking caveat: valid serialization intentionally includes the bounded comment body and target fields for future fixture use. Any future persisted artifact, log, or CLI surface must treat serialized request values as sensitive and should avoid exposing them by default.

## 9. Serde And Compatibility Assessment

Serde support is appropriate for a model boundary:

- valid request serialization/deserialization round trips;
- invalid serialized request input fails closed through constructors;
- custom deserialization preserves validation;
- field names are stable and sensible for future schema discussion.

No workflow schema or CLI contract changes were introduced.

Response serde behavior is partly covered by construction and Debug tests, but a direct response serde round trip test would be useful before fixture-backed adapter work.

## 10. Test Quality Assessment

Tests cover meaningful boundary behavior:

- valid request model construction;
- target URL/path rejection;
- zero pull request rejection;
- mismatched preflight target rejection;
- mismatched SideEffect ID rejection;
- secret-like comment rejection;
- raw payload marker rejection;
- secret-like redaction metadata rejection;
- request Debug non-leakage;
- request serde round trip;
- invalid serialized request failure without leaking secret-like value;
- valid fixture response;
- fixture provider reference rejection;
- provider success provider-reference requirement;
- provider failure bounded error-code requirement;
- response Debug non-leakage.

Existing workspace validation also passed during the implementation phase.

Non-blocking test follow-ups:

- add direct response serde round trip coverage;
- add explicit idempotency mismatch coverage;
- add explicit provider reference secret-like rejection coverage;
- add preflight helper Debug non-leakage coverage;
- add a fixture composition test once preflight execution is wired.

## 11. Documentation Review

Docs accurately state:

- GitHub PR comment is the first future write candidate;
- the model-only request/response boundary is implemented;
- provider write calls are not implemented;
- runtime side-effect execution is not implemented;
- fixture-backed adapter execution is not implemented;
- live sandbox writes are not implemented;
- CLI behavior, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes are not implemented.

The phase report is honest about the boundary and remaining limitations.

## 12. Blockers

No blockers.

## 13. Non-Blocking Follow-Ups

- Plan and implement preflight execution composition before fixture-backed adapter execution.
- Add response serde round trip coverage.
- Add explicit idempotency mismatch coverage.
- Add explicit provider-reference secret-like rejection coverage.
- Treat serialized valid write requests as sensitive in any future artifact, CLI, or log surface.
- Require persisted SideEffect linkage before any live write path.

## 14. Recommended Next Phase

Recommended next phase: preflight composition planning before fixture-backed GitHub PR comment adapter implementation.

The model now carries a matching preflight request, but runtime composition has not yet enforced that preflight execution occurs before a candidate write path. Planning and implementing that composition closes a real runtime-governance gap while still avoiding live provider mutation.

## 15. Validation

Implementation phase validation recorded in `docs/concepts/GITHUB_PR_COMMENT_WRITE_BOUNDARY_REPORT.md`:

- `cargo fmt --all` - passed
- `cargo test -p workflow-core --test provider_write` - passed
- `cargo fmt --all --check` - passed
- `cargo clippy --workspace --all-targets -- -D warnings` - passed
- `cargo test --workspace` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

Review phase validation:

- `npm run check:docs` - passed
- `git diff --check` - passed

## 16. Dogfood Governance Summary

This review phase is governed by the repo-local self-governed build benchmark.

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1783197765533638000-2`
- Approval ID: `approval/run-1783197765533638000-2/review-scope-approved`
- Approval outcome: granted
- Final run status: `Completed`
- Terminal: true
- Events total: 39
- Approvals: 1
- Retries: 0
- Escalations: 0
- Event summary: `ApprovalGranted:1`, `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`, `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`, `SkillInvocationRequested:6`, `SkillInvocationStarted:6`, `SkillInvocationSucceeded:6`, `StepScheduled:6`

Out-of-kernel work disclosed: repository inspection, maintainer review drafting, documentation update, validation commands, and phase-close inspection are performed by the agent outside kernel execution. The kernel coordinates governance only.
