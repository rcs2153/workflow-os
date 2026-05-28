# Workflow OS User Guide

These guides are for RC1 internal evaluation of the current Workflow OS repository.

RC1 internal evaluation means the local kernel preview can be evaluated seriously, and Phase 2 read-only adapters can be evaluated against fixtures. It does not mean production readiness, hosted service readiness, public read-only integration preview readiness, or write-capable adapter readiness.

## Current Posture

| Area | Status |
| --- | --- |
| Local kernel preview | Implemented and ready for public local-kernel preview evaluation. |
| Vertical slice approval example | Implemented with explicit deterministic local mock handler. |
| GitHub/Jira/CI read-only adapters | Internal fixture-backed Phase 2 capability on the development branch. |
| Adapter telemetry mapping | Internal read-only telemetry mapping for controlled fixture-backed examples. |
| Live provider proof | Not recorded yet. Public read-only integration preview remains blocked. |
| Governed Work Pattern | Proposed architecture direction only. |
| Reasoning Lineage / Claim Graph | Proposed architecture direction only. |
| GitHub/Jira writes and CI reruns/dispatch | Unsupported. |
| Production backend, distributed workers, hosted service, UI, marketplace | Unsupported. |
| Level 3/4 autonomy | Declaration-only and denied by default. |

## Guides

- [Field Guide](field-guide.md): narrative guide for the operating model, current implementation boundary, and why Workflow OS matters.
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
- [Phase 2 public read-only preview readiness](../integrations/PHASE_2_PUBLIC_READ_ONLY_PREVIEW_READINESS.md)

Use these guides as evaluation and operating artifacts, not as proof that unsupported production behavior exists.
