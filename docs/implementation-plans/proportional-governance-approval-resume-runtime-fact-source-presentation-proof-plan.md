# Proportional-Governance Approval-Resume Runtime-Fact Source Presentation-Proof Plan

Status: implemented and accepted in the
[implementation report](../concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_RESUME_RUNTIME_FACT_SOURCE_PRESENTATION_PROOF_REPORT.md)
and [focused maintainer review](../concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_RESUME_RUNTIME_FACT_SOURCE_PRESENTATION_PROOF_REVIEW.md).

## 1. Executive Summary

Workflow OS now has two accepted local approval-resume boundaries that protect
different governance invariants:

- the approval-presentation path proves that the exact bounded approval scope
  was presented before a decision; and
- the current runtime-fact source path proves that a granted decision resumes
  only after fresh facts from the registered source reproduce the durable V3
  proportional-governance assessment.

No public helper composes both invariants for the generic registered
runtime-fact source. This phase will add one explicit, local, opt-in wrapper
that validates durable presentation proof before source access and then
delegates grant reassessment and mutation to the accepted source-backed
approval helper.

The implementation will not change default approval behavior, automatically
approve work, persist raw facts, add CLI or UI behavior, invoke providers,
integrate OpenShell, execute SideEffects, add writes, or broaden release
posture.

## 2. Goals

- Require matching durable approval-presentation proof for one explicit
  source-backed approval-resume path.
- Validate proof identity, content binding, decision binding, and optional
  freshness before a runtime-fact source is invoked.
- Preserve the accepted source-backed grant ordering: freeze the exact resume
  plan, validate immutable state and V3 source registration, resolve current
  facts once, reproduce the durable assessment, and only then mutate the run.
- Preserve source-free denial while still requiring presentation proof for the
  human or delegated-maintainer decision.
- Attach the existing bounded proof marker to granted and denied approval
  events.
- Return the accepted payload-free decision-time runtime-fact snapshot only
  for a granted decision that performed reassessment.
- Fail without new events or skill invocation when proof or source validation
  fails.
- Use stable, non-leaking errors and redaction-safe Debug output.

## 3. Non-Goals

- Changing `LocalExecutor::decide_approval(...)` or any existing default.
- Automatic approval, model self-approval, or inferred approval authority.
- Making every approval source-backed or presentation-proof-enforced.
- Creating another approval state machine or reassessment implementation.
- Reusing an initial or decision-time runtime-fact snapshot as authority.
- Persisting raw runtime facts, source payloads, or approval-presentation text.
- Persisting a second runtime-fact snapshot or replacing the initial durable
  provenance commitment.
- Workflow or project schema changes.
- CLI, UI, approval-card, or public operator rendering.
- Report, artifact, or audit projection changes.
- Provider execution, OpenShell integration, SideEffect execution, external
  writes, hosted expansion, or additional mutation families.
- Enterprise identity, RBAC, quorum, cryptographic attestation, or production
  time authority.
- Release posture changes.

## 4. Current Boundary

`LocalExecutor::decide_approval_with_presentation(...)` and the explicit
presentation-policy helpers validate one durable `ApprovalPresentationRecord`
before applying an approval decision. They derive a bounded proof marker that
is attached to the approval event. They do not reassess proportional-governance
facts from a registered source.

`decide_approval_with_current_runtime_facts_governance_reassessment(...)`
accepts an explicit source registration and injected
`GovernanceRuntimeFactSource`. A grant freezes the exact resolved resume plan,
validates the immutable bundle and durable V3 source commitment, invokes the
source once, and requires the fresh assessment to reproduce the durable
binding before any approval or resume event. A denial invokes no source. This
helper does not require presentation proof.

The authoritative docs-check path already composes reassessment and
presentation proof for one specialized local-check route. It is not a generic
registered runtime-fact source consumer and must not be generalized by
duplicating its handler-specific implementation.

## 5. Candidate API

Add the smallest additive request type and public helper, likely:

- `LocalCurrentRuntimeFactsGovernanceApprovalPresentationDecisionRequest`
  - existing `LocalApprovalPresentationDecisionRequest`;
  - active `GovernanceStrictnessProfile`;
  - explicit `GovernanceRuntimeFactSourceRegistration`;
  - Core-selected `evaluated_at`;
  - optional expected aggregate fingerprint.
