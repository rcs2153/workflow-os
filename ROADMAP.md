# Roadmap

Workflow OS grows from the local-first kernel outward.

## Current Status

Status date: 2026-07-29.

Workflow OS has a working local governance kernel, governed sequential multi-step
execution, durable local run/event state, policy and approval gates, evidence and
WorkReport foundations, SideEffect governance, existing-repository onboarding,
and bounded workflow recommendation and authoring paths.

The shared `PostgreSQL` state milestone is accepted. The explicit opt-in
adapter, all seven Core transaction families, compare-and-set revisions,
expiring fenced leases, one shared run-event consumer, projection rebuild,
concurrent CI conformance, and logical backup/restore rehearsal passed local
validation and mandatory live `PostgreSQL` CI proof.

The active lane is now the single-tenant hosted alpha. Its phase-ready boundary
is defined in the
[Single-Tenant Hosted Alpha Plan](docs/implementation-plans/single-tenant-hosted-alpha-plan.md)
and accepted in the
[Single-Tenant Hosted Alpha Plan Review](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_PLAN_REVIEW.md).
The runtime-composition hardening now includes a transport-neutral immutable
bundle store seam, server-owned governed-run creation, proof-enforced approval,
eligible cancellation, bounded event/report/operational inspection, durable
pre-invocation attempt posture, fence-preserving renewal, and atomic
attempt/receipt/work-item terminal commit. Core now also owns an explicit
single-step no-write dispatch path: it derives the hosted request from the
scheduled governed invocation, atomically appends invocation events with the
queued work item, and atomically projects an exactly bound terminal provider
receipt into authoritative workflow events and snapshot state. Callers still
cannot submit hosted work items or provider requests, and only this constrained
Core path may treat the bound receipt as skill-execution evidence.
Core now also owns explicit provider-failure projection. Requests rejected
before provider start atomically fail the invocation and run without creating
an attempt or receipt. A provider outcome that may have started atomically
marks the attempt reconciliation-required, moves the work item to ambiguous,
and escalates the run; it cannot become fabricated success, ordinary failure,
or a blind retry. Exactly bound ambiguous receipts likewise escalate instead
of failing the run.
The hosted deployment/recovery proof now adds one server-owned no-write
workflow fixture, an authenticated API-to-worker governed run, atomic terminal
report-artifact persistence, API/worker restart checks, database-interruption
recovery, and a dedicated Linux compose rehearsal. The existing live
`PostgreSQL` suite remains the source of lease-takeover, stale-fence,
schema-mismatch, backup/restore, projection-rebuild, and immutable-bundle
recovery proof.
The implementation and review are recorded in the
[Hosted Dispatch And Result Projection Report](docs/concepts/SINGLE_TENANT_HOSTED_DISPATCH_RESULT_PROJECTION_REPORT.md)
and
[Hosted Dispatch And Result Projection Review](docs/concepts/SINGLE_TENANT_HOSTED_DISPATCH_RESULT_PROJECTION_REVIEW.md).
The failure/reconciliation hardening is recorded in the
[Hosted Provider Outcome Projection Report](docs/concepts/SINGLE_TENANT_HOSTED_PROVIDER_OUTCOME_PROJECTION_REPORT.md)
and
[Hosted Provider Outcome Projection Review](docs/concepts/SINGLE_TENANT_HOSTED_PROVIDER_OUTCOME_PROJECTION_REVIEW.md).
The single-tenant no-write evaluation milestone is now complete. It is not a
production-readiness claim. Production-suitable identity and authority,
access-material isolation, separate service identities, TLS/network controls,
HA, capacity, and recovery objectives remain future requirements.
The hosted evaluation image
uses the validated Rust 1.95 builder rather than preserving a builder that
cannot compile the lockfile.
Hosted production, automatic backend selection, multi-tenancy, enterprise
identity, OpenShell integration, broader provider mutations, and production
readiness remain excluded.

The first narrow provider-write vertical slice remains accepted and bounded to
a GitHub pull request comment in an explicitly configured live sandbox. It
composes proof-enforced approval presentation, SideEffect lifecycle state,
provider response reconciliation, and durable workflow event proof. No new
provider mutation family should precede acceptance of the hosted alpha's
no-write execution-provider, authority, credential, lease, and recovery
boundaries.

The historical [Next Roadmap Sprint Plan](docs/implementation-plans/next-roadmap-sprint-plan.md)
records an earlier hook-disclosure and local-check sprint. It is retained as phase
evidence but is no longer the source of current sequencing. This section is the
authoritative current queue.

Roadmap delivery now follows the
[Roadmap Vertical-Slice Acceleration Plan](docs/implementation-plans/roadmap-vertical-slice-acceleration-plan.md).
After the currently active governed phase closes, runnable end-to-end
capabilities are the default unit of delivery. Closely related model, adapter,
persistence, runtime-consumer, test, documentation, and review work should be
completed inside one governed milestone rather than automatically becoming
separate micro-phases. Narrow standalone planning or focused review remains
required where unresolved authority, approval, immutable-input, idempotency,
external-effect, migration, concurrency, credential, tenant-isolation, or
recovery risk makes broader implementation unsafe.

Fresh-pull product evaluation on current `main` confirms that conservative
scaffolding, concise first-run posture, recommendation-to-authoring, and
approval/audit boundaries now form a coherent preview experience. The
evaluation also found two bounded credibility defects: integration checks could
surface an opaque child-process result under Node 24, and `validate` rendered a
missing-manifest diagnostic twice. Both are fixed and documented in the
[Fresh-Pull Evaluator UX And Tooling Fix Report](docs/concepts/FRESH_PULL_EVALUATOR_UX_AND_TOOLING_FIX_REPORT.md).
This correction does not change current phase sequencing. The evaluator's
recommendation to reduce low-risk ceremony aligns with the existing
proportional-governance and quiet-success lane.

## Active Phase Queue

1. **Complete governed sandbox proof: implemented.** The accepted path exercised
   explicit target/auth, provider outcome, SideEffect transition, durable event
   proof, and bounded phase disclosure on draft PR #318.
2. **Bind live authority to proof-enforced approval: accepted.**
   The opt-in live-sandbox composition now validates the terminal run,
   approval-presentation proof and decision, and persisted approval/SideEffect
   linkage before deriving `LinkedAndApproved`; missing, stale, mismatched, or
   unlinked authority blocks before provider invocation. The focused review
   found that the supplied run also had to be bound exactly to the executor's
   durable event state. The follow-up now rehydrates through the executor,
   requires exact run equality, and uses durable state for every authority
   check before provider invocation. Re-review found that durable rehydration
   now uses the backend's read-only path rather than the snapshot-projecting
   executor helper, so authority validation does not mutate runtime state. The
   final focused re-review accepted the complete boundary. See the
   [blocker-fix report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_APPROVAL_AUTHORITY_LINKAGE_BLOCKER_FIX_REPORT.md)
   and [focused review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_APPROVAL_AUTHORITY_LINKAGE_BLOCKER_FIX_REVIEW.md).
3. **Define proportional governance before broadening defaults: P0 decision-axis
   correction accepted.** Follow the
   [Proportional Governance And Quiet Success Plan](docs/implementation-plans/proportional-governance-quiet-success-plan.md)
   and the
   [Proportional Governance Read-Only Projection Plan](docs/implementation-plans/proportional-governance-read-only-projection-plan.md).
   The accepted core selector deterministically maps
   profile, policy, authority, evidence/check, sensitivity, SideEffect, prior
   decision, and runtime-escalation posture to quiet capture, visible
   non-blocking disclosure, blocking approval, or denial. No executor or CLI
   behavior consumes this model yet. The additive projection now exposes an
   accepted result as assessed-not-enforced and not-persisted machine-readable
   posture without changing runtime behavior. Focused review found that derived
   enum deserialization could echo an unknown caller-supplied value before the
   projection's fixed non-leaking validation error ran. A projection-specific
   safe wire parser and unknown-value regression matrix fix that boundary, and
   focused re-review accepts the result. Initial core-model review blockers covering
   validated decision deserialization and profile-minimum semantics are fixed
   and accepted.
   External dogfood feedback correctly identified that `VisibleDisclosure`
   conflated operator presentation with execution strictness, and that callers
   must manually manufacture already-classified decision inputs. The corrected
   model now selects proceed/approval/denial independently from quiet/visible
   disclosure, and the read-only projection derives blocking action only from
   execution disposition. Focused review found that the public requirement
   constructor can still produce approval or denial paired with quiet
   disclosure. The blocker fix now normalizes blocking and denied decisions to
   visible disclosure and rejects contradictory serialized accepted decisions.
   Focused re-review accepts the correction. See the
   [blocker-fix review](docs/concepts/PROPORTIONAL_GOVERNANCE_DECISION_AXIS_BLOCKER_FIX_REVIEW.md).
   The first model-only workload-assessment slice is now implemented. Initial
   review found architecture-width fingerprint framing and false
   workflow-declaration reason provenance. The focused blocker fix now uses
   fixed-width framing with a known v1 vector and a distinct
   workload-assessment selector source/reason; focused re-review accepts both
   corrections. It
   derives review-only recommendations from bounded action, authority,
   evidence/check, sensitivity, and SideEffect facts, composes them with all
   explicit minima through the accepted selector, reports unresolved fact
   categories, and binds the result to a versioned payload-free fingerprint
   over the immutable definition root and every decision-relevant input. It is
   assessed, in-memory, not persisted, and not runtime-enforced. Follow the
   [Proportional Governance Decision Axes And Workload Inference Plan](docs/implementation-plans/proportional-governance-decision-axis-and-inference-plan.md).
   The first deterministic declaration-derivation slice is now implemented.
   Focused review found that workflow-level retry and escalation policy
   definitions were omitted from the relevant-definition root. The narrow fix
   now includes those references and proves their invalidation behavior; focused
   re-review accepts the complete derivation boundary. A pure core helper
   resolves one validated workflow step, skill, and referenced policy set into
   bounded assessment input, keeps
   unproven authority/check/reversibility facts explicit, and binds the result
   to the relevant definition hashes so unrelated policy changes do not churn
   the root. It does not scan repositories, change schemas, configure workflows,
   persist decisions, or enforce runtime behavior. The next bounded phase is one
   explicit read-only onboarding recommendation path. That path is now
   implemented in `workflow-os first-run --verbose` and preview JSON: each
   validated workflow step receives a review-only assessment with separate
   execution and disclosure axes, explicit unknown facts and completeness,
   algorithm identity, and a payload-free input fingerprint. The concise human
   first-run summary remains quiet, and every projected result is labeled
   assessed-not-enforced and not persisted. Focused review accepts the boundary
   with non-blocking future coverage for multi-workflow ordering and any later
   cached reassessment path. No YAML configuration, inferred
   authority, runtime enforcement, UI server, or provider mutation is added;
   runtime enforcement remains unauthorized. External feedback correctly
   identifies the remaining build-cache-style gap: relevant changes must
   invalidate an accepted runtime assessment rather than relying on callers to
   remember reassessment. Follow the
   [Runtime Proportional-Governance Reassessment Plan](docs/implementation-plans/runtime-proportional-governance-reassessment-plan.md).
   The first pure helper is now implemented. It derives ordered assessments
   directly from validated stored immutable-run-bundle definitions plus
   exactly one explicit runtime-fact record per workflow step, rejects missing,
   duplicate, extra, or mismatched facts, and emits a versioned fixed-width
   framed aggregate fingerprint. Focused review found that live runtime
   escalation was omitted from the fact boundary. The blocker fix now composes
   explicit runtime escalation monotonically with static declarations and
   proves relevant-definition invalidation plus unreferenced-definition
   stability. Focused re-review accepts the blocker fix. The durable
   assessment-binding model and additive event vocabulary are now implemented.
   The validated payload-free binding records algorithm, immutable bundle
   identity/root, aggregate fingerprint, bounded step count, strictest posture,
   and completeness. An idempotent `GovernanceAssessmentBound` event can retain
   that binding in a run snapshot before validation and projects only bounded
   posture into audit output. The event records an already-established binding;
   it does not make volatile assessment facts durable by itself. One explicit
   opt-in executor path now derives the assessment from the stored immutable
   bundle, validates an optional expected aggregate fingerprint, persists the
   exact binding create-only before `RunCreated`, and emits the binding event
   before run validation and start. Existing executor APIs and defaults remain
   unchanged, and this first integration records rather than enforces the
   assessment disposition. Exact retry and approval-resume reassessment are now
   implemented for that opt-in path: the executor re-reads the stored immutable
   bundle, recomputes from current typed facts, and requires exact durable
   binding equality before rehydration or approval mutation. Fact freshness,
   schemas, CLI behavior, UI, and default enforcement remain later reviewed phases. Initial
   review found that a set derived from one valid bundle could be paired with a
   different valid bundle when workflow/run IDs matched across stores. The
   blocker fix now retains the exact immutable bundle binding in the assessment
   set and requires exact equality during binding construction. Focused
   re-review accepts the fix. The explicit opt-in executor integration is now
   accepted with non-blocking persistence-test follow-ups. Retry/resume
   reassessment hardening is implemented and focused review accepts the complete
   opt-in local path with non-blocking follow-ups for persisted corruption
   coverage and the caller-supplied expected-fingerprint contract. Registered
   current runtime-fact source resolution and bounded freshness validation are
   now implemented and accepted as a Core-owned same-call model/helper boundary.
   The helper binds exact facts to the immutable run bundle, validates the
   stricter source/Core age limit, and returns a payload-free accepted snapshot
   with the assessment. One explicit opt-in local executor path now consumes
   that boundary. It persists or validates the exact immutable bundle, resolves
   current facts once, durably records the resulting assessment binding before
   run events, and returns the accepted payload-free source snapshot. Exact
   retries resolve a new snapshot and must reproduce the durable assessment
   before rehydration; changed facts fail closed without duplicate execution or
   new events. The source snapshot itself is not persisted or reusable
   authority, approval resume does not consume the source yet, and no default
   enforcement, schemas, CLI, provider, OpenShell, SideEffect, or write behavior
   is added. Follow the
   [executor consumer plan](docs/implementation-plans/proportional-governance-runtime-fact-source-executor-consumer-plan.md)
   and [review](docs/concepts/PROPORTIONAL_GOVERNANCE_RUNTIME_FACT_SOURCE_EXECUTOR_CONSUMER_REVIEW.md).
   The accepted initial source observation is now durably committed inside
   assessment-binding V3. The payload-free record binds the trusted source
   registration, exact immutable bundle, initial snapshot, canonical fact set,
   freshness inputs, and resulting assessment aggregate before run events.
   Exact retry still resolves current facts and may accept a new snapshot only
   when the same registered source and bundle reproduce the durable governance
   assessment; it never replaces the initial provenance commitment. Corrupt
   commitment state, changed registration, or changed assessment fails before
   new events or duplicate execution. Approval-resume source consumption,
   reusable authority, raw fact persistence, default enforcement, schemas,
   providers, OpenShell, SideEffects, and writes remain unimplemented. Follow
   the [snapshot commitment plan](docs/implementation-plans/proportional-governance-runtime-fact-snapshot-commitment-plan.md)
   and [review](docs/concepts/PROPORTIONAL_GOVERNANCE_RUNTIME_FACT_SNAPSHOT_COMMITMENT_REVIEW.md).
   The stored durable binding is the mandatory reassessment expectation today;
   a caller fingerprint remains optional additional confirmation.
   Inference may recommend or escalate but may never weaken explicit workflow,
   policy, profile, authority, evidence/check, SideEffect, or steward minima.
   Product configuration should therefore be constraint-first rather than
   classifier-first: onboarding should derive roughly 90 percent of ordinary
   posture from validated repository, workflow, capability, authority,
   evidence/check, sensitivity, and SideEffect facts, while users and future
   stewards declare only unresolved constraints, minimums, and overrides.
   Pure inference is not sufficient for enforcement and cannot replace explicit
   authority or policy. Relevant definition or runtime-fact changes must
   invalidate the prior assessment and trigger deterministic reassessment.
   Operator presentation remains an independent concern: a local UI may show
   quiet-capture decisions live without changing their execution disposition,
   while a policy-required visible disclosure remains a durable obligation even
   when work does not pause. This lane must prove that onboarding, invalidation,
   and presentation boundary before broadening proportional-governance defaults
   or treating additional provider mutations such as PR creation or Jira issue
   creation as automatic extensions of the GitHub comment sandbox.
4. **Approval/resume resolved-context TOCTOU: P0 fixed and accepted.** External
   dogfood review identified, and current-main inspection confirmed, that a
   granted approval can append decision/resume events before current workflow,
   skill, policy, and request-side execution context is proven to match the
   context that paused. Follow the
   [Approval Resume Resolved-Context Integrity Plan](docs/implementation-plans/approval-resume-resolved-context-integrity-plan.md).
   New approvals now carry a versioned payload-free commitment over the
   resolved workflow, skills, referenced policies, request-side checkpoint and
   hook posture, SideEffect input counts, and report-artifact policy posture.
   Every grant path rebuilds and compares that context before any grant-side
   mutation. Legacy missing commitments and changed context fail closed while
   denial remains available. Focused review found and fixed one path-leaking
   reconstruction-error boundary; the implementation is accepted with
   non-blocking follow-ups before its commitment becomes a run-bundle integrity
   root.
5. **Harden immutable run inputs before mutation expansion: explicit executor binding accepted.** Current runs bind
   workflow identity, version, schema version, and spec content hash and reject
   mismatched durable state. External dogfood review correctly identified that
   this is not yet a self-contained immutable run bundle. After the read-only
   projection review, plan the exact validated workflow, policy, skill,
   governance, and configuration references required for later inspection and
   safe replay. The boundary is defined in the
   [Immutable Run Bundle Boundary Plan](docs/implementation-plans/immutable-run-bundle-boundary-plan.md):
   a payload-free manifest plus content-addressed canonical validated workflow,
   skill, and policy records and explicit execution/handler posture. Do not
   claim executable replay or handler attestation before those later boundaries
   are implemented and reviewed. The core manifest model is accepted in
   [Immutable Run Bundle Core Model Review](docs/concepts/IMMUTABLE_RUN_BUNDLE_CORE_MODEL_REVIEW.md);
   The canonical workflow, skill, and policy definition-record model is
   implemented in [Immutable Run Bundle Definition Record Model Report](docs/concepts/IMMUTABLE_RUN_BUNDLE_DEFINITION_RECORD_MODEL_REPORT.md)
   and accepted with non-blocking follow-ups in
   [Immutable Run Bundle Definition Record Model Review](docs/concepts/IMMUTABLE_RUN_BUNDLE_DEFINITION_RECORD_MODEL_REVIEW.md).
   The pure in-memory bundle builder is implemented in
   [Immutable Run Bundle Builder Report](docs/concepts/IMMUTABLE_RUN_BUNDLE_BUILDER_REPORT.md).
   It revalidates a loaded project, selects one workflow plus its resolved
   skills and referenced policies, sources hashes from `LoadedSpec`, and
   returns canonical records with a matching manifest without persistence or
   runtime mutation. The builder is accepted with non-blocking follow-ups in
   [Immutable Run Bundle Builder Review](docs/concepts/IMMUTABLE_RUN_BUNDLE_BUILDER_REVIEW.md).
   The create-only local immutable store is now implemented. Canonical records
   are addressed by their canonical-record hashes and may be reused
   idempotently; one create-only manifest address per run prevents silent
   rebinding. A private store envelope commits the exact canonical-record hashes
   selected for the accepted source-hash manifest references. Complete reads
   validate both identities and fail closed on missing, corrupt, mismatched, or
   ambiguous storage. The manifest envelope is the commit marker, so a failed
   publication cannot create a partially bundled run; harmless unreferenced
   immutable records may remain. Focused maintainer review accepts this storage
   boundary with non-blocking follow-ups. The first explicit opt-in executor
   path is now implemented: it prepares and validates the run, publishes or
   verifies the complete immutable bundle before `RunCreated`, and binds the
   bundle ID, bundle version, and integrity root into durable run identity.
   Exact retries rehydrate the bound run without duplicate execution;
   rebinding, legacy unbundled use through the bundle-required path, and bundle
   persistence failure fail closed. Existing executor APIs and legacy run
   readability remain unchanged. Focused review found and fixed a changed-retry
   posture acceptance bug, then accepted the binding with non-blocking
   follow-ups. Default bundle creation, executable replay, handler/check
   attestation, CLI/schema exposure, scoped authority, and new provider
   mutation families remain unimplemented.
