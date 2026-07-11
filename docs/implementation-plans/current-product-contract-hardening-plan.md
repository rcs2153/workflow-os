# Current Product Contract Hardening Plan

Status: Planned.

This plan follows the accepted [Provider Write Sandbox Readiness Helper
Review](../concepts/PROVIDER_WRITE_SANDBOX_READINESS_HELPER_REVIEW.md). That
review accepted the write-readiness helper but recommended pausing broader
write-adjacent expansion long enough to harden the current user-facing product
contract.

## 1. Executive Summary

External evaluator testing confirmed that Workflow OS is credible as a local
governance kernel: validation is deterministic, approval-gated execution is
real, event history is inspectable, state is local and durable, and unsupported
skill execution fails closed.

The same testing identified a preview-readiness risk: users can read a large
body of docs and see many implemented primitives, but the first-use contract
still needs to be sharper. The next work should not add another primitive
family. It should make the current local kernel contract harder to
misunderstand and easier to verify.

This plan is planning only. It does not implement runtime behavior, provider
writes, schemas, examples, hosted behavior, reasoning lineage, recursive
agents, agent swarms, Level 3/4 autonomy, or release posture changes.

## 2. Goals

- Keep the current product contract explicit: local governance kernel, not
  production workflow automation runtime.
- Lock CLI identity behavior so users can inspect version/build posture without
  needing a project.
- Sweep docs for stale limitations and scaffold-file mismatches.
- Preserve and document the distinction between real `first-run` posture
  analysis and optional mock approval/audit demos.
- Treat existing AGENTS/agent guidance preservation as part of the onboarding
  contract.
- Make bounded safe-repository metadata detection and concrete first-run
  recommendations visible without overclaiming source analysis.
- Strengthen the bridge from `first-run` recommendations to reviewed workflow
  authoring.
- Keep recommendations review-only until explicit authoring, preflight,
  stewardship, and promotion boundaries are used.
- Preserve redaction, bounded output, and non-leaking error behavior.

## 3. Non-Goals

Do not implement in this phase:

- provider writes;
- write-capable adapters;
- automatic workflow generation;
- automatic workflow promotion;
- automatic local check execution;
- hidden skill handler registration;
- runtime report artifact generation outside existing explicit paths;
- hosted or distributed runtime behavior;
- CLI commands that call external systems;
- workflow schema changes;
- examples;
- reasoning lineage or claim graph;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Implemented Baseline

Some external feedback items are already implemented in current main. This plan
treats them as baseline contract that should be documented and protected, not
as brand-new work unless audit finds drift.

Implemented baseline includes:

- `workflow-os --version`, `workflow-os version`, and bounded JSON version
  behavior without requiring a project.
- `workflow-os init-repo-governance` includes `policies/local.policy.yml` in
  its generated-file documentation.
- `init-repo-governance` and `init-agent-harness` preserve existing unmanaged
  `AGENTS.md` content by default and update managed Workflow OS blocks in
  place.
- `workflow-os first-run` has concise default text plus `--verbose` for the
  full posture matrix.
- `workflow-os first-run` detects bounded safe metadata for TypeScript/package,
  Rust, Python, Go, GitHub Actions, conventional source/test directories, and
  common repo documents without reading source contents or executing commands.
- `workflow-os first-run` distinguishes generated Workflow OS scaffold
  directories from user repository metadata.
- `workflow-os first-run --recommendation <id>` exposes bounded detail for one
  existing review-only recommendation.
- `workflow-os author workflow --from-recommendation <id> --dry-run` previews
  inactive workflow-authoring obligations without writing files.
- `workflow-os author workflow --from-recommendation <id> --output ...` writes
  one inactive draft workflow only under explicit output scope.

## 5. Product Contract Risks

The remaining risk is not that the kernel is fake. The risk is that a new user
cannot quickly distinguish:

- implemented local kernel behavior from future roadmap vocabulary;
- first-run posture analysis from optional mock runtime demos;
- review-only recommendations from active workflows;
- scaffold-managed files from user repository metadata;
- docs that describe current behavior from historical phase reports;
- explicit local preview helpers from production automation guarantees.

Those distinctions are part of the product, not just documentation polish.

## 6. Workstream A: CLI Identity Contract

The CLI must reliably answer "what version/build am I running?" without a
project, state directory, network, provider access, or workflow spec.

Future implementation should verify and lock:

- `workflow-os --version`;
- `workflow-os version`;
- `workflow-os --json version` or the implemented JSON equivalent;
- bounded output with no paths, environment values, secrets, or local project
  payloads;
- tests that these commands work outside a Workflow OS project;
- docs that mention the command in first-use troubleshooting.

If current behavior already satisfies this, the implementation phase should
limit itself to regression tests and docs alignment.

## 7. Workstream B: Documentation Truth Sweep

The next implementation should run a focused truth sweep over current user docs:

- README;
- CLI docs;
- release readiness and known limitations docs;
- agent harness quickstart;
- first-run docs;
- init-repo-governance docs;
- current roadmap pointers.

