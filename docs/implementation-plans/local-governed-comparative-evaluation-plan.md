# Local Governed Comparative Evaluation Plan

Status: strategic planning only. No evaluation runtime, public schema,
provider admission, or release gate is implemented or authorized here.

## Purpose And Boundary

Compare models and harnesses on identical real work, under the same declared
policies, with evidence retained in the operator's environment. Measure whether
the work was correct and whether the candidate respected scope, resolved current
authority, handled denial and uncertainty, and left another authorized executor
enough evidence to continue. A passing patch does not excuse an unauthorized
attempt, and an approval or a checkpoint does not establish that an effect ran.

Workflow OS structures experiments, binds evidence to each run, and derives
comparable observable facts. Deterministic checks and identified evaluators
judge results. External systems may consume exports; this is not a hosted
observability product, universal judge, scheduler, or replacement for existing
eval tools. Canonical contracts stay Rust-owned and versioned; SDKs may consume
them, not implement another governance engine.

## Experiment Identity

An evaluation family should commit to one experiment ID and immutable
baseline: exact repo and commit/tree, task intent and acceptance contract,
workflow and policy contents, tool schemas and permitted capabilities,
environment/toolchain and lockfiles, evidence supplied, and scoring plan with
evaluator identity. Record a separate comparison revision, trial ID, candidate
ID, and run ID. Lineage to the baseline means shared inputs, never shared
approval, execution state, or effects.

Each candidate descriptor binds provider, model, pinned revision if available,
inference parameters, harness/adapter versions, prompt commitments where
permitted, tool/capability posture, compaction configuration, checkout identity,
OS/architecture, dependencies, Workflow OS revision, workflow/policy revisions,
and evaluator/scorer versions. Explicitly distinguish mutable aliases and
unavailable facts from verified immutable inputs. Declare intended differences
before running; report incidental ones rather than silently treating arms as
equivalent.

Each terminal or explicitly unresolved trial has an evidence bundle bound to
its baseline, candidate, run and operation. Cite WorkReports, EvidenceReferences,
SideEffects and provider observations by their actual identity and assurance.
Reuse continuation checkpoint/envelope commitments; a serialized next action
is historical or advisory until re-resolved. Reports, signed envelopes, scores
and winning results never grant authority. An aggregate cites verified bundles
and scorer results without rewriting candidate truth.

## Isolation And Trust

Separate worktrees, writable dependencies and caches where they affect results,
state namespaces or databases/schemas, run/event/SideEffect identities,
idempotency keys, leases/windows, credentials, evidence/report paths and
temporary resources. Equalize initial cache posture where possible and
record deviations. No candidate can see another candidate's mutable work
unless the trial is explicitly classified and analyzed as collaboration.

For effect cases, use equivalent separate sandbox targets, or an explicitly
labeled simulation. A comparison can also inform selection of one future
effect, which requires fresh non-evaluation authority. Never reuse an approval,
target namespace or idempotency key across arms, or race candidates against a
production target. Keep reusable provider credentials out of candidate reach;
enforce the applicable tool, credential and egress boundaries outside its
control. No new write path or automatic promotion is in scope.

The local operator and storage owner are in the declared trust boundary.
Commitments detect identity mismatch against a trusted reference; they do not
prove an owner could not replace both data and reference. Distinguish local
observations, provider reports, and independently anchored evidence. Capture
observable inputs where policy permits, tool schemas, actions, checks, decisions,
outputs and references. Never collect private chain of thought. Minimize and
redact prompts and sensitive payloads at capture and export; define access,
retention, deletion, and when redaction limits verification. Do not export secrets
or reversible representations of them.

The contract review must specify attacker capabilities and controls for:

