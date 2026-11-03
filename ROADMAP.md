# Roadmap

Workflow OS grows from the local-first kernel outward.

## Current Status

Status date: 2026-11-03.

Workflow OS is a working local-first governance kernel with governed sequential
multi-step execution, durable run and event state, policy and approval gates,
immutable run bundles, current-fact reassessment, independent local-check proof,
SideEffect governance, evidence-backed WorkReports, existing-repository
onboarding, and bounded workflow recommendation and authoring paths. Selected
runtime paths now prove authoritative continuation rehydration, atomic execution
continuity, trusted-host admission, one-step no-write execution, and terminal
workflow reconciliation without treating agent memory or a host process as the
governance authority.

The shared `PostgreSQL` state milestone and the single-tenant hosted no-write
evaluation milestone are implemented and reviewed. The hosted milestone proves
one authenticated trust domain, stateless fenced workers, Core-owned dispatch,
provider outcome reconciliation, terminal report persistence, restart behavior,
and database-interruption recovery. It is not a production-readiness,
multi-tenancy, enterprise-identity, high-availability, or hosted-SaaS claim.

Project-scoped approval routing and its PostgreSQL authority integration are
implemented and reviewed. The collaborative identity lifecycle Core model and
its blocker fix are implemented and reviewed. Caller-assembled identity facts
now produce only an explicitly non-authorizing validated claim. Planning for a
genuinely authorizing identity source is complete. The first persistent source
slice is implemented: a Core-owned PostgreSQL source stores complete
canonical identity revisions, activates them monotonically, rejects rollback
and conflicting replay, reconstructs and validates the full revision at
database time, and emits payload-free complete exact-query source evidence.
Its focused maintainer/security review is accepted after first-activation
serialization, durable source-version verification, ambiguous-grant failure,
query-minimized count disclosure, and recovery-rehearsal fixes. The source
now has one explicit opt-in consumer: an executor-facing collaborative project
approval decision proves current exact-project `ApprovalDecide` authority in
the same PostgreSQL transaction that records the decision event, approval
projection, and payload-free receipt. It rehydrates committed state before
continuation and reconciles exact restart replay without duplicating the
decision event. No hosted authentication cutover is implemented. A dedicated payload-free
collaborative approval-authority receipt and PostgreSQL v4 create-only store
are implemented and their required-CI and recovery verification blocker is
fixed and accepted. The receipt remains point-in-time trusted evidence and
cannot authorize another action. Focused maintainer/security review found a
narrow runtime-boundary evidence blocker: backend conformance is present, but
the public executor-facing consumer is not invoked by tests, and the exact
run-binding read did not use the accepted locking helper. The focused blocker
fix is implemented and accepted: the exact binding is locked, public-consumer
tests prove success, replay, post-commit recovery, project-scope substitution
rejection, and revoked-authority non-invocation, and required PostgreSQL CI
proves the corrected receipt replay and backup/restore behavior. No additional
identity consumer is authorized by that acceptance.

The first provider-write sandbox is implemented and accepted for GitHub pull
request comments only. The second bounded Core slice now implements an
explicit injected-provider helper for managed draft GitHub pull request
creation from an already-pushed branch. The phase-level maintainer and security
review is accepted after fix-forward hardening added exact terminal report
artifact identity, current V3 runtime-fact reassessment, adapter-write policy
semantics, and granted approval-presentation proof-marker enforcement. Both
paths remain explicit and sandbox-bound. No provider write is enabled by
default. Concrete GitHub.com transport and environment-gated live-smoke
planning is documented, and the explicit GitHub.com transport is now
implemented behind the accepted provider trait. It pins API version
`2026-03-10`, disables redirects and retries, bounds response parsing, supports
same-repository branches only, and conservatively treats uncertainty after the
create call boundary as may-have-started. Ordinary tests are network-free; one
full-helper smoke is ignored by default and compile-time allowlisted to a
dedicated sandbox repository. The phase review is accepted with non-blocking
follow-ups. The live sandbox smoke has passed against the exact allowlisted
repository and pinned refs. It created one managed draft pull request and the
same unchanged test independently reconciled that draft without a second
create. Focused maintainer/security review accepts the proof with non-blocking
operational follow-ups. No default write behavior or additional mutation
family is authorized by this proof.

## Roadmap At A Glance

Only **Active Delivery Queue** is the authoritative forward-looking sequence.
The lane records and historical phase evidence below preserve accepted
boundaries and explain how the project arrived here; their embedded `next
phase` language does not supersede the active queue.

### Active Delivery Queue

A P0 governed-phase kernel-freshness blocker is accepted and merged. Every live
material `phase-start` now rebuilds the Workflow OS CLI
from the exact current checkout before validation, run creation, approval
creation, or phase-state persistence. Existing binaries, timestamps, and
version strings are not freshness evidence. Live material phase starts reject
`--no-build`, and build failure must leave no governed run or approval state.
Dry-run and non-phase helper behavior remain unchanged. See the [P0
report](docs/concepts/GOVERNED_PHASE_KERNEL_FRESHNESS_P0_REPORT.md) and
[focused review](docs/concepts/GOVERNED_PHASE_KERNEL_FRESHNESS_P0_REVIEW.md).

A P0 acceptance-hardening blocker is accepted and merged after independent fresh-pull
evaluation reproduced five defects on current `main`: parallel local-host test
fixtures could collide, a two-gate first-run WorkReport described one approval,
generated agent guidance contradicted the explicit first-run artifact path, the
report emitted an invalid inspection command, and aggregate-governance and
authored-step approvals used semantically indistinguishable presentation copy.
The accepted fix is intentionally narrow: make fixture identity deterministic,
make reports enumerate all approval requests and decisions, emit a valid
copy-safe inspection command, align generated guidance with the bounded
artifact capability, and key approval presentation scope to the actual approval
subject. Required CI was terminal-green for the exact reviewed commit. See the [P0 implementation
report](docs/concepts/FIRST_RUN_REPORT_TRUTH_AND_TEST_STABILITY_P0_REPORT.md)
and [focused review](docs/concepts/FIRST_RUN_REPORT_TRUTH_AND_TEST_STABILITY_P0_REVIEW.md).

A later P0 preemption now makes **authoritative post-compaction understanding
and continuation rehydration** the active delivery lane. This is the only
authorized forward implementation sequence until its exit criteria are met.
The preemption is required because a replacement executor must recover the same
bounded, evidence-backed understanding of lawful work rather than merely the
same runtime disposition. Reconstructed orientation remains non-authorizing.

The accepted increments are:

1. the authoritative continuation context inventory, bundle, understanding,
   gap, and non-authorizing decision model;
2. purpose-specific complete source-coverage profiles;
3. claim-level semantic obligation profiles;
4. a reviewed trusted-local-collector architecture with source-specific
   revision fences and non-consuming observation; and
5. the private Phase A closed collector model, accepted after fixing complete
   typed negative-knowledge accounting for empty results.

Phase B existing-store-backed collectors are now implemented as a private,
non-consuming layer over the accepted store interfaces. Stable observations
cover declarations, immutable bundles, durable state, event history, policies,
approvals, checks, side effects, and report artifacts. Capability,
required-context, and evidence collection remain typed `Unsupported` because
their current interfaces do not yet prove complete revision-fenced,
non-consuming reads. The implementation does not create new stores or weaken
those contracts. See the [Phase B implementation
report](docs/concepts/CONTINUATION_CONTEXT_STORE_BACKED_COLLECTORS_REPORT.md).
Focused maintainer/security review accepts Phase B after adding direct proofs
for durable-snapshot drift rejection and unchanged revision stability after
backend restart. See the [Phase B
review](docs/concepts/CONTINUATION_CONTEXT_STORE_BACKED_COLLECTORS_REVIEW.md).

Phase C bounded-local-observer implementation is now accepted after focused
maintainer/security review corrected one producer-substitution blocker. A
private injected observer contract can supply typed Git/worktree metadata and
observations for exactly `AGENTS.md`, `docs/ENGINEERING_STANDARD.md`, and
`ROADMAP.md`. Core validates request binding, source revisions, bounded counts,
closed document identity, path-safety attestations, size and sensitivity
ceilings, deterministic ordering, debug redaction, and one payload-free observer
contract commitment across anchor capture and collection. That commitment binds
continuity; it does not authenticate or authorize a future host producer. The
implementation does not shell out to Git, read the filesystem, trust arbitrary
paths, or register a runtime consumer. No stable typed-handoff store exists, so
typed handoff remains explicitly `Unsupported` rather than being inferred or
fabricated. See the [Phase C implementation
report](docs/concepts/CONTINUATION_CONTEXT_BOUNDED_LOCAL_OBSERVERS_REPORT.md)
and [focused review](docs/concepts/CONTINUATION_CONTEXT_BOUNDED_LOCAL_OBSERVERS_REVIEW.md).

Phase D accepted-intent capture is now implemented and accepted after focused
maintainer/security review as a Core-private, non-authorizing contract. Accepted
semantic effects are attributable to one actor, bound to exact project,
workflow, run, and phase scope, committed to one directive provenance hash,
bounded by the existing continuation text model, and stored through an atomic
compare-and-replace interface. Capture is idempotent for the exact same record,
rejects identity or directive rebinding, requires explicit same-family
supersession for singleton objective, priority, pause, stop, and resume posture,
and verifies the committed replacement by exact revision readback. The private
collector admits records without manufacturing a generic source payload and
rejects store drift after anchor capture. The phase adds only a store contract
and in-memory conformance fixture; no production persistence backend, host
directive ingestion path, raw conversation storage, public API, or runtime
consumer exists. Review corrected two blocking replay defects by binding exact
supersession lineage into the durable record and accepting replay from the
original pre-write request only when the complete record is identical. See the
[Phase D implementation
report](docs/concepts/CONTINUATION_CONTEXT_ACCEPTED_INTENT_CAPTURE_REPORT.md)
and [focused review](docs/concepts/CONTINUATION_CONTEXT_ACCEPTED_INTENT_CAPTURE_REVIEW.md).

The Phase E reference aggregate composition is implemented and accepted after
focused maintainer/security review. The private closed reference registry can
compose one canonical fixture observation set under equal pre- and
post-collection anchors, derive the existing claim-level non-authorizing
decision in the same call, and return a redaction-safe proof chain binding the
request, exact collector/source/authority contract, anchors, observations,
coverage, understanding, and decision commitments. The proof distinguishes
complete accounting for all 18 collector classes from whether required sources
were actually available; unavailable, unsupported, denied, faiRK_REPORT_ARTIFACT_QUEUE_SELECTION_REPORT.md).