6. **Define scoped runtime authority and capability projection.** After the
   resolved-context and immutable-run boundaries are accepted, follow the
   [Scoped Runtime Authority And Capability Projection Plan](docs/implementation-plans/scoped-runtime-authority-capability-projection-plan.md).
   The first model-only capability-grant and availability slice is implemented
   in [Capability Grant And Availability Core Model Report](docs/concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_REPORT.md).
   Its maintainer review found a source-of-truth blocker: the availability
   record can currently assert authority outcomes without carrying or
   validating authority proof. The required blocker fix is documented in
   [Capability Grant And Availability Core Model Review](docs/concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_REVIEW.md)
   and fixed in [Capability Grant And Availability Core Model Blocker Fix Report](docs/concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_BLOCKER_FIX_REPORT.md).
   The fix restricts availability records to inventory/connectivity facts and
   is accepted in [Capability Grant And Availability Core Model Blocker Fix Review](docs/concepts/CAPABILITY_GRANT_AVAILABILITY_CORE_MODEL_BLOCKER_FIX_REVIEW.md).
   The pure capability resolution helper is implemented in
   [Capability Resolution Helper Report](docs/concepts/CAPABILITY_RESOLUTION_HELPER_REPORT.md)
   and reviewed in
   [Capability Resolution Helper Review](docs/concepts/CAPABILITY_RESOLUTION_HELPER_REVIEW.md).
   The review found one wire-invariant blocker, fixed in
   [Capability Resolution Helper Blocker Fix Report](docs/concepts/CAPABILITY_RESOLUTION_HELPER_BLOCKER_FIX_REPORT.md).
   The fix is accepted in
   [Capability Resolution Helper Blocker Fix Review](docs/concepts/CAPABILITY_RESOLUTION_HELPER_BLOCKER_FIX_REVIEW.md).
   The helper resolves explicit availability,
   grants, actor, resource, workflow, run, step, harness, sensitivity,
   prerequisite, and evaluation-time posture without runtime mutation.
   Availability alone never authorizes, and referenced policy, approval,
   evidence, or check prerequisites remain independent evaluation obligations.
   The bounded capability request model and pure review-only projection are now
   implemented in
   [Capability Request Review Projection Report](docs/concepts/CAPABILITY_REQUEST_REVIEW_PROJECTION_REPORT.md).
   Requests always carry explicit `not_granted` authority posture, reject
   already-authorized resolutions, and cannot activate connectors, expose
   tools, resume runs, or invoke providers. Review projections retain the
   ordered source resolution reasons and fail closed unless their deterministic
   review actions match those reasons. They remain non-authoritative snapshots;
   any future grant issuance or runtime use must resolve current authority from
   fresh explicit inputs rather than trusting a request or projection.
   Focused review found two semantic-binding blockers: request scope was not
   bound to resolution context, and projection posture did not prove its reasons
   were legal. The blocker fix now carries validated actor, capability,
   resource, workflow, run, step, harness, and sensitivity context in every
   resolution; requests require exact context equality; and resolutions plus
   projections share canonical posture/reason validation. Freshness and
   time-of-use re-resolution remain later runtime obligations. Focused
   re-review accepts the fix with non-blocking follow-ups. Pure step-scoped
   capability projection is now implemented in
   [Step-Scoped Capability Projection Report](docs/concepts/STEP_SCOPED_CAPABILITY_PROJECTION_REPORT.md).
   It filters fresh, exact-context capability resolutions into a deterministic
   payload-free set of authorized references for one actor, workflow, run,
   step, and optional harness. Each serialized entry retains its validated
   authorized source resolution, so grant or context substitution fails closed.
   This remains a non-executing model/helper boundary: it does not load tools,
   invoke providers, persist authority, or make a projection sufficient for
   time-of-use authorization.
   Focused review accepts this phase with non-blocking runtime freshness and
   immutable-source follow-ups in
   [Step-Scoped Capability Projection Review](docs/concepts/STEP_SCOPED_CAPABILITY_PROJECTION_REVIEW.md).
   Governed context-access planning is documented in the
   [Governed Context Access Projection Plan](docs/implementation-plans/governed-context-access-projection-plan.md).
   Focused plan review found and corrected three blockers: access-level
   capability mapping was open, serialized gaps were not bound to the complete
   candidate set, and the first stable-target set was deferred to
   implementation. The corrected plan fixes exact capability and resource
   mapping, retains the complete evaluated candidate set for wire
   recomputation, and limits the first model to existing typed Core IDs.
   [Focused review](docs/concepts/GOVERNED_CONTEXT_ACCESS_PROJECTION_PLAN_REVIEW.md)
   accepts the plan. The model-only implementation is now complete in
   [Governed Context Access Projection Report](docs/concepts/GOVERNED_CONTEXT_ACCESS_PROJECTION_REPORT.md).
   It projects only authorized stable references and fixed bounded metadata for
   one exact actor, workflow, run, step, and optional harness. The serialized
   model retains the complete evaluated candidate set and recomputes exact
   entries and bounded gaps. Knowing, citing, or projecting a reference still
   does not authorize target dereference. No source or target payload access,
   runtime consumption, persistence, events, receipts, schemas, SDKs, CLI
   behavior, providers, sandbox integration, SideEffect execution, or writes
   are implemented. Focused review found and corrected two blockers:
   standalone entries could retain unavailable targets, and default enum
   deserialization could echo rejected wire values. The corrected phase is
   accepted with non-blocking follow-ups in
   [Governed Context Access Projection Review](docs/concepts/GOVERNED_CONTEXT_ACCESS_PROJECTION_REVIEW.md).
   Candidate completeness is enforced relative to the supplied candidate set,
   not asserted as global repository or store discovery. Required-context
   contract consumption planning is complete in the
   [Required Context Contract Consumption Plan](docs/implementation-plans/required-context-contract-consumption-plan.md)
   and accepted in its
   [focused plan review](docs/concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_PLAN_REVIEW.md).
   The plan requires exact typed target and access-level matching, immutable
   contract binding, required-gap blocking, optional-gap disclosure, and
   rejection of undeclared projected context. Existing name-only harness
   context declarations are not silently reinterpreted as enforceable
   authority. The core model and pure helper are now implemented in the
   [Required Context Contract Consumption Report](docs/concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_REPORT.md).
   The implementation binds canonical typed requirements to a versioned harness
   contract content hash, consumes only exact same-context projections, blocks
   required gaps, retains optional gaps, and rejects ambient extra context. It
   remains payload-free and does not grant authority, dereference targets,
   integrate with the executor, persist results, emit events, expose schemas or
   CLI behavior, invoke providers or sandboxes, or enable writes. Focused review
   found one blocker: projections were mutually consistent but were not bound to
   an independently declared execution context. The
   [blocker-fix report](docs/concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_BLOCKER_FIX_REPORT.md)
   documents the correction: consumption now retains an explicit actor,
   workflow, run, step, harness, and evaluation time and requires every
   projection to match it. Focused re-review accepts the correction in
   [Required Context Contract Consumption Blocker Fix Review](docs/concepts/REQUIRED_CONTEXT_CONTRACT_CONSUMPTION_BLOCKER_FIX_REVIEW.md).
   Immutable-run binding and time-of-use re-resolution are now planned in
   [Required Context Immutable-Run Binding And Time-Of-Use Plan](docs/implementation-plans/required-context-immutable-run-time-of-use-plan.md).
   The plan keeps the current consumption result explicitly non-authoritative
   for dereference, binds the exact contract and execution scope to a validated
   stored bundle root, and requires fresh same-call capability resolution and
   projection reconstruction before future use. Focused plan review accepts
   the boundary in
   [Required Context Immutable-Run And Time-Of-Use Plan Review](docs/concepts/REQUIRED_CONTEXT_IMMUTABLE_RUN_TIME_OF_USE_PLAN_REVIEW.md).
   The required-context immutable execution-binding core model is now
   implemented in the
   [Required Context Immutable Execution Binding Report](docs/concepts/REQUIRED_CONTEXT_IMMUTABLE_EXECUTION_BINDING_REPORT.md).
   It derives workflow and run identity from a validated stored bundle, proves
   the exact step exists in the canonical frozen workflow, commits actor,
   harness contract identity/version/hash, sensitivity, and binding time, and
   detects serialized substitution without retaining payloads. It is not
   authority, a dereference lease, time-of-use resolution, executor
   integration, persistence, an event, a provider call, sandbox execution, or
   a write. Focused maintainer review accepts the phase with non-blocking
   known-vector and future-consumer provenance follow-ups in
   [Required Context Immutable Execution Binding Review](docs/concepts/REQUIRED_CONTEXT_IMMUTABLE_EXECUTION_BINDING_REVIEW.md).
   The next bounded phase is current authority fact-set planning. It must
   define the complete validated grant, revocation, expiry, availability,
   policy, approval, evidence, check, sensitivity, and SideEffect inputs before
   an authoritative same-call time-of-use result can be implemented. That
   planning boundary is now documented in the
   [Required Context Current Authority Fact-Set Plan](docs/implementation-plans/required-context-current-authority-fact-set-plan.md).
   Focused review accepts the plan with implementation guardrails in
   [Required Context Current Authority Fact-Set Plan Review](docs/concepts/REQUIRED_CONTEXT_CURRENT_AUTHORITY_FACT_SET_PLAN_REVIEW.md).
   The first recommended implementation is the fact-set core model only. It
   must not expose authorization or let arbitrary wire values confer trusted
   completeness; an authoritative time-of-use result remains deferred. The
   model-only implementation is now complete in the
   [Required Context Current Authority Fact-Set Report](docs/concepts/REQUIRED_CONTEXT_CURRENT_AUTHORITY_FACT_SET_REPORT.md).
   It derives the complete query set from the exact contract, commits supplied
   source, grant, and availability inventories, and rejects duplicate,
   out-of-query, incomplete, or tampered records. Claimed completeness remains
   non-authoritative until a future Core-owned source proves it. Focused
   maintainer review accepts the phase after adding the required fixed v1 hash
   vector and framing regression in
   [Required Context Current Authority Fact-Set Review](docs/concepts/REQUIRED_CONTEXT_CURRENT_AUTHORITY_FACT_SET_REVIEW.md).
   The next bounded phase is a Core-owned in-memory current-authority source
   model for tests only. It must own a complete bounded inventory and answer
   the exact derived query without making arbitrary caller slices
   authoritative. That boundary is now specified in the
   [Current Authority In-Memory Source Plan](docs/implementation-plans/current-authority-in-memory-source-plan.md)
   and its
   [planning report](docs/concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_PLAN_REPORT.md).
   Focused review accepts the corrected private source boundary in
   [Current Authority In-Memory Source Plan Review](docs/concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_PLAN_REVIEW.md).
   The first implementation is now complete in the
   [Current Authority In-Memory Source Report](docs/concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REPORT.md).
   It remains private, test-only, synchronous, and incapable of returning
   readiness or dereference authority. Focused implementation review accepts
   the source trust, completeness, selection, determinism, privacy, and
   non-authority boundaries in the
   [Current Authority In-Memory Source Review](docs/concepts/CURRENT_AUTHORITY_IN_MEMORY_SOURCE_REVIEW.md).
   Pure same-call time-of-use resolver planning is complete in the
   [Current Authority Same-Call Time-Of-Use Resolver Plan](docs/implementation-plans/current-authority-same-call-time-of-use-resolver-plan.md)
   and its
   [planning report](docs/concepts/CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_PLAN_REPORT.md).
   The plan preserves the accepted trust boundary: the public caller-owned
   fact-set commitment cannot confer readiness. The first implementation stays
   private and test-only, owns complete grant, availability, and context
   reference inventories, and composes capability resolution, context
   projection, and required-context consumption in one non-reusable call.
   Unresolved policy, approval, evidence, or check prerequisites never project
   authority: required obligations block, while optional obligations remain
   explicit non-blocking gaps. Focused review accepts that boundary in the
   [Current Authority Same-Call Time-Of-Use Resolver Plan Review](docs/concepts/CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_PLAN_REVIEW.md).
   The private test-only implementation is now complete and documented in the
   [Current Authority Same-Call Time-Of-Use Resolver Report](docs/concepts/CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_REPORT.md).
   It queries complete private authority and reference inventories, invokes
   existing capability resolution, rebuilds projections, and reruns
   required-context consumption in one non-reusable call. The next bounded
   phase was accepted in the
   [Current Authority Same-Call Time-Of-Use Resolver Review](docs/concepts/CURRENT_AUTHORITY_SAME_CALL_TIME_OF_USE_RESOLVER_REVIEW.md).
   The next bounded phase is production current-authority source boundary
   planning only. That boundary is now specified in the
   [Production Current-Authority Source Boundary Plan](docs/implementation-plans/production-current-authority-source-boundary-plan.md)
   and its
   [planning report](docs/concepts/PRODUCTION_CURRENT_AUTHORITY_SOURCE_BOUNDARY_PLAN_REPORT.md).
   The plan requires Core-owned source registration, exact-query completeness,
   one coherent snapshot or high-watermark, explicit freshness, fail-closed
   concurrency, and stable non-leaking failures. Focused review accepts the
   plan after clarifying that an opaque watermark proves identity/change, not
   monotonic ordering, and that Core-owned freshness policy must cap any
   source validity claim. The first model-only boundary is now implemented in
   the
   [Production Current-Authority Source Boundary Model Report](docs/concepts/PRODUCTION_CURRENT_AUTHORITY_SOURCE_BOUNDARY_MODEL_REPORT.md).
   It defines bounded source identity and registration commitments, exact
   immutable request commitments, coherent payload-free snapshots, opaque
   watermark identity, optional source-defined generation, completeness,
   consistency, stricter-of-source-and-Core freshness, and stable failure
   posture. Public model construction does not authenticate a source or confer
   readiness. Focused review accepts the model in the
   [Production Current-Authority Source Boundary Model Review](docs/concepts/PRODUCTION_CURRENT_AUTHORITY_SOURCE_BOUNDARY_MODEL_REVIEW.md).
   The private registered-source interface proof with one in-memory aggregate
   source is now implemented in the
   [Registered Current-Authority Source Interface Proof Report](docs/concepts/REGISTERED_CURRENT_AUTHORITY_SOURCE_INTERFACE_PROOF_REPORT.md).
   A Core-owned private constructor binds registration to one canonical
   complete grant, availability, and governed-context-reference inventory.
   One exact immutable binding and contract request returns either one
   coherent payload-free snapshot commitment or one bounded source failure.
   The interface remains private and cannot confer readiness or dereference
   targets. Focused source-interface review accepts the proof in the
   [Registered Current-Authority Source Interface Proof Review](docs/concepts/REGISTERED_CURRENT_AUTHORITY_SOURCE_INTERFACE_PROOF_REVIEW.md).
   Private registered-source and same-call resolver composition is now
   implemented in the
   [Registered Current-Authority Source Resolver Composition Report](docs/concepts/REGISTERED_CURRENT_AUTHORITY_SOURCE_RESOLVER_COMPOSITION_REPORT.md).
   One Core-owned call now keeps the selected source records and coherent
   source snapshot together, constructs the exact fact set, reruns capability
   resolution, rebuilds step-scoped context projections, and consumes the
   exact required-context contract. Source failure short-circuits resolution;
   unresolved prerequisites and required context gaps block. The result is
   private, payload-free, and not a reusable authorization handle. Focused
   maintainer review accepts the source-backed assessment semantics in the
   [Registered Current-Authority Source Resolver Composition Review](docs/concepts/REGISTERED_CURRENT_AUTHORITY_SOURCE_RESOLVER_COMPOSITION_REVIEW.md).
   One-time-use and replay posture is now specified in the
   [Current-Authority One-Time-Use And Replay Posture Plan](docs/implementation-plans/current-authority-one-time-use-replay-posture-plan.md)
   and its
   [planning report](docs/concepts/CURRENT_AUTHORITY_ONE_TIME_USE_REPLAY_POSTURE_PLAN_REPORT.md).
   The plan rejects reusable TTL-based authority tokens. It requires a private
   Core-owned resolve-and-use call, a non-cloneable and non-serializable
   borrowed use capability, fresh source resolution for every use, retry,
   approval resume, and worker restart, and explicit future persistence before
   any claim of durable replay prevention. Focused maintainer review accepts
   the plan in the
   [Current-Authority One-Time-Use And Replay Posture Plan Review](docs/concepts/CURRENT_AUTHORITY_ONE_TIME_USE_REPLAY_POSTURE_PLAN_REVIEW.md).
   The private same-call use boundary is now implemented with one Core-owned
   bounded `FnOnce` consumer and no generic repeatable authority methods. It
   reruns registered-source resolution for every call, invokes the consumer
   only for `Ready`, keeps blocked and source-failure paths non-invoking, and
   preserves explicit failed and ambiguous consumer outcomes. This proves
   same-call non-reuse only; it does not claim durable replay prevention or
   consumer idempotency. Focused maintainer review accepts the boundary with a
   non-blocking requirement that any later real consumer remain one concrete
   Core-owned operation rather than a broadened generic callback. Direct
   negative-path and fixed-vector hardening is now implemented and accepted.
   Use-boundary tests prove that expired or revoked grants, unresolved
   prerequisites, coherent changed contract/binding pairs, and mismatched
   contracts block before consumer invocation. A stable bounded outcome vector
   covers success, blocked, stale-source, and ambiguous completion without
   exposing payloads. See the
   [hardening report](docs/concepts/CURRENT_AUTHORITY_USE_BOUNDARY_HARDENING_REPORT.md)
   and [review](docs/concepts/CURRENT_AUTHORITY_USE_BOUNDARY_HARDENING_REVIEW.md).
   Planning for the first concrete Core-owned read-only consumer is now
   documented in the
   [Current-Authority WorkReport Artifact Metadata Read Plan](docs/implementation-plans/current-authority-work-report-artifact-metadata-read-plan.md).
   The selected first consumer is a private exact-target read of bounded
   `WorkReport` artifact metadata through an explicit caller-supplied store.
   It must resolve current authority and consume the exact required-context
   contract in the same call before the store is reachable, return no report
   body, and expose no generic authority callback. The planning record is in
   the
   [Current-Authority WorkReport Artifact Metadata Read Plan Report](docs/concepts/CURRENT_AUTHORITY_WORK_REPORT_ARTIFACT_METADATA_READ_PLAN_REPORT.md).
   The plan is accepted in the
   [Current-Authority WorkReport Artifact Metadata Read Plan Review](docs/concepts/CURRENT_AUTHORITY_WORK_REPORT_ARTIFACT_METADATA_READ_PLAN_REVIEW.md);
   the private implementation is now complete in the
   [Current-Authority WorkReport Artifact Metadata Read Report](docs/concepts/CURRENT_AUTHORITY_WORK_REPORT_ARTIFACT_METADATA_READ_REPORT.md).
   One Core-owned call validates the exact required bounded-metadata target,
   freshly resolves registered current authority, touches an explicit
   `WorkReportArtifactStore` only after readiness, reads at most one exact
   artifact, and returns only report ID, run ID, terminal status, and
   sensitivity. Blocked and source-failure paths perform zero store reads;
   report bodies, generic authority callbacks, and public APIs do not escape.
   Focused implementation review accepts the phase in the
   [Current-Authority WorkReport Artifact Metadata Read Review](docs/concepts/CURRENT_AUTHORITY_WORK_REPORT_ARTIFACT_METADATA_READ_REVIEW.md).
   The operation remains private; no broader authority consumer is authorized.
   Production time-of-use readiness, executor integration, persistence
   changes, providers, OpenShell, sandbox execution, SideEffects, and writes
   remain deferred. Return sequencing to the active proportional-governance
   and quiet-success lane.
   The first private proportional-governance runtime composition is now
   implemented in the
   [Current-Authority Proportional-Governance Runtime Composition Report](docs/concepts/CURRENT_AUTHORITY_PROPORTIONAL_GOVERNANCE_RUNTIME_COMPOSITION_REPORT.md).
   One closed single-step Core route rejects caller-preclassified authority,
   binds the exact actor/workflow/run/step/harness contract, freshly resolves
   registered current authority, and injects `Sufficient` only inside the
   same-call executor consumer. Blocked and stale authority never reach the
   governance route. The CLI compatibility path still supplies a hardcoded
   authority fact and is not yet switched because no reviewed production
   local current-authority source/configuration boundary exists. Focused
   review accepts the private bridge in the
   [Current-Authority Proportional-Governance Runtime Composition Review](docs/concepts/CURRENT_AUTHORITY_PROPORTIONAL_GOVERNANCE_RUNTIME_COMPOSITION_REVIEW.md).
   The first production local source for the closed project-validation profile
   is now implemented in the
   [Local Project Authority Source Runtime Composition Report](docs/concepts/LOCAL_PROJECT_AUTHORITY_SOURCE_RUNTIME_COMPOSITION_REPORT.md).
   Project-declared execution no longer trusts a CLI-preclassified authority
   fact. Core verifies the validated declaration captured in the immutable run
   bundle and derives sufficient authority for fresh execution and approval
   reassessment. Focused review accepts the phase in the
   [Local Project Authority Source Runtime Composition Review](docs/concepts/LOCAL_PROJECT_AUTHORITY_SOURCE_RUNTIME_COMPOSITION_REVIEW.md).
   The standalone runtime `--authoritative-governance` compatibility flag is
   now retired for `run` and `approve`. Those commands derive authoritative
   execution only from the validated project declaration and immutable run
   activation. The same named `init-repo-governance` scaffold option remains
   available because it writes that declaration rather than asserting
   per-command authority. See the
   [Authoritative Runtime Flag Retirement Report](docs/concepts/AUTHORITATIVE_RUNTIME_FLAG_RETIREMENT_REPORT.md)
   and
   [Review](docs/concepts/AUTHORITATIVE_RUNTIME_FLAG_RETIREMENT_REVIEW.md).
   The first local unsigned authority-receipt model is now implemented in the
   [Local Unsigned Authority Receipt Report](docs/concepts/LOCAL_UNSIGNED_AUTHORITY_RECEIPT_REPORT.md)
   and
   [Review](docs/concepts/LOCAL_UNSIGNED_AUTHORITY_RECEIPT_REVIEW.md).
   The first internal Core-owned read-only receipt producer is now implemented
   and accepted in the
   [Current-Authority WorkReport Metadata Receipt Production Report](docs/concepts/CURRENT_AUTHORITY_WORK_REPORT_METADATA_RECEIPT_PRODUCTION_REPORT.md)
   and
   [Review](docs/concepts/CURRENT_AUTHORITY_WORK_REPORT_METADATA_RECEIPT_PRODUCTION_REVIEW.md).
   It is opt-in and issues a trusted receipt only after the existing exact
   WorkReport artifact bounded-metadata read succeeds. The receipt binds the
   immutable execution context, exact requirement and grant, fresh source
   commitments, operation kind, and a payload-free operation-outcome
   commitment. Not-found, blocked, stale, source-failure, store-failure, and
   inconsistent paths issue no receipt. The prior non-receipt read remains
   unchanged. Serialized input still becomes an explicitly unverified claim
   with no conversion into trusted evidence. Receipts remain point-in-time,
   local unsigned, payload-free, and explicitly non-authorizing. No public or
   executor producer, persistence, events, provider behavior, OpenShell
   integration, sandbox execution, SideEffect execution, or writes are
   authorized.
   The authority foundation provides validated scoped grants, lifecycle and delegation posture,
   prerequisite references, sensitivity/redaction bounds, and explicit
   availability vocabulary without runtime consumption. Continue with the
   step-scoped tool/context
   projection, and receipt enforcement in small reviewed phases before broader
   mutation families. This lane must reuse policy, approval, SideEffect,
   EvidenceReference, proportional-governance, and Composable Harness
   foundations; it must not turn Workflow OS into an agent platform, memory
   system, hosted control plane, or generic MCP gateway.