| Threat | Required review or test |
| --- | --- |
| Baseline substitution, hidden prompt/tool differences, stale aliases | Compare declared and observed commitments; mark unverifiable facts and incompatible pairs. |
| Cross-contamination, shared targets or idempotency | Deny cross-arm access; demonstrate isolated writable state, credentials and effect scope. |
| Mixed reports, wrong-run evidence, selective deletion | Verify identities against the preregistered trial inventory; surface missing, invalid and incomplete trials. |
| Scorer tampering, identity laundering, benchmark gaming | Attribute versioned independent inputs and results; keep evaluators read-only; track task leakage and selection. |
| Owner tampering, secret or prompt leakage | State what the verifier trusts, test redaction and retention, and distinguish local integrity from independent assurance. |
| Model output or a winning score treated as permission | Reject executable authority derived from any experiment artifact. Resolve ordinary authority anew. |
| Falsified telemetry, cache and order bias | Track metric producer and measurement boundary; disclose unknowns, randomized order and cache conditions. |

## Scoring And Study Design

Keep four layers separate. Never hide incomparable trials or authority failures
behind one composite ranking.

1. **Deterministic acceptance:** Tests, build/checks, schema and content
   commitments, policy and expected state transitions, duplicate-effect
   checks, approval/denial and recovery outcomes, report/evidence binding.
2. **Operational measurements:** Completion and time to correct terminal or
   explicitly unresolved posture; elapsed time, recovery time, turns including
   compaction/resume, tool calls, lookups, retries, approvals, human interventions,
   custom integration code, policy violations, denied and unauthorized
   attempts, duplicate accepted effects, preserved ambiguity, evidence
   completeness and successful independent reconstruction. Separate locally
   measured values from provider-reported input/output/cached tokens and cost;
   missing data is unknown, not zero.
3. **Human or model judgment:** Bind identity/version, rubric and exact allowed
   inputs. Attribute subjective judgments; retain disagreement and uncertainty.
   Prevent judge mutation of candidate evidence or authority. Model judgment
   is not independent assurance merely because a different model scored it.
4. **Authority and continuity:** Check correct approval requests, denial and
   scope adherence, fresh fact resolution, expiry/revocation, absence of
   inferred conversational permission, distinction between provider acceptance
   and kernel completion, no blind retry after ambiguity, sufficient handoff
   evidence, and correct next action after interruption or compaction.

Preregister paired versus unpaired analysis, sample count, order/randomization,
warm/cold cache rules, expected cut points, exclusions, and aggregation (e.g.,
median and range with counts). Timestamp runs, classify outages/rate limits
separately, and retain failed and incomplete trials. Change one factor when
attributing causality; compare model/harness combinations as combinations if
they cannot be separated. Track task exposure and leakage. If baseline, tools,
policy, environment, or acceptance changes break comparability, mark the pair
invalid and preserve evidence. Provider/model drift starts a new comparison
revision; never overwrite the old result or infer a definitive ranking from one
nondeterministic run.

After the paired-run MVP, preregister 10-20 real repository tasks with
deterministic acceptance, two candidate models or model/harness combinations,
and at least three repetitions per candidate/task where cost permits. Disclose
any reduced count. Include approval, denial, compaction/recovery and stale
summary versus current kernel facts. Add ambiguous real effects only after
Milestone 3 admits that test. Compare deterministic success, correct authority
and recovery decisions, unauthorized/duplicate accepted effects, evidence
sufficiency, time/turns, calls/lookups, provider tokens/cost, interventions
and independent reconstruction. Report qualitative review separately.

## Milestones And Exits

### 0. Contract And Threat Review

During or adjacent to the Authority Continuation Architecture Decision,
review identity and immutable lineage, isolation and effect boundaries,
privacy, threats, scorer provenance, repetition and drift, compatibility,
and the use of canonical evidence and continuation models. Exit requires
maintainer/security acceptance of the lineage/authority and fact/judgment
distinctions. No runtime required.

### 1. Paired-Run Local MVP

After public checkpoint/envelope and candidate commitments stabilize, use one
bounded real repository task and exactly two candidates, with equal declared
baseline, policies, evidence, tools and checks. No external writes. Deliver
versioned descriptors, isolated runs, rerunnable commands and fixtures, two
bound evidence bundles, and one side-by-side report. Exit requires identical
baseline proof, independently isolated writable state, reproducible
deterministic scores, attributable differences, and rejection of mixed,
missing or substituted evidence.

