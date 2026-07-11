# GitHub PR Comment Sandbox Target Proof Helper Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The sandbox target proof helper is an appropriately narrow and conservative
write-adjacent model slice. It gives the future live sandbox validation path a
bounded way to classify a GitHub PR comment target before provider transport,
while preserving the current no-write product boundary.

Recommended next phase: live sandbox validation helper planning or
implementation, still explicit, injected, non-default, and non-CLI.

Fix-forward note: the first explicit injected live sandbox validation helper is
implemented in [GitHub PR Comment Live Sandbox Validation Helper Report](GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HELPER_REPORT.md).

## 2. Scope Verification

The phase stayed within the approved helper/model scope.

It added:

- `ProviderWriteSandboxTargetClassification`;
- `ProviderWriteSandboxTargetProofDefinition`;
- `ProviderWriteSandboxTargetProof`;
- bounded validation and posture derivation;
- redaction-safe Debug behavior;
- serde validation;
- focused provider-write tests;
- roadmap, plan, and phase-report documentation.

It did not add:

- provider writes;
- live sandbox mutation;
- production writes;
- hidden auth loading;
- automatic executor writes;
- CLI mutation commands;
- workflow schema fields;
- example updates;
- hosted or distributed runtime behavior;
- broad write-capable adapters;
- automatic retries, repair, or recovery mutation;
- report artifact writes;
- reasoning lineage;
- release posture changes.

## 3. Model Assessment

The model is minimal and domain-specific to the first provider-write candidate:
GitHub pull request comments.

`ProviderWriteSandboxTargetProof` captures the fields needed by the accepted
live sandbox validation plan:

- GitHub PR comment target;
- fail-closed sandbox classification;
- expected provider-write capability;
- non-production confirmation;
- bounded non-production statement;
- actor;
- correlation ID;
- idempotency key;
- sensitivity;
- redaction metadata.

The helper also exposes `target_posture()` and `adapter_target()` so future
validation code can feed the existing sandbox readiness helper without
inventing a parallel target model.

This is the right size for the phase. It does not try to validate repository
visibility, repository ownership, branch protection, or maintainer identity
through a provider lookup. Those checks would require a separately reviewed
lookup or live validation boundary.

## 4. Validation Assessment

Validation is deterministic and fail-closed.

The helper validates:

- the GitHub PR comment target;
- the capability is exactly `GitHubPullRequestComment`;
- the non-production statement is present;
- the non-production statement is bounded;
- the non-production statement is not secret-like;
- redaction metadata is valid.

The posture mapping is appropriately conservative:

- confirmed `Disposable`, `Test`, `Preview`, and `MaintainerSandbox`
  classifications derive `ExplicitSandbox`;
- unconfirmed targets derive `ProductionLike`, even if the classification is a
  sandbox-like value;
- `ProductionLike` derives `ProductionLike`;
- `Unknown` derives `Unknown`.

That means callers cannot get an explicit sandbox posture from classification
alone. They must also provide the explicit non-production confirmation.

## 5. Authority Boundary Assessment

The helper does not authorize writes.

The model exposes explicit non-authority flags:

- `provider_call_allowed()` returns `false`;
- `workflow_event_append_allowed()` returns `false`;
- `report_artifact_write_allowed()` returns `false`.

Those flags are not the only safety mechanism, but they make the boundary
visible and testable. The helper does not load auth, call a provider, append
workflow events, mutate side-effect records, write report artifacts, or emit CLI
output.

The helper also pins the target proof to
`AdapterWriteCapability::GitHubPullRequestComment`, rejecting other capabilities
such as GitHub merge. This prevents the proof vocabulary from being reused as a
general write-authority token.

## 6. Privacy And Redaction Assessment

The privacy posture is acceptable for this phase.

Debug output redacts:

- non-production statement;
- actor;
- correlation ID;
- idempotency key;
- redaction metadata;
- target owner/repository through the existing target Debug implementation.

Validation errors use stable codes and do not include raw target strings,
statements, token-like values, provider payload markers, command output, or
secret-like values.

Serialization can carry the bounded target proof, including the target and
bounded non-production statement. That is acceptable for this model because the
proof itself is the bounded record being exchanged. Secret-like statement and
redaction metadata values fail closed during construction/deserialization.

The model does not store provider tokens, authorization headers, provider
payloads, raw PR bodies, raw issue comments, repository file contents, command
output, browser/session state, or CI logs.

