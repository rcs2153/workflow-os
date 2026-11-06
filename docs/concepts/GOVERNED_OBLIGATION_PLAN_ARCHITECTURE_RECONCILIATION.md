# Governed Obligation Plan Architecture Reconciliation

Status: **Phase 1 complete; a narrow model gap is demonstrated**

## Scope

This Phase 1 architecture reconciliation tests whether Workflow OS needs a
derived Governed Obligation Plan or whether current Core projections already
provide one deterministic pre-execution inventory of the requirements that
must be established before governed work may proceed or complete.

The answer is deliberately narrow:

- Core already owns the canonical contracts, runtime decisions, evidence,
  durable events, report artifacts, and terminal state.
- Core does not currently expose one deterministic, source-traceable inventory
  that assembles those distributed **static requirements** before execution and
  can later be compared with authoritative terminal evidence.
- A private, non-authorizing requirement index is therefore justified for
  further model-only evaluation.
- That index must not become a second policy engine, approval system, evidence
  store, completion model, authored workflow language, or source of runtime
  satisfaction.

This reconciliation changes no runtime, schema, SDK, CLI, dependency,
provider, persistence, or public API behavior.

## User And Runtime Failure

An operator can inspect workflow definitions, resolved skills and policies,
required-context contracts, check declarations, approval records,
SideEffects, run events, and WorkReports. The surfaces are individually useful,
but answering the following question still requires bespoke knowledge of
several Core paths:

> Before execution, what exactly must this governed unit establish, where did
> each requirement come from, and which authoritative terminal evidence later
> proves satisfaction, denial, deferral, or unsupported posture?

`proportional_governance_workflow_derivation` is the closest existing static
projection. It derives a per-step governance assessment input from validated
workflow, skill, policy, and definition-root data while leaving runtime facts
explicitly unknown. It does not inventory required context, canonical local
checks, report obligations, or all other canonical requirements as one
cross-contract structure, and it is not a terminal-evidence reconciliation
surface.

WorkReports and terminal projections answer what happened after execution.
They do not provide the complete pre-execution requirement inventory. The gap
is therefore inspectability and traceability, not authority or execution.

## Fact Classes

The proposed inventory may classify facts, but it may not collapse these
classes:

| Fact class | Meaning | Authority |
| --- | --- | --- |
| Authored requirement | A requirement explicitly present in a validated canonical definition. | The referenced canonical definition. |
| Derived static requirement | A deterministic consequence of validated authored contracts and typed effects. | The canonical inputs plus a versioned derivation algorithm; never the projection alone. |
| Runtime fact | Current state observed at or near the protected action boundary. | The existing Core runtime source that produced it. |
| Runtime satisfaction or denial | An authoritative decision that a requirement passed, waited, denied, failed, expired, or was revoked. | Existing policy, authority, approval, evidence, SideEffect, check, and completion paths. |
| Durable evidence/reference | A payload-minimal reference to an authoritative record or artifact. | The existing durable record and its integrity contract. |
| Disclosure-only posture | Information useful to an operator that does not enforce or satisfy a requirement. | The canonical disclosure producer. |
| Unsupported/deferred | A requirement or mapping that Core cannot currently establish. | Fail-closed posture; no inferred satisfaction. |

## Concept-To-Core Matrix