7. **Prove real checks before broader expansion.** External dogfood feedback
   correctly distinguishes mock or caller-asserted success from independent
   engineering evidence. The next cross-cutting phase is independent check
   attestation, planned in the
   [Independent Local Check Attestation Plan](docs/implementation-plans/independent-local-check-attestation-plan.md):
   bind check identity, invocation, structured result,
   provenance, freshness, and immutable run context without storing raw command
   output or treating a mock handler as proof. Governed context-access planning
   remains next inside the separate authority lane. The first check-attestation
   core model is implemented as explicitly unverified requirement, source,
   assurance, freshness, and payload-free binding vocabulary. The planning
   phase is accepted in
   [Independent Local Check Attestation Plan Review](docs/concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_PLAN_REVIEW.md).
   A separately reviewed verifier remains required before any record can claim
   accepted independent proof. Focused model review found deterministic-binding
   blockers. The focused fix now removes caller-chosen attestation identity from
   proof identity and binds the complete requirement fingerprint; focused fix
   review accepts both corrections. Pure verifier planning is documented in the
   [Independent Local Check Attestation Verifier Plan](docs/implementation-plans/independent-local-check-attestation-verifier-plan.md).
   Focused review found that current immutable bundles do not freeze the local
   check command contract or trusted handler implementation identity. The
   planning blocker fix now defines a separate content-addressed pre-execution
   binding for the canonical command contract, Core-derived registered-handler
   selection metadata and honest posture, and effective execution policy. Focused
   re-review accepts that correction. The immutable local-check execution
   binding core model is now implemented and phase-level review accepts it with
   non-blocking provenance follow-ups. It provides canonical command and effective
   policy commitments, typed handler selection metadata, honest
   registered-unattested posture, safe serde/Debug behavior, and focused tests.
   The pure verifier is implemented and accepted after its stored-bundle
   integrity blocker was fixed. The first explicit runtime-composition slice is
   now documented in the
   [DocsCheck Attestation Runtime Composition Plan](docs/implementation-plans/docs-check-attestation-runtime-composition-plan.md).
   Focused review found planning blockers in observation-time ownership and
   typed no-proof eligibility semantics. The planning fix now assigns all time
   sampling to a helper-owned injected clock and requires typed requirement
   eligibility before verifier invocation. Focused re-review accepts the
   corrected plan. One explicit crate-internal in-memory `DocsCheck`
   attestation composition helper is now implemented and documented in the
   [DocsCheck Attestation Runtime Composition Report](docs/concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_REPORT.md).
   It freezes exact stored-run, command, handler, policy, and invocation context
   before process execution; derives result, observation, candidate, and proof
   inside Core; and returns honest no-proof outcomes for ineligible statuses.
   Phase-level review found an immutable attribution blocker: step and skill
   identity were caller-selected rather than resolved from the stored canonical
   records. The focused fix now removes caller-supplied skill identity and
   derives it from the selected stored workflow step and exact skill record.
   Focused re-review accepts the fix; explicit consumer integration planning is
   now documented in the
   [DocsCheck Attestation Consumer Integration Plan](docs/implementation-plans/docs-check-attestation-consumer-integration-plan.md).
   The first proposed consumer is a crate-private same-call gate wrapper that
   preserves the structured result, reevaluates freshness, and cannot import or
   cache accepted proof. Focused plan review found blockers in freshness
   disposition and proof reuse. The focused correction now treats expiry as
   typed not-satisfaction, invalid time as error, and exposes only a proof
   commitment after same-call consumption. Focused re-review accepts the
   correction. The crate-private same-call gate is now implemented and
   documented in the
   [DocsCheck Attestation Consumer Integration Report](docs/concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_REPORT.md).
   It preserves the structured result, returns typed satisfied/not-satisfied
   disposition, reevaluates maximum-age freshness at consumption, and exposes
   only a bounded proof fingerprint after satisfaction. Phase-level review
   accepts the gate with non-blocking follow-ups in the
   [DocsCheck Attestation Consumer Integration Review](docs/concepts/DOCS_CHECK_ATTESTATION_CONSUMER_INTEGRATION_REVIEW.md).
   One explicit proportional-governance reassessment mapping is now planned in
   the
   [DocsCheck Attestation Proportional-Governance Integration Plan](docs/implementation-plans/docs-check-attestation-proportional-governance-integration-plan.md).
   It maps satisfied proof, deterministic check failure, and stale required
   proof into a bounded evidence/check contribution. Focused plan review found
   that one leaf check cannot safely replace the aggregate workload fact
   without proving complete obligation coverage. A focused planning blocker
   fix now stops the first implementation at a requirement-scoped contribution
   and keeps aggregate reassessment blocked until an authoritative exact
   obligation set and fail-closed aggregator exist. Focused re-review accepts
   that correction and requires a dedicated
   private leaf-posture type. The requirement-scoped contribution wrapper is
   now implemented and documented in the
   [DocsCheck Attestation Governance Contribution Report](docs/concepts/DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REPORT.md).
   It binds leaf posture to exact immutable bundle, step, and requirement
   identity without producing aggregate satisfaction. Phase-level review
   accepts the implementation with non-blocking wrapper-level test-depth
   follow-ups in the
   [DocsCheck Attestation Governance Contribution Review](docs/concepts/DOCS_CHECK_ATTESTATION_GOVERNANCE_CONTRIBUTION_REVIEW.md).
   Authoritative obligation-set and complete-coverage aggregation planning is
   now documented in the
   [Evidence And Check Obligation-Set Aggregation Plan](docs/implementation-plans/evidence-check-obligation-set-aggregation-plan.md).
   It separates safe onboarding recommendations from authoritative canonical
   declarations, defines exact fail-closed coverage, and keeps the first model
   implementation private and unwired. Focused plan review found blockers: an
   explicitly supplied set cannot produce authoritative-looking aggregate
   posture, and v1 must be narrowed to the accepted local-check attestation
   family. The focused correction now separates non-authoritative structural
   coverage from future authoritative aggregate posture, narrows v1, and
   defines empty-set, optional-failure, and private leaf-adaptation semantics.
   Focused re-review accepts the correction. The private local-check candidate
   model and pure structural evaluator are now implemented and documented in
   the
   [Local Check Governance Structural Coverage Report](docs/concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_REPORT.md).
   The model proves exact structural coverage only relative to an explicitly
   supplied unresolved candidate set; it exposes no aggregate workload
   conversion or runtime authority. Phase-level maintainer review found a
   construction-time cross-bundle relabeling blocker at the private leaf
   adapter boundary. The focused fix now derives exact obligation identity from
   candidate bundle/step binding plus the requirement fingerprint and adds a
   direct relabeling regression; it is documented in the
   [Local Check Governance Structural Coverage Blocker Fix Report](docs/concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_BLOCKER_FIX_REPORT.md).
   Focused re-review accepts the fix in the
   [Local Check Governance Structural Coverage Blocker Fix Review](docs/concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_BLOCKER_FIX_REVIEW.md).
   Canonical local-check declaration and immutable-bundle derivation planning
   is now documented in the
   [Canonical Local-Check Declaration And Immutable-Bundle Derivation Plan](docs/implementation-plans/canonical-local-check-declaration-immutable-bundle-derivation-plan.md).
   It selects a typed workflow-step declaration as the first authoritative
   source, resolves it against an explicit allowlisted command-contract
   inventory, and freezes one canonical declaration-set record per step before
   runtime. Existing skill criteria, policy effects, profile disclosure,
   repository metadata, and inference remain non-authoritative until they gain
   separately reviewed typed constraints. Typed step-scoped declaration
   vocabulary and schema-facing validation are now implemented and documented
   in the
   [Local Check Requirement Declaration Model Report](docs/concepts/LOCAL_CHECK_REQUIREMENT_DECLARATION_MODEL_REPORT.md).
   The default-empty step field preserves existing specs; validated
   declarations require kernel-observed assurance, a passed-only accepted
   result, explicit freshness, exact immutable-run binding, disabled network,
   and classified non-source-writing SideEffect posture. Command-contract
   resolution and immutable-bundle derivation remain unimplemented. The
   [Local Check Requirement Declaration Model Review](docs/concepts/LOCAL_CHECK_REQUIREMENT_DECLARATION_MODEL_REVIEW.md)
   accepts the corrected model. The canonical declaration-set record and pure
   resolver are now implemented. They resolve exact step declarations against
   an explicit validated command-contract inventory, enforce declaration
   maxima, bind canonical command and independent-attestation fingerprints,
   sort by deterministic obligation identity, emit authoritative empty records,
   and fail closed on serialized fingerprint mismatch. Phase-level review
   accepted that model boundary. Immutable-bundle construction and create-only
   storage now publish and validate one typed declaration-set reference and
   content-addressed record per workflow step, including authoritative empty
   sets. The enriched builder is explicit; legacy bundles remain readable with
   an omitted declaration-set collection and cannot claim authoritative
   local-check coverage. Declaration-set references participate in the bundle
   root, while unreferenced command inventory does not cause bundle churn.
   Maintainer review accepts this publication boundary with no blockers; see
   the
   [Canonical Local-Check Declaration Immutable-Bundle Publication Review](docs/concepts/CANONICAL_LOCAL_CHECK_DECLARATION_IMMUTABLE_BUNDLE_PUBLICATION_REVIEW.md).
   A private authoritative adapter now derives structural-coverage obligation
   candidates only from validated `StoredImmutableRunBundle` declaration
   records. It distinguishes canonical stored provenance from caller-supplied
   unresolved candidates, accepts canonical empty step records, and rejects
   legacy, incomplete, duplicate, mismatched, or unknown-step sources with
   stable non-leaking errors. See the
   [Canonical Local-Check Declaration Structural-Coverage Adapter Report](docs/concepts/CANONICAL_LOCAL_CHECK_DECLARATION_STRUCTURAL_COVERAGE_ADAPTER_REPORT.md).
   Maintainer review initially found one ambiguous skill-to-step binding
   blocker. The focused fix now rejects duplicate step bindings before
   deduplication, and governed re-review accepts the private adapter. See the
   [Canonical Local-Check Declaration Structural-Coverage Adapter Review](docs/concepts/CANONICAL_LOCAL_CHECK_DECLARATION_STRUCTURAL_COVERAGE_ADAPTER_REVIEW.md).
   The private authoritative aggregate-fact model and pure conversion helper
   defined in the
   [Authoritative Local-Check Aggregate Posture Conversion Plan](docs/implementation-plans/authoritative-local-check-aggregate-posture-conversion-plan.md).
   Focused planning review accepts that boundary in the
   [Authoritative Local-Check Aggregate Posture Conversion Plan Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_PLAN_REVIEW.md).
   It is now implemented and documented in the
   [Authoritative Local-Check Aggregate Posture Conversion Report](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_REPORT.md).
   It maps only complete canonical stored-bundle structural coverage into the
   existing evidence/check posture vocabulary while retaining exact counts and
   provenance-bearing candidate, coverage, and fact commitments. It does not
   invoke proportional governance, enforce quiet success, run checks, or
   change executor behavior.
   Phase-level review found one focused blocker: direct regression proof that
   every decision-relevant fingerprint input invalidates aggregate-fact
   identity was missing. The focused blocker fix now proves valid semantic
   variants and each current hash input independently; re-review accepts the
   fix. See the
   [Authoritative Local-Check Aggregate Posture Conversion Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_REVIEW.md)
   and
   [Authoritative Local-Check Aggregate Posture Conversion Blocker Fix Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_AGGREGATE_POSTURE_CONVERSION_BLOCKER_FIX_REVIEW.md).
   Later runtime composition must bind the fact fingerprint rather than trust a
   caller-selected posture enum.
   The smallest payload-free visible-disclosure prerequisite is now
   implemented model-only in the
   [Governance Disclosure Delivery Model Report](docs/concepts/GOVERNANCE_DISCLOSURE_DELIVERY_MODEL_REPORT.md).
   It binds one complete authoritative `Proceed + Visible` assessment to one
   explicit injected-local surface and returns a validated surface-acceptance
   receipt that explicitly does not claim human observation, understanding,
   acknowledgement, or approval. Focused review accepts the model with the
   constraint that executor integration must derive the receipt from the
   explicitly injected surface call rather than accept a caller-supplied
   receipt as authority; see
   [Governance Disclosure Delivery Model Review](docs/concepts/GOVERNANCE_DISCLOSURE_DELIVERY_MODEL_REVIEW.md).
   The first explicit injected-local visible `Proceed` executor path is now
   implemented in the
   [Visible Proceed Executor Integration Report](docs/concepts/VISIBLE_PROCEED_EXECUTOR_INTEGRATION_REPORT.md).
   It reuses the authoritative immutable-bundle and same-call `DocsCheck`
   reassessment boundary, requires a complete source-bound aggregate
   `Proceed + Visible` result, constructs the exact payload-free disclosure
   request in Core, invokes the injected surface before `RunCreated` and skill
   execution, and constructs the receipt from the surface's bounded acceptance
   timestamp. Delivery failure, invalid acceptance time, quiet posture,
   approval-required posture, denial, and fresh-run reuse fail closed before
   skill execution. The receipt remains in memory and claims only surface
   acceptance. Persistence, events, audit projection, approval/denial routing,
   CLI/UI behavior, providers, OpenShell, SideEffect execution, and writes
   remain unimplemented. Focused review accepts the route with non-blocking
   trusted-clock and future persistence constraints; see
   [Visible Proceed Executor Integration Review](docs/concepts/VISIBLE_PROCEED_EXECUTOR_INTEGRATION_REVIEW.md).
   The aggregate approval model prerequisite is implemented in the
   [Proportional Governance Approval Binding Report](docs/concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_BINDING_REPORT.md).
   The existing `ApprovalRequest` is step and skill scoped, so it cannot
   truthfully represent the aggregate gate by itself. The new payload-free
   subject commitment binds only a complete, source-bound
   `RequireApproval + Visible` assessment and creates no synthetic step. It
   does not request, decide, persist, present, or resume an approval.
   Focused review accepts the model while requiring same-call Core
   construction before it can participate in runtime authority; see
   [Proportional Governance Approval Binding Review](docs/concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_BINDING_REVIEW.md).
   The first approval-required executor integration is now implemented in the
   [Proportional Governance Approval Executor Integration Report](docs/concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_EXECUTOR_INTEGRATION_REPORT.md).
   The additive fresh-run path constructs the aggregate approval subject
   inside Core from the same-call source-bound `DocsCheck` assessment, pauses
   before any workflow step is scheduled, and reuses the existing durable
   approval request, presentation-proof, decision, and resume lifecycle.
   Grant and denial both require fresh exact reassessment and matching
   presentation proof before decision events. Aggregate approval does not
   authorize SideEffects and does not satisfy later step-level approvals.
   Existing executor and step-approval defaults remain unchanged. CLI/schema
   exposure, automatic approval, providers, OpenShell, SideEffect execution,
   and writes remain unimplemented. Focused phase review accepts the route
   with two non-blocking hardening follow-ups; see
   [Proportional Governance Approval Executor Integration Review](docs/concepts/PROPORTIONAL_GOVERNANCE_APPROVAL_EXECUTOR_INTEGRATION_REVIEW.md).
   The authoritative `Denied + Visible` route is now implemented in the
   [Proportional Governance Denial Executor Integration Report](docs/concepts/PROPORTIONAL_GOVERNANCE_DENIAL_EXECUTOR_INTEGRATION_REPORT.md).
   The additive fresh-run route persists the exact source-bound assessment,
   appends ordinary run-start events, and terminates with the stable
   `executor.authoritative_local_check.governance_denied` code and
   `PolicyDenied` failure class before any step, skill, approval, provider, or
   SideEffect activity. Existing event vocabulary remains truthful: the
   durable assessment binding carries denial provenance and `RunFailed`
   records the terminal result. The combined
   [Authoritative Proportional Governance Routing Review](docs/concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTING_REVIEW.md)
   accepts quiet proceed, visible proceed, approval-required, and denied
   routes together for their explicit local `DocsCheck` slices. The next
   runtime composition gap is one narrow dispatcher in which the derived
   assessment selects the accepted route. Today callers still choose among
   separate exact-route APIs; a wrong choice fails closed and cannot downgrade
   governance, but it is not yet the desired product boundary. The bounded
   implementation sequence is defined in the
   [Authoritative Proportional-Governance Route Dispatcher Plan](docs/implementation-plans/authoritative-proportional-governance-route-dispatcher-plan.md).
   The focused
   [Authoritative Proportional-Governance Route Dispatcher Plan Review](docs/concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_PLAN_REVIEW.md)
   accepts that plan without blockers. The next implementation phase is the
   additive fresh-run local `DocsCheck` dispatcher. That dispatcher is now
   implemented in the
   [Authoritative Proportional-Governance Route Dispatcher Report](docs/concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_REPORT.md):
   one complete source-bound assessment selects quiet proceed, visible
   proceed, approval-required, or denied behavior without a caller route enum
   or a second check execution. The focused
   [Authoritative Proportional-Governance Route Dispatcher Review](docs/concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_ROUTE_DISPATCHER_REVIEW.md)
   accepts the implementation without blockers. The next phase should plan
   one narrow explicit consumer before any broad default or operator-facing
   integration. That consumer is now planned in the
   [Authoritative Governance Report Consumer Plan](docs/implementation-plans/authoritative-governance-report-consumer-plan.md).
   The recommended first slice is now implemented as one additive, in-memory,
   fresh-run-only dispatcher-plus-report helper. It preserves route-specific
   results, generates a WorkReport only for terminal outcomes, treats
   approval-pending posture as report-deferred, and constructs one validated
   payload-free local check result reference from the actual same-call
   `DocsCheck` result and explicit caller metadata. The
   [Authoritative Governance Report Consumer Report](docs/concepts/AUTHORITATIVE_GOVERNANCE_REPORT_CONSUMER_REPORT.md)
   records the implementation boundary and validation. CLI/UI exposure,
   default executor integration, report artifacts, persistence, providers,
   OpenShell, SideEffect execution, writes, and hosted behavior remain out of
   scope. Fresh-pull evaluation confirms that the next product pressure is
   lower ceremony for low-risk work, but direct CLI exposure is not yet an
   honest generic boundary: the accepted authoritative consumer requires the
   Workflow OS repository-specific `DocsCheckLocalHandler`, and approval resume
   cannot yet complete the deferred report path. The
   [Authoritative Quiet-Success CLI Preview Plan](docs/implementation-plans/authoritative-quiet-success-cli-preview-plan.md)
   defines the future opt-in operator contract and defers implementation until
   authoritative approval-resume report completion and one generic explicit
   check-profile source are implemented and reviewed. Ordinary `run` behavior
   remains unchanged. Focused review accepts that sequencing and recommends
   authoritative approval-resume report completion planning as the next
   runtime phase; see the
   [Authoritative Quiet-Success CLI Preview Plan Review](docs/concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_PLAN_REVIEW.md).
   That completion path is now planned in the
   [Authoritative Approval-Resume Report Completion Plan](docs/implementation-plans/authoritative-approval-resume-report-completion-plan.md).
   The plan preserves the accepted decision-time canonical check
   reassessment, carries its exact fresh bounded result through approval
   mutation, and uses that result for terminal WorkReport citation. The
   request-time check remains historical approval-request context; it is not
   reused as current terminal authorization evidence. The first local,
   in-memory completion slice is now implemented: one additive proof-enforced
   approval decision helper retains the exact fresh reassessment result,
   derives its bounded local-check reference, and generates a terminal report
   for grant or denial without a second report-only check. Non-terminal
   continuation remains report-deferred, and post-decision report failures do
   not rewrite workflow status or event history. See the
   [Authoritative Approval-Resume Report Completion Report](docs/concepts/AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_REPORT.md).
   Focused planning review accepts the evidence-freshness,
   mutation-ordering, failure-separation, privacy, and compatibility boundary;
   see the
   [Authoritative Approval-Resume Report Completion Plan Review](docs/concepts/AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_PLAN_REVIEW.md).
   Focused implementation review accepts the phase without blockers; see the
   [Authoritative Approval-Resume Report Completion Review](docs/concepts/AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_REVIEW.md).
   The next runtime prerequisite is one generic explicit local-check profile
   source before any CLI exposure. That boundary is now planned in the
   [Generic Explicit Local-Check Profile Source Plan](docs/implementation-plans/generic-explicit-local-check-profile-source-plan.md).
   The recommended first profile validates the selected Workflow OS project
   through one fixed, allowlisted, source-read-only, network-disabled command
   contract. It binds the same contract, stable handler identity, and
   immutable declaration inventory without accepting shell strings, inferring
   repository commands, or enabling default registration. Focused planning
   review identified and corrected one runtime compatibility gap: the
   implementation must also add a closed profile-to-authoritative-composition
   bridge because the accepted path is currently typed to
   `DocsCheckLocalHandler`. See the
   [Generic Explicit Local-Check Profile Source Plan Review](docs/concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_SOURCE_PLAN_REVIEW.md).
   That Core-only profile source and closed bridge are now implemented. The
   resolved profile binds one fixed `workflow-os validate` contract, stable
   handler identity, collision-rejecting registry installation, and immutable
   declaration inventory. It can enter the existing authoritative quiet,
   visible, approval, denial, and report routes without opening an arbitrary
   handler authority surface. The report-bearing path cites the exact
   same-call result and does not execute a second check. See the
   [Generic Explicit Local-Check Profile Source Report](docs/concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_SOURCE_REPORT.md).
   Focused implementation review found one blocker: the public handler
   constructor must reject any supplied contract that differs from the
   complete canonical `workflow-os validate` contract before CLI exposure.
   See the
   [Generic Explicit Local-Check Profile Source Review](docs/concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_SOURCE_REVIEW.md).
   The narrow canonical-contract blocker is now fixed: public handler
   construction requires full equality with the built-in contract and fails
   before execution on any drift. See the
   [Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Report](docs/concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_CANONICAL_CONTRACT_BLOCKER_FIX_REPORT.md).
   Focused blocker-fix review accepts the phase without remaining blockers;
   see the
   [Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Review](docs/concepts/GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_CANONICAL_CONTRACT_BLOCKER_FIX_REVIEW.md).
   Focused prerequisite re-review then found one cross-prerequisite composition
   blocker: fresh-run report generation accepts the resolved explicit profile,
   while proof-enforced approval-resume report completion still accepts only
   `DocsCheckLocalHandler`. See the
   [Authoritative Quiet-Success CLI Preview Prerequisite Re-Review](docs/concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_PREREQUISITE_REREVIEW.md).
   The next phase is one narrow explicit-profile authoritative approval-resume
   report-completion bridge. That bridge is now implemented: a resolved
   project-validation profile can perform proof-enforced decision-time
   reassessment and terminal report citation through the same closed handler
   authority. See the
   [Explicit-Profile Authoritative Approval-Resume Report Completion Plan](docs/implementation-plans/explicit-profile-authoritative-approval-resume-report-completion-plan.md)
   and
   [Implementation Report](docs/concepts/EXPLICIT_PROFILE_AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_REPORT.md).
   Focused review accepts the bridge without blockers; see the
   [Explicit-Profile Authoritative Approval-Resume Report Completion Review](docs/concepts/EXPLICIT_PROFILE_AUTHORITATIVE_APPROVAL_RESUME_REPORT_COMPLETION_REVIEW.md).
   The explicit authoritative quiet-success CLI preview is now implemented as
   additive `run --authoritative-governance` and
   `approve --authoritative-governance` paths. The preview uses the closed
   project-validation profile, delegates route selection to Core, renders
   bounded quiet/visible/approval/denial posture, persists complete approval
   presentation proof, keeps aggregate governance and authored workflow
   approvals as separate gates, and produces an in-memory terminal WorkReport.
   Ordinary command behavior remains unchanged. See the
   [Implementation Report](docs/concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_REPORT.md).
   Phase-level review accepts the preview with non-blocking follow-ups after
   fixing the approval renderer to emit the persisted presentation record
   rather than a duplicate string copy. See the
   [Authoritative Quiet-Success CLI Preview Review](docs/concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_REVIEW.md).
   Defaults, report persistence, artifacts, schemas, providers, OpenShell,
   SideEffect execution, and writes remain out of scope.
   The next bounded product phase was defined in the
   [Profile-Controlled Authoritative Governance Activation Plan](docs/implementation-plans/profile-controlled-authoritative-governance-activation-plan.md).
   It is now implemented: one typed, optional
   project declaration that binds the existing `observe_and_report` minimum to
   the closed `workflow_os_project_validation` profile activates the existing
   authoritative path without a per-command flag. Rust, JSON Schema,
   TypeScript SDK, immutable-run binding, `run`/`approve` resolution, and
   first-run disclosure are synchronized. The complete project manifest
   content identity is committed into the immutable activation posture, so
   changed or removed manifest input fails closed before approval resume. See
   the
   [Implementation Report](docs/concepts/PROFILE_CONTROLLED_AUTHORITATIVE_GOVERNANCE_ACTIVATION_REPORT.md).
   Projects without the declaration retain ordinary behavior; incomplete,
   unsupported, conflicting, or changed declarations fail closed. The plan
   and implementation avoid another model-only chain and do not authorize scaffold
   defaults, inferred activation, arbitrary commands, providers, OpenShell,
   writes, artifacts, hosted controls, or broader profile families.
   Phase-level review accepts the implementation with non-blocking follow-ups
   after fixing two compatibility blockers found by the full validation and
   review passes: undeclared runs retain legacy immutable hashes, and
   undeclared invalid projects retain their ordinary validation path. See the
   [Profile-Controlled Authoritative Governance Activation Review](docs/concepts/PROFILE_CONTROLLED_AUTHORITATIVE_GOVERNANCE_ACTIVATION_REVIEW.md).
   The first quiet-success operator UX hardening slice is now implemented and
   accepted.
   Completed `QuietProceed` runs with a successfully generated in-memory report
   emit concise human output with a durable inspect command; `run --verbose`
   retains bounded route, disclosure, report, and local-check reference detail,
   and preview JSON remains unchanged. Failed runs, report failures, visible
   disclosures, approvals, and denials remain explicit. See the
   [Quiet-Success Operator UX Hardening Report](docs/concepts/AUTHORITATIVE_QUIET_SUCCESS_OPERATOR_UX_HARDENING_REPORT.md)
   and
   [Review](docs/concepts/AUTHORITATIVE_QUIET_SUCCESS_OPERATOR_UX_HARDENING_REVIEW.md).
   The next runtime composition phase was defined in the
   [Authoritative WorkReport Artifact Persistence Plan](docs/implementation-plans/authoritative-work-report-artifact-persistence-plan.md).
   It is now implemented. Explicit and project-controlled authoritative
   terminal paths persist their validated `WorkReport` through the existing
   SideEffect, high-assurance disclosure, and approval proof-marker artifact
   gates. Pending approvals defer the artifact obligation; successful
   approval resume completes it. Exact terminal retries revalidate the
   immutable bundle, rerun the closed local check, reproduce the durable
   governance binding, and accept only an exactly equal existing artifact.
   Conflicting duplicates fail closed, concurrent equal duplicates reconcile
   to one record, and `inspect` exposes metadata without report body content.
   Ordinary undeclared runs remain unchanged. See the
   [Implementation Report](docs/concepts/AUTHORITATIVE_WORK_REPORT_ARTIFACT_PERSISTENCE_REPORT.md).
   Phase-level review initially found two publication blockers: the durable
   approval presentation excluded the exact artifact that approval completion
   could persist, and the new JSON fields did not match the planned names. See
   the
   [Initial Review](docs/concepts/AUTHORITATIVE_WORK_REPORT_ARTIFACT_PERSISTENCE_REVIEW.md).
   The focused
   [Blocker Fix](docs/concepts/AUTHORITATIVE_WORK_REPORT_ARTIFACT_PERSISTENCE_BLOCKER_FIX_REPORT.md)
   makes the persisted and rendered approval scope truthful, retains the
   prohibition on arbitrary artifacts and broader persistence, aligns the
   JSON contract, and adds visible and denied-route regressions. The
   [Blocker-Fix Review](docs/concepts/AUTHORITATIVE_WORK_REPORT_ARTIFACT_PERSISTENCE_BLOCKER_FIX_REVIEW.md)
   accepts the complete phase without remaining blockers.
   The phase does not authorize provider expansion, OpenShell integration, new
   SideEffect families, hosted storage, report export, or automatic artifacts
   for all runs.
   The first explicit onboarding activation is now implemented in the
   [Authoritative Governance Scaffold Opt-In Plan](docs/implementation-plans/authoritative-governance-scaffold-opt-in-plan.md).
   `workflow-os init-repo-governance --authoritative-governance` writes the
   already-supported closed `observe_and_report` and
   `workflow_os_project_validation` declaration. Default scaffolds remain
   undeclared, dry-run remains non-writing, and scaffolding executes no check
   or workflow. The option does not infer authority, accept arbitrary
   commands, call providers, integrate OpenShell, execute SideEffects, or
   authorize external writes. See the
   [Implementation Report](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_OPT_IN_REPORT.md).
   Phase review found and fixed one setup-integrity blocker: unknown scaffold
   options are now rejected so a misspelled activation flag cannot silently
   leave the project undeclared. The
   [Review](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_OPT_IN_REVIEW.md)
   accepts the phase and recommends a disposable external-repository
   evaluation before any broader default. That
   [external-repository evaluation](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_EXTERNAL_REPOSITORY_EVALUATION.md)
   preserved default compatibility and existing agent guidance, but found a
   runtime blocker: the explicit scaffold wrote the project-level profile
   selection without adding the exact workflow-step project-validation check
   declaration required by the authoritative CLI consumer. The generated
   project validated and reported enforced posture, then failed closed with
   `cli.authoritative_governance.check_profile_missing` before run creation.
   The focused fix now adds the canonical requirement only to explicit
   authoritative scaffolds. The default workflow remains unchanged, and the
   repeated disposable evaluation reaches separate governance and workflow
   approvals, completes, and persists one WorkReport artifact. Default
   activation remains unchanged. The focused
   [blocker-fix review](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_RUNTIME_CONTRACT_BLOCKER_FIX_REVIEW.md)
   accepts the complete scaffold-to-runtime contract with no remaining
   blockers. Broader activation is documented in the
   [Authoritative Governance Scaffold Default Activation Plan](docs/implementation-plans/authoritative-governance-scaffold-default-activation-plan.md)
   and assessed in its
   [planning review](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_DEFAULT_ACTIVATION_PLAN_REVIEW.md).
   Focused code inspection corrected the initial readiness verdict: the CLI
   still predicts visible disclosure before the same-call check result exists
   and can classify non-selected workflow steps' evidence/check posture as
   satisfied. Default activation is therefore deferred. The next bounded
   implementation is the
   [Core-Owned Authoritative Runtime-Fact Derivation Plan](docs/implementation-plans/core-owned-authoritative-runtime-fact-derivation-plan.md),
   accepted in its
   [planning review](docs/concepts/CORE_OWNED_AUTHORITATIVE_RUNTIME_FACT_DERIVATION_PLAN_REVIEW.md).
   The phase is implemented and accepted in the
   [implementation report](docs/concepts/CORE_OWNED_AUTHORITATIVE_RUNTIME_FACT_DERIVATION_REPORT.md)
   and
   [review](docs/concepts/CORE_OWNED_AUTHORITATIVE_RUNTIME_FACT_DERIVATION_REVIEW.md).
   The closed fact-free request constrains the profile to its proven one-step
   shape, constructs unresolved facts inside Core, derives authority and check
   posture from canonical sources, and lets Core conditionally consume
   visible-delivery capability after the actual assessment selects the route.
   The CLI no longer constructs runtime facts or predicts disclosure. It does not
   authorize multi-step authoritative governance, a scaffold default change,
   inferred commands, automatic approval, providers, OpenShell, SideEffect
   execution, writes, hosted behavior, schemas, examples, or release changes.
   The prerequisite review cleared the bounded onboarding default. New
   `init-repo-governance` scaffolds now carry the exact closed
   `observe_and_report` declaration and canonical
   `workflow_os_project_validation` requirement by default. The positive flag
   remains compatible, `--no-authoritative-governance` explicitly selects the
   legacy undeclared scaffold, and contradictory posture flags fail before
   writes. Existing repositories are not migrated. See the
   [Default Activation Report](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_DEFAULT_ACTIVATION_REPORT.md)
   and
   [Review](docs/concepts/AUTHORITATIVE_GOVERNANCE_SCAFFOLD_DEFAULT_ACTIVATION_REVIEW.md).
   Exact same-call composition is now implemented and accepted in the
   [Authoritative Local-Check Same-Call Composition Plan](docs/implementation-plans/authoritative-local-check-same-call-composition-plan.md).
   The private Core-owned helper preflights an explicit batch against
   canonical stored declarations before any process starts, derives
   required/optional posture from those declarations, executes accepted
   `DocsCheck` inputs in canonical order through the existing contribution
   path, preserves bounded results, evaluates exact coverage, and returns the
   provenance-bearing aggregate fact. Exact requirement and command-contract
   identity are checked against canonical records during full-batch preflight.
   It does not invoke proportional governance or change executor behavior.
   Phase-level review accepts the authority, preflight, coverage, failure, and
   privacy boundaries in the
   [Authoritative Local-Check Same-Call Composition Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_SAME_CALL_COMPOSITION_REVIEW.md).
   Private aggregate-fact reassessment binding is now planned in the
   [Authoritative Local-Check Reassessment Binding Plan](docs/implementation-plans/authoritative-local-check-reassessment-binding-plan.md).
   The planned same-call wrapper invokes authoritative check composition,
   rejects caller-selected evidence/check posture for the selected step, and
   binds the complete local-check fact identity to the selected and aggregate
   reassessment identities. Phase-level review found two focused planning
   blockers: complete deterministic wrapper preflight must precede process
   execution, and the outcome must not expose an unbound assessment set as
   reusable authority. See the
   [Authoritative Local-Check Reassessment Binding Plan Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_REVIEW.md).
   The focused correction now requires full wrapper preflight before process
   use and replaces the separable assessment-set/fingerprint shape with one
   private bound-assessment value. See the
   [Authoritative Local-Check Reassessment Binding Plan Blocker Fix Report](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REPORT.md).
   Focused re-review accepts both corrected boundaries in the
   [Authoritative Local-Check Reassessment Binding Plan Blocker Fix Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_PLAN_BLOCKER_FIX_REVIEW.md).
   The private binding implementation is now complete. It performs pure
   immutable-bundle and runtime-fact preflight before local process use,
   composes the authoritative check fact in the same call, injects only the
   selected evidence/check axis, and returns one private fact-bound assessment
   with a versioned identity. See the
   [Authoritative Local-Check Reassessment Binding Report](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_REPORT.md).
   Phase-level review accepts the complete preflight, same-call authority,
   binding-identity, monotonicity, privacy, and private-surface boundaries in
   the
   [Authoritative Local-Check Reassessment Binding Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_REASSESSMENT_BINDING_REVIEW.md).
   The first opt-in executor consumer is implemented and accepted after its
   atomic fresh-run claim blocker was fixed and re-reviewed. See the
   [Authoritative Local-Check Executor Consumer Plan](docs/implementation-plans/authoritative-local-check-executor-consumer-plan.md).
   The additive path selects one fresh-run-only, explicit `DocsCheck` step,
   derives check identities from the immutable bundle, executes the accepted
   same-call reassessment, and consumes the private bound assessment without
   detaching its authoritative fact. It retains a backward-readable source
   commitment in the durable governance binding,
   and executes the existing sequential workflow only for a complete aggregate
   quiet `Proceed` result. Visible disclosure, approval-required, denied,
   incomplete, failed-check, or existing-run contexts fail before
   `RunCreated`. The binding event and audit projection disclose bounded source
   kind and presence without raw IDs or fingerprints. See the
   [implementation report](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_REPORT.md).
   Phase review found that concurrent callers can both pass the initial
   empty-state check and that the second caller can accept the first caller's
   identical immutable bundle before re-executing the local check. The next
   focused fix makes create-only manifest publication the authoritative claim
   and rejects every losing claimant before process use. See the
   [implementation review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_REVIEW.md).
   The correction and focused regression are documented in the
   [blocker-fix report](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_BLOCKER_FIX_REPORT.md).
   Focused re-review accepts create-only immutable-manifest publication as the
   authoritative fresh-run claim in the
   [blocker-fix review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_BLOCKER_FIX_REVIEW.md).
   The next runtime-composition boundary is now defined in the
   [Authoritative Proportional-Governance Executor Routing Plan](docs/implementation-plans/authoritative-proportional-governance-executor-routing-plan.md).
   It preserves the accepted quiet path and separates visible delivery,
   proof-enforced approval, and denial into explicit routes. The first proposed
   implementation is a payload-free visible-disclosure delivery prerequisite
   followed by one explicit visible `Proceed` executor slice. Focused review
   accepts the plan with non-blocking constraints on surface-acceptance receipt
   semantics, aggregate approval binding, and denial lifecycle. Planning does
   not authorize runtime behavior yet. See the
   [plan review](docs/concepts/AUTHORITATIVE_PROPORTIONAL_GOVERNANCE_EXECUTOR_ROUTING_PLAN_REVIEW.md).
   The plan requires a
   model-prerequisite split if the source commitment cannot remain a small
   backward-compatible extension; it does not authorize a forgeable or
   posture-only shortcut. Phase-level review found and corrected one planning
   blocker: the complete multi-step assessment set, not only the selected
   checked step, must resolve to complete quiet `Proceed` before execution.
   The corrected plan is accepted in the
   [Authoritative Local-Check Executor Consumer Plan Review](docs/concepts/AUTHORITATIVE_LOCAL_CHECK_EXECUTOR_CONSUMER_PLAN_REVIEW.md).
   Executor checkpoint integration remains separately governed. The lane keeps
   automatic checks, executor defaults, new standalone stores or event kinds,
   evidence, reports, schemas, CLI behavior, providers, SideEffects, and writes
   out of scope. The consumer extends only the existing durable
   governance binding and its existing event projection with a
   backward-compatible authoritative-source commitment; it must split that
   model prerequisite rather than persist detached posture if compatibility
   cannot be preserved.
   Dogfooding this handoff found and fixed a runner blocker: the generic
   approval non-scope prohibited schema changes even for the dedicated
   `dg/spec-field-operationalization` workflow. Phase-aware non-scope now keeps
   schema work forbidden everywhere except the exact approved spec-field scope;
   see the
   [Governed Spec-Field Phase Schema-Scope Blocker Fix Report](docs/concepts/GOVERNED_SPEC_FIELD_PHASE_SCHEMA_SCOPE_BLOCKER_FIX_REPORT.md).
   Implementation inspection found a prerequisite verifier blocker: exact
   workflow and run identity are not yet derived from and compared to the
   validated stored manifest. The focused fix now enforces both manifest
   identities and adds consistent-relabelling regression coverage. Review that
   fix before resuming runtime composition. Focused review accepted the fix and
   the explicit helper resumed under that corrected identity boundary.
   The pure crate-private verifier is now implemented with Core-owned observation
   authority, deterministic mismatch and freshness enforcement, a read-only
   accepted record, stable non-leaking errors, and focused tests. It does not
   treat a publicly recomputable fingerprint as authenticity proof. Phase-level
   review found one blocker: verification received only a bundle-root binding
   rather than the validated stored manifest and canonical records. The focused
   fix now requires `StoredImmutableRunBundle`, derives the trusted binding from
   its validated manifest, and rejects independently valid but mismatched stored
   bundles. Focused re-review accepts the correction. The explicit opt-in
   `DocsCheck` runtime-composition helper and its first fresh-run executor
   consumer now exist and are accepted for complete quiet `Proceed`.
   Automatic/default runtime integration, visible-disclosure continuation,
   proportional approval routing, denial routing, retry, resume, and additional
   check families remain unimplemented. Do not add speculative execution
   providers or broader mutation families first.
8. **Select an existing open-source durable store before collaborative state.**
   Workflow OS should not invent a database or continue treating Git and local
   files as the eventual collaboration backend. Before multi-user workflow
   ownership, shared catalog state, enterprise stewardship, or hosted runtime
   work begins, run an ADR-backed evaluation of existing open-source databases
   against kernel invariants. The evaluation must cover append-only ordered
   events, create-only and idempotent records, immutable run bundles,
   transactional approval/authority/SideEffect boundaries, concurrent workers,
   optimistic conflict detection, deterministic reads, schema migration and
   recovery, local development, self-hosting, operational maturity, backup and
   restore, inspectability, and a credible path from one user to collaborating
   teams. It must distinguish the embedded local-store need from the shared
   collaborative-store need and decide whether one database can responsibly
   serve both or whether compatible adapters are required. Candidate selection
   must prefer a mature maintained open-source database behind existing state,
   event, catalog, evidence, report, and artifact store interfaces; core domain
   semantics must not depend on vendor-specific behavior. This phase selects
   and proves the storage boundary only. It does not authorize hosted SaaS,
   automatic migration of user state, enterprise administration, provider
   mutations, or a bespoke Workflow OS database.
   The accepted
   [Open-Source Durable Store Selection Plan](docs/implementation-plans/open-source-durable-store-selection-plan.md)
   and
   [ADR 0012](docs/adr/0012-compatible-sqlite-postgresql-durable-state-adapters.md)
   select compatible SQLite embedded and PostgreSQL shared adapters behind one
   Core-owned semantic contract. The planning phase adds no database
   dependency. Focused review in the
   [Open-Source Durable Store Selection Plan Review](docs/concepts/OPEN_SOURCE_DURABLE_STORE_SELECTION_PLAN_REVIEW.md)
   corrects the external-effect atomicity boundary and excludes current
   CockroachDB releases from the open-source candidate set. The first
   database-free semantic contract and executable local-backend conformance
   harness are now implemented in the
   [Durable State Semantic Contract Report](docs/concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REPORT.md).
   The filesystem backend passes applicable ordered-event, identity,
   idempotency, lock, and health scenarios while explicitly declaring all
   cross-record transactions, compare-and-set revisions, expiring fenced
   leases, managed migrations, verified backup/restore, and shared-worker
   concurrency unsupported. The
   [Durable State Semantic Contract Review](docs/concepts/DURABLE_STATE_SEMANTIC_CONTRACT_REVIEW.md)
   accepts the phase after adding bounded negative-scenario IDs and executable
   immutable-identity mismatch proof. The first opt-in SQLite embedded adapter
   is implemented in the
   [SQLite Embedded Durable State Adapter Report](docs/concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REPORT.md).
   It provides managed schema version one, canonical validated record
   envelopes, WAL/full-synchronous local durability, existing store behavior,
   and expanded reopen, contention, schema, corruption, and non-leakage tests.
   Focused review in the
   [SQLite Embedded Durable State Adapter Review](docs/concepts/SQLITE_EMBEDDED_DURABLE_STATE_ADAPTER_REVIEW.md)
   accepts the adapter after adding authoritative read-time relational identity
   enforcement.
   It is not selected automatically and does not claim cross-record atomicity,
   managed migration, verified backup/restore, shared-worker concurrency, or
   collaborative state. Explicit filesystem-to-SQLite migration is now planned
   in the
   [Filesystem-To-SQLite State Migration Plan](docs/implementation-plans/filesystem-to-sqlite-state-migration-plan.md).
   The plan separates read-only inventory, canonical import, projection rebuild,
   destination verification, and explicit activation. The first recommended
   implementation is a read-only inventory and compatibility model only; no
   destination write, CLI migration, automatic selection, or source mutation is
   authorized. Focused review in the
   [Filesystem-To-SQLite State Migration Plan Review](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_REVIEW.md)
   accepts this boundary. The first read-only inventory and compatibility model
   is implemented in the
   [Filesystem-To-SQLite State Migration Inventory Report](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_INVENTORY_REPORT.md).
   It inventories every known filesystem family, classifies canonical,
   projection, ephemeral, and companion state, rejects ambiguous or corrupt
   source shapes, and derives a path-independent payload-free fingerprint
   without modifying source state or creating a destination. It is accepted
   with non-blocking follow-ups in the
   [Filesystem-To-SQLite State Migration Inventory Review](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_INVENTORY_REVIEW.md).
   The model-only migration plan and unreachable staging destination are
   implemented in the
   [Filesystem-To-SQLite State Migration Plan Model Report](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_REPORT.md).
   The plan binds a validated migration identity to the accepted source
   fingerprint, a logical SQLite destination identity and adapter schema,
   canonical family ordering and dispositions, exact-plan resume posture, and
   typed verification obligations. It does not create or write a database,
   import state, expose CLI behavior, or activate a backend. Focused maintainer
   review found one blocker in the
   [Filesystem-To-SQLite State Migration Plan Model Review](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_REVIEW.md):
   deserialization can currently weaken the required local-filesystem writer
   quiescence posture without invalidating the plan. The focused correction is
   implemented in the
   [Filesystem-To-SQLite State Migration Plan Model Blocker Fix Report](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_BLOCKER_FIX_REPORT.md).
   Focused re-review accepts the correction in the
   [Filesystem-To-SQLite State Migration Plan Model Blocker Fix Review](docs/concepts/FILESYSTEM_TO_SQLITE_STATE_MIGRATION_PLAN_MODEL_BLOCKER_FIX_REVIEW.md).
   The cross-process writer-quiescence and importer transaction boundary is
   now defined in the
   [Filesystem-To-SQLite Writer Quiescence And Import Transaction Plan](docs/implementation-plans/filesystem-to-sqlite-writer-quiescence-import-transaction-plan.md).
   It requires a cooperating root-wide writer guard, explicit authority and
   writer-protocol compatibility, stable source fingerprints, one atomic
   staging import transaction, deterministic interruption behavior, and
   separate verification and activation. The plan adds no runtime behavior or
   destination write. Focused review in the
   [Filesystem-To-SQLite Writer Quiescence And Import Transaction Plan Review](docs/concepts/FILESYSTEM_TO_SQLITE_WRITER_QUIESCENCE_IMPORT_TRANSACTION_PLAN_REVIEW.md)
   accepts the boundary after adding an immutable migration-attempt fingerprint
   that binds writer, guard, importer-transaction, and adapter-schema versions.
   The first model-only writer-guard capability and compatibility slice is now
   implemented in the
   [Filesystem-To-SQLite Writer Guard Capability Model Report](docs/concepts/FILESYSTEM_TO_SQLITE_WRITER_GUARD_CAPABILITY_MODEL_REPORT.md).
   It represents typed writer, guard, and importer-transaction protocol
   versions; shared-writer and exclusive-migration modes; bounded future
   acquisition outcomes; exact compatibility posture; and an immutable
   migration-attempt fingerprint. It does not acquire a filesystem lock,
   inspect or stop processes, create SQLite, import records, verify a
   destination, activate a backend, or expose CLI behavior. Review this
   model-only boundary before implementing the cooperating writer guard across
   every filesystem mutation path. Focused review in the
   [Filesystem-To-SQLite Writer Guard Capability Model Review](docs/concepts/FILESYSTEM_TO_SQLITE_WRITER_GUARD_CAPABILITY_MODEL_REVIEW.md)
   accepts the phase. The local filesystem cooperating writer guard is now
   implemented in the
   [Filesystem-To-SQLite Local Writer Guard Report](docs/concepts/FILESYSTEM_TO_SQLITE_LOCAL_WRITER_GUARD_REPORT.md).
   Ordinary `LocalStateBackend` mutations and canonical immutable run-bundle
   writes under the state root acquire a shared cross-process advisory guard;
   migration callers can acquire an exclusive read-only inspection guard.
   A path-independent versioned protocol marker, stable non-leaking
   contention errors, separate-process exclusion, and release on process death
   are covered. This remains cooperating-writers-only local filesystem
   protection. It does not stop older or hostile writers, guarantee
   network-filesystem behavior, create SQLite, import state, verify or activate
   a destination, or expose migration CLI behavior. Focused maintainer review
   in the
   [Filesystem-To-SQLite Local Writer Guard Review](docs/concepts/FILESYSTEM_TO_SQLITE_LOCAL_WRITER_GUARD_REVIEW.md)
   finds no unguarded public mutation path in source, but requires direct
   contention proof for every remaining public state and canonical companion
   writer before acceptance. That narrow coverage fix is now implemented and
   documented in the
   [Filesystem-To-SQLite Local Writer Guard Blocker Fix Report](docs/concepts/FILESYSTEM_TO_SQLITE_LOCAL_WRITER_GUARD_BLOCKER_FIX_REPORT.md).
   It adds direct contention and no-mutation proof for every public canonical
   writer named by the review without redesigning the guard or beginning
   import. Focused review in the
   [Filesystem-To-SQLite Local Writer Guard Blocker Fix Review](docs/concepts/FILESYSTEM_TO_SQLITE_LOCAL_WRITER_GUARD_BLOCKER_FIX_REVIEW.md)
   accepts the fix with no blocker. Proceed to the accelerated Operational
   Embedded Durable State build. That complete local vertical slice is now
   implemented in the
   [Operational Embedded Durable State Report](docs/concepts/OPERATIONAL_EMBEDDED_DURABLE_STATE_REPORT.md)
   and accepted in the
   [Operational Embedded Durable State Review](docs/concepts/OPERATIONAL_EMBEDDED_DURABLE_STATE_REVIEW.md).
   An explicit guarded helper and bounded CLI import compatible filesystem
   state into unreachable SQLite staging, rebuild projections in one
   transaction, verify counts, canonical content, run rehydration, identity,
   and WorkReport-to-SideEffect references, and persist a payload-free receipt
   while retaining the source. A separate exact-receipt command marks only the
   destination ready. Neither command selects SQLite for runtime use, removes
   the source, performs automatic migration, adds shared workers, or claims
   production readiness.
   PostgreSQL remains later.
