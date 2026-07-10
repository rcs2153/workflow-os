# Approval Gate Presentation Default Enforcement Plan

Status: Planned only. This plan follows the accepted
[Provider Call Orchestration Gate Clarity Hardening Review](../concepts/PROVIDER_CALL_ORCHESTRATION_GATE_CLARITY_HARDENING_REVIEW.md)
and the open P0 gap tracked in
[Approval Gate Presentation Enforcement Gap](../concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md).

Related implemented foundations:

- [Approval Gate Presentation Core Model Review](../concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REVIEW.md)
- [Approval Gate Presentation Persistence Review](../concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md)
- [Approval Gate Presentation Opt-In Enforcement Review](../concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_REVIEW.md)
- [Dogfood Runner Approval-Presentation Enforcement Review](../concepts/DOGFOOD_RUNNER_APPROVAL_PRESENTATION_ENFORCEMENT_REVIEW.md)
- [Dogfood Approval-Presentation Freshness Enforcement Review](../concepts/DOGFOOD_APPROVAL_PRESENTATION_FRESHNESS_ENFORCEMENT_REVIEW.md)
- [Dogfood Approval-Presentation Denial Proof Implementation Review](../concepts/DOGFOOD_APPROVAL_PRESENTATION_DENIAL_PROOF_IMPLEMENTATION_REVIEW.md)

## 1. Executive Summary

Workflow OS can now model, persist, and explicitly enforce approval-presentation
proof for opt-in local executor callers. The repo-local dogfood runner also
persists proof and uses the proof-enforced approval path for material phases.

The remaining P0 governance question is whether ordinary approval paths should
eventually require presentation proof by default.

This plan defines a conservative default-enforcement path. The next
implementation should not flip default behavior globally. It should add a
small runtime policy boundary that lets callers opt into default-like
enforcement for selected approval surfaces, then gather evidence before any
public default changes.

This plan does not implement code, change default approval behavior, add UI
cards, add schemas, add CLI mutation behavior, enable provider writes, or
change release posture.

## 2. Goals

- Prepare ordinary approval paths to require durable presentation proof where
  explicitly configured.
- Preserve existing approval semantics until a reviewed compatibility gate says
  a default change is safe.
- Prevent vague approval decisions for configured high-risk approval surfaces.
- Reuse existing `ApprovalPresentationRecord`,
  `ApprovalPresentationRecordStore`, and
  `LocalExecutor::decide_approval_with_presentation(...)` behavior.
- Keep enforcement local, deterministic, fail-closed, and redaction-safe.
- Separate presentation proof enforcement from approval-card UI.
- Prepare high-assurance approvals and write-capable adapter work without
  implementing them in this phase.
- Preserve auditability by proving what bounded approval content was presented
  before approval.

## 3. Non-Goals

This plan does not authorize:

- implementation in this planning phase;
- global default approval behavior changes;
- automatic approval;
- hidden approval;
- model self-review as approval proof;
- public approval-card UI;
- hosted approval review;
- workflow schema fields;
- CLI mutation behavior;
- examples;
- high-assurance approval integration;
- WorkReport citation changes;
- provider writes;
- side effects;
- automatic report artifact writing;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 4. Current Baseline

Implemented:

- bounded approval-presentation core model;
- deterministic content hash;
- local approval-presentation record store;
- explicit opt-in executor approval path requiring matching proof;
- optional freshness policy for explicit callers;
- dogfood phase-start proof persistence;
- dogfood proof-enforced grant and denial paths;
- proof marker inspect and report/audit projection foundations.

Not implemented:

- default public approval enforcement;
- a configuration surface for requiring proof on ordinary approvals;
- approval-card rendering;
- workflow-declared approval-presentation requirements;
- high-assurance approval proof integration;
- default executor proof-marker citation behavior;
- automatic artifact writing;
- CLI approval UX.

## 5. Recommended Enforcement Strategy

Do not change `LocalExecutor::decide_approval(...)` globally in the first
default-enforcement implementation.

Instead, add an explicit local runtime policy boundary such as:

- `ApprovalPresentationDefaultEnforcementPolicy`;
- `ApprovalPresentationDefaultEnforcementMode`;
- `ApprovalPresentationDefaultEnforcementContext`;
- `LocalExecutor::decide_approval_with_default_presentation_policy(...)`, only
  if an executor method is the smallest idiomatic shape.

The first implementation should let a caller say:

- proof is not required;
- proof is required for this approval decision;
- proof is required only when the approval has a high-assurance or
  write-adjacent posture that is already known to the caller.

The policy should delegate to the existing opt-in proof-enforced path when
proof is required. It should delegate to the existing approval path when proof
is not required.

## 6. Enforcement Modes

Recommended initial modes:

- `NotRequired`: preserve existing approval behavior.
- `Required`: require matching durable presentation proof before approval
  events are appended.
- `RequiredForSensitiveAction`: require proof only when the caller supplies a
  bounded sensitive/write-adjacent approval posture.

Do not add an `Automatic` or `InferFromText` mode. The kernel must not infer
sensitivity from raw approval reasons, chat text, source contents, provider
payloads, or model opinion.

## 7. Required Inputs

