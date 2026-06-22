# Ownership And Escalation Check Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The ownership/escalation check is a useful, bounded next step in scaffold field operationalization. It turns important scaffold metadata into visible first-run warnings without changing validation semantics, runtime behavior, approval behavior, RBAC, escalation routing, schema shape, or report artifact behavior.

## 2. Scope Verification

The phase stayed within the approved warning/reporting scope.

Implemented scope:

- `workflow-os first-run` emits a deterministic ownership/escalation check summary.
- Loaded workflow and skill definitions are inspected.
- Missing owner metadata is reported.
- Placeholder owner metadata is reported.
- Missing escalation contact metadata is reported.
- Placeholder escalation contact metadata is reported.
- Experimental/deprecated lifecycle posture is reported.
- Approval/adapter-sensitive definitions without configured ownership/escalation posture receive an authority-context warning.
- Text and preview JSON output are bounded.
- Focused CLI tests and documentation were added.

No accidental implementation was found for:

- hard validation failures for missing or placeholder ownership;
- governance profile selection;
- RBAC, IdP, org-directory lookup, enterprise admin controls, or steward console behavior;
- escalation notifications, paging, email, Slack, or incident routing;
- workflow schema changes;
- automatic workflow generation or registration;
- automatic command execution;
- automatic local check execution;
- provider calls or provider writes;
- report artifact writing;
- runtime event emission;
- runtime state mutation;
- hosted behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 3. Behavior Assessment

The check behaves as intended for this phase.

It reports bounded warning counts and stable issue codes for ownership, escalation, lifecycle, and authority-context posture. It uses target kind plus ordinal, such as `workflow#1` or `skill#1`, instead of raw IDs or file paths. This keeps the output useful during onboarding while avoiding path and identity leakage.

The authority-context warning is correctly narrow. It applies when workflows or skills already carry approval, escalation, adapter, or higher-sensitivity posture but do not have configured ownership and escalation metadata. This is a good bridge between rich YAML fields and practical governance without pretending the kernel can resolve organizational authority yet.

## 4. Runtime Boundary Assessment

The implementation does not:

- run workflows;
- create runtime state;
- append events;
- request approvals;
- execute checks;
- invoke local skill handlers;
- call adapters;
- route escalation;
- write report artifacts;
- change executor semantics.

The check remains part of explicit `first-run` report-ready context output.

## 5. Validation Semantics Assessment

Project validation behavior is unchanged.

The check runs only after the project is loaded and validated successfully by the existing first-run path. Findings are warnings in first-run output, not `Diagnostic` errors and not validation blockers.

That is the right boundary for the current `observe_and_report` posture.

## 6. Privacy And Redaction Assessment

The check is redaction-safe.

It emits:

- counts;
- target kind labels;
- target ordinals;
- stable issue codes;
- warning severity labels.

It does not emit:

- raw owner names;
- raw maintainer IDs;
- raw escalation contacts;
- source paths;
- raw source content;
- command output;
- parser payloads;
- provider payloads;
- environment values;
- credentials;
- tokens;
- private keys.

Tests verify that scaffold placeholder values and configured owner/escalation values are not printed.

## 7. Test Quality Assessment

Tests cover the important current behavior:

- generated scaffold placeholder owner/escalation warnings;
- missing owner/escalation warnings;
- bounded text output;
- bounded JSON output;
- stable issue code output;
- configured owner/escalation values classified without printing raw values;
- raw scaffold owner values not leaked;
- configured owner/escalation values not leaked;
- no runtime state is created.

Non-blocking gaps:

- Add a test for a fully configured stable project where `ownership_escalation_check` is `passed`.
- Add a targeted test for `authority.owner_context_required` on adapter or high-sensitivity skill posture outside the generated scaffold path.
- Add a targeted test for deprecated lifecycle posture.

These are non-blocking because the implemented warning path is simple, the scaffold/missing/configured privacy cases are covered, and the full workspace test suite passes.

## 8. Documentation Review

Documentation accurately states:

- ownership/escalation check output is implemented in `workflow-os first-run`;
- findings are warning-only;
- output is bounded and does not print raw owner/escalation values;
- validation pass/fail behavior is unchanged;
- runtime execution is unchanged;
- runtime state creation is not implemented;
- command/local check execution is not implemented;
- provider calls and writes are not implemented;
- report artifacts are not implemented;
- RBAC/admin controls and escalation routing are not implemented;
- hosted/distributed behavior is not implemented;
- recursive agents, agent swarms, and Level 3/4 autonomy are not enabled.

The roadmap and scaffold field operationalization plan correctly position the next lane as broader spec field coverage.

## 9. Blockers

No blockers.

## 10. Non-Blocking Follow-Ups

- Add a passing/no-findings ownership-escalation check test.
- Add an explicit adapter or high-sensitivity skill authority-context warning test.
- Add an explicit deprecated lifecycle warning test.
- Consider moving the first-run posture/check helpers into a small module if the next spec field coverage slice expands the implementation further.
- Keep future stricter enforcement behind reviewed governance profiles rather than changing default validation behavior.

## 11. Recommended Next Phase

Proceed to spec field coverage check planning.

Reason: first-run now discloses field posture and reports ownership/escalation warnings. The next load-bearing gap is a broader field coverage map that tells users which rich workflow/skill fields are enforced, validated, disclosed, advisory, or deferred. That should be planned before implementation so the coverage taxonomy remains honest and does not overclaim automation.

## 12. Validation

Commands run during implementation and review:

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate` - passed with expected experimental lifecycle warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-owner-escalation-check-state --mock-all-local-skills run dg/spec-field-operationalization` - paused for approval as expected.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-owner-escalation-check-state --mock-all-local-skills approve run-1782096187863599000-2 approval/run-1782096187863599000-2/implementation-scope-approved --actor user/maintainer --reason user-approved-ownership-escalation-check-implementation` - completed the governed run.
- `cargo test -p workflow-cli --test cli first_run` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
