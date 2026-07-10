# Provider-Write Approval-Presentation Adoption Plan

Status: Planning complete. The first explicit GitHub PR comment provider-write
approval-presentation gate is implemented in
[Provider-Write Approval-Presentation Gate Implementation Report](../concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_IMPLEMENTATION_REPORT.md).
Default executor writes, hidden auth loading, CLI mutation behavior, schemas,
examples, hosted runtime, reasoning lineage, and release posture changes remain
unimplemented.

## 1. Executive Summary

High-assurance approval-presentation adoption is implemented and reviewed. The
first explicit provider-write path now exists as a local, opt-in,
injected-provider GitHub PR comment helper, with reconciliation, event-proof,
gate-clarity, and recovery posture foundations.

The next question is where approval-presentation proof must be required before
write-adjacent or provider-call work can proceed.

This plan defines the adoption boundary. It does not implement provider writes,
change default executor behavior, add CLI mutation behavior, add schemas,
update examples, introduce hosted runtime, implement reasoning lineage, or
change release posture.

## 2. Goals

- Require durable approval-presentation proof for selected write-adjacent and
  provider-call surfaces.
- Preserve existing default approval behavior.
- Preserve explicit opt-in provider-write behavior.
- Validate proof before provider invocation, artifact writes, or
  write-adjacent event append.
- Keep proof validation stable, deterministic, and non-leaking.
- Keep provider-write reconciliation semantics intact.
- Ensure report/artifact/event proof paths can cite presentation proof markers
  without copying approval-presentation payloads.
- Prepare a small implementation phase that composes existing primitives rather
  than creating new governance vocabulary.

## 3. Non-Goals

This plan does not authorize:

- implementation in this prompt;
- new provider-write capabilities;
- default executor provider writes;
- automatic provider calls;
- hidden auth loading;
- automatic retries;
- automatic repair;
- automatic report generation;
- automatic report artifact writing;
- CLI mutation behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime;
- enterprise RBAC, IdP, quorum approval, or revocation;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. Current Baseline

Implemented foundations:

- approval-presentation records and durable local proof store;
- proof-enforced approval decisions;
- proof markers on approval decision events;
- proof marker inspect/report/audit/artifact gate foundations;
- default approval-presentation enforcement policy/helper;
- high-assurance approval-presentation policy path;
- high-assurance approval disclosure integration;
- SideEffect core model and lifecycle transitions;
- GitHub PR comment proposed SideEffect records;
- approval-side-effect linkage;
- provider-call orchestration with injected provider/auth boundary;
- executor-integrated GitHub PR comment provider-write helper;
- provider-write reconciliation and recovery posture helpers;
- provider event-proof and report artifact gates.

Still not implemented by this plan:

- default public approval-presentation enforcement;
- provider-write approval-presentation proof adoption;
- approval-card UI;
- workflow-declared approval-presentation requirements;
- CLI mutation behavior.

## 5. Candidate Adoption Surfaces

### Provider-Call Orchestration Input

Classification: first implementation target.

Reason: this is the last local gate before an injected provider may perform an
external mutation. Approval-presentation proof must be validated before the
provider call, not inferred afterward from approval IDs or report disclosures.

### Executor-Integrated GitHub PR Comment Provider-Write Helper

Classification: first implementation target.

Reason: this is the explicit executor-adjacent path that composes local
execution with a provider call. It should expose a proof-required variant or
input field that fails before provider invocation when presentation proof is
missing, stale, mismatched, or scoped to the wrong approval.

### Approval-Resume Artifact/Projection Composition

Classification: already adjacent, defer direct changes unless required.

Reason: artifact/projection composition already consumes proof markers and
gate helpers. Provider-call adoption should produce proof posture before this
path depends on it more broadly.

### Report Artifact Writes

Classification: defer.

Reason: report artifact paths already have proof-marker gates. They should
continue to consume durable proof-marker projections, not become the first
provider-write presentation enforcement point.

### Ordinary Approval Decisions

Classification: defer.

Reason: default public approval behavior remains unchanged. Ordinary approvals
should not become proof-required globally until explicit write-adjacent paths
prove the migration shape.

### CLI Mutation Commands

Classification: reject for first adoption.

