# Authoritative Proportional-Governance Executor Routing Plan

Status: Planning complete and accepted with non-blocking implementation
constraints in the
[focused plan review](../concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_EXECUTOR_ROUTING_PLAN_REVIEW.md).
The first prerequisite is now implemented and accepted as a model-only,
payload-free visible-disclosure delivery request and surface-acceptance
receipt. Executor integration remains unimplemented.
This document defines the next bounded runtime composition sequence after the
accepted fresh-run authoritative `DocsCheck` quiet-success consumer. No runtime
behavior is implemented by this plan.

Related foundations:

- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Proportional Governance Decision Axes And Workload Inference Plan](proportional-governance-decision-axis-and-inference-plan.md)
- [Runtime Proportional-Governance Reassessment Plan](runtime-proportional-governance-reassessment-plan.md)
- [Authoritative Local-Check Executor Consumer Plan](authoritative-local-check-executor-consumer-plan.md)
- [Approval Resume Resolved-Context Integrity Plan](approval-resume-resolved-context-integrity-plan.md)
- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)

## 1. Executive Summary

Workflow OS now has one accepted executor path that derives an authoritative
governance assessment from an immutable run bundle and a same-call canonical
`DocsCheck`, persists a source-bound assessment, and executes a fresh local run
only when the aggregate result is:

```text
execution=proceed
disclosure=quiet
completeness=complete
```

That slice proves quiet success without weakening evidence or immutable-run
authority. It intentionally rejects every other proportional-governance
outcome.

The next runtime question is how the executor should route a complete,
authoritative assessment without conflating execution, disclosure, and
approval:

- `Proceed + Quiet` continues through the accepted path;
- `Proceed + Visible` continues only through an explicit bounded disclosure
  delivery boundary;
- `RequireApproval + Visible` pauses through the existing approval and
  proof-enforced presentation boundary;
- `Denied + Visible` fails closed with durable bounded decision evidence; and
- incomplete assessment remains an error rather than a fifth permissive route.

This plan defines that boundary and a small implementation sequence. It does
not implement Rust, CLI or UI behavior, schemas, providers, OpenShell, writes,
automatic approvals, hosted administration, enterprise policy, reasoning
lineage, or release changes.

## 2. Product Decision

Proportional governance is a routing decision over independent obligations. It
is not a caller-selected runtime mode.

The executor must consume an assessment that is:

- derived from the validated immutable run bundle;
- composed with current typed runtime facts;
- complete;
- source-bound when authoritative local-check facts contribute;
- persisted create-only against the same run identity; and
- equal to the exact assessment that the executor routes.

The routing result must never be reconstructed from a caller-supplied enum,
display preference, or detached serialized projection.

Operator presentation remains independent from execution disposition:

- a user interface may display `Quiet` decisions without changing governance;
- `Visible` requires bounded delivery through an explicit surface;
- recording that visibility is required does not prove delivery; and
- approval presentation proof is not interchangeable with non-blocking
  disclosure delivery.

## 3. Goals

- Route every complete proportional-governance outcome deterministically.
- Preserve the accepted fresh-run quiet-success path unchanged.
- Continue `Proceed + Visible` without inventing human approval.
- Require proof-enforced presentation before approval can authorize execution.
- Deny before skill execution when the authoritative assessment is denied.
- Keep incomplete or stale facts fail-closed.
- Preserve immutable bundle, source binding, and create-only run ownership.
- Emit durable bounded records that explain which route was selected.
- Keep local process output, source contents, policy payloads, and secrets out
  of routing, disclosure, audit, and error surfaces.
- Define the smallest sequence that closes runtime composition before an
  optional execution provider such as OpenShell is considered.

## 4. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- default or automatic local checks;
- additional local-check command families;
- retry, approval resume, or cancellation in the first routing slice;
- automatic approval, model self-approval, or inferred approval authority;
- a CLI, terminal dashboard, local server, web UI, notification system, or
  hosted disclosure service;
- workflow or policy schema changes;
- provider calls, OpenShell, sandbox lifecycle management, network access, or
  credential injection;
- SideEffect execution or new provider mutation families;
- report artifacts or automatic report generation;
- enterprise steward administration, RBAC, IdP, or shared policy sync;
- reasoning lineage or claim graphs;
- release-posture changes.

## 5. Existing Accepted Inputs

The first implementation should reuse, not duplicate:

- `StoredImmutableRunBundle`;
- `GovernanceAssessmentBinding`;
- `GovernanceAssessmentSourceBinding`;
- `GovernanceExecutionDisposition`;
- `GovernanceDisclosureRequirement`;
- `GovernanceAssessmentCompleteness`;
- `GovernanceAssessmentBound`;
- the create-only immutable run-bundle claim;
- the create-only governance-assessment binding;
- existing `ApprovalRequest` and `ApprovalDecision`;
- existing approval-presentation record and proof-enforcement helpers; and
- existing run event, snapshot, audit, and observability projection.

