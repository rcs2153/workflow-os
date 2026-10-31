# First-Run Governed WorkReport Artifact Implementation Report

## 1. Executive Summary

Workflow OS now has one explicit opt-in path that turns the existing bounded
first-run repository posture analysis into a governed terminal `WorkReport` and
create-only local artifact:

```text
workflow-os first-run --write-report-artifact
```

Ordinary `workflow-os first-run` remains non-mutating. The new path does not
execute repository build or test commands, read arbitrary source, call a
provider, access the network, publish a report, or create an external side
effect.

## 2. Scope Completed

- Added the explicit `--write-report-artifact` first-run flag and optional
  `--run-id` for deterministic retry and recovery.
- Reused the existing bounded first-run analysis as one owned snapshot.
- Added a deterministic versioned commitment over the validated project
  identity, exact workflow/skill/policy/test hashes, and bounded analysis.
- Bound that commitment as the immutable run bundle's resolved execution
  context before any approval or skill execution.
- Admitted only the exact generated first-run workflow, skill, and policy
  fingerprints to the trusted local analysis handler.
- Routed the run through the existing authoritative project-validation and
  Core-derived proportional-governance path.
- Preserved the generated contract's two distinct gates: aggregate governance
  approval followed by the authored workflow-step approval.
- Required persisted approval-presentation proof and a fresh analysis
  commitment at each approval resume.
- Reran the canonical project-validation check at each granted decision and
  cited the exact decision-time result in the terminal report.
- Persisted the terminal report through the existing authoritative local
  artifact gates.
- Added exact terminal replay and commitment-gated missing-artifact recovery.
- Added bounded human and preview-JSON posture without rendering report
  contents by default.

## 3. Scope Explicitly Not Completed

This phase did not make default first-run mutating, execute package scripts or
repository checks, inspect arbitrary source contents, call CI or providers,
access the network, publish or export reports, activate workflow
recommendations, add provider mutations, integrate OpenShell, add hosted
behavior, add public schemas or SDK contracts, add nested harnesses, or change
release posture.

## 4. Runtime Composition

The implemented path is:

```text
validated exact generated contract
  -> bounded first-run analysis snapshot
  -> deterministic analysis commitment
  -> immutable run bundle
  -> authoritative project-validation attestation
  -> proportional-governance assessment
  -> proof-backed aggregate approval
  -> fresh analysis and check reassessment
  -> proof-backed authored-step approval
  -> fresh analysis and check reassessment
  -> trusted bounded first-run handler
  -> terminal truthful WorkReport
  -> existing authoritative artifact gates
  -> create-only local artifact
```

The separate approvals are intentional. The aggregate gate authorizes the
complete current governance assessment. It does not silently satisfy the
generated workflow step's independently declared human approval policy.

## 5. Immutable Analysis Identity

The analysis commitment uses domain
`workflow-os/first-run-analysis/v1` and labeled length-delimited fields in
stable order. It includes the manifest project and schema identity, manifest
hash, every loaded workflow/skill/policy/test identity and content hash, and
the deterministic bounded first-run JSON projection.

The commitment excludes paths, timestamps, run and approval IDs, environment
values, source contents, command output, provider data, and credentials. Exact
retry or approval resume with changed bounded metadata fails before approval,
handler invocation, or artifact persistence.

## 6. Contract Admission

The trusted handler is registered only by the explicit artifact command and
only when the generated workflow, skill, default approval policy, and local
policy match the current allowlisted canonical fingerprints. A customized or
widened contract is rejected. The ordinary `run` command still treats the
generated skill as a mockable placeholder unless another reviewed handler is
explicitly registered.

## 7. Checks, Policy, And Approval

The default generated scaffold declares the closed
`workflow_os_project_validation` local-check profile. The artifact path now
uses the established source-backed authoritative route rather than treating
static CLI validation as equivalent to the declared attested check. The check
result participates in the governance assessment and is rerun at each granted
approval boundary.

The generated `default/governed-work` approval policy and authored step gate
remain enforced. Approval presentation is persisted and proof-bound. A changed
analysis commitment, changed generated contract, failed check, missing or stale
presentation proof, changed governance binding, or denied policy fails closed.

## 8. Report And Artifact Truth

The report is generated only for a terminal completed run and from the same
owned bounded snapshot used by the trusted handler. It discloses:

- bounded repository and governance analysis performed;
- safe metadata and validated definitions considered;
- recommendation and proportional-governance counts;
- policy and approvals evaluated;
- authoritative project validation and skipped repository-specific checks;
- local durable state and artifact writes;
- absence of external side effects;
- incomplete work, limitations, risks, and operator handoff.

The validation section cites the exact decision-time local-check result. Mock
output is never report evidence.

## 9. Replay And Recovery

An exact terminal retry with an existing artifact returns
`already_persisted` without recomputing first-run analysis or rerunning the
skill. A terminal run missing its artifact may reconstruct the report only
when the current bounded analysis exactly reproduces the immutable commitment
and the durable run contains the authoritative governance binding. Drift fails
closed. Artifact writes remain create-only.

## 10. Tests Added

Focused coverage proves:

- resolved-context commitment publication and exact retry;
- changed commitment rejection;
- proof-enforced approval resume with exact context;
- explicit first-run start, aggregate approval, authored-step approval,
  completion, artifact persistence, and replay;
- missing-artifact recovery and recovery drift rejection;
- approval-time bounded metadata drift rejection;
- drifted grants remain blocked while proof-backed denial remains available;
- mock-mode rejection; and
- customized and legacy non-authoritative generated-contract rejection; and
- recovery reports cite the fresh canonical project-validation result.

## 11. Validation

Validation recorded through focused review:

- `cargo fmt --all --check`: passed.
- focused resolved-context and first-run artifact tests: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- CLI suite: 176 passed.
- local-executor suite: 345 passed, 1 ignored live test.
- pinned Node 20 `npm run check:docs`: passed.
- `git diff --check`: passed.
- disposable external-repository start, aggregate approval, authored-step
  approval, completion, artifact persistence, replay, and state-health proof:
  passed.

## 12. Governed Phase Record

- Dogfood workflow ID: `dg/implement`
- Run ID: `run-1793433611853605000-2`
- Approval ID:
  `approval/run-1793433611853605000-2/implementation-approved`
- Approval presentation ID: `presentation/93ad598de02f770c`
- Approval outcome: granted by delegated maintainer.
- Governed phase close: completed with 39 events, one proof-enforced approval,
  zero retries, and zero escalations.

The kernel governed scope and approval. The agent performed repository
inspection, implementation, local validation, documentation, and git work
outside the kernel under that approved scope.

## 13. Remaining Limitations

- The report artifact has no released rendering, export, or publication
  surface.
- Recovery reuses durable governance truth and commitment equality; it does not
  rerun the already completed skill.
- The exact allowlist is intentionally coupled to the current generated
  preview scaffold contract.
- Repository-specific build, test, lint, typecheck, and CI evidence remain
  unavailable until explicitly configured and executed through reviewed
  handlers.

## 14. Recommended Next Phase

Focused maintainer/security review is accepted after fixing denial behavior.
See [First-Run Governed WorkReport Artifact Review](FIRST_RUN_GOVERNED_WORK_REPORT_ARTIFACT_REVIEW.md).

Run a fresh governed queue-selection phase after merge. Do not infer a broader
execution or mutation authority from this acceptance.