## 7. Test Quality Assessment

The focused tests cover the important helper-level behavior:

- explicit sandbox proof derives `ExplicitSandbox`;
- proof feeds the existing sandbox readiness input;
- proof does not authorize provider calls;
- unconfirmed proof derives `ProductionLike`;
- unknown classification derives `Unknown`;
- secret-like statements are rejected without leakage;
- unsupported capability is rejected;
- Debug output redacts sensitive values;
- serde round-trip works for valid proof;
- invalid serialized proof fails closed without leaking target strings.

Existing provider-write tests also continue to cover the broader readiness,
auth-source, provider-call, lookup, reconciliation, artifact, side-effect, and
redaction boundaries.

Missing or shallow coverage:

- Empty and too-long non-production statements are not directly tested.
- `ProductionLike` classification with confirmation is not directly tested.
- Invalid redaction metadata on this exact proof type is not directly tested.

These are non-blocking because the validation code is simple, existing helper
patterns cover equivalent redaction behavior elsewhere, and full
`provider_write` plus workspace tests passed. They are good follow-ups for the
next helper expansion or a small hardening pass.

The prior plan-review follow-up to prove that target proof failure prevents an
injected provider call is not applicable to this model-only helper: this helper
has no provider boundary to invoke. That test should be added when the explicit
live sandbox validation helper composes target proof with an injected provider
boundary.

## 8. Documentation Review

Documentation is honest about current capability.

The phase report and roadmap state that the sandbox target proof helper is
implemented, while the following remain unimplemented:

- provider writes;
- live sandbox mutation;
- production writes;
- hidden auth loading;
- automatic executor writes;
- CLI mutation commands;
- workflow schema fields;
- examples;
- hosted behavior;
- broad write-capable adapters;
- report artifact writes;
- reasoning lineage;
- release posture changes.

The live sandbox validation plan now links to the implemented helper report.

## 9. Relationship To Live Sandbox Validation

The helper is a useful precondition for live sandbox validation, but it is not
itself live validation.

A future live sandbox validation helper still needs to compose:

- sandbox target proof;
- explicit caller-supplied auth posture;
- policy allowance;
- SideEffect proposal and attempted lifecycle state;
- approval-side-effect linkage when required;
- approval-presentation proof when required;
- idempotency and correlation binding;
- injected provider boundary;
- bounded success/failure classification.

That future helper must remain non-default and non-CLI until separately
reviewed.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add direct tests for empty and too-long non-production statements.
- Add direct tests for confirmed `ProductionLike` classification.
- Add direct tests for invalid redaction metadata on
  `ProviderWriteSandboxTargetProof`.
- In the next live sandbox validation helper, prove target-proof failure stops
  before the injected provider is called.
- Keep ignored live integration tests separate from non-network helper tests.

## 12. Recommended Next Phase

Recommended next phase: live sandbox validation helper planning or
implementation, explicit and non-default.

Why: the accepted plan now has a reviewed target-proof precondition. The next
useful runtime-composition step is to validate target proof together with
caller-supplied auth posture, policy/approval/SideEffect authority signals, and
an injected provider boundary before any live transport is possible.

That next phase must still not add production writes, CLI mutation commands,
hidden auth loading, automatic executor writes, schema fields, examples, hosted
behavior, broad adapters, report artifact writes, reasoning lineage, or release
posture changes.

## 13. Validation

Validation for this review:

```sh
npm run check:docs
git diff --check
```

Result: passed.

## 14. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1783761664076460000-2`
- approval ID: `approval/run-1783761664076460000-2/review-scope-approved`
- presentation ID: `presentation/9304aae51d3c03db`
- approval outcome: granted by delegated maintainer
- phase-close status: completed
- event summary:
  - total events: 39
  - approvals: 1
  - retries: 0
  - escalations: 0
  - approval-presentation enforcement: proof-enforced
  - approval-presentation event marker: present
  - approval-presentation content hash:
    `9304aae51d3c03dbb0bbc5e6cbb07909ba5147fbe5d845083918c2b6b19fb7a4`

Out-of-kernel work disclosed:

- implementation and test review;
- review document creation;
- docs and whitespace validation;
- no provider calls;
- no hidden auth loading;
- no CLI mutation behavior;
- no runtime writes performed by the kernel.