### 2. Authority And Recovery Corpus

Maintain the following preregistered cases. Use at least two model/harness
combinations where practical; effect cases here use fixtures.

1. Ordinary bounded success.
2. Required approval granted.
3. Explicit denial.
4. Approval pending across compaction.
5. Expired authority before resume.
6. Revoked authority before resume.
7. Task or target drift after approval.
8. Correctly delegated replacement.
9. Unauthorized replacement.
10. Completed work presented to a stale summary.
11. Corrupt or wrong-run event/evidence.
12. Duplicate completion attempt.
13. Interruption before effect.
14. Interruption after intent and before dispatch.
15. Provider acceptance before acknowledgement or persistence.
16. Insufficient observation requiring an unresolved outcome.
17. Report or evidence substitution.
18. Changed policy or check requirement before continuation.

Exit requires correct deterministic decisions at every registered authority
and fault boundary, no expanded authority, recovery cost alongside accuracy,
and explicit failed or unresolved trials.

### 3. Consequential-Effect Comparison

Use one existing accepted sandboxed action and reconciliation path with
equivalent, independent targets and idempotency namespaces. No reusable
production credentials reach a model. Exercise post-effect/pre-ack interruption
and reconcile without blindly retrying. Exit requires zero unauthorized or
duplicate accepted effects at the declared boundary, correct ambiguity
classification, independent reconstruction through public evidence surfaces,
and explicit provider, host and storage limitations.

### 4. Repeated Trials, Reporting And Export

Support N-trial paired summaries, preserved raw trials, local reports and
versioned machine-readable exports, with a documented external consumer
boundary. Exit requires independent verification of bundle identity and
reproduction of deterministic scores, clearly attributed subjective results,
redaction and compatibility tests, explicit rejection of unsupported versions,
and no private reasoning or reusable credentials in exports.

### 5. Regression Gates And Selection Support

Only after trustworthy comparisons, support optional repository CI thresholds,
auditable waivers, and explicit baseline-to-candidate drift assessment. Require
a human decision before changing selection for consequential work. Exit:
failures cannot be bypassed silently, waivers are recorded, and selection is
separate from execution admission. Automatic routing or delegator integration
is later: require freshness, fallback and no-eligible-candidate behavior,
policy compatibility and current-authority resolution.

## Canonical Dependencies And Review Record

- [Compact Recovery Checkpoint Evaluation](compact-recovery-checkpoint-evaluation-plan.md)
  owns the checkpoint-injection and framework-comparison protocols. Reuse its
  measurements and controls. Prior gross-token observations compare two arms
  with access to the same evidence and do not establish whole-task billed
  savings or a new study's expected scores.
- [Cross-Harness Authoritative Continuation](cross-harness-authoritative-continuation-milestone-plan.md)
  owns the live handoff milestone and its independent proof. Its existing
  synthetic evidence is not corpus or real-effect acceptance.
- [Durable Authority Continuation And Proof-Carrying Work](durable-authority-continuation-profile-plan.md)
  owns envelope, identity, transport and enforcement-plane contracts.
  Use comparative evaluation to measure later task-cell and generic-compute
  portability, cross-host recovery, and any later delegator work. Score
  Observe (visibility), Enforce (mediation on supported paths), and Strict
  (demonstrated bypass resistance at the declared credential/tool/egress
  boundary) separately; these are evaluation distinctions, not shipped modes.
  A cell cannot mint authority. Audit visibility does not prove enforcement.
- Reasoning Lineage may later enrich claim/evidence relationships; the baseline
  comparison does not depend on it.

Maintainer planning review: the addition is complementary to these plans.
The current product contract and V1 readiness criteria need no change: this
proposal does not alter active P0 order, waive an independent review, or add a
release blocker. Keep the full Milestone 0 security review separately governed.
This review is owner-conducted and is not independent assurance.

Planning run: `dg/d`, `run-1794723957033135000-2`; proof-backed delegated
approval `approval/run-1794723957033135000-2/planning-approved`. Repository
inspection, edits, docs checks and delivery are executor work outside the
preview handlers.
