# Proportional-Governance Selected Local Project Consumer CLI Adoption Plan Report

## 1. Executive Summary

Workflow OS now has a phase-ready plan for adopting the accepted selected
project-validation consumer in the existing manifest-controlled CLI path.
The plan preserves the public `run` and `approve` commands, ordinary execution
for undeclared projects, human and JSON output, proof-enforced approvals,
durable event ordering, and local report-artifact obligations.

No runtime implementation was added.

## 2. Scope Completed

- Inspected the current declared authoritative CLI run and approval paths.
- Inspected the accepted selected route and approval-artifact APIs.
- Identified the missing selected fresh-run report-composition adapter.
- Defined Core and CLI ownership boundaries.
- Defined exact command routing, store posture, failure ordering, compatibility,
  migration, tests, and phased implementation.
- Required separate focused review before any CLI behavior changes.

## 3. Scope Explicitly Not Completed

The phase added no Rust behavior, CLI change, default activation, executor
change, provider or OpenShell integration, SideEffect execution, external
mutation, schema, SDK, example, hosted behavior, enterprise administration, or
release change.

## 4. Product Decision

The existing validated project declaration remains the sole activation source.
There is no new flag, command, mode, or default. The CLI may select explicit
local dependencies, but it may not construct authority-bearing facts, source
registration, evaluation time, route disposition, or receipts.

## 5. Prerequisite Identified

The selected fresh-run API returns route truth and the actual same-call check
result, but the CLI requires the accepted terminal WorkReport envelope across
quiet, visible, denied, existing-terminal, and approval-required outcomes.

The first implementation phase must add one selected report adapter in Core.
It must reuse the same route call and check result, defer reports for
non-terminal approvals, generate reports for terminal routes, and retain run
truth when report construction fails. It must not change the CLI.

## 6. CLI Adoption Decision

After the adapter is accepted:

- declared `run` uses the selected report adapter;
- every approval gate created under the selected V3 binding uses the selected
  decision helper;
- aggregate and authored-step approvals remain distinct;
- receipt and artifact stores are explicit deterministic local dependencies;
- no production shadow execution reruns checks or workflow effects; and
- existing human/JSON output and exit behavior remain the compatibility
  contract.

## 7. Validation

- `npm run check:docs`: passed under the repository Node 20 toolchain.
- `git diff --check`: passed.
- Governed phase inspection and closure: passed.

## 8. Governed Planning Record

- Dogfood workflow: `dg/d`.
- Run ID: `run-1786445022247849000-2`.
- Approval ID:
  `approval/run-1786445022247849000-2/planning-approved`.
- Presentation ID: `presentation/7e1ef5336bc3206d`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: `Completed`.
- Event summary: 39 events, including one approval request, one approval grant,
  six scheduled steps, six successful skill invocations, no retries, and no
  escalations.
- Approval-presentation enforcement: proof enforced with the presentation
  marker present in the durable event trail.
- Out-of-kernel work: Core and CLI inventory plus documentation authoring.

## 9. Remaining Limitations

- No selected fresh-run report adapter exists yet.
- The CLI still invokes the earlier Core-owned authoritative composition.
- Exact receipt-store subpath and denial artifact compatibility require focused
  implementation evidence.
- Multi-step authoritative governance and additional check profiles remain out
  of scope.

## 10. Recommended Next Phase

Perform focused maintainer review of the CLI adoption plan. If accepted,
implement only the selected fresh-run report-composition adapter and review it
before changing CLI behavior.
