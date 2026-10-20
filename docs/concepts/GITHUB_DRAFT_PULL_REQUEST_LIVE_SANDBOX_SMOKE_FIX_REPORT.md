# GitHub Draft Pull Request Live Sandbox Smoke Fix Report

Report date: 2026-10-20

## 1. Executive Summary

The test-fixture approval-binding blocker is fixed without changing production
provider-write behavior. The live smoke now binds its durable approval marker
to the exact content it presents. Direct regression coverage proves content
substitution still fails before provider use.

The live proof remains incomplete because the installed GitHub credential is
valid for the main repository but cannot access the dedicated private sandbox.
The transport reached pre-observation and failed closed. No provider mutation
occurred.

## 2. Governed Phase

- workflow: `dg/blocker`
- run: `run-1792533391661217000-2`
- approval: `approval/run-1792533391661217000-2/fix-approved`
- presentation: `presentation/580a439100b2252b`
- approval outcome: granted under delegated maintainer authority

## 3. Change

The integration fixture now provides:

- `approval_decision_for(content)` for exact content-bound proof construction;
- `terminal_run_for(content)` for exact durable run construction; and
- an ordinary substitution regression that requires zero provider calls.

The original `approval_decision()` and `terminal_run()` helpers remain wrappers
over the ordinary fixture, preserving existing test behavior.

## 4. Validation

- `cargo fmt --all --check`: passed.
- focused ordinary draft pull request integration tests: 11 passed.
- exact ignored live smoke: reached provider pre-observation and failed closed.
- credential scope probe: main repository `200`, sandbox repository `404`.
- sandbox open pull requests after both attempts: zero.

## 5. Privacy And Safety

The credential value remained process-local. It was not printed, serialized,
persisted in Workflow OS state, or copied into diagnostics or reports. The
bounded provider error disclosed no URL, repository payload, or access value.

## 6. Remaining Blocker

Provision a separate repository-scoped credential for
`rcs2153/workflow-os-sandbox` with only contents read and pull requests write.
Do not broaden the main-repository credential and do not store provider access
inside Workflow OS Core.

## 7. Recommended Next Phase

**Dedicated sandbox credential provisioning and unchanged live-smoke rerun.**

No code change is currently justified. If the unchanged smoke still fails with
correct sandbox access, treat that result as a transport or provider-contract
blocker and investigate before any additional mutation family.

## 8. Phase Close

The governed blocker-fix run closed `Completed` with 39 events, one approval,
zero retries, and zero escalations. Approval presentation enforcement was
proof-enforced through persisted presentation
`presentation/580a439100b2252b`.

Work performed outside the kernel was limited to the disclosed repository test
fixture edit, focused Rust validation, read-only credential-scope probes,
sandbox state inspection, documentation updates, and git diff inspection. The
kernel coordinated governance and retained the approval/event trail; it did not
execute the edits, tests, provider probes, or git operations.
