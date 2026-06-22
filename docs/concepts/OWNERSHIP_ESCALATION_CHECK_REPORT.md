# Ownership And Escalation Check Report

## 1. Executive Summary

`workflow-os first-run` now includes a deterministic, warning-only ownership and escalation check for loaded workflow and skill definitions.

The check makes scaffold ownership and escalation fields more than decorative metadata by surfacing missing, placeholder, lifecycle, and authority-context warnings during first-run onboarding. It remains disclosure-only: it does not change validation pass/fail behavior, executor behavior, approval behavior, escalation routing, RBAC, schema shape, or runtime state.

## 2. Scope Completed

- Added an internal ownership/escalation check helper inside the CLI first-run context.
- Inspected loaded workflow and skill definitions only.
- Reported missing owner metadata.
- Reported scaffold placeholder owner metadata.
- Reported missing escalation contacts.
- Reported scaffold placeholder escalation contacts.
- Reported experimental/deprecated lifecycle posture.
- Reported workflows or skills with approval/adapter authority context but incomplete configured ownership/escalation posture.
- Emitted bounded text output.
- Emitted bounded preview JSON output.
- Added focused CLI tests for scaffold placeholder warnings, missing metadata warnings, JSON output, redaction, and no runtime state creation.
- Updated CLI docs, roadmap, and scaffold field operationalization plan.

## 3. Scope Explicitly Not Completed

This phase does not implement:

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

## 4. Behavior Added

`workflow-os first-run` now emits warning-only ownership/escalation output such as:

```text
ownership_escalation_check: warnings
ownership_escalation_findings: 7
ownership_missing_owner: 0
ownership_placeholder_owner: 2
escalation_missing_contact: 0
escalation_placeholder_contact: 2
lifecycle_warnings: 2
authority_context_warnings: 1
ownership_escalation_finding: target=workflow#1 code=ownership.placeholder_owner severity=warning
```

Preview JSON includes the same bounded summary and issue list.

Findings use stable issue codes and target ordinals. They do not print raw owner, maintainer, escalation-contact, file, command, provider, or source-content values.

## 5. Validation Boundary Summary

The check reads the already-loaded and validated project bundle. It does not read arbitrary repository source files, execute commands, inspect Git history, call providers, create runtime state, append events, or write artifacts.

The check is intentionally warning/report-only. It does not change project validation semantics and does not treat owner strings as authority.

## 6. Redaction And Privacy Summary

The check emits:

- counts;
- target kind labels (`workflow`, `skill`);
- target ordinals;
- stable issue codes;
- warning severity labels.

The check does not emit:

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

## 7. Test Coverage Summary

Focused tests cover:

- generated scaffold placeholder ownership/escalation warnings;
- bounded text output;
- bounded preview JSON output;
- configured owner/escalation values classified without printing raw values;
- missing owner/escalation metadata warnings;
- no scaffold placeholder owner values leaked;
- no configured owner/escalation values leaked;
- no runtime state created by first-run mode.

Existing first-run invalid project and no-raw-payload tests continue to cover failure and privacy boundaries.

## 8. Commands Run And Results

- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance validate` - passed with expected experimental lifecycle warnings.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-owner-escalation-check-state --mock-all-local-skills run dg/spec-field-operationalization` - paused for approval as expected.
- `target/debug/workflow-os --project-dir dogfood/workflow-os-self-governance --state-dir /tmp/workflow-os-owner-escalation-check-state --mock-all-local-skills approve run-1782096187863599000-2 approval/run-1782096187863599000-2/implementation-scope-approved --actor user/maintainer --reason user-approved-ownership-escalation-check-implementation` - completed the governed run.
- `cargo test -p workflow-cli --test cli first_run` - passed.
- `cargo fmt --all` - applied formatting.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- The check is first-run scoped and CLI-local.
- It reports warnings but does not fail validation.
- It does not resolve owners against an org directory.
- It does not route escalations.
- It uses bounded target ordinals rather than source paths or raw IDs.
- It does not yet integrate with workflow discovery recommendations.
- It does not implement governance strictness profiles.

## 10. Recommended Next Phase

Proceed to **ownership and escalation check review**.

After review, the next implementation should be **spec field coverage check planning or implementation**, depending on reviewer confidence. That phase should inventory rich scaffold/spec fields and report whether each field is enforced, validated, disclosed, advisory, or deferred without adding schema changes, automatic execution, RBAC, provider writes, or hosted behavior.
