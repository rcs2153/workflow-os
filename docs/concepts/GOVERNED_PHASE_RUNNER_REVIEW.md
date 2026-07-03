# Governed Phase Runner Review

## 1. Executive Verdict

Needs blocker fixes.

The governed phase runner is directionally correct and materially improves Workflow OS dogfooding. It starts the right `dg/*` workflow, preserves explicit approval, produces useful phase-close summaries, and keeps Codex/human work outside the kernel boundary.

One acceptance requirement is not fully met: `phase-start` does not display the approval reason as a first-class field. It prints an approval command with the reason redacted as `<redacted-reason>`, which preserves command-output safety but fails the stated P0 requirement to display the approval reason.

## 2. Scope Verification

The phase stayed within the approved repo-local dogfood helper scope.

No accidental implementation was found for:

- hidden approvals;
- automatic approvals;
- repository edits from inside the runner;
- git operations from inside the runner;
- PR creation from inside the runner;
- arbitrary shell execution beyond wrapping existing local Workflow OS CLI commands;
- local check execution;
- report artifact writing;
- WorkReport rendering;
- workflow schema changes;
- runtime hook broadening;
- write-capable adapters;
- recursive agents;
- agent swarms;
- hosted execution;
- production self-hosting;
- Level 3/4 autonomy.

The helper remains repo-local development tooling, not a public stable product API.

## 3. Runner API Assessment

The implementation adds:

- `npm run dogfood:benchmark -- phase-start --phase <phase>`;
- `npm run dogfood:benchmark -- phase-close <run-id> --phase <phase>`.

The API is narrow, explicit, and testable.

`phase-start`:

- validates the dogfood project;
- starts the mapped `dg/*` workflow;
- uses deterministic mock local skills for dogfood governance checkpoints;
- extracts the real `run_id`;
- extracts the real `approval_id`;
- prints status and next action;
- prints an approval command instead of running it.

`phase-close`:

- reads run status;
- reads inspect output;
- summarizes event counts;
- summarizes approval/retry/escalation counts;
- summarizes event kinds;
- prints required phase-report disclosure fields.

The helper does not require hidden global state beyond the repo-local dogfood project and supplied state directory.

## 4. Phase Mapping Assessment

The phase mappings are appropriate for the current dogfood suite:

| Phase | Workflow |
| --- | --- |
| `planning` | `dg/d` |
| `docs` | `dg/d` |
| `implementation` | `dg/implement` |
| `review` | `dg/review` |
| `blocker` | `dg/blocker` |
| `pr` | `dg/pr` |
| `release` | `dg/release` |
| `runtime-composition` | `dg/runtime-composition` |
| `branch-cleanup` | `dg/branch-cleanup` |
| `workflow-discovery` | `dg/workflow-discovery` |
| `spec-field-operationalization` | `dg/spec-field-operationalization` |

Unsupported phase values fail closed with a stable helper usage error and do not echo secret-like phase values.

## 5. Approval Boundary Assessment

The approval boundary is mostly correct.

Positive findings:

- `phase-start` does not approve automatically.
- `phase-start --dry-run` explicitly reports `approval_outcome: not_requested`.
- A live `phase-start` reports `approval_outcome: pending`.
- The runner prints a command for the human/agent to execute separately.
- Existing approval helper behavior still requires `--reason`.

Blocker:

- The P0 requirement says the runner must display `run_id`, `approval_id`, status, approval reason, and next action.
- The runner displays `run_id`, `approval_id`, status, and next action.
- The runner does not display approval reason as a separate field.
- The approval command redacts the reason as `<redacted-reason>`, so a user cannot see the exact bounded reason that will be used.

Required fix:

- Add a non-secret first-class output line such as `approval_reason: approved-implementation-phase`.
- Keep command-display redaction for arbitrary `--reason` values.
- Add a focused regression test proving `phase-start` displays the bounded phase approval reason without exposing arbitrary caller-supplied secret-like values.

## 6. Phase-Close Assessment

`phase-close` meets the intended P0 closure posture.

It prints:

