# External Kernel Review Follow-Up Report

## 1. Executive Summary

An external kernel review against an older commit found several first-use trust
and product-contract issues: missing CLI version behavior, docs drift around
project initialization, incomplete scaffold file documentation, ambiguity in
first-run repository metadata, and a need for a stronger bridge from
recommendations to governed workflow authoring.

Current `main` has already addressed the actionable items in bounded local
preview form. This follow-up records the verification so the feedback remains
visible and does not get mistaken for unresolved current behavior.

## 2. Feedback Reviewed

The review described Workflow OS as a credible local governance kernel with
strong deterministic validation, approval-gated execution, event-sourced local
state, fail-closed behavior, and honest mock-handler boundaries.

The high-signal issues were:

- `workflow-os --version` should work without a project.
- known limitations docs should not claim project initialization is missing
  now that `init-repo-governance` exists.
- `init-repo-governance` generated-file docs should include
  `policies/local.policy.yml`.
- first-run output should distinguish repository metadata from scaffold-created
  governance files.
- the bridge from first-run recommendations to governed workflow authoring
  should be visible.

## 3. Current-State Assessment

Current `main` already includes:

- `workflow-os --version` and `workflow-os version` bounded CLI version output.
- `workflow-os run --help` and command-local help handling that do not treat
  `--help` as a workflow id.
- current known limitations text that documents `init-repo-governance`,
  `init-agent-harness`, `first-run`, recommendation detail, and authoring
  dry-run boundaries.
- `docs/cli/init-repo-governance.md` generated-file docs that include
  `policies/local.policy.yml`.
- first-run safe repository metadata output that separates scaffold-created
  Workflow OS directories from user repository source/test directory signals.
- first-run recommendation detail and `author workflow --dry-run` guidance for
  review-only workflow authoring.

## 4. Scope Completed

This phase verified that the current tree has already closed the concrete
feedback items without adding new runtime behavior.

No code change was required.

## 5. Scope Explicitly Not Completed

This follow-up does not implement:

- new runtime primitives;
- workflow schema changes;
- handler execution changes;
- automatic local check execution;
- persistence changes;
- CLI command expansion beyond already-implemented version/help polish;
- provider writes;
- examples;
- hosted behavior;
- release posture changes.

## 6. Validation

Commands run:

- `workflow-os --version` - passed.
- `workflow-os run --help` - passed.
- `cargo test -p workflow-cli --test cli version` - passed.
- `cargo test -p workflow-cli --test cli help` - passed.
- `cargo test -p workflow-cli --test cli init_repo_governance` - passed.
- `cargo test -p workflow-cli --test cli first_run_separates_scaffold_only_test_dir_from_repo_metadata` - passed.

## 7. Dogfood Governance

This follow-up was governed by the local Workflow OS dogfood runner.

- workflow ID: `dg/implement`
- run ID: `run-1783773173861321000-2`
- approval ID: `approval/run-1783773173861321000-2/implementation-approved`
- approval presentation ID: `presentation/3a4401b4e1fd6b84`
- approval outcome: granted

Out-of-kernel work: current-state inspection, targeted validation commands, and
this report were performed by the executor under the governed phase boundary.

## 8. Remaining Limitations

The feedback still points at a larger product direction: first-run should keep
getting easier to scan, and recommendation-to-workflow authoring should continue
to become more concrete without becoming automatic workflow generation.

Those are roadmap execution items, not blockers for the current product-contract
surface.

## 9. Recommended Next Phase

Recommended next phase: continue the roadmap item after current-product
first-use hardening.

Reason: the concrete defects identified by this external review are already
closed on current `main`; the next valuable work should continue reducing the
gap between reviewed recommendations and explicit governed workflow authoring
while preserving the local preview boundary.