The accepted authoritative `DocsCheck` consumer remains the first source of
runtime-backed assessment facts. The routing design must not widen the check
family while proving routing.

## 6. Source-Of-Truth Boundaries

| Question | Source of truth | Not sufficient |
| --- | --- | --- |
| What work is being governed? | Validated stored immutable run bundle | Current mutable repository files |
| What posture was selected? | Exact persisted governance assessment binding | Caller enum or read-only projection |
| Did a local check contribute? | Source-bound same-call assessment commitment | Serialized attestation alone |
| Must execution pause? | `GovernanceExecutionDisposition` | Disclosure preference |
| Must an operator be notified? | `GovernanceDisclosureRequirement` | Presence of an audit event |
| Was disclosure delivered? | Explicit delivery receipt from the selected surface | `disclosure=visible` in the assessment |
| Was approval requested? | Durable `ApprovalRequested` event and projection | A report summary |
| Was approval context presented? | Fresh matching approval-presentation proof | Approval decision alone |
| Was approval granted? | Matching durable approval decision | Actor prose or inferred authority |
| Was execution denied? | Authoritative binding plus durable denial route record | Missing handler or unrelated failure |

These boundaries prevent two dangerous substitutions: treating an audit record
as proof that a human saw a disclosure, and treating a visible disclosure as a
human authorization gate.

## 7. Routing Matrix

Only complete assessments enter the routing matrix.

| Execution | Disclosure | Executor route |
| --- | --- | --- |
| `Proceed` | `Quiet` | Persist binding and continue through the accepted quiet path |
| `Proceed` | `Visible` | Deliver a bounded non-blocking disclosure, retain its receipt, then continue |
| `RequireApproval` | `Visible` | Persist binding, create a matching approval request, pause, require fresh presentation proof before grant |
| `Denied` | `Visible` | Persist bounded denial evidence and fail before skill execution |

`RequireApproval + Quiet` and `Denied + Quiet` are invalid accepted decision
states under the normalized model and must fail validation. The executor must
not repair or reinterpret them.

An incomplete assessment must fail before route-specific delivery, approval,
or skill execution. Unknown facts are not silently converted to quiet success.

## 8. Quiet Proceed Route

The accepted `execute_with_authoritative_docs_check_governance(...)` behavior
is the compatibility baseline:

1. acquire the create-only immutable run claim;
2. execute the canonical check;
3. derive the exact aggregate assessment;
4. enforce complete quiet `Proceed`;
5. persist the source-bound binding;
6. append `RunCreated`, `GovernanceAssessmentBound`, validation, and start
   events; and
7. execute the existing sequential workflow.

The routing implementation must preserve its ordering, errors, event identity,
privacy, and single-claim behavior.

## 9. Visible Proceed Route

`Proceed + Visible` is non-blocking with respect to human authorization. It is
not optional with respect to delivery.

The first implementation should accept one explicit injected disclosure
delivery interface rather than a global, environment-selected, or hidden
surface. The interface should consume only a bounded projection derived by
Core and return a validated payload-free receipt.

Candidate concepts, subject to model review:

- `GovernanceDisclosureDelivery`;
- `GovernanceDisclosureReceipt`;
- `GovernanceDisclosureChannel`;
- `GovernanceDisclosureDeliveryId`; and
- `GovernanceDisclosureDeliveryStatus`.

The first channel should be `InMemory` or `InjectedLocal`, not CLI-specific.
The receipt should commit:

- workflow, run, and assessment binding identity;
- disclosure requirement;
- bounded reason codes or assessment fingerprint reference;
- channel kind;
- delivery timestamp;
- correlation identity;
- sensitivity and redaction posture; and
- a versioned receipt identity.

It must not store rendered prose, source contents, commands, process output,
paths, provider payloads, environment values, credentials, or tokens.

The route must distinguish:

- disclosure required;
- delivery attempted;
- delivery accepted by the injected surface; and
- later operator acknowledgement, which remains out of scope.

If no delivery interface is supplied or delivery cannot be confirmed, the
first implementation should fail before workflow skill execution. This is a
delivery-integrity failure, not a human approval requirement. A later product
decision may define durable queued delivery, but the first local slice must not
claim visible disclosure while silently dropping it.

## 10. Approval Route

`RequireApproval + Visible` must reuse the existing approval model and
proof-enforced presentation path. It must not create a second approval system
inside proportional governance.

The first approval implementation should:

1. persist the exact authoritative assessment binding;
2. create a deterministic approval request bound to the run, workflow, step,
   immutable bundle, and assessment;