9. **Review expansion readiness again: complete.** The
   [Expansion Readiness Review](docs/concepts/EXPANSION_READINESS_REVIEW.md)
   finds the governance, immutable-input, authority, durable-state, hosted
   no-write, evidence, and report foundations sufficient to plan one optional
   OpenShell no-write execution-provider vertical slice. The provider-neutral
   contract must first bind effective loaded policy revision/digest,
   enforcement and degradation posture, runtime image identity, lifecycle and
   cleanup outcome, denied-action/log/artifact references, and reconciliation
   state. Requested policy identity alone is not execution attestation.
   OpenShell remains optional and upstream-tracked; a fork is not justified.
   The focused
   [OpenShell Optional No-Write Execution Provider Vertical Slice Plan](docs/implementation-plans/openshell-optional-no-write-execution-provider-plan.md)
   is now documented for review. It requires a pinned upstream boundary,
   effective-policy and control attestation, OCSF/artifact evidence, cleanup,
   restart, and reconciliation proof in one integrated implementation
   milestone. The focused
   [Plan Review](docs/concepts/OPENSHELL_OPTIONAL_NO_WRITE_EXECUTION_PROVIDER_PLAN_REVIEW.md)
   accepts the plan with no blocker. The first implementation slice adds a
   provider-neutral effective-policy/control/cleanup attestation, makes the
   hosted worker consume an explicitly injected provider, and implements an
   optional OpenShell no-write lifecycle provider behind an injected client
   boundary. Scripted-client tests cover hard-control validation, policy
   revision drift, denied-egress proof, cleanup ambiguity, and pre-invocation
   SideEffect rejection. A version-pinned OpenShell v0.0.101 CLI compatibility
   transport now verifies the exact binary version, constructs fixed
   no-provider sandbox-create arguments, bounds subprocess time/output, and
   strictly parses reviewed create/get/effective-policy JSON fixtures. The
   compatibility spike also proved that this CLI release does not expose the
   driver-observed immutable image identity, complete OCSF observations, or
   machine-readable cleanup confirmation required by `OpenShellNoWriteClient`;
   the transport is therefore not wired as an execution provider and fails
   closed at that boundary. An upstream/API attestation solution and live
   sandbox smoke proof remain required before the integrated milestone is
   accepted. The focused compatibility review is documented in the
   [OpenShell Pinned CLI Compatibility Transport Review](docs/concepts/OPENSHELL_PINNED_CLI_COMPATIBILITY_TRANSPORT_REVIEW.md).
   Its first bounded hardening slice now binds an expected executable digest,
   verifies it before and after each subprocess, rejects successful stderr,
   enforces detailed policy revision/source coherence, and rejects observable
   state drift through a before/policy/after reconciliation result. The
   [OpenShell CLI Compatibility Hardening Report](docs/concepts/OPENSHELL_CLI_COMPATIBILITY_HARDENING_REPORT.md)
   keeps this result explicitly non-atomic and non-attesting. The focused
   [Hardening Review](docs/concepts/OPENSHELL_CLI_COMPATIBILITY_HARDENING_REVIEW.md)
   found one additional blocker: failures observed after subprocess start
   could be mislabeled `NotStarted`. The bounded
   [Attempt-Posture Blocker Fix](docs/concepts/OPENSHELL_CLI_ATTEMPT_POSTURE_BLOCKER_FIX_REPORT.md)
   now preserves `NotStarted` only for failures proven to occur before the
   governed operation and reports post-invocation uncertainty as
   `MayHaveStarted`. The focused
   [Blocker-Fix Review](docs/concepts/OPENSHELL_CLI_ATTEMPT_POSTURE_BLOCKER_FIX_REVIEW.md)
   accepts the correction with non-blocking test-strengthening follow-ups. The
   smallest
   [OpenShell Upstream API Attestation Contract Plan](docs/implementation-plans/openshell-upstream-api-attestation-contract-plan.md)
   now defines the required authoritative facts, evidence-sufficiency matrix,
   retry/reconciliation posture, and strict fork threshold. The focused
   [Plan Review](docs/concepts/OPENSHELL_UPSTREAM_API_ATTESTATION_CONTRACT_PLAN_REVIEW.md)
   accepts that boundary. The
   [OpenShell v0.0.101 Evidence-Sufficiency Matrix](docs/implementation-plans/openshell-v0-0-101-evidence-sufficiency-matrix.md)
   now maps every required fact to the exact pinned upstream schema and
   enforcing or observing component. The pin provides authoritative sandbox
   identity and effective-policy revision/load facts, but provider wiring
   remains blocked: restart-safe invocation identity, driver-observed image
   identity, durable operation outcome, complete interval-bound observations,
   exact cleanup proof, and typed capability negotiation are unavailable.
   The focused
   [Matrix Review](docs/concepts/OPENSHELL_V0_0_101_EVIDENCE_SUFFICIENCY_MATRIX_REVIEW.md)
   accepts the classifications and blockers. A focused upstream API proposal
   is now documented in the
   [OpenShell Upstream Attestation API Proposal](docs/implementation-plans/openshell-upstream-attestation-api-proposal.md).
   It proposes general idempotent creation, canonical policy and applied-state
   snapshots, driver-observed image identity, durable operations, complete
   observation manifests, exact cleanup receipts, and typed capabilities.
   Focused review in the
   [OpenShell Upstream Attestation API Proposal Review](docs/concepts/OPENSHELL_UPSTREAM_ATTESTATION_API_PROPOSAL_REVIEW.md)
   accepts the provider-neutral architecture, staged compatibility posture,
   privacy boundary, and no-fork decision. One bounded
   [OpenShell Trustworthy Sandbox Attestation Discussion Draft](docs/implementation-plans/openshell-upstream-attestation-discussion-draft.md)
   is now accepted by focused maintainer
   [review](docs/concepts/OPENSHELL_UPSTREAM_ATTESTATION_DISCUSSION_DRAFT_REVIEW.md)
   for exactly one separately governed submission to OpenShell's Design
   Discussion category. The review rechecked the official source references,
   contribution posture, venue, generality, tone, privacy boundary, and
   no-fork decision. The accepted body was submitted exactly once as
   [OpenShell Discussion #2661](https://github.com/NVIDIA/OpenShell/discussions/2661),
   with the stable URL and immutable content commitments recorded in the
   [Submission Report](docs/concepts/OPENSHELL_UPSTREAM_ATTESTATION_DISCUSSION_SUBMISSION_REPORT.md).
   No issue, pull request, provider wiring, or runtime execution is authorized
   while awaiting upstream feedback. A
   [current-upstream re-verification](docs/concepts/OPENSHELL_CURRENT_UPSTREAM_ATTESTATION_REVERIFICATION_REVIEW.md)
   compared official `main` commit
   `4cb77a900ebd6b789d2b68daaba4830866833b1c` with the exact `v0.0.101` release
   tree. Their public protobuf and attestation-relevant observability contracts
   are identical; the four unrelated tree differences do not close any
   blocker. Provider wiring, Rust changes, and live execution remain blocked.
   A live smoke proof also remains required. Another provider mutation,
   automatic sandboxing default, production credential flow, or hosted
   production claim remains blocked.

## Milestone Status

The next infrastructure milestones are deliberately larger vertical builds:

1. **Operational embedded durable state:** complete the cooperating writer
   guard, atomic filesystem-to-SQLite import, verification, explicit activation,
   recovery posture, and bounded operator entry point as one integrated build.
2. **Shared PostgreSQL state:** implement the shared adapter, transactional
   mutation families, revisions, fenced leases, concurrent-worker conformance,
   migration/recovery posture, and one shared consumer path as one milestone.
3. **Single-tenant hosted alpha:** add a narrow authenticated remote API,
   PostgreSQL state, stateless workers, one reviewed execution/credential
   boundary, observability, and recovery runbook without claiming
   multi-tenancy.
4. **Collaborative team beta:** add identity, projects, shared catalog
   versioning, approval routing, ownership/escalation, notifications, and
   tenant-aware governance.
5. **Enterprise hosted readiness:** add verified tenant isolation, high
   availability, disaster recovery, retention, credential lifecycle, quotas,
   SLOs, and enterprise stewardship.

These builds consolidate delivery ceremony; they do not relax fail-closed
governance or authorize a capability before its complete acceptance criteria
pass.

| Milestone | Status | Current boundary |
| --- | --- | --- |
| Local deterministic kernel | Implemented | Local-first, sequential, durable event state |
| Governed multi-step workflows | Implemented | Sequential execution; no general parallel or branching runtime |
| Evidence, reports, approvals, and policy gates | Implemented foundations | Selected runtime composition is explicit; several defaults remain opt-in |
| Existing-repository onboarding | Implemented preview | Safe metadata and review-only recommendations; no automatic workflow activation |
| SideEffect governance | Implemented foundations | Lifecycle, persistence, discovery, approval linkage, and artifact gates exist |
| Scoped authority and capability projection | Implemented foundations | Grant, availability, resolution, request review, and pure step projection exist; context projection, receipts, and enforcement remain future |
| First provider-write sandbox | Active | GitHub PR comments only, explicit live-sandbox path, no default writes |
| Broader write-capable adapters | Not started | Requires acceptance of the first complete provider-write proof |
| Operational embedded durable state | Implemented local opt-in vertical slice | Guarded atomic filesystem-to-SQLite staging import, canonical/projection verification, exact-receipt activation, retained source, and bounded CLI exist; automatic selection, source cleanup, shared state, and production-readiness claims do not |
| Shared PostgreSQL state | Accepted | [Plan](docs/implementation-plans/shared-postgresql-state-plan.md), [report](docs/concepts/SHARED_POSTGRESQL_STATE_REPORT.md), and [review](docs/concepts/SHARED_POSTGRESQL_STATE_REVIEW.md) cover the explicit adapter, transaction families, revisions, fenced leases, shared consumer, projection rebuild, concurrent CI conformance, and recovery rehearsal; hosted operation, automatic selection, production TLS/pooling/HA, and production-readiness claims remain excluded |
| Single-tenant hosted alpha | Implemented and reviewed for one no-write evaluation trust domain | [Plan](docs/implementation-plans/single-tenant-hosted-alpha-plan.md), foundation [report](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_REVIEW.md), runtime-composition [report](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_RUNTIME_COMPOSITION_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_RUNTIME_COMPOSITION_REVIEW.md), dispatch/result [report](docs/concepts/SINGLE_TENANT_HOSTED_DISPATCH_RESULT_PROJECTION_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_DISPATCH_RESULT_PROJECTION_REVIEW.md), provider-outcome [report](docs/concepts/SINGLE_TENANT_HOSTED_PROVIDER_OUTCOME_PROJECTION_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_PROVIDER_OUTCOME_PROJECTION_REVIEW.md), deployment/recovery [report](docs/concepts/SINGLE_TENANT_HOSTED_DEPLOYMENT_RECOVERY_PROOF_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_DEPLOYMENT_RECOVERY_PROOF_REVIEW.md), [runtime guide](docs/runtime/single-tenant-hosted-alpha.md), and focused [threat model](docs/security/single-tenant-hosted-alpha-threat-model.md) cover one authenticated trust domain, shared state, proof-enforced run/approval/cancellation paths, durable attempts, stateless fenced workers, an explicit no-write execution provider, Core-owned atomic dispatch and terminal report projection, failure/reconciliation projection, live PostgreSQL recovery proof, and a deployed API/worker restart rehearsal; no multi-tenancy, enterprise identity, access-material resolver, OpenShell integration, broader writes, HA/PITR, or production-readiness claim |
| Collaborative workflow/catalog state | Future | Local and Git-backed posture precedes the selected shared durable store and migration plan |
| Composable Harness Contracts | Future | Model and runtime work follows stable governance and typed handoffs |
| Reasoning Lineage / Claim Graph | Future | Must not interrupt provider-write correctness or preview readiness |
| Hosted/distributed production backend | Future | Deferred until local contracts and operational boundaries stabilize |

Recent external dogfood feedback is reconciled against the current repository
in [External Dogfood Feedback Reconciliation](docs/concepts/EXTERNAL_DOGFOOD_FEEDBACK_RECONCILIATION.md).
The review confirms that agent-instruction preservation, safe metadata-aware
onboarding, concise/verbose first-run output, mock-demo separation, independent
proportional-governance decision axes, deterministic workload derivation, input
fingerprint invalidation, and immutable run binding are already implemented for
their accepted boundaries. Exact retry and approval-resume reassessment are now
implemented for the explicit opt-in local path. Registered source-bound runtime
fact freshness is implemented and reviewed as a same-call Core model/helper,
but executor adoption and durable replay semantics remain open. The remaining
load-bearing gaps are independent check attestation, one explicit executor
consumer for fresh source-bound facts, actor-bound time-of-use authority
enforcement, and broader integrity-safe report/export composition. Capability
resolution and pure step projection are accepted; no new provider mutation
family should precede these authority and proof boundaries.

## Current Product Boundary

Workflow OS currently does not provide default provider mutations, broad
write-capable adapters, hidden credential loading, automatic provider retries or
recovery, hosted/distributed execution, production nested harness execution,
reasoning lineage, agent swarms, recursive agents, or Level 3/4 autonomy by
default. Historical implementation detail and accepted/deferred boundaries remain
documented in the capability sections below.

Future demo workflow concepts are captured in [Workflow OS Demo Workflow Portfolio](docs/concepts/WORKFLOW_OS_DEMO_WORKFLOW_PORTFOLIO.md). They are candidate examples and benchmark narratives only; they do not implement schemas, runtime behavior, writes, hosted behavior, recursive agents, agent swarms, or release posture changes.

The repo-local `dg/*` workflows are Workflow OS's internal dogfood benchmark workflows for building Workflow OS itself. They are not downstream plug-and-play assets. The generalized user path is `workflow-os init-repo-governance`, `workflow-os first-run`, review-only recommendations, and explicit user-authored or promoted workflows.

Current-product contract hardening is implemented in [Current Product Contract Hardening Report](docs/concepts/CURRENT_PRODUCT_CONTRACT_HARDENING_REPORT.md), following [Current Product Contract Hardening Plan](docs/implementation-plans/current-product-contract-hardening-plan.md), and accepted in [Current Product Contract Hardening Review](docs/concepts/CURRENT_PRODUCT_CONTRACT_HARDENING_REVIEW.md). Recent external testing confirmed that the local governance kernel is credible, but the next preview-readiness risk was user-facing contract clarity: CLI version/build identity, docs truth, scaffold-file documentation, first-run operator UX, and the bridge from review-only recommendations to governed workflow authoring. The hardening pass patched stale current-product docs, added explicit CLI version documentation, verified existing regression coverage for version, scaffold preservation, first-run summary, safe metadata, and recommendation authoring surfaces, and kept the current boundary intact. It does not authorize provider writes, automatic workflow generation, automatic local check execution, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes.

Dogfood approval-presentation hardening is now the active self-governance lane. The repo-local runner persists bounded approval-presentation proof during material `phase-start` runs, and dogfood approval enforcement is implemented in [Dogfood Runner Approval-Presentation Enforcement Plan](docs/implementation-plans/dogfood-runner-approval-presentation-enforcement-plan.md) and [Dogfood Runner Approval-Presentation Enforcement Implementation Report](docs/concepts/DOGFOOD_RUNNER_APPROVAL_PRESENTATION_ENFORCEMENT_IMPLEMENTATION_REPORT.md). Material dogfood phase-start output now prints a proof-enforced approval command that passes the persisted `presentation_id` and a bounded max-age freshness policy into the existing opt-in enforcement path; freshness hardening is documented in [Dogfood Approval-Presentation Freshness Enforcement Report](docs/concepts/DOGFOOD_APPROVAL_PRESENTATION_FRESHNESS_ENFORCEMENT_REPORT.md). Bounded approval proof marker inspect/projection is implemented and accepted in [Approval Event Proof Marker Inspect Projection Review](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_INSPECT_PROJECTION_REVIEW.md). WorkReport and audit citation behavior for proof markers is planned in [Approval Proof Marker WorkReport And Audit Citation Plan](docs/implementation-plans/approval-proof-marker-workreport-audit-citation-plan.md), the first pure in-memory citation derivation helper is implemented in [Approval Proof Marker Citation Helper Report](docs/concepts/APPROVAL_PROOF_MARKER_CITATION_HELPER_REPORT.md) and accepted in [Approval Proof Marker Citation Helper Review](docs/concepts/APPROVAL_PROOF_MARKER_CITATION_HELPER_REVIEW.md), terminal report opt-in integration is implemented in [Terminal Report Approval Proof Marker Citation Integration Report](docs/concepts/TERMINAL_REPORT_APPROVAL_PROOF_MARKER_CITATION_INTEGRATION_REPORT.md) and accepted in [Terminal Report Approval Proof Marker Citation Integration Review](docs/concepts/TERMINAL_REPORT_APPROVAL_PROOF_MARKER_CITATION_INTEGRATION_REVIEW.md), executor report input propagation is implemented in [Executor Proof Marker Citation Report Input Propagation Report](docs/concepts/EXECUTOR_PROOF_MARKER_CITATION_REPORT_INPUT_PROPAGATION_REPORT.md) and accepted in [Executor Proof Marker Citation Report Input Propagation Review](docs/concepts/EXECUTOR_PROOF_MARKER_CITATION_REPORT_INPUT_PROPAGATION_REVIEW.md), audit projection persistence planning is documented in [Approval Proof Marker Audit Projection Persistence Plan](docs/implementation-plans/approval-proof-marker-audit-projection-persistence-plan.md) and accepted in [Approval Proof Marker Audit Projection Persistence Plan Review](docs/concepts/APPROVAL_PROOF_MARKER_AUDIT_PROJECTION_PERSISTENCE_PLAN_REVIEW.md), the first pure in-memory audit projection posture helper is implemented in [Approval Proof Marker Audit Projection Helper Report](docs/concepts/APPROVAL_PROOF_MARKER_AUDIT_PROJECTION_HELPER_REPORT.md) and accepted in [Approval Proof Marker Audit Projection Helper Review](docs/concepts/APPROVAL_PROOF_MARKER_AUDIT_PROJECTION_HELPER_REVIEW.md), durable local audit projection persistence is implemented as an explicit helper in [Approval Proof Marker Durable Audit Projection Persistence Helper Report](docs/concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_HELPER_REPORT.md) and accepted in [Approval Proof Marker Durable Audit Projection Persistence Helper Review](docs/concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_HELPER_REVIEW.md), following [Approval Proof Marker Durable Audit Projection Persistence Plan](docs/implementation-plans/approval-proof-marker-durable-audit-projection-persistence-plan.md) and accepted planning in [Approval Proof Marker Durable Audit Projection Persistence Plan Review](docs/concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_PLAN_REVIEW.md), and the first pure in-memory report artifact proof-marker gate helper is implemented in [Report Artifact Approval Proof Marker Gate Helper Report](docs/concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REPORT.md), following [Report Artifact Approval Proof Marker Gate Plan](docs/implementation-plans/report-artifact-approval-proof-marker-gate-plan.md), and accepted in [Report Artifact Approval Proof Marker Gate Helper Review](docs/concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REVIEW.md). Store-backed report artifact proof-marker gate integration planning is documented in [Report Artifact Approval Proof Marker Store-Backed Gate Integration Plan](docs/implementation-plans/report-artifact-approval-proof-marker-store-backed-gate-integration-plan.md), and the first explicit store-backed validation helper is implemented in [Report Artifact Approval Proof Marker Store-Backed Gate Helper Report](docs/concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_STORE_BACKED_GATE_HELPER_REPORT.md) and accepted in [Report Artifact Approval Proof Marker Store-Backed Gate Helper Review](docs/concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_STORE_BACKED_GATE_HELPER_REVIEW.md). Helper-level artifact-write composition with the store-backed proof-marker gate is implemented in [Report Artifact Proof-Marker Write Composition Helper Report](docs/concepts/REPORT_ARTIFACT_PROOF_MARKER_WRITE_COMPOSITION_HELPER_REPORT.md), following [Report Artifact Proof-Marker Write Composition Plan](docs/implementation-plans/report-artifact-proof-marker-write-composition-plan.md), and accepted in [Report Artifact Proof-Marker Write Composition Helper Review](docs/concepts/REPORT_ARTIFACT_PROOF_MARKER_WRITE_COMPOSITION_HELPER_REVIEW.md). Executor artifact path proof-marker gate integration is implemented in [Executor Artifact Proof-Marker Gate Integration Report](docs/concepts/EXECUTOR_ARTIFACT_PROOF_MARKER_GATE_INTEGRATION_REPORT.md), following [Executor Artifact Proof-Marker Gate Integration Plan](docs/implementation-plans/executor-artifact-proof-marker-gate-integration-plan.md). Default public approval behavior, automatic approvals, executor default proof-marker citation behavior, automatic artifact writing, automatic projection persistence, public approval cards, CLI behavior, schemas, examples, writes, hosted behavior, and release posture changes remain unimplemented.

Dogfood approval-presentation denial proof planning is documented in [Dogfood Approval-Presentation Denial Proof Plan](docs/implementation-plans/dogfood-approval-presentation-denial-proof-plan.md), and the focused repo-local implementation is documented in [Dogfood Approval-Presentation Denial Proof Implementation Report](docs/concepts/DOGFOOD_APPROVAL_PRESENTATION_DENIAL_PROOF_IMPLEMENTATION_REPORT.md). Default public approval behavior remains unchanged.

Workflow-declared proof-marker artifact requirements are planned in [Workflow-Declared Proof-Marker Artifact Requirements Plan](docs/implementation-plans/workflow-declared-proof-marker-artifact-requirements-plan.md), the first internal model/policy-mapping slice is implemented in [Workflow-Declared Proof-Marker Artifact Requirement Model Report](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_MODEL_REPORT.md), and accepted in [Workflow-Declared Proof-Marker Artifact Requirement Model Review](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_MODEL_REVIEW.md). Schema/parser/SDK vocabulary is implemented in [Workflow-Declared Proof-Marker Artifact Requirement Schema Report](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md), following [Workflow-Declared Proof-Marker Artifact Requirement Schema Plan](docs/implementation-plans/workflow-declared-proof-marker-artifact-requirement-schema-plan.md), and accepted in [Workflow-Declared Proof-Marker Artifact Requirement Schema Review](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_REQUIREMENT_SCHEMA_REVIEW.md). The pure runtime derivation helper is implemented in [Workflow-Declared Proof-Marker Artifact Runtime Derivation Helper Report](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_RUNTIME_DERIVATION_HELPER_REPORT.md), following [Workflow-Declared Proof-Marker Artifact Runtime Derivation Plan](docs/implementation-plans/workflow-declared-proof-marker-artifact-runtime-derivation-plan.md), accepted in [Workflow-Declared Proof-Marker Artifact Runtime Derivation Helper Review](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_RUNTIME_DERIVATION_HELPER_REVIEW.md), and explicit executor artifact-path integration is implemented in [Workflow-Declared Proof-Marker Artifact Executor Integration Report](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_EXECUTOR_INTEGRATION_REPORT.md), following [Workflow-Declared Proof-Marker Artifact Executor Integration Plan](docs/implementation-plans/workflow-declared-proof-marker-artifact-executor-integration-plan.md), and accepted in [Workflow-Declared Proof-Marker Artifact Executor Integration Review](docs/concepts/WORKFLOW_DECLARED_PROOF_MARKER_ARTIFACT_EXECUTOR_INTEGRATION_REVIEW.md). Executor-adjacent approval proof-marker projection persistence is implemented in [Executor-Adjacent Approval Proof-Marker Projection Persistence Helper Report](docs/concepts/EXECUTOR_ADJACENT_APPROVAL_PROOF_MARKER_PROJECTION_PERSISTENCE_HELPER_REPORT.md), following [Executor-Adjacent Approval Proof-Marker Projection Persistence Plan](docs/implementation-plans/executor-adjacent-approval-proof-marker-projection-persistence-plan.md), and accepted in [Executor-Adjacent Approval Proof-Marker Projection Persistence Helper Review](docs/concepts/EXECUTOR_ADJACENT_APPROVAL_PROOF_MARKER_PROJECTION_PERSISTENCE_HELPER_REVIEW.md). Explicit projected proof-marker artifact-path composition is implemented in [Explicit Projected Proof-Marker Artifact Composition Report](docs/concepts/EXPLICIT_PROJECTED_PROOF_MARKER_ARTIFACT_COMPOSITION_REPORT.md), following [Explicit Artifact-Path Composition Plan](docs/implementation-plans/explicit-artifact-path-composition-plan.md), and accepted in [Explicit Projected Proof-Marker Artifact Composition Review](docs/concepts/EXPLICIT_PROJECTED_PROOF_MARKER_ARTIFACT_COMPOSITION_REVIEW.md). Approval-resume artifact/projection composition is implemented in [Approval-Resume Artifact Projection Composition Report](docs/concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REPORT.md), following [Approval-Resume Artifact Projection Composition Plan](docs/implementation-plans/approval-resume-artifact-projection-composition-plan.md), and accepted in [Approval-Resume Artifact Projection Composition Review](docs/concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REVIEW.md). The helper is an explicit, local, caller-supplied-store boundary for persisting bounded approval proof-marker posture from supplied workflow run approval decision events before terminal report artifact gates need durable projection coverage. The schema now knows `report_artifact_requirements.approval_proof_markers`; `not_required` passes default validation, while `projection_required` and `marker_required` fail default semantic validation except in the explicit artifact/proof-marker-gate executor path that derives and enforces them. Default executor enforcement, automatic artifact writing, automatic projection persistence, CLI behavior, examples, provider writes, hosted behavior, reasoning lineage, and release posture changes remain unimplemented.

P0 existing-repo onboarding fixes are planned in [Existing Repo Governance Onboarding Plan](docs/implementation-plans/existing-repo-governance-onboarding-plan.md). The first scaffold slice, `workflow-os init-repo-governance`, is implemented and reviewed. The first-run ledger/report posture slice, `workflow-os first-run`, is implemented in [First-Run Governed Ledger/Report Plan](docs/implementation-plans/first-run-governed-ledger-report-plan.md). It connects, maps, documents, discloses gaps, validates bounded report sections/disclosures, and recommends first workflows/checkpoints before mature custom workflows exist. It emits a report-ready context rather than fabricating a terminal WorkReport, and it does not authorize arbitrary command execution, write-capable adapters, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy. Corrected existing-repo onboarding retesting identified follow-up P0 UX fixes, implemented in [Onboarding Retest P0 Fixes Report](docs/concepts/ONBOARDING_RETEST_P0_FIXES_REPORT.md), to make `first-run` part of the scaffolded next-step path, make generated agent instructions portable to downstream repos, improve missing-manifest guidance, and clarify optional schema doctor posture. Additional real-repository onboarding testing identified the next P0 UX lane in [Real-Repo Onboarding UX Plan](docs/implementation-plans/real-repo-onboarding-ux-plan.md) and [Real-Repo Onboarding UX Plan Report](docs/concepts/REAL_REPO_ONBOARDING_UX_PLAN_REPORT.md): preserve existing agent guidance by default, make `first-run` safe-repo-metadata-aware, produce more concrete review-only recommendations, and clearly separate real first-run posture from the optional mock approval/audit demo. The first implementation slice now preserves existing `AGENTS.md` content by default in `init-repo-governance` and `init-agent-harness`, updates managed Workflow OS blocks in place, keeps explicit `--force` replacement, and emits bounded dry-run/preservation messages without echoing existing file content. The second implementation slice adds bounded `package.json`/TypeScript first-run metadata detection and review-only recommendations, documented in [Safe Repo Metadata First-Run Recommendations Report](docs/concepts/SAFE_REPO_METADATA_FIRST_RUN_RECOMMENDATIONS_REPORT.md); it reports script keys, package-manager posture, TypeScript markers, conventional source/test directories, GitHub workflow counts, and common repo-document presence without executing commands, reading source contents, copying script bodies, or auto-generating workflows. The third implementation slice adds a concise human `what_matters_now` summary and labels the generated mock workflow command as an optional approval/audit demo, documented in [First-Run Summary And Demo Separation Report](docs/concepts/FIRST_RUN_SUMMARY_DEMO_SEPARATION_REPORT.md). The fourth implementation slice adds `workflow-os first-run --verbose`, keeping default human output concise while preserving the full bounded posture matrix for users who want audit detail. The fifth implementation slice adds bounded Rust, Python, Go, and GitHub Actions metadata labels plus concrete review-only first-run recommendations, documented in [Broader Ecosystem First-Run Metadata Report](docs/concepts/BROADER_ECOSYSTEM_FIRST_RUN_METADATA_REPORT.md) and accepted with non-blocking follow-ups in [Broader Ecosystem First-Run Metadata Review](docs/concepts/BROADER_ECOSYSTEM_FIRST_RUN_METADATA_REVIEW.md); it reports manifest/lockfile/count labels only and still does not read manifest bodies, execute commands, inspect source contents, call providers, generate workflows, or register workflows. The sixth implementation slice adds bounded recommendation next-action hints to default, verbose, and preview JSON first-run output, documented in [First-Run Recommendation Next-Action Report](docs/concepts/FIRST_RUN_RECOMMENDATION_NEXT_ACTION_REPORT.md) and accepted with non-blocking follow-ups in [First-Run Recommendation Next-Action Review](docs/concepts/FIRST_RUN_RECOMMENDATION_NEXT_ACTION_REVIEW.md); it makes recommendations easier to review and author explicitly without automatic workflow generation, command execution, provider calls, schemas, examples, writes, hosted behavior, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes. The follow-on bounded detail surface is implemented as `workflow-os first-run --recommendation <id>` in [First-Run Recommendation Detail Implementation Report](docs/concepts/FIRST_RUN_RECOMMENDATION_DETAIL_IMPLEMENTATION_REPORT.md), following [First-Run Recommendation Detail Plan](docs/implementation-plans/first-run-recommendation-detail-plan.md), and accepted in [First-Run Recommendation Detail Implementation Review](docs/concepts/FIRST_RUN_RECOMMENDATION_DETAIL_IMPLEMENTATION_REVIEW.md); it lets operators inspect why an individual recommendation exists while keeping recommendations review-only and non-mutating. Governed workflow authoring is now planned in [Governed Workflow Authoring Plan](docs/implementation-plans/governed-workflow-authoring-plan.md), and the first helper-only slice is implemented in [Governed Workflow Draft Proposal Implementation Report](docs/concepts/GOVERNED_WORKFLOW_DRAFT_PROPOSAL_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Draft Proposal Implementation Review](docs/concepts/GOVERNED_WORKFLOW_DRAFT_PROPOSAL_IMPLEMENTATION_REVIEW.md): recommendation-to-draft authoring now exposes inactive proposal obligations through recommendation detail output. The dry-run CLI boundary is implemented in [Governed Workflow Authoring CLI Dry-Run Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_CLI_DRY_RUN_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Authoring CLI Dry-Run Implementation Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_CLI_DRY_RUN_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow --from-recommendation <id> --dry-run` previews inactive authoring obligations before any future workflow generation, file writing, registration, command execution, provider calls, schemas, examples, hosted behavior, or writes are considered. The explicit inactive file-output boundary is implemented in [Governed Workflow Authoring File Output Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_FILE_OUTPUT_IMPLEMENTATION_REPORT.md), following [Governed Workflow Authoring File Output Plan](docs/implementation-plans/governed-workflow-authoring-file-output-plan.md): `workflow-os author workflow --from-recommendation <id> --output workflows/drafts/<name>.workflow.yml` writes one review-only draft under `workflows/drafts/`, checks path safety and duplicate workflow ids, refuses overwrites, and still avoids workflow registration, promotion, command execution, provider calls, runtime state, schemas, examples, hosted behavior, write-capable adapters, and release posture changes.

Governed workflow authoring promotion and steward review is planned in [Governed Workflow Authoring Promotion And Steward Review Plan](docs/implementation-plans/governed-workflow-authoring-promotion-plan.md). Promotion is the boundary where an inactive draft becomes an active workflow spec, and it must require deterministic preflight, owner/escalation completion, policy/evidence/check/report posture, conflict checks, and explicit steward or delegated maintainer approval. The first preflight-only implementation is documented in [Governed Workflow Authoring Promotion Preflight Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Authoring Promotion Preflight Implementation Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_PROMOTION_PREFLIGHT_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow preflight --draft workflows/drafts/<name>.workflow.yml` inspects one inactive draft, reports bounded blocker/warning codes, checks active workflow id conflicts, and validates the draft as a candidate without moving files, registering workflows, activating drafts, creating runtime state, executing commands, calling providers, adding schemas, adding examples, enabling writes, or changing release posture. The steward-review boundary is planned in [Governed Workflow Authoring Steward Review Plan](docs/implementation-plans/governed-workflow-authoring-steward-review-plan.md) and documented in [Governed Workflow Authoring Steward Review Plan Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_PLAN_REPORT.md); the first pure in-memory steward-review helper is implemented in [Governed Workflow Authoring Steward Review Helper Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_HELPER_REPORT.md) and accepted in [Governed Workflow Authoring Steward Review Helper Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_HELPER_REVIEW.md). The bounded CLI preview surface is implemented in [Governed Workflow Authoring Steward Review CLI Preview Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_CLI_PREVIEW_IMPLEMENTATION_REPORT.md), following [Governed Workflow Authoring Steward Review CLI Preview Plan](docs/implementation-plans/governed-workflow-authoring-steward-review-cli-preview-plan.md), and accepted in [Governed Workflow Authoring Steward Review CLI Preview Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_STEWARD_REVIEW_CLI_PREVIEW_REVIEW.md): `workflow-os author workflow steward-review --draft ...` derives fresh preflight context, calls the existing in-memory helper, and prints a bounded review card and decision result. The first active promotion implementation is documented in [Governed Workflow Authoring Active Promotion Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_ACTIVE_PROMOTION_IMPLEMENTATION_REPORT.md), following [Governed Workflow Authoring Active Promotion Plan](docs/implementation-plans/governed-workflow-authoring-active-promotion-plan.md), and accepted in [Governed Workflow Authoring Active Promotion Implementation Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_ACTIVE_PROMOTION_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow promote --draft ...` derives fresh preflight context, validates the candidate in active-placement context before writing, requires explicit reviewer/reason input, refuses active-path overwrites, writes exactly one active workflow file, preserves the draft, and reloads validation after the write. Draft cleanup and supersession semantics are planned in [Governed Workflow Authoring Draft Cleanup And Supersession Plan](docs/implementation-plans/governed-workflow-authoring-draft-cleanup-plan.md) and documented in [Governed Workflow Authoring Draft Cleanup Plan Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_CLEANUP_PLAN_REPORT.md); the first non-mutating draft-status inspection command is implemented in [Governed Workflow Authoring Draft Status Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_STATUS_IMPLEMENTATION_REPORT.md) and accepted in [Governed Workflow Authoring Draft Status Implementation Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_STATUS_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow draft-status --draft ...` reports active candidate, promoted-preserved, or superseded-by-active status without moving, editing, deleting, archiving, promoting, registering, creating runtime state, executing commands, calling providers, adding schemas, adding examples, enabling writes, or changing release posture. The first explicit archive command is implemented in [Governed Workflow Authoring Draft Archive Command Implementation Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_ARCHIVE_COMMAND_IMPLEMENTATION_REPORT.md), following [Governed Workflow Authoring Draft Archive Command Plan](docs/implementation-plans/governed-workflow-authoring-draft-archive-command-plan.md), and accepted in [Governed Workflow Authoring Draft Archive Command Implementation Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_DRAFT_ARCHIVE_COMMAND_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow archive-draft --draft ... --reviewer ... --reason ...` supports dry-run preview and moves exactly one eligible promoted/superseded draft into `workflows/drafts/archive/` while refusing active candidates and archive overwrite. Workflow authoring catalog and persisted stewardship are planned in [Governed Workflow Authoring Catalog And Stewardship Plan](docs/implementation-plans/governed-workflow-authoring-catalog-stewardship-plan.md), documented in [Governed Workflow Authoring Catalog And Stewardship Plan Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_CATALOG_STEWARDSHIP_PLAN_REPORT.md), implemented as core model only in [Governed Workflow Authoring Catalog And Stewardship Core Model Report](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_CATALOG_STEWARDSHIP_CORE_MODEL_REPORT.md), and accepted in [Governed Workflow Authoring Catalog And Stewardship Core Model Review](docs/concepts/GOVERNED_WORKFLOW_AUTHORING_CATALOG_STEWARDSHIP_CORE_MODEL_REVIEW.md). Local catalog persistence and promotion/archive integration are planned in [Workflow Catalog Persistence And Stewardship Integration Plan](docs/implementation-plans/workflow-catalog-persistence-plan.md). The first local workflow catalog store helper is implemented in [Workflow Catalog Store Helper Report](docs/concepts/WORKFLOW_CATALOG_STORE_HELPER_REPORT.md) and accepted in [Workflow Catalog Store Helper Review](docs/concepts/WORKFLOW_CATALOG_STORE_HELPER_REVIEW.md): it writes and reads validated catalog, stewardship, and archive records under caller-supplied `.workflow-os/catalog/`-style roots with encoded file names, duplicate rejection, deterministic listing, health summaries, and non-leaking errors. Workflow catalog indexing and conflict helper planning is documented in [Workflow Catalog Indexing And Conflict Helper Plan](docs/implementation-plans/workflow-catalog-indexing-conflict-plan.md). The pure in-memory indexing/conflict helper is implemented in [Workflow Catalog Indexing Conflict Helper Report](docs/concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REPORT.md): it builds deterministic catalog inventory and bounded conflict disclosures from explicit active workflow, draft, archived draft, catalog, stewardship, and archive inputs. Maintainer review in [Workflow Catalog Indexing Conflict Helper Review](docs/concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_REVIEW.md) found one blocker: serde deserialization can bypass constructor validation for exported helper summary and conflict types. The blocker fix is documented in [Workflow Catalog Indexing Conflict Helper Blocker Fix Report](docs/concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_BLOCKER_FIX_REPORT.md) and accepted in [Workflow Catalog Indexing Conflict Helper Blocker Fix Review](docs/concepts/WORKFLOW_CATALOG_INDEXING_CONFLICT_HELPER_BLOCKER_FIX_REVIEW.md). Workflow catalog command integration is planned in [Workflow Catalog Command Integration Plan](docs/implementation-plans/workflow-catalog-command-integration-plan.md), and the first non-mutating status command is implemented in [Workflow Catalog Status Command Report](docs/concepts/WORKFLOW_CATALOG_STATUS_COMMAND_REPORT.md): `workflow-os author workflow catalog-status` consumes loader-visible active workflows, inactive drafts, archived drafts, and optional local catalog-store records through the reviewed index helper, then reports bounded inventory and conflict summaries without writing files or creating catalog roots. Opt-in promotion catalog record writing is implemented in [Promotion Catalog Write Implementation Report](docs/concepts/PROMOTION_CATALOG_WRITE_IMPLEMENTATION_REPORT.md). Persisted approvals, workflow catalog registration beyond loader-visible file placement, draft deletion, automatic archive cleanup, runtime state, schemas, examples, hosted behavior, writes, and release posture changes remain unimplemented; archive metadata catalog writes are implemented only as explicit local archive sidecars.

Workflow catalog persistence integration planning has been refreshed after the accepted status command review. Opt-in steward-review persistence is implemented in [Workflow Catalog Steward Review Persistence Report](docs/concepts/WORKFLOW_CATALOG_STEWARD_REVIEW_PERSISTENCE_REPORT.md) and accepted in [Workflow Catalog Steward Review Persistence Review](docs/concepts/WORKFLOW_CATALOG_STEWARD_REVIEW_PERSISTENCE_REVIEW.md): `workflow-os author workflow steward-review --persist-stewardship` writes one validated local catalog stewardship record for an explicit review decision while preserving the default preview-only behavior. Opt-in promotion catalog writes are implemented in [Promotion Catalog Write Implementation Report](docs/concepts/PROMOTION_CATALOG_WRITE_IMPLEMENTATION_REPORT.md), following [Promotion Catalog Write Plan](docs/implementation-plans/promotion-catalog-write-plan.md), and accepted in [Promotion Catalog Write Implementation Review](docs/concepts/PROMOTION_CATALOG_WRITE_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow promote --persist-catalog-record` writes one validated workflow catalog record after active promotion validation and may cite a verified persisted stewardship decision. Archive metadata writes are implemented in [Archive Metadata Write Implementation Report](docs/concepts/ARCHIVE_METADATA_WRITE_IMPLEMENTATION_REPORT.md), following [Archive Metadata Write Plan](docs/implementation-plans/archive-metadata-write-plan.md), and accepted in [Archive Metadata Write Implementation Review](docs/concepts/ARCHIVE_METADATA_WRITE_IMPLEMENTATION_REVIEW.md): `workflow-os author workflow archive-draft --persist-archive-record` writes one validated local archive sidecar for successful archive moves. Workflow catalog repair and recovery planning is documented in [Workflow Catalog Repair And Recovery Plan](docs/implementation-plans/workflow-catalog-repair-recovery-plan.md) and accepted in [Workflow Catalog Repair And Recovery Plan Review](docs/concepts/WORKFLOW_CATALOG_REPAIR_RECOVERY_PLAN_REVIEW.md). The first non-mutating repair proposal helper is implemented in [Workflow Catalog Repair Proposal Helper Report](docs/concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_HELPER_REPORT.md) and accepted in [Workflow Catalog Repair Proposal Helper Review](docs/concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_HELPER_REVIEW.md): it maps existing catalog-status conflicts into bounded review-required proposal records without reading files, writing records, applying repairs, or mutating runtime state. The read-only repair dry-run CLI surface is implemented in [Workflow Catalog Repair Dry-Run CLI Report](docs/concepts/WORKFLOW_CATALOG_REPAIR_DRY_RUN_CLI_REPORT.md) and accepted in [Workflow Catalog Repair Dry-Run CLI Review](docs/concepts/WORKFLOW_CATALOG_REPAIR_DRY_RUN_CLI_REVIEW.md): `workflow-os author workflow catalog-repair --dry-run` loads the same bounded catalog-status inputs and prints review-required repair proposals without writing files, applying repairs, deleting records, overwriting records, registering workflows, creating runtime state, or calling providers. Repair proposal review and approval is planned in [Workflow Catalog Repair Proposal Review And Approval Plan](docs/implementation-plans/workflow-catalog-repair-proposal-review-approval-plan.md) and accepted in [Workflow Catalog Repair Proposal Review And Approval Plan Review](docs/concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_REVIEW_APPROVAL_PLAN_REVIEW.md). The in-memory repair proposal review model/helper is implemented in [Workflow Catalog Repair Proposal Review Helper Report](docs/concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_REVIEW_HELPER_REPORT.md) and accepted in [Workflow Catalog Repair Proposal Review Helper Review](docs/concepts/WORKFLOW_CATALOG_REPAIR_PROPOSAL_REVIEW_HELPER_REVIEW.md): it records bounded maintainer decisions against typed repair proposals, preserves proposal identity for stale-checking, cites stable approval/policy/evidence/validation/report references, and remains in-memory only. Repair review persistence is implemented at the core store-helper boundary in [Workflow Catalog Repair Review Store Helper Report](docs/concepts/WORKFLOW_CATALOG_REPAIR_REVIEW_STORE_HELPER_REPORT.md) and accepted in [Workflow Catalog Repair Review Store Helper Review](docs/concepts/WORKFLOW_CATALOG_REPAIR_REVIEW_STORE_HELPER_REVIEW.md): `LocalWorkflowCatalogStore` can write, read, and list validated repair review sidecars under `repair-reviews/` only when the review matches a fresh proposal identity. Explicit CLI repair review write behavior is implemented in [Workflow Catalog Repair Review CLI Write Implementation Report](docs/concepts/WORKFLOW_CATALOG_REPAIR_REVIEW_CLI_WRITE_IMPLEMENTATION_REPORT.md), following [Workflow Catalog Repair Review CLI Write Plan](docs/implementation-plans/workflow-catalog-repair-review-cli-write-plan.md): `workflow-os author workflow catalog-repair review --dry-run --persist-review ...` recomputes fresh proposals, selects exactly one proposal id, constructs a bounded review, and writes exactly one local repair review sidecar without applying repairs. Repair apply mode, automatic repair, deletion, overwrite, workflow runtime registration, schemas, examples, hosted behavior, provider calls, and release posture changes remain deferred.

First-run authoring command guidance is implemented in [First-Run Authoring Command Guidance Implementation Report](docs/concepts/FIRST_RUN_AUTHORING_COMMAND_GUIDANCE_IMPLEMENTATION_REPORT.md), following [First-Run Authoring Command Guidance Plan](docs/implementation-plans/first-run-authoring-command-guidance-plan.md). Default `workflow-os first-run` output now makes the existing non-mutating recommendation detail and authoring dry-run commands visible for one already-computed recommendation without automatically generating, writing, registering, promoting, running, approving, or validating workflows.

First-run detected-vs-scaffolded metadata clarity is implemented in [First-Run Metadata Provenance Clarity Report](docs/concepts/FIRST_RUN_METADATA_PROVENANCE_CLARITY_REPORT.md). Scaffold-only Workflow OS test directories are reported under `workflow_os_scaffold_dirs` instead of conventional repository `test_dirs`, so generated governance files are not mistaken for user repository test metadata.

P0 scaffold field operationalization is planned in [Scaffold Field Operationalization Plan](docs/implementation-plans/scaffold-field-operationalization-plan.md). The core product requirement is that rich scaffold/YAML fields must not become decorative metadata as automation increases. Every important scaffolded field should have an explicit posture: enforced, validated, disclosed, advisory, or deferred. The first implementation extends `workflow-os first-run` with a bounded governance field posture summary for ownership, escalation, profile, approvals, policy, evidence/checks, audit/observability, side-effect/capability posture, and advisory/deferred fields; it is documented in [First-Run Governance Field Posture Report](docs/concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REPORT.md) and accepted with non-blocking follow-ups in [First-Run Governance Field Posture Review](docs/concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REVIEW.md). The second implementation adds a deterministic warning-only ownership/escalation check to `workflow-os first-run`, documented in [Ownership And Escalation Check Report](docs/concepts/OWNERSHIP_ESCALATION_CHECK_REPORT.md) and accepted with non-blocking follow-ups in [Ownership And Escalation Check Review](docs/concepts/OWNERSHIP_ESCALATION_CHECK_REVIEW.md). The third implementation adds a warning-only first-run spec-field coverage check, documented in [Spec Field Coverage Check Report](docs/concepts/SPEC_FIELD_COVERAGE_CHECK_REPORT.md) and accepted with non-blocking follow-ups in [Spec Field Coverage Check Review](docs/concepts/SPEC_FIELD_COVERAGE_CHECK_REVIEW.md). Workflow discovery field coverage integration is implemented as bounded first-run recommendation output in [Workflow Discovery Field Coverage Integration Report](docs/concepts/WORKFLOW_DISCOVERY_FIELD_COVERAGE_INTEGRATION_REPORT.md), following [Workflow Discovery Field Coverage Integration Plan](docs/implementation-plans/workflow-discovery-field-coverage-integration-plan.md): recommendations now cite existing posture, ownership/escalation, and spec-field coverage codes while remaining review-only. This lane does not authorize schema changes, automatic command execution, automatic workflow generation, catalog storage, RBAC, escalation notifications, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

P0 policy effect enforcement is implemented in [Policy Effect Enforcement P0 Report](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_REPORT.md), following [Policy Effect Enforcement P0 Plan](docs/implementation-plans/policy-effect-enforcement-p0-plan.md), and reviewed in [Policy Effect Enforcement P0 Review](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_REVIEW.md). The core requirement is: declared policy effects must be enforced or rejected. Policy files must not become decorative governance strings. The first implementation adds a small typed v0 policy-effect vocabulary, rejects unsupported or misplaced effects fail-closed during validation, rejects unsupported policy-rule actor bindings, and feeds supported effects into the local executor/conservative policy boundary for approval, retry, escalation, and supported read-only adapter access. The review identified a blocker: standalone `max_attempts=N` validated as retry policy vocabulary but did not enable runtime retry. That blocker is fixed in [Policy Effect Enforcement P0 Blocker Fix Report](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_BLOCKER_FIX_REPORT.md) and accepted in [Policy Effect Enforcement P0 Blocker Fix Review](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_BLOCKER_FIX_REVIEW.md): standalone `max_attempts=N` now enables bounded retry and is covered by model and executor regression tests. This does not authorize a broad policy DSL, RBAC/IdP, hosted policy service, write-capable adapters, side-effect execution, schemas, recursive agents, agent swarms, or Level 3/4 autonomy.

High-assurance approval controls are planned in [High-Assurance Approval Controls Plan](docs/implementation-plans/high-assurance-approval-controls-plan.md), and the domain-neutral core model is implemented in [High-Assurance Approval Control Core Model Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_REPORT.md). The core model review identified a nested required-reference deserialization blocker, fixed in [High-Assurance Approval Control Core Model Blocker Fix Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REPORT.md) and accepted in [High-Assurance Approval Control Core Model Blocker Fix Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REVIEW.md). Opt-in runtime enforcement is planned in [High-Assurance Approval Runtime Enforcement Plan](docs/implementation-plans/high-assurance-approval-runtime-enforcement-plan.md), the first pure validation helper is implemented in [High-Assurance Approval Runtime Validation Helper Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_RUNTIME_VALIDATION_HELPER_REPORT.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Runtime Validation Helper Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_RUNTIME_VALIDATION_HELPER_REVIEW.md), and the first opt-in executor-integrated enforcement slice is implemented in [High-Assurance Approval Executor Enforcement Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REPORT.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Executor Enforcement Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REVIEW.md). WorkReport disclosure planning is documented in [WorkReport High-Assurance Approval Disclosure Plan](docs/implementation-plans/work-report-high-assurance-approval-disclosure-plan.md), the first explicit report-only disclosure slice is implemented in [WorkReport High-Assurance Approval Disclosure Report](docs/concepts/WORK_REPORT_HIGH_ASSURANCE_APPROVAL_DISCLOSURE_REPORT.md), the first pure high-assurance approval disclosure discovery helper is implemented in [High-Assurance Approval Disclosure Discovery Plan](docs/implementation-plans/high-assurance-approval-disclosure-discovery-plan.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Disclosure Discovery Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_DISCLOSURE_DISCOVERY_REVIEW.md), the first explicit in-memory executor/report disclosure bridge is implemented in [High-Assurance Approval Disclosure Executor/Report Integration Plan](docs/implementation-plans/high-assurance-approval-disclosure-executor-report-integration-plan.md), the explicit report artifact high-assurance disclosure gate is implemented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](docs/implementation-plans/report-artifact-high-assurance-disclosure-gate-plan.md) and accepted with non-blocking follow-ups in [Report Artifact High-Assurance Disclosure Gate Review](docs/concepts/REPORT_ARTIFACT_HIGH_ASSURANCE_DISCLOSURE_GATE_REVIEW.md), and workflow-declared high-assurance artifact requirements are planned in [Workflow-Declared High-Assurance Artifact Requirement Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-plan.md). The first internal model/policy-mapping slice is implemented in [Workflow-Declared High-Assurance Artifact Requirement Model Report](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_MODEL_REPORT.md), and the first workflow schema/parser/SDK/validation slice is implemented in [Workflow-Declared High-Assurance Artifact Requirement Schema Report](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md), following [Workflow-Declared High-Assurance Artifact Requirement Schema Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-schema-plan.md). The pure workflow-declared artifact gate derivation helper is implemented in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Report](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REPORT.md), following [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-runtime-derivation-plan.md), and accepted in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Review](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REVIEW.md). Explicit executor artifact-path integration is implemented in [Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-executor-integration-plan.md). High-assurance approval-resume artifact/projection composition is implemented in [High-Assurance Approval-Resume Artifact Projection Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_REPORT.md), following [High-Assurance Approval-Resume Artifact Projection Plan](docs/implementation-plans/high-assurance-approval-resume-artifact-projection-plan.md), accepted with non-blocking follow-ups in [High-Assurance Approval-Resume Artifact Projection Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_REVIEW.md), hardened in [High-Assurance Approval-Resume Artifact Projection Hardening Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_HARDENING_REPORT.md), and accepted with non-blocking follow-ups in [High-Assurance Approval-Resume Artifact Projection Hardening Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_HARDENING_REVIEW.md). This is a safety-sensitive prerequisite before write-capable adapter work: sensitive or irreversible actions need explicit requester/approver posture, evidence and policy context, expiration/revocation semantics, auditability, and WorkReport disclosure. The current implementation provides model vocabulary, an explicit in-memory validation helper, an opt-in executor approval decision method, explicit report input/disclosure propagation, a pure disclosure derivation helper, an additive approval decision-with-disclosure executor API, an explicit opt-in report artifact disclosure gate, an internal artifact requirement-to-gate policy mapping model, a workflow schema field, a pure workflow-to-artifact-gate derivation helper, an explicit artifact-capable executor path that derives workflow-declared gate policy and composes it with caller policy by strictness, and an explicit high-assurance approval-resume artifact/projection helper that requires durable approval-presentation proof plus high-assurance validation before projection-backed artifact writing. The hardening follow-up now derives effective artifact policy before approval mutation and covers exact-helper denial, projection failure, same-actor rejection, and disclosure-conflict fail-closed behavior. Enforcement postures remain semantically rejected outside the explicit artifact-capable executor path; this does not implement automatic high-assurance approval enforcement, automatic report generation, automatic artifact writing from default executor paths, write-capable adapters, RBAC/IdP, quorum approval, hosted behavior, side-effect execution, CLI behavior, examples, or Level 3/4 autonomy.


Governance strictness profiles and enterprise stewardship are planned in [Governance Strictness Profiles And Stewardship Plan](docs/implementation-plans/governance-strictness-profiles-and-stewardship-plan.md). The first local disclosure model is implemented in [Governance Strictness Profile Disclosure Model Report](docs/concepts/GOVERNANCE_STRICTNESS_PROFILE_DISCLOSURE_MODEL_REPORT.md): `workflow-os first-run` now uses typed core vocabulary to disclose the current `observe_and_report` posture instead of raw strings. This is an important separation point: a single local user may choose non-blocking observe/report-only governance so the agent can execute quickly while Workflow OS standardizes evidence, skipped-check disclosure, side-effect disclosure, audit posture, and reports. Enterprise deployments need a steward/admin layer that decides which profiles are allowed, which gates require humans, which gates may be satisfied by agent-provided evidence, who owns workflows, who can approve workflow changes, and when escalation is required. The local preview does not implement admin controls, RBAC, IdP integration, hosted policy enforcement, enterprise notification/escalation, write-capable adapters, or Level 3/4 autonomy.

Cross-project architecture review notes are captured in [Metric Protocol Learnings For Workflow OS](docs/concepts/METRIC_PROTOCOL_LEARNINGS_FOR_WORKFLOW_OS.md). The review reinforces stable identity, content-addressed contracts, conformance suites, machine-readable agent safety disclosures, local-first/federation-ready store posture, and deferred signed provenance. It does not authorize a hosted registry, cryptographic signing, MCP server, write-capable adapters, or workflow schema changes.

## Governance Without Brittle Orchestration

Workflow OS is not designed to enumerate every internal agent reasoning step, tool transition, or execution edge. Agent execution remains probabilistic, adaptive, and fast. Workflow OS governs the work around that execution.

The kernel should present required steps, gates, stops, approvals, evidence obligations, side-effect disclosure requirements, validation/check requirements, typed handoffs, and final report requirements. The executor, whether Codex, Claude Code, a human, deterministic code, or a future bounded harness, performs the work inside those boundaries.

The goal is not perceived control over agents. The goal is inspectability that improves outcomes: evidence-backed work, policy-tested decisions, auditable side effects, durable logs, final reports, and workflow recommendations based on repeated governed work.

Governance should preserve automation speed. Evidence should be gathered from existing run events, validation diagnostics, adapter telemetry, local checks, side-effect records, reports, and explicit citations without interrupting every agent action. The kernel should block only at meaningful governance boundaries: missing approval, denied policy, unsafe side effect, failed validation, missing required evidence, unsupported authority, or required report closure.

This is the basis for workflow evolution. Governed work records should help humans and teams see what happened, what evidence supported it, what risks remained, and which workflows should be created, changed, split, merged, or retired. Humans should monitor and approve workflow evolution; they should not be required to hand-author every useful workflow forever.

Non-goals:

- No attempt to replace agent execution frameworks by forcing every edge into a rigid graph.
- No claim that Workflow OS controls every internal agent thought, prompt, tool choice, or reasoning transition.
- No replacement of deterministic gates, validation, approvals, and audit records with model self-review.
- No automatic workflow generation, promotion, or registry mutation in v0.
- No write-capable adapters, hosted collaboration registry, recursive agents, agent swarms, or Level 3/4 autonomy as part of this roadmap framing.

## Foundation

- Establish governance, contribution, security, release, and quality-gate standards.
- Set up the Rust workspace and TypeScript SDK workspace.
- Prepare documentation structure for concepts, specs, runtime, CLI, SDK, operations, security, and release.

## v0 Kernel

- Model canonical workflow specs in Rust.
- Define schema versioning and content hashing.
- Build validation for workflow definitions.
- Define durable state interfaces.
- Define append-only meaningful runtime events.
- Define policy, audit, and observability primitives.
- Build local-first CLI commands only after their contracts are documented.

## v0 Local Kernel Preview Release Hygiene

- Keep the public posture clear: v0 is a local kernel preview, not a production distributed runtime.
- Keep README, changelog, release readiness, known limitations, and example docs aligned.
- Keep CI green across Rust, TypeScript, docs, dependency audits, examples, and schema/SDK contracts.
- Apply release versions consistently across crates, packages, changelog, and release notes.
- Track schema/TypeScript synchronization explicitly until generated contracts exist.
- `YAML-001`: replace `serde_yaml` or isolate YAML parsing behind a maintained, bounded parser strategy before any production-readiness or malicious-spec hardening claim.
- Keep CLI JSON output marked as preview until a stable machine-output contract is designed.

## Adapter Readiness Criteria

Write-capable and production adapters should not be built until release posture and local kernel contracts are settled. Phase 2 read-only adapters are the narrow exception: they exist to prove the adapter contract against real systems without writes.

Write-capable adapter readiness is planned in [Write-Capable Adapter Readiness Plan](docs/implementation-plans/write-adapter-readiness-plan.md).

The GitHub pull request comment lane is the first provider write candidate, but remains no-provider-write:

- preflight-only helper: [Write Adapter Preflight Helper Report](docs/concepts/WRITE_ADAPTER_PREFLIGHT_HELPER_REPORT.md);
- model-only request/response boundary: [First Provider Write Candidate Plan](docs/implementation-plans/first-provider-write-candidate-plan.md);
- preflight composition: [GitHub PR Comment Preflight Composition Plan](docs/implementation-plans/github-pr-comment-preflight-composition-plan.md);
- fixture-backed adapter validation: [GitHub PR Comment Fixture Adapter Helper Report](docs/concepts/GITHUB_PR_COMMENT_FIXTURE_ADAPTER_HELPER_REPORT.md);
- proposed `SideEffectRecord` composition: [GitHub PR Comment Proposed SideEffectRecord Composition Helper Report](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_COMPOSITION_HELPER_REPORT.md);
- proposed record persistence: [GitHub PR Comment Proposed SideEffectRecord Persistence Helper Report](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_RECORD_PERSISTENCE_HELPER_REPORT.md);
- proposed side-effect event projection planning: [GitHub PR Comment Proposed SideEffect Event/Audit Projection Plan](docs/implementation-plans/github-pr-comment-side-effect-event-audit-projection-plan.md);
- proposed event construction: [GitHub PR Comment SideEffect Event Helper Report](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_HELPER_REPORT.md);
- persisted-record-to-executor-input bridge planning: [GitHub PR Comment Proposed SideEffect Event Append Plan](docs/implementation-plans/github-pr-comment-side-effect-event-append-plan.md);
- first bridge helper: [GitHub PR Comment SideEffect Event Append Helper Report](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_HELPER_REPORT.md);
- bridge helper review: [GitHub PR Comment SideEffect Event Append Helper Review](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_HELPER_REVIEW.md);
- executor append proof: [GitHub PR Comment SideEffect Event Append Executor Proof Report](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_EXECUTOR_PROOF_REPORT.md);
- executor append proof review: [GitHub PR Comment SideEffect Event Append Executor Proof Review](docs/concepts/GITHUB_PR_COMMENT_SIDE_EFFECT_EVENT_APPEND_EXECUTOR_PROOF_REVIEW.md);
- report artifact citation plan: [GitHub PR Comment Report Artifact Citation Plan](docs/implementation-plans/github-pr-comment-report-artifact-citation-plan.md);
- report artifact citation helper: [GitHub PR Comment Report Artifact Citation Helper Report](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_CITATION_HELPER_REPORT.md);
- report artifact citation helper review: [GitHub PR Comment Report Artifact Citation Helper Review](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_CITATION_HELPER_REVIEW.md);
- helper-to-artifact-write composition plan: [GitHub PR Comment Report Artifact Write Composition Plan](docs/implementation-plans/github-pr-comment-report-artifact-write-composition-plan.md);
- artifact write composition helper: [GitHub PR Comment Report Artifact Write Composition Helper Report](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_WRITE_COMPOSITION_HELPER_REPORT.md);
- artifact write composition helper review: [GitHub PR Comment Report Artifact Write Composition Helper Review](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_WRITE_COMPOSITION_HELPER_REVIEW.md);
- artifact write composition hardening: [GitHub PR Comment Report Artifact Write Composition Hardening Report](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_WRITE_COMPOSITION_HARDENING_REPORT.md);
- artifact write composition hardening review: [GitHub PR Comment Report Artifact Write Composition Hardening Review](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_WRITE_COMPOSITION_HARDENING_REVIEW.md);
- broader executor-adjacent integration plan: [GitHub PR Comment Report Artifact Executor Integration Plan](docs/implementation-plans/github-pr-comment-report-artifact-executor-integration-plan.md);
- executor integration plan review: [GitHub PR Comment Report Artifact Executor Integration Plan Review](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_EXECUTOR_INTEGRATION_PLAN_REVIEW.md);
- explicit local executor-adjacent integration helper: [GitHub PR Comment Report Artifact Executor Integration Helper Report](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_EXECUTOR_INTEGRATION_HELPER_REPORT.md);
- explicit local executor-adjacent integration helper review: [GitHub PR Comment Report Artifact Executor Integration Helper Review](docs/concepts/GITHUB_PR_COMMENT_REPORT_ARTIFACT_EXECUTOR_INTEGRATION_HELPER_REVIEW.md);
- broader explicit artifact-write integration planning: [Report Artifact Write Integration Plan](docs/implementation-plans/report-artifact-write-integration-plan.md);
- broader explicit artifact-write integration helper: [Report Artifact Write Integration Helper Report](docs/concepts/REPORT_ARTIFACT_WRITE_INTEGRATION_HELPER_REPORT.md);
- broader explicit artifact-write integration helper review: [Report Artifact Write Integration Helper Review](docs/concepts/REPORT_ARTIFACT_WRITE_INTEGRATION_HELPER_REVIEW.md);
- executor artifact path generic helper integration planning: [Executor Report Artifact Write Integration Plan](docs/implementation-plans/executor-report-artifact-write-integration-plan.md);
- executor artifact path generic helper integration: [Executor Report Artifact Write Integration Report](docs/concepts/EXECUTOR_REPORT_ARTIFACT_WRITE_INTEGRATION_REPORT.md);
- executor artifact path generic helper integration review: [Executor Report Artifact Write Integration Review](docs/concepts/EXECUTOR_REPORT_ARTIFACT_WRITE_INTEGRATION_REVIEW.md);
- executor provider-candidate report artifact integration planning: [Executor Provider-Candidate Report Artifact Integration Plan](docs/implementation-plans/executor-provider-candidate-report-artifact-integration-plan.md);
- executor provider-candidate report artifact inputs: [Executor Provider-Candidate Report Artifact Integration Report](docs/concepts/EXECUTOR_PROVIDER_CANDIDATE_REPORT_ARTIFACT_INTEGRATION_REPORT.md);
- executor provider-candidate report artifact integration review: [Executor Provider-Candidate Report Artifact Integration Review](docs/concepts/EXECUTOR_PROVIDER_CANDIDATE_REPORT_ARTIFACT_INTEGRATION_REVIEW.md);
- provider write readiness planning: [GitHub PR Comment Provider Write Readiness Plan](docs/implementation-plans/github-pr-comment-provider-write-readiness-plan.md);
- broader runtime write-readiness checkpoint planning: [Runtime Write-Readiness Checkpoint Plan](docs/implementation-plans/runtime-write-readiness-checkpoint-plan.md);
- provider-write sandbox readiness helper: [Provider Write Sandbox Readiness Helper Report](docs/concepts/PROVIDER_WRITE_SANDBOX_READINESS_HELPER_REPORT.md);
- provider-write sandbox auth/source planning, review, and hardening: [Provider Write Sandbox Auth/Source Plan](docs/implementation-plans/provider-write-sandbox-auth-source-plan.md), [Provider Write Sandbox Auth/Source Plan Review](docs/concepts/PROVIDER_WRITE_SANDBOX_AUTH_SOURCE_PLAN_REVIEW.md), [Provider Write Sandbox Auth/Source Hardening Report](docs/concepts/PROVIDER_WRITE_SANDBOX_AUTH_SOURCE_HARDENING_REPORT.md), [Provider Write Sandbox Auth/Source Hardening Review](docs/concepts/PROVIDER_WRITE_SANDBOX_AUTH_SOURCE_HARDENING_REVIEW.md);
- GitHub PR comment live sandbox validation planning, review, explicit injected helper, and focused helper-specific test hardening: [GitHub PR Comment Live Sandbox Validation Plan](docs/implementation-plans/github-pr-comment-live-sandbox-validation-plan.md), [GitHub PR Comment Live Sandbox Validation Plan Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_PLAN_REPORT.md), [GitHub PR Comment Live Sandbox Validation Plan Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_PLAN_REVIEW.md), [GitHub PR Comment Live Sandbox Validation Helper Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HELPER_REPORT.md), [GitHub PR Comment Live Sandbox Validation Helper Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HELPER_REVIEW.md), [GitHub PR Comment Live Sandbox Validation Hardening Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HARDENING_REPORT.md), [GitHub PR Comment Live Sandbox Validation Hardening Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_VALIDATION_HARDENING_REVIEW.md);
- live sandbox runtime composition planning, review, first helper implementation, helper review, event-proof composition planning, event-proof helper implementation, blocker-finding helper review, bounded identity blocker fix, and accepting blocker-fix review: [GitHub PR Comment Live Sandbox Runtime Composition Plan](docs/implementation-plans/github-pr-comment-live-sandbox-runtime-composition-plan.md), [GitHub PR Comment Live Sandbox Runtime Composition Plan Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_PLAN_REPORT.md), [GitHub PR Comment Live Sandbox Runtime Composition Plan Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_PLAN_REVIEW.md), [GitHub PR Comment Live Sandbox Runtime Composition Helper Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_HELPER_REPORT.md), [GitHub PR Comment Live Sandbox Runtime Composition Helper Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_RUNTIME_COMPOSITION_HELPER_REVIEW.md), [GitHub PR Comment Live Sandbox Event-Proof Composition Plan](docs/implementation-plans/github-pr-comment-live-sandbox-event-proof-composition-plan.md), [GitHub PR Comment Live Sandbox Event-Proof Composition Plan Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_PLAN_REPORT.md), [GitHub PR Comment Live Sandbox Event-Proof Composition Helper Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_HELPER_REPORT.md), [GitHub PR Comment Live Sandbox Event-Proof Composition Helper Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_HELPER_REVIEW.md), [GitHub PR Comment Live Sandbox Event-Proof Composition Blocker Fix Report](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_BLOCKER_FIX_REPORT.md), [GitHub PR Comment Live Sandbox Event-Proof Composition Blocker Fix Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_SANDBOX_EVENT_PROOF_COMPOSITION_BLOCKER_FIX_REVIEW.md);
- GitHub PR comment sandbox target proof helper and review: [GitHub PR Comment Sandbox Target Proof Helper Report](docs/concepts/GITHUB_PR_COMMENT_SANDBOX_TARGET_PROOF_HELPER_REPORT.md), [GitHub PR Comment Sandbox Target Proof Helper Review](docs/concepts/GITHUB_PR_COMMENT_SANDBOX_TARGET_PROOF_HELPER_REVIEW.md);
- SideEffect lifecycle transition planning: [SideEffect Lifecycle Transition Plan](docs/implementation-plans/side-effect-lifecycle-transition-plan.md);
- SideEffect lifecycle transition plan review: [SideEffect Lifecycle Transition Plan Review](docs/concepts/SIDE_EFFECT_LIFECYCLE_TRANSITION_PLAN_REVIEW.md);
- pure SideEffect lifecycle transition helper: [SideEffect Lifecycle Transition Helper Report](docs/concepts/SIDE_EFFECT_LIFECYCLE_TRANSITION_HELPER_REPORT.md);
- pure SideEffect lifecycle transition helper review: [SideEffect Lifecycle Transition Helper Review](docs/concepts/SIDE_EFFECT_LIFECYCLE_TRANSITION_HELPER_REVIEW.md);
- store-backed SideEffect lifecycle transition planning: [SideEffect Store-Backed Lifecycle Transition Plan](docs/implementation-plans/side-effect-store-backed-lifecycle-transition-plan.md);
- store-backed SideEffect lifecycle transition plan review: [SideEffect Store-Backed Lifecycle Transition Plan Review](docs/concepts/SIDE_EFFECT_STORE_BACKED_LIFECYCLE_TRANSITION_PLAN_REVIEW.md);
- store-backed SideEffect lifecycle transition helper: [SideEffect Store-Backed Lifecycle Transition Helper Report](docs/concepts/SIDE_EFFECT_STORE_BACKED_LIFECYCLE_TRANSITION_HELPER_REPORT.md);
- executor attempted/completed/failed SideEffect lifecycle event append planning: [Executor SideEffect Lifecycle Event Append Plan](docs/implementation-plans/executor-side-effect-lifecycle-event-append-plan.md);
- executor attempted/completed/failed SideEffect lifecycle event append helper: [Executor SideEffect Lifecycle Event Append Report](docs/concepts/EXECUTOR_SIDE_EFFECT_LIFECYCLE_EVENT_APPEND_REPORT.md);
- executor attempted/completed/failed SideEffect lifecycle event append review: [Executor SideEffect Lifecycle Event Append Review](docs/concepts/EXECUTOR_SIDE_EFFECT_LIFECYCLE_EVENT_APPEND_REVIEW.md);
- no-provider-call write-adapter orchestration helper: [Write-Adapter Orchestration Helper Report](docs/concepts/WRITE_ADAPTER_ORCHESTRATION_HELPER_REPORT.md);
- no-provider-call write-adapter orchestration helper review: [Write-Adapter Orchestration Helper Review](docs/concepts/WRITE_ADAPTER_ORCHESTRATION_HELPER_REVIEW.md);
- no-provider completed/failed outcome orchestration planning: [Write-Adapter No-Provider Outcome Orchestration Plan](docs/implementation-plans/write-adapter-no-provider-outcome-orchestration-plan.md);
- no-provider completed/failed outcome orchestration implementation: [Write-Adapter No-Provider Outcome Orchestration Report](docs/concepts/WRITE_ADAPTER_NO_PROVIDER_OUTCOME_ORCHESTRATION_REPORT.md);
- no-provider completed/failed outcome orchestration review: [Write-Adapter No-Provider Outcome Orchestration Review](docs/concepts/WRITE_ADAPTER_NO_PROVIDER_OUTCOME_ORCHESTRATION_REVIEW.md);
- live provider-call boundary planning: [GitHub PR Comment Live Provider Call Plan](docs/implementation-plans/github-pr-comment-live-provider-call-plan.md);
- live provider-call boundary plan review: [GitHub PR Comment Live Provider Call Plan Review](docs/concepts/GITHUB_PR_COMMENT_LIVE_PROVIDER_CALL_PLAN_REVIEW.md);
- provider-call trait/input model implementation: [GitHub PR Comment Provider-Call Trait/Input Model Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CALL_TRAIT_INPUT_MODEL_REPORT.md);
- provider-call trait/input model review: [GitHub PR Comment Provider-Call Trait/Input Model Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CALL_TRAIT_INPUT_MODEL_REVIEW.md);
- injected provider-call orchestration helper implementation: [GitHub PR Comment Provider-Call Orchestration Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CALL_ORCHESTRATION_HELPER_REPORT.md);
- injected provider-call orchestration helper review: [GitHub PR Comment Provider-Call Orchestration Helper Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CALL_ORCHESTRATION_HELPER_REVIEW.md);
- concrete provider client/auth loading planning: [GitHub PR Comment Provider Client and Auth Loading Plan](docs/implementation-plans/github-pr-comment-provider-client-auth-loading-plan.md);
- concrete provider client/auth loading plan review: [GitHub PR Comment Provider Client/Auth Loading Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_PLAN_REVIEW.md);
- concrete injected-transport provider client implementation: [GitHub PR Comment Provider Client/Auth Loading Implementation Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REPORT.md);
- concrete injected-transport provider client review: [GitHub PR Comment Provider Client/Auth Loading Implementation Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_CLIENT_AUTH_LOADING_IMPLEMENTATION_REVIEW.md);
- provider write reconciliation planning: [GitHub PR Comment Provider Write Reconciliation Plan](docs/implementation-plans/github-pr-comment-provider-write-reconciliation-plan.md);
- provider write reconciliation plan review: [GitHub PR Comment Provider Write Reconciliation Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_RECONCILIATION_PLAN_REVIEW.md);
- provider write reconciliation model/helper implementation: [GitHub PR Comment Provider Write Reconciliation Model Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_RECONCILIATION_MODEL_REPORT.md);
- provider write reconciliation model/helper review: [GitHub PR Comment Provider Write Reconciliation Model Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_RECONCILIATION_MODEL_REVIEW.md);
- executor-integrated live provider write planning: [Executor-Integrated Live Provider Write Plan](docs/implementation-plans/executor-integrated-live-provider-write-plan.md);
- executor-integrated live provider write plan review: [Executor-Integrated Live Provider Write Plan Review](docs/concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_PLAN_REVIEW.md);
- executor-integrated live provider write request/result/helper implementation: [Executor-Integrated Live Provider Write Implementation Report](docs/concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_IMPLEMENTATION_REPORT.md);
- executor-integrated live provider write implementation review and blocker fix: [Executor-Integrated Live Provider Write Implementation Review](docs/concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_IMPLEMENTATION_REVIEW.md), [Executor-Integrated Live Provider Write Blocker Fix Report](docs/concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_BLOCKER_FIX_REPORT.md), [Executor-Integrated Live Provider Write Blocker Fix Review](docs/concepts/EXECUTOR_INTEGRATED_LIVE_PROVIDER_WRITE_BLOCKER_FIX_REVIEW.md);
- provider write event append planning: [GitHub PR Comment Provider Write Event Append Plan](docs/implementation-plans/github-pr-comment-provider-write-event-append-plan.md);
- provider write event append helper implementation: [GitHub PR Comment Provider Write Event Append Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REPORT.md);
- provider reconciliation disclosure report composition planning, first in-memory WorkReport slice, and review: [GitHub PR Comment Provider Disclosure Report Composition Plan](docs/implementation-plans/github-pr-comment-provider-disclosure-report-composition-plan.md), [GitHub PR Comment Provider Disclosure Report Composition Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_DISCLOSURE_REPORT_COMPOSITION_REPORT.md), [GitHub PR Comment Provider Disclosure Report Composition Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_DISCLOSURE_REPORT_COMPOSITION_REVIEW.md);
- provider report artifact event-proof gate helper: [GitHub PR Comment Provider Report Artifact Event-Proof Gate Plan](docs/implementation-plans/github-pr-comment-provider-report-artifact-event-proof-gate-plan.md) and [GitHub PR Comment Provider Report Artifact Event-Proof Gate Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_REPORT_ARTIFACT_EVENT_PROOF_GATE_HELPER_REPORT.md);
- provider event-proof recovery planning, first local classifier, and review: [GitHub PR Comment Provider Event-Proof Recovery Plan](docs/implementation-plans/github-pr-comment-provider-event-proof-recovery-plan.md), [GitHub PR Comment Provider Event-Proof Recovery Plan Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_EVENT_PROOF_RECOVERY_PLAN_REPORT.md), [GitHub PR Comment Provider Event-Proof Recovery Model Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_EVENT_PROOF_RECOVERY_MODEL_REPORT.md), and [GitHub PR Comment Provider Event-Proof Recovery Model Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_EVENT_PROOF_RECOVERY_MODEL_REVIEW.md);
- provider lookup/query reconciliation planning, review, first model/helper implementation, concrete lookup HTTP client planning/implementation/review, lookup integration planning, first in-memory recovery integration helper, and helper review: [GitHub PR Comment Provider Lookup/Query Reconciliation Plan](docs/implementation-plans/github-pr-comment-provider-lookup-reconciliation-plan.md), [GitHub PR Comment Provider Lookup/Query Reconciliation Plan Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_PLAN_REPORT.md), [GitHub PR Comment Provider Lookup/Query Reconciliation Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_PLAN_REVIEW.md), [GitHub PR Comment Provider Lookup Reconciliation Model Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REPORT.md), [GitHub PR Comment Provider Lookup HTTP Client Plan](docs/implementation-plans/github-pr-comment-provider-lookup-http-client-plan.md), [GitHub PR Comment Provider Lookup HTTP Client Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_HTTP_CLIENT_REVIEW.md), [GitHub PR Comment Provider Lookup Integration Plan](docs/implementation-plans/github-pr-comment-provider-lookup-integration-plan.md), [GitHub PR Comment Provider Lookup Recovery Integration Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECOVERY_INTEGRATION_HELPER_REPORT.md), and [GitHub PR Comment Provider Lookup Recovery Integration Helper Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECOVERY_INTEGRATION_HELPER_REVIEW.md);
- provider lookup operator recovery planning, review, first in-memory summary helper, helper review, local CLI planning, CLI plan review, first explicit local summary-input CLI implementation, and implementation review: [GitHub PR Comment Provider Lookup Operator Recovery Plan](docs/implementation-plans/github-pr-comment-provider-lookup-operator-recovery-plan.md), [GitHub PR Comment Provider Lookup Operator Recovery Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_PLAN_REVIEW.md), [GitHub PR Comment Provider Lookup Operator Recovery Summary Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_SUMMARY_HELPER_REPORT.md), [GitHub PR Comment Provider Lookup Operator Recovery Summary Helper Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_SUMMARY_HELPER_REVIEW.md), [GitHub PR Comment Provider Lookup Operator Recovery CLI Plan](docs/implementation-plans/github-pr-comment-provider-lookup-operator-recovery-cli-plan.md), [GitHub PR Comment Provider Lookup Operator Recovery CLI Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_PLAN_REVIEW.md), [GitHub PR Comment Provider Lookup Operator Recovery CLI Implementation Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_IMPLEMENTATION_REPORT.md), and [GitHub PR Comment Provider Lookup Operator Recovery CLI Implementation Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_IMPLEMENTATION_REVIEW.md);
- provider-call orchestration gate clarity hardening: [Provider Call Orchestration Gate Clarity Hardening Report](docs/concepts/PROVIDER_CALL_ORCHESTRATION_GATE_CLARITY_HARDENING_REPORT.md);
- provider-write runtime composition planning: [Provider-Write Runtime Composition Plan](docs/implementation-plans/provider-write-runtime-composition-plan.md);
- no-provider-call write-adapter orchestration planning: [Write-Adapter Orchestration Plan](docs/implementation-plans/write-adapter-orchestration-plan.md);
- no-provider-call write-adapter orchestration plan review: [Write-Adapter Orchestration Plan Review](docs/concepts/WRITE_ADAPTER_ORCHESTRATION_PLAN_REVIEW.md).

The explicit executor attempted/completed/failed SideEffect lifecycle event append helper is implemented and reviewed as a local opt-in path. It composes validated `SideEffectLifecycleTransitionResult` values into the executor event append boundary without provider writes, live GitHub comment creation, runtime side-effect execution, CLI mutation commands, schemas, examples, hosted behavior, reasoning lineage, recursive agents, agent swarms, Level 3/4 autonomy, or release posture changes. The smallest no-provider-call write-adapter orchestration helper is implemented and reviewed: it composes proposed record persistence, approval linkage, and store-backed attempted lifecycle transition without provider calls, event append, or artifact writes. Completed/failed no-provider outcome orchestration is implemented as an explicit local helper for fixture/dry-run/local outcome closure. Live provider-call boundary planning is documented, the provider-call trait/input model is implemented and reviewed, and the injected provider-call orchestration helper is implemented and reviewed as a narrow caller-supplied-provider path that can transition attempted records from classified provider success/failure responses. Concrete provider client/auth loading planning is documented and reviewed, and the first concrete GitHub PR comment provider client is implemented and reviewed with explicit caller-supplied auth and injected transport only. Provider write reconciliation planning is accepted, and the first model/helper-only reconciliation candidate is implemented and reviewed to classify remote-success/local-transition-failure and other ambiguous provider outcomes. Executor-integrated live provider write planning is accepted, the first explicit executor-adjacent request/result/helper slice is implemented, and the implementation-review blocker for post-provider local transition failure reconciliation is fixed and reviewed: the helper wraps local execution, invokes only a supplied provider after existing gates pass, returns an in-memory result with provider response/error and reconciliation posture, blocks retry for post-provider local transition ambiguity, and keeps default execution unchanged. Provider write event append planning is accepted, and the first explicit helper path is implemented: completed/failed SideEffect lifecycle workflow events can now be appended only for eligible reconciled provider outcomes while preserving default execution behavior. Provider reconciliation disclosure report composition is implemented and reviewed for the first in-memory WorkReport slice: explicit provider disclosure inputs can populate bounded Side Effects section posture while keeping event proof distinct from provider/local agreement. Strict report artifact event-proof gates are implemented as an explicit opt-in helper before artifact writes, with denied-posture matrix hardening documented and reviewed in [GitHub PR Comment Provider Report Artifact Event-Proof Gate Matrix Hardening Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_REPORT_ARTIFACT_EVENT_PROOF_GATE_MATRIX_HARDENING_REPORT.md) and [GitHub PR Comment Provider Report Artifact Event-Proof Gate Matrix Hardening Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_REPORT_ARTIFACT_EVENT_PROOF_GATE_MATRIX_HARDENING_REVIEW.md). Provider event-proof recovery classification is implemented as a local model/helper boundary that maps explicit disclosure posture and mismatch signals into bounded recovery posture, next-action vocabulary, retry blocking, artifact-write allowance, and operator-action posture. Provider lookup/query reconciliation is implemented and reviewed as an explicit injected-client model/helper that classifies bounded remote observations as observed, absent, ambiguous, unauthorized, unavailable, rate-limited, invalid, or untrusted while keeping report artifact writes blocked without durable workflow event proof. Concrete injected lookup HTTP client planning is documented in [GitHub PR Comment Provider Lookup HTTP Client Plan](docs/implementation-plans/github-pr-comment-provider-lookup-http-client-plan.md), the first explicit injected-transport lookup HTTP client is implemented in [GitHub PR Comment Provider Lookup HTTP Client Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_HTTP_CLIENT_REPORT.md), and the phase is accepted in [GitHub PR Comment Provider Lookup HTTP Client Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_HTTP_CLIENT_REVIEW.md). Provider lookup integration planning is documented in [GitHub PR Comment Provider Lookup Integration Plan](docs/implementation-plans/github-pr-comment-provider-lookup-integration-plan.md), and the first explicit in-memory recovery integration helper is implemented in [GitHub PR Comment Provider Lookup Recovery Integration Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECOVERY_INTEGRATION_HELPER_REPORT.md): it composes caller-supplied lookup reconciliation input, an injected lookup client, and explicit event-proof recovery context into bounded lookup/recovery posture without provider writes, automatic lookup, hidden auth, retries, repair, event append, side-effect record mutation, artifact writes, CLI output, schemas, examples, hosted behavior, approval-presentation enforcement, or release posture changes. Provider lookup operator recovery planning is accepted, and the first in-memory operator recovery summary helper is implemented in [GitHub PR Comment Provider Lookup Operator Recovery Summary Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_SUMMARY_HELPER_REPORT.md) and accepted in [GitHub PR Comment Provider Lookup Operator Recovery Summary Helper Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_SUMMARY_HELPER_REVIEW.md): it projects an already validated lookup/recovery integration result into bounded operator posture, next-action vocabulary, retry/artifact blocks, and redaction-safe metadata without performing lookup or mutating state. Local CLI exposure is implemented in [GitHub PR Comment Provider Lookup Operator Recovery CLI Implementation Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_IMPLEMENTATION_REPORT.md) as `workflow-os provider github-pr-comment recovery-summary --summary <path>` and accepted in [GitHub PR Comment Provider Lookup Operator Recovery CLI Implementation Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_OPERATOR_RECOVERY_CLI_IMPLEMENTATION_REVIEW.md): it reads explicit local serialized summary input, validates the existing model boundary, and renders bounded text or JSON recovery posture. Provider-call orchestration gate clarity hardening is implemented in [Provider Call Orchestration Gate Clarity Hardening Report](docs/concepts/PROVIDER_CALL_ORCHESTRATION_GATE_CLARITY_HARDENING_REPORT.md) and accepted in [Provider Call Orchestration Gate Clarity Hardening Review](docs/concepts/PROVIDER_CALL_ORCHESTRATION_GATE_CLARITY_HARDENING_REVIEW.md): explicit executor-integrated provider-write results now expose bounded gate posture for pre-provider context, provider call/response, post-provider local transition, workflow event proof, retry, artifact event-proof, and operator recovery while preserving existing provider-call behavior. Hidden auth loading, automatic provider lookup, automatic provider writes, automatic retries, automatic repair, workflow event append from recovery, automatic report generation, automatic report artifact writes, state lookup, CLI mutation behavior, schemas, examples, hosted behavior, approval-presentation enforcement, reasoning lineage, and release posture changes remain future scoped work.

Before any real adapter implementation:

- Adapter capability, policy, idempotency, audit, and redaction contracts must remain enforced.
- External writes must remain denied unless explicitly designed, policy-gated, audited, and idempotent.
- Adapter health, error classification, dry-run/plan behavior, and redacted response summaries must be tested.
- Docs must continue to state that adapters cannot mutate core workflow state directly.

## Phase 2 Read-Only Integration Posture

Phase 2 is the read-only integration capability phase. It is documented in [docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md](docs/integrations/PHASE_2_READ_ONLY_INTEGRATIONS.md).

The `0.2.0-preview.1` public read-only integration preview includes initial Phase 2 read-only adapters:

- GitHub read-only adapter foundation.
- Jira read-only adapter foundation.
- GitHub Actions CI read-only adapter foundation.

GitHub Actions is the first CI target for read-only adapter proving. Other CI providers remain future work.

The `0.2.0-preview.1` posture approves a narrow public read-only integration preview after live smoke evidence was recorded and reviewed. That approval is limited to read-only provider access, fixture-first normal CI, and opt-in live tests.

Read-only adapter work must not imply write support, OAuth completeness, webhook ingestion, hosted operation, distributed workers, production database readiness, production integration readiness, broad live provider compatibility, or Level 3/4 autonomy enablement.

The following remain out of scope for Phase 2:

- Creating branches.
- Opening pull requests.
- Posting pull request comments.
- Updating Jira issues or comments.
- Changing Jira status.
- Rerunning CI.
- Workflow dispatch.
- Webhooks or an event ingestion service.
- OAuth app implementation.
- External writes of any kind.

## Governed Work Pattern Architecture

[Governed Work Pattern](docs/concepts/governed-work-pattern.md) is accepted as architecture direction by [ADR 0007](docs/adr/0007-governed-work-pattern.md). Acceptance does not implement runtime behavior or authorize schemas, CLI changes, writes, generic runtime adapter execution, or domain packs.

## P0 Blocker: Governed Multi-Step Workflows

Kernel dogfooding surfaced the next product blocker: one-governance-check workflows are not enough to govern realistic work at scale. Workflow OS becomes more valuable when a run can move through multiple deterministic governed steps, each with explicit policy checks, approval semantics, validation/check references, event history, failure behavior, and final work-report citations.

Governed multi-step workflow execution is now the P0 roadmap priority. [ADR 0010: Governed Multi-Step Workflow Execution](docs/adr/0010-governed-multi-step-workflow-execution.md) is accepted, and the bounded implementation plan is [Governed Multi-Step Workflow Execution Plan](docs/implementation-plans/governed-multi-step-workflow-execution-plan.md). The first sequential local executor slice is implemented: the local executor can run one or more ordered local steps, preserve per-step policy and approval behavior, retry/fail/escalate at the current step, and return report-bearing results for completed multi-step runs. It does not introduce parallel execution, branching execution, nested harness execution, writes, hosted/distributed runtime, schemas, examples, CLI behavior, automatic report generation, or reasoning lineage.

This pivot is distinct from Composable Harness Contracts. Multi-step governed execution is the kernel prerequisite; harness contracts and nested harness execution remain later capabilities that depend on the kernel proving durable step-by-step governance first.

The first sequential local multi-step executor slice has been reviewed and hardened with focused later-step approval, retry, policy-denial, cancellation, and report-generation-failure coverage. The self-governance dogfood workflow has been converted to a sequential multi-step workflow and reviewed. A tiny follow-up docs cleanup is implemented in [Self-Governance Dogfood Docs Cleanup Plan](docs/implementation-plans/self-governance-dogfood-docs-cleanup-plan.md), aligning the implemented conversion plan's historical current-state wording with the converted workflow.

With the dogfood docs cleanup complete, [Self-Governance Dogfood Hardening Test Plan](docs/implementation-plans/self-governance-dogfood-hardening-test-plan.md) is implemented as a test-only phase covering dogfood cancellation at the planning approval checkpoint, duplicate run-id replay/rehydration behavior, and report-bearing dogfood execution through existing explicit APIs. Real command execution, default handler registration, command-output evidence, side-effect boundary implementation, writes, and nested harness runtime behavior remain deferred.

## P0 Adoption: Agent Harness By Default

User feedback showed that evaluators often begin by hand-writing YAML and manually testing the kernel. The stronger adoption path is to connect Codex, Claude Code, or another coding agent to the local kernel and instruct the agent to use Workflow OS as the governing layer.

Follow-up dogfood testing showed a second adoption gap: Workflow OS can govern projects that are already Workflow OS projects, but a normal existing repository still needs a scaffold before `workflow-os validate` or `workflow-os run` has a project contract to load. Existing-repo onboarding is now a P0 adoption lane. The goal is to help a user turn an existing repository into a valid Workflow OS project, or later create a sidecar project that governs an external target repository, without asking the user to copy Workflow OS's internal dogfood workflows or hand-author all YAML from scratch. The first scaffold slice is implemented as `workflow-os init-repo-governance` and accepted in [Existing Repo Governance Scaffold Review](docs/concepts/EXISTING_REPO_GOVERNANCE_SCAFFOLD_REVIEW.md). The follow-on first-run ledger/report posture mode is implemented as `workflow-os first-run` in [First-Run Governed Ledger/Report Plan](docs/implementation-plans/first-run-governed-ledger-report-plan.md). That first-run path carries Workflow OS's default governance opinions immediately: bounded goal, context, missing-evidence disclosure, skipped checks, approval posture, side-effect disclosure, skipped work, risks, report section closure, and review-only workflow recommendations. It does not run workflows, create runtime state, inspect raw source contents, write artifacts, or auto-register workflows.

Real-repository onboarding evaluation found that this loop is valuable but still too generic for normal repositories. The P0 UX lane is documented in [Real-Repo Onboarding UX Plan](docs/implementation-plans/real-repo-onboarding-ux-plan.md): preserve existing `AGENTS.md` content by default instead of pushing users toward destructive `--force`, inspect only safe repository metadata such as manifests and conventional directory presence, make recommendations concrete without executing commands, and label the generated mock workflow as an optional approval/audit demonstration rather than repository analysis. Existing `AGENTS.md` preservation is implemented for the scaffold commands, safe metadata-aware first-run recommendations are implemented for bounded package/TypeScript, Rust, Python, Go, and GitHub Actions metadata without executing commands, reading source contents, copying script bodies, reading manifest bodies, or auto-generating workflows, and first-run human output now separates the recommended review/setup action from the optional mock approval/audit demo.

The onboarding phase is implemented in [Agent Harness Onboarding Plan](docs/implementation-plans/agent-harness-onboarding-plan.md), [Agent Harness Quickstart](docs/user-guide/agent-harness-quickstart.md), and [AGENTS.md](AGENTS.md). The explicit scaffold command `workflow-os init-agent-harness` is implemented as documented in [Agent Harness CLI Scaffold Plan](docs/implementation-plans/agent-harness-cli-scaffold-plan.md). The scaffold has been dogfooded in [Agent Harness Scaffold Dogfood And Adoption Plan](docs/implementation-plans/agent-harness-scaffold-dogfood-adoption-plan.md). The next adoption maturity layer is planned in [Agent Harness Hook Integration Plan](docs/implementation-plans/agent-harness-hook-integration-plan.md), and the first model-only agent harness hook contract is implemented.

The intended mental model is:

```text
Agent executes. Workflow OS governs.
```

This is a P0 adoption/docs layer, not nested harness runtime behavior. The scaffold command creates or updates `AGENTS.md` and `.workflow-os/agent-harness-prompt.md` only. The scaffold is an orientation layer for humans and agents: useful for declaring conventions, expectations, and structure, but not itself an enforcement layer.

Hard side-effect checkpoint enforcement is a P0 product unlock. The intended production shape is that irreversible or customer-visible tools such as Slack sends, GitHub writes, Jira updates, deploy steps, and other adapter mutations are routed through deterministic hooks or adapter boundaries that can fail closed before the side effect happens. Workflow OS should define the governed contract, required evidence, approval or draft-only posture, event/audit record, and final report disclosure; the hook or adapter boundary should provide the mechanical block at the point of execution. Today this is not broadly implemented: Workflow OS has narrow explicit hook/checkpoint and side-effect/report primitives, but it does not yet provide production `PreToolUse`-style enforcement for arbitrary external tools, automatic workflow-declared hook configuration, runtime hook configuration, or hard blocking of external Slack/GitHub/Jira tool calls unless those calls are explicitly routed through implemented Workflow OS boundaries.

The future hook layer should provide deterministic, named checkpoints that a harness or agent invokes before or after important work phases. The hook contract model is implemented as vocabulary and validation only, and the in-memory invocation helper model is implemented as documented in [Agent Harness Hook Runtime Invocation Plan](docs/implementation-plans/agent-harness-hook-runtime-invocation-plan.md). Hook audit/event semantics planning is documented in [Agent Harness Hook Audit/Event Semantics Plan](docs/implementation-plans/agent-harness-hook-audit-event-semantics-plan.md), and the hook audit record core model is implemented as model-only vocabulary and validation.

WorkReport hook citation target planning is documented in [WorkReport Agent Harness Hook Citation Target Plan](docs/implementation-plans/work-report-hook-citation-target-plan.md), and WorkReport citation vocabulary for agent harness hook invocation IDs is implemented as model-only vocabulary. Terminal report helper hook citation integration is implemented in [Terminal Report Agent Harness Hook Citation Integration Plan](docs/implementation-plans/terminal-report-hook-citation-integration-plan.md) for explicit supplied IDs only. Executor report input propagation for hook IDs is implemented in [Executor Hook Report Input Propagation Plan](docs/implementation-plans/executor-hook-report-input-plan.md). Runtime hook execution planning is documented in [Agent Harness Hook Runtime Execution Plan](docs/implementation-plans/agent-harness-hook-runtime-execution-plan.md), and the explicit in-memory runtime hook execution helper is implemented.

Executor hook checkpoint planning is documented in [Executor Hook Checkpoint Plan](docs/implementation-plans/executor-hook-checkpoint-plan.md), and the explicit `BeforeReport` report-path checkpoint is implemented for `execute_with_report(...)` only. Deterministic required-checkpoint enforcement for `BeforeReport` is implemented as an explicit report input policy in [Deterministic Hook Checkpoint Enforcement Report](docs/concepts/DETERMINISTIC_HOOK_CHECKPOINT_ENFORCEMENT_REPORT.md). Executor hook event and audit semantics planning is documented in [Executor Hook Event And Audit Semantics Plan](docs/implementation-plans/executor-hook-event-audit-semantics-plan.md); the model-only hook workflow event vocabulary is implemented for bounded, state-preserving future hook events. Generic hook workflow event audit projection is implemented as projection-only in [Hook Event Audit Projection Plan](docs/implementation-plans/hook-event-audit-projection-plan.md), and the first explicit `BeforeSkillInvocation` executor hook event append path is implemented in [Executor Hook Event Append Plan](docs/implementation-plans/executor-hook-event-append-plan.md).

BeforeSkillInvocation status and failure semantics planning is documented in [BeforeSkillInvocation Hook Status And Failure Semantics Plan](docs/implementation-plans/before-skill-hook-status-failure-semantics-plan.md), boundary hardening tests cover later-step targeting, missing handlers, policy denial, and redaction behavior, the first explicit failed-closed result path is implemented as documented in [BeforeSkillInvocation Failed-Closed Result Path Plan](docs/implementation-plans/before-skill-hook-failed-closed-result-plan.md), warning/skipped disclosure semantics planning is documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](docs/implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md), unsupported-status hardening tests are implemented in [BeforeSkillInvocation Unsupported Status Hardening Report](docs/concepts/BEFORE_SKILL_HOOK_UNSUPPORTED_STATUS_HARDENING_REPORT.md), required pre-skill checkpoint planning is documented in [BeforeSkillInvocation Required Checkpoint Plan](docs/implementation-plans/before-skill-required-checkpoint-plan.md), the first explicit selected-step required enforcement slice is implemented in [BeforeSkillInvocation Required Checkpoint Enforcement Report](docs/concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REPORT.md), and the unknown required-step blocker is fixed in [BeforeSkillInvocation Required Checkpoint Blocker Fix Report](docs/concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_BLOCKER_FIX_REPORT.md).

