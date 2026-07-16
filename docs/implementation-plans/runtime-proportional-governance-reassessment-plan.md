# Runtime Proportional-Governance Reassessment Plan

Status: The first pure immutable-bundle reassessment helper is implemented and
accepted after a focused runtime-escalation blocker fix. Durable fingerprint
binding, events, executor enforcement, retry/resume reassessment, schema, CLI,
and UI behavior are not implemented.

## 1. Executive Summary

Workflow OS already separates execution disposition from disclosure obligation
and can derive review-only governance assessments from validated workflow,
skill, policy, repository, and explicit runtime facts. Those assessments carry
a versioned payload-free input fingerprint.

The remaining product gap is runtime use of that boundary. A local run can
currently begin without proving that its proportional-governance assessment was
derived from the immutable definitions and current facts that will govern the
run. A changed relevant input does not automatically invalidate a prior
assessment at execution or resume.

This plan defines a conservative path to one explicit opt-in local executor
integration. It begins with a pure helper over a validated stored immutable run
bundle, then binds accepted per-step assessment fingerprints before
`RunCreated`. Changed, missing, or unsupported decision inputs fail closed.

The first implementation remains review-only and does not implement runtime
enforcement.

## 2. Product Decision From External Feedback

The external feedback is correct about the desired product behavior and is
already reflected in the accepted model:

- visible disclosure is not an execution mode;
- a local UI may display quiet decisions without changing their authority;
- ordinary users should not construct the full decision input manually;
- deterministic inference should derive common posture from validated facts;
- explicit profile, policy, authority, evidence/check, sensitivity,
  `SideEffect`, prior-decision, and steward minima may only hold or raise the
  result;
- relevant changes should invalidate the prior assessment like a build cache.

The unresolved work is automatic runtime composition and invalidation, not a
redesign of the decision axes.

## 3. Goals

- Reassess every selected workflow step from immutable run definitions.
- Use explicit typed runtime facts instead of hidden global state.
- Reuse the accepted workload derivation and assessment selectors.
- Bind each accepted result to its algorithm and input fingerprint.
- Reject stale or changed fingerprints before execution or resume.
- Preserve quiet success when execution may proceed and no visible disclosure
  is required.
- Preserve visible disclosure independently from blocking behavior.
- Preserve all explicit governance minima and monotonic escalation.
- Keep the first executor integration local, additive, and opt-in.
- Produce stable, non-leaking failure codes and inspectable decision posture.

## 4. Non-Goals

- No implementation in this planning phase.
- No global default executor behavior.
- No workflow or policy YAML schema changes.
- No model-based or probabilistic authority decision.
- No silent weakening of declared requirements.
- No automatic approvals.
- No UI server or operator dashboard.
- No provider calls, connectors, or provider writes.
- No new mutation family.
- No raw source, provider, command, parser, or environment payload capture.
- No persistence redesign or hosted backend.
- No enterprise RBAC, IdP, or steward administration.
- No reasoning lineage or authority receipt implementation.

## 5. Source-Of-Truth Boundaries

Runtime reassessment must distinguish four sources:

1. **Immutable definitions.** Workflow, resolved skill, and referenced policy
   definitions come from a validated `StoredImmutableRunBundle`, not mutable
   project files.
2. **Current runtime facts.** Authority, evidence/check result, `SideEffect`
   reversibility, runtime escalation, and prior accepted posture are explicit
   typed inputs evaluated for the exact run and step.
3. **Deterministic assessment.** The accepted derivation and selector compute
   execution, disclosure, reasons, completeness, algorithm, and fingerprint.
4. **Durable binding.** The executor path eventually records which assessment
   fingerprint governed each step before the run is created or resumed.

Repository metadata and natural-language agent judgment may recommend posture
during onboarding. They are not runtime authority sources.

## 6. Candidate Core Types

The first implementation should add only the smallest justified types, likely:

- `ImmutableBundleGovernanceAssessmentRequest`
- `StepGovernanceRuntimeFacts`
- `ImmutableBundleStepGovernanceAssessment`
- `ImmutableBundleGovernanceAssessmentSet`
- `GovernanceAssessmentSetFingerprint`

The request should carry:

- validated stored immutable run bundle;
- active governance profile;
- exactly one runtime-fact record for every ordered workflow step;
- optional prior accepted execution and disclosure posture;
- optional steward minimum.