3. append the existing approval request event and pause the run;
4. require a persisted, matching, fresh approval-presentation record before a
   grant may mutate state;
5. preserve denial as an available decision;
6. revalidate resolved execution context and the exact durable assessment
   before grant-side mutation; and
7. resume only through the existing executor state machine.

The implementation phase must first determine whether the existing
step-scoped `ApprovalRequest` can represent a pre-execution aggregate
governance gate without ambiguity. If it cannot, split a narrow model
prerequisite before executor integration. Do not manufacture a synthetic
workflow step or attach the gate to an unrelated first step merely to avoid
that review.

No automatic approver may be installed. Delegated maintainers may approve only
through the same explicit authority and presentation-proof rules as any other
approver.

## 11. Denial Route

`Denied + Visible` must stop before workflow skill execution.

The route should preserve enough durable bounded state to distinguish a
governance denial from:

- incomplete assessment;
- missing local-check handler;
- failed check execution;
- disclosure-delivery failure;
- approval denial; and
- ordinary workflow execution failure.

The implementation review must decide whether the existing
`GovernanceAssessmentBound` event plus a terminal run event can express this
truthfully. Prefer existing event vocabulary when it is unambiguous. Add a new
event only if existing events would misstate lifecycle or denial provenance.

Errors and audit projection must expose stable codes and bounded posture only.
They must not expose raw reason inputs, fingerprints, IDs not already approved
for projection, paths, commands, output, source contents, or secrets.

## 12. Event And Persistence Ordering

The implementation must define one crash-safe ordering for each route.

Common prerequisites:

1. validate request and fresh-run identity;
2. build and validate the immutable run bundle;
3. preflight canonical check and complete runtime facts;
4. acquire create-only immutable run ownership;
5. execute the canonical check;
6. derive and validate the authoritative assessment;
7. persist the exact source-bound assessment binding.

Route-specific mutation begins only after step 7.

The planning review must test whether persisting a visible receipt, approval
request, or denial after the immutable claim but before ordinary run events
creates an ambiguous recovery state. The first implementation may retain
bounded immutable residue, as the accepted quiet path does, but must document
every create-only commit marker and recovery limitation.

## 13. Retry, Resume, And Freshness

The first implementation should remain fresh-run-only unless approval routing
cannot be tested without the existing resume path.

Before any later retry or resume support:

- reload the exact stored immutable run bundle;
- recompute governance from current typed runtime facts;
- require exact equality with the durable binding or monotonic escalation;
- revalidate authoritative check-source freshness;
- revalidate disclosure receipt or approval-presentation freshness;
- reject downgrades from visible to quiet or approval to proceed; and
- preserve resolved-context commitment checks.

No stale receipt, presentation, check attestation, or assessment binding may
authorize resumed work.

## 14. Privacy And Redaction

Routing inputs, receipts, events, errors, `Debug`, serialization, audit, and
report-ready references must not copy:

- raw source or spec contents;
- raw local-check output;
- raw command lines or environment values;
- raw policy payloads;
- approval reasons or presentation prose;
- provider payloads or logs;
- filesystem paths;
- credentials, authorization headers, private keys, or tokens.

All new identifiers and summaries must be bounded. Unknown wire values must
fail through custom non-echoing deserialization or an equivalent safe boundary.

## 15. Error Contract

Future errors should use stable families, likely:

- `executor.authoritative_governance.assessment_incomplete`;
- `executor.authoritative_governance.assessment_denied`;
- `executor.authoritative_governance.disclosure_sink_required`;
- `executor.authoritative_governance.disclosure_delivery_failed`;
- `executor.authoritative_governance.disclosure_receipt_mismatch`;
- `executor.authoritative_governance.approval_model_unsupported`;
- `executor.authoritative_governance.approval_context_mismatch`; and
- `executor.authoritative_governance.route_conflict`.

Names may be refined during implementation, but messages must remain static and
must not include caller values.

## 16. Test Plan

Future tests must prove:

- existing complete quiet `Proceed` behavior remains unchanged;
- complete visible `Proceed` invokes exactly one explicit disclosure surface;
- visible delivery occurs before skill execution;
- a missing or failed disclosure surface creates no skill invocation;
- a receipt binds the exact run and assessment;
- a receipt for another run, bundle, or assessment fails closed;
- visible execution does not create an approval request;
- approval-required execution creates one deterministic request and pauses;
- grant requires matching fresh presentation proof;
- approval denial fails closed;
- changed resolved context or assessment blocks approval resume;
- denied execution invokes no skills or disclosure surface that claims success;
- incomplete assessment enters no permissive route;
- invalid quiet approval or quiet denial wire states fail closed;
- competing fresh-run callers cannot duplicate check, disclosure, or approval;
- event ordering and snapshot posture are deterministic;
- audit projection distinguishes quiet, disclosed, approval-pending, denied,
  and delivery-failed posture;