- `decide_approval_with_current_runtime_facts_governance_reassessment_and_presentation(...)`
  - existing executor;
  - existing local immutable-bundle store;
  - injected `GovernanceRuntimeFactSource`;
  - explicit request;
  - existing `LocalCurrentRuntimeFactsGovernanceApprovalDecisionResult` result.

Final names should follow repository conventions. The result should be reused
because proof enforcement changes authorization evidence, not the shape of the
fresh runtime-fact outcome.

The caller must explicitly supply the store, source, registration, profile,
decision time, approval request, proof selector, and optional proof age. The
helper must not discover them from hidden global state or runtime config.

## 6. Composition Strategy

The implementation should refactor the accepted boundaries into narrow private
helpers rather than call one public decision helper from another after partial
validation.

Recommended ownership:

1. existing approval preparation remains the only pending-state validator;
2. existing presentation-proof resolution and validation remains the only
   proof authority and proof-marker constructor;
3. existing grant-precondition path remains the only resume-plan freeze and
   approval mutation boundary; and
4. existing current-runtime-fact reassessment helper remains the only source,
   V3 commitment, bundle, freshness, and assessment validator.

The public source-only helper should continue to work unchanged. The new
proof-enforced helper should prepare the decision, validate and attach the
proof marker, then invoke the same private source-backed grant-precondition
composition used by the source-only helper.

## 7. Required Operation Ordering

For both grant and denial, Core must:

1. rehydrate the run and validate that it is waiting on the exact pending
   approval;
2. resolve the selected durable presentation record;
3. validate run, approval, actor, decision, content binding, and optional proof
   freshness; and
4. derive and attach the existing proof marker to the in-memory decision.

If proof validation fails, Core must return before source invocation, approval
events, resume events, policy events, or skill invocation.

For a grant, Core must then:

5. reconstruct and freeze the exact resolved resume plan;
6. validate resolved-context commitment and immutable run-bundle identity;
7. load and validate the durable V3 assessment and initial source-snapshot
   commitment;
8. require supplied registration equality before source invocation;
9. invoke the source exactly once at the explicit decision time;
10. validate source identity, version, bundle binding, freshness, fact
    coverage, and canonical assessment;
11. require the fresh assessment to reproduce the durable V3 binding; and
12. append `ApprovalGranted` with the proof marker, evaluate resume policy,
    append `RunResumed`, and execute the already-frozen plan.

No state mutation may occur before step 12. The proof marker must identify the
presentation used without copying presentation text.

## 8. Denial Semantics

A denial does not resume execution and therefore must not depend on runtime
fact source availability. It must still prove that the exact denial scope was
presented.

After approval preparation and presentation-proof validation, denial should
reuse the existing fail-closed denial path:

- invoke the source zero times;
- append `ApprovalDenied` with the proof marker;
- fail the run through existing semantics; and
- return no governance reassessment binding or decision-time snapshot.

This preserves the accepted source-backed denial behavior while making the
decision itself auditable.

## 9. Proof And Source Failure Semantics

Proof failures take precedence over source failures because the source must not
be consulted for a decision whose presentation authority is absent, stale,
ambiguous, corrupt, or mismatched.

The helper must prove:

- missing, ambiguous, corrupt, stale, or mismatched proof causes zero source
  calls and zero new events;
- changed resolved context causes zero source calls;
- changed source registration causes zero source calls;
- source error, changed facts, changed bundle, stale facts, or changed
  assessment causes zero new events;
- no failure path fabricates a proof marker, source snapshot, evidence
  reference, approval decision, or workflow result; and
- the event history remains exactly equal after any pre-mutation failure.

If both proof and source input are invalid, the stable proof error should be
returned because proof validation occurs first.

## 10. Privacy And Error Posture

- Reuse existing approval-presentation and runtime-fact source error codes when
  those boundaries own the failure.
- Add no raw identifiers or values to error messages.
- Do not include approval reasons, presentation content, content hashes,
  project paths, source IDs, source versions, snapshot IDs, fact values,
  timestamps, fingerprints, command output, provider output, tokens, or
  credentials in errors.
- Redact the nested approval request, proof identity, registration, decision
  time, expected fingerprint, and result metadata in Debug output.
