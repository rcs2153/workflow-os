# Onboarding Retest P0 Fixes Review

## 1. Executive Verdict

Phase accepted; proceed to first-run spec field coverage check implementation.

The implementation fixes the corrected existing-repo onboarding blockers without widening runtime authority. The changes are small, user-facing, tested, and aligned with the current local preview boundary.

## 2. Scope Verification

The phase stayed within approved onboarding P0 fix scope.

Implemented:

- scaffold next-step output now includes `workflow-os first-run`;
- generated downstream `AGENTS.md` no longer references Workflow OS internal `docs/ENGINEERING_STANDARD.md`;
- missing-manifest `workflow-os validate` output suggests `workflow-os init-repo-governance`;
- optional doctor schema posture is reported as `schemas: unavailable_optional`;
- focused CLI tests and docs updates;
- end-of-phase report.

No accidental scope expansion found:

- no automatic workflow execution;
- no automatic `first-run` execution;
- no command execution;
- no local check execution;
- no workflow generation or registration;
- no workflow schema changes;
- no runtime config;
- no report artifacts;
- no persistence changes;
- no provider calls;
- no write-capable adapters;
- no hosted/distributed runtime;
- no recursive agents or agent swarms;
- no Level 3/4 autonomy posture change.

## 3. Onboarding Sequence Assessment

The corrected scaffold sequence is appropriate:

```text
workflow-os validate
workflow-os first-run
workflow-os --mock-all-local-skills run local/first-run-governance
```

This matches the intended product experience:

- validate the scaffolded local project;
- inspect the first-run ledger/report posture before running anything;
- run the explicit mock workflow only after the user chooses to do so.

The scaffold command still does not create runtime state or run workflows automatically. That is the right boundary.

## 4. Generated AGENTS.md Assessment

The generated downstream `AGENTS.md` wording is now portable.

It asks agents to read:

- the target repository's own engineering standard or contribution guide if one exists;
- `.workflow-os/README.md`;
- `.workflow-os/agent-harness-prompt.md`.

It no longer assumes downstream repositories contain Workflow OS internal docs. This directly addresses the retest failure mode where an agent could confuse user onboarding with Workflow OS self-governance.

The Workflow OS root `AGENTS.md` remains unchanged and can still require `docs/ENGINEERING_STANDARD.md` for work in this repository.

## 5. Missing-Manifest Guidance Assessment

`workflow-os validate` still fails when `workflow-os.yml` is missing. That is correct.

The new human-readable next step:

```text
next_step: workflow-os init-repo-governance
```

makes the error actionable without silently scaffolding files, converting validation failure to success, or inspecting raw repository source contents.

The guidance is text-only and bounded.

## 6. Doctor Schema Posture Assessment

Changing missing optional schema output from `schemas: failed` to `schemas: unavailable_optional` is appropriate for the local preview.

The implementation preserves important failure behavior:

- missing or invalid project manifests still fail `doctor`;
- local backend health still contributes to `doctor` failure;
- schema availability is not hidden, only classified less alarmingly.

No schema downloads, schema generation, or network behavior were introduced.

## 7. Runtime And Safety Assessment

Runtime behavior is unchanged.

The implementation does not:

- mutate `WorkflowRun`;
- create runtime state;
- append workflow events;
- emit audit events;
- create report artifacts;
- register real skill handlers;
- call adapters;
- change validation semantics beyond human-readable guidance.

The generated first-run workflow remains explicit and mockable. It still requires the user to run it and then approve the checkpoint.

## 8. Privacy And Redaction Assessment

The changes are redaction-safe.

The new outputs expose only static command guidance and posture labels:

- `workflow-os init-repo-governance`;
- `workflow-os first-run`;
- `schemas: unavailable_optional`;
- portable generated instruction text.

They do not print raw source contents, owner values, escalation contacts, command output, provider payloads, environment values, credentials, tokens, private keys, or secret-like values.

## 9. Test Quality Assessment

The focused tests cover the core retest fixes:

- missing-manifest validation suggests `init-repo-governance`;
- generated `AGENTS.md` includes portable downstream wording;
- generated `AGENTS.md` excludes `docs/ENGINEERING_STANDARD.md`;
- `init-repo-governance` output includes `workflow-os first-run`;
- `doctor` reports `schemas: unavailable_optional` and not `schemas: failed`.

The full CLI test target and full workspace suite passed during implementation. The clean temporary onboarding smoke also passed and showed the mock run pausing for approval as expected.

Non-blocking test follow-up:

- Add a small JSON-specific assertion if future preview JSON should expose a separate machine-readable onboarding hint. Current scope intentionally changed only human-readable output.

## 10. Documentation Review

Docs accurately state:

- first-run is the recommended step after scaffold validation;
- generated downstream agent instructions are portable;
- missing-manifest validation output points to `init-repo-governance`;
- doctor schema availability is optional in the local preview;
- no automatic workflow execution, command execution, local check execution, report artifacts, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy were introduced.

The plan and report are linked from the onboarding plan and roadmap.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Consider adding a `first-run` JSON onboarding hint only if preview JSON output needs parity with human-readable output.
- Consider adding targeted wording in `.workflow-os/agent-harness-prompt.md` that explicitly says repo-local standards are optional when absent.
- Keep dogfood commands visually separated from user onboarding commands in future guide cleanup.

## 13. Recommended Next Phase

Proceed to first-run spec field coverage check implementation.

Reason: the immediate retest blockers are fixed. The next load-bearing onboarding improvement is to make rich scaffold/spec fields visible as enforced, validated, disclosed, advisory, or deferred in `workflow-os first-run`, as planned in [Spec Field Coverage Check Plan](../implementation-plans/spec-field-coverage-check-plan.md).

## 14. Validation

Implementation validation reviewed:

- `cargo fmt --all` - passed.
- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-cli --test cli` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- clean temporary existing-repo onboarding smoke - passed.

Review validation:

- `npm run check:docs` - passed.
- `git diff --check` - passed.