| Proposed concept | Canonical Core source or path | Current posture and fact class | Gap, compatibility, and duplication verdict |
| --- | --- | --- | --- |
| Contract | `ProjectBundle` and `LoadedSpec` in `crates/workflow-core/src/loader.rs`; `ProjectManifest` and policy documents in `project.rs`; `WorkflowDefinition`, `StepDefinition`, and `SkillDefinition` in `definitions.rs` | Validated authored requirements. | No new authored contract is justified. A separate `WorkContract` would duplicate the canonical source language and is rejected. |
| Validated project bundle | `ProjectBundle`, `LoadedSpec`, and `load_project` in `loader.rs`, followed by `validate_project_bundle` | Validated canonical input closure. | The future index must accept validated inputs only and preserve their exact commitments. It must not parse a second source tree. |
| Workflow and step | `WorkflowDefinition` and `StepDefinition` in `definitions.rs` | Authored workflow topology, skill references, policy requirements, local-check declarations, and terminal behavior. | No semantic gap. These remain canonical; the index may only reference their stable identities and commitments. |
| Resolved skill closure | `SkillDefinition` plus project resolution used by `derive_workflow_step_governance_assessment_input` in `proportional_governance_workflow_derivation.rs` | Validated authored requirements and derived static input. | Reuse the existing resolver and error posture. Copying its resolution rules would create drift and is rejected. |
| Resolved policy closure | `PolicySpecDocument` in `project.rs`, typed `PolicyEffect` and `PolicyEffectSet` in `policy.rs`, and existing project resolution | Authored policy requirements become typed derived static requirements; raw rule shells alone are not runtime authority. | The index must consume validated typed effects, not reinterpret `PolicyRuleShell.effect` strings. Policy evaluation and `PolicyDecision` remain authoritative. |
| Typed resource | Existing resource references in required context, SideEffects, provider-specific declarations, current-authority inputs, and immutable bundles | Mixed authored requirement and runtime fact depending on the owning contract. | There is no justified universal replacement resource type. The index may carry a tagged canonical source reference; inventing a parallel resource hierarchy is deferred. |
| Effect | `PolicyEffect`, `PolicyEffectSet`, SideEffect records, and provider write contracts | Policy effects are derived static requirements; SideEffects are runtime facts and durable records. | These are not interchangeable. A policy effect may require or deny behavior; a runtime SideEffect proves an attempted or completed governed effect. No generic effect evaluator is justified. |
| Obligation | Distributed canonical requirements in workflow/step definitions, required context, typed policy effects, canonical check declarations, approval policy, and WorkReport contracts | Authored or derived static requirement only. | This is the genuine model gap: no one deterministic inventory currently assembles and traces all supported requirements. The future model may index them but may not carry satisfaction. |
| Required context | `RequiredContextRequirement`, `RequiredContextContractBinding`, consumption input/result, satisfaction, and gap types in `required_context.rs` | Authored requirement plus authoritative runtime satisfaction or gap. | Inventory the requirement and source binding; reference the existing consumption result at runtime. Do not re-evaluate context in the plan. |
| Check | Step `local_check_requirements` in `definitions.rs`; `CanonicalLocalCheckDeclarationSetRecord` and reference in `local_check_declaration_set.rs`; `LocalCheckResultReference` in `local_check.rs` | Authored or canonicalized static requirement, followed by durable runtime evidence/reference. Some vocabulary remains deferred unless composed through a supported runtime path. | Inventory only canonicalized declarations and disclose unsupported posture. Existing attestation and result verification remain authoritative. |
| Approval | Step approval policy, `ApprovalPresentationRecord` and `ApprovalDecisionProofMarker` in `approval_presentation.rs`, `ApprovalDecision` in `runtime.rs`, and high-assurance validation paths | Authored or derived static requirement; durable presentation evidence; authoritative runtime grant or denial. | The index may say approval is required and cite its source. It must never say approval is granted, current, or sufficient. Those remain protected-time runtime decisions. |
| Evidence | Required-context results, local-check references, approval proof markers, SideEffect references, durable events, authority receipts, and WorkReport citations | Durable evidence/reference or disclosure-only posture, depending on the producer. | No generic evidence truth store is justified. The index may specify expected evidence classes and later link to existing references without copying payloads or deciding validity. |
| SideEffect | `SideEffectRecord`, lifecycle transitions, approval linkage in `side_effect.rs`, and store-backed discovery in `side_effect_discovery.rs` | Runtime fact, runtime satisfaction or failure, and durable evidence/reference. | Current observation does not prove an authored SideEffect obligation. Until a canonical authored source exists, the inventory must mark such a requirement unsupported/deferred rather than infer it from a write-capable skill or observed record. |
| Proportional-governance assessment | `derive_workflow_step_governance_assessment_input` in `proportional_governance_workflow_derivation.rs` and immutable assessment binding in `proportional_governance_immutable_bundle.rs` | Deterministic derived static requirement input plus explicitly unknown runtime facts. | This is canonical prior art and should be an input or shared derivation primitive. A wrapper that independently recomputes action class, minima, sensitivity, or policy closure is rejected. |
| Immutable run binding | `ImmutableRunBundleBinding`, definition references, manifest, execution posture, and activation in `immutable_run_bundle.rs` | Durable commitment to exact execution inputs and posture. | Any run-bound obligation inventory must commit the derivation algorithm version and complete canonical source closure inside this existing boundary. Mutable-checkout rederivation is forbidden. |
| Program | Existing validated workflow graph and immutable run bundle | Authored topology plus exact run-bound execution input. | A public Governed Work Program or bytecode would duplicate the workflow/runtime model and is rejected. The private inventory is not executable. |
| Evaluation | Existing policy engine, required-context consumption, local-check verification, approval/current-authority validation, SideEffect integrity, executor transitions, and report gates | Authoritative runtime satisfaction, denial, waiting, or failure. | No standalone evaluator is justified. A future explanation projection may compare independently specified expectations with these outcomes, but disagreement fails closed against Core. |
| Durable event history | `WorkflowRunEventKind`, event validation, snapshots, and status projection in `runtime.rs`, with backend stores | Authoritative runtime facts and durable transition evidence. | The index may cite event identities after execution. It must not duplicate the ledger or manufacture events. |
| WorkReport contract | `WorkReportContract`, section/citation/disclosure requirements, generation inputs, artifact integrity gates, and artifact records in `work_report.rs` | Authored report requirement, derived static requirement, durable evidence/reference, and disclosure. | Inventory report obligations and canonical source references. Existing report generation, citation validation, identity binding, and artifact persistence remain authoritative. |
| Terminal completion | `RunCompleted`/`RunFailed` transition rules and status projection in `runtime.rs`; terminal report composition in `executor.rs` and `work_report.rs` | Authoritative runtime satisfaction or denial plus terminal evidence. | Completion is not a static field that the plan can set. The inventory may name required completion conditions and later reconcile references to canonical terminal state. A parallel completion model is rejected. |

