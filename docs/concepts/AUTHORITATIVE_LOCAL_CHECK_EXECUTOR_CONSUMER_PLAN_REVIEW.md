# Authoritative Local-Check Executor Consumer Plan Review

## 1. Executive Verdict

**Plan accepted after focused correction; proceed to the explicit
fresh-run-only executor consumer implementation.**

The plan connects accepted local-check, immutable-run, and proportional-
governance primitives into one narrow runtime path. It preserves the defining
product boundary: a fully checked, complete low-risk workflow may proceed
quietly, while every unsupported disclosure, approval, denial, incomplete-fact,
or replay posture fails closed before `RunCreated`.

Review found one planning blocker: the initial decision table inspected the
selected checked step rather than the complete multi-step assessment set. The
plan now requires aggregate `Proceed`, quiet disclosure, and complete facts.
It also clarifies that serialized source commitments are integrity-checked
data, not independent authenticity. Both corrections are sufficient at the
planning level.

## 2. Scope Verification

The plan remains limited to one future opt-in local executor path.

It does not authorize:

- default, automatic, background, parallel, or repository-wide checks;
- more than one selected check-bearing step;
- additional check families;
- retry, rehydration, approval resume, or cancellation on the new path;
- visible-disclosure continuation;
- proportional approval creation;
- reports, artifacts, evidence attachment, CLI, UI, schemas, SDK, or examples;
- providers, OpenShell, SideEffects, network access, or writes;
- hosted or distributed behavior;
- reasoning lineage;
- enterprise administration; or
- release changes.

The planned extension to the existing governance binding is not a new
standalone persistence system or event family. If it cannot remain a small,
backward-readable extension, the plan requires a separate prerequisite rather
than widening implementation.

## 3. Product Value Assessment

The selected slice is a meaningful runtime step rather than another advisory
projection.

Today:

```text
caller-supplied evidence/check posture
  -> proportional assessment binding
  -> record-only opt-in executor path
```

After the planned slice:

```text
canonical check declarations
  -> kernel-observed check
  -> complete authoritative fact
  -> fact-bound complete reassessment
  -> aggregate quiet-success gate
  -> local sequential execution
```

This directly addresses fresh-pull feedback that Workflow OS should reduce
ceremony for low-risk work while retaining the evidence trail. It does not
pretend to solve general repository automation or every governance
disposition.

## 4. Selected Integration Boundary Assessment

An additive free function beside
`execute_with_governance_assessment_binding(...)` is appropriate.

The proposed input reuses:

- `LocalExecutionWithImmutableRunBundleRequest`;
- `GovernanceStrictnessProfile`;
- exact per-step `StepGovernanceRuntimeFacts`; and
- one explicit `DocsCheckLocalHandler`.

It adds only one selected `StepId` and optional expected aggregate
fingerprint. It correctly excludes caller-supplied attestation requirements,
check identities, posture, detached facts, and prior bindings.

Core-owned derivation prevents callers from assembling an authoritative-
looking result from unrelated IDs or postures.

## 5. Fresh-Run Boundary Assessment

Fresh-run-only is the correct first limit.

The current private binding proves same-call composition. It is not replayable
attestation authority. Rerunning a process during retry or approval resume
would require separate decisions about:

- freshness;
- one-time claim semantics;
- environment and repository-state stability;
- repeated process side effects;
- expected binding equality; and
- failure behavior after a partial prior run.

Rejecting any existing run, bundle, binding, or event history before clock or
process use is safer than silently rehydrating or falling back to the existing
caller-fact path.

## 6. Preflight And Ordering Assessment

The plan requires complete deterministic preflight before clock or process
use:

- execution plan and immutable identity;
- selected step and canonical declarations;
- command and handler commitments;
- complete per-step runtime facts;
- absent selected caller posture; and
- existing reassessment and composition prerequisites.

The required public error-precedence regression is important. An invalid
immutable context must win over a process failure that would occur later.
Otherwise error behavior could vary with local tooling or timing.

Persisting the immutable bundle before process use is acceptable only after
all pure preflight succeeds. A later check or binding failure may leave an
immutable bundle without run events; the implementation report must disclose
that bounded create-only residue and must not claim rollback.

## 7. Authority Continuity Assessment

The plan correctly rejects the tempting shortcut:

```text
bound_assessment.local_check_posture()
  -> new caller-shaped assessment request
```

That would detach posture from the fact fingerprint.

The planned private consumption method instead owns the complete assessment
set and local-check fact while deriving:

- aggregate execution, disclosure, and completeness;
- the ordinary durable assessment binding; and
- an authoritative source commitment.

The raw fact and assessment set remain private and non-serializable.

## 8. Aggregate Decision Correction

The initial plan considered the selected step's execution and disclosure axes.
That was unsafe for a multi-step workflow.

Example:

```text
selected checked step: Proceed + quiet
another step: ApprovalRequired + visible
```

Executing from the selected step alone would bypass the stricter workflow
posture.

The corrected plan now requires the complete assessment set to be:

- execution: `Proceed`;
- disclosure: quiet; and
- completeness: complete.

Any visible, approval-required, denied, or incomplete aggregate posture fails
before `RunCreated`. The selected assessment must still be present and
fact-derived, but it is not standalone workflow authority.

## 9. Durable Source Commitment Assessment

The ordinary `GovernanceAssessmentBinding` commits to the assessment set but
not to the authoritative runtime fact that supplied one axis.

