# Cross-Harness Fixture Continuation Bridge Implementation Report

## Outcome

Implemented the first fixture-only public continuation bridge for the existing
GitHub pull-request comment provider family.

The bridge lets a replacement executor reconstruct one exact continuation from
durable Workflow OS state and validate it without transcript authority. It is a
validation boundary only. It does not call GitHub, read credentials, transition
the `SideEffect`, append workflow events, or write a report artifact.

## Public Surface

```text
workflow-os --json provider github-pr-comment continue-sandbox \
  --request <bounded-request.json>
```

The bounded request contains:

- the exact fixture-mode `GitHubPullRequestCommentWriteRequest`;
- the matching fixture definition;
- the replacement actor; and
- an optional delegating actor field.

The fixture-first slice accepts only exact same-actor recovery. A non-empty
delegating actor fails closed because the repository does not yet have a
persisted GitHub PR-comment delegation contract that this path can verify.
Different execution harnesses do not require different authority identities.

## Enforced Gates

Core validation now requires:

1. caller-supplied run equality with the rehydrated durable run;
2. a terminal run whose workflow, version, schema, spec hash, and run ID match
   the attempted `SideEffect`;
3. an immutable run-bundle binding on the run and an exact matching bundle
   loaded from the create-only bundle store;
4. exact request matching for workflow identity, step, `SideEffect`, target,
   and idempotency key;
5. an attempted GitHub-write `SideEffect`;
6. exact actor equality across the reconstructed request, durable side effect,
   and replacement identity;
7. proof-enforced approval presentation for a write-adjacent action;
8. store-backed approval-to-`SideEffect` linkage; and
9. the existing fixture preflight and fixture-match validation.

Unknown delegation, missing bundles, stale or absent presentation proof,
changed identity, changed target or idempotency, missing side effects, and
unsupported fixture mode fail closed with stable non-payload error codes.

## Output Posture

Human and JSON output expose only bounded posture:

- validation status;
- fixture mode and outcome;
- exact-actor posture;
- approval-linkage counts; and
- explicit false values for provider call, side-effect mutation, workflow
  event append, and report-artifact write.

The output does not echo comment contents, approval identifiers, run IDs,
target values, idempotency keys, fixture references, or credentials.

## Tests

Focused Core tests prove:

- exact bundle-backed, proof-backed fixture continuation succeeds;
- success does not change events or the stored attempted side effect;
- replacement actor substitution fails closed; and
- unsupported delegated actor substitution fails closed.

CLI tests prove the command is discoverable, requires an explicit request path,
and does not leak a missing caller-supplied path.

Validation completed:

- `cargo fmt --all -- --check`;
- `cargo clippy -p workflow-core -p workflow-cli --all-targets -- -D warnings`;
- `cargo test -p workflow-core --test local_executor`: 354 passed, one explicit
  credential-backed live test ignored;
- `cargo test -p workflow-cli --test cli`: 182 passed;
- `npm run check:docs`; and
- `git diff --check`.

One existing executable-identity test failed while Cargo builds were running
concurrently. It passed immediately in isolation and the complete Core suite
then passed serially. The accepted result is the uncontended serial run.

## Explicit Non-Goals

This phase does not implement:

- live GitHub transport or auth ingestion;
- provider mutation or provider lookup;
- a generic provider-write continuation surface;
- delegated actor grants;
- execution-window opening or consumption;
- automatic report or event persistence;
- Codex or LangGraph adapters;
- transcript, conversation, or private harness-state transfer;
- nested harnesses, agent teams, hosted orchestration, or a production
  interoperability claim.

The later live slice must separately bind and consume current execution-window,
delegation, evidence/check, target, provider-observation, and reconciliation
posture before external mutation.

## Governed Phase

- workflow: `dg/implement`
- run: `run-1794024201149889000-2`
- approval: `approval/run-1794024201149889000-2/implementation-approved`
- approval presentation: `presentation/b0921398479e3bfb`
- approval outcome: granted under delegated maintainer authority
- phase close: `Completed`, 39 events, one approval, zero retries, zero
  escalations, approval-presentation proof enforced
- scope: fixture-first GitHub PR-comment continuation bridge only
- out-of-kernel execution: repository inspection, code edits, tests, and
  documentation were performed by Codex under the governed phase

## Next Phase

Focused maintainer/security review of the fixture continuation bridge. The
independent Codex/LangGraph fixture corpus must not begin until that review is
accepted.