Bounded hook disclosure core model implementation is documented in [Hook Disclosure Model Plan](docs/implementation-plans/hook-disclosure-model-plan.md), WorkReport hook disclosure citation vocabulary is implemented as model-only vocabulary as documented in [WorkReport Hook Disclosure Citation Plan](docs/implementation-plans/work-report-hook-disclosure-citation-plan.md), terminal report helper hook disclosure citation integration is implemented in [Terminal Report Hook Disclosure Citation Integration Plan](docs/implementation-plans/terminal-report-hook-disclosure-citation-integration-plan.md) for explicit supplied IDs only, and executor hook disclosure report input propagation is implemented in [Executor Hook Disclosure Report Input Propagation Plan](docs/implementation-plans/executor-hook-disclosure-report-input-plan.md). Hook disclosure discovery planning and the first in-memory implementation are documented in [Hook Disclosure Discovery Plan](docs/implementation-plans/hook-disclosure-discovery-plan.md); discovery is implemented only for already-validated in-memory `BeforeReport` hook results in the explicit report-bearing executor path. `Passed` remains the only continuing hook status today, while explicit `FailedClosed` fails the run before `SkillInvocationRequested`. Explicit report-bearing paths can now require a `BeforeReport` checkpoint before report generation. Local execution requests can now require `BeforeSkillInvocation` for explicit selected step IDs, failing closed before `SkillInvocationRequested` when a required hook is absent or mismatched, and unknown required step IDs fail closed before run creation. Warning/skipped/blocked status broadening, discovery from workflow events or audit projections, dedicated hook audit sink emission, hook persistence, workflow-declared hook configuration, runtime hook configuration, and broader automatic executor checkpoints are not implemented. This does not implement runtime harness auto-generation, workflow schema fields, automatic local check execution, recursive agents, agent swarms, hosted execution, writes, side-effect modeling, or Level 3/4 autonomy, and it must not silently enable command execution, writes, schemas, hosted behavior, or release posture changes.