The result should carry:

- workflow, run, and step references;
- assessment algorithm;
- execution disposition;
- disclosure obligation;
- completeness and unknown fact categories;
- per-step input fingerprint;
- deterministic aggregate assessment-set fingerprint;
- explicit `assessed_not_executed` posture during the pure-helper phase.

Do not store raw definitions twice, raw evidence, check output, provider
payloads, or free-form decision explanations.

## 7. Pure Immutable-Bundle Derivation

The first code phase should be a pure helper only.

It should:

1. validate the stored bundle and manifest/record integrity;
2. resolve exactly one workflow record;
3. resolve each ordered workflow step to exactly one canonical skill record;
4. resolve referenced canonical policy records;
5. derive static workload facts from those immutable definitions;
6. combine explicit current runtime facts;
7. invoke the accepted workload assessment;
8. sort step results in workflow order;
9. compute a versioned aggregate fingerprint over bundle root, workflow/run
   identity, ordered step IDs, algorithms, and per-step fingerprints.

Missing, ambiguous, extra, or mismatched definition records must fail closed.
Missing, duplicate, extra, or step-mismatched runtime-fact records must also
fail closed. The helper must not silently reuse one step's facts for another
step.
Mutable project files must not be read by this helper.

## 8. Runtime-Fact Posture

The helper should derive most ordinary posture from immutable declarations, but
it must not invent facts that only runtime can prove.

Explicit runtime facts remain required for:

- actor/capability authority resolution;
- executed evidence and check posture;
- actual or proposed `SideEffect` reversibility;
- validated runtime escalation;
- prior accepted decision minima;
- strict-enterprise steward minimum.

Unknown facts remain typed unknowns. The accepted selector determines whether
they require approval or denial. Inference may never convert unknown authority
or evidence into quiet success.

Future capability integration may derive authority posture from fresh exact
step-scoped capability resolutions. Capability visibility alone is not enough.

The first pure helper can validate and fingerprint only the facts supplied to
it. It does not independently prove that those facts are fresh. Trusted fact
references, validity windows, and time-of-use reassessment belong to the later
durable binding and executor phases and must not be claimed by the helper.

## 9. Fingerprint And Invalidation Policy

The build-cache analogy should become an enforced invariant:

- identical immutable definitions, current facts, explicit minima, prior
  posture, and algorithm produce the same fingerprint;
- any decision-relevant change produces a different fingerprint;
- unrelated definitions outside the immutable bundle do not cause churn;
- a supplied expected fingerprint that differs from reassessment fails closed;
- missing expected fingerprints fail closed on retry/resume once the opt-in path
  has established a bound assessment;
- no fallback may silently reuse an older assessment.

The aggregate fingerprint must use a new versioned domain separator and the
accepted fixed-width length framing used by existing governance fingerprints.
Its tests must include a stable known vector and delimiter-collision cases. It
must not concatenate variable-length values with ambiguous separators.

Stable failure codes should distinguish bundle inconsistency, missing runtime
facts, stale fingerprints, unsupported algorithms, missing durable bindings,
and changed retry or resume context. Errors must not echo definition content,
IDs, hashes, paths, or sensitive facts.

## 10. Execution And Disclosure Semantics

The executor must consume the axes independently:

| Execution | Disclosure | Runtime posture |
| --- | --- | --- |
| proceed | quiet | continue without interruption; retain required evidence, audit, and report posture |
| proceed | visible | continue without approval; retain an operator-visible disclosure obligation |
| approval required | visible | pause before the governed action and present complete approval context |
| denied | visible | fail closed before the governed action |

A UI preference may display quiet decisions live. That is presentation, not a
governance escalation. A required visible disclosure may not be hidden by an
operator preference.

## 11. Smallest Runtime Integration Point

After the pure helper is reviewed, add one new opt-in executor request/result
path adjacent to `execute_with_immutable_run_bundle`.

The path should:

1. prepare and validate execution;
2. build and persist the immutable run bundle;
3. derive all step assessments from the persisted bundle plus explicit runtime
   facts;
4. validate any expected assessment-set fingerprint;
5. bind the accepted aggregate fingerprint and algorithm to durable run
   identity before `RunCreated`;
6. enforce the current step's execution disposition before invocation;
7. retain disclosure obligation without turning visible disclosure into an
   approval;
