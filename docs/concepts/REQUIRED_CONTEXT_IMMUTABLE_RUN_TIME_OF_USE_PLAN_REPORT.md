# Required Context Immutable-Run And Time-Of-Use Plan Report

## 1. Executive Summary

Planning now defines the safety boundary between accepted required-context
consumption and any future runtime use. The plan separates immutable
pre-consumption binding from same-call time-of-use authority re-resolution and
states explicitly that a satisfied consumption result is not a dereference
lease.

## 2. Scope Completed

- Defined immutable-run and contract binding requirements.
- Defined a separate pre-consumption execution binding.
- Defined current-authority and availability re-resolution at time of use.
- Defined source-of-truth, completeness, freshness, privacy, and failure
  boundaries.
- Defined a phased implementation sequence beginning with the binding model
  only.
- Documented the future optional sandbox relationship without implementing it.

## 3. Scope Explicitly Not Completed

No model or helper implementation, target dereference, runtime consumer,
executor integration, persistence, event, schema, CLI, provider, sandbox,
process execution, SideEffect execution, write, hosted behavior, enterprise
administration, reasoning lineage, or release change was added.

## 4. Architecture Decision

The first binding will commit:

- the exact `ImmutableRunBundleBinding`;
- exact required-context contract identity, version, and content hash;
- exact actor, workflow, run, step, harness, sensitivity, and time; and
- a versioned deterministic binding hash.

The current immutable bundle definition taxonomy will not be silently widened
to claim harness contracts are already canonical bundle records.

## 5. Time-Of-Use Boundary

Future readiness must be recomputed in one call from current typed references,
availability records, and grants. The helper must rebuild capability
resolutions, governed projections, and required-context consumption. It must
not trust prior serialized projections or consumption results.

## 6. Product And Feedback Alignment

Fresh-pull evaluation confirms the current local kernel is coherent and that
the next product need is lower ceremony without losing the evidence trail.
That aligns with proportional governance. The present plan addresses a
different prerequisite: runtime consumption must be immutable and fresh before
quiet execution can safely use context.

The previously reported Node 24 integration output and duplicate
missing-manifest diagnostic are already fixed on current `main`; no duplicate
roadmap lane was created.

## 7. Privacy And Security

The plan is payload-free. It forbids raw source, provider, command, parser,
environment, credential, log, and target content. It requires stable
non-leaking errors and redaction-safe Debug output.

## 8. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 9. Dogfood Governance

- workflow: `dg/d`
- run ID: `run-1785137351039845000-2`
- approval ID:
  `approval/run-1785137351039845000-2/planning-approved`
- presentation ID: `presentation/244090f5a2b174d5`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted planning handoff was presented
- event summary: 39 events, one approval, zero retries, zero escalations
- approval-presentation posture: proof enforced
- out-of-kernel work: architecture inspection, documentation edits, validation,
  and review writing were performed by the delegated maintainer; the kernel
  governed scope and approval but did not edit files, run git actions, invoke a
  runtime context consumer, or create a report artifact

## 10. Remaining Limitations

- Current immutable bundles do not contain canonical harness contract records.
- No complete current grant/availability candidate-set authority exists yet.
- No runtime consumer or dereference boundary exists.
- Independent policy, approval, evidence, and check records are not yet
  composed into time-of-use readiness.
- Optional sandbox execution remains future work.

## 11. Recommended Next Phase

Implement the **required-context immutable execution-binding core model only**,
then review it before planning the time-of-use re-resolution helper.

Do not broaden runtime execution or provider mutation in that phase.
