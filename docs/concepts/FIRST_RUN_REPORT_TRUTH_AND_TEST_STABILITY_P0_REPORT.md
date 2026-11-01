# First-Run Report Truth And Test Stability P0 Report

## Status

Implementation and focused maintainer/security review complete. Terminal-green
GitHub acceptance remains required for the exact committed head.

## Problem

Independent fresh-pull evaluation reproduced five acceptance defects on the
then-current `main` branch:

1. local-host tests used a clock-only temporary fixture suffix and could collide
   during parallel execution;
2. a first-run WorkReport with two approval requests and grants described one
   approval;
3. generated `AGENTS.md` guidance declared report artifact generation broadly
   unsupported even though an explicit bounded first-run artifact path exists;
4. the WorkReport suggested `workflow-os run <run-id>` instead of the supported
   `workflow-os inspect <run-id>` command; and
5. aggregate-governance and authored-step approvals presented materially the
   same action and scope.

These are one P0 truth boundary. Tests, reports, generated instructions,
approval handoffs, and operator commands are all acceptance evidence. If any of
them misstate the real run, the kernel's audit posture is weaker than its model.

## Implemented Boundary

- Local Cargo observer test fixtures now combine process ID, timestamp, and an
  in-process atomic sequence and require creation of a unique root.
- First-run WorkReports count every approval request and granted decision and
  disclose both counts in approval and local-state sections.
- Operator handoff uses the supported `workflow-os inspect <run-id>` command.
- Generated agent guidance identifies automatic report generation as
  unsupported while preserving the explicit
  `workflow-os first-run --write-report-artifact` exception.
- Approval presentations branch on the typed approval subject. Aggregate
  proportional-governance authorization explicitly does not approve an authored
  step; a later authored-step gate describes only that exact step.

## Validation Contract

Acceptance requires:

- focused repeated and parallel local-host fixture tests;
- focused CLI end-to-end proof of two distinct presentations, two approval
  requests, two grants, truthful report text, and the valid inspect command;
- formatting, lint, docs, contract, and full workspace tests; and
- terminal-green required GitHub checks for the exact reviewed commit.

## Non-Goals

This fix does not add provider access, external writes, public runtime
selection, new approval kinds, automatic approval, hosted behavior, schemas, or
new Workflow OS workflow families. It does not broaden the paused GitHub
read-only Phase 2 scope.
