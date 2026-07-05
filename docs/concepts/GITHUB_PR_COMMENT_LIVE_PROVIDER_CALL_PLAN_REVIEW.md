# GitHub PR Comment Live Provider Call Plan Review

## 1. Executive Verdict

Plan accepted; proceed to provider-call trait/input model implementation.

The plan defines a conservative live provider-call boundary without authorizing provider writes in the planning phase. It correctly treats the next implementation as a staged boundary, not a jump straight to live GitHub mutation.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization found for:

- provider writes in the planning phase;
- automatic provider writes;
- default executor write behavior;
- runtime side-effect execution;
- automatic workflow event append;
- automatic report artifact writing;
- CLI mutation behavior;
- workflow schema fields;
- examples;
- hosted/distributed runtime;
- auth material loading in this phase;
- production credential management;
- RBAC/IdP/enterprise stewardship;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. Provider-Call Boundary Assessment

The plan draws the correct boundary: a future live provider-call helper may call GitHub only after the side-effect has already passed explicit pre-call gates.

The pre-call gate list is appropriately strict:

- existing stored `SideEffectRecord`;
- attempted lifecycle state;
- GitHub write capability;
- GitHub pull request comment target;
- policy references;
- approval linkage when required;
- idempotency binding;
- explicit live mode;
- explicit auth input;
- explicit provider-call enablement.

This preserves the architecture principle that a token is not authority.

## 4. Auth Posture Assessment

The auth posture is conservative and appropriate for the next step.

Accepted:

- auth must be caller-supplied explicitly;
- no environment, keychain, GitHub CLI, git remote, config file, or hidden global credential loading;
- auth values must not be `Debug` formatted, serialized, stored, logged, emitted in errors, or copied into reports/events/artifacts;
- production auth loading remains future work.

This is the right distinction between a first local provider-call boundary and future production credential management.

## 5. Provider Client Boundary Assessment

The plan correctly recommends an injected provider-call trait before a live network implementation.

This is important because it lets the next code phase prove:

- request classification;
- auth boundary;
- success/failure classification;
- lifecycle transition;
- non-leakage;
- no workflow mutation;
- no event append;
- no report artifact write;

without requiring real GitHub credentials or network calls.

## 6. Idempotency Assessment

The plan correctly distinguishes local idempotency binding from provider-native idempotency.

Accepted:

- reject missing idempotency binding;
- no automatic provider retries;
- no silent duplicate comment creation;
- no provider guessing for prior-completed cases;
- provider-native idempotency remains an open question.

Non-blocking follow-up: the implementation prompt should be very explicit about duplicate-call handling when a stored record already has an outcome.

## 7. Success And Failure Classification Assessment

The success policy is appropriately strict:

- provider success requires a stable bounded provider comment reference;
- outcome kind should be `Outcome`;
- provider references must be distinct from `fixture/`, `dry-run/`, and `local/` no-provider references;
- raw provider response payloads must not be copied.

The failure policy is also appropriate:

- failure must classify into stable reason codes;
- raw provider errors, headers, stack traces, and auth values must not be copied;
- unclassified errors fail closed without payload leakage.

## 8. Event, Report, And Runtime Boundary Assessment

The plan preserves all required boundaries.

The future helper may return reference-only completed/failed lifecycle transition payloads, but must not:

- append workflow events;
- mutate `WorkflowRun`;
- emit audit or observability events;
- write report artifacts;
- infer workflow pass/fail semantics.

This keeps provider-call execution from collapsing into automatic runtime behavior.

## 9. Privacy And Redaction Assessment

The plan clearly forbids:

- raw provider payloads;
- raw GitHub responses;
- raw HTTP headers;
- raw command output;
- raw CI logs;
- raw file contents;
- raw spec contents;
- environment values;
- credentials;
- authorization headers;
- private keys;
- token-like values.

It also requires `Debug`, serialization, errors, audit candidates, and report candidates to remain redaction-safe.

## 10. Test Plan Assessment

The planned tests cover the right risks:

- missing attempted record;
- wrong lifecycle;
- invalid approval linkage;
- missing auth input;
- live call not explicitly enabled;
- injected provider success path;
- injected provider failure path;
- provider/no-provider reference separation;
- no raw payload copying;
- auth non-leakage;
- no event append;
- no workflow mutation;
- no report artifact write;
- no automatic retries;
- existing no-provider regressions.

Non-blocking follow-up: include a test proving the injected provider client is not invoked when any pre-call gate fails.

## 11. Documentation Review

The plan is honest that it does not implement live writes.

It keeps:

- provider writes future;
- auth loading future;
- runtime execution future;
- CLI future;
- schemas/examples future;
- hosted behavior future;
- reasoning lineage future;
- release posture unchanged.

## 12. Governed Dogfood Summary

- Workflow: `dg/review`
- Run ID: `run-1783278208279092000-2`
- Approval ID: `approval/run-1783278208279092000-2/review-scope-approved`
- Approval actor: `user/delegated-maintainer`
- Approval outcome: granted
- Approval reason: `delegated-maintainer-approved-live-provider-call-plan-review`
- Final status: `Completed`
- Event summary: 39 events; 1 approval; 0 retries; 0 escalations.
- Out-of-kernel work: review document edits and documentation validation were performed by Codex outside the kernel and disclosed here.

The review was run under the local Workflow OS dogfood governance loop.

## 13. Validation

- `npm run check:docs` - passed.

## 14. Planning Blockers

None.

## 15. Non-Blocking Follow-Ups

- Specify duplicate-call handling for already-outcome-bearing records in the implementation prompt.
- Require an explicit test that the provider client is not invoked when pre-call gates fail.
- Decide the exact live provider outcome reference prefix during implementation.

## 16. Recommended Next Phase

Recommended next phase: provider-call trait/input model implementation.

The first implementation should add a narrow injected provider-call trait and validated live-call input model without performing real network calls. If that implementation remains small, it may also add the injected-client orchestration helper that classifies mocked provider success/failure into completed/failed lifecycle transitions. It must not load auth from the environment, call GitHub directly, append workflow events, write report artifacts, add CLI behavior, add schemas/examples, or change release posture.
