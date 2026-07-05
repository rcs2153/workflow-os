# Report Artifact Write Integration Plan

Status: Implemented as a local explicit helper in [Report Artifact Write Integration Helper Report](../concepts/REPORT_ARTIFACT_WRITE_INTEGRATION_HELPER_REPORT.md). This plan did not implement runtime behavior.

## 1. Executive Summary

Workflow OS now has a chain of local, explicit report artifact primitives:

- `WorkReport` and `WorkReportArtifactRecord` models;
- local report artifact storage;
- SideEffect citation vocabulary and referential integrity gates;
- approval-linkage gates;
- high-assurance approval disclosure gates;
- workflow-declared artifact gate derivation for high-assurance approval requirements;
- explicit executor artifact path for generic report artifacts;
- GitHub PR comment report artifact citation validation;
- explicit local GitHub PR comment artifact write composition;
- explicit local GitHub PR comment executor-adjacent integration helper.

The next question is how a caller should compose these primitives into one explicit opt-in artifact-write integration boundary without making artifact writing automatic.

This plan defines that boundary. It does not implement provider writes, runtime side-effect execution, automatic artifact writes, CLI mutation behavior, schemas, examples, hosted behavior, reasoning lineage, or release posture changes.

## 2. Goals

- Define the next explicit local artifact-write integration boundary.
- Compose existing report artifact, SideEffect integrity, approval-linkage, high-assurance disclosure, workflow-declared gate derivation, and provider-candidate-specific citation gates.
- Preserve workflow pass/fail semantics.
- Keep artifact writing opt-in and caller-supplied.
- Keep provider writes out of scope.
- Avoid hidden runtime state and hidden runtime config.
- Keep failures structured and non-leaking.
- Prepare a narrow implementation prompt for the next phase.

## 3. Non-Goals

Do not implement:

- provider mutation;
- live GitHub PR comment creation;
- runtime side-effect execution;
- automatic artifact writes from default executor paths;
- automatic report generation for every run;
- CLI mutation behavior;
- workflow schema changes;
- example updates;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 4. Current Primitive Inventory

Already implemented and available for composition:

- explicit local `WorkReport` generation and runtime result exposure;
- explicit executor-integrated report-bearing execution;
- explicit local report artifact store;
- generic report artifact `SideEffect` referential integrity validation;
- explicit artifact write with SideEffect integrity and approval-linkage gates;
- high-assurance approval disclosure gate for report artifacts;
- workflow-declared high-assurance artifact requirement derivation;
- explicit artifact-capable executor path for generic artifact gate policy;
- GitHub PR comment request/preflight/fixture/proposed SideEffect record primitives;
- persisted GitHub PR comment proposed SideEffect record;
- proposed SideEffect event helper and executor append proof;
- GitHub PR comment report artifact citation validation;
- GitHub PR comment report artifact write composition helper;
- explicit local GitHub PR comment artifact integration helper.

The missing piece is a broader integration boundary that chooses and composes these gates from one caller-facing request.

## 5. Proposed Integration Boundary

Add a local explicit helper, not a default executor method, such as:

```text
write_report_artifact_with_integrations(...)
```

or:

```text
compose_explicit_report_artifact_write(...)
```

The exact name should follow existing repository conventions.

The helper should accept:

- terminal `WorkflowRun`;
- validated `WorkReportArtifactRecord`;
- explicit `WorkReportArtifactStore`;
- explicit `SideEffectRecordStore`;
- optional workflow events;
- generic artifact gate policy;
- optional workflow-derived artifact gate policy;
- provider-candidate-specific integration options.

The first provider-candidate-specific option should be GitHub PR comment citation validation using the already-implemented GitHub helper.

## 6. Input Model

The future input type should be explicit and bounded.

Required:

- `run`;
- `artifact`;
- artifact store;
- SideEffect record store;
- generic SideEffect integrity policy;
- approval-linkage policy;
- high-assurance disclosure policy.

Optional:

- workflow definition or already-derived artifact gate policy;
- workflow events;
- GitHub PR comment expected `SideEffectId`;
- GitHub PR comment citation policy;
- future provider-candidate integration selectors.

The helper must not read hidden global state, infer runtime config, or load workflow definitions implicitly.

## 7. Gate Policy Composition

The helper should compose policies by strictness:

- caller-supplied generic artifact policy;
- workflow-derived artifact policy, when explicitly supplied;
- provider-candidate-specific citation policy.

Stricter requirements should win. A disabled caller policy must not weaken an explicitly supplied workflow-derived policy.

Failure must occur before artifact write whenever possible:

- invalid artifact/run identity;
- missing required SideEffect citation;
- missing required SideEffect record;
- missing required accepted event;
- failed approval linkage;
- missing high-assurance disclosure;
- unsupported provider-candidate integration.