When proof is required, the caller must provide:

- run ID;
- approval ID;
- ordinary approval decision input;
- approval-presentation proof reference, preferably presentation ID;
- optional freshness policy;
- explicit enforcement mode;
- bounded reason for why proof is required.

The caller must not provide raw handoff text, chat transcripts, screenshots,
provider payloads, command output, raw spec contents, source contents, tokens,
credentials, or unbounded natural language.

## 8. Fail-Closed Rules

When proof is required, approval must fail before mutation if:

- no proof is supplied;
- no matching proof exists;
- proof is stale under the supplied freshness policy;
- proof was presented after the decision timestamp;
- proof identity does not match the pending approval request;
- proof is ambiguous;
- proof is corrupt;
- proof redaction or sensitivity metadata is invalid;
- proof validation cannot be completed safely.

Failure must not append approval events, resume the run, invoke skills, append
side-effect events, write artifacts, or perform provider calls.

## 9. Compatibility And Migration Posture

Default public approval behavior should remain unchanged until all of the
following are true:

- the explicit policy boundary is implemented and reviewed;
- dogfood usage has proved the proof-enforced path across grants and denials;
- high-assurance approval integration has an accepted plan;
- write-adjacent approval surfaces have clear authority and evidence
  requirements;
- public docs explain how proof-required approvals differ from ordinary
  approvals;
- tests prove non-proof paths remain unchanged when policy says proof is not
  required.

The first implementation should be opt-in through explicit caller input, not
schema configuration, runtime config, CLI flags, or ambient defaults.

## 10. Relationship To Dogfood

Dogfood already uses proof-enforced approval for material phases. It should
continue to be the benchmark for the default-enforcement policy, but dogfood
behavior must not be mistaken for public default behavior.

Future dogfood phases can use the new policy boundary if it removes custom
dogfood-only plumbing without weakening proof enforcement.

## 11. Relationship To High-Assurance Approvals

High-assurance approvals should eventually require presentation proof, but this
phase should not implement high-assurance integration.

Future integration should decide:

- which high-assurance approval controls require proof;
- whether freshness is mandatory;
- whether requester/approver separation is required before proof validation;
- how proof posture appears in reports and artifacts;
- whether proof is required for approval denial as well as approval grant.

## 12. Relationship To Provider Writes

Provider writes and write-capable adapters should not proceed on the basis of
ordinary approval events alone.

For write-adjacent paths, approval-presentation proof should become part of the
pre-provider gate posture before a provider call is attempted. This plan does
not implement provider writes or change any provider-call behavior.

## 13. Error Handling

Use stable, non-leaking errors.

Candidate code families:

- `approval_presentation_default_enforcement.policy_invalid`;
- `approval_presentation_default_enforcement.proof_required`;
- `approval_presentation_default_enforcement.proof_not_required`;
- `approval_presentation_default_enforcement.proof_missing`;
- `approval_presentation_default_enforcement.proof_failed`;
- `approval_presentation_default_enforcement.sensitive_posture_missing`;

Errors must not include raw approval IDs, presentation IDs, paths, handoff
text, approval reasons, command output, provider payloads, source snippets,
tokens, credentials, or secret-like values.

## 14. Test Plan

Future implementation tests should cover:

- `NotRequired` preserves existing approval behavior;
- `Required` delegates to the proof-enforced path;
- missing proof fails closed before approval events;
- stale proof fails closed when max age is supplied;
- mismatched proof fails closed;
- corrupt proof errors are stable and non-leaking;
- policy Debug output is redaction-safe;
- serialized policy values are bounded;
- grant and denial paths both honor required proof;
- no run resume occurs after proof failure;
- no skill invocation occurs after proof failure;
- no side-effect event is appended after proof failure;
- no provider call is attempted after proof failure;
- no report artifact is written after proof failure;
- existing `decide_approval(...)` tests continue to pass unchanged.

## 15. Proposed Implementation Sequence

1. Add default-enforcement policy model and helper, no executor behavior change.
2. Add focused tests for policy validation, redaction, and non-leakage.
3. Add an explicit executor-adjacent helper that delegates to existing approval
   or proof-enforced approval based on caller-supplied policy.
4. Review.
5. Only after review, consider high-assurance approval integration.
6. Only after high-assurance review, consider write-adjacent provider gate
   integration.
7. Defer public default behavior changes until dogfood and compatibility
   evidence justify them.

## 16. Open Questions

- Should proof-required approval be represented as policy input, runtime input,
  or a future workflow-declared requirement?
- Should proof be required for denial decisions as well as grant decisions in
  public default paths?
- Should freshness be mandatory for write-adjacent approvals?
- Should high-assurance approval controls own the proof-required switch?
- How should default enforcement be surfaced in human-readable approval output
  without implementing UI cards too early?
- What is the minimum compatibility evidence needed before changing public
  default approval behavior?

## 17. Final Recommendation

Next implementation phase: default approval-presentation enforcement policy
model/helper only.

Do not implement global default approval behavior changes, UI approval cards,
workflow schema fields, CLI mutation behavior, high-assurance integration,
provider writes, side effects, hosted behavior, reasoning lineage, examples, or
release posture changes in the next phase.