The sweep should remove stale claims such as "initialization is not
implemented" when `init-repo-governance` is implemented, and should ensure
generated-file lists match actual scaffold behavior.

It should not rewrite historical phase reports. Historical docs can remain
historical if they are not presented as current product contract.

## 8. Workstream C: Scaffold Preservation Contract

Real repositories may already have useful agent guidance in `AGENTS.md`,
`CLAUDE.md`, Cursor rules, Copilot instructions, or project-specific
contribution notes. Workflow OS should coexist with that guidance.

The current baseline preserves unmanaged `AGENTS.md` content by default. The
hardening work should make this explicit in docs and tests:

- managed Workflow OS blocks are updated in place;
- unmanaged surrounding content is preserved;
- `--force` is an explicit replacement boundary;
- dry-run output discloses intended writes without echoing existing file
  contents;
- errors remain bounded and non-leaking.

Future support for other agent-instruction files is out of scope for this
phase, but should remain visible as a follow-up.

## 9. Workstream D: First-Run Operator UX Contract

`workflow-os first-run` is the preview's strongest product loop. It should feel
like a bounded operator briefing, not an internal dump.

The current baseline already has concise default output and `--verbose`. The
hardening work should verify that default output clearly states:

- what happened;
- what did not happen;
- what matters now;
- which recommendations are review-only;
- which command inspects one recommendation;
- which command previews workflow authoring;
- that the optional mock run is an approval/audit demo, not additional repo
  analysis.

Verbose and JSON output should preserve the full bounded posture matrix for
audit-minded users and machine consumers.

## 10. Workstream E: Recommendation-To-Workflow Bridge

The user-facing bridge should be:

1. `workflow-os first-run`
2. `workflow-os first-run --recommendation <id>`
3. `workflow-os author workflow --from-recommendation <id> --dry-run`
4. `workflow-os author workflow --from-recommendation <id> --output ...`
5. `workflow-os author workflow preflight --draft ...`
6. `workflow-os author workflow steward-review --draft ...`
7. `workflow-os author workflow promote --draft ...`

The contract hardening implementation should ensure docs and command guidance
make this ladder visible without implying automatic workflow generation.

Recommendations remain review-only until a draft is explicitly written,
preflighted, reviewed, and promoted.

## 11. Privacy And Redaction

All hardening work must preserve the current privacy boundary:

- no raw source contents;
- no raw package script bodies;
- no raw GitHub Actions workflow contents;
- no command output;
- no provider payloads;
- no environment values;
- no credentials, authorization headers, tokens, or private keys;
- no unbounded owner, escalation, note, risk, limitation, or handoff text;
- no path leakage beyond explicitly documented safe path/posture outputs.

Errors should use stable codes and should not echo unsafe caller-supplied
values.

## 12. Test Plan

Future implementation should add or verify tests for:

- version commands outside a Workflow OS project;
- generated-file docs parity for `init-repo-governance`;
- existing unmanaged `AGENTS.md` preservation by default;
- explicit `--force` replacement posture;
- first-run default output includes concise operator summary;
- first-run verbose output still includes detailed posture matrix;
- first-run JSON remains bounded and preview-marked;
- safe metadata detection does not copy manifest bodies, script bodies, source
  contents, or workflow contents;
- scaffold-generated `tests/` is not mislabeled as user test metadata;
- recommendation detail uses only existing recommendation IDs;
- authoring dry-run remains non-mutating;
- optional mock run remains labeled as approval/audit demo only;
- docs check passes.

## 13. Proposed Implementation Sequence

1. Run a current-product contract audit against README, CLI docs, release docs,
   first-run docs, and known limitations.
2. Patch stale docs and generated-file mismatches.
3. Add or tighten focused regression tests only where current behavior is not
   already covered.
4. Improve default first-run/operator copy only if audit finds the implemented
   output still obscures the current contract.
5. Review the phase before returning to broader write-capable adapter work.

## 14. Deferred Work

Deferred:

- stable CLI JSON contract;
- generated schemas;
- public examples for every recommendation-to-workflow path;
- automatic local check registration;
- automatic workflow recommendation promotion;
- support for preserving non-`AGENTS.md` agent-instruction files;
- hosted workflow catalog;
- collaborative steward/admin controls;
- provider writes;
- reasoning lineage.

## 15. Recommended Next Phase

Recommended next phase: current-product contract hardening implementation.

The implementation should start with docs truth and regression lock-in, not new
runtime primitives. If audit confirms most behavior is already implemented, the
phase should be small: align docs, add missing tests, and make the first-run to
authoring path unmissable.

Do not proceed directly to broader provider writes until this contract is
reviewed.

## 16. Governed Planning Run

- workflow: `dg/d`;
- run ID: `run-1783749619410888000-2`;
- approval ID: `approval/run-1783749619410888000-2/planning-approved`;
- presentation ID: `presentation/1f3a62792bd2243e`;
- approval presentation hash:
  `1f3a62792bd2243e78f1804afc66b3d12637013592a3809c06b1edc38d325549`;
- approval outcome: delegated maintainer approved;
- approval presentation enforcement: proof-enforced.

