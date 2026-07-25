# DocsCheck Attestation Consumer Integration Plan

Status: Implemented as one crate-private in-memory consumer. The
runtime-composition helper and its immutable-attribution blocker fix are
accepted. Focused plan review found blockers in freshness disposition and proof
reuse; the corrected plan resolved them, focused re-review accepted the fix,
and the bounded same-call gate is now implemented. See
[DocsCheck Attestation Consumer Integration Plan Review](../concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_PLAN_REVIEW.md).
The correction is documented in
[DocsCheck Attestation Consumer Integration Plan Blocker Fix Report](../concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_PLAN_BLOCKER_FIX_REPORT.md).
Focused re-review accepts the correction in
[DocsCheck Attestation Consumer Integration Plan Blocker Fix Review](../concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_PLAN_BLOCKER_FIX_REVIEW.md).
Implementation is documented in
[DocsCheck Attestation Consumer Integration Report](../concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_REPORT.md).

## 1. Executive Summary

Workflow OS can execute one explicit bounded `DocsCheck`, construct its
immutable execution binding before launch, observe the process inside Core,
verify the resulting payload-free attestation, and return accepted proof in
memory. Nothing currently consumes that proof as a gate result.

The first consumer should be one crate-private, explicit, in-memory wrapper that
executes the reviewed composition helper and immediately evaluates whether the
independent-check requirement is satisfied. It should return the structured
check result plus a typed satisfied or not-satisfied gate disposition. It must
not mutate a workflow run or silently turn proof presence into executor
authority.

No executor or automatic-check consumer is implemented.

## 2. Goals

- Give accepted proof its first explicit decision consumer.
- Keep check execution, verification, and consumption in one auditable call.
- Make `NoReuse` mean the exact current invocation, not reusable cached proof.
- Reevaluate maximum-age freshness at consumption time.
- Preserve honest failed and timed-out check results without manufacturing
  proof.
- Bind gate satisfaction to the exact requirement, immutable run, workflow,
  run, step, invocation, result, and handler selection already accepted.
- Return stable non-leaking failures for invalid or inconsistent posture.
- Prepare a typed input for later proportional-governance reassessment.

## 3. Non-Goals

This plan does not authorize:

- implementation during planning;
- executor integration or workflow state mutation;
- automatic local check execution;
- default handler registration or handler discovery;
- persisted, cached, serialized, or reusable accepted proof;
- events, audit records, evidence attachment, reports, or artifacts;
- mapping the result into proportional-governance enforcement in this phase;
- workflow or policy schema changes;
- CLI, UI, SDK, or example behavior;
- additional command families;
- raw stdout, stderr, command transcripts, paths, or source contents;
- provider calls, SideEffects, external writes, or network access;
- hosted runners, distributed execution, enterprise identity, or release
  changes.

## 4. Current Foundation And Gap

Accepted foundations:

- immutable run bundles with validated stored canonical definitions;
- immutable local-check execution binding;
- payload-free independent-check requirement and candidate models;
- crate-private Core-owned observation and verifier;
- read-only accepted attestation with exact fingerprints and identities;
- explicit `DocsCheck` runtime composition around the bounded local process
  runner; and
- honest no-proof results for statuses outside the accepted requirement.

The gap is consumption. `Option<AcceptedLocalCheckAttestation>` communicates
proof presence but does not state what a gate must do with absence, freshness,
or context at the decision boundary. A later caller could also delay use of a
maximum-age proof without reevaluating its age.

## 5. Selected First Consumer

Add one crate-private helper, tentatively:

```text
execute_docs_check_attestation_gate(input)
  -> Result<DocsCheckAttestationGateOutcome, WorkflowOsError>
```

The helper should call the existing
`execute_docs_check_with_attestation(...)` function and consume its result in
the same stack. It must not accept a separately supplied accepted attestation.
That restriction prevents this first boundary from becoming an implicit cache
or proof-import API.

The caller remains explicit and supplies the same validated execution input.
No global state, runtime configuration, handler discovery, or ambient default
is introduced.

## 6. Candidate Result Model

Use the smallest model that preserves the check result and gate meaning:

- `DocsCheckAttestationGateOutcome`
  - structured `LocalCheckResult`;
  - `DocsCheckAttestationGateDisposition`;
  - bounded proof fingerprint only when disposition is satisfied.
- `DocsCheckAttestationGateDisposition`
  - `Satisfied`;
  - `NotSatisfied(DocsCheckAttestationGateReason)`.
- `DocsCheckAttestationGateReason`
  - result status not accepted;
  - proof freshness expired.

Exact names may change during implementation. Do not add public serde merely to
stabilize speculative vocabulary. The first outcome should remain crate-private,
read-only, and in memory.

## 7. Satisfaction Semantics

`Satisfied` requires all of the following:

- the composition helper returned accepted proof;
- the proof requirement fingerprint equals the active requirement fingerprint;
- assurance meets the requirement;
- result identity and status match the structured result;
- the immutable bundle, workflow, run, step, and invocation match the exact
  current input;
- truncation posture remains allowed by the requirement; and
- freshness is valid at the consumption sample.

An accepted fingerprint is a commitment, not independent authenticity. The
consumer trusts only the verifier-returned private accepted type from the same
Core call. The first gate outcome does not return or expose that accepted type;
it exposes only the bounded proof fingerprint after consuming the proof.

## 8. Honest Not-Satisfied Semantics

A failed or timed-out structured result whose status is not accepted should
return `NotSatisfied`, preserve the bounded result, and carry no accepted proof.
This is an expected gate outcome, not an internal model error.

