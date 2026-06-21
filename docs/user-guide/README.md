# Workflow OS User Guide

These guides are for RC1 internal evaluation of the current Workflow OS repository.

RC1 internal evaluation means the local kernel preview can be evaluated seriously, and Phase 2 read-only adapters can be evaluated against fixtures. The `0.2.0-preview.1` public read-only integration preview adds narrow opt-in live-provider evidence, but RC1 evaluation still does not mean production readiness, hosted service readiness, broad live provider compatibility, or write-capable adapter readiness.

## Current Posture

| Area | Status |
| --- | --- |
| Local kernel preview | Implemented and ready for public local-kernel preview evaluation. |
| Vertical slice approval example | Implemented with explicit deterministic local mock handler. |
| GitHub/Jira/CI read-only adapters | Public read-only integration preview in `0.2.0-preview.1`; fixture-first and opt-in for live providers. |
| Adapter telemetry mapping | Local/runtime-preview telemetry mapping for controlled fixture-backed examples. |
| Live provider proof | Recorded for one narrow read path per provider family; broader provider operation coverage remains fixture-tested, not live-proven. |
| Governed Work Pattern | Accepted architecture direction. Selected evidence-reference and work-report foundations are implemented; broader pattern behavior remains scoped future work. |
| Evidence references | Core model plus selected adapter telemetry, diagnostic, and schema-version diagnostic attachment paths are implemented. Persistence, CLI rendering, examples, approval attachment, and broad automatic attachment remain unsupported. |
| Work reports | Core contract/report models, explicit in-memory local report generation/result helper APIs, executor-integrated report-bearing execution, and explicit local artifact store are implemented. Automatic runtime report generation, CLI rendering, examples, schemas, and automatic artifact writing remain unsupported. |
| Self-governance dogfooding | First dogfood project is implemented as kernel-governed, Codex-executed Workflow OS build governance. The `dg/*` workflows are reference dogfood workflows for this repository, not community defaults or required user templates. The repo-local `npm run dogfood:benchmark` helper wraps existing CLI commands for the benchmark. Real build skills, automatic execution, recursive agents, and production self-hosting remain unsupported. |
| Agent harness onboarding | Documented as the recommended local adoption path: a coding agent executes repository work while Workflow OS governs validation, approval checkpoints, durable state, and report posture. `workflow-os init-agent-harness` is implemented as a documentation scaffold for agent instructions and a session prompt. Runtime harness auto-generation remains unsupported. |
| Reasoning Lineage / Claim Graph | Proposed architecture direction only. |
| GitHub/Jira writes and CI reruns/dispatch | Unsupported. |
| Production backend, distributed workers, hosted service, UI, marketplace | Unsupported. |
| Level 3/4 autonomy | Declaration-only and denied by default. |

## Guides

- [Field Guide](field-guide.md): narrative guide for the operating model, current implementation boundary, and why Workflow OS matters.
- [Agent Harness Quickstart](agent-harness-quickstart.md): copy/paste setup path for using Codex, Claude Code, or another coding agent with Workflow OS as the governing layer.
- [Self-Governed Build Benchmark](self-governed-build-benchmark.md): maintainer runbook and repo-local helper path for using the local kernel to govern Workflow OS development work without overclaiming automation.
- [Workbook](workbook.md): fillable markdown workbook for qualifying, designing, governing, and evaluating workflows before writing specs.
- [RC1 Evaluation Guide](rc1-evaluation-guide.md): exact safe evaluation paths for the local kernel, vertical slice, read-only fixture adapters, and telemetry inspection.

## Related Documentation

- [Root quickstart](../../README.md)
- [CLI overview](../cli/overview.md)
- [Project charter](../PROJECT_CHARTER.md)
- [Known limitations](../release/V0_KNOWN_LIMITATIONS.md)
- [Troubleshooting](../operations/TROUBLESHOOTING.md)
- [Security overview](../security/README.md)
- [Policy engine](../runtime/policy-engine.md)
- [Phase 2 public read-only preview readiness rerun](../integrations/PHASE_2_PUBLIC_READ_ONLY_PREVIEW_READINESS_RERUN.md)

Use these guides as evaluation and operating artifacts, not as proof that unsupported production behavior exists.
