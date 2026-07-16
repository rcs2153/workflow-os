# Runtime Proportional-Governance Reassessment Helper Review

## 1. Executive Verdict

**Needs blocker fixes.**

The pure helper is narrow, deterministic, non-executing, and correctly rooted
in validated stored immutable definitions. However, its public runtime-fact
boundary omits explicit validated runtime-escalation posture required by the
accepted plan. The current helper therefore cannot reassess or invalidate on a
live runtime escalation that is not already represented by static workflow
declarations.

## 2. Scope Verification

The implementation stayed within the approved pure-helper scope. It added no
executor integration, durable binding, events, persistence changes, schema,
CLI, UI, provider calls, provider writes, automatic approvals, enterprise
administration, or default behavior.

## 3. Source-Of-Truth Assessment

Static definitions are resolved from `StoredImmutableRunBundle`; mutable
project files are not read. The helper requires exactly one fact record per
ordered step and rejects missing, duplicate, extra, or mismatched records.
Workflow, skill, and policy resolution is bounded and uses stable non-leaking
errors.

The shared resolved-definition derivation is preferable to duplicating
classifiers. Existing project-based derivation remains behaviorally compatible
and the full regression suite passes.

## 4. Blocker: Runtime Escalation Is Missing

`StepGovernanceRuntimeFacts` models authority, evidence/check posture,
`SideEffect` posture, prior decision axes, and steward minimum. It does not
model explicit validated runtime escalation.

The accepted plan identifies runtime escalation as a fact that only runtime can
prove. The shared derivation currently derives `runtime_escalation` only from
static workflow and step escalation-policy declarations. Consequently:

- a live runtime change cannot raise the reassessed posture;
- that change cannot alter the per-step or aggregate fingerprint;
- a later executor could incorrectly reuse an assessment after runtime
  escalation changed;
- the implementation does not yet satisfy its planned decision-relevant input
  boundary.

Fix: add an optional explicit runtime-escalation requirement to each exact step
fact record and compose it monotonically with the static declared escalation
minimum before assessment. Add tests proving quiet, visible, approval, and
denied escalation requirements can only hold or raise posture and always
invalidate the fingerprint when decision-relevant.

## 5. Determinism And Fingerprint Assessment

The aggregate fingerprint uses a new versioned domain, fixed-width length
framing, immutable bundle root, workflow/run identity, workflow-ordered steps,
assessment algorithm, and per-step fingerprints. The stable vector and framing
collision tests pass.

The focused tests prove runtime-fact changes invalidate and mutable project-file
changes cannot affect a stored bundle. Two planned definition-boundary tests are
still missing:

- a relevant immutable definition change alters the aggregate fingerprint;
- an unreferenced definition outside the bundle does not alter it.

These are required in the blocker fix because they prove the central
build-cache-style invalidation claim at the bundle boundary.

## 6. Privacy And Error Assessment

`Debug` output redacts workflow, run, step, and fingerprint values. Helper-owned
errors use stable codes and a fixed bounded message. The helper does not carry
raw definitions, evidence, check output, provider payloads, source content,
commands, parser output, environment values, or credentials.

The serialized result contains stable identity and assessment references by
design and remains sensitive operational metadata. No raw payload field was
introduced.

## 7. Freshness Assessment

The implementation report correctly states that supplied facts are
fingerprinted but not independently proven fresh. Trusted fact references,
validity windows, durable binding, and time-of-use reassessment remain future
phases. This limitation is honest and non-blocking for the pure helper.

## 8. Test And Validation Assessment

Passing validation:

- focused immutable-bundle reassessment tests;
- framing unit test;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

Opt-in live integration tests remained intentionally ignored. No skipped
required local test was identified.

## 9. Blockers

1. Add explicit runtime-escalation posture to the exact step fact boundary and
   compose it monotonically with static escalation declarations.
2. Add relevant-definition invalidation and unreferenced-definition stability
   tests at the immutable-bundle boundary.

## 10. Non-Blocking Follow-Ups

- Define trusted fact references and validity windows in the later durable
  binding phase.
- Decide whether later bindings store per-step fingerprints in addition to the
  aggregate fingerprint.
- Keep serialization of assessment sets classified as sensitive operational
  metadata when persistence is designed.

## 11. Recommended Next Phase

Perform a narrow blocker fix for runtime escalation and the two missing
definition-boundary tests. Do not begin durable binding or executor enforcement
until the fix passes focused re-review.

## 12. Governed Review Record

- Dogfood workflow: `dg/review`
- Run ID: `run-1784185348382815000-2`
- Approval ID: `approval/run-1784185348382815000-2/review-scope-approved`
- Approval outcome: granted with persisted presentation proof
- Validation summary: all required local checks passed
- Out-of-kernel work: code and test inspection, diff analysis, review drafting,
  documentation validation, and phase reporting

## 13. Fix-Forward Note

The blocker fix adds explicit step-bound runtime escalation, composes it
axis-by-axis with static escalation declarations, and adds the required
relevant-definition invalidation and unreferenced-definition stability tests.
The original blocker finding remains the review record until focused re-review
accepts the fix.