- **Posted and verified the accepted OpenShell `v0.1.5` follow-up comment
  exactly once.** Existing Discussion #2661 now contains one stable comment at
  [discussioncomment-19310244](https://github.com/NVIDIA/OpenShell/discussions/2661#discussioncomment-19310244).
  Its raw GitHub body uses CRLF line endings; after explicit LF
  canonicalization, the 4,503-byte content exactly matches accepted SHA-256
  `4541ea7d05380112d86e45f78c225fd98ddf278531031452eb928289a6f700a8`.
  Exact verification caught one rich-paste transformation, and the same
  comment was corrected in place without creating a duplicate. Provider
  wiring remains blocked and no OpenShell maintainer response has been
  evaluated. See the [submission
  report](docs/concepts/OPENSHELL_V0_1_5_FOLLOW_UP_SUBMISSION_REPORT.md).

- **Accepted the OpenShell `v0.1.5` upstream gap refresh and follow-up draft.**
  Focused maintainer/security review verifies source fidelity, selected Docker
  projection claims, improvement credit, collaborative tone, privacy,
  non-duplication, no-fork posture, and the separate external-action boundary.
  The exact accepted body is committed by hash and may be posted only through
  one separately governed action on existing Discussion #2661. See the
  [review](docs/concepts/OPENSHELL_V0_1_5_UPSTREAM_GAP_REFRESH_REVIEW.md).

- **Refreshed the OpenShell upstream gap record and discussion draft for stable
  `v0.1.5`.** The proposal now credits durable request identity, desired-state
  operations, lifecycle ownership, endpoint status, deletion outcomes, and new
  workload/fence vocabulary while retaining exact selected-driver,
  reconciliation, observation, capability, and cleanup blockers. The original
  Discussion #2661 body remains unchanged; one separate follow-up comment
  draft is prepared but neither reviewed nor posted. See the
  [phase report](docs/concepts/OPENSHELL_V0_1_5_UPSTREAM_GAP_REFRESH_REPORT.md).

- **Accepted the OpenShell `v0.1.5` execution-evidence matrix.** Focused
  maintainer/security review confirms the exact stable and selected Docker
  findings, conservative fact classifications, fail-closed mappings, privacy
  boundary, blocked provider verdict, and no-fork posture. Schema vocabulary
  remains insufficient where the Docker production projection is empty, and
  launch admission remains distinct from terminal reconciliation. The next
  phase may refresh upstream communication material only; transport, smoke,
  provider wiring, and execution remain blocked. See the
  [review](docs/concepts/OPENSHELL_V0_1_5_EXECUTION_EVIDENCE_MATRIX_REVIEW.md).

- **Completed the OpenShell `v0.1.5` execution-evidence matrix.** The stable
  release materially strengthens request identity, lifecycle ownership,
  desired-state operations, endpoint status, and delete-target identity. The
  selected Docker driver does not project required workload/fence evidence,
  and exact exec reconciliation, observation completeness, owner-loss
  resolution, and terminal cleanup remain blocked. Focused matrix review is
  next; no transport or smoke is authorized. See the
  [matrix](docs/implementation-plans/openshell-v0-1-5-execution-evidence-sufficiency-matrix.md).

- **Accepted the trusted-host local Cargo execution-provider suitability
  re-evaluation.** Focused maintainer/security review confirms that stable
  OpenShell `v0.1.5` materially closes or narrows former request-identity,
  workload/fence-evidence, desired-state-operation, and deletion gaps without
  closing exact exec reconciliation, pending-driver owner loss, complete
  operation observations, terminal cleanup, or selected-platform proof. The
  refreshed `v0.1.5` evidence matrix is next; provider wiring and a fork remain
  unjustified. See the [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_EXECUTION_PROVIDER_SUITABILITY_REEVALUATION_REVIEW.md).

- **Re-evaluated trusted-host local Cargo execution-provider suitability.**
  Stable OpenShell `v0.1.5` materially improves request identity, observed
  workload/fence evidence, capability negotiation, desired-state operations,
  and deletion outcomes. Native execution remains unsuitable, and OpenShell
  wiring remains blocked on exact exec reconciliation, pending-driver owner
  loss, complete operation observations, terminal cleanup proof, and a live
  selected-platform proof. A refreshed exact `v0.1.5` evidence matrix is next;
  a fork remains unjustified. See the
  [decision](docs/implementation-plans/trusted-host-local-cargo-execution-provider-suitability-reevaluation.md).

- **Accepted the local Cargo composition observation-binding blocker fix.**
  Focused review verifies that the private coordinator cannot accept provider
  observations, exact absence produces only `DeniedBeforeStart` with
  `ObservationUnavailable`, deterministic fingerprints are input-sensitive,
  and the future Core SPI remains non-authorizing. Provider suitability
  re-evaluation is next; provider wiring remains blocked. See the [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_BINDING_BLOCKER_FIX_REVIEW.md).

- **Fixed the local Cargo composition observation-binding blocker.** The
  private pre-start coordinator no longer accepts provider observations that
  it cannot commit. It routes explicit absence to the Core assessment SPI,
  preserves deterministic pre-start denial, and now tests unchanged replay and
  changed-request fingerprint sensitivity. Focused blocker-fix review is next.
  See the [fix report](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_BINDING_BLOCKER_FIX_REPORT.md).

- **Reviewed the private local Cargo observation/execution composition with one
  blocker.** Immutable execution, observer request/evidence, requirements,
  attempt, Core-owned assessment, pre-start denial, process-free posture, and
  privacy are sound. The optional provider-observation input is not included
  in the composition commitment or returned outcome, so exact provider claims
  are not yet identity-bound. A narrow blocker fix is required before provider
  suitability work. See the [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_EXECUTION_COMPOSITION_REVIEW.md).

- **Implemented the private local Cargo observation/execution composition
  contract.** The process-free coordinator validates exact immutable execution
  and observation bindings, validates the observer evidence bundle, delegates
  security assessment to Core, and returns `DeniedBeforeStart` when execution-
  provider observations are absent. It exposes no runner or process capability
  and adds no runtime, persistence, event, schema, CLI, provider, or write
  behavior. Focused maintainer/security review is next. See the
  [implementation report](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_EXECUTION_COMPOSITION_REPORT.md).

- **Accepted same-call local Cargo observation and execution composition
  planning.** Review confirms that same-call adjacency is not custody and the
  native observer cannot unlock Cargo execution. The accepted first
  implementation is a private injected contract that uses Core-owned security
  assessment and proves deterministic pre-start denial without entering a
  runner. See the [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_EXECUTION_COMPOSITION_PLAN_REVIEW.md).

- **Planned same-call local Cargo observation and execution composition.** The
  plan defines one private local-host coordinator and exact bindings across
  immutable run, observer, roots, executables, environments, live-security
  assessment, invocation, result, attestation, and reporting. It preserves the
  key security conclusion: successful native preflight observation is not
  admission-to-terminal custody and cannot start Cargo. Focused
  maintainer/security review is next. See the [planning
  report](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_OBSERVATION_EXECUTION_COMPOSITION_PLAN_REPORT.md).

- **Accepted the fixed local Cargo source and opt-in smoke.** Focused
  maintainer/security review confirms closed typed selection, fixed probe
  arguments, cleared environment, bounded process behavior, repository and
  executable drift detection, conservative fact posture, process-free
  ordinary tests, and the isolated Apple Silicon smoke. The review preserves
  the central non-claim: pre/post commitments are useful preflight evidence,
  not same-call custody or execution authority. Same-call composition
  planning is next. See the [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_SOURCE_REVIEW.md).

- **Implemented the fixed local Cargo source and opt-in smoke.** The private
  local-host source accepts only canonical closed roots and the exact Cargo,
  rustc, rustdoc, cargo-clippy, and clippy-driver executable set. It performs
  four fixed probes with absolute executables, a cleared environment, bounded
  output, timeout/process-group termination, strict parsers, and pre/post
  identity checks. Ordinary tests remain process-free, and one ignored
  `aarch64-apple-darwin` smoke passes when explicitly selected. No fact is
  promoted to `Observed`, and no runtime composition or admission is added.
  Focused review is next. See the [implementation
  report](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_SOURCE_REPORT.md).

- **Accepted private local Cargo trusted-host observer contracts.** Focused
  maintainer/security review confirms exact request, producer, repository,
  revision, posture, and in-memory evidence binding; injected source closure;
  deterministic references; and non-leaking failures. The current definition
  remains a commitment envelope, so the concrete source must add typed closed
  probe inputs rather than interpreting hashes or ambient state. See the
  [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_CONTRACT_REVIEW.md).

- **Implemented private local Cargo trusted-host observer contracts.** The
  unpublished local-host crate now contains a selection-bound definition,
  private observer and injected source interfaces, exact in-memory evidence
  entries and bundle validation, and deterministic process-free tests. No
  concrete filesystem or process source exists, and no fact can claim
  `Observed` before same-call execution continuity is implemented. Focused
  review is next. See the [implementation
  report](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_CONTRACT_REPORT.md).

- **Accepted local Cargo trusted-host observer provider plan.** Focused
  maintainer/security review accepts `workflow-local-host` ownership, exact
  selection and ordering binding, conservative no-`Observed` preflight
  ceilings, deterministic in-memory evidence resolution, fixed probe and
  privacy rules, one initial platform, and continued OpenShell deferral. The
  private injected contract slice is next. See the [focused
  review](docs/concepts/TRUSTED_HOST_LOCAL_CARGO_INPUT_OBSERVER_PROVIDER_PLAN_REVIEW.md).

- **Closed local Cargo trusted-host observer provider planning.** The plan
  selects one native read-only `workflow-local-host` producer, maps all nine
  host obligations and repository-root observation to exact evidence needs
  and conservative posture ceilings, and preserves an intentionally
  non-authorizing result when preflight cannot prove execution continuity.
  OpenShell remains a later optional containment provider and its accepted
  upstream gaps still block wiring. Focused plan review is next. See the
  [implementation
  plan](docs/implementation-plans/trusted-host-local-cargo-input-observer-provider-plan.md).

- **Accepted fixture-backed in-memory trusted-host input observer proof.**
  Focused maintainer/security review confirms that the helper accepts only
  explicit fixture-only producers, derives exact authoritative bindings from
  the request, reuses the accepted Core constructor and validator, produces
  deterministic results, rejects incomplete, stale, or substituted inputs,
  and remains process-free and non-authorizing. See the [focused
  review](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_FIXTURE_PROOF_REVIEW.md).

- **Closed fixture-backed in-memory trusted-host input observer proof.** An
  explicit synthetic fixture source now constructs a complete
  exrusted-Host Cargo Format Check Profile Blocker Fix Report](docs/concepts/TRUSTED_HOST_CARGO_FMT_PROFILE_BLOCKER_FIX_REPORT.md).
   The acceptance record is in
   [Trusted-Host Cargo Format Check Profile Blocker Fix Review](docs/concepts/TRUSTED_HOST_CARGO_FMT_PROFILE_BLOCKER_FIX_REVIEW.md).
   The accepted profile uses the already modeled `cargo fmt --all --check`
   command with complete formatter-chain identity and persistent-mutation
   detection without claiming sandbox prevention. The
   [Trusted-Host Cargo Format Workflow Composition Plan](docs/implementation-plans/trusted-host-cargo-fmt-workflow-composition-plan.md)
   now defines one additive exact three-step workflow that preserves the
   accepted project-validation and docs-check prefix, appends Cargo format,
   retains the existing two-step v3 path unchanged, and uses a distinct v4
   report/artifact commitment. Focused plan review accepts the plan after
   correcting two blockers: every attempted step must bind its durable
   trusted-host dispatch executor commitment, and pre-result integrity failure
   must be reported from the exact failed-invocation history without a
   fabricated local-check result or unpersisted cause. The implementation is
   complete. It adds the exact closed three-profile registry, immutable
   three-contract bundle, Core-derived three-step progression, bounded
   three-operation host loop, causal-history validation, restart-safe exact
   replay, and a distinct create-only v4 terminal report artifact. The report
   verifies each durable dispatch commitment against immutable step input;
   pre-result identity failure records no fake local-check result and discloses
   later checks as not attempted. Full workspace validation and the opt-in real
   bundled-toolchain Cargo format smoke pass. Focused implementation/security
   review found blockers: resumed admission did not validate complete
   attempted-step dispatch history, failed terminal history is under-
   constrained, pre-result failed invocation identity is not linked to the
   authorized attempt, and required corruption/security regressions are
   missing. The narrow blocker fix is implemented. It validates exactly one
   causally and cryptographically matching dispatch for every attempted step,
   constrains failed terminal history to accepted-result or exact pre-result
   truth, binds pre-result failure to the deterministic invocation request,
   cites schedule events in v4 reports, and adds direct corruption,
   substitution, artifact-conflict, and privacy regressions. Focused
   maintainer/security re-review found one remaining correctness blocker and
   two proof gaps: pre-result failure is incorrectly limited to the final
   workflow step rather than the last scheduled step, and direct duplicate-
   dispatch and composed-timeout regressions are still missing. The additional
   narrow blocker fix is implemented: history validation now uses the durable
   scheduled prefix, early pre-result failures remain reportable without fake
   result truth, duplicate dispatch commitments fail closed, and composed
   Cargo timeout produces a bounded v4 artifact. Final focused re-review
   accepts the fix with no remaining blocker, closing this exact composition
   lane. Clippy, tests, arbitrary commands, providers,
   external writes, public surfaces, hosted expansion, and topology broadening
   remain out of scope. See
   [Trusted-Host Cargo Format Workflow Composition Implementation Report](docs/concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_IMPLEMENTATION_REPORT.md)
   and
   [Trusted-Host Cargo Format Workflow Composition Implementation Review](docs/concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_IMPLEMENTATION_REVIEW.md),
   plus the
   [Trusted-Host Cargo Format Workflow Composition Blocker Fix Report](docs/concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_BLOCKER_FIX_REPORT.md)
   and
   [Trusted-Host Cargo Format Workflow Composition Blocker Fix Review](docs/concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_BLOCKER_FIX_REVIEW.md).
   The final narrow fix record is in
   [Trusted-Host Cargo Format Workflow Composition Final Blocker Fix Report](docs/concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_FINAL_BLOCKER_FIX_REPORT.md).
   The final acceptance record is in
   [Trusted-Host Cargo Format Workflow Composition Final Blocker Fix Review](docs/concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_FINAL_BLOCKER_FIX_REVIEW.md).

   Historical accepted evidence:

   **Bounded two-step no-write trusted-host workflow.**
   The private trusted-host terminal WorkReport artifact blocker fix is
   accepted and merged. The same closed runtime now advances from independent
   one-step profiles to one exact sequential workflow:
   `workflow-os` project validation followed by docs check, then one terminal
   report and private artifact. The [Trusted-Host Two-Step No-Write Workflow
   Plan](docs/implementation-plans/trusted-host-two-step-no-write-workflow-plan.md)
   requires Core-owned runnable-step and profile selection, atomic first-step
   result/invocation/next-step progression, restart-safe exact replay, and one
   honest terminal report over the complete durable history. Focused
   maintainer/security review accepts the plan after a private v3 report-
   commitment correction in [Trusted-Host Two-Step No-Write Workflow Plan
   Review](docs/concepts/TRUSTED_HOST_TWO_STEP_NO_WRITE_WORKFLOW_PLAN_REVIEW.md).
   The implementation adds Core-owned runnable-step/profile selection, atomic
   first-step result/invocation/next-step progression, restart-safe exact
   replay, and one private v3 terminal report/artifact over complete durable
   history. Focused maintainer/security review found and fixed an
   outcome-domain blocker and a causal-history validation blocker before
   accepting the phase. Do not add a third profile, arbitrary commands,
   providers, external writes, public CLI output, schemas, examples, hosted
   expansion, parallelism, or nested harnesses.

   The completed private artifact history is preserved below. The private
   artifact path composes the accepted v2 report, existing no-provider gates,
   and SQLite store, reconciles only exact duplicates, and returns nonzero host
   posture for generation or persistence failure without rewriting workflow
   terminal truth.

   Historical accepted evidence:

   **Private trusted-host terminal WorkReport artifact path.**
   The second closed no-write profile is implemented and focused-review
   accepted. The next executable composition slice is now implemented from the
   [Trusted-Host Terminal WorkReport Composition
   Plan](docs/implementation-plans/trusted-host-terminal-work-report-composition-plan.md).
   Core preserves the actual same-call local-check result in one payload-free
   event committed atomically with invocation and run terminalization,
   rehydrates the exact run and immutable bundle, and only then constructs a
   validated report with deterministic private identity. Report failure does
   not rewrite workflow pass/fail semantics, and exact replay neither reruns
   the process nor appends events. The implementation report is recorded in
   [Trusted-Host Terminal WorkReport Composition Report](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_COMPOSITION_REPORT.md).
   Focused maintainer/security review found one blocker: composition validated
   identities but did not enforce result-status compatibility with the
   invocation/run terminal kinds or the strict result-to-invocation-to-run
   event order promised by the plan. The narrow fix is implemented with
   fail-closed status and ordering checks plus mismatch tests, as documented in
   [Trusted-Host Terminal WorkReport Composition Blocker Fix Report](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_COMPOSITION_BLOCKER_FIX_REPORT.md).
   Focused blocker-fix review now accepts the integrity correction in
   [Trusted-Host Terminal WorkReport Composition Blocker Fix
   Review](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_COMPOSITION_BLOCKER_FIX_REVIEW.md).
   Planning is complete in the [Trusted-Host Terminal WorkReport Artifact
   Plan](docs/implementation-plans/trusted-host-terminal-work-report-artifact-plan.md).
   The plan reuses existing artifact models, gates, and SQLite storage; requires
   a storage-neutral v2 private report before persistence; preserves terminal
   truth across artifact failure; reconciles only exact duplicate artifacts;
   and promotes full corrupt-history entry-point fixtures from follow-up to
   implementation prerequisite. Focused maintainer/security review accepts the
   plan in [Trusted-Host Terminal WorkReport Artifact Plan
   Review](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_ARTIFACT_PLAN_REVIEW.md).
   The private artifact path is now implemented. Core composes the accepted v2
   report, constructs a validated artifact, invokes the existing no-provider
   integrity gate, writes through the existing SQLite store, and reconciles
   only fully equal duplicates. The unpublished host emits fixed persistence
   posture without report contents or identifiers. Focused entry-point tests
   prove passing and failed terminal artifacts, process-free exact replay,
   recovery after terminal commit with no artifact, event-history immutability,
   and fail-closed rejection of contradictory result, invocation, run,
   cancellation, and event-order histories before any replacement artifact is
   written. The implementation is documented in [Trusted-Host Terminal
   WorkReport Artifact Implementation Report](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_ARTIFACT_IMPLEMENTATION_REPORT.md).
   Focused maintainer/security review found one blocker in [Trusted-Host
   Terminal WorkReport Artifact Implementation Review](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_ARTIFACT_IMPLEMENTATION_REVIEW.md):
   generation and persistence failure postures were rendered as fixed failure
   text but still returned through the host success path, and the corruption
   fixtures permitted that zero-exit behavior. The narrow blocker fix is now
   implemented in [Trusted-Host Terminal WorkReport Artifact Blocker Fix
   Report](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_ARTIFACT_BLOCKER_FIX_REPORT.md).
   Both artifact failure postures now return bounded non-success host results;
   corrupt-history fixtures require non-success; and focused real-SQLite tests
   prove valid conflicting and unreadable duplicates fail closed without
   overwrite, repair, event mutation, or identifier disclosure. Perform one
   focused maintainer/security review of this correction before accepting the
   private artifact path. That review now accepts the correction in
   [Trusted-Host Terminal WorkReport Artifact Blocker Fix
   Review](docs/concepts/TRUSTED_HOST_TERMINAL_WORK_REPORT_ARTIFACT_BLOCKER_FIX_REVIEW.md):
   both failure postures exit nonzero, durable terminal truth remains
   unchanged, conflicting and unreadable duplicates remain untouched, and all
   focused and workspace validation passes.
   That completed phase added no released CLI output, another profile,
   arbitrary command, scheduler, nested harness, automatic approval, provider,
   external write, schema, example, or hosted expansion.