Reason: CLI mutation behavior remains unimplemented and should not be added as
part of approval-presentation adoption.

## 6. First Implementation Target Recommendation

The first implementation should add an explicit provider-write
approval-presentation gate for GitHub PR comment provider-call orchestration.

Recommended shape:

- add a small input wrapper or optional policy field that pairs existing
  GitHub PR comment provider-write inputs with
  `ApprovalPresentationDefaultEnforcementPolicy`;
- require `ApprovalPresentationSensitiveActionPosture::WriteAdjacent` for
  `RequiredForSensitiveAction`;
- validate proof before `orchestrate_github_pr_comment_provider_call(...)`
  can invoke the provider;
- expose gate clarity for approval-presentation proof as satisfied, blocked,
  not evaluated, or not required;
- return a structured non-leaking provider-write error when proof blocks the
  call.

The first implementation must not add a new provider capability. It should
only gate the already-explicit GitHub PR comment provider-write path.

## 7. Required Composition Order

The implementation must validate approval-presentation proof before any
provider call.

Required order:

1. execute or rehydrate the local workflow run through the existing explicit
   path;
2. validate terminal status requirements;
3. validate existing provider-write preflight/store/SideEffect inputs;
4. validate approval-side-effect linkage and high-assurance disclosure when
   required by existing gates;
5. evaluate approval-presentation policy;
6. require `WriteAdjacent` posture for the write-adjacent path;
7. resolve and validate durable presentation proof when proof is required;
8. attach or expose proof-marker posture for downstream report/audit/artifact
   paths;
9. only then call the injected provider;
10. perform existing reconciliation and recovery handling.

Failures before step 9 must not call providers, mutate provider state, append
provider outcome events, write report artifacts, or retry.

## 8. Enforcement Policy Rules

For provider-write adoption:

- `NotRequired` may remain available only for compatibility and explicit
  non-sensitive test fixtures.
- `Required` requires matching durable presentation proof before provider
  invocation.
- `RequiredForSensitiveAction` must require
  `ApprovalPresentationSensitiveActionPosture::WriteAdjacent`.
- `HighAssurance` posture should be rejected for the provider-write-specific
  path unless a later phase explicitly composes both high-assurance and
  write-adjacent posture.
- `SideEffect` posture should be reserved for lower-level side-effect
  lifecycle gates and not used as the first provider-call enforcement posture.

Do not infer write-adjacent posture from approval reasons, comments, command
output, provider payloads, source snippets, or model opinion.

## 9. Citation And Disclosure Rules

The provider-write path should cite proof by stable references only.

Rules:

- Do not copy approval-presentation content into provider-write records.
- Do not copy approval-card text into WorkReport sections.
- Do not copy raw approval reasons into provider-write disclosure.
- Do not copy provider payloads, command output, source contents, paths,
  tokens, or credentials into errors or Debug output.
- Preserve proof markers as bounded references.
- Allow WorkReport and artifact paths to cite proof markers only through
  existing proof-marker projection/citation helpers.
- Missing proof must be a blocking gate, not a fabricated citation.

## 10. Error Handling

Errors must use stable codes and fixed messages.

Required error families:

- missing proof when proof is required;
- stale or mismatched proof;
- proof scoped to the wrong run or approval;
- missing sensitive posture when required;
- mismatched sensitive posture for provider-write adoption;
- provider-write proof gate failed before provider call;
- provider-write proof gate blocked report/artifact continuation when
  explicitly selected.

Errors must not include raw run IDs, approval IDs, presentation IDs, actor IDs,
approval reasons, handoff text, touched surfaces, command output, provider
payloads, source snippets, paths, tokens, credentials, or secret-like values.

## 11. Privacy And Redaction

The adoption path must:

- use existing approval-presentation proof records;
- use existing proof validation;
- avoid storing raw presentation payloads in provider-write records;
- avoid copying provider payloads or response bodies;
- avoid hidden auth loading;
- keep Debug output bounded and redaction-safe;
- preserve provider-write reconciliation non-leakage;
- treat report/artifact outputs as sensitive even when all referenced records
  are local.

## 12. Relationship To Provider-Write Reconciliation

