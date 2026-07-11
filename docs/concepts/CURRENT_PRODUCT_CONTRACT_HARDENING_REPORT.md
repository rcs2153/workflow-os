# Current Product Contract Hardening Report

## 1. Executive Summary

Current-product contract hardening is complete for this slice. The phase
responded to external evaluator feedback by tightening the front-door
documentation around what Workflow OS does today, what is mock/demo-only, and
how first-run recommendations bridge into reviewed workflow authoring.

No runtime behavior, provider writes, schemas, examples, hosted behavior,
reasoning lineage, or release posture changed.

## 2. Scope Completed

- Added the accepted current-product contract hardening plan.
- Added a roadmap pointer that prioritizes current-product clarity before
  broader write-capable adapter expansion.
- Updated README to point first-time evaluators at the Current Product
  Contract.
- Expanded the Current Product Contract with:
  - version command posture;
  - existing agent-instruction preservation posture;
  - safe repository metadata posture;
  - recommendation detail posture;
  - explicit recommendation-to-workflow bridge;
  - clearer distinction between real first-run posture and the optional mock
    approval/audit demo.
- Updated release readiness and known limitations docs with the same current
  CLI/onboarding contract.

## 3. Scope Explicitly Not Completed

Not implemented:

- provider writes;
- automatic workflow generation;
- automatic workflow promotion;
- automatic local check execution;
- hidden skill handler registration;
- runtime report artifact expansion;
- hosted behavior;
- schemas;
- examples;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Baseline Verified

The audit found that several external-feedback items were already implemented
and tested in current main:

- `workflow-os --version`;
- `workflow-os version`;
- bounded JSON version output;
- generated `policies/local.policy.yml` docs;
- unmanaged `AGENTS.md` preservation by default;
- concise default `first-run` output plus `--verbose`;
- safe metadata-aware first-run recommendations;
- generated scaffold directory separation;
- recommendation detail;
- authoring dry-run.

This phase documented those behaviors as the current product contract rather
than adding duplicate runtime code.

## 5. Validation Summary

Validation commands run:

```sh
npm run check:docs
git diff --check
```

Both passed before PR creation.

No Rust code or Rust tests changed, so cargo formatting, clippy, and full cargo
test were not required for this docs-only implementation slice.

## 6. Governed Run Summary

Planning run:

- workflow: `dg/d`;
- run ID: `run-1783749619410888000-2`;
- approval ID: `approval/run-1783749619410888000-2/planning-approved`;
- presentation ID: `presentation/1f3a62792bd2243e`;
- approval presentation enforcement: proof-enforced.

Implementation run:

- workflow: `dg/implement`;
- run ID: `run-1783750639521146000-2`;
- approval ID: `approval/run-1783750639521146000-2/implementation-approved`;
- presentation ID: `presentation/23c11d328b2ae6a6`;
- approval presentation enforcement: proof-enforced.

## 7. Remaining Known Limitations

- CLI JSON remains preview and not a stable machine contract.
- The first-run recommendation bridge still requires explicit user/agent
  action; recommendations do not automatically become workflows.
- Non-`AGENTS.md` agent instruction files are not yet merged or managed.
- Current-product truth still depends on discipline across README, release docs,
  CLI docs, and user guide docs until a generated product-contract index exists.

## 8. Recommended Next Phase

Recommended next phase: current-product contract hardening review.

Reason: this slice intentionally changed user-facing docs only. A focused review
should verify that the docs now match implemented behavior and do not overclaim
runtime, provider-write, schema, example, hosted, reasoning-lineage, or release
capabilities.

