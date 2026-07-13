# Proportional Governance Workflow Declaration Derivation Report

## 1. Executive Summary

Workflow OS now has a pure core helper that derives one proportional-governance
workload-assessment input from an already-loaded, validated workflow step. It
resolves the step's skill and referenced policies, classifies bounded static
facts, and produces a payload-free relevant-definition root for deterministic
reassessment.

This is the first implementation response to onboarding feedback that ordinary
users should not construct every proportional-governance decision input by
hand. It is not yet first-run integration or automatic configuration. Runtime
authority, executed check/evidence results, and write reversibility remain
explicit unknowns unless a caller supplies typed facts.

Execution and disclosure remain independent. Required visible disclosure is a
governance obligation that a future UI may render; it is not a separate
blocking execution mode.

## 2. Scope Completed

This phase added:

- `WorkflowStepGovernanceDerivationRequest`;
- `derive_workflow_step_governance_assessment_input`;
- shared internal normalization of declared capability names;
- deterministic workflow, step, skill, and policy resolution;
- action-class derivation from validated skill capabilities;
- sensitivity derivation from declared approval sensitivity and secret access;
- workflow and policy minimum derivation from existing declarations;
- explicit runtime-escalation disclosure derivation;
- conservative SideEffect derivation with contradiction rejection;
- a versioned relevant-definition root over the workflow, step, skill, and
  referenced policies;
- focused determinism, invalidation, unknown-fact, and privacy tests.

## 3. Derivation Boundary

The helper consumes an existing `ProjectBundle` and revalidates it before
derivation. It resolves exactly one workflow step and derives:

| Declaration | Assessment fact |
| --- | --- |
| local or external read capability | read-only action |
| local write capability | local mutation |
| unsupported capability | unsupported action |
| low approval sensitivity | routine sensitivity |
| medium approval sensitivity | elevated sensitivity |
| high sensitivity or secret-read capability | restricted sensitivity |
| workflow approval/autonomy declaration | workflow minimum |
| step approval or `require_approval` policy | policy minimum |
| escalation declaration | required visible disclosure |
| no mutation capability | no SideEffect |
| mutation capability without supplied reversibility | unknown SideEffect |

Unknown facts are not guessed. Authority, executed evidence/check results, and
write reversibility are not proven by static workflow YAML, so they default to
typed unknown posture. A caller may supply compatible bounded runtime facts;
contradictory SideEffect facts fail with a stable non-leaking error.

## 4. Invalidation Boundary

The definition root uses the versioned domain
`workflow-os/proportional-governance-workflow-step-derivation/v1` and fixed
eight-byte big-endian field framing. It includes:

- loaded workflow content hash;
- selected step ID;
- resolved skill content hash;
- sorted IDs and content hashes for policies referenced by that step or by the
  workflow's retry and escalation declarations.

Relevant workflow, skill, step-level policy, or workflow-level retry/escalation
policy changes therefore force reassessment.
Unrelated policy changes do not churn the selected step's definition root.
The root is not an authorization grant, immutable run bundle, persistence key,
or executable replay proof.

## 5. Recommendation Versus Authority

The helper derives review-only input for the already accepted workload
assessment. It does not approve, deny, execute, or persist anything. Explicit
workflow, policy, profile, steward, prior-decision, and caller-supplied typed
minima remain authoritative. Derived facts may raise posture through the
assessment model but may not weaken those minima.

## 6. Privacy And Redaction

The helper reads no files and accepts no source contents, prompts, command
output, provider payloads, environment values, credentials, tokens, or
unbounded natural language. Debug output redacts project, workflow, and step
identity. Errors use stable codes and do not include caller-supplied IDs or
definition material.

## 7. Scope Explicitly Not Completed

This phase does not add:

- filesystem or repository scanning;
- first-run, CLI, executor, or runtime integration;
- YAML or public schema fields;
- automatic workflow configuration or approval;
- persistence, events, audit records, or report artifacts;
- a UI or disclosure presentation surface;
- provider calls, GitHub PR creation, Jira issue creation, or other mutations;
- model inference or model authority;
- hosted administration, identity, RBAC, or release changes.

## 8. Test Coverage

Focused tests cover:

- read-only declaration derivation;
- local mutation and restricted-sensitivity derivation;
- explicit unknown authority and check posture;
- compatible caller-supplied runtime facts;
- contradictory SideEffect rejection with stable non-leaking error;
- relevant-definition invalidation;
- workflow-level retry and escalation policy invalidation;
- unrelated-policy stability;
- bounded unresolved-identity errors and redaction-safe Debug.

## 9. Validation

Validation completed during implementation:

- `cargo test -p workflow-core --test proportional_governance_workflow_derivation`:
  passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 10. Governed Phase Evidence

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783932713540934000-2`.
- Approval ID:
  `approval/run-1783932713540934000-2/implementation-approved`.
- Approval presentation: `presentation/bfe8cc6fc941aaa0`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
- Out-of-kernel work: Codex implemented and tested the helper; the kernel
  coordinated governance only.
- Report posture: this document is repository phase evidence, not a generated
  or persisted runtime WorkReport artifact.

## 11. Remaining Limitations

- No user-facing path invokes the helper.
- Safe repository metadata outside current Workflow OS declarations is not
  inspected.
- Authority, evidence/check results, and reversibility require separate typed
  facts.
- No stored assessment is automatically invalidated or recomputed.
- No presentation surface renders quiet or required-visible disclosure.
- External write declarations remain outside the validated v0 workflow path.

## 12. Recommended Next Phase

Perform a focused maintainer review of the workflow-declaration derivation
boundary. If accepted, integrate it into one explicit read-only first-run
recommendation path using already accepted safe metadata. Do not add runtime
enforcement, persistence, automatic approval, schema changes, or broader
provider mutations in that phase.

## 13. Blocker Fix

Initial focused review found that the definition root omitted workflow-level
retry and escalation policy definitions. The fix extends the same deterministic
deduplicated policy resolution used for step policies to those workflow-level
references. Focused tests mutate each referenced workflow policy independently
and prove the root changes, while the unrelated-policy control remains stable.

Blocker-fix governed evidence:

- Dogfood workflow: `dg/blocker`.
- Run ID: `run-1783936076097131000-2`.
- Approval ID: `approval/run-1783936076097131000-2/fix-approved`.
- Approval presentation: `presentation/056ad0408d32fd60`.
- Approval outcome: granted with persisted presentation proof.
- Phase status: completed.