## Source-Closure Requirements

A future private model is safe only if each entry contains enough identity to
return to its canonical source without copying governance authority. At
minimum, a run-bound commitment would need:

- derivation algorithm identity and version;
- validated project and schema identity;
- workflow, step, and resolved skill commitments;
- complete resolved policy closure and typed-effect commitment;
- required-context contract commitments;
- canonical local-check declaration-set commitments;
- report contract and artifact-requirement commitments;
- proportional-governance assessment input or immutable assessment binding;
- any other canonical source explicitly included by the derivation; and
- deterministic ordering and duplicate/conflict rejection;
- one aggregate source-closure commitment over the complete sorted canonical
  input set; and
- one aggregate inventory commitment over the algorithm identity, source-
  closure commitment, and complete sorted derived entry set.

The source closure is not satisfied by workflow ID/version alone. Runtime code
must not rederive an accepted inventory from mutable checkout files.
Per-entry source references are also insufficient: without aggregate closure
and inventory commitments, a derivation could silently omit one mandatory
entry while every retained entry remained individually well formed.

## Minimal Model Boundary Justified By This Review

Phase 1 supports evaluating a private model with only:

- deterministic obligation identity derived from canonical source identity,
  source commitment, and a closed obligation kind;
- closed, Core-owned obligation kind rather than caller-authored text;
- mandatory, optional, unsupported, or unknown posture;
- canonical source reference and source commitment;
- deterministic ordering key;
- a closed reference to an existing authoritative evidence class, only when
  Core already defines one;