Proof absence for an otherwise accepted status must fail closed. It must not be
silently treated as optional, caller-asserted evidence, or quiet success.
Because the composition helper either returns proof or propagates verifier
failure for an accepted status, this remains an internal invariant failure and
does not require an artificial production seam solely for testing.

The disposition must not claim that a workflow failed or was denied because no
workflow consumer exists in this phase.

## 9. Freshness And Reuse

The wrapper should sample consumption time through the same injected Core-owned
clock after verification.

- `NoReuse` is eligible only for the exact invocation executed and consumed in
  the current wrapper call. The outcome exposes no accepted-proof accessor, so
  satisfaction cannot be imported into a later consumer through this API.
- `MaxAgeSeconds` compares consumption time with the accepted observation
  completion time. Expiry returns
  `NotSatisfied(FreshnessExpired)` and no proof commitment.
- Clock regression or impossible ordering is a structured error, not an
  ordinary failed check.

A future persisted consumer must define replay, one-time use, and concurrent
claim semantics separately. This phase must not imply them.

## 10. Identity And Substitution Boundary

The consumer must recheck the accepted proof against:

- active requirement fingerprint;
- exact stored immutable run binding;
- workflow and run identity;
- selected canonical step;
- current invocation ID;
- structured result ID/status; and
- handler selection commitment.

The wrapper should derive these values from its execution input and composition
outcome rather than accepting duplicate caller assertions. Mismatch is a stable
error and returns no satisfied gate.

## 11. Error Handling

Stable errors should distinguish:

- invalid input or underlying composition failure;
- accepted-status result without accepted proof;
- proof/context mismatch;
- consumption clock ordering failure; and
- impossible or regressing consumption time.

Freshness expiration is a typed not-satisfied gate result because the
requirement is not met but the runtime model remains valid. It must never
produce satisfaction.

Errors must not include IDs, hashes, paths, command details, output, source
content, environment values, credentials, or provider payloads.

## 12. Relationship To Proportional Governance

Execution disposition and disclosure remain independent axes. This check gate
does not select either axis.

A later reviewed adapter may map:

- satisfied accepted proof to `GovernanceWorkloadEvidenceCheckPosture::Satisfied`;
- a deterministic failed check to `Failed`; and
- required proof that is unavailable or stale to `RequiredUnavailable`.

That later mapping must trigger deterministic reassessment and may strengthen
governance. It must not weaken explicit workflow, policy, profile, authority,
SideEffect, or steward minimums. A UI may display the gate live without changing
its execution authority.

## 13. Runtime And Compatibility Posture

- `LocalExecutor::execute(...)` remains unchanged.
- Existing immutable-bundle and proportional-governance opt-in executor APIs
  remain unchanged.
- Existing explicit `DocsCheckLocalHandler` invocation remains unchanged.
- `LocalSkillRegistry::new()` remains empty.
- No run, snapshot, event history, state backend, report, or artifact is
  created or mutated by the consumer.

The helper is an explicit decision primitive for a future consumer, not an
automatic checkpoint.

## 14. Privacy And Redaction

The consumer may retain only typed status, bounded disposition/reason, and the
accepted proof fingerprint after satisfaction. It must not retain or expose the
accepted proof object, and it must not copy raw process output,
summaries, command arguments, executable paths, working directories, environment
values, source contents, tokens, or credentials.

Debug output should expose disposition, result status, proof presence, and
freshness policy only. Identity and fingerprint values remain redacted.

## 15. Future Test Plan

Implementation tests should prove:

1. passed current-invocation DocsCheck satisfies the gate;
2. failed and timed-out results are not satisfied and carry no proof;
3. accepted-status verifier failure propagates and cannot become satisfaction;
4. requirement mismatch fails without leakage;
5. immutable bundle, workflow, run, step, invocation, result, and handler
   substitution cannot satisfy the gate;
6. `NoReuse` accepts only the exact current wrapper invocation;
7. maximum-age proof is reevaluated at consumption time;
8. expired and future-dated consumption cannot satisfy the gate;
9. clock failure returns no partial satisfied outcome;
10. the process runs once and the verifier remains the only proof constructor;
11. Debug output is bounded and non-leaking;
12. no state, events, reports, artifacts, files, or CLI output are created;
13. existing local check, immutable bundle, attestation, executor,
    proportional-governance, provider, and workspace tests remain green.

## 16. Proposed Implementation Sequence

1. Review this plan.
2. Add the crate-private gate disposition/outcome model.
3. Add the explicit wrapper around the accepted composition helper.
4. Add focused satisfaction, no-proof, freshness, identity, ordering, and
   privacy tests.
5. Run full repository validation.
6. Perform phase-level maintainer review.
7. Only after acceptance, plan one explicit proportional-governance
   reassessment consumer or one opt-in executor checkpoint.

## 17. Open Questions

- Should the later runtime consumer map proof absence to
  `RequiredUnavailable` or preserve a more specific attestation posture first?
- Which opt-in executor checkpoint can consume this gate without changing
  legacy workflow semantics?

## 18. Final Recommendation

Implement the explicit in-memory `DocsCheck` attestation gate wrapper next.
It should consume proof only from the same current
execution call, expose only a proof commitment after satisfaction, preserve
honest no-proof results, and reevaluate freshness.

Do not add executor integration, automatic checks, default registration,
persistence, events, evidence, reports, artifacts, schemas, CLI behavior,
providers, SideEffects, writes, hosted behavior, or release changes in that
implementation.