The first scoped MVP concept is [EvidenceReference](docs/concepts/evidence-reference.md), proposed in [ADR 0009](docs/adr/0009-evidence-reference-core-model.md) with a phased implementation plan in [docs/implementation-plans/evidence-reference-mvp.md](docs/implementation-plans/evidence-reference-mvp.md). EvidenceReference Phase 1 core type model is implemented and reviewed. Adapter telemetry evidence attachment, `Diagnostic` evidence attachment, and selected schema-version validation diagnostic call-site evidence are implemented and reviewed. Broader validation attachment, approval attachment, persistence, CLI, and example attachments remain future scoped work.

The current scoped report foundation has advanced through the `WorkReportContract` core model, `WorkReport` core model, in-memory terminal local report generation helper, in-memory runtime result exposure helper, explicit executor-integrated report-bearing execution for local runs, and an explicit local report artifact store. These phases are documented in [docs/implementation-plans/work-report-contract-plan.md](docs/implementation-plans/work-report-contract-plan.md), [docs/implementation-plans/terminal-local-report-generation-plan.md](docs/implementation-plans/terminal-local-report-generation-plan.md), [docs/implementation-plans/runtime-result-report-exposure-plan.md](docs/implementation-plans/runtime-result-report-exposure-plan.md), [docs/implementation-plans/executor-integrated-report-result-plan.md](docs/implementation-plans/executor-integrated-report-result-plan.md), and [docs/implementation-plans/report-artifact-plan.md](docs/implementation-plans/report-artifact-plan.md). Report/audit/missing-citation semantics are hardened in [docs/implementation-plans/report-audit-missing-citation-semantics-plan.md](docs/implementation-plans/report-audit-missing-citation-semantics-plan.md): reports remain derived governed handoff artifacts rather than audit events, report-generation failures remain separate from workflow results, and absent optional references remain explicit section text instead of fabricated missing citations. Explicit high-assurance approval disclosure gating for report artifacts is implemented in [docs/implementation-plans/report-artifact-high-assurance-disclosure-gate-plan.md](docs/implementation-plans/report-artifact-high-assurance-disclosure-gate-plan.md) and accepted in [docs/concepts/REPORT_ARTIFACT_HIGH_ASSURANCE_DISCLOSURE_GATE_REVIEW.md](docs/concepts/REPORT_ARTIFACT_HIGH_ASSURANCE_DISCLOSURE_GATE_REVIEW.md). The explicit artifact-capable executor path now derives workflow-declared high-assurance artifact requirements and composes them with caller-supplied artifact policy by strictness. Automatic runtime report generation for every run, approval/cancellation report-bearing methods, automatic report artifact writing from default executor paths, CLI rendering, schema changes, and examples remain later phases and require separate accepted implementation work.

