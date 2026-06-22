# First-Run Governance Field Posture Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The first-run governance field posture slice is appropriately scoped, redaction-safe, and useful for the P0 onboarding gap. It makes scaffolded governance fields visible without turning advisory YAML into hidden enforcement or runtime behavior.

## 2. Scope Verification

The phase stayed within the approved first-run disclosure scope.

Implemented scope:

- `workflow-os first-run` emits bounded governance field posture output.
- Ownership and escalation are classified as `configured`, `placeholder`, or `missing`.
- Governance profile is disclosed as `observe_and_report`.
- Profile posture is disclosed as `disclosed_not_enforced`.
- Approval, policy, evidence, check, side-effect, audit/observability, and deferred-field posture are disclosed.
- Text and preview JSON output remain bounded.
- Focused CLI tests and documentation were added.

No accidental implementation was found for:

- governance profile selection;
- executor behavior changes;
- approval automation or approval bypass;
- RBAC, IdP, enterprise admin controls, or stewardship enforcement;
- escalation notifications, paging, or directory lookup;
- workflow schema changes;
- automatic command execution;
- automatic local check execution;
- workflow generation, registration, or promotion;
- provider calls or provider writes;
- report artifact writing;
- hosted behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 3. User Experience Assessment

The first-run output now gives users a useful immediate map of the governance envelope:

- what ownership and escalation metadata exists;
- whether scaffold defaults are still placeholders;
- whether approvals and policy gates are declared;
- whether evidence, checks, side effects, and audit/observability are available, skipped, unsupported, or deferred;
- which fields are advisory/deferred rather than enforced.

This is the right product move for existing-repo onboarding: the kernel starts carrying opinions and documenting gaps immediately, without requiring a user to hand-author mature workflows first.

## 4. Field Posture Assessment

The posture taxonomy is narrow and understandable.

The ownership and escalation classifier is conservative: mixed configured and placeholder values are still classified as `placeholder`, which avoids overclaiming readiness. Missing metadata is classified separately.

The profile posture is honest. `observe_and_report` is disclosed but not enforced as a selectable policy profile. That matches the current product boundary for single local users who want evidence and reporting without mandatory human approval.

Policy, evidence, check, side-effect, and audit/observability fields are framed as current posture, not as proof of automation. This preserves the important distinction between declared governance intent and runtime-enforced behavior.

## 5. Runtime Boundary Assessment

The implementation does not:

- run workflows;
- create runtime state;
- append events;
- request approvals;
- execute checks;
- invoke local skill handlers;
- call adapters;
- write report artifacts;
- change executor semantics.

`first-run` remains an explicit onboarding/report-ready-context command, not automatic runtime generation. That boundary is important and was preserved.

## 6. Privacy And Redaction Assessment

The phase is redaction-safe.

The output prints posture classifications rather than raw governance values. Tests cover configured owner/escalation values and assert the raw values are not emitted.

No leakage was found for:

- owner names;
- maintainer IDs;
- escalation contacts;
- raw repository contents;
- raw command output;
- provider payloads;
- parser payloads;
- environment values;
- credentials;
- tokens;
- private keys.

The preview JSON also uses bounded labels, not raw owner or escalation values.

## 7. Test Quality Assessment

Tests cover the important first slice:

- first-run text output includes governance profile and field posture;
- scaffold ownership and escalation are classified as placeholders;
- configured owner/escalation metadata is classified without printing raw values;
- JSON output includes bounded governance field posture;
- JSON output does not print scaffold placeholder owner values;
- first-run creates no runtime state;
- raw repository payload markers are not copied.

The tests are focused and appropriate for this phase.

Shallow or missing coverage:

- There is no dedicated test for `missing` ownership/escalation posture.
- There is no dedicated test for `declared_unsupported` side-effect posture when adapter requirements are present.
- There is no dedicated test for `missing` audit/observability posture.

These are non-blocking because the implemented branches are simple and the primary scaffold/configured paths are covered, but they should be added as this posture surface becomes more relied upon.

## 8. Documentation Review

Documentation accurately states:

- first-run governance field posture output is implemented;
- first-run emits a report-ready context rather than a terminal `WorkReport`;
- automatic workflow generation is not implemented;
- automatic command/local check execution is not implemented;
- runtime state creation is not implemented;
- provider calls and writes are not implemented;
- report artifacts are not implemented;
- hosted/distributed behavior is not implemented;
- recursive agents, agent swarms, and Level 3/4 autonomy are not enabled.

The roadmap and implementation plans correctly position this as the first implementation slice of scaffold field operationalization, with ownership/escalation checks and broader field coverage still future work.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- Add tests for missing ownership and escalation posture.
- Add tests for adapter requirement side-effect posture.
- Add tests for missing audit/observability posture.
- Consider extracting posture classification into a small helper module if future checks need to share the logic.
- In the next implementation phase, keep owner/escalation findings warning/report-only unless a stricter profile is separately implemented and reviewed.

## 11. Recommended Next Phase

Proceed to ownership and escalation check implementation.

Reason: first-run now discloses whether ownership and escalation metadata is missing, placeholder, or configured. The next load-bearing step is a deterministic local check that can inspect workflows and skills for missing or placeholder ownership/escalation posture and report those findings without schema changes, RBAC, paging, enterprise admin controls, or runtime mutation.

## 12. Validation

Commands run during implementation and review:

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate` - passed with expected experimental lifecycle warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-spec-field-operationalization-state --mock-all-local-skills run dg/spec-field-operationalization` - paused for approval as expected.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-spec-field-operationalization-state --mock-all-local-skills approve run-1782088937354580000-2 approval/run-1782088937354580000-2/implementation-scope-approved --actor user/maintainer --reason user-approved-first-run-field-posture-implementation` - completed the governed run.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