- explicit derivation-algorithm identity;
- aggregate canonical source-closure commitment; and
- aggregate inventory commitment.

Constructors must derive identities and commitments from validated canonical
inputs. Callers may not supply arbitrary obligation IDs, kinds, evidence-class
strings, source-closure roots, or inventory roots.

The model must not contain:

- policy allowed or denied;
- approval granted, denied, current, expired, or revoked;
- evidence passed or failed;
- local check passed or failed;
- SideEffect attempted, completed, or failed;
- authority available or consumed;
- run completed or failed; or
- caller-authored satisfaction state of any kind.

Those are runtime outcomes owned by existing Core paths.
The model must also distinguish an authored requirement whose runtime support
is deferred from the absence of any authored requirement. The latter cannot be
promoted into an obligation merely by assigning it `unsupported` posture.

## Differential Validation Boundary

Later differential validation is useful only if the expected inventory is
specified independently from the derivation implementation. Required negative
fixtures must include at least:

- a mandatory policy or approval gate omitted from the derived inventory while
  the canonical runtime still denies or waits;
- a required-context obligation missing from the derived inventory;
- a local-check declaration whose unsupported posture is incorrectly presented
  as satisfied;
- a runtime SideEffect incorrectly promoted into an authored obligation;
- a changed canonical input closure presented under an old inventory
  commitment; and
- identical canonical inputs producing non-identical semantic ordering.

Using the same helper to produce the plan and the expected fixture is circular
and does not satisfy this boundary.

## Compatibility And Migration

No migration is justified in Phase 1. Existing projects, workflows, policies,
state stores, immutable bundles, reports, and event streams remain canonical
and unchanged. If a model-only phase is later accepted, absence of a derived
inventory must preserve current behavior; it cannot make existing runs more
permissive or silently reinterpret historical state.

## Verdict

There is a genuine but bounded gap: Workflow OS cannot currently present all
supported pre-execution governance requirements as one deterministic,
source-traceable inventory and reconcile that inventory with terminal evidence
without bespoke cross-module knowledge.

The gap justifies a separately reviewed **private model-only requirement
index**. It does not justify a new authored language, universal resource/effect
model, evaluator, satisfaction store, completion engine, public CLI/SDK/schema,
incremental query engine, or Salsa dependency.

Phase 2 remains unauthorized until a focused maintainer/security review accepts
this matrix and its source-of-truth boundaries.

## Validation

- inspected the canonical Core sources named in the matrix;
- checked complete required concept and fact-class coverage;
- `npm run check:docs`;
- `git diff --check`.

Runtime tests are intentionally out of scope because this phase changes only
architecture documentation.

## Governed Phase Record

- Workflow: `dg/d`
- Run: `run-1793950736150090000-2`
- Approval: `approval/run-1793950736150090000-2/planning-approved`
- Approval presentation: `presentation/5d42bc0fbf6d2296`
- Presentation content hash:
  `5d42bc0fbf6d2296eb29f403fae558cf6c1c07a237273fa306af365dcbdbcb95`
- Approval outcome: granted under delegated-maintainer authority after the
  complete proof-backed handoff was presented
- Governed scope: Phase 1 architecture reconciliation only
- Phase result: completed
- Event summary: 39 ordered events, one approval request, one approval grant,
  eight policy decisions, six scheduled steps, six requested, started, and
  succeeded skill invocations, zero retries, and zero escalations
- Approval presentation enforcement: `proof_record_present_granted_approval_seen`;
  one persisted record matched the run, while the current inspect projection
  does not expose a separate proof-use event marker

Workflow OS governed scope and approval. Source inspection, documentation
authoring, validation, Git, PR, and merge actions occur outside the kernel and
must remain disclosed. The run did not simulate runtime enforcement, provider
actions, independent review, or terminal evidence reconciliation.