## Future Capability: Governed Extension Boundary

Workflow OS must eventually support user-specific side-effect surfaces without asking users to fork Rust core or smuggle network/write behavior through local skills. The current design correctly keeps skill handlers from hiding real SaaS, shell, network, or external write behavior, because that would bypass policy, idempotency, audit, and report gates. The long-term answer is a governed extension boundary: an out-of-process or otherwise sandboxed adapter protocol that lets third parties connect systems such as Slack, Gmail, Salesforce, internal bots, customer queues, and deploy tools while still satisfying Workflow OS capability, policy, approval, idempotency, evidence, audit, side-effect, and report contracts.

This is a future capability, not a current v0 claim. Today there is no sanctioned third-party adapter SDK, no production Slack adapter, no generic adapter registration mechanism, no automatic hook authorization check for arbitrary agent tools, no stable event-tail/export API, and no approval API beyond the implemented local/CLI-oriented surfaces. Core-owned adapter work remains focused on proving the safety boundary before generalizing extension.

The governed extension boundary should include:

- an out-of-process adapter protocol or SDK so users can add integrations without forking core;
- a hard hook/adapter authorization check, such as "does this governed run authorize this tool call?";
- a first-class human-executed side-effect pattern, where a system drafts or stages work, the human executes in the native tool, and Workflow OS later reconciles what was sent, edited, discarded, or skipped;
- an external observation and reconciliation API generalized beyond the first GitHub PR comment recovery lane;
- stable machine-readable event export or tailing so downstream audit-and-learn systems can consume workflow history without scraping CLI text;
- machine-readable pending approval surfaces so approval can happen where the human already works while preserving verified actor identity;
- clear separation between instruction scaffolds, deterministic hooks, governed adapters, report artifacts, and downstream learning loops.

Slack/customer-channel sweeps are a useful motivating shape: scan channels, stage drafts, require human send or approval, record what happened, and feed the observed outcome to downstream learning. Evidence-driven investigations are a harder shape because the workflow may branch semantically, but they still need the same hard side-effect boundary when a customer-visible claim or message is about to leave the system. This future lane should preserve the product thesis: Workflow OS governs authority, evidence, side effects, auditability, and handoffs, while agents and external tools execute inside explicit boundaries.

Workflow OS has begun self-governance dogfooding. The current dogfood slice is [dogfood/workflow-os-self-governance](dogfood/workflow-os-self-governance/README.md): local, approval-gated, sequential multi-step workflows that use the kernel as the governing wrapper for Workflow OS work. The conversion is documented in [Self-Governance Dogfood Multi-Step Conversion Plan](docs/implementation-plans/self-governance-dogfood-multi-step-conversion-plan.md). The dogfood suite now includes `dg/d` for planning/docs benchmark work, `dg/implement` for bounded implementation phases, `dg/review` for phase-level maintainer reviews, `dg/pr` for PR hygiene and conflict avoidance, `dg/runtime-composition` for connecting already-built primitives into explicit runtime paths, `dg/blocker` for focused blocker fixes, `dg/release` for release hygiene and public-preview readiness, `dg/branch-cleanup` for merged branch cleanup governance, `dg/workflow-discovery` for recommendation-only workflow discovery, and `dg/spec-field-operationalization` for turning rich scaffold/spec fields into explicit enforcement, validation, disclosure, checks, or deferred posture. These workflows govern scope, context, approvals, implementation/review handoff, validation disclosure, findings classification, conflict-risk disclosure, PR readiness reporting, runtime-composition posture, blocker regression posture, release-readiness disclosure, delete-candidate review, cleanup approval, cleanup reporting, repeated-pattern discovery, overlap/conflict review, recommendation handoff, field posture classification, and scaffold operationalization reporting. This is kernel-governed and Codex-executed. It does not add real build-command skills, git automation, automatic code execution, PR creation, branch deletion, workflow file generation, workflow registration, release publishing, recursive agents, agent swarms, production self-hosting, or Level 3/4 autonomy.

Maintainer tooling reliability is part of dogfood hygiene but remains outside runtime capability. [GitHub PR Connector Boundary Blocker Report](docs/concepts/GITHUB_PR_CONNECTOR_BOUNDARY_BLOCKER_REPORT.md) records the operating rule that GitHub UI, Codex GitHub connector, local git credential, REST API, and Workflow OS provider-auth failures are separate evidence classes, and the boundary is accepted in [GitHub PR Connector Boundary Blocker Review](docs/concepts/GITHUB_PR_CONNECTOR_BOUNDARY_BLOCKER_REVIEW.md). External connector failures must not be attributed to Workflow OS provider auth unless a Workflow OS provider path actually ran.

The `dg/*` workflows are self-governance dogfood workflows for this repository, not community-default workflows. They are useful as reference patterns for kernel-governed work, but a generic user should not be expected to adopt Workflow OS's branch cleanup, release hygiene, PR hygiene, blocker-fix, or roadmap-discovery workflows unchanged. Portable examples belong under `examples/`; user and team workflows belong in their own projects and, later, a governed workflow catalog/store with explicit ownership, lifecycle, authority, evidence, approval, state, and report boundaries.

The required P0 correction is to provide starter scaffolds for user repositories rather than treating dogfood workflows as onboarding assets. The intended separation is:

```text
dogfood/   = how Workflow OS governs building Workflow OS
examples/  = portable learning examples for evaluators
scaffolds/ = starter setup paths for a user's own repo
```

Self-governance should now become a maintained benchmark protocol for building Workflow OS with Workflow OS. Planning is documented in [Self-Governed Build Benchmark Plan](docs/implementation-plans/self-governed-build-benchmark-plan.md) and accepted in [Self-Governed Build Benchmark Plan Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_PLAN_REVIEW.md). The benchmark framing is: Workflow OS governs its own development loop while agents and maintainers execute the work. The benchmark runbook is implemented in [Self-Governed Build Benchmark](docs/user-guide/self-governed-build-benchmark.md), linked from the dogfood project, and accepted in [Self-Governed Build Benchmark Runbook Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_RUNBOOK_REVIEW.md). Focused behavior coverage through existing explicit APIs is implemented in [Self-Governed Build Benchmark Behavior Test Report](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_BEHAVIOR_TEST_REPORT.md) and accepted in [Self-Governed Build Benchmark Behavior Test Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_BEHAVIOR_TEST_REVIEW.md). The repo-local `npm run dogfood:benchmark` helper is implemented as development tooling, documented in [Self-Governed Build Benchmark CLI/Dev-Helper Plan](docs/implementation-plans/self-governed-build-benchmark-cli-dev-helper-plan.md), accepted in [Self-Governed Build Benchmark CLI/Dev-Helper Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_REVIEW.md), hardened in [Self-Governed Build Benchmark CLI/Dev-Helper Hardening Report](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_HARDENING_REPORT.md), and accepted in [Self-Governed Build Benchmark CLI/Dev-Helper Hardening Review](docs/concepts/SELF_GOVERNED_BUILD_BENCHMARK_CLI_DEV_HELPER_HARDENING_REVIEW.md). The helper now includes repo-local governed phase runner commands, `phase-start` and `phase-close`, for material Workflow OS roadmap work. They validate the dogfood project, start the appropriate `dg/*` workflow, display run and approval IDs, emit a structured `approval_handoff` block for agents to relay, require explicit human approval outside the runner, and summarize the governed event trail for phase reports. A P0 approval handoff emission bug is fixed and recorded in [Governed Phase Approval Handoff Context Bug](docs/concepts/GOVERNED_PHASE_APPROVAL_HANDOFF_CONTEXT_BUG.md): the helper now emits a structured approval handoff instruction block. A follow-on P0 preservation bug is fixed in [Governed Phase Approval Handoff Preservation Bug](docs/concepts/GOVERNED_PHASE_APPROVAL_HANDOFF_PRESERVATION_BUG.md): agents must preserve and present the complete emitted block in the user-facing approval request instead of collapsing it into vague prose. The P0 approval work-summary bug is fixed in [Governed Phase Approval Work Summary Bug](docs/concepts/GOVERNED_PHASE_APPROVAL_WORK_SUMMARY_BUG.md), following [Governed Phase Approval Work Summary Plan](docs/implementation-plans/governed-phase-approval-work-summary-plan.md): approval handoffs now include bounded work summary, approved scope, strict non-goals, likely touched surfaces, validation expectations, and why-now context, and live material phase starts fail closed when that context is missing. The repeated final-response preservation bug is fixed in [Governed Phase Approval Final Request Preservation Bug](docs/concepts/GOVERNED_PHASE_APPROVAL_FINAL_REQUEST_PRESERVATION_BUG.md): `phase-start` now emits a copy-safe final approval request that agents must use when the turn ends waiting for approval. Dogfood approval-presentation enforcement is implemented for material phase approvals, and phase-close proof disclosure is implemented in [Dogfood Phase-Close Proof-Enforcement Disclosure Report](docs/concepts/DOGFOOD_PHASE_CLOSE_PROOF_ENFORCEMENT_DISCLOSURE_REPORT.md). `phase-close` now reports whether matching approval-presentation proof records are present for the governed run, including bounded presentation IDs and content hashes. Approval-event proof marker planning is documented in [Approval Event Proof Marker Plan](docs/implementation-plans/approval-event-proof-marker-plan.md), accepted in [Approval Event Proof Marker Plan Review](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_PLAN_REVIEW.md), implemented as model-only vocabulary in [Approval Event Proof Marker Model Report](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_MODEL_REPORT.md), accepted with non-blocking follow-ups in [Approval Event Proof Marker Model Review](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_MODEL_REVIEW.md), wired into the opt-in approval-presentation decision path in [Approval Event Proof Marker Runtime Wiring Report](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_RUNTIME_WIRING_REPORT.md), accepted in [Approval Event Proof Marker Runtime Wiring Review](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_RUNTIME_WIRING_REVIEW.md), and exposed through bounded inspect/projection output in [Approval Event Proof Marker Inspect Projection Report](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_INSPECT_PROJECTION_REPORT.md). Proof-enforced approval decision events can now record which presentation proof they used and `phase-close` can report `proof_enforced` when inspect output exposes the marker, without changing default approval behavior. All material Workflow OS implementation, review, blocker, PR hygiene, release, and workflow-discovery phases should begin with a governed dogfood run unless explicitly exempted. The helper recommends using governed run identity, approvals, validation/check references, hooks, typed handoffs, WorkReports, and report artifacts as those primitives are implemented and reviewed. It does not authorize automatic kernel control of agents, hidden approvals, automatic local check execution, arbitrary shell execution, workflow schema changes, repository writes from inside the kernel, git or PR actions, recursive agents, agent swarms, hosted execution, production self-hosting, write-capable adapters, or Level 3/4 autonomy claims.


## Future Capability: Workflow Discovery And Catalog Governance

Workflow OS should eventually govern not only workflow execution, but also the workflow catalog itself. As teams adopt governed workflows, the kernel should help organizations discover repeated work patterns, recommend candidate workflows, recommend changes to existing workflows, prevent conflicting workflows from accumulating, and manage workflow lifecycle, ownership, authority, and handoffs from a central governance point.

This is a future collaboration capability, not a current local-kernel runtime claim. Today, workflows are local files validated and run by the local kernel. That manual workflow-authoring posture is acceptable for the local preview, but it is not the long-term product shape. As agents take more autonomous actions, humans should increasingly monitor, approve, reject, or amend proposed workflow changes rather than manually creating every workflow from scratch. Manual workflow creation is a collaboration-scale adoption blocker if it remains the primary path.

Over time, Workflow OS should grow toward a collaboration model where workflows can be recommended, proposed, reviewed, promoted, deprecated, superseded, and composed without becoming a pile of uncoordinated YAML files.

The strategic thesis is:

```text
Agent or human work reveals patterns. Workflow OS governs which patterns become workflows.
```

The kernel should eventually be able to recommend:

- create a workflow because the same ad hoc task recurs across users, teams, projects, or companies;
- change an existing workflow because observed execution shows a missing gate, missing evidence requirement, missing handoff, unclear failure behavior, or repeated manual workaround;
- split a workflow because it mixes planning, execution, approval, and reporting too broadly;
- merge or relate workflows because two definitions duplicate the same governed work;
- retire a workflow because it is stale, unused, superseded, or unsafe;
- add a policy gate because a workflow crosses an authority, data, side-effect, or risk boundary;
- add approval requirements because a workflow controls sensitive resources or high-impact decisions;
- add evidence requirements because downstream decisions rely on unverified claims;
- add typed handoffs because natural-language summaries are causing context drift;
- add final report requirements because work is completed without auditable closure;
- flag conflict because multiple workflows claim overlapping ownership, authority, resources, side effects, state transitions, or approval responsibilities.

The human role should shift from manual workflow authoring to governed catalog stewardship. Humans should review recommendations, inspect evidence and rationale, approve or reject proposed workflow additions and changes, resolve conflicts, assign ownership, and decide when workflow changes are promoted from draft to active. The kernel should not assume humans have enough time to hand-create every workflow needed by an increasingly autonomous agent environment.

Conflict prevention is central. In a collaboration setting, two workflows should not silently govern the same resource boundary in incompatible ways. A future catalog should reason about:

- workflow ID and ownership;
- purpose and lifecycle status;
- allowed inputs and required context;
- required outputs and handoff contracts;
- required evidence and report sections;
- policy gates and approval rules;
- authority scope and delegated capabilities;
- side-effect declarations and denied/unsupported actions;
- resources touched or reserved;
- event and audit requirements;
- terminal status and failure semantics;
- dependencies on other workflows;
- version compatibility and deprecation posture.

The first local step is implemented as dogfood-oriented and recommendation-only: a `dg/workflow-discovery` workflow that reviews recent Workflow OS build work, identifies repeated patterns, proposes new dogfood workflows, flags overlap or conflict among existing dogfood workflows, recommends workflow splits/merges/retirements, and produces a bounded workflow discovery report. It does not generate workflow files automatically, register workflows automatically, mutate specs, modify roadmap state, or approve its own recommendations.

Longer-term implementation should be staged:

1. Local dogfood workflow discovery runbook: manual/recommendation-only.
2. Workflow catalog planning: model workflow ownership, lifecycle, purpose, authority, dependencies, and review status.
3. Workflow conflict taxonomy: define what counts as resource, authority, policy, side-effect, handoff, report, and lifecycle conflict.
4. Catalog validation model: detect duplicate IDs, overlapping authority, incompatible side-effect posture, missing owners, stale lifecycle states, and unsafe dependency cycles.
5. Recommendation report model: produce bounded, redaction-safe recommendations without auto-applying changes.
6. Draft workflow proposal model: represent proposed workflow additions and changes as reviewable drafts, not active workflows.
7. Promotion/review workflow: require human approval before recommendations become workflow definitions.
8. Collaboration registry: centralize workflow discovery and lifecycle only after local contracts are stable.
9. Organization-scale discovery: use bounded signals from runs, audit records, reports, hooks, typed handoffs, approvals, and side-effect records to recommend workflow changes.

Relationship to existing concepts:

- Workflow OS remains the governed work runtime.
- A workflow is an authored unit of governed work.
- The future workflow catalog is the governed registry of authored workflows and their lifecycle.
- EvidenceReference and WorkReport provide the citation and closure substrate for workflow recommendations.
- Typed handoffs should prevent natural-language-only context drift between workflows.
- Policy gates and approvals should define authority boundaries for workflow adoption and execution.
- SideEffect boundaries should prevent workflow overlap from turning into conflicting writes.
- Composable Harness Contracts remain a later execution-topology capability; workflow catalog governance should come first.
- Reasoning Lineage / Claim Graph remains later provenance work that may eventually explain why recommendations were made.

Non-goals:

- No automatic workflow generation in v0.
- No automatic workflow registration, promotion, or deletion.
- No claim that manual workflow creation is the desired long-term adoption path.
- No automatic mutation of roadmap, specs, examples, or runtime state.
- No hosted organization registry in the current local kernel.
- No claim that Workflow OS currently resolves cross-team workflow conflicts.
- No replacement of deterministic validation and human approval with model opinion.
- No recursive agents, agent swarms, or autonomous organization design.
- No write-capable adapters or provider mutations as part of catalog discovery.
- No Level 3/4 autonomy claim.

## Future Capability: Durable Workflow State And Catalog Store

Workflow OS should not treat git as the long-term database for governed execution or workflow collaboration.

The boundary should be:

```text
Git stores authored contracts.
Workflow OS stores governed execution state.
```

Git remains valuable for versioning and reviewing workflow definitions, policies, examples, and dogfood specs. It should continue to be part of the authored contract lifecycle. But runtime state and collaboration state need an explicit Workflow OS store because they change during execution, may be concurrent, and must be queryable without rewriting source-controlled specs.

The store boundary should eventually cover:

- workflow definitions and spec hashes as immutable run inputs;
- workflow catalog metadata: owner, lifecycle, purpose, authority, dependencies, review status, promotion status, deprecation status;
- run state: current state, terminal status, retries, approvals, checkpoints, generated reports;
- event log: append-only workflow events and audit projections;
- approval records: requested, granted, denied, expired, revoked, linked evidence, linked side effects;
- evidence ledger: references, summaries, sensitivity, redaction metadata, and source preservation without raw payload copying by default;
- work reports and report artifacts: generated handoff records, integrity links, citations, and disclosures;
- hook records: checkpoint status, failed-closed results, skipped/warning disclosure, audit/event projections;
- local check records: stable check references, command identity, bounded output posture, and redaction metadata;
- side-effect records: proposed, approved, denied, attempted, completed, failed, skipped, unsupported;
- workflow recommendations: discovered patterns, proposed changes, conflicts, accepted/rejected/deferred decisions;
- collaboration state: review comments, stewardship decisions, ownership changes, and catalog promotion decisions.

This should be staged conservatively:

1. Clarify authored-contract versus runtime-state boundaries in docs.
2. Define a local durable store contract for run/event/report/evidence/approval state.
3. Keep workflow definitions in files/git while storing execution state outside the authored specs.
4. Add a local embedded store option for serious single-user dogfooding and repeatable local runs.
5. Add migration and integrity rules for local state.
6. Add catalog metadata models only after workflow discovery and conflict taxonomy are stable.
7. Add a team backend interface for collaborative workflow stewardship.
8. Add a production backend only after local contracts, event semantics, and privacy boundaries are reviewed.

Store and catalog backend work should also include conformance suites before third-party or alternate backends are treated as compatible. The suite should prove idempotency, isolation, append-only event behavior, lookup semantics, report artifact integrity, side-effect record discovery, and catalog lifecycle behavior. Future catalog objects should keep human-readable workflow identity paired with immutable content hashes. Federation and signed provenance remain future topology and trust capabilities, not v0 requirements.

Non-goals:

- No database-backed runtime is implemented by this roadmap update.
- No hosted service claim.
- No workflow catalog backend claim.
- No replacement of git for authored workflow definitions.
- No automatic migration of existing local state.
- No collaboration backend, sync service, or access-control system in v0.
- No write-capable adapter behavior as part of the store transition.
- No required RocksDB, central registry, federation, cryptographic signing, or MCP server as part of this store roadmap note.

Self-governed validation/check planning is documented in [Self-Governed Validation/Check Plan](docs/implementation-plans/self-governed-validation-check-plan.md). A local validation/check command contract model is implemented with canonical command-template binding, and the first explicit test-only handler for `WorkflowOsValidateDogfood` is implemented and documented in [Test-Only Local Check Handler Plan](docs/implementation-plans/test-only-local-check-handler-plan.md). Broader local check handler planning is documented in [Broader Local Check Handler Plan](docs/implementation-plans/broader-local-check-handler-plan.md), and the first infrastructure slice adds a structured local check result model plus injectable process-runner boundary. The first non-dogfood explicit handler, `DocsCheck`, has advanced to a production-shaped explicit `DocsCheckLocalHandler` while remaining non-default/non-CLI; it is documented in [DocsCheck Local Handler Plan](docs/implementation-plans/docs-check-local-handler-plan.md), [DocsCheck Local Handler Production-Posture Plan](docs/implementation-plans/docs-check-production-posture-plan.md), and [DocsCheck Default-Registration Plan](docs/implementation-plans/docs-check-default-registration-plan.md). An explicit non-default registry helper is implemented for callers that supply a prebuilt `DocsCheckLocalHandler`. [Local Check Handler Default-Registration Plan](docs/implementation-plans/local-check-handler-default-registration-plan.md) implements an explicit non-default registration profile/helper before any ambient default registration. The local-check dogfood lane in [Dogfood Real DocsCheck Plan](docs/implementation-plans/dogfood-real-docs-check-plan.md) is implemented: the self-governance workflow now has an explicit docs-check checkpoint that can run only when a caller supplies `DocsCheckLocalHandler` through explicit profile registration, with injected-runner tests proving the boundary. Local check side-effect/cache/write boundary planning and the model-only boundary are documented in [Local Check Side-Effect Boundary Plan](docs/implementation-plans/local-check-side-effect-boundary-plan.md), and the ignored opt-in live DocsCheck smoke is implemented as documented in [Opt-In Live DocsCheck Smoke Plan](docs/implementation-plans/opt-in-live-docscheck-smoke-plan.md). Local check result citation planning is documented in [Local Check Result Citation Plan](docs/implementation-plans/local-check-result-citation-plan.md), and the first local check result reference model is implemented. WorkReport local check citation target planning is documented in [WorkReport Local Check Result Citation Target Plan](docs/implementation-plans/work-report-local-check-citation-target-plan.md), and WorkReport citation vocabulary for local check results is implemented. Terminal report helper integration for supplied local check result references is implemented and documented in [Terminal Report Local Check Citation Integration Plan](docs/implementation-plans/terminal-report-local-check-citation-integration-plan.md). Command-output evidence policy planning is documented in [Command Output Evidence Policy Plan](docs/implementation-plans/command-output-evidence-policy-plan.md), with command-output evidence attachment explicitly deferred. Evidence attachment, command-output evidence implementation, true default registration, arbitrary shell execution, CLI exposure, automatic check execution, non-ignored live local check execution, live side-effect enforcement, and writes remain future scoped work.