Approval-presentation proof is a pre-provider gate. It does not replace
provider-write reconciliation.

After proof succeeds and the provider call occurs, existing reconciliation
rules still apply:

- provider success/local completed;
- provider failure/local failed;
- provider success/local transition failure;
- provider failure/local transition failure;
- provider response ambiguity;
- local state ambiguity;
- provider not called.

If presentation proof blocks the call, the result should disclose provider not
called because the approval-presentation gate blocked execution.

## 13. Relationship To High-Assurance Approval

High-assurance approval-presentation adoption proves presentation enforcement
for high-assurance approval decisions. Provider-write adoption is a separate
write-adjacent posture.

Future phases may require both:

- high-assurance approval validation; and
- write-adjacent approval-presentation proof.

The first provider-write adoption should not assume high-assurance disclosure
is always present. It should compose with high-assurance disclosure only when
the existing provider-write request already requires it.

## 14. Test Plan

Future implementation tests should cover:

- provider-write path with `NotRequired` preserves existing behavior;
- `Required` proof succeeds before provider call;
- missing proof blocks before provider call;
- stale/mismatched proof blocks before provider call;
- `RequiredForSensitiveAction` requires `WriteAdjacent` posture;
- wrong posture blocks before provider call;
- proof validation failure produces provider not called posture;
- provider is not invoked when proof fails;
- successful proof does not change provider reconciliation semantics;
- provider success/local completed still reconciles correctly;
- provider failure/local failed still reconciles correctly;
- provider success/local transition failure remains retry-blocked;
- provider failure/local transition failure remains retry-blocked;
- Debug output does not leak approval IDs, presentation IDs, provider refs,
  comment bodies, paths, tokens, or secret-like values;
- WorkReport/report artifact proof citation uses proof marker references only;
- existing approval-presentation, high-assurance, provider-write, SideEffect,
  report artifact, WorkReport, Diagnostic, validation, adapter telemetry, and
  runtime tests continue to pass;
- `cargo test --workspace` passes.

## 15. Documentation Requirements

Future implementation docs must say:

- selected provider-write approval-presentation adoption is implemented only
  when that phase is complete;
- default public approval behavior remains unchanged;
- the first adoption is explicit and opt-in;
- provider writes remain unavailable through default executor paths;
- hidden auth loading is not implemented;
- automatic retries and repair are not implemented;
- CLI mutation behavior is not implemented;
- schemas and examples are not updated;
- hosted runtime, reasoning lineage, recursive agents, agent swarms, Level 3/4
  autonomy, and release posture changes remain unsupported.

## 16. Proposed Implementation Sequence

1. Add a provider-write approval-presentation gate input/helper around the
   existing GitHub PR comment provider-write path.
2. Validate `WriteAdjacent` proof before provider invocation.
3. Add gate-clarity projection for approval-presentation proof.
4. Add focused tests for success, fail-closed, provider-not-called, and
   non-leakage.
5. Review.
6. Only after review, consider applying the same pattern to artifact/write
   composition or CLI planning.

## 17. Governed Dogfood Run

- workflow_id: `dg/d`
- run_id: `run-1783717204117577000-2`
- approval_id: `approval/run-1783717204117577000-2/planning-approved`
- approval presentation: `presentation/4983cdbb01602bb4`
- approval presentation hash:
  `4983cdbb01602bb427ab28c619c1f137dbb3f6d6fba4c70da6dda681b0a86ba5`
- approval outcome: granted by delegated maintainer
- approval reason: `approved-provider-write-approval-presentation-adoption-planning`

Workflow OS governed the planning approval boundary. Codex performed repository
inspection, documentation authoring, validation, git, and PR work outside the
kernel.

## 18. Validation

Planning validation:

- `npm run check:docs` - passed;
- `git diff --check` - passed.

## 19. Final Recommendation

Proceed next to provider-write approval-presentation gate implementation for
the explicit GitHub PR comment provider-write path.

The implementation must remain local, explicit, opt-in, and fail-closed before
provider invocation when proof is missing or invalid. It must not add new
provider-write capabilities, default executor writes, hidden auth loading,
automatic retries, CLI mutation behavior, schemas, examples, hosted runtime,
reasoning lineage, or release posture changes.
