# Current Product Contract

Workflow OS v0 is a local-first governance kernel for AI-assisted work. It is
useful today for evaluating governed local workflow execution, repository
onboarding posture, durable event trails, approval checkpoints, report posture,
and selected read-only integration contracts.

It is not yet a production automation platform, hosted runtime, write-capable
adapter framework, recursive agent system, or enterprise control plane.

## What Is Real Today

- `workflow-os --version` and `workflow-os version` report the CLI version
  without requiring a Workflow OS project.
- `workflow-os validate` loads and validates a local Workflow OS project.
- `workflow-os init-repo-governance` scaffolds a minimal valid governance
  envelope into an existing repository while preserving existing unmanaged
  `AGENTS.md` content by default.
- `workflow-os init-agent-harness` adds agent-orientation files with Workflow
  OS managed blocks and preserves unmanaged surrounding content by default.
- `workflow-os first-run` produces bounded report-ready governance posture and
  review-only workflow recommendations without starting a run.
- `workflow-os first-run` detects bounded safe repository metadata such as
  manifest presence, allowlisted package script keys, ecosystem markers,
  conventional source/test directories, GitHub Actions counts, and common repo
  documents without reading source contents or executing commands.
- `workflow-os first-run --recommendation <id>` explains one existing
  recommendation using bounded rationale and next-action codes.
- `workflow-os first-run` discloses the current local governance strictness
  profile as `observe_and_report`; this is a typed product-contract disclosure,
  not enterprise profile enforcement.
- `workflow-os author workflow --from-recommendation <id> --dry-run` previews
  inactive workflow authoring obligations.
- `workflow-os author workflow --from-recommendation <id> --output <path>`
  writes one inactive draft workflow file for review under the approved output
  boundary.
- `workflow-os run`, `approve`, `status`, `inspect`, and `doctor state` exercise
  the local executor and local filesystem state backend.
- Approval-gated local workflows pause, resume, fail closed on denial, and leave
  durable event history.
- Sequential multi-step local workflows are implemented.
- Read-only GitHub, Jira, and GitHub Actions/CI adapters are available as
  fixture-first preview integrations with opt-in live reads.
- EvidenceReference, WorkReport, SideEffect, high-assurance approval, hook, and
  report artifact foundations exist where documented, mostly as explicit
  model/helper or opt-in local paths.

## What Is Mock Or Demonstration-Only

- `--mock-all-local-skills` registers deterministic mock handlers for eligible
  `local/*` skills. It proves the kernel path; it is not a production skill
  plugin system.
- The scaffolded `local/first-run-governance` workflow is an approval/audit demo
  unless the user later supplies real local handlers.
- Repo-local `dg/*` workflows are Workflow OS dogfood benchmark workflows for
  this repository, not downstream plug-and-play defaults.
- Draft workflow authoring outputs are review-only until explicitly promoted
  through the supported promotion path.

## What Is Not Implemented

- Hosted or distributed runtime.
- Production database, queue, or distributed locking backend.
- Automatic report generation for every run.
- Automatic report artifact writing from default executor paths.
- CLI report rendering/export.
- Generic live adapter execution commands.
- GitHub/Jira/CI write operations by default.
- Arbitrary shell command execution.
- Automatic local check execution by default.
- Runtime nested harness execution.
- Recursive agents, agent swarms, or Level 3/4 autonomy.
- Enterprise RBAC, IdP integration, quorum approval, or hosted policy service.
- Enterprise governance strictness stewardship or admin-controlled profile
  enforcement.
- Reasoning Lineage / Claim Graph runtime implementation.

## Safe First Evaluation Loop

From an existing repository:

```sh
workflow-os --version
workflow-os validate
workflow-os init-repo-governance
workflow-os validate
workflow-os first-run
workflow-os first-run --recommendation <id>
workflow-os author workflow --from-recommendation <id> --dry-run
workflow-os --mock-all-local-skills run local/first-run-governance
workflow-os inspect <run-id>
workflow-os doctor state
```

The useful first product loop is:

1. create a local governance envelope;
2. inspect first-run posture and recommendations;
3. inspect one recommendation in detail;
4. preview workflow authoring obligations without writing files;
5. optionally run the mock approval/audit demo;
6. author, preflight, steward-review, and promote draft workflows explicitly.

The optional mock run is not additional repository analysis. The real
first-use product signal is `first-run`: it maps bounded safe project context,
discloses what is missing or skipped, and points at review-only governed
workflow candidates.

## Recommendation To Workflow Bridge

The current explicit bridge from recommendation to active workflow is:

```sh
workflow-os first-run
workflow-os first-run --recommendation <id>
workflow-os author workflow --from-recommendation <id> --dry-run
workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml
workflow-os author workflow preflight --draft workflows/drafts/<name>.workflow.yml
workflow-os author workflow steward-review --draft workflows/drafts/<name>.workflow.yml
workflow-os author workflow promote --draft workflows/drafts/<name>.workflow.yml --reviewer <actor> --reason <reason>
```

Each step is explicit. Recommendations are not active workflows until a draft is
written, preflighted, reviewed, and promoted.

## Trust Boundary

Workflow OS should not be trusted because it claims to control an agent. It
should be evaluated by what it records, validates, gates, discloses, and refuses
to overclaim.

The operating boundary remains:

```text
Agent executes. Workflow OS governs.
```
