# Authoritative Local-Check Executor Consumer Plan

Status: implemented and accepted after the atomic fresh-run claim blocker was
fixed and re-reviewed. The
[implementation review](../concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_REVIEW.md)
found that concurrent callers may both pass the initial empty-state check and
that the second caller can accept the first caller's identical bundle before
re-executing the local check. The fix is documented in
[Authoritative Local-Check Executor Consumer Blocker Fix Report](../concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_BLOCKER_FIX_REPORT.md).
The focused re-review accepts the create-only claim boundary in
[Authoritative Local-Check Executor Consumer Blocker Fix Review](../concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_BLOCKER_FIX_REVIEW.md).
The accepted plan review remains in
[Authoritative Local-Check Executor Consumer Plan Review](../concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_PLAN_REVIEW.md).

The implementation is documented in the
[phase report](../concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_REPORT.md).
It adds one explicit fresh-run-only `DocsCheck` executor consumer and a
backward-readable V2 governance binding with an authoritative source
commitment. Existing executor APIs and defaults remain unchanged.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)
- [Authoritative Local-Check Same-Call Composition Plan](authoritative-local-check-same-call-composition-plan.md)
- [Authoritative Local-Check Reassessment Binding Plan](authoritative-local-check-reassessment-binding-plan.md)
- [Authoritative Local-Check Reassessment Binding Review](../concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_REVIEW.md)

## 1. Executive Summary

Workflow OS can now execute an exact declared `DocsCheck`, derive complete
structural coverage from canonical immutable declarations, convert that
coverage into an authoritative evidence/check fact, and bind the fact to a
complete proportional-governance reassessment in one private call.

No executor consumes that fact-bound value today. The existing opt-in
assessment-bound executor path still receives caller-supplied runtime facts.
Copying only the authoritative posture into that path would detach the posture
from the fact that justified it.

The first consumer should therefore be one additive, fresh-run-only executor
path for explicit `DocsCheck` execution. It should perform complete preflight,
execute the accepted same-call composition and reassessment helper, retain the
authoritative source commitment in the durable assessment binding, and execute
only when the selected step resolves to `Proceed` plus quiet disclosure.

Visible disclosure, approval-required, and denied results should fail closed
before `RunCreated` in this first slice. Later phases may implement durable
visible disclosure and proportional approval creation. This plan does not
implement anything.

## 2. Product Rationale

Fresh-pull evaluation confirms that Workflow OS is already a coherent local
governance kernel and that the next product problem is reducing ceremony for
low-risk work without losing the evidence trail.

The first runtime value should be:

```text
declared deterministic check
  -> kernel-observed result
  -> authoritative complete check fact
  -> fact-bound proportional reassessment
  -> quiet execution only when every enforced axis permits it
  -> durable source-bound governance record
```

This is a narrow execution proof, not automatic repository automation. It
connects already-reviewed primitives into one enforceable runtime path while
keeping existing executor behavior unchanged.

The same evaluation also confirmed that Node 24 integration-check output and
duplicate missing-manifest diagnostics were evaluator-facing papercuts. Those
issues are already fixed and do not change this phase sequence.

## 3. Goals

- Add one explicit opt-in executor consumer of the private fact-bound
  reassessment value.
- Support a fresh local run only.
- Support one explicitly selected workflow step with canonical `DocsCheck`
  declarations.
- Accept an explicit `DocsCheckLocalHandler`; never register or discover it by
  default.
- Derive attestation requirements and invocation identities inside Core from
  the stored immutable bundle.
- Run complete deterministic preflight before clock or process use.
- Execute the declared checks in canonical order through the accepted
  same-call helper.
- Consume the private bound assessment without exposing or reconstructing its
  assessment set.
- Preserve the authoritative local-check binding fingerprint in the durable
  governance binding.
- Execute only when the complete workflow assessment set resolves to
  `Proceed`, quiet disclosure, and complete facts.
- Fail closed before `RunCreated` for visible disclosure, approval-required,
  denied, incomplete, failed-check, unavailable-check, or invalid contexts.
- Return bounded local-check results with the run and durable bindings.
- Preserve deterministic, non-leaking error precedence.
- Keep every existing executor API and default unchanged.

## 4. Strict Non-Goals

The first implementation must not add:

- default, automatic, background, parallel, or repository-wide check
  execution;
- more than one selected local-check step;
- check families other than the existing accepted `DocsCheck` path;
- retry, rehydration, approval resume, or cancellation support for the new
  path;