Queue maintenance rule: every merged roadmap phase must update this list and
the status date in the same PR. Completed work moves into the lane evidence or
milestone table; dependency-blocked work stays explicit and does not masquerade
as the next executable phase.

### Current Lane Evidence

This section records the active architectural lanes and their accepted proof
history. It is reference material, not a second delivery queue.

1. **P0 authoritative agent continuation context and rehydration.** External
   dogfood testing found that long-running model sessions, context compaction,
   and parallel subagents can continue from a stale conversational summary
   instead of re-reading current Workflow OS state. Model context is working
   memory, not governance authority. Before additional provider mutation proof,
   nested harness execution, or broader autonomous dogfooding, Workflow OS must
   make current kernel state the mandatory source of every material next
   action. The accepted directOCKER_FIX_REVIEW.md).
   Atomic projection is now implemented for the accepted SQLite semantic V2
   backend. All five continuity operations reuse one transaction boundary for
   continuity mutation, operation receipt, one closed bounded runtime event,
   deterministic snapshot advancement, and relational projection binding.
   Exact replay reuses the committed binding without another event, generic
   SQLite event append derives its snapshot atomically, stale caller-authored
   snapshots fail closed, and explicit schema V3 migration refuses historical
   V2 continuity operations that cannot be projected honestly. See the
   [implementation report](docs/concepts/AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_REPORT.md).
   Focused maintainer/security review found blockers in complete projected
   reconciliation and reusable projected-operation fault/conformance proof.
   See the [implementation
   review](docs/concepts/AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_REVIEW.md).
   The bounded blocker fix is now implemented. Fresh-connection reconciliation
   verifies and returns the complete operation/event/snapshot binding;
   before/during/after faults cover all five projected operation families;
   generic event contention and unsupported-backend zero-write behavior are
   explicit. See the [blocker-fix
   report](docs/concepts/AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_BLOCKER_FIX_REPORT.md).
   Focused blocker-fix re-review accepts the fresh-connection reconciliation,
   all-five-family fault proof, cursor contention, and unsupported-backend
   zero-write guarantees. See the [blocker-fix
   review](docs/concepts/AUTHORIZED_EXECUTION_CONTINUITY_EVENT_STATE_PROJECTION_BLOCKER_FIX_REVIEW.md).
   Operational window opening and one local injected trusted-host supervisor
   vertical slice are now the next separate P0 runtime boundary. The boundary
   is specified in the [Operational Execution Window And Trusted-Host
   Supervisor Plan](docs/implementation-plans/operational-execution-window-trusted-host-supervisor-plan.md).
   The plan requires Core to commit an exact window and first started attempt
   before a one-shot injected host may invoke work; the host cannot construct
   authority, approve gates, fabricate completion, or create model turns. See
   the [planning report](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_TRUSTED_HOST_SUPERVISOR_PLAN_REPORT.md).
   Focused maintainer/security review accepts the plan after incorporating
   corrections that prohibit caller-authored opening authority, preserve the
   accepted closed five-operation continuity contract, and bind the first
   window to exactly `invoke_current_step_skill`. See the [plan
   review](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_TRUSTED_HOST_SUPERVISOR_PLAN_REVIEW.md).
   The next implementation phase is the separately versioned operational
   opening capability only; that opening-only phase is now implemented for
   explicit local SQLite state. Core derives opening authorization inside one
   registered current-authority same-call use, atomically creates an
   `executing` window and first `started` attempt, projects one bounded runtime
   event/snapshot, and supports exact replay and fresh-connection
   reconciliation. The accepted five-operation continuity contract remains
   closed, pre-opening continuity state is not assigned invented authority,
   and filesystem/PostgreSQL support remains unavailable. See the [opening
   implementation report](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_OPENING_REPORT.md).
   Focused maintainer/security review found blocker-level gaps in request-
   commitment completeness, expected snapshot binding, serialized projection
   integrity, concurrent-opener/cursor-contention proof, and commit-fault
   proof. See the [opening review](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_OPENING_REVIEW.md).
   The bounded opening blocker fix is now implemented. Request commitment V2
   binds the receipt, actor, immutable run bundle, expected event cursor and
   snapshot, authority and Core-governance commitments, expiry, attempt budget,
   trusted-time observation/provenance/epoch, and exact operation binding.
   Opening projection V2 uses validated deserialization and recomputes its
   canonical commitment. Concurrent opener, generic-event cursor contention,
   and before/during/after commit-fault tests prove one-winner and
   reconciliation-only recovery without returning authority on ambiguity. See
   the [blocker-fix
   report](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_OPENING_BLOCKER_FIX_REPORT.md).
   Focused maintainer/security re-review accepts the repaired opening boundary
   with non-blocking test-harness and future snapshot-wire follow-ups. The
   complete request and snapshot binding, validated event reconstruction,
   one-winner contention behavior, and no-capability ambiguity posture are
   sufficient for the next private slice. See the [blocker-fix
   review](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_OPENING_BLOCKER_FIX_REVIEW.md).
   The one-shot injected local trusted-host supervisor from the accepted plan
   is now implemented as a private Core boundary. It consumes only an opening
   or resumed attempt capability, validates the exact
   `invoke_current_step_skill` binding, calls one injected executor once,
   records one bounded outcome or turn-boundary yield through the accepted
   projected continuity operations, and returns the current Core-derived
   disposition. The focused local proof opens the first attempt, records a
   turn-boundary yield, derives `ResumeNow`, consumes one directive, invokes an
   existing local `SkillHandler`, records success, and proves that supervisor
   success does not complete the workflow run. Focused maintainer/security
   review found blockers in one-use dispatch authority, complete immutable
   invocation and executor binding, ambiguous result-persistence
   reconciliation, and the closed result/fault regression matrix. The bounded
   blocker fix now consumes attempt authority by value, validates current
   dispatchability before executor entry, binds every invocation field and an
   explicit executor identity before authority issuance, reconciles ambiguous
   projected writes through a fresh connection, and covers the closed result,
   replay, substitution, yield, and fault matrix. See the
   [supervisor review](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_TRUSTED_HOST_SUPERVISOR_REVIEW.md).
   See also the [supervisor blocker-fix
   report](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_TRUSTED_HOST_SUPERVISOR_BLOCKER_FIX_REPORT.md).
   Focused maintainer/security re-review accepts the repaired private one-shot
   boundary with non-blocking follow-ups. Owned capability transfer, fresh
   durable dispatchability validation, complete invocation/executor binding,
   exact reconciliation, and the closed regression matrix are sufficient for
   one conforming injected caller. The review also makes the next boundary
   explicit: dispatchability is currently a read, not an atomic reservation.
   See the [supervisor blocker-fix
   review](docs/concepts/OPERATIONAL_EXECUTION_WINDOW_TRUSTED_HOST_SUPERVISOR_BLOCKER_FIX_REVIEW.md).
   The next P0 phase is a bounded atomic dispatch-reservation slice before any
   repeated supervisor loop. Its planning boundary is documented in the
   [Atomic Dispatch Reservation
   Plan](docs/implementation-plans/atomic-dispatch-reservation-plan.md) and
   [planning report](docs/concepts/ATOMIC_DISPATCH_RESERVATION_PLAN_REPORT.md).
   The plan requires one atomic winner, never reconstructs authority from
   durable replay or ambiguous commit reconciliation, and keeps the result,
   yield, and recovery operation set closed. Its [focused maintainer/security
   review](docs/concepts/ATOMIC_DISPATCH_RESERVATION_PLAN_REVIEW.md) found one
   planning blocker: dispatch admission needed an exact append-only event and
   atomic projection contract before implementation. The bounded fix now
   defines the payload-free admission event, atomic snapshot binding,
   reservation-bound outcome/yield/recovery inputs, and module-private
   capability construction. The [focused fix
   review](docs/concepts/ATOMIC_DISPATCH_RESERVATION_PLAN_BLOCKER_FIX_REVIEW.md)
   accepted the plan. The private SQLite one-winner implementation is now
   complete and documented in the [Atomic Dispatch Reservation Implementation
   Report](docs/concepts/ATOMIC_DISPATCH_RESERVATION_IMPLEMENTATION_REPORT.md).
   One immediate transaction admits exactly one caller, appends and projects
   one payload-free admission event, returns non-reconstructable in-memory
   authority only after an unambiguous commit, binds outcome/yield/recovery
   writes to that admission, and withholds authority on replay or ambiguous
   commit reconciliation. The focused maintainer/security
   [implementation review](docs/concepts/ATOMIC_DISPATCH_RESERVATION_IMPLEMENTATION_REVIEW.md)
   found and fixed one blocker: replay and downstream use now cross-check the
   relational reservation, durable admission event, point-in-time projection,
   and current rehydrated run snapshot. The private one-shot slice is accepted.
   The next P0 phase is bounded trusted-host redispatch-loop planning over the
   accepted execution-window, directive, supervisor, and reservation
   boundaries. That planning phase is documented in the [Bounded Trusted-Host
   Redispatch Loop
   Plan](docs/implementation-plans/bounded-trusted-host-redispatch-loop-plan.md)
   and [planning
   report](docs/concepts/BOUNDED_TRUSTED_HOST_REDISPATCH_LOOP_PLAN_REPORT.md).
   The plan keeps Core authoritative for every iteration: only a freshly
   derived `ResumeNow` may consume a fresh directive, commit a one-winner
   dispatch reservation, and invoke one injected executor. Typed wait,
   blocked, terminal, ambiguous, corrupt, and security-rejected posture stop
   the loop without fabricated progress. The durable execution-window attempt
   limit is the finite bound; no caller-selected host budget may masquerade as
   workflow completion or wait.
   The focused maintainer/security
   [plan review](docs/concepts/BOUNDED_TRUSTED_HOST_REDISPATCH_LOOP_PLAN_REVIEW.md)
   found two blockers. The focused planning fix removes caller-selected budget
   exhaustion as a normal stop: the durable execution-window attempt limit is
   now the finite bound, and the helper may not return normally while Core
   still says `ResumeNow`. The fix also makes exact `SkillInput` and executor
   binding immutable loop inputs and limits the injected provider to fresh
   non-authorizing operation identities. The correction is documented in the
   [blocker-fix
   report](docs/concepts/BOUNDED_TRUSTED_HOST_REDISPATCH_LOOP_PLAN_BLOCKER_FIX_REPORT.md).
   The focused
   [blocker-fix review](docs/concepts/BOUNDED_TRUSTED_HOST_REDISPATCH_LOOP_PLAN_BLOCKER_FIX_REVIEW.md)
   accepts both corrections. The private local SQLite bounded redispatch loop
   is now implemented in the [implementation
   report](docs/concepts/BOUNDED_TRUSTED_HOST_REDISPATCH_LOOP_IMPLEMENTATION_REPORT.md).
   It composes the accepted one-shot supervisor in one private helper, derives
   every continuation posture freshly, consumes a distinct directive and
   atomic dispatch reservation for each resumed attempt, reuses the exact
   immutable invocation, and cannot return normally while Core still derives
   `ResumeNow`. Exhausted attempt authority fails closed rather than becoming
   completion, wait, or approval. The focused maintainer/security
   [implementation review](docs/concepts/BOUNDED_TRUSTED_HOST_REDISPATCH_LOOP_IMPLEMENTATION_REVIEW.md)
   accepts the private slice with non-blocking composition-level race and
   fault-injection follow-ups. The smallest fresh-process and restart boundary
   is now documented in the [Trusted-Host Operational Entry Boundary
   Plan](docs/implementation-plans/trusted-host-operational-entry-boundary-plan.md)
   and [planning
   report](docs/concepts/TRUSTED_HOST_OPERATIONAL_ENTRY_BOUNDARY_PLAN_REPORT.md).
   It rehydrates authoritative SQLite state, obtains fresh one-use authority
   only from accepted opening or directive-consumption operations, validates
   exact injected invocation and executor b unpublished `workflow-local-host` binary, permits only a
   Core-proven complete empty current-authority inventory for an exact
   obligation-free docs-check contract, and defines typed prepared, waiting,
   terminal, blocked, denied, and failure outcomes. The focused blocker-fix
   review is next. Rust implementation remains blocked until it accepts these
   boundaries. The focused [blocker-fix
   review](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_PLAN_BLOCKER_FIX_REVIEW.md)
   accepts the corrected boundary with one mandatory implementation condition:
   the closed docs-check profile must explicitly declare zero current-authority
   facts rather than relying on absent fields. The next phase is the complete
   unpublished binary-to-Core-admission-to-local-host-operation Rust vertical
   slice. Implementation inspection found a canonical-model blocker before any
   runtime wiring: `RequiredContextContractBinding` rejects an empty requirement
   set, and required-context consumption rejects an empty projection set. Core
   therefore cannot currently represent or consume the exact explicit
   zero-obligation contract required by the accepted admission review. The
   [implementation blocker report](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_IMPLEMENTATION_BLOCKER_REPORT.md)
   records why fabricating an optional requirement, treating absence as proof,
   or bypassing registered current authority would violate the accepted
   boundary. The next phase is a narrowly scoped explicit zero-required-context
   contract model and consumption-semantics plan. That [plan](docs/implementation-plans/explicit-zero-required-context-contract-plan.md)
   and its [planning report](docs/concepts/EXPLICIT_ZERO_REQUIRED_CONTEXT_CONTRACT_PLAN_REPORT.md)
   now define an explicit hash-bound posture, compatibility-conscious serde,
   zero-projection consumption, private closed docs-check issuance, and exact
   empty-inventory resolution. The focused [maintainer/security
   review](docs/concepts/EXPLICIT_ZERO_REQUIRED_CONTEXT_CONTRACT_PLAN_REVIEW.md)
   accepts the plan with one mandatory compatibility condition: existing
   non-empty contract hashes and serialized output must remain byte-for-byte
   unchanged, while explicit zero uses a separate marker and hash domain. The
   prerequisite-only implementation is now complete. It preserves the legacy
   non-empty hash and wire shape, adds a private canonical docs-check issuer,
   accepts exactly zero projections only for explicit-zero contracts, and lets
   the private registered authority source commit and resolve complete empty
   grant, availability, and context-reference inventories for the exact
   execution binding. The [implementation
   report](docs/concepts/EXPLICIT_ZERO_REQUIRED_CONTEXT_CONTRACT_IMPLEMENTATION_REPORT.md)
   records the boundary and focused tests. The focused [implementation
   review](docs/concepts/EXPLICIT_ZERO_REQUIRED_CONTEXT_CONTRACT_IMPLEMENTATION_REVIEW.md)
   accepts the prerequisite with non-blocking follow-ups. The trusted-host
   local application admission vertical slice is now implemented. One
   unpublished `workflow-local-host` foreground binary opens a single SQLite
   backend, supplies explicit selection and lifecycle input, and asks Core to
   admit the exact one-step `dg/trusted-host-docs-check` operation. Core owns
   project validation, immutable run-bundle creation or exact rehydration,
   policy and approval posture, exact step/profile/actor binding, explicit-zero
   current-authority proof, executor commitment, and opaque one-shot session
   preparation. The local-host package consumes the pair synchronously and
   emits only bounded posture strings. SQLite schema v7 adds atomic create-only
   immutable-run-bundle storage so the admitted run does not split truth across
   backends. Production-shaped tests invoke the actual unpublished binary,
   prove durable bundle/event state and idempotent replay, and use no provider,
   network, mock handler, released CLI, or filesystem state bridge. An
   operational terminal outcome deliberately leaves the workflow run
   `Running`; application return does not fabricate workflow completion. See
   the [implementation
   report](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_IMPLEMENTATION_REPORT.md).
   The focused [implementation/security
   review](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_IMPLEMENTATION_REVIEW.md)
   accepts the unpublished docs-check slice with non-blocking follow-ups.
   Bounded application-boundary negative-path proof hardening is now
   implemented for the existing slice. The real unpublished foreground binary
   now proves approval wait, unsupported-topology rejection, actor and
   correlation replay rejection, resolved-executable substitution rejection,
   incompatible SQLite rejection, bounded non-leaking failures, and unchanged
   exact replay. The focused security review found that the first resolved
   process commitment used lossy path text and did not bind same-path
   executable replacement. The governed blocker fix now commits exact encoded
   path bytes plus a streaming executable content identity and revalidates that
   identity before request construction. The focused blocker-fix review accepts
   the substantive identity binding and identified one missing regression. The
   governed follow-up now proves that executable identity becoming unavailable
   after resolution fails before runner invocation and without leakage. The
   blocker is closed. Policy-denial and pre-entry cancellation remain
   lower-level proofs because the foreground binary exposes no injection seam
   for either posture. See the [hardening
   report](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_NEGATIVE_PATH_HARDENING_REPORT.md).
   Broader profiles and runtime surfaces remain separately scoped rather than
   authorized by this acceptance. See the [review](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_NEGATIVE_PATH_HARDENING_REVIEW.md),
   [blocker-fix report](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_NEGATIVE_PATH_HARDENING_BLOCKER_FIX_REPORT.md),
   and [blocker-fix review](docs/concepts/TRUSTED_HOST_LOCAL_APPLICATION_ADMISSION_NEGATIVE_PATH_HARDENING_BLOCKER_FIX_REVIEW.md).
   The next bounded phase is now specified in the [Trusted-Host Workflow
   Completion Reconciliation
   Plan](docs/implementation-plans/trusted-host-workflow-completion-reconciliation-plan.md).
   It keeps the foreground host non-authoritative and plans one private
   Core-owned atomic reconciliation from the exact terminal continuity
   attempt to the matching invocation-result and workflow-terminal events for
   the existing one-step docs-check profile. The plan preserves the
   uncollapsed in-memory `SkillOutput` until Core has freshly verified the
   run, immutable bundle, cursor, window, attempt, invocation, and executor.
   Retryable, yielded, ambiguous, stale, mismatched, or incomplete posture
   remains non-terminal. Focused maintainer/security review found two planning
   blockers: the proposed post-outcome transaction could not recover a
   successful in-memory result after the continuity outcome committed, and the
   exact invocation lifecycle identity and event ordering were unspecified.
   The documentation-only blocker fix now assigns invocation request to the
   opening transaction, invocation start to dispatch, and invocation plus run
   terminal events to the attempt-outcome transaction. Core owns invocation
   identity, derives attempt identity from the exact continuity attempt, and
   requires complete-sequence replay. Focused re-review accepts the corrected
   plan with one mandatory implementation condition: required-output
   validation must select success or fixed terminal failure before any
   terminal write set is applied. The bounded private Core and SQLite
   implementation is now complete for the existing one-step docs-check
   profile. Opening atomically appends the invocation request before the
   operational opening event; dispatch atomically appends admission before
   invocation start; and a terminal outcome atomically appends the continuity
   projection, invocation terminal event, and matching run terminal event.
   Required-output validation selects success or fixed terminal failure before
   any terminal write set is applied. Exact terminal replay adds no events,
   while changed actor, correlation, or resolved executable identity fails
   closed before replay is accepted. The foreground host remains
   non-authoritative and receives only bounded posture output. Focused
   maintainer/security review found and fixed two terminal-transaction
   blockers before accepting the slice. Current-authority security rejection
   now projects no invocation or workflow terminal event, and terminal replay
   commitment now binds the complete lifecycle context plus the exact bounded
   terminal outcome. The accepted implementation remains limited to the
   private one-step docs-check profile. See the [planning
   report](docs/concepts/TRUSTED_HOST_WORKFLOW_COMPLETION_RECONCILIATION_PLAN_REPORT.md),
   [plan review](docs/concepts/TRUSTED_HOST_WORKFLOW_COMPLETION_RECONCILIATION_PLAN_REVIEW.md),
   [blocker-fix report](docs/concepts/TRUSTED_HOST_WORKFLOW_COMPLETION_RECONCILIATION_PLAN_BLOCKER_FIX_REPORT.md),
   [blocker-fix review](docs/concepts/TRUSTED_HOST_WORKFLOW_COMPLETION_RECONCILIATION_PLAN_BLOCKER_FIX_REVIEW.md),
   [implementation report](docs/concepts/TRUSTED_HOST_WORKFLOW_COMPLETION_RECONCILIATION_IMPLEMENTATION_REPORT.md),
   and [implementation review](docs/concepts/TRUSTED_HOST_WORKFLOW_COMPLETION_RECONCILIATION_IMPLEMENTATION_REVIEW.md).
   General scheduling, public configuration, provider
   mutation, OpenShell, nested harnesses, automatic approval, CLI, SDK, and
   schema exposure remain blocked.
