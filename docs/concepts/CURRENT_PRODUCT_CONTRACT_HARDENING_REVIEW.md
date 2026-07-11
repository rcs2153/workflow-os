# Current Product Contract Hardening Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The current-product contract hardening slice stayed within docs-only scope and
improves preview trust. It makes the front-door product boundary easier to
verify: version identity, existing-repository onboarding, first-run posture,
mock/demo behavior, and the recommendation-to-workflow bridge are now visible in
the README, release docs, known limitations, and Current Product Contract.

No blocker fixes are required.

## 2. Scope Verification

The phase stayed within approved scope.

Completed:

- README now points first-time evaluators to the Current Product Contract.
- Current Product Contract now states:
  - CLI version commands work without a project;
  - existing unmanaged `AGENTS.md` content is preserved by default;
  - first-run safe metadata detection is bounded and non-executing;
  - recommendation detail is available through
    `workflow-os first-run --recommendation <id>`;
  - authoring dry-run remains inactive and non-mutating;
  - mock first-run workflow execution is an approval/audit demo, not additional
    repository analysis;
  - recommendation-to-workflow promotion requires explicit draft, preflight,
    steward-review, and promote steps.
- Release readiness and known limitations docs now include the same current
  CLI/onboarding contract.
- A phase report was created.

No accidental implementation was found for:

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

## 3. Docs Truth Assessment

The updated docs now align with implemented behavior:

- `workflow-os --version` and `workflow-os version` are presented as real CLI
  identity commands.
- `init-repo-governance` is presented as implemented existing-repository
  scaffolding, not future initialization.
- `policies/local.policy.yml` remains documented in the scaffold file list.
- `first-run` is correctly presented as a report-ready posture command rather
  than a terminal WorkReport or workflow run.
- Safe repository metadata detection is bounded and explicitly does not read
  raw source contents or execute commands.
- Recommendations remain review-only until explicit authoring and promotion.
- The optional mock local run is clearly separated from real first-run
  repository posture analysis.

No dangerous stale limitation was found in the touched docs.

## 4. Product Boundary Assessment

The phase improves the product boundary without overclaiming. The Current
Product Contract now reads like the first stop for a serious evaluator:

- what is real today;
- what is mock or demonstration-only;
- what is not implemented;
- what a safe first evaluation loop looks like;
- how a recommendation can become an active workflow through explicit review
  steps.

That directly addresses the external feedback that Workflow OS is a credible
kernel but needs a sharper current-state contract.

## 5. Recommendation Bridge Assessment

The documented bridge is appropriately explicit:

1. `workflow-os first-run`
2. `workflow-os first-run --recommendation <id>`
3. `workflow-os author workflow --from-recommendation <id> --dry-run`
4. `workflow-os author workflow --from-recommendation <id> --output ...`
5. `workflow-os author workflow preflight --draft ...`
6. `workflow-os author workflow steward-review --draft ...`
7. `workflow-os author workflow promote --draft ...`

The docs correctly avoid claiming that recommendations are automatically
generated active workflows. This keeps the current preview honest while still
showing users the concrete next path.

## 6. Privacy And Safety Assessment

The docs continue to preserve the privacy boundary:

- no raw source contents;
- no raw package script bodies;
- no raw workflow file contents;
- no command output;
- no provider payloads;
- no credentials or token-like values;
- no hidden handler registration;
- no implied external writes.

The hardening is wording and navigation only; it does not introduce new output
surfaces or serialization behavior.

## 7. Test And Validation Assessment

The phase report says the implementation ran:

```sh
npm run check:docs
git diff --check
```

Both passed.

Because the phase was docs-only and did not change Rust code or tests, omitting
cargo fmt, clippy, and cargo test was acceptable. Existing CLI tests already
cover the key contract behaviors called out by the external feedback:

- version commands outside a project;
- bounded JSON version output;
- `init-repo-governance` generated file posture;
- unmanaged `AGENTS.md` preservation;
- concise first-run output plus verbose posture;
- safe metadata-aware first-run recommendations;
- recommendation detail and authoring command guidance.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Add a generated or scripted current-product-contract audit later so README,
  release docs, CLI docs, and user guide docs cannot drift silently.
- Consider adding explicit CLI docs for the full recommendation-to-workflow
  ladder in one place if future user testing shows the Current Product Contract
  is still too dense.
- Consider future preservation/discovery support for non-`AGENTS.md` agent
  instruction files such as `CLAUDE.md` or editor-specific rules.

## 10. Recommended Next Phase

Recommended next phase: provider write sandbox auth/source planning.

Reason: the current-product contract hardening lane has done the necessary
preview-trust cleanup before broader write expansion. The next write-adjacent
step should still be planning, not live mutation: define how explicit
caller-supplied auth, sandbox target proof, and no-hidden-auth rules must work
before any live GitHub PR comment sandbox call is attempted.

Do not implement provider writes, hidden auth loading, generic live adapter
execution, schemas, examples, hosted behavior, reasoning lineage, recursive
agents, agent swarms, Level 3/4 autonomy, or release posture changes as part of
that next planning phase.

## 11. Governed Review Run

- workflow: `dg/review`;
- run ID: `run-1783751512123578000-2`;
- approval ID: `approval/run-1783751512123578000-2/review-scope-approved`;
- presentation ID: `presentation/5d7d4ff16e4f2b53`;
- approval presentation enforcement: proof-enforced.

## 12. Validation

Validation commands for this review:

```sh
npm run check:docs
git diff --check
```