## 8. Provider-Candidate Integration Policy

For the GitHub PR comment lane:

- require a stable expected `SideEffectId`;
- optionally require a stored proposed GitHub PR comment `SideEffectRecord`;
- optionally require a matching accepted proposed-event reference;
- reuse `write_github_pr_comment_report_artifact_from_explicit_context(...)`;
- never create `EvidenceReference` values;
- never fabricate IDs;
- never copy provider payloads or comment bodies.

Future provider-candidate integrations should be added only after each candidate has the same local no-provider-write primitives:

- request model;
- preflight;
- fixture validation;
- proposed SideEffect record;
- persistence;
- event projection or accepted event proof;
- report artifact citation validation.

## 9. Workflow Semantics

Artifact-write integration failure must not rewrite workflow execution history.

The helper must not:

- change `WorkflowRun` status;
- mutate snapshots;
- append workflow events;
- emit audit events;
- emit observability events;
- execute side effects;
- call providers;
- create CLI output.

Callers should receive structured success or failure and decide how to surface artifact-write outcomes.

## 10. Failure Behavior

Recommended behavior:

- return `Result<..., WorkflowOsError>` for pure helper use;
- map provider-candidate-specific failures to stable integration error codes;
- preserve underlying gate categories in bounded result accessors where useful;
- do not include run IDs, report IDs, SideEffect IDs, target references, file paths, provider payloads, tokens, snippets, or raw command output in error messages;
- write nothing if a pre-write gate fails;
- allow artifact store write failures to surface as bounded artifact-write failures.

## 11. Storage And Artifact Posture

The next implementation may write a report artifact only when the caller explicitly supplies an artifact store and invokes the helper.

It must not:

- write artifacts automatically from default executor paths;
- create filesystem artifacts outside the configured `WorkReportArtifactStore`;
- add persistence beyond existing local store traits;
- add CLI rendering or export commands;
- change workflow schemas.

## 12. Privacy And Redaction

The helper must use existing validated models and redaction-safe constructors.

It must not store or copy:

- raw provider payloads;
- GitHub comment bodies;
- pull request bodies;
- diffs;
- CI logs;
- command output;
- raw spec contents;
- local paths;
- environment variable values;
- credentials;
- authorization headers;
- tokens;
- private keys.

Debug output must be bounded and redaction-safe.

## 13. Test Plan

Future implementation tests should cover:

- generic artifact write succeeds through explicit integration;
- GitHub PR comment artifact write succeeds through provider-candidate option;
- workflow-derived gate policy cannot be weakened by caller policy;
- missing required SideEffect citation fails before artifact write;
- missing required GitHub proposed event fails before artifact write;
- approval-linkage failure fails before artifact write;
- high-assurance disclosure failure fails before artifact write;
- artifact store failure is bounded and non-leaking;
- helper does not mutate run/snapshot/events;
- helper does not append workflow/audit/observability events;
- helper does not call providers;
- helper creates no CLI output;
- debug output does not leak IDs or payloads;
- existing WorkReport, SideEffect, high-assurance, GitHub PR comment, and executor tests still pass.

## 14. Proposed Implementation Sequence

1. Add the generic explicit artifact-write integration input and result types.
2. Compose generic artifact gates through the existing governed artifact write path.
3. Add an optional GitHub PR comment integration branch that delegates to the existing GitHub helper.
4. Add tests for gate strictness, no-write-on-failure, no runtime mutation, and redaction.
5. Review.
6. Only after review, consider explicit executor result exposure for this integration.
7. Only after separate planning, consider CLI inspection or artifact export.

## 15. Open Questions

- Should the generic integration helper return one unified result type or provider-candidate-specific result variants?
- Should workflow-derived artifact policy be passed as a pre-derived policy or derive from an explicitly supplied `WorkflowDefinition`?
- Should unsupported provider-candidate options fail closed or be ignored when disabled?
- Should artifact store duplicate writes remain hard failures or support explicit idempotent readback?
- When should this become visible through an executor API rather than a standalone helper?

## 16. Validation And Governance

- Dogfood workflow: `dg/d`
- Run: `run-1783224317345176000-2`
- Approval: `approval/run-1783224317345176000-2/planning-approved`
- Approval outcome: granted by delegated maintainer
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations
- Validation: `npm run check:docs` passed

## 17. Final Recommendation

Recommended next implementation phase: generic explicit report artifact write integration helper, local only.

The implementation should compose the already-built artifact gates and optionally delegate to the GitHub PR comment helper. It must remain explicit, fixture-first, no-provider-write, no automatic artifact writing, no CLI mutation, no schemas, no examples, no hosted behavior, no reasoning lineage, and no release posture change.
