# Workflow OS User Guide

These guides cover the current Workflow OS `0.4.0-preview.1` preparation
identity on `main`. Workflow OS is an advanced local governance preview, not a
production enterprise control plane.

Start with the [Current Product Contract](current-product-contract.md) when a
guide, implementation report, or older release document appears to disagree
with current behavior.

## Current Posture

| Area | Current status |
| --- | --- |
| Repository onboarding | Implemented. Existing unmanaged `AGENTS.md` content is preserved by default. |
| First-run analysis | Implemented for allowlisted repository metadata, concise or verbose posture, and review-only workflow recommendations. |
| Local execution | Deterministic sequential multi-step workflows with policy evaluation, approvals, retries, escalation, cancellation, and durable rehydration. |
| Authority and continuity | Immutable run inputs, approval-presentation proof, bounded continuation projection, execution-window and wait foundations. |
| Evidence and reporting | Evidence references, side-effect records, WorkReport generation, and explicit governed local report-artifact paths. Assurance is path-specific; local report storage is not independently tamper-proof. |
| Proportional governance | Bounded routing among quiet capture, visible disclosure, blocking approval, and denial. |
| State | Local filesystem, SQLite continuity, and PostgreSQL implementations; production HA is not claimed. |
| Read-only integrations | Fixture-first and opt-in live GitHub, Jira, and GitHub Actions paths. |
| External writes | Narrow GitHub comment and managed draft-PR execution paths only; no general write-adapter product. |
| Hosting | Single-tenant no-write alpha; not production multi-tenant hosting. |
| OpenShell | Optional no-write boundary and compatibility work; not a complete execution-provider integration. |
| Self-governance | `dg/*` workflows govern this repository and serve as team-specific examples, not downstream defaults. |
| Higher autonomy | Level 3/4 declarations remain denied by default and are not a production autonomy offering. |

## Start Here

1. [Root quickstart](../../README.md): build the CLI and govern an existing
   repository.
2. [Current Product Contract](current-product-contract.md): exact implemented
   and unavailable behavior.
3. [Agent Harness Quickstart](agent-harness-quickstart.md): use a coding agent
   as executor while Workflow OS governs the work.
4. [First-run CLI guide](../cli/first-run.md): repository posture,
   recommendations, and governed report artifacts.
5. [Vertical-slice example](../../examples/vertical-slice-approval/README.md):
   approval, resume, persistence, and event inspection.

## Design And Evaluation Guides

- [Workbook](workbook.md): qualify and design a governed workflow before
  writing specifications.
- [Self-Governed Build Benchmark](self-governed-build-benchmark.md): maintained
  dogfood loop for Workflow OS development.
- [RC1 Evaluation Guide](rc1-evaluation-guide.md): historical bounded
  evaluation path for the earlier local-kernel/read-only-adapter milestone.
- [Field Guide](field-guide.md): earlier operating-model narrative. Use the
  Current Product Contract for present capability claims.

## Reference

- [CLI overview](../cli/overview.md)
- [Workflow specifications](../specs/workflows.md)
- [Runtime overview](../runtime/README.md)
- [Architecture overview](../architecture/overview.md)
- [Operations](../operations/README.md)
- [Security](../security/README.md)
- [Troubleshooting](../operations/TROUBLESHOOTING.md)
- [Known limitations](../release/V0_KNOWN_LIMITATIONS.md)

Older release, readiness, review, and implementation-plan documents are
historical evidence for the version or phase named in the file. They should not
override the Current Product Contract or current CLI documentation.