- Keep raw runtime facts and presentation text out of events, returned results,
  serialization, evidence summaries, and reports.
- Preserve the existing payload-free proof marker and runtime-fact snapshot
  contracts.

## 11. Compatibility And Product Posture

This path remains explicit and opt-in. Existing source-only, proof-only,
caller-fact, authoritative local-check, ordinary approval, and default policy
paths must remain unchanged.

The composition is a prerequisite for safely reducing low-risk ceremony. It
does not itself make approval quieter or automatic. Proportional governance may
later avoid an approval when validated facts permit quiet execution, but any
approval that remains required must prove both what was presented and what
current facts authorized at resume time.

The fresh-pull evaluator review supports this sequencing. First-run and
governance authoring are credible; the next product need is lower ceremony for
low-risk work without weakening evidence or approval integrity. Previously
reported Node 24 integration-check opacity and duplicate missing-manifest
diagnostics are already fixed on current `main` and do not reopen scope here.

## 12. Test Plan

Add focused tests proving:

1. a valid proof-enforced grant invokes the source once and completes;
2. the approval event contains the expected bounded proof marker;
3. the result returns the accepted decision-time binding and snapshot;
4. the durable initial source commitment remains unchanged;
5. missing proof fails before source invocation or new events;
6. ambiguous proof fails before source invocation or new events;
7. corrupt proof fails before source invocation or new events;
8. stale proof fails before source invocation or new events;
9. proof for another run, approval, actor, decision, or content fails before
   source invocation;
10. invalid proof takes precedence when source inputs are also invalid;
11. changed resolved context fails before source invocation;
12. changed source registration fails before source invocation;
13. source failure returns a stable non-leaking error after valid proof and
    before new events;
14. changed current facts or assessment leaves exact event history unchanged;
15. V1 or V2 durable binding remains unsupported on this source path;
16. a valid proof-enforced denial invokes no source and carries the proof marker;
17. denial returns no reassessment binding or runtime-fact snapshot;
18. duplicate decision and terminal-run behavior remain unchanged;
19. request/result Debug and errors do not leak secret-like values;
20. raw presentation text and raw facts do not enter events or serialization;
21. existing proof-only, source-only, authoritative, ordinary approval, and
    runtime tests remain green; and
22. `cargo test --workspace` passes.

Use event-vector equality, not only event-count equality, on pre-mutation
failure tests. Use a counting source to prove the exact source-call boundary.

## 13. Documentation And Validation

Implementation should update this plan and `ROADMAP.md`, create a focused
implementation report and maintainer review, and preserve explicit statements
that default approval behavior, automatic approvals, CLI/UI behavior, schemas,
providers, OpenShell, SideEffects, writes, hosted expansion, and release
posture remain unchanged.

Run:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused executor tests;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 14. Acceptance Criteria

- One explicit local helper composes durable presentation proof with fresh
  registered runtime-fact approval reassessment.
- A grant validates proof before source access and validates fresh facts before
  mutation.
- A denial validates proof but invokes no source.
- Granted and denied events carry the existing bounded proof marker.
- Missing, stale, corrupt, ambiguous, or mismatched proof causes zero source
  calls and zero new events.
- Source or reassessment failure causes zero new events.
- Existing approval APIs and defaults remain unchanged.
- No raw facts, presentation content, secrets, or provider payloads leak.
- No schemas, CLI/UI behavior, providers, OpenShell integration, SideEffects,
  writes, hosted expansion, or release changes are introduced.

## 15. Recommended Next Phase

Implement the explicit local proof-enforced current-runtime-fact
approval-resume wrapper in one governed runtime-composition phase, followed by
a focused maintainer review.

## 16. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run ID: `run-1786289982161301000-2`.
- Approval ID: `approval/run-1786289982161301000-2/planning-approved`.
- Approval presentation ID: `presentation/0848855afdd4da7f`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Approved scope: planning and documentation only.
- Phase status: completed with 39 ordered events, one approval, zero retries,
  and zero escalations.
- Validation: `npm run check:docs` passed; `git diff --check` passed. Rust
  checks were not run because this phase changed documentation only.
- Out-of-kernel work: Codex inspected code and documentation and authored these
  planning artifacts, executed documentation validation, and will perform git
  and PR actions. The kernel governed scope, approval, and phase close but did
  not edit files or execute those commands.