2. **Bounded second provider mutation vertical slice.** The integrated Core
   helper for draft GitHub pull request creation from an already-pushed branch
   is implemented and accepted. The slice separates Git transport
   from provider metadata creation, treats head/base SHAs as governed
   observations of mutable refs rather than an atomic freeze, and requires
   fresh proportional-governance assessment, current scoped authority,
   proof-enforced approval,
   durable SideEffect lifecycle, lookup-first idempotency, reconciliation,
   evidence, and report-ready citations. It exposes an injected provider call
   boundary. The follow-on GitHub HTTP transport is now implemented as an
   explicit compile-time sandbox-repository path; no automatic executor path,
   CLI behavior, or broader write family was added. That transport boundary is
   defined in [GitHub Draft Pull Request HTTP Sandbox Transport
   Plan](docs/implementation-plans/github-draft-pull-request-http-sandbox-transport-plan.md)
   and its [planning
   report](docs/concepts/GITHUB_DRAFT_PULL_REQUEST_HTTP_SANDBOX_TRANSPORT_PLAN_REPORT.md).
   The focused maintainer/security review is complete and accepts the plan
   after fix-forward hardening in [GitHub Draft Pull Request HTTP Sandbox
   Transport Plan
   Review](docs/concepts/GITHUB_DRAFT_PULL_REQUEST_HTTP_SANDBOX_TRANSPORT_PLAN_REVIEW.md).
   The concrete provider mapping, private HTTP transport, scripted proof, and
   ignored allowlisted full-helper smoke are implemented in [GitHub Draft Pull
   Request HTTP Sandbox Transport
   Report](docs/concepts/GITHUB_DRAFT_PULL_REQUEST_HTTP_SANDBOX_TRANSPORT_REPORT.md).
   The focused implementation and security review accepts the bounded transport
   with non-blocking follow-ups in [GitHub Draft Pull Request HTTP Sandbox
   Transport Review](docs/concepts/GITHUB_DRAFT_PULL_REQUEST_HTTP_SANDBOX_TRANSPORT_REVIEW.md).
   The next proof is dedicated provisioning and execution of the ignored live
   sandbox smoke; it must not broaden targets or mutation families. The first
   governed execution attempt failed closed before any provider call with
   `github_draft_pull_request.approval.proof_marker_mismatch`. The ignored
   smoke built its durable approval proof from the ordinary content fixture
   while presenting distinct live-sandbox content. GitHub retained no open
   pull request. The focused test-fixtuT_CONSUMER_CLI_ADOPTION_PLAN_REPORT.md).
   The inventory found one required Core prerequisite: the selected fresh-run
   route must gain a report-composition adapter that reuses the same route and
   canonical check result before CLI cutover. The existing manifest-controlled
   `run` and `approve` command surface, output, activation, two-gate approval
   behavior, durable ordering, and artifact obligation remain the compatibility
   contract. The
   [focused plan review](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_CONSUMER_CLI_ADOPTION_PLAN_REVIEW.md)
   accepts the plan after requiring exact check-reference and approval-gate-kind
   projection, workflow-derived artifact and proof-marker gate parity, and
   truthful denial artifacts without a decision-time check rerun. The next
   implementation is the selected fresh-run report adapter only. A separate
   selected approval adoption envelope must be implemented and reviewed before
   CLI cutover. The selected fresh-run report adapter is now implemented in the
   [implementation report](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_PROJECT_VALIDATION_REPORT_ADAPTER_REPORT.md).
   It composes terminal WorkReports from the exact same-call check reference,
   defers report generation while approval is pending, and does not rerun
   workflow skills. Its
   [focused review](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_PROJECT_VALIDATION_REPORT_ADAPTER_REVIEW.md)
   found one blocker in existing-terminal reassessment: the adapter reruns the
   canonical check while replaying the original runtime-fact snapshot
   evaluation time as the new observation time. The blocker fix must select a
   fresh Core-owned reassessment time, compare stable semantic commitments to
   the durable original binding, preserve the original run binding, and fail
   closed on drift. That fix is now implemented in the
   [blocker-fix report](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_PROJECT_VALIDATION_REPORT_ADAPTER_BLOCKER_FIX_REPORT.md):
   terminal reassessment uses a fresh Core-owned time, validates source/fact/
   assessment commitments semantically, returns only transient reassessment
   time alongside the original durable binding, and fails closed without event
   or skill replay when current facts drift. The selected approval adoption
   envelope and CLI cutover remain blocked pending focused blocker-fix review.
   The focused
   [blocker-fix review](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_PROJECT_VALIDATION_REPORT_ADAPTER_BLOCKER_FIX_REVIEW.md)
   now accepts the provenance fix with no remaining blocker. The selected
   approval adoption envelope is now implemented in the
   [implementation report](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_APPROVAL_ADOPTION_ENVELOPE_REPORT.md).
   It derives approval-gate kind from durable state, reruns the canonical check
   only for grants, cites the exact Core-produced check result, keeps
   non-terminal aggregate receipts transient, produces check-free truthful
   denial artifacts, and applies workflow-derived artifact gates at terminal
   closure. The focused
   [maintainer review](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_APPROVAL_ADOPTION_ENVELOPE_REVIEW.md)
   accepts the envelope with no blocker. Combined selected CLI `run` and
   `approve` adoption is now implemented in the
   [implementation report](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_CONSUMER_CLI_ADOPTION_REPORT.md).
   The declared run path uses the selected source-backed report adapter, and
   both aggregate and authored approval gates use one Core approval envelope.
   Existing commands, activation, output, event ordering, retry posture,
   ordinary workflow behavior, and artifact obligations remain the public
   compatibility boundary. The
   [focused maintainer review](docs/concepts/PROPORTIONAL_GOVERNANCE_SELECTED_LOCAL_PROJECT_CONSUMER_CLI_ADOPTION_REVIEW.md)
   accepts the cutover with no blocker and only test-depth follow-ups. No
   provider execution, write capability, hosted behavior, or generic runtime
   default is enabled. The post-cutover
   [Legacy Core API Compatibility Review](docs/concepts/PROPORTIONAL_GOVERNANCE_LEGACY_CORE_API_COMPATIBILITY_REVIEW.md)
   found that the older public explicit-fact route family accepted
   caller-authored authority and other route-driving facts without requiring
   the selected validated project activation. The bounded
   [Legacy Core API Retirement Report](docs/concepts/PROPORTIONAL_GOVERNANCE_LEGACY_CORE_API_RETIREMENT_REPORT.md)
   removes that caller-fact route, report, and approval family from the public
   preview surface. Core retains the common private enforcement machinery;
   Core-owned and selected project-validation consumers remain public and
   covered. Ordinary undeclared execution and CLI behavior are unchanged.
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
   when work does not pause. The first selected-path durable proof is now
   implemented in the
   [Visible-Disclosure Surface-Acceptance Proof Report](docs/concepts/PROPORTIONAL_GOVERNANCE_VISIBLE_DISCLOSURE_SURFACE_ACCEPTANCE_PROOF_REPORT.md):
   one payload-free event records that the injected local surface accepted the
   exact request before validation and skill execution, generic audit remains
   bounded, terminal reports cite the durable event, and CLI success is emitted
   only after persistence. It explicitly does not claim external delivery,
   human observation, acknowledgement, or exactly-once surface invocation. The
   focused review accepts the phase with no blocker. The lane must still prove
   broader onboarding, invalidation, and presentation boundaries before broadening
   proportional-governance defaults or treating additional provider mutations
   such as PR creation or Jira issue creation as automatic extensions of the
   GitHub comment sandbox.
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
   [Capability Grant And Availability Core Modeduplicate, mismatched, or unknown-step sources with
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
   [Authestation solution and live
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
| Evidence, reports, approvals, and policy gates | Implemented selected runtime paths | Authoritative local project validation can route, approve or deny, reassess, and persist a gated WorkReport artifact; broader profiles remain bounded |
| Existing-repository onboarding | Implemented preview | New default scaffolds activate the closed project-validation profile; existing repositories are not migrated automatically and workflow recommendations remain review-only |
| SideEffect governance | Implemented foundations | Lifecycle, persistence, discovery, approval linkage, and artifact gates exist |
| Scoped authority and capability projection | Implemented selected foundations and private consumers | Grants, availability, resolution, request review, step/context projection, same-call current-authority resolution, and selected receipt production exist; no general public authority token or broad executor default exists |
| First provider-write sandbox | Implemented and accepted | GitHub PR comments only, explicit live-sandbox path, no default writes or second mutation family |
| Broader write-capable adapters | Not started | Requires acceptance of the first complete provider-write proof |
| Operational embedded durable state | Implemented local opt-in vertical slice | Guarded atomic filesystem-to-SQLite staging import, canonical/projection verification, exact-receipt activation, retained source, and bounded CLI exist; automatic selection, source cleanup, shared state, and production-readiness claims do not |
| Shared PostgreSQL state | Accepted | [Plan](docs/implementation-plans/shared-postgresql-state-plan.md), [report](docs/concepts/SHARED_POSTGRESQL_STATE_REPORT.md), and [review](docs/concepts/SHARED_POSTGRESQL_STATE_REVIEW.md) cover the explicit adapter, transaction families, revisions, fenced leases, shared consumer, projection rebuild, concurrent CI conformance, and recovery rehearsal; hosted operation, automatic selection, production TLS/pooling/HA, and production-readiness claims remain excluded |
| Single-tenant hosted alpha | Implemented and reviewed for one no-write evaluation trust domain | [Plan](docs/implementation-plans/single-tenant-hosted-alpha-plan.md), foundation [report](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_REVIEW.md), runtime-composition [report](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_RUNTIME_COMPOSITION_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_ALPHA_RUNTIME_COMPOSITION_REVIEW.md), dispatch/result [report](docs/concepts/SINGLE_TENANT_HOSTED_DISPATCH_RESULT_PROJECTION_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_DISPATCH_RESULT_PROJECTION_REVIEW.md), provider-outcome [report](docs/concepts/SINGLE_TENANT_HOSTED_PROVIDER_OUTCOME_PROJECTION_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_PROVIDER_OUTCOME_PROJECTION_REVIEW.md), deployment/recovery [report](docs/concepts/SINGLE_TENANT_HOSTED_DEPLOYMENT_RECOVERY_PROOF_REPORT.md) and [review](docs/concepts/SINGLE_TENANT_HOSTED_DEPLOYMENT_RECOVERY_PROOF_REVIEW.md), [runtime guide](docs/runtime/single-tenant-hosted-alpha.md), and focused [threat model](docs/security/single-tenant-hosted-alpha-threat-model.md) cover one authenticated trust domain, shared state, proof-enforced run/approval/cancellation paths, durable attempts, stateless fenced workers, an explicit no-write execution provider, Core-owned atomic dispatch and terminal report projection, failure/reconciliation projection, live PostgreSQL recovery proof, and a deployed API/worker restart rehearsal; no multi-tenancy, enterprise identity, access-material resolver, OpenShell integration, broader writes, HA/PITR, or production-readiness claim |
| Collaborative workflow/catalog state | Implemented locally; live CI acceptance pending | One organization, fixed registered projects, pre-provisioned principals, exact PostgreSQL resource bindings, immutable project catalog versions, bounded audit decisions, and separate alpha compatibility exist; no hostile-tenant, enterprise identity, notification, or hosted-SaaS claim |
| Composable Harness Contracts | Future | Model and runtime work follows stable governance and typed handoffs |
| Reasoning Lineage / Claim Graph | Future | Must not interrupt provider-write correctness or preview readiness |
| Hosted/distributed production backend | Future | Deferred until local contracts and operational boundaries stabilize |

Recent external dogfood feedback is reconciled against the current repository
in [External Dogfood Feedback Reconciliation](docs/concepts/EXTERNAL_DOGFOOD_FEEDBACK_RECONCILIATION.md).
The review confirms that agent-instruction preservation, safe metadata-aware
onboarding, concise/verbose first-run output, mock-demo separation, independent
proportional-governance decision axes, deterministic workload derivation, input
fingerprint invalidation, and immutable run binding are already implemented for
their accepted boundaries. Exact retry and approval-resume reassessment,
one canonical authoritative local-check consumer, all four source-bound
proportional-governance routes, authoritative terminal WorkReport artifact
persistence, and selected actor-bound time-of-use authority are now implemented
and reviewed. These remain explicit selected paths, not broad defaults or
enterprise RBAC. The next load-bearing boundary is the narrow local-host caller
composition authorized by the accepted trusted-host preparation review,
followed by carefully reviewed expansion of proof-bearing consumers and
integrity-safe report/export behavior. No new provider mutation family should
precede these concrete authority and proof boundaries.

## Current Product Boundary

Workflow OS currently does not provide default provider mutations, broad
write-capable adapters, hidden credential loading, s. Every important scaffolded field should have an explicit posture: enforced, validated, disclosed, advisory, or deferred. The first implementation extends `workflow-os first-run` with a bounded governance field posture summary for ownership, escalation, profile, approvals, policy, evidence/checks, audit/observability, side-effect/capability posture, and advisory/deferred fields; it is documented in [First-Run Governance Field Posture Report](docs/concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REPORT.md) and accepted with non-blocking follow-ups in [First-Run Governance Field Posture Review](docs/concepts/FIRST_RUN_GOVERNANCE_FIELD_POSTURE_REVIEW.md). The second implementation adds a deterministic warning-only ownership/escalation check to `workflow-os first-run`, documented in [Ownership And Escalation Check Report](docs/concepts/OWNERSHIP_ESCALATION_CHECK_REPORT.md) and accepted with non-blocking follow-ups in [Ownership And Escalation Check Review](docs/concepts/OWNERSHIP_ESCALATION_CHECK_REVIEW.md). The third implementation adds a warning-only first-run spec-field coverage check, documented in [Spec Field Coverage Check Report](docs/concepts/SPEC_FIELD_COVERAGE_CHECK_REPORT.md) and accepted with non-blocking follow-ups in [Spec Field Coverage Check Review](docs/concepts/SPEC_FIELD_COVERAGE_CHECK_REVIEW.md). Workflow discovery field coverage integration is implemented as bounded first-run recommendation output in [Workflow Discovery Field Coverage Integration Report](docs/concepts/WORKFLOW_DISCOVERY_FIELD_COVERAGE_INTEGRATION_REPORT.md), following [Workflow Discovery Field Coverage Integration Plan](docs/implementation-plans/workflow-discovery-field-coverage-integration-plan.md): recommendations now cite existing posture, ownership/escalation, and spec-field coverage codes while remaining review-only. This lane does not authorize schema changes, automatic command execution, automatic workflow generation, catalog storage, RBAC, escalation notifications, provider writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

P0 policy effect enforcement is implemented in [Policy Effect Enforcement P0 Report](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_REPORT.md), following [Policy Effect Enforcement P0 Plan](docs/implementation-plans/policy-effect-enforcement-p0-plan.md), and reviewed in [Policy Effect Enforcement P0 Review](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_REVIEW.md). The core requirement is: declared policy effects must be enforced or rejected. Policy files must not become decorative governance strings. The first implementation adds a small typed v0 policy-effect vocabulary, rejects unsupported or misplaced effects fail-closed during validation, rejects unsupported policy-rule actor bindings, and feeds supported effects into the local executor/conservative policy boundary for approval, retry, escalation, and supported read-only adapter access. The review identified a blocker: standalone `max_attempts=N` validated as retry policy vocabulary but did not enable runtime retry. That blocker is fixed in [Policy Effect Enforcement P0 Blocker Fix Report](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_BLOCKER_FIX_REPORT.md) and accepted in [Policy Effect Enforcement P0 Blocker Fix Review](docs/concepts/POLICY_EFFECT_ENFORCEMENT_P0_BLOCKER_FIX_REVIEW.md): standalone `max_attempts=N` now enables bounded retry and is covered by model and executor regression tests. This does not authorize a broad policy DSL, RBAC/IdP, hosted policy service, write-capable adapters, side-effect execution, schemas, recursive agents, agent swarms, or Level 3/4 autonomy.

High-assurance approval controls are planned in [High-Assurance Approval Controls Plan](docs/implementation-plans/high-assurance-approval-controls-plan.md), and the domain-neutral core model is implemented in [High-Assurance Approval Control Core Model Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_REPORT.md). The core model review identified a nested required-reference deserialization blocker, fixed in [High-Assurance Approval Control Core Model Blocker Fix Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REPORT.md) and accepted in [High-Assurance Approval Control Core Model Blocker Fix Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_CONTROL_CORE_MODEL_BLOCKER_FIX_REVIEW.md). Opt-in runtime enforcement is planned in [High-Assurance Approval Runtime Enforcement Plan](docs/implementation-plans/high-assurance-approval-runtime-enforcement-plan.md), the first pure validation helper is implemented in [High-Assurance Approval Runtime Validation Helper Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_RUNTIME_VALIDATION_HELPER_REPORT.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Runtime Validation Helper Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_RUNTIME_VALIDATION_HELPER_REVIEW.md), and the first opt-in executor-integrated enforcement slice is implemented in [High-Assurance Approval Executor Enforcement Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REPORT.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Executor Enforcement Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_EXECUTOR_ENFORCEMENT_REVIEW.md). WorkReport disclosure planning is documented in [WorkReport High-Assurance Approval Disclosure Plan](docs/implementation-plans/work-report-high-assurance-approval-disclosure-plan.md), the first explicit report-only disclosure slice is implemented in [WorkReport High-Assurance Approval Disclosure Report](docs/concepts/WORK_REPORT_HIGH_ASSURANCE_APPROVAL_DISCLOSURE_REPORT.md), the first pure high-assurance approval disclosure discovery helper is implemented in [High-Assurance Approval Disclosure Discovery Plan](docs/implementation-plans/high-assurance-approval-disclosure-discovery-plan.md) and accepted with non-blocking follow-ups in [High-Assurance Approval Disclosure Discovery Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_DISCLOSURE_DISCOVERY_REVIEW.md), the first explicit in-memory executor/report disclosure bridge is implemented in [High-Assurance Approval Disclosure Executor/Report Integration Plan](docs/implementation-plans/high-assurance-approval-disclosure-executor-report-integration-plan.md), the explicit report artifact high-assurance disclosure gate is implemented in [Report Artifact High-Assurance Approval Disclosure Gate Plan](docs/implementation-plans/report-artifact-high-assurance-disclosure-gate-plan.md) and accepted with non-blocking follow-ups in [Report Artifact High-Assurance Disclosure Gate Review](docs/concepts/REPORT_ARTIFACT_HIGH_ASSURANCE_DISCLOSURE_GATE_REVIEW.md), and workflow-declared high-assurance artifact requirements are planned in [Workflow-Declared High-Assurance Artifact Requirement Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-plan.md). The first internal model/policy-mapping slice is implemented in [Workflow-Declared High-Assurance Artifact Requirement Model Report](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_MODEL_REPORT.md), and the first workflow schema/parser/SDK/validation slice is implemented in [Workflow-Declared High-Assurance Artifact Requirement Schema Report](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_SCHEMA_REPORT.md), following [Workflow-Declared High-Assurance Artifact Requirement Schema Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-schema-plan.md). The pure workflow-declared artifact gate derivation helper is implemented in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Report](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REPORT.md), following [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-runtime-derivation-plan.md), and accepted in [Workflow-Declared High-Assurance Artifact Requirement Runtime Derivation Review](docs/concepts/WORKFLOW_DECLARED_HIGH_ASSURANCE_ARTIFACT_REQUIREMENT_RUNTIME_DERIVATION_REVIEW.md). Explicit executor artifact-path integration is implemented in [Workflow-Declared High-Assurance Artifact Requirement Executor Integration Plan](docs/implementation-plans/workflow-declared-high-assurance-artifact-requirement-executor-integration-plan.md). High-assurance approval-resume artifact/projection composition is implemented in [High-Assurance Approval-Resume Artifact Projection Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_REPORT.md), following [High-Assurance Approval-Resume Artifact Projection Plan](docs/implementation-plans/high-assurance-approval-resume-artifact-projection-plan.md), accepted with non-blocking follow-ups in [High-Assurance Approval-Resume Artifact Projection Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_REVIEW.md), hardened in [High-Assurance Approval-Resume Artifact Projection Hardening Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_HARDENING_REPORT.md), and accepted with non-blocking follow-ups in [High-Assurance Approval-Resume Artifact Projection Hardening Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_RESUME_ARTIFACT_PROJECTION_HARDENING_REVIEW.md). This is a safety-sensitive prerequisite before write-capable adapter work: sensitive or irreversible actions need explicit requester/approver posture, evidence and policy context, expiration/revocation semantics, auditability, and WorkReport disclosure. The current implementation provides model vocabulary, an explicit in-memory validation helper, an opt-in executor approval decision method, explicit report input/disclosure propagation, a pure disclosure derivation helper, an additive approval decision-with-disclosure executor API, an explicit opt-in report artifact disclosure gate, an internal artifact requirement-to-gate policy mapping model, a workflow schema field, a pure workflow-to-artifact-gate derivation helper, an explicit artifact-capable executor path that derives workflow-declared gate policy and composes it with caller policy by strictness, and an explicit high-assurance approval-resume artifact/projection helper that requires durable approval-presentation proof plus high-assurance validation before projection-backed artifact writing. The hardening follow-up now derives effective artifact policy before approval mutation and covers exact-helper denial, projection failure, same-actor rejection, and disclosure-conflict fail-closed behavior. Enforcement postures remain semantically rejected outside the explicit artifact-capable executor path; this does not implement automatic high-assurance approval enforcement, automatic report generation, automatic artifact writing from default executor paths, write-capable adapters, RBAC/IdP, quorum approval, hosted behavior, side-effect execution, CLI behavior, examples, or Level 3/4 autonomy.


Governance strictness profiles and enterprise stewardship are planned in [Governance Strictness Profiles And Stewardship Plan](docs/implementation-plans/governance-strictness-profiles-and-stewardship-plan.md). The first local disclosure model is implemented in [Governance Strictness Profile Disclosure Model Report](docs/concepts/GOVERNANCE_STRICTNESS_PROFILE_DISCLOSURE_MODEL_REPORT.md): `workflow-os first-run` now uses typed core vocabulary to disclose the current `observe_and_report` posture instead of raw strings. This is an important separation point: a single local user may choose non-blocking observe/report-only governance so the agent can execute quickly while Workflow OS standardizes evidence, skipped-check disclosure, side-effect disclosure, audit posture, and reports. Enterprise deployments need a steward/admin layer that decides which profiles are allowed, which gates require humans, which gates may be satisfied by agent-provided evidence, who owns workflows, who can approve workflow changes, and when escalation is required. The local preview does not implement admin controls, RBAC, IdP integration, hosted policy enforcement, enterprise notification/escalation, write-capable adapters, or Level 3/4 autonomy.

Cross-project architng CI.
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

BeforeSkillInvocation status and failure semantics planning is documented in [BeforeSkillInvocation Hook Status And Failure Semantics Plan](docs/implementation-plans/before-skill-hook-status-failure-semantics-plan.md), boundary hardening tests cover later-step targeting, missing handlers, policy denial, and redaction behavior, the first explicit failed-closed result path is implemented as documented in [BeforeSkillInvocation Failed-Closed Result Path Plan](docs/implementation-plans/before-skill-hook-failed-closed-result-plan.md), warning/skipped disclosure semantics planning is documented in [BeforeSkillInvocation Warning And Skipped Disclosure Semantics Plan](docs/implementation-plans/before-skill-hook-warning-skipped-disclosure-plan.md), unsupported-status hardening tests are implemented in [BeforeSkillInvocation Unsupported Status Hardening Report](docs/concepts/BEFORE_SKILL_HOOK_UNSUPPORTED_STATUS_HARDENING_REPORT.md), required pre-skill checkpoint planning is documented in [BeforeSkillInvocation Required Checkpoint Plan](docs/implementation-plans/before-skill-required-checkpoint-plan.md), the first explicit selected-step required enforcement slice is implemented in [BeforeSkillInvocation Required Checkpoint Enforcement Report](docs/concepts/BEFORE_SKILL_REQUIRED_CHECKPOINT_ENFORCEMENT_REPORT.md), anpts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REPORT.md), following [Report Artifact SideEffect Referential Integrity Plan](docs/implementation-plans/report-artifact-side-effect-referential-integrity-plan.md), and accepted in [Report Artifact SideEffect Referential Integrity Review](docs/concepts/REPORT_ARTIFACT_SIDE_EFFECT_REFERENTIAL_INTEGRITY_REVIEW.md). Approval-side-effect linkage planning is documented in [Approval SideEffect Linkage Plan](docs/implementation-plans/approval-side-effect-linkage-plan.md), and the validation-only helper is implemented in [SideEffect Approval Linkage Report](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REPORT.md) and accepted with non-blocking follow-ups in [SideEffect Approval Linkage Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_REVIEW.md). Approval-side-effect linkage composition planning is documented in [Approval SideEffect Linkage Composition Plan](docs/implementation-plans/approval-side-effect-linkage-composition-plan.md). The explicit store-backed approval linkage helper is accepted in [SideEffect Approval Linkage Store-Backed Review](docs/concepts/SIDE_EFFECT_APPROVAL_LINKAGE_STORE_BACKED_REVIEW.md), and explicit executor report artifact writing with SideEffect integrity and approval-linkage gates is implemented in [Executor Report Artifact SideEffect Gates Report](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REPORT.md) and accepted in [Executor Report Artifact SideEffect Gates Review](docs/concepts/EXECUTOR_REPORT_ARTIFACT_SIDE_EFFECT_GATES_REVIEW.md). Provider-write completed/failed workflow event append is implemented for eligible reconciled GitHub PR comment provider outcomes in [GitHub PR Comment Provider Write Event Append Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REPORT.md) and accepted in [GitHub PR Comment Provider Write Event Append Helper Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_WRITE_EVENT_APPEND_HELPER_REVIEW.md). Reconciliation-aware report/artifact disclosure is planned in [GitHub PR Comment Provider Reconciliation Report Artifact Disclosure Plan](docs/implementation-plans/github-pr-comment-provider-reconciliation-report-artifact-disclosure-plan.md), accepted in [GitHub PR Comment Provider Reconciliation Report Artifact Disclosure Plan Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_RECONCILIATION_REPORT_ARTIFACT_DISCLOSURE_PLAN_REVIEW.md), and the first bounded projection helper is implemented in [GitHub PR Comment Provider Reconciliation Disclosure Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_RECONCILIATION_DISCLOSURE_HELPER_REPORT.md). Provider lookup/query reconciliation is implemented in [GitHub PR Comment Provider Lookup Reconciliation Model Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REPORT.md) and accepted in [GitHub PR Comment Provider Lookup Reconciliation Model Review](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECONCILIATION_MODEL_REVIEW.md). Concrete injected lookup HTTP client planning is documented in [GitHub PR Comment Provider Lookup HTTP Client Plan](docs/implementation-plans/github-pr-comment-provider-lookup-http-client-plan.md), the first explicit injected-transport lookup HTTP client is implemented in [GitHub PR Comment Provider Lookup HTTP Client Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_HTTP_CLIENT_REPORT.md), and the first explicit in-memory lookup recovery integration helper is implemented in [GitHub PR Comment Provider Lookup Recovery Integration Helper Report](docs/concepts/GITHUB_PR_COMMENT_PROVIDER_LOOKUP_RECOVERY_INTEGRATION_HELPER_REPORT.md). This does not implement default writes, hidden auth loading, automatic retries, broad provider mutations, automatic provider lookup, schemas, CLI behavior, examples, hosted behavior, runtime side-effect execution beyond explicit reviewed helper paths, EvidenceReference side-effect attachment, automatic executor report discovery, automatic artifact writes from existing executor paths, automatic approval-side-effect validation in existing report/artifact paths, or release posture changes.

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