- run ID;
- workflow ID;
- status;
- terminal status;
- total events;
- approvals;
- retries;
- escalations;
- event-kind counts;
- required phase-report fields;
- out-of-kernel work disclosure;
- missing coverage policy.

The command uses experimental CLI JSON internally and prints a bounded summary rather than raw JSON payloads. That is appropriate for repo-local dogfood tooling and is documented as a known limitation.

## 7. Privacy And Redaction Assessment

The helper maintains a conservative output boundary.

Positive findings:

- Displayed commands redact values following `--reason`.
- Secret-like actor/reason/phase values are rejected.
- Unsupported arbitrary command text is not echoed.
- Errors use stable helper codes.
- `phase-close` does not print raw inspect JSON after the post-implementation tightening.

Residual risk:

- The helper uses command output parsing for `run_id` and `approval_id`; if future CLI output changes, the helper can fail to detect approval IDs. Current behavior fails visibly with `approval_id: not_available` rather than fabricating values.

## 8. Test Quality Assessment

Tests cover:

- `phase-start` command mapping for implementation phases;
- dry-run approval boundary;
- unsupported/secret-like phase non-leakage;
- `phase-close` dry-run command posture;
- command-guide boundary wording;
- approval reason redaction in displayed commands;
- secret-like metadata rejection;
- missing binary fail-closed behavior.

Missing/blocking test:

- No test proves `phase-start` displays the bounded approval reason required by the P0 acceptance criteria.

Non-blocking test gap:

- There is no focused test for every phase mapping. Current mapping coverage exercises `implementation` and `review`; broader table-driven coverage would reduce future drift.

## 9. Documentation Review

Docs now state:

- the governed phase runner exists;
- material Workflow OS phases should use `phase-start` unless explicitly exempted;
- `phase-close` should summarize event trail and phase-report disclosures;
- the runner is repo-local development tooling;
- approvals remain explicit;
- local checks, git, PR actions, reports, artifacts, schemas, writes, hosted behavior, recursive agents, and Level 3/4 autonomy remain unsupported.

The docs keep the dogfood/community boundary clear.

## 10. Dogfood Review Context

This review phase was governed by:

- Workflow ID: `dg/review`
- Run ID: `run-1783052150861697000-2`
- Approval ID: `approval/run-1783052150861697000-2/review-scope-approved`
- Approval outcome: granted

Repository review and documentation edits were performed outside the kernel. The kernel governed the review boundary and approval trail.

## 11. Blockers

1. `phase-start` must display the bounded approval reason as a first-class field.

Required action:

- Add `approval_reason: <bounded phase approval reason>` to live and dry-run `phase-start` output.
- Preserve existing command reason redaction.
- Add regression coverage.

## 12. Non-Blocking Follow-Ups

- Add table-driven phase mapping tests covering every supported phase.
- Consider changing exported `buildWorkflowCommand(...)` behavior for `phase-close`, or document that it returns only the legacy inspect command shape while runtime `phase-close` uses both status and inspect.
- Consider a future `--exempt-reason` mode for explicitly exempted phases so exemption disclosure is standardized.

## 13. Recommended Next Phase

Governed phase runner blocker fix.

This should be a narrow fix only: display the bounded approval reason and add tests. It should not add automatic approvals, new workflow mappings, CLI product surface, local check execution, git operations, PR automation, report artifacts, schemas, writes, hosted behavior, recursive agents, or Level 3/4 autonomy.

## 14. Validation

Validation commands run during the implementation phase:

- `npm run dogfood:benchmark -- validate --no-build` - passed.
- `npm run test:dogfood-helper` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783050656112397000-2 --phase implementation --state-dir /private/tmp/workflow-os-governed-phase-runner-state --no-build` - passed.
- `npm run check` - passed.
- `cargo fmt --all --check` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.

Review-phase validation:

- `npm run dogfood:benchmark -- phase-start --phase review --state-dir /private/tmp/workflow-os-governed-phase-runner-review --no-build` - passed and paused for approval.
- `workflow-os approve ... --reason approved-governed-phase-runner-review` - completed the review dogfood run.

The review document itself was added after those checks and should be included in the next docs validation run.
