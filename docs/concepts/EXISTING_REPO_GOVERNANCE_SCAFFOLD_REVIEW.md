# Existing Repo Governance Scaffold Review

Fix-forward note: after this scaffold review, the follow-on first-run ledger/report posture slice was implemented as `workflow-os first-run`. It emits a bounded report-ready context and still does not run workflows, create runtime state, write report artifacts, call providers, or auto-register workflows.

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

`workflow-os init-repo-governance` is an appropriately narrow first existing-repository onboarding slice. It creates a valid local Workflow OS project envelope, a first-run approval-gated mock workflow, conservative policy/skill/test scaffolds, and agent orientation files without claiming automatic report generation, command execution, runtime state creation, provider integration, or write-capable behavior.

The phase is ready to proceed to first-run governed ledger/report mode planning.

## 2. Scope Verification

The phase stayed within the approved scaffold-only scope.

Implemented scope:

- CLI command `workflow-os init-repo-governance`.
- Explicit `--output-dir`, `--agent`, `--force`, and `--dry-run` options.
- Minimal valid generated project files.
- Approval-gated first-run mock workflow.
- Conservative default governance policy.
- Agent orientation files using existing managed-block behavior.
- Focused CLI tests and documentation updates.

No accidental implementation was found for:

- first-run ledger/report mode;
- automatic WorkReport generation;
- automatic workflow recommendation generation;
- sidecar external-repo governance;
- arbitrary command execution;
- real local skill handler registration;
- runtime state writes during scaffold creation;
- report artifact writes;
- GitHub/Jira/CI/provider writes;
- schema changes;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy.

## 3. CLI API Assessment

The new command is explicit, local, and reviewable:

```sh
workflow-os init-repo-governance [--output-dir <path>] [--agent generic|codex|claude] [--force] [--dry-run]
```

The implementation accepts explicit user input and does not read hidden global state, invent runtime config, register handlers, execute workflows, or touch the state backend. The output makes the next manual steps clear: validate the generated project and optionally run the approval-gated mock workflow.

`--dry-run` reports planned writes without writing scaffold files or runtime state. `--force` is explicit and limited to replacing scaffold targets.

## 4. Generated Project Assessment

The generated project is a coherent minimal Workflow OS envelope:

- `workflow-os.yml`
- `workflows/first-run-governance.workflow.yml`
- `skills/first-run-report.skill.yml`
- `policies/default-governance.policy.yml`
- `tests/first-run-governance.test.yml`
- `.workflow-os/README.md`
- `AGENTS.md`
- `.workflow-os/agent-harness-prompt.md`

The generated workflow is appropriately low-risk. It uses a local mockable skill, requires human approval, declares audit and observability expectations, and keeps the Governed Work Pattern posture visible without pretending the generated skill is a production handler.

The generated files are useful as a first bridge from "normal repository" to "governed local project." They do not ask users to copy Workflow OS's internal dogfood workflows.

## 5. Safety And File-Boundary Assessment

The scaffold file boundary is conservative.

Plain scaffold targets fail closed when a file already exists unless `--force` is supplied. The error names the path class and stable code but does not echo existing file contents.

`AGENTS.md` and `.workflow-os/agent-harness-prompt.md` reuse the existing managed-block replacement behavior. This preserves surrounding user content when a Workflow OS managed block exists and fails closed on unmanaged content unless `--force` is supplied.

The command writes only scaffold files under the selected output root. It does not append events, create state, execute commands, call providers, or create artifacts.

## 6. Runtime Boundary Assessment

The runtime boundary is clean.

The command does not:

- run the generated workflow;
- approve checkpoints;
- invoke local skill handlers;
- register real handlers;
- execute local commands;
- emit runtime events;
- create workflow state;
- write report artifacts;
- call external providers;
- mutate Git, GitHub, Jira, CI, or any external system.

The generated workflow can be exercised with `--mock-all-local-skills`, but documentation correctly identifies that flag as a local preview convenience rather than proof of a real handler.

## 7. Governed Work Pattern Assessment

The generated materials correctly express the Governed Work Pattern as the default onboarding posture:

- bounded goal and scope;
- context and evidence expectations;
- validation/check posture;
- approval checkpoint;
- side-effect disclosure posture;
- risks, skipped work, and deferred work posture;
- final report closure posture;
- future workflow recommendation posture.

This is the right product position: Workflow OS should be valuable before mature custom workflows exist, but it must not fabricate evidence or imply it executed unsupported work.

## 8. Test Quality Assessment

The focused tests cover the important behavior:

- help text includes `init-repo-governance`;
- scaffold creates expected files;
- generated project validates;
- generated workflow runs to approval with mock skill;
- approval completion succeeds with mock skill;
- dry run writes no project files or state;
- existing scaffold target fails closed without leaking content;
- `--force` replaces existing scaffold target and still validates.

Non-blocking test gaps:

- Add direct `--output-dir` coverage for `init-repo-governance`.
- Add `--agent codex|claude` assertions for generated prompt flavor in the repo-governance command.
- Add direct managed-block preservation coverage for `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` through `init-repo-governance`, even though the underlying helper is already covered by `init-agent-harness`.
- Consider asserting generated `.workflow-os/README.md` boundary language so future edits do not accidentally overclaim.

These gaps do not block acceptance because the core scaffold, validation, fail-closed, and dry-run behaviors are covered.

## 9. Documentation Review

Documentation is honest and aligned with implementation.

Docs state:

- `workflow-os init-repo-governance` is implemented;
- the command creates a minimal local project envelope;
- the generated first-run workflow is approval-gated and mockable;
- first-run Governed Work Pattern reporting remains planned, not implemented;
- automatic runtime report generation is not implemented;
- arbitrary command execution is not implemented;
- real handler registration is not implemented;
- runtime state writes during scaffolding are not implemented;
- report artifacts are not implemented;
- provider writes are not implemented;
- hosted/distributed behavior is not implemented;
- recursive agents, agent swarms, and Level 3/4 autonomy are not implemented.

No dangerous false claim was found.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add `--output-dir` test coverage for `init-repo-governance`.
- Add generated prompt flavor tests for `--agent codex` and `--agent claude`.
- Add managed-block preservation tests through `init-repo-governance`.
- Add README/setup-note boundary assertions to protect against future overclaiming.
- Improve missing-manifest diagnostics so a normal repository points users toward `init-repo-governance`.

## 12. Recommended Next Phase

Proceed to first-run governed ledger/report mode planning.

The scaffold gives existing repositories a valid governance envelope. The next product gap is immediate ledger/report value: after setup, Workflow OS should be able to produce a bounded, evidence-aware first-run WorkReport or report-ready context that records what was known, what was missing, what was skipped, and what workflows/checkpoints should be formalized next. That should be planned before implementation and must not add arbitrary command execution, provider writes, report artifacts by default, schemas, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 13. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
