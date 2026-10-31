# First-Run Governed WorkReport Artifact Review

## 1. Review Disposition

Accepted after one required fix.

The explicit `workflow-os first-run --write-report-artifact` path now
provides one bounded, local, governed WorkReport artifact for the exact
authoritative generated first-run scaffold. Ordinary `workflow-os first-run`
remains non-mutating.

The review found no remaining blocker in immutable analysis identity, exact
contract admission, authoritative project validation, proportional-governance
routing, approval separation, presentation proof, same-snapshot report truth,
local-check citation, create-only persistence, replay, recovery, privacy, or
scope containment.

## 2. Required Finding And Fix

### Denial incorrectly depended on current repository analysis

The implementation originally recomputed and compared the current bounded
first-run analysis before both approval grants and denials. Grant behavior was
correct, but the shared preflight could prevent an operator from denying a
stale pending run after repository metadata changed.

That contradicted the accepted plan: denial must fail closed, retain the
durable denial trail, invoke no check or handler, and write no success artifact.

The fix routes denial through the existing proof-enforced,
resolved-context-bound approval path using only the immutable stored context.
It does not load the current project, rerun first-run analysis, invoke the
project-validation check, register or call the trusted handler, or write a
report artifact. Denial still requires an explicit reason, matching persisted
approval-presentation proof, and the exact immutable context commitment.

Focused CLI coverage proves that a drifted grant is rejected while a subsequent
proof-backed denial of the same pending run succeeds, leaves the run failed,
and writes no report artifact.

## 3. Immutable Analysis And Contract Admission

Accepted:

- the bounded analysis commitment is computed before run creation;
- the commitment is the immutable run bundle's resolved execution context;
- approval grants recompute the bounded analysis and require exact equality
  before check reassessment or execution;
- artifact persistence performs a final current-input commitment comparison;
- only the canonical authoritative generated workflow, skill, and policies are
  admitted;
- the legacy non-authoritative scaffold, customized contracts, and mock mode
  are rejected before run state is created; and
- ordinary run and first-run behavior remain unchanged unless the explicit
  artifact flag is selected.

## 4. Check, Policy, And Approval Enforcement

Accepted:

- the declared closed project-validation profile executes through the existing
  Core source-backed authoritative route;
- project validation participates in the proportional-governance assessment;
- the aggregate governance approval and authored workflow-step approval remain
  separate gates;
- each grant requires durable approval-presentation proof and fresh canonical
  project-validation evidence;
- analysis, immutable bundle, governance, definition, activation, handler, or
  check drift fails before approval events or skill execution; and
- denial remains source-free and artifact-free after the required fix.

No declared policy or check in this path is merely disclosed.

## 5. Report And Artifact Truth

Accepted:

- one owned bounded snapshot feeds the trusted handler, terminal report, and
  final commitment check;
- the report cites the exact decision-time local-check result;
- report sections disclose skipped repository-specific commands, unavailable
  evidence, no external side effects, limitations, risks, and handoff;
- mock output cannot enter report evidence;
- the report is generated only after a completed governed run;
- existing proof-marker and workflow-derived artifact gates remain active; and
- persistence is local, create-only, and idempotent.

## 6. Replay And Recovery

Accepted:

- exact terminal replay returns the existing equal artifact without rerunning
  analysis, checks, or the workflow skill;
- a terminal run missing its artifact must reproduce the immutable bounded
  commitment and durable governance binding;
- recovery reruns only the canonical project-validation check;
- the reconstructed report cites that fresh same-call check result;
- recovery never reruns the completed workflow skill; and
- changed bounded metadata or immutable request material fails closed.

## 7. Privacy And Scope Review

The path does not read arbitrary source contents, execute repository build,
test, lint, or typecheck commands, access the network, load credentials, call a
provider, publish a report, activate recommendations, create an external
SideEffect, integrate OpenShell, add hosted behavior, or broaden provider
mutations.

Human and JSON output remain bounded to identities, posture, references, and
next actions. Report contents are not printed by default.

## 8. Validation

Post-remediation validation:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- CLI suite: 176 passed;
- local-executor suite: 345 passed, 1 ignored live test;
- focused first-run artifact, denial, resolved-context, replay, recovery, and
  drift tests: passed;
- pinned Node 20 `npm run check:docs`: passed;
- `git diff --check`: passed; and
- disposable external Git repository acceptance: authoritative scaffold,
  aggregate approval, authored-step approval, completion, persisted artifact,
  exact replay, and healthy state passed.

## 9. Governed Review Record

- workflow ID: `dg/review`;
- run ID: `run-1793436169167982000-2`;
- approval ID:
  `approval/run-1793436169167982000-2/review-scope-approved`;
- approval presentation ID: `presentation/49149238eb59a47f`;
- approval presentation content hash:
  `49149238eb59a47fee9fdc96e2012b85b473bdf1d3090118261a0bb826a99108`;
- approval outcome: granted under standing delegated-maintainer authority for
  the exact review scope;
- review disposition: accepted after the denial fix; and
- governed close: completed with 39 events, one proof-enforced approval, zero
  retries, and zero escalations.

The kernel governed scope, approval proof, and phase history. Repository review,
edits, commands, validation, documentation, Git, and PR work were performed
outside the kernel under that approved scope.

## 10. Remaining Boundaries

This acceptance does not authorize default mutation, automatic approval,
arbitrary local execution, provider or network work, report publication,
hosted behavior, public schema or SDK commitments, OpenShell integration,
nested harnesses, or release changes.

The exact contract allowlist remains intentionally coupled to the current
preview scaffold and requires reviewed updates for future scaffold versions.

## 11. Next Step

Run a fresh governed queue-selection phase after this review merges. Selection
must use current runtime and product evidence, avoid reopening completed
primitive families, and prefer the next bounded composition that reduces user
ceremony without weakening evidence, policy, approval, or report truth.