The proposed optional source commitment is justified because the executor
would otherwise persist an assessment while discarding the proof-of-origin
relationship that made the selected check posture trustworthy.

The implementation must preserve these distinctions:

- a source fingerprint is a commitment, not cryptographic authenticity;
- a well-formed deserialized value is not proof that a process ran;
- runtime authority exists only when the exact create-only stored binding
  matches the current same-call private result;
- old bindings without a source commitment remain readable; and
- event/audit output discloses bounded source kind and presence, not raw
  fingerprints or IDs.

If a safe constructor and serde boundary require broad public redesign, the
source-binding model must become a prerequisite phase. The executor must not
fall back to detached posture.

## 10. Runtime Semantics Assessment

The quiet-only decision table is conservative and product-relevant:

- complete aggregate `Proceed` plus quiet disclosure may execute;
- visible disclosure fails because no durable presentation obligation exists
  yet;
- approval-required fails because this path cannot create a proportional
  approval request yet;
- denied fails; and
- incomplete facts fail.

This is not a permanent runtime matrix. It is the smallest cell that can prove
quiet success without swallowing governance obligations.

## 11. Failure And Privacy Assessment

The planned failures are specific, stable, and non-leaking. There is no
fallback to:

- caller posture;
- the existing unbound or caller-fact executor path;
- stale results;
- best-effort continuation; or
- hidden approval.

The plan keeps stdout, stderr, command lines, repository paths, source content,
environment values, credentials, provider payloads, raw facts, and raw
assessments outside state, events, errors, and debug output.

## 12. Compatibility Assessment

The plan preserves existing executor APIs and defaults.

The source-binding compatibility requirement needs careful implementation:

- old serialized bindings must remain readable;
- new source-bound records should use an explicit compatible version posture;
- old readers are not promised to understand a new version;
- unknown source kinds must fail closed with static errors; and
- existing binding equality and create-only behavior must include source
  commitment presence and value.

No workflow schema or CLI contract changes are needed.

## 13. Test Plan Assessment

The test plan covers:

- fresh multi-step quiet execution;
- sequential step ordering;
- canonical check order;
- Core-owned identity derivation;
- caller-posture rejection;
- preflight-before-process ordering;
- public error precedence;
- monotonic governance;
- aggregate cross-step strictness;
- failed and unavailable check behavior;
- visible, approval, denial, and incomplete failures;
- existing-state rejection;
- source-binding retention and substitution;
- old-binding compatibility;
- serialization non-authenticity;
- event/audit redaction;
- no report, provider, SideEffect, write, or CLI behavior; and
- workspace regression coverage.

This is adequate for implementation. Add a stable source-binding serialization
vector if a new binding version or fingerprint algorithm is introduced.

## 14. Documentation Assessment

The implementation plan and roadmap now state:

- the private binding exists;
- the executor consumer is planned, not implemented;
- the first path is explicit and fresh-run-only;
- only aggregate complete quiet `Proceed` may execute;
- visible, approval-required, denied, and incomplete postures fail closed;
- a detached posture shortcut is forbidden;
- default and automatic checks remain unsupported; and
- providers, OpenShell, SideEffects, writes, schemas, hosted behavior, and
  release changes remain outside the phase.

## 15. Planning Blockers

None after the aggregate-decision and source-authenticity corrections.

## 16. Non-Blocking Follow-Ups

- Decide during implementation whether the source commitment fits safely in a
  backward-readable binding version or needs a focused prerequisite.
- Add a stable source-binding serialization/fingerprint vector if applicable.
- Document immutable-bundle residue if a check fails after bundle persistence
  but before `RunCreated`.
- Keep Node 20 as the documented integration-check environment until broader
  Node compatibility is intentionally tested; the opaque Node 24 evaluator
  failure is already fixed at the tooling boundary.
- Preserve the existing duplicate missing-manifest diagnostic regression fix.

## 17. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785023908935444000-2`
- approval:
  `approval/run-1785023908935444000-2/planning-approved`
- presentation: `presentation/508b6007b2411d2e`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: source inspection, roadmap and plan authoring, shell
  validation, and review analysis
- missing coverage: the kernel coordinated governance only; it did not edit
  files, execute engineering checks, create a WorkReport artifact, or invoke
  providers

## 18. Governed Review Record

- workflow: `dg/review`
- run: `run-1785024173921721000-2`
- approval:
  `approval/run-1785024173921721000-2/review-scope-approved`
- presentation: `presentation/97204098cf505789`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- validation: documentation and diff checks are required before phase close
- out-of-kernel work: source inspection, review authoring, correction of the
  aggregate decision table, and validation commands
- missing coverage: the kernel coordinated governance only; it did not perform
  review reasoning, edit files, run implementation tests, or create a
  persisted WorkReport artifact

## 19. Recommended Next Phase

Implement the explicit fresh-run-only authoritative `DocsCheck` executor
consumer.

The implementation may include the minimal backward-readable source-binding
extension only if it remains tightly scoped. If that boundary requires a broad
public model or persistence redesign, stop and implement the source-binding
model as a focused prerequisite.

Do not implement retry, approval resume, visible-disclosure continuation,
proportional approval creation, automatic checks, additional check families,
reports, artifacts, evidence attachment, CLI behavior, schemas, providers,
OpenShell, SideEffects, writes, hosted behavior, reasoning lineage, enterprise
administration, or release changes.