8. on exact retry, re-read the stored bundle, reassess current facts, and
   require exact binding equality;
9. on approval resume, reassess before appending grant/resume events and reject
   changed posture or fingerprint.

Existing executor APIs remain unchanged. The new path must not make
proportional governance a global default in its first phase.

## 12. Durability And Event Posture

The executor-integration phase needs a separately reviewed durable binding. A
minimal binding should include assessment algorithm, aggregate fingerprint,
immutable bundle root, workflow/run identity, and bounded posture.

Events should record stable references and dispositions, not raw inputs. Event
changes must be additive and backward-readable. The pure-helper phase adds no
persistence or events.

## 13. Failure And Workflow Semantics

- Assessment failure before `RunCreated` creates no partial workflow run.
- A stale fingerprint is an internal governance failure, not a misleading
  project diagnostic.
- Approval-required posture pauses through existing approval semantics.
- Denial fails before skill/provider invocation.
- Visible disclosure alone never pauses execution.
- Reassessment failure on resume appends no grant or resume event first.
- Existing non-opt-in executor behavior remains unchanged.
- Projection failure must not fabricate evidence or alter a terminal run.

## 14. Privacy And Redaction

- Use bounded typed facts and immutable-definition references.
- Store fingerprints and stable reason/disposition vocabulary, not raw content.
- Treat hashes, workflow IDs, step IDs, actor IDs, and resource references as
  potentially sensitive in Debug and errors.
- Do not copy diagnostic messages, check output, provider payloads, source
  snippets, paths, environment values, tokens, credentials, or authorization
  material.
- Serialized assessment bindings remain sensitive operational metadata.

## 15. Test Plan

Future focused tests should prove:

- valid immutable bundle derives ordered per-step assessments;
- mutable project-file changes after bundle creation do not change assessment;
- relevant immutable definition changes alter the fingerprint;
- unrelated definitions outside the bundle do not alter the fingerprint;
- current authority/evidence/check/`SideEffect` changes alter posture and
  fingerprint;
- explicit minima cannot be weakened by inference;
- visible disclosure with proceed does not pause;
- approval and denial prevent invocation;
- stale expected fingerprint fails before `RunCreated`;
- changed retry and resume fail before new execution/resume events;
- exact retry rehydrates consistently;
- missing, ambiguous, or extra definition records fail closed;
- missing, duplicate, extra, or step-mismatched runtime facts fail closed;
- unsupported algorithms fail closed;
- aggregate fingerprint has a stable known vector and resists framing
  collisions;
- the pure helper does not claim independent freshness for supplied facts;
- Debug, serialization, and errors do not leak identifiers or payloads;
- existing executor, immutable bundle, proportional governance, capability,
  approval, SideEffect, evidence, report, and runtime tests pass.

## 16. Proposed Implementation Sequence

1. Pure immutable-bundle governance reassessment helper and focused tests.
2. Maintainer review.
3. Durable assessment-binding model and event vocabulary only.
4. Maintainer review.
5. One explicit opt-in executor path before `RunCreated`.
6. Resume/retry reassessment hardening.
7. Maintainer review of the complete local path.
8. Only then consider default behavior, schema declaration, CLI exposure, UI
   projection, or provider-mutation adoption.

Each item remains a separate governed phase. The first implementation prompt
must target item 1 only.

## 17. Open Questions

- Should the aggregate fingerprint bind an assessment timestamp, or should
  freshness come exclusively from current fact references?
- Which authority facts can be derived from reviewed capability resolutions
  without treating visibility as authority?
- Should incomplete-but-proceed assessments be permitted outside local
  permissive profiles?
- What minimal event vocabulary makes visible disclosure inspectable without
  becoming noisy?
- When should a local UI show quiet decisions by operator preference?
- Which profile first enables the opt-in executor path?
- How should a future workflow schema declare minima without requiring users to
  reproduce the classifier?

## 18. Final Recommendation

The pure immutable-bundle proportional-governance reassessment helper and its
focused blocker fix are implemented and accepted. The next phase should add the
durable assessment-binding model and additive event vocabulary only.

Do not add executor enforcement, retry/resume behavior, schemas, CLI behavior,
UI, provider calls, writes, automatic approvals, enterprise administration, or
default runtime behavior in that phase.