- `Debug`, errors, serde, audit, and receipt projection do not leak forbidden
  values;
- existing local executor, approval, immutable bundle, local-check,
  proportional-governance, audit, and WorkReport tests pass.

## 17. Implementation Sequence

Use small reviewed phases:

1. **Planning review.** Confirm the routing matrix, event ordering, and whether
   existing approval/event models are sufficient.
2. **Visible-disclosure prerequisite, if required.** Add only the smallest
   payload-free delivery contract and receipt model with validation, serde,
   privacy, and tests. Status: implemented in
   [Governance Disclosure Delivery Model Report](../concepts/GOVERNANCE_DISCLOSURE_DELIVERY_MODEL_REPORT.md)
   and accepted with non-blocking constraints in
   [Governance Disclosure Delivery Model Review](../concepts/GOVERNANCE_DISCLOSURE_DELIVERY_MODEL_REVIEW.md).
3. **Visible `Proceed` executor integration.** Extend the explicit
   authoritative fresh-run consumer through one injected local disclosure
   surface. Status: implemented in
   [Visible Proceed Executor Integration Report](../concepts/VISIBLE_PROCEED_EXECUTOR_INTEGRATION_REPORT.md).
   Core constructs the exact delivery request, the injected handler may return
   only an acceptance timestamp or a structured failure, and Core constructs
   and validates the receipt before `RunCreated` or skill invocation.
4. **Visible route review and blocker fixes.** Status: accepted with no
   blockers in
   [Visible Proceed Executor Integration Review](../concepts/VISIBLE_PROCEED_EXECUTOR_INTEGRATION_REVIEW.md).
5. **Approval model prerequisite, if required.** Status: implemented in
   [Proportional Governance Approval Binding Report](../concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_BINDING_REPORT.md).
   The payload-free binding commits one bounded approval identity to the exact
   complete, source-bound aggregate `RequireApproval + Visible` assessment.
   It does not request, decide, persist, or resume an approval and does not
   create a synthetic workflow step. Focused review accepts the model while
   requiring the future executor to construct it from same-call authority; see
   [Proportional Governance Approval Binding Review](../concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_BINDING_REVIEW.md).
6. **Approval-required executor integration.** Reuse existing pause,
   presentation-proof, decision, and resolved-context enforcement.
7. **Denial route integration.** Persist bounded denial provenance and fail
   before skill execution.
8. **Combined routing review.** Verify monotonicity, crash ordering,
   non-leakage, and compatibility.
9. **Operator UX planning later.** Project accepted runtime state through
   concise human and bounded machine surfaces without moving policy authority
   into UI code.

Do not implement all routes in one unreviewable change.

## 18. Open Questions

- Can the existing `ApprovalRequest` truthfully represent an aggregate
  pre-execution governance gate, or is a narrow binding field required?
- Should visible delivery failure terminate the run or return a pre-run
  structured result before `RunCreated`?
- Which existing event is the correct commit marker for a visible receipt?
- Is an injected synchronous surface enough for the first proof, or does any
  durable queued delivery require a separate plan?
- Can existing terminal failure events distinguish governance denial without a
  new event kind?
- Which reason-code subset is safe and useful in a bounded disclosure?
- How should a future UI display quiet decisions live without creating false
  `Visible` receipts?
- What freshness policy should apply to disclosure receipts during resume?

## 19. Relationship To Optional Execution Providers

An execution substrate such as NVIDIA OpenShell is complementary but later.
Workflow OS should eventually be able to route an authorized invocation to an
optional sandbox provider and cite its sandbox ID, effective policy revision,
image digest, exit status, denial logs, and artifact references.

That integration must consume this routing and authority boundary; it must not
be used to avoid it. OpenShell planning or implementation remains out of scope
until authoritative routing, immutable inputs, and scoped capability authority
are accepted.

## 20. Final Recommendation

Proceed next with a focused maintainer review of this plan.

If accepted, implement only the smallest visible `Proceed` prerequisite and
executor slice first. Preserve the accepted quiet path, require explicit
delivery proof, and keep approval and denial as subsequent reviewed slices.

Do not add OpenShell, providers, writes, CLI/UI behavior, schemas, automatic
approvals, hosted administration, or new mutation families.

## 21. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785032157063812000-2`
- approval:
  `approval/run-1785032157063812000-2/planning-approved`
- presentation: `presentation/890bc03190252cb3`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- validation summary: `npm run check:docs` and `git diff --check` passed
- out-of-kernel work: architecture inspection, plan authoring, documentation
  edits, validation commands, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run documentation checks, create a WorkReport artifact, or perform git
  actions