Side-effect boundary modeling must be accepted before policy-gated writes, generic runtime adapter execution, or domain packs. [ADR 0011: Side-Effect Boundary Core Model](docs/adr/0011-side-effect-boundary.md) is accepted as the domain-neutral architecture boundary for side-effect intent, authority, lifecycle state, idempotency, audit, evidence, and report citation. The SideEffect core model is implemented as model-only Rust types and accepted in [SideEffect Core Model Review](docs/concepts/SIDE_EFFECT_CORE_MODEL_REVIEW.md). WorkReport side-effect citation vocabulary is implemented as model-only vocabulary and accepted in [WorkReport SideEffect Citation Review](docs/concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md). Terminal report SideEffect citation propagation is implemented for explicit helper inputs and accepted in [Terminal Report SideEffect Citation Integration Review](docs/concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md). Executor SideEffect report input propagation is implemented in [Executor SideEffect Report Input Propagation Report](docs/concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REPORT.md) and accepted in [Executor SideEffect Report Input Propagation Review](docs/concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md). SideEffect workflow event and audit projection planning is documented in [SideEffect Workflow Event And Audit Projection Plan](docs/implementation-plans/side-effect-workflow-event-audit-projection-plan.md), and model-only SideEffect workflow event vocabulary plus bounded generic audit projection are implemented in [SideEffect Workflow Event Model Report](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md) and accepted in [SideEffect Workflow Event Model Review](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REVIEW.md). Executor SideEffect event append planning is documented in [Executor SideEffect Event Append Plan](docs/implementation-plans/executor-side-effect-event-append-plan.md), and the first explicit local proposed/denied/skipped append path is implemented in [Executor SideEffect Event Append Report](docs/concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md). SideEffect persistence and discovery planning is documented in [SideEffect Persistence And Discovery Plan](docs/implementation-plans/side-effect-persistence-discovery-plan.md), the first explicit local `SideEffectRecordStore` persistence slice is implemented in [SideEffect Record Store Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md), and the immutable run identity blocker found in [SideEffect Record Store Review](docs/concepts/SIDE_EFFECT_RECORD_STORE_REVIEW.md) is fixed in [SideEffect Record Store Blocker Fix Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md) and accepted in [SideEffect Record Store Blocker Fix Review](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REVIEW.md). Concrete SideEffect discovery planning is documented in [SideEffect Discovery Plan](docs/implementation-plans/side-effect-discovery-plan.md), the first explicit in-memory discovery helper is implemented in [SideEffect Discovery Helper Report](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md) and accepted in [SideEffect Discovery Helper Review](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md), and store-backed discovery is implemented in [SideEffect Store-Backed Discovery Report](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) following [SideEffect Store-Backed Discovery Plan](docs/implementation-plans/side-effect-store-backed-discovery-plan.md) and accepted in [SideEffect Store-Backed Discovery Review](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md). WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](docs/implementation-plans/work-report-side-effect-discovery-integration-plan.md), and the explicit WorkReport-side discovery helper is implemented in [WorkReport SideEffect Discovery Integration Report](docs/concepts/WORK_REPORT_SIDE_EFFECT_DISCOVERY_INTEGRATION_REPORT.md). Executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), following [Executor SideEffect Discovery Opt-In Plan](docs/implementation-plans/executor-side-effect-discovery-opt-in-plan.md), and accepted with non-blocking follow-ups in [Executor SideEffect Discovery Opt-In Review](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md). Report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](docs/implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). Approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](docs/implementation-plans/approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md) and accepted with non-blocking follow-ups in [SideEffect Approval Linkage Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). Approval-side-effect linkage composition planning is documented in [Approval SideEffect Linkage Composition Plan](docs/implementation-plans/approval-side-effect-linkage-composition-plan.md). The explicit store-backed approval linkage helper is accepted in [SideEffect Approval Linkage Store-Backed Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REVIEW.md), and explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md) and accepted in [Executor Report Artifact SideEffect Gates Review](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REVIEW.md). Provider-write completed/failed workflow event append is implemented for eligible reconciled GitHub PR comment provider outcomes in [GitHub PR Comment Provider Write Event Append Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REPORT.md) and accepted in [GitHub PR Comment Provider Write Event Append Helper Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REVIEW.md). Reconciliation-aware report/artifact disclosure is planned in [GitHub PR Comment Provider Reconciliation Report Artifact Disclosure Plan](docs/implementation-plans/github-pr-comment-provider-reconciliation-report-artifact-disclosure-plan.md), accepted in [GitHub PR Comment Provider Reconciliation Report Artifact Disclosure Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_RECONCILIATION_REPORT_ARTIFACT_DISCLOSURE_PLAN_REVIEW.md), and the first bounded projection helper is implemented in [GitHub PR Comment Provider Reconciliation Disclosure Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_RECONCILIATION_DISCLOSURE_HELPER_REPORT.md). Provider lookup/query reconciliation is implemented in [GitHub PR Comment Provider Lookup Reconciliation Model Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REPORT.md) and accepted in [GitHub PR Comment Provider Lookup Reconciliation Model Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REVIEW.md). Concrete injected lookup HTTP client planning is documented in [GitHub PR Comment Provider Lookup HTTP Client Plan](docs/implementation-plans/github-pr-comment-provider-lookup-http-client-plan.md), the first explicit injected-transport lookup HTTP client is implemented in [GitHub PR Comment Provider Lookup HTTP Client Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_HTTP_CLIENT_REPORT.md), and the first explicit in-memory lookup recovery integration helper is implemented in [GitHub PR Comment Provider Lookup Recovery Integration Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECOVERY_INTEGRATION_HELPER_REPORT.md). This does not implement default writes, hidden auth loading, automatic retries, broad provider mutations, automatic provider lookup, schemas, CLI behavior, examples, hosted behavior, runtime side-effect execution beyond explicit reviewed helper paths, EvidenceReference side-effect attachment, automatic executor report discovery, automatic artifact writes from existing executor paths, automatic approval-side-effect validation in existing report/artifact paths, or release posture changes.

## High-Assurance Approval Controls

High-assurance approval controls are a future governance capability, not a current production claim. User feedback has highlighted "nuclear key" style approval workflows as an important mental model: sensitive actions should be impossible unless the required authority, evidence, policy gates, approvals, audit trail, and report disclosures are all present.

Workflow OS already has several prerequisites in place or underway:

- event-sourced approval requests and decisions;
- policy gates before meaningful runtime actions;
- approval expiration metadata;
- denial reasons and fail-closed denial behavior;
- audit and observability records;
- EvidenceReference foundations;
- report and report-artifact foundations;
- sequential governed multi-step execution.

The future roadmap capability should be framed as **high-assurance multi-party approval controls**, not as safety-critical certification. Candidate features include:

- multi-party approval or quorum rules;
- separation of requester and approver;
- role-bound approval authority;
- prevention of self-approval for sensitive actions;
- approval expiry, revocation, and escalation semantics;
- evidence-required approval contexts;
- policy-tested approval chains;
- immutable approval audit trails;
- final work-report disclosure of approvals requested, granted, denied, expired, skipped, or deferred.

### Approval Gate Context And UX

Approval gates must become human-reviewable governed change cards, not raw machine handoff dumps. This is a general kernel requirement, not a dogfood-only convenience. Any workflow author should be able to define approval rules and have Workflow OS present the approver with enough bounded context to understand what is being authorized before the executor continues.

A future approval gate should distinguish:

- requested action: what the executor is asking permission to do;
- planned work: bounded executor-supplied intent, such as planned changes, expected outputs, and known risks;
- expected touched surfaces: files, workflows, policies, adapters, providers, systems, or stores that may be affected;
- allowed scope: what the approval unlocks;
- disallowed scope: what remains forbidden even after approval;
- required evidence: validation, checks, policy decisions, side-effect disclosures, citations, and final WorkReport obligations expected before closure;
- post-approval behavior: what Workflow OS records, permits, or expects next;
- technical IDs: run ID, approval ID, policy ID, step ID, workflow ID, and other stable audit references.

The executor may propose bounded intent context, but the kernel must validate and render it safely. Approval context must reject or redact secret-like values, raw provider payloads, raw command output, raw spec contents, local secrets, tokens, private keys, and unbounded natural-language dumps. Missing context should be explicit as `not supplied` or `not available`; the kernel must not fabricate evidence, touched surfaces, planned work, or approval authority.

The user-facing presentation should lead with human meaning and collapse technical details behind the approval card. The machine-readable approval handoff remains necessary for audit, replay, and agent integration, but it should not be the primary experience for ordinary approval review.

P0 hardening gap: [Approval Gate Presentation Enforcement Gap](docs/concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md) tracks the remaining proof problem. Planning is documented in [Approval Gate Presentation Enforcement Plan](docs/implementation-plans/approval-gate-presentation-enforcement-plan.md), the first model/helper slice is implemented in [Approval Gate Presentation Core Model Report](docs/concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REPORT.md), and the model review is documented in [Approval Gate Presentation Core Model Review](docs/concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REVIEW.md). Follow-on persistence and explicit opt-in enforcement planning is documented in [Approval Gate Presentation Persistence And Enforcement Plan](docs/implementation-plans/approval-gate-presentation-persistence-enforcement-plan.md), the local persistence helper is implemented in [Approval Gate Presentation Persistence Report](docs/concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REPORT.md), and the helper review is documented in [Approval Gate Presentation Persistence Review](docs/concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md). The explicit opt-in enforcement path is implemented in [Approval Gate Presentation Opt-In Enforcement Plan](docs/implementation-plans/approval-gate-presentation-opt-in-enforcement-plan.md), reported in [Approval Gate Presentation Opt-In Enforcement Implementation Report](docs/concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_IMPLEMENTATION_REPORT.md), and accepted in [Approval Gate Presentation Opt-In Enforcement Review](docs/concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_REVIEW.md): explicit executor callers can now require matching durable presentation proof, optional freshness checks, and fail-closed proof validation before approval events are appended. Dogfood runner proof persistence planning is documented in [Dogfood Runner Approval-Presentation Persistence Plan](docs/implementation-plans/dogfood-runner-approval-presentation-persistence-plan.md), and the repo-local runner now persists bounded `ApprovalPresentationRecord` proof during material `phase-start` before approval. Default/public enforcement planning is documented in [Approval Gate Presentation Default Enforcement Plan](docs/implementation-plans/approval-gate-presentation-default-enforcement-plan.md), accepted in [Approval Gate Presentation Default Enforcement Plan Review](docs/concepts/APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_PLAN_REVIEW.md), implemented as an explicit policy model/helper in [Approval Gate Presentation Default Enforcement Implementation Report](docs/concepts/APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_IMPLEMENTATION_REPORT.md), accepted in [Approval Gate Presentation Default Enforcement Implementation Review](docs/concepts/APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_IMPLEMENTATION_REVIEW.md), selected high-assurance/write-adjacent adoption is planned in [Approval-Presentation Sensitive Adoption Plan](docs/implementation-plans/approval-presentation-sensitive-adoption-plan.md), and the first selected high-assurance adoption path is implemented in [High-Assurance Approval-Presentation Adoption Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_PRESENTATION_ADOPTION_REPORT.md) and accepted in [High-Assurance Approval-Presentation Adoption Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_PRESENTATION_ADOPTION_REVIEW.md). Provider-write/write-adjacent approval-presentation adoption is planned in [Provider-Write Approval-Presentation Adoption Plan](docs/implementation-plans/provider-write-approval-presentation-adoption-plan.md), and the first explicit GitHub PR comment provider-write proof gate is implemented in [Provider-Write Approval-Presentation Gate Implementation Report](docs/concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_IMPLEMENTATION_REPORT.md), accepted in [Provider-Write Approval-Presentation Gate Review](docs/concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_REVIEW.md), and edge-hardened in [Provider Write Approval Presentation Edge Hardening Review](docs/concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_EDGE_HARDENING_REVIEW.md). The kernel can emit the correct approval details, and core can now model, locally persist, review, opt-in enforce, dogfood-persist, explicitly policy-route bounded presentation proof, require proof on selected high-assurance approval decisions, and require write-adjacent presentation proof on the selected explicit provider-write path with a deterministic content hash. Default public approval behavior, UI/cards, write-capable adapter defaults, hidden auth loading, CLI mutation behavior, schemas, examples, hosted behavior, and release posture changes remain unimplemented. Repo-local dogfood approval-presentation proof persistence, proof-enforced approval, and bounded freshness enforcement are implemented for material dogfood phases.

P0 dogfood follow-up: phase-close proof disclosure currently fails to reread
the accumulated local presentation store once its bounded listing reaches 250
records, even though the proof-enforced approval command succeeds. Phase close
must resolve the exact run and approval proof without an unbounded global list
before this disclosure is scale-safe.

This belongs before any serious write-capable adapter work. Write-capable operations should not be introduced until high-risk approvals can be modeled with scoped authority, evidence requirements, durable audit, and deterministic fail-closed behavior. Approval decision proof markers are now wired into the opt-in approval-presentation decision path and accepted in [Approval Event Proof Marker Runtime Wiring Review](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_RUNTIME_WIRING_REVIEW.md). Bounded inspect/projection exposure for those markers is implemented and accepted in [Approval Event Proof Marker Inspect Projection Review](docs/concepts/APPROVAL_EVENT_PROOF_MARKER_INSPECT_PROJECTION_REVIEW.md). WorkReport and audit citation behavior for proof markers is planned in [Approval Proof Marker WorkReport And Audit Citation Plan](docs/implementation-plans/approval-proof-marker-workreport-audit-citation-plan.md), a pure in-memory citation derivation helper is implemented in [Approval Proof Marker Citation Helper Report](docs/concepts/APPROVAL_PROOF_MARKER_CITATION_HELPER_REPORT.md) and accepted in [Approval Proof Marker Citation Helper Review](docs/concepts/APPROVAL_PROOF_MARKER_CITATION_HELPER_REVIEW.md), terminal report opt-in integration is implemented in [Terminal Report Approval Proof Marker Citation Integration Report](docs/concepts/TERMINAL_REPORT_APPROVAL_PROOF_MARKER_CITATION_INTEGRATION_REPORT.md), executor report input propagation is implemented in [Executor Proof Marker Citation Report Input Propagation Report](docs/concepts/EXECUTOR_PROOF_MARKER_CITATION_REPORT_INPUT_PROPAGATION_REPORT.md) and accepted in [Executor Proof Marker Citation Report Input Propagation Review](docs/concepts/EXECUTOR_PROOF_MARKER_CITATION_REPORT_INPUT_PROPAGATION_REVIEW.md), audit projection persistence is implemented as an explicit helper in [Approval Proof Marker Durable Audit Projection Persistence Helper Report](docs/concepts/APPROVAL_PROOF_MARKER_DURABLE_AUDIT_PROJECTION_PERSISTENCE_HELPER_REPORT.md), the first pure in-memory report artifact proof-marker gate helper is implemented in [Report Artifact Approval Proof Marker Gate Helper Report](docs/concepts/REPORT_ARTIFACT_APPROVAL_PROOF_MARKER_GATE_HELPER_REPORT.md), the helper-level artifact-write composition with the store-backed proof-marker gate is implemented in [Report Artifact Proof-Marker Write Composition Helper Report](docs/concepts/REPORT_ARTIFACT_PROOF_MARKER_WRITE_COMPOSITION_HELPER_REPORT.md), and the proof-enforced approval-resume artifact/projection composition helper is implemented in [Approval-Resume Artifact Projection Composition Report](docs/concepts/APPROVAL_RESUME_ARTIFACT_PROJECTION_COMPOSITION_REPORT.md). Executor default artifact composition remains future scoped work.

Non-goals:

- No claim that Workflow OS supports nuclear-grade, medical, aviation, defense, or other safety-critical certification.
- No claim that v0 approvals implement multi-party approval, quorum approval, role-based authority, external identity provider integration, or approval revocation.
- No claim that v0 approval gates provide rich governed change-card UX, executor intent validation, or generalized approval-context rendering.
- No claim that default v0 approval gates durably prove the exact approval presentation shown to the human before approval.
- No replacement of deterministic policy and audit with model self-review.
- No write-capable adapter authorization as part of this roadmap note.
- No Level 3/4 autonomy claim.

## Composable Harness Contracts

Composable Harness Contracts are a future governed-work capability, not a v1 requirement. Planning is documented in [Composable Harness Contract Plan](docs/implementation-plans/composable-harness-contract-plan.md), and the core model is implemented. Typed handoff planning is documented in [Typed Handoff Plan](docs/implementation-plans/typed-handoff-plan.md), and the typed handoff core model is implemented. No harness contract or typed handoff runtime behavior is implemented.

Workflow OS should not become agents managing agents. The strategic direction is for Workflow OS to become the governed substrate that makes nested harness work safe, durable, auditable, composable, and useful.

A harness is a bounded, governed execution envelope inside a workflow. It is not synonymous with an agent: a harness may contain an agent, deterministic code, tools, policy checks, validation, or human approval. A future harness contract should define the harness name or ID, purpose, allowed inputs, required context, allowed tools, allowed side effects, output schema, evidence requirements, approval policy, timeout/budget/retry policy, failure semantics, and handoff requirements.

This belongs after the local deterministic kernel and basic governed workflow execution are stable. Nested harness execution depends on earlier primitives:

- workflow and run identity;
- durable state or event log;
- EvidenceReference and evidence-ledger behavior;
- policy gates;
- approval model;
- typed handoffs;
- scoped authority;
- validation;
- terminal work reports.

Roadmap placement:

- Local deterministic kernel: foundational.
- Governed single-run workflows: foundational.
- Core governance primitives: evidence, approval, policy gates, audit records, and work reports.
- Composable Harness Contracts: future contract model for bounded harnesses.
- Nested harness execution patterns: future execution topology after contracts are reviewed.
- Reasoning Lineage / Claim Graph: later provenance layer after evidence, reports, and harness boundaries are understood.

Initial illustrative future pattern: an AI-assisted software engineering workflow could be decomposed into a spec harness, planning harness, implementation harness, test/verification harness, review harness, security/risk harness, and final work report harness. This is illustrative only; it is not an immediate implementation promise and should not imply production nested execution support.

Non-goals:

- No arbitrary recursive agent spawning.
- No agent swarm positioning.
- No claim that Workflow OS currently supports production nested execution.
- No live write integrations as part of this roadmap direction.
- No hosted or distributed runtime claim.
- No Level 3/4 autonomy claim.
- No replacement of deterministic governance with model self-review.

Current planning decisions:

- governed multi-step workflow execution ADR and implementation planning
- remaining EvidenceReference attachment boundaries, including approval evidence and broader validation evidence
- explicit executor/helper artifact-writing planning
- report/audit/missing-citation semantics review
- explicit DocsCheck registry helper before any default production check handler registration
- whether generated report exposure should return report-generation errors separately from workflow results
- how much report structure the runtime should enforce
- how side-effect boundaries should be represented before write-capable adapters
- how future Reasoning Lineage or Claim Graph concepts should relate to governed work

Parallel planning sprint outputs are documented in [Parallel Planning Sprint Report](docs/concepts/PARALLEL_PLANNING_SPRINT_REPORT.md). Typed handoff planning is documented in [Typed Handoff Plan](docs/implementation-plans/typed-handoff-plan.md), and the core model is implemented and reviewed. WorkReport typed handoff citation planning is documented in [WorkReport Typed Handoff Citation Plan](docs/implementation-plans/work-report-typed-handoff-citation-plan.md), and WorkReport typed handoff citation target vocabulary is implemented and reviewed. Terminal report helper typed handoff citation integration is implemented and documented in [Terminal Report Typed Handoff Citation Integration Plan](docs/implementation-plans/terminal-report-typed-handoff-citation-integration-plan.md). Executor-integrated typed handoff report input propagation is implemented in [Executor Typed Handoff Report Input Propagation Plan](docs/implementation-plans/executor-typed-handoff-report-input-plan.md). Report/audit/missing-citation semantics hardening is implemented in [Report, Audit, And Missing-Citation Semantics Plan](docs/implementation-plans/report-audit-missing-citation-semantics-plan.md). Side-effect boundary ADR planning is documented in [Side-Effect Boundary ADR Plan](docs/implementation-plans/side-effect-boundary-adr-plan.md), [ADR 0011: Side-Effect Boundary Core Model](docs/adr/0011-side-effect-boundary.md) is accepted, the SideEffect core model is accepted in [SideEffect Core Model Review](docs/concepts/SIDE_EFFECT_CORE_MODEL_REVIEW.md), WorkReport side-effect citation vocabulary is accepted in [WorkReport SideEffect Citation Review](docs/concepts/WORK_REPORT_SIDE_EFFECT_CITATION_REVIEW.md), terminal report helper SideEffect citation propagation is accepted in [Terminal Report SideEffect Citation Integration Review](docs/concepts/TERMINAL_REPORT_SIDE_EFFECT_CITATION_INTEGRATION_REVIEW.md), executor SideEffect report input propagation is accepted in [Executor SideEffect Report Input Propagation Review](docs/concepts/EXECUTOR_SIDE_EFFECT_REPORT_INPUT_PROPAGATION_REVIEW.md), side-effect workflow event/audit projection planning is documented in [SideEffect Workflow Event And Audit Projection Plan](docs/implementation-plans/side-effect-workflow-event-audit-projection-plan.md), the model-only SideEffect workflow event vocabulary plus bounded generic audit projection are implemented in [SideEffect Workflow Event Model Report](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REPORT.md), SideEffect workflow event/audit projection review is accepted in [SideEffect Workflow Event Model Review](docs/concepts/SIDE_EFFECT_WORKFLOW_EVENT_MODEL_REVIEW.md), the first explicit local proposed/denied/skipped SideEffect event append path is implemented in [Executor SideEffect Event Append Report](docs/concepts/EXECUTOR_SIDE_EFFECT_EVENT_APPEND_REPORT.md), SideEffect persistence/discovery planning is documented in [SideEffect Persistence And Discovery Plan](docs/implementation-plans/side-effect-persistence-discovery-plan.md), the first explicit local SideEffect record persistence slice is implemented in [SideEffect Record Store Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_REPORT.md), the immutable run identity blocker is fixed in [SideEffect Record Store Blocker Fix Report](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REPORT.md) and accepted in [SideEffect Record Store Blocker Fix Review](docs/concepts/SIDE_EFFECT_RECORD_STORE_BLOCKER_FIX_REVIEW.md), concrete discovery planning is documented in [SideEffect Discovery Plan](docs/implementation-plans/side-effect-discovery-plan.md), the first explicit in-memory discovery helper is implemented in [SideEffect Discovery Helper Report](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REPORT.md) and accepted in [SideEffect Discovery Helper Review](docs/concepts/SIDE_EFFECT_DISCOVERY_HELPER_REVIEW.md), store-backed discovery is implemented in [SideEffect Store-Backed Discovery Report](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REPORT.md) and accepted in [SideEffect Store-Backed Discovery Review](docs/concepts/SIDE_EFFECT_STORE_BACKED_DISCOVERY_REVIEW.md), WorkReport SideEffect discovery integration planning is documented in [WorkReport SideEffect Discovery Integration Plan](docs/implementation-plans/work-report-side-effect-discovery-integration-plan.md), executor SideEffect discovery opt-in is implemented in [Executor SideEffect Discovery Opt-In Report](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REPORT.md), executor SideEffect discovery opt-in helper review is accepted in [Executor SideEffect Discovery Opt-In Review](docs/concepts/EXECUTOR_SIDE_EFFECT_DISCOVERY_OPT_IN_REVIEW.md), report artifact SideEffect referential integrity validation is implemented as an explicit helper in [Report Artifact SideEffect Referential Integrity Report](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](docs/implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md), approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](docs/implementation-plans/approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md) and accepted with non-blocking follow-ups in [SideEffect Approval Linkage Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). Approval-side-effect linkage composition planning is documented in [Approval SideEffect Linkage Composition Plan](docs/implementation-plans/approval-side-effect-linkage-composition-plan.md), and the explicit store-backed helper is implemented in [SideEffect Approval Linkage Store-Backed Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REPORT.md) and accepted in [SideEffect Approval Linkage Store-Backed Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REVIEW.md). Explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md). Artifact-gated provider-write composition is implemented as an explicit helper in [Artifact-Gated Provider-Write Composition Helper Report](docs/concepts/ARTIFACT_GATED_PROVIDER_WRITE_COMPOSITION_HELPER_REPORT.md), following [Artifact-Gated Provider-Write Composition Plan](docs/implementation-plans/artifact-gated-provider-write-composition-plan.md), and accepted in [Artifact-Gated Provider-Write Composition Helper Review](docs/concepts/ARTIFACT_GATED_PROVIDER_WRITE_COMPOSITION_HELPER_REVIEW.md). The broader runtime write-readiness checkpoint is planned and accepted in [Runtime Write-Readiness Checkpoint Plan](docs/implementation-plans/runtime-write-readiness-checkpoint-plan.md) and [Runtime Write-Readiness Checkpoint Plan Review](docs/concepts/RUNTIME_WRITE_READINESS_CHECKPOINT_PLAN_REVIEW.md), and the first provider-write sandbox readiness helper is implemented and accepted in [Provider Write Sandbox Readiness Helper Report](docs/concepts/PROVIDER_WRITE_SANDBOX_READINESS_HELPER_REPORT.md) and [Provider Write Sandbox Readiness Helper Review](docs/concepts/PROVIDER_WRITE_SANDBOX_READINESS_HELPER_REVIEW.md). Default executor writes, broader write-capable adapters, runtime side-effect execution expansion, hosted behavior, provider mutation defaults, CLI mutation behavior, schemas, examples, hidden auth loading, automatic recovery, reasoning lineage, and release posture changes remain unimplemented.

This milestone must not introduce domain packs, write-capable adapters, or new runtime primitives until a scoped ADR or implementation plan is accepted.

## Reasoning Lineage / Claim Graph Architecture

The [Governed Work Pattern](docs/concepts/governed-work-pattern.md) is accepted as architecture direction, and [Reasoning Lineage / Claim Graph](docs/concepts/reasoning-lineage.md) remains captured as proposed architecture direction in [ADR 0008](docs/adr/0008-reasoning-lineage-claim-graph.md). Reasoning Lineage is a follow-on provenance direction after Governed Work Pattern, and neither direction is implemented as runtime behavior.

Revisit Reasoning Lineage after the EvidenceReference and WorkReportContract foundations are scoped. Revisit these directions together before policy-gated writes, generic runtime adapter execution, or broader domain packs. Implementation of either direction requires a separate accepted ADR or scoped implementation plan.

This milestone should treat reasoning lineage as supporting structure for governed work, not as the primary workflow runtime. Workflow OS must remain a declarative workflow kernel with durable state, policy gates, approvals, auditability, observability, and adapter boundaries.

### Decision Deliberation And Alternative-Path Lineage

Reasoning Lineage must eventually preserve more than the selected outcome. At
material decision boundaries, Workflow OS should be able to show the governed
decision surface: what was decided, why it was decided, which credible
alternatives were actually considered, and why each alternative was selected,
rejected, deferred, ruled out, or left unresolved.

This requirement applies whether work is performed by one local agent, a human
and agent together, deterministic code, a future composable harness, or a
future multi-harness execution topology. Parent harnesses should cite bounded
lineage records from child harnesses through typed handoffs rather than flatten
private transcripts or every internal execution step into one log.

Workflow OS should preserve five related but distinct trails:

- **execution lineage**: what happened, in what order, and with which result;
- **decision lineage**: why the selected path was chosen;
- **alternative lineage**: which credible paths were considered but not taken
  and why;
- **governance lineage**: which policy, authority, check, approval, budget,
  sensitivity, and SideEffect constraints shaped the decision; and
- **evidence lineage**: which evidence supported, contradicted, weakened, or
  failed to resolve each option.

The future minimum decision-grade deliberation record should evaluate:

- stable decision and actor identities plus delegated authority;
- immutable input, workflow, harness, policy, and context bindings;
- the selected option and a bounded rationale summary;
- a bounded set of alternatives that were actually considered;
- bounded rejection, deferral, or unresolved reason codes and summaries;
- supporting, contradictory, missing, and inconclusive evidence references;
- assumptions, constraints, uncertainty, confidence, and evidence strength;
- policy decisions, checks, approvals, budgets, and authority boundaries;
- the evidence or changed condition that would trigger reconsideration;
- resulting actions, SideEffects, artifacts, handoffs, claims, and report
  sections; and
- additive correction, override, supersession, or escalation relationships.

Workflow OS must not claim that an alternative was considered when the actor
or harness did not provide that record. Missing deliberation must remain
explicit rather than being inferred or fabricated after the fact.

Deliberation requirements should be risk-proportional. Low-risk reversible
work may require only a concise selected-path rationale. Material decisions may
require named alternatives and rejection reasons. High-risk, irreversible, or
high-assurance actions may require evidence per option, contradictory evidence,
counterfactual reconsideration conditions, independent review, or separation
of recommender and approver. Future policy should be able to govern this
decision procedure without forcing the same ceremony onto every decision.

This is **decision-grade provenance**, not private model chain-of-thought.
Workflow OS must not require, store, or claim access to hidden token-by-token
reasoning. Raw prompts, private scratchpads, unrestricted transcripts, provider
payloads, and sensitive internal reasoning are not the lineage contract. The
durable surface should be bounded, structured, reference-first,
redaction-aware, selectively disclosable, and suitable for both machine and
human review.

This direction should eventually support workflow evolution: later reviewers
and machines should be able to identify repeatedly rejected paths, weak
rejection rationales, invalidated assumptions, unresolved alternatives,
decision reversals, and conditions under which a previously rejected option
should be reconsidered. That learning must remain advisory until separately
governed authoring, validation, policy, approval, and promotion make a workflow
change authoritative.

Before implementation, ADR 0008 and the reasoning-lineage concept must be
re-reviewed to define decision, option, rationale, alternative disposition,
evidence relationship, correction, privacy, retention, and selective
disclosure boundaries. The first implementation should remain a model-only,
domain-neutral contract. It must not add transcript capture, hidden reasoning
capture, runtime prompting, agent orchestration, automatic workflow mutation,
persistence, UI, provider behavior, SideEffects, writes, or release changes.

Candidate decisions:

- how to represent claim or finding nodes
- how to represent derivation edges between claims, evidence, validations, decisions, and reports
- how to represent selected options, considered alternatives, rejection or
  deferral reasons, and reconsideration conditions
- which risk levels require alternative-path disclosure, contradictory
  evidence, independent review, or separation of recommender and approver
- how to distinguish missing deliberation from an explicitly recorded
  no-viable-alternative decision without fabricating provenance
- how additive corrections should work without rewriting history
- whether confidence metadata belongs in core, skills, domain packs, or reports
- how actor attribution should attach to generated, reviewed, corrected, or approved claims
- how reference resolution and context binding should connect claims to evidence
- how reasoning lineage should link to evidence references, work reports, audit events, adapter invocation records, validation results, and approval decisions
- what belongs in core versus skills versus domain packs

This milestone must not interrupt Phase 2 live-smoke/public-preview readiness. Implementation of either concept requires a separate accepted ADR or scoped implementation plan.

## Later Production Backend Phase

Production backends are deferred until after local kernel preview release hygiene and adapter readiness criteria are settled.

Future backend work should include:

- Production database contract tests.
- Migration and compatibility strategy for persisted state.
- Backup and restore guidance.
- Corruption detection and repair procedures.
- Locking/fencing semantics.
- Audit persistence and export posture.
- Threat model updates.

## Deferred Until Kernel Correctness And Release Posture

- GitHub write adapters.
- Jira write adapters.
- CI write adapters and additional CI providers.
- Production database backend.
- Distributed workers.
- SaaS control plane.
- UI product.
- Marketplace or package registry.
- High-autonomy external write behavior.