- proportional approval-request creation;
- visible-disclosure persistence or presentation;
- warning continuation or best-effort fallback;
- caller-supplied evidence/check posture for the selected step;
- imported, cached, persisted, replayed, or caller-constructed local-check
  facts;
- raw command output, source content, parser payload, environment values, or
  credentials in state, events, errors, reports, or debug output;
- report generation, report artifacts, evidence attachment, or CLI rendering;
- workflow, policy, project, SDK, or schema changes;
- providers, OpenShell, SideEffects, external writes, or network access;
- hosted or distributed behavior;
- reasoning lineage;
- enterprise administration; or
- release-posture changes.

## 5. Selected Integration Boundary

Add one public additive free function beside
`execute_with_governance_assessment_binding(...)`, with final names subject to
implementation review:

```text
execute_with_authoritative_docs_check_governance(
  executor,
  immutable_bundle_store,
  docs_check_handler,
  request,
) -> Result<LocalExecutionWithAuthoritativeDocsCheckGovernanceResult, WorkflowOsError>
```

The request should contain:

- the existing `LocalExecutionWithImmutableRunBundleRequest`;
- one selected `StepId`;
- an explicit `GovernanceStrictnessProfile`;
- exactly one `StepGovernanceRuntimeFacts` record per immutable workflow step,
  with no evidence/check posture for the selected step; and
- an optional expected aggregate assessment fingerprint.

The request must not contain:

- attestation requirements;
- invocation, result, attestation, obligation, fact, or binding IDs;
- check posture;
- a detached assessment set;
- a prior check result; or
- a prior authoritative binding.

Core should derive those values from the stored immutable bundle and a
versioned, length-framed identity algorithm.

## 6. Fresh-Run-Only Boundary

The first consumer should require a caller-supplied run ID and require that the
state backend and immutable-bundle store contain no state for it.

An existing run, bundle, governance binding, or event history must fail before
clock or process use with a stable unsupported-existing-run error. The helper
must not rehydrate, rerun a check, reuse a prior result, or fall back to
`execute_with_governance_assessment_binding(...)`.

This restriction is deliberate. The current private binding is same-call
authority, not a replayable attestation. Retry and approval-resume support
require separately reviewed freshness, one-time claim, and durable fact-source
reassessment semantics.

## 7. Complete Preflight And Error Precedence

Before clock or process use, the consumer should:

1. require a fresh explicit run ID;
2. prepare and validate the existing execution plan;
3. evaluate existing pre-run policy;
4. load the validated project with the existing default capability;
5. build the immutable run bundle in memory;
6. require exact execution-plan and immutable-manifest identity;
7. resolve the selected workflow step;
8. resolve its canonical local-check declaration set;
9. require at least one declaration and require every declaration to use the
   accepted `DocsCheck` command kind;
10. require the explicit handler contract to match every canonical command
    commitment;
11. require exactly one runtime-fact record for every workflow step;
12. require no selected-step evidence/check posture; and
13. preflight the complete wrapper context through the existing pure
    reassessment and same-call composition boundaries.

Only after that preflight succeeds may the immutable bundle be written or the
local process start.

The implementation must add a deterministic public error-precedence
regression: an earlier invalid immutable or assessment context must win over a
later process/check failure. Error order must not depend on filesystem timing,
clock values, process output, or collection iteration order.

## 8. Core-Owned Check Input Derivation

For every canonical selected-step declaration, Core should construct:

- `LocalCheckAttestationRequirement` from the canonical declaration;
- invocation, result, and attestation IDs from the exact run, step, and
  requirement identity;
- an idempotency key from the same bounded identity;
- the stored immutable bundle and exact workflow/run/step context; and
- the explicit handler plus the existing system observation clock.

The derivation algorithm must be versioned and length framed. IDs must remain
within current bounds and change when any exact run, step, requirement, or
command commitment changes.

Callers must not be able to supply or override these identities.

## 9. Inseparable Assessment Consumption

The executor must consume `AuthoritativeLocalCheckBoundAssessment` directly.
It must not call `local_check_posture()` and rebuild a separate assessment
request.

Add the smallest crate-private consumption method that can:

- inspect the complete assessment set's strictest execution, disclosure, and
  completeness posture;
- derive the existing durable `GovernanceAssessmentBinding` from the owned
  complete assessment set; and
- attach the authoritative local-check reassessment algorithm and binding
  fingerprint as an optional source commitment.

The raw assessment set and raw local-check fact must remain private and
non-serializable.

## 10. Durable Source Commitment

The current `GovernanceAssessmentBinding` proves the accepted assessment set,
but it does not prove which authoritative runtime fact supplied a selected
axis.

The first consumer must not persist that ordinary binding while discarding the
fact commitment. Extend the binding with one backward-compatible optional
source commitment, likely:

```text
GovernanceAssessmentSourceBinding {
  kind: authoritative_local_check_reassessment,
  algorithm: bounded identifier,
  fingerprint: SpecContentHash,
  selected_step_id: StepId,
}
```

Requirements:

- absence remains valid for existing bindings and serialized state;
- presence is validated, redaction-safe, and included in binding equality;
- the existing binding event and audit projection disclose only bounded kind
  and presence, not raw IDs or fingerprints;
- create-only storage retains the exact source commitment;
- the source commitment is derived only by consuming the private bound value;
  and
- no public constructor may accept an arbitrary fingerprint and claim
  authoritative local-check provenance.

Serialized source commitments remain integrity-checked data, not independent
authenticity or reusable authority. A deserialized commitment becomes runtime
authority only when the executor matches it to the exact create-only stored
binding and a current same-call private reassessment. Validation must not
describe an arbitrary well-formed serialized fingerprint as proof that a check
ran.

If this backward-compatible extension cannot be implemented without exposing a
forgeable public constructor, split it into a focused prerequisite model phase
before executor wiring. Do not weaken the boundary to keep the phase count
small.

## 11. Runtime Decision Semantics

The first consumer should implement one aggregate execution cell only:

| Aggregate execution | Aggregate disclosure | Aggregate completeness | First consumer behavior |
| --- | --- | --- | --- |
| `Proceed` | quiet | complete | Persist exact bindings, append existing binding/run events, and execute. |
| `Proceed` | visible | any | Fail closed before `RunCreated`; durable visible disclosure is not implemented here. |
| approval required | visible | any | Fail closed before `RunCreated`; proportional approval creation is not implemented here. |
| denied | visible | any | Fail closed before `RunCreated`. |
| any | any | incomplete | Fail closed before `RunCreated`; unknown required facts cannot authorize quiet execution. |

This makes quiet success real for one explicitly selected, fully checked,
low-risk path without silently swallowing a disclosure or inventing an
approval.

The selected assessment must also be present and must carry the authoritative
check-derived posture, but it is not the final execution decision by itself.
Workflow-, skill-, policy-, authority-, sensitivity-, SideEffect-,
prior-decision-, runtime-escalation-, profile-, and steward-derived minima
across every step remain monotonic. A passing check may satisfy only the
selected evidence/check axis.

## 12. Execution Ordering

For a valid quiet result, ordering should be:

1. complete pure preflight;
2. persist or validate the new immutable bundle create-only;
3. reload the exact stored bundle;
4. execute canonical checks and derive the private bound assessment;
5. validate optional expected aggregate fingerprint;
6. derive and persist the source-bound governance binding create-only;
7. attach immutable and governance bindings to the execution plan;
8. append existing binding and run-start events; and
9. execute the existing sequential workflow.

No workflow event may exist if check execution, attestation, coverage,
reassessment, source binding, or unsupported disposition fails.

The check process may complete before a later store failure. Errors and the
result must state no rollback claim. The check contract remains no-source-write
and network-disabled.

## 13. Result Model

Return a narrow result containing:

- owned `WorkflowRun`;
- immutable run bundle binding;
- source-bound governance assessment binding; and
- bounded `LocalCheckResult` values in canonical declaration order.

Read-only accessors and `into_parts()` should follow existing executor result
patterns.

`Debug` should expose only:

- run status;
- result count and statuses;
- bounded governance posture; and
- source-binding presence.

It must redact IDs, paths, commands, hashes, output summaries, report text,
environment values, and fingerprints.

## 14. Failure Behavior

Stable errors should distinguish:

- existing-run unsupported;
- selected step unresolved;
- declaration set missing or unsupported;
- handler contract mismatch;
- runtime-fact mismatch or selected posture supplied;
- immutable context mismatch;
- check execution or attestation failure;
- check coverage failure;
- reassessment failure;
- expected fingerprint mismatch;
- source binding invalid;
- visible disclosure unsupported;
- proportional approval unsupported; and
- proportional denial.

Errors must use static bounded messages. They must not echo workflow, run,
step, requirement, command, path, hash, output, source, environment, token,
provider, or secret-like values.

There must be no fallback to caller posture, the existing caller-fact executor
path, unbound execution, or best-effort continuation.

## 15. Privacy And Redaction

The consumer may retain only:

- validated immutable definitions already permitted by the bundle;
- bounded local-check result metadata;
- payload-free binding commitments;
- bounded governance disposition; and
- existing run/event state.

It must not retain or expose:

- stdout or stderr;
- raw command lines;
- repository paths;
- source contents;
- package or parser payloads;
- environment values;
- credentials or tokens;
- provider payloads; or
- raw fact or assessment structures.

## 16. Compatibility

- Existing `LocalExecutor::execute(...)` remains unchanged.
- Existing immutable-bundle and caller-fact governance paths remain unchanged.
- Existing approval and cancellation APIs remain unchanged.
- Existing serialized governance bindings without a source commitment remain
  valid.
- No workflow schema or CLI surface changes.
- No automatic migration or default activation.

The new function should be documented as experimental, local, explicit, and
fresh-run-only.

## 17. Test Plan

Future implementation tests should prove:

1. one fresh multi-step workflow with a selected declared `DocsCheck` step and
   quiet `Proceed` posture executes successfully;
2. all workflow steps remain in existing sequential order;
3. canonical check declarations determine check order;
4. Core derives all check and attestation identities;
5. caller-selected selected-step evidence/check posture is rejected;
6. an invalid immutable context fails before clock or process use;
7. public error precedence is deterministic when early context and later
   process failures coexist;
8. a passed complete check set can satisfy only the selected evidence/check
   axis;
9. other governance minima cannot be weakened;
10. failed, unavailable, missing, duplicate, unexpected, or mismatched checks
    create no run events;
11. a quiet selected step cannot mask visible disclosure, approval, denial, or
    incomplete facts on another step;
12. aggregate visible disclosure fails before `RunCreated`;
13. aggregate approval-required posture fails before `RunCreated`;
14. aggregate denied or incomplete posture fails before `RunCreated`;
15. an existing run, bundle, binding, or event history is rejected before
    process use;
16. the durable assessment binding retains the authoritative source
    commitment;
17. existing bindings without a source commitment remain readable;
18. source-binding substitution or removal fails closed at the exact
    create-only store and current same-call comparison boundary;
19. deserialized source commitments are not treated as independent proof;
20. the binding event and audit projection remain bounded and non-leaking;
21. result ordering is deterministic;
22. `Debug`, serialization, persistence errors, and runtime errors do not leak
    IDs, hashes, paths, commands, output, source, environment, or secrets;
23. no report, artifact, evidence record, provider call, SideEffect, external
    write, or CLI output is created;
24. all existing executor, immutable-bundle, proportional-governance,
    local-check, provider, report, and workspace tests remain green.

## 18. Implementation Sequence

1. Perform phase-level maintainer review of this plan.
2. If source-binding compatibility is accepted, implement the optional durable
   source commitment and focused compatibility tests.
3. Implement the explicit fresh-run executor request/result and Core-owned
   identity derivation.
4. Consume the private bound assessment and enforce the quiet-only decision
   table.
5. Add focused ordering, preflight, monotonicity, failure, privacy, and
   no-side-effect tests.
6. Run full repository validation.
7. Perform phase-level maintainer review before planning retry, approval,
   visible disclosure, additional check families, or any provider integration.

Implementation should remain one reviewed runtime-composition phase only if
the source-binding model remains a small backward-compatible extension.
Otherwise split the model prerequisite rather than create detached authority.

## 19. Open Questions

- Should the optional source commitment live directly on
  `GovernanceAssessmentBinding` or in a nested validated source-binding type?
- Should the new result expose full bounded `LocalCheckResult` values or only
  stable result references and statuses?
- Is one selected step sufficient for the first runtime proof, or should the
  first consumer require every check-declaring step while still remaining
  fresh-run-only?
- What exact event or durable record should carry visible disclosure before
  that disposition can continue without blocking?
- What one-time claim and freshness model is required before retry or approval
  resume can rerun or reuse a check?

## 20. Final Recommendation

The fresh-run-only executor implementation and blocker fix are accepted.
Re-read the roadmap from current `main` before beginning the next separately
governed runtime-composition phase.

Do not implement retry, approval resume, visible disclosure continuation,
automatic checks, additional check families, reports, artifacts, evidence
attachment, CLI behavior, schemas, providers, OpenShell, SideEffects, writes,
hosted behavior, reasoning lineage, enterprise administration, or release
changes.