P0 hardening gap: [Approval Gate Presentation Enforcement Gap](docs/concepts/APPROVAL_GATE_PRESENTATION_ENFORCEMENT_GAP.md) tracks the remaining proof problem. Planning is documented in [Approval Gate Presentation Enforcement Plan](docs/implementation-plans/approval-gate-presentation-enforcement-plan.md), the first model/helper slice is implemented in [Approval Gate Presentation Core Model Report](docs/concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REPORT.md), and the model review is documented in [Approval Gate Presentation Core Model Review](docs/concepts/APPROVAL_GATE_PRESENTATION_CORE_MODEL_REVIEW.md). Follow-on persistence and explicit opt-in enforcement planning is documented in [Approval Gate Presentation Persistence And Enforcement Plan](docs/implementation-plans/approval-gate-presentation-persistence-enforcement-plan.md), the local persistence helper is implemented in [Approval Gate Presentation Persistence Report](docs/concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REPORT.md), and the helper review is documented in [Approval Gate Presentation Persistence Review](docs/concepts/APPROVAL_GATE_PRESENTATION_PERSISTENCE_REVIEW.md). The explicit opt-in enforcement path is implemented in [Approval Gate Presentation Opt-In Enforcement Plan](docs/implementation-plans/approval-gate-presentation-opt-in-enforcement-plan.md), reported in [Approval Gate Presentation Opt-In Enforcement Implementation Report](docs/concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_IMPLEMENTATION_REPORT.md), and accepted in [Approval Gate Presentation Opt-In Enforcement Review](docs/concepts/APPROVAL_GATE_PRESENTATION_OPT_IN_ENFORCEMENT_REVIEW.md): explicit executor callers can now require matching durable presentation proof, optional freshness checks, and fail-closed proof validation before approval events are appended. Dogfood runner proof persistence planning is documented in [Dogfood Runner Approval-Presentation Persistence Plan](docs/implementation-plans/dogfood-runner-approval-presentation-persistence-plan.md), and the repo-local runner now persists bounded `ApprovalPresentationRecord` proof during material `phase-start` before approval. Default/public enforcement planning is documented in [Approval Gate Presentation Default Enforcement Plan](docs/implementation-plans/approval-gate-presentation-default-enforcement-plan.md), accepted in [Approval Gate Presentation Default Enforcement Plan Review](docs/concepts/APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_PLAN_REVIEW.md), implemented as an explicit policy model/helper in [Approval Gate Presentation Default Enforcement Implementation Report](docs/concepts/APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_IMPLEMENTATION_REPORT.md), accepted in [Approval Gate Presentation Default Enforcement Implementation Review](docs/concepts/APPROVAL_GATE_PRESENTATION_DEFAULT_ENFORCEMENT_IMPLEMENTATION_REVIEW.md), selected high-assurance/write-adjacent adoption is planned in [Approval-Presentation Sensitive Adoption Plan](docs/implementation-plans/approval-presentation-sensitive-adoption-plan.md), and the first selected high-assurance adoption path is implemented in [High-Assurance Approval-Presentation Adoption Report](docs/concepts/HIGH_ASSURANCE_APPROVAL_PRESENTATION_ADOPTION_REPORT.md) and accepted in [High-Assurance Approval-Presentation Adoption Review](docs/concepts/HIGH_ASSURANCE_APPROVAL_PRESENTATION_ADOPTION_REVIEW.md). Provider-write/write-adjacent approval-presentation adoption is planned in [Provider-Write Approval-Presentation Adoption Plan](docs/implementation-plans/provider-write-approval-presentation-adoption-plan.md), and the first explicit GitHub PR comment provider-write proof gate is implemented in [Provider-Write Approval-Presentation Gate Implementation Report](docs/concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_IMPLEMENTATION_REPORT.md), accepted in [Provider-Write Approval-Presentation Gate Review](docs/concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_GATE_REVIEW.md), and edge-hardened in [Provider Write Approval Presentation Edge Hardening Review](docs/concepts/PROVIDER_WRITE_APPROVAL_PRESENTATION_EDGE_HARDENING_REVIEW.md). The kernel can emit the correct approval details, and core can now model, locally persist, review, opt-in enforce, dogfood-persist, explicitly policy-route bounded presentation proof, require proof on selected high-assurance approval decisions, and require write-adjacent presentation proof on the selected explicit provider-write path with a deterministic content hash. Default public approval behavior, UI/cards, write-capable adapter defaults, hidden auth loading, CLI mutation behavior, schemas, examples, hosted behavior, and release posture changes remain unimplemented. Repo-local dogfood approval-presentation proof persistence, proof-enforced approval, and bounded f