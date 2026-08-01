# Authoritative Governance Scaffold Default Activation Report

## 1. Executive Summary

New `workflow-os init-repo-governance` scaffolds now select the accepted closed
authoritative governance path by default. The generated project declares
`observe_and_report`, binds the fixed `workflow_os_project_validation` profile,
and places the canonical project-validation requirement on the generated
workflow.

The scaffold command remains non-executing. It does not infer repository
commands, approve work, call providers, create runtime state, or authorize
external writes.

## 2. Scope Completed

- Replaced the scaffold boolean with an explicit enabled/disabled selection.
- Made enabled authoritative governance the no-flag default.
- Preserved `--authoritative-governance` as a compatible spelling.
- Added `--no-authoritative-governance` for the legacy undeclared scaffold.
- Rejected contradictory positive and negative flags before filesystem
  planning or writes.
- Disclosed enabled or disabled posture in normal and dry-run output.
- Updated CLI help, current-product, onboarding, roadmap, and planning docs.

## 3. Scope Explicitly Not Completed

This phase did not add inferred checks, arbitrary commands, automatic approval,
new proportional-governance profiles, providers, OpenShell, sandbox execution,
credentials, network access, SideEffect execution, external writes, hosted
behavior, schemas, SDK changes, examples, or release changes.

Existing repositories and manifests are not migrated.

## 4. CLI And Compatibility Behavior

The command now behaves as follows:

- no posture flag: closed authoritative scaffold;
- `--authoritative-governance`: the same closed authoritative scaffold;
- `--no-authoritative-governance`: legacy undeclared scaffold; and
- both posture flags: stable fail-closed error before writes.

Existing file protection, `--force`, managed `AGENTS.md` preservation, and
dry-run non-mutation remain unchanged.

## 5. Generated Governance Contract

The default manifest contains:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

The generated workflow contains the exact canonical, required, no-reuse,
source-read-only, network-disabled project-validation check requirement.

## 6. Runtime And Approval Behavior

Later execution derives activation from the validated declaration and immutable
run bundle. Workflow Core derives runtime facts from canonical sources and
selects quiet, visible, approval-required, or denied posture.

The aggregate governance approval and the authored workflow-step approval
remain separate gates. Completing the accepted path persists exactly one
terminal WorkReport artifact.

## 7. Privacy And Failure Posture

The new default stores only fixed identifiers and closed posture values.
Contradictory flags, invalid projects, declaration drift, missing canonical
checks, failed checks, stale approval context, and artifact-integrity failures
continue to fail closed through existing bounded errors.

No source content, command output, provider payload, path, environment value,
credential, or token is added to scaffold output.

## 8. Test Coverage

Focused coverage proves:

- exact default declaration and workflow requirement;
- positive compatibility behavior;
- explicit legacy behavior and validation;
- contradictory selection before files or state;
- enabled/disabled dry-run disclosure without writes;
- existing agent-guidance preservation;
- full default first-run posture;
- the accepted two-approval terminal path;
- one WorkReport artifact;
- legacy one-approval compatibility; and
- unknown-option fail-closed behavior.

The complete CLI suite passes with 169 tests.

## 9. Validation

Completed during implementation:

- focused `init_repo_governance` CLI tests: 14 passed;
- complete `workflow-cli` CLI integration suite: 169 passed;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `npm run check:integrations` under the repository's validated Node 20
  runtime; and
- `git diff --check`.

A disposable repository proof generated the no-flag default, validated the
project, reported enforced first-run posture, completed the separate aggregate
and authored approvals, and persisted one WorkReport artifact.

## 10. Governed Phase Record

- workflow: `dg/implement`;
- run: `run-1785523043959130000-2`;
- approval:
  `approval/run-1785523043959130000-2/implementation-approved`;
- presentation: `presentation/d65d7907a943873f`;
- approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof;
- phase status: `Completed`;
- event summary: 39 events, one approval, no retries, and no escalations;
- approval-presentation enforcement: proof enforced with the approval event
  marker present; and
- out-of-kernel work: repository edits, validation commands, disposable proof,
  git operations, and PR operations were performed by the maintainer/executor
  and are disclosed here rather than represented as kernel execution.

## 11. Remaining Limitations

- The closed authoritative path remains constrained to one immutable workflow
  step.
- The only accepted local-check profile is fixed project validation.
- Low-risk work may still encounter more ceremony than the proportional
  governance product target intends.
- Node 20 remains the validated integrations runtime; the Node 24 failure
  experience needs clearer handling.
- Pre-scaffold validation still has a duplicated missing-manifest diagnostic.
- OpenShell is not integrated.

## 12. Recommended Next Phase

Continue the proportional-governance and quiet-success lane by reducing
non-blocking ceremony on the now-default closed path while preserving durable
evidence, disclosure, and report posture. Keep OpenShell as a separately
planned optional execution provider.
