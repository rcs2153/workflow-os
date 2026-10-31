# First-Run Quiet-Success Artifact Composition Plan Review

## 1. Disposition

Accepted after two required corrections.

The corrected plan preserves the existing approval/audit demo, defines a
separate exact low-ceremony contract, keeps Core authoritative for routing, and
does not confuse command invocation with approval.

## 2. Finding: Demo Skill Sensitivity Was Reused

The initial plan reused `local/first-run-report`. That skill is intentionally
medium-sensitivity for the approval demo. Reusing it would either keep the new
path approval-required or pressure implementation to override a validated
sensitivity declaration.

The correction adds a separate exact `local/first-run-report-artifact` skill
with low sensitivity limited to bounded first-run analysis and one create-only
local report artifact. The demo skill and workflow remain unchanged. Exact
contract hashes prevent the low-sensitivity skill from becoming ambient local
execution authority.

## 3. Finding: Local Request Authority Was Undefined

The initial plan required current authority but did not identify its source.
Absence of an external capability cannot be interpreted as permission for a
filesystem mutation, and explicit command invocation is not approval.

The correction defines a CLI-private, one-use explicit local artifact request
whose commitment is bound into immutable run context before state creation. It
is scoped to actor, project, workflow, run, analysis, artifact-store root, and
operation version. It is not serializable bearer authority, approval, policy,
reusable capability, credential, or authority receipt, and it cannot satisfy a
higher minimum. Cross-project, cross-run, output-root, and replay substitutions
join the required negative matrix.

## 4. Policy And Routing Review

Accepted:

- the new policy uses the existing enforced `allow_local` effect;
- route selection remains Core-owned;
- unknown safety-relevant facts cannot produce `Proceed`;
- explicit policy, profile, workflow, sensitivity, authority, check, and future
  steward minima remain monotonic;
- quiet and visible proceed routes create no approval records or proof markers;
- approval escalation uses the existing proof-backed fresh reassessment path;
  and
- denial remains handler-free and artifact-free.

## 5. Compatibility And Recovery Review

Accepted:

- new scaffold files are additive and conflict-refusing;
- missing files produce an upgrade diagnostic before state creation;
- current demo behavior remains unchanged;
- durable legacy runs route by immutable identity rather than current defaults;
- replay performs no work; and
- recovery preserves the authorizing route and cannot invent approval proof.

## 6. Scope Review

The plan does not authorize arbitrary local execution, repository commands,
source inspection, schema or SDK changes, providers, network, credentials,
external SideEffects, OpenShell, hosted behavior, enterprise identity, nested
harnesses, or release changes.

## 7. Validation

Validation passed:

- pinned Node 20 `npm run check:docs`;
- `git diff --check`; and
- governed review close with `Completed`, 39 events, one proof-enforced
  approval, zero retries, and zero escalations.

Rust checks are intentionally deferred because this milestone changes planning
and review documentation only. No runtime behavior was simulated.

## 8. Governed Review Record

- workflow: `dg/review`;
- run: `run-1793438205891382000-2`;
- approval: `approval/run-1793438205891382000-2/review-scope-approved`;
- presentation: `presentation/001cc377d5cc2b2b`;
- decision: granted under delegated-maintainer authority; and
- kernel boundary: review scope, proof-enforced approval, and durable history.

## 9. Next Step

After merge, implement the accepted plan as one bounded phase. Focused runtime
review remains required before the quiet-success composition is accepted.
