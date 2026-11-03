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
were actually available; unavailable, unsupported, denied, failed, and stale
sources therefore remain inspectable without fabricating readiness. Review
corrected one blocker by binding the exact closed collector contract into the
proof. The accepted store-backed, bounded-local-observer, and accepted-intent
source sets remain independently tested and are not yet composed through one
integrated aggregate path, so this is not the operational Phase E exit. The
phase adds no production registry, public preview, runtime consumer, projection
delivery, acknowledgment, authority resolution, consumption, provider action,
host ingestion, or Salsa. See the [Phase E implementation
report](docs/concepts/CONTINUATION_CONTEXT_AGGREGATE_COMPOSITION_REPORT.md) and
[focused review](docs/concepts/CONTINUATION_CONTEXT_AGGREGATE_COMPOSITION_REVIEW.md).

Projection delivery and replacement-executor work remain unauthorized until
the integrated private Phase E conformance composition is implemented and
focused-review accepted.

The integrated private Phase E conformance composition is now implemented and
accepted after focused maintainer/security review. One private source set layers the
accepted store-backed anchor, bounded local-observer anchor, and accepted-intent
anchor; invokes those accepted collectors plus explicit closed provider and
agent-advisory postures; rereads the complete layered anchor; and enters the
accepted aggregate proof only when the pre- and post-collection anchors match.
Focused tests prove stable-state composition and reject cross-source drift. This
remains conformance integration rather than a production registry or runtime
consumer. See the [integrated conformance implementation
report](docs/concepts/CONTINUATION_CONTEXT_INTEGRATED_CONFORMANCE_REPORT.md) and
[focused review](docs/concepts/CONTINUATION_CONTEXT_INTEGRATED_CONFORMANCE_REVIEW.md).
The focused review is an owner-conducted repository review, not independent
external security assurance.

Exact private projection delivery and recipient acknowledgment are now
implemented and accepted after focused maintainer/security review. The private delivery
derives purpose, scope, source actor, collection-request commitment, aggregate
proof commitment, decision commitment and posture, and the redacted
understanding projection from the accepted aggregate; callers cannot resupply
those bindings at delivery time. One exact recipient returns a commitment-bound
receipt that proves only receipt of the exact projection. It explicitly does
not claim comprehension or execution authority. Recipient, delivery,
projection, and receipt-commitment substitution fail closed. The slice adds no
persistence, host integration, compaction signal, runtime consumer, next-action
admission, authority consumption, provider operation, public contract, or
Salsa. See the [implementation
report](docs/concepts/CONTINUATION_CONTEXT_PROJECTION_DELIVERY_REPORT.md) and
[focused review](docs/concepts/CONTINUATION_CONTEXT_PROJECTION_DELIVERY_REVIEW.md).
The focused review is an owner-conducted repository review, not independent
external security assurance.

The local end-to-end replacement-executor conformance proof is implemented,
hardened, and accepted after focused maintainer/security re-review.
Two independently composed
orientations from one exact authoritative aggregate deliver the same canonical
understanding to distinct context-free recipients. The private proof preserves
current accepted intent while excluding superseded intent, rejects one exact-
value lawful-next replay fixture, carries bounded decision gaps to the
recipient, withholds fresh authority resolution while blocked, and invokes a
separate fresh-authority boundary only after exact aggregate and semantic
equivalence. It remains an in-process local conformance proof rather than
evidence of host compaction, model comprehension, process replacement, runtime
admission, or automatic WorkReport production. See the [implementation
report](docs/concepts/CONTINUATION_CONTEXT_REPLACEMENT_EXECUTOR_PROOF_REPORT.md)
and [focused review](docs/concepts/CONTINUATION_CONTEXT_REPLACEMENT_EXECUTOR_PROOF_REVIEW.md).

The three initial review blockers and two hardening re-review findings are
corrected in the private conformance boundary.
Typed pause/stop/resume disposition now survives composition and delivery,
stable work identities carried by exact Core-selected authoritative private
sources replace text equality, and a payload-free fresh-
authority request binds the exact scope, aggregate, decision, recipient, and
operation after proof validation. Existing collectors do not fabricate these
identities. Focused re-review accepts this boundary with no remaining blocker.
No public API, runtime consumer, persistence, provider, schema, hosted, nested-
harness, agent-memory, or Salsa work is authorized. See the [hardening
report](docs/concepts/CONTINUATION_CONTEXT_REPLACEMENT_EXECUTOR_PROOF_HARDENING_REPORT.md)
and [hardening focused
review](docs/concepts/CONTINUATION_CONTEXT_REPLACEMENT_EXECUTOR_PROOF_HARDENING_REVIEW.md).

No phase may claim compaction-safe continuation, consume approval or capability
authority during reconstruction, or expose a runtime consumer before the
preceding phase and focused security review are accepted. The P0 queue-integrity
repair is recorded in [Roadmap Active Queue Authority P0
Report](docs/concepts/ROADMAP_ACTIVE_QUEUE_AUTHORITY_P0_REPORT.md).

The first of the two fresh-rerun P1 defects is implemented and accepted after
focused maintainer review corrected one blocker. Generated downstream agent
instructions are context-aware and reference only repository-local artifacts
created by the selected scaffold entry point; neither path references a guide
available only in the Workflow OS source checkout. The bounded correction is
recorded in the [P1 implementation
report](docs/concepts/GENERATED_AGENT_GUIDANCE_PORTABILITY_P1_REPORT.md) and
[focused review](docs/concepts/GENERATED_AGENT_GUIDANCE_PORTABILITY_P1_REVIEW.md).

The remaining authoritative P1 queue item, provider-fixture-aware `next-action`
routing, is implemented and accepted after focused maintainer review. Public
GitHub fixture runs now route `next-action` through the same fixture SQLite
state and immutable-bundle source used by `status` and `inspect`, while
ordinary local runs preserve their existing backend path. The bounded
correction is recorded in the [P1 implementation
report](docs/concepts/PROVIDER_FIXTURE_NEXT_ACTION_ROUTING_P1_REPORT.md) and
[focused review](docs/concepts/PROVIDER_FIXTURE_NEXT_ACTION_ROUTING_P1_REVIEW.md).
This is an acceptance fix, not a broadening of continuation behavior.

The **capability-gated GitHub read-only public adoption and recovery** lane now
resumes from its accepted public fixture milestone. The existing
crate-private executor composition already proves one exact repository-metadata
read through immutable run identity,
current source-backed authority, durable continuity, one-time continuation,
policy evaluation, bounded telemetry, and a payload-free receipt. It is not yet
a user-selectable runtime path, and its claim-only and invocation-started
ambiguous states do not yet have an accepted recovery contract.

The complete public-adoption and recovery boundary is now documented in the
[Capability-Gated GitHub Read-Only Public Adoption And Recovery
Plan](docs/implementation-plans/capability-gated-github-read-only-public-adoption-recovery-plan.md).
It requires exact immutable operation declaration, an explicit project-level
resource allowlist and access reference, Core-owned run-scoped authority
derivation, migration from the private consumed idempotency claim to accepted
atomic authorized-execution continuity, truthful ambiguity recovery, bounded
disclosure, and ordinary governed `run` semantics. It also records the stale
read-only adapter user-agent identity as a baseline compatibility defect that
must be fixed before public live use.

Focused maintainer/security review is accepted after correcting three blocking
ambiguities: provider activation now requires existing authoritative execution,
transient credential delivery is separate from current-authority facts, and
provider dispatch must reuse the complete operational-window and attempt
lifecycle rather than reservation alone. See the [plan
review](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_PUBLIC_ADOPTION_RECOVERY_PLAN_REVIEW.md).

The first dark implementation slice and focused maintainer/security review are
accepted. Review found and corrected an incomplete contract closure: the exact
operation now requires the complete accepted skill capability and input shape,
not only the adapter tuple. The implementation adds exact operation and
subordinate project activation contracts, immutable binding, schema/SDK parity,
compatibility behavior, and focused validation. It does not expose public
runtime selection, read access material, call GitHub, or change CLI execution.
See the [implementation
report](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_CONTRACT_FOUNDATION_REPORT.md)
and [focused
review](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_CONTRACT_FOUNDATION_REVIEW.md).
Phase 2 Core-private authority derivation, separate transient-access resolution,
and atomic continuity/ambiguity composition is implemented and accepted after
blocker correction. The corrected boundary reloads the exact bundle inside
Core, derives the authored operation, literal target, project allowlist, skill
input, and operation commitment from immutable records, and rejects caller
substitution before lifecycle preparation. Exact SQLite tests prove one-call
success, ambiguity without blind redispatch, terminal replay without provider
re-entry, and access-marker absence from durable bytes. The implementation
remains non-public and no-write. See the [Phase 2 implementation
report](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_CORE_PRIVATE_AUTHORITY_LIFECYCLE_REPORT.md).
Phase 3 public fixture activation is implemented and accepted after focused
maintainer/security review corrected the denial path. The ordinary CLI now selects only the exact
fixture-backed repository metadata operation, binds fixture content into the
approved immutable context, requires durable approval-presentation proof,
persists bounded telemetry, and exposes provider-backed status, inspect,
recovery disclosure, and terminal replay without live transport. See the
[implementation report](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_FIXTURE_ACTIVATION_REPORT.md),
[focused review](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_FIXTURE_ACTIVATION_REVIEW.md),
and the preceding [Phase 2 focused
review](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_CORE_PRIVATE_AUTHORITY_LIFECYCLE_REVIEW.md).
The authoritative post-compaction understanding lane and its two fresh-rerun
P1 acceptance fixes are complete. Core-private same-call connector authority
verification with a fixture connector is implemented and accepted after
focused review correction. The next authorized connector increment is one
end-to-end fixture-backed pull-request lifecycle proof, including replay,
ambiguity, wait recovery, and authoritative source composition for facts that
remain fixture-injected in the accepted admission slice. Connector credentials,
live GitHub behavior, provider writes, scheduler claims, and any parallel
permission system remain excluded.

Before broader provider mutation families, nested harness work, or claims of
continuous delegated-maintainer autonomy, complete the P0 [Delegated Authority
Projection Into External Connectors
Plan](docs/implementation-plans/delegated-authority-external-connector-projection-plan.md).
Focused maintainer/security review accepted the plan after correcting six
security boundaries: immutable projection is separate from mutable transition
state, serialized projections are non-authorizing claims, draft creation
derives one exact PR successor binding, ref updates are distinct from PR
transitions, evidence and payload commitments are exact, and ambiguous provider
outcomes block redispatch. See the [focused
review](docs/concepts/DELEGATED_AUTHORITY_EXTERNAL_CONNECTOR_PROJECTION_PLAN_REVIEW.md).
The plan integrates existing scoped runtime authority, approval-presentation
proof, same-call current-authority resolution, authorized-execution
continuity, provider reconciliation, proportional governance, and durable wait
conditions. The first model-only slice is implemented: immutable
external-action projection, closed PR transition vocabulary, exact
resource/payload/evidence commitments, successor-resource binding,
canonicalization, deterministic commitment, validated deserialization, and
redaction-safe diagnostics. It remains non-authorizing, non-persistent, and
no-write. See the [model
report](docs/concepts/EXTERNAL_ACTION_AUTHORIZATION_PROJECTION_MODEL_REPORT.md).
Focused maintainer/security review accepted the model after correcting exact
approval-scope/non-goal binding, ready-before-merge dependency, GitHub resource
validation, and successor observation timing. See the [focused
review](docs/concepts/EXTERNAL_ACTION_AUTHORIZATION_PROJECTION_MODEL_REVIEW.md).
The durable transition lifecycle and local state composition slice is
implemented. Core now derives one immutable operation per projected transition,
persists a create-only operation definition and commitment-chained event
history, applies optimistic cursor admission, and models reservation,
admission, attempt, observed outcome, ambiguity reconciliation, retry after
known failure, genuine continuity waits, denial, consumption, and revocation.
Exact replay does not duplicate events, ambiguous outcomes block redispatch,
and corrupt or stale local state fails closed. The slice remains local,
payload-free, non-authorizing, and no-write. See the [lifecycle
report](docs/concepts/EXTERNAL_ACTION_TRANSITION_LIFECYCLE_REPORT.md) and
[focused review](docs/concepts/EXTERNAL_ACTION_TRANSITION_LIFECYCLE_REVIEW.md).
Core-private same-call current-authority verification and a fixture connector
are implemented and accepted after focused maintainer/security review corrected
local retry attempt identity, known-failure classification, and fixture-fact
claim clarity. The dark Core
boundary reloads registered current authority, verifies the exact projected
grant, approval-presentation proof, resource, execution window, revocation
source, policy/governance commitments, and fresh evidence before one injected
fixture connector receives a borrowed payload-free use capability. Durable
lifecycle admission prevents terminal or ambiguous replay from re-entering the
connector. The slice has no credentials, network, live GitHub behavior,
provider writes, public connector API, schema, SDK, CLI, scheduler, or broader
transition family. See the [implementation
report](docs/concepts/EXTERNAL_ACTION_CONNECTOR_SAME_CALL_ADMISSION_REPORT.md)
and [focused
review](docs/concepts/EXTERNAL_ACTION_CONNECTOR_SAME_CALL_ADMISSION_REVIEW.md).
The next authorized increment is delivery-sequence Phase 5: one end-to-end
fixture-backed PR lifecycle proof. No live provider integration or broader
mutation family is authorized by this acceptance.

Planning for Phase 5 is now captured in the [External Action Fixture
Pull-Request Lifecycle Proof
Plan](docs/implementation-plans/external-action-fixture-pr-lifecycle-proof-plan.md).
The plan limits the proof to one credential-free, Core-private create-draft,
mark-ready-after-terminal-green, and squash-merge sequence. It requires
authoritative source composition in place of caller-authored current-fact
booleans, exact successor binding, durable evidence and connector-refusal
waits, ambiguous-outcome reconciliation, and no-redispatch replay. Ref updates,
branch deletion, live GitHub access, credentials, network, provider writes,
public connector APIs, schemas, SDKs, CLI mutation commands, schedulers, and
broader mutation families remain excluded. Focused maintainer/security review
accepts the plan after correcting three blockers: provider operations now
require atomic expected-revision comparison, successor bindings are explicitly
Core-owned, and provider-success/local-persistence ambiguity has a deterministic
reconciliation and repair path. The review also clarifies that closed windows
are not resumable waits, connector refusal requires proof that dispatch did not
begin, and failed checks remain typed evidence blockers unless policy makes
them terminal. See the [focused plan
review](docs/concepts/EXTERNAL_ACTION_FIXTURE_PR_LIFECYCLE_PROOF_PLAN_REVIEW.md).

Phase 5A source and fixture contracts are implemented and accepted after
focused maintainer/security review corrected deterministic replay. A closed ten-source coverage
model rejects missing, duplicate, unavailable, stale, denied, failed, or
drifting required inputs. Stable source anchors bind complete coverage to the
provider revision. A revision-fenced fixture-provider contract proves atomic
compare-and-operate, exact operation replay, and concurrent one-winner behavior
without connector dispatch. A Core-owned in-memory successor-binding store
accepts exact replay and rejects conflicting projection bindings. The slice
adds no complete PR lifecycle, live provider behavior, network, credentials,
public API, schema, SDK, CLI mutation, scheduler, broader mutation, or Salsa.
See the [Phase 5A implementation
report](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5A_REPORT.md) and
[focused
review](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5A_REVIEW.md).
Phase 5B fixture-backed draft creation and exact successor binding is now
implemented and accepted after focused maintainer/security review corrected
two blockers. The private entrypoint accepts only stable bindings and source
interfaces, derives complete Phase 5A coverage and admission facts in the same
call, enters the accepted same-call current-authority and durable lifecycle
path, atomically compares provider revision, derives the successor from the
exact provider receipt, persists and rereads it through the Core-owned store,
and repairs provider-success/local-persistence read or write ambiguity without
redispatch. Focused tests prove one create, terminal replay, write and read
persistence repair, provider/local successor conflict rejection, and fixture-
source substitution rejection before provider mutation. See the [Phase 5B
implementation
report](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5B_REPORT.md) and
[focused
review](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5B_REVIEW.md).
Phase 5C evidence wait and exact-successor mark-ready is implemented and
accepted after focused maintainer/security review corrected wait-proof,
successor-replay, dispatch-time-refusal, and resume-drift blockers. The
Core-private fixture now carries the complete create, ready, and later merge
scope before first connector entry; observes exact-head required-check and
pre-dispatch connector posture; binds ready to the persisted successor;
registers pending or failed checks and connector refusal as durable typed
waits; explicitly resolves the same bound wait under fresh source observation;
and resumes only from a typed, rehydration-validated wait-resolution proof
through the same-call authority boundary. Exact ready replay and receipt lookup
bind the immutable provider operation and successor without redispatch.
Dispatch-time pre-mutation check regression or connector refusal becomes a
durable wait, while changed-head or revoked-source re-entry rejects before
connector mutation. See the [Phase 5C implementation
report](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5C_REPORT.md) and
[focused
review](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5C_REVIEW.md).
Phase 5D exact unchanged-head merge, replay, and bounded ambiguity recovery is
implemented and accepted after focused maintainer/security review corrected an
authoritative-negative-lookup blocker. Squash merge requires the exact
Core-owned successor, unchanged reviewed head, ready posture, terminal-green
checks, current authority, and the expected provider revision in one atomic
fixture operation. Success records a payload-free exact merge result and exact
terminal replay never redispatches the mutation. Ambiguous create, ready, and
merge outcomes require typed lookup: exact found reconciles success;
inconclusive or unavailable preserves ambiguity; and authoritative not-found
may become known failure only when coverage binds the exact operation and the
provider remains at the attempted pre-dispatch revision. A lawful retry then
uses a distinct local attempt while retaining stable provider-operation
identity. Substituted results, revision-drifted negative coverage, and
denied/revoked terminal states fail closed. See the [Phase 5D implementation
report](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5D_REPORT.md) and
[focused
review](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5D_REVIEW.md).
Phase 5E end-to-end conformance is implemented and accepted after focused
maintainer/security review. One Core-private entrypoint now admits exactly the
closed create-draft, mark-ready, and squash-merge transition set; resolves a
distinct current-authority, evidence, and evaluation-time context for every
transition; carries payload-free source-anchor commitments; returns durable
typed wait posture for create, ready, and merge without collapsing lawful
re-entry into an error; and
inspects the exact successor and successful lifecycle records on completion.
Focused tests prove complete execution, exact replay without duplicate
mutation, zero-mutation rejection of pre-create resource drift and extra
projected transitions, late authority rejection, wrong-evidence rejection,
typed wait resumption, authority-gated lifecycle writes, and zero-write
rejection when current authority is revoked. See the [Phase 5E implementation
report](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5E_REPORT.md) and
[focused
review](docs/concepts/EXTERNAL_ACTION_FIXTURE_LIFECYCLE_PHASE_5E_REVIEW.md).
Live connector work remains unauthorized; Phase 5E acceptance does not select
or authorize a live provider mutation lane.

After the current delegated-authority connector lane is complete, the next
authoritative milestone is **Hacker News launch admission**. This is a bounded
product-admission lane, not another primitive family and not authorization to
publish. It must compose existing kernel capabilities into one credential-free,
non-mock golden path for a fresh external repository: install a verified release
artifact, preserve existing repository guidance while initializing governance,
produce bounded first-run posture and recommendations, execute one exact reviewed
operation through ordinary governed runtime semantics, select proportional
governance from authoritative facts, persist an evidence-backed WorkReport, and
inspect or replay the durable event trail. Two clean-room repositories must pass
the exact released-artifact path without internal repository knowledge, mock
handlers, hidden manual state repair, or unsupported product claims. Launch
admission also requires a concise default UX, explicit limitations, terminal-green
release validation, and a separately governed publication decision. During this
lane, new primitive families, Salsa, broad provider mutation, OpenShell activation,
nested harness execution, enterprise administration, and general arbitrary-command
execution remain deferred unless a clean-room failure proves one is a correctness
blocker. See the [Hacker News Launch Admission
Plan](docs/implementation-plans/hacker-news-launch-admission-plan.md).
Phase 0 focused maintainer/product/security review is accepted after correcting
one P0 product-loop ambiguity and five P1 truth/UX gaps. The lane selects one
exact non-mock operation:
`workflow-os first-run --write-report-artifact --run-id
launch-first-run-v1`. It must preserve the initial project-validation citation,
expose bounded report and citation posture through `inspect`, recommend the real
artifact path instead of mock execution, treat `next-action` as nonterminal-only,
use fault-injected recovery without manual state repair, and rebuild final native
archives from the exact post-composition commit. Generic handler registration,
parallel runtime, provider fixtures, package-manager execution, and publication
remain unauthorized. See the [Phase 0 focused
review](docs/concepts/HACKER_NEWS_LAUNCH_ADMISSION_PLAN_REVIEW.md).
Phase 1 packaging-contract implementation is complete subject to terminal-green
exact-head CI. It adds deterministic provisional native archives for Apple
Silicon macOS and musl Linux x86-64, exact source-commit and native-version
verification, checksums, bounded install/uninstall behavior, local-state
preservation, focused tests, and a non-uploading native Linux CI rehearsal. The
local macOS rehearsal passed and truthfully records a dirty provisional
checkout; the macOS archive is not promotable. Artifact publication, final
candidate claims, additional platforms, package managers, signing, and Phase 2
composition remain unauthorized. Final candidates must be rebuilt from the
exact clean post-composition commit. See the [Phase 1
report](docs/concepts/HACKER_NEWS_LAUNCH_ADMISSION_PHASE_1_REPORT.md) and [native
archive contract](docs/release/NATIVE_ARCHIVE_CONTRACT.md). Focused review
corrected workspace-wide version verification, fork-origin exact-head checkout,
and installer argument handling; no P0 or unresolved P1 remains locally. Merge
still requires terminal-green exact-head CI, including native Linux packaging.
See the [Phase 1 focused
review](docs/concepts/HACKER_NEWS_LAUNCH_ADMISSION_PHASE_1_REVIEW.md).

After Hacker News launch admission is accepted, evaluate one future **Governed
Obligation Plan reconciliation** lane. The purpose is to test
whether existing Workflow OS workflows, policies, required context, checks,
approvals, SideEffects, and report contracts can derive one deterministic,
inspectable obligation plan without creating a second source language,
evaluator, authority system, or completion model. Core remains the only
governance and completion authority; any derived plan is non-authorizing,
rebuildable projection data bound to canonical inputs. The lane must begin with
an architecture-to-existing-Core reconciliation matrix, then proceed only
through a model-only derived plan, deterministic explanation and differential
validation, and one separately reviewed opt-in runtime composition. Incremental
computation may be considered only after measured clean-derivation and
reevaluation costs demonstrate a material need. Salsa is not selected,
authorized, or placed on the roadmap by this lane; no dependency or MSRV change
is permitted without a later evidence-backed architecture decision. A separate
`WorkContract` language, public GWP ABI, competing evaluator, optimizer, and
compiler-specific SDK/CLI remain deferred. See the [Governed Obligation Plan
Reconciliation Plan](docs/implementation-plans/governed-obligation-plan-reconciliation-plan.md).
See the
[post-0.3 queue selection
report](docs/concepts/POST_0_3_ROADMAP_QUEUE_SELECTION_REPORT.md) and the
[planning report](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_PUBLIC_ADOPTION_RECOVERY_PLAN_REPORT.md).

#### Recently Closed Queue Evidence

- **Accepted the capability-gated GitHub read-only contract foundation after
  one security correction.** Review found that a valid adapter tuple could be
  attached to an unrelated skill shape. Rust and TypeScript validation now
  require the complete exact capability and input contract before recognizing
  the operation. The slice remains dark, local, and no-write. See the [focused
  review](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_CONTRACT_FOUNDATION_REVIEW.md).

- **Accepted the capability-gated GitHub read-only public-adoption plan after
  security corrections.** Review made authoritative execution a prerequisite,
  separated transient credential delivery from authority state, required the
  complete authorized-execution lifecycle, and split implementation into four
  governed phases so partial contracts cannot expose a provider path. See the
  [review](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_PUBLIC_ADOPTION_RECOVERY_PLAN_REVIEW.md).

- **Planned capability-gated GitHub read-only public adoption and recovery.**
  The plan turns the existing private repository-metadata proof into one
  coherent prospective user path without treating activation as authority or
  adding a one-off provider command. It defines exact authored selection,
  reviewed project constraints, Core-owned current facts, atomic dispatch,
  claim-only and invocation-started recovery, disclosure, compatibility, and a
  complete acceptance matrix. Focused plan review is next; no runtime or
  public contract changed in planning. See the [plan](docs/implementation-plans/capability-gated-github-read-only-public-adoption-recovery-plan.md)
  and [planning report](docs/concepts/CAPABILITY_GATED_GITHUB_READ_ONLY_PUBLIC_ADOPTION_RECOVERY_PLAN_REPORT.md).

- **Selected the post-`0.3.0-preview.1` no-write adoption lane.** Current main
  has enough accepted governance and executor composition to stop adding
  disconnected primitives and make one existing read-only operation safely
  adoptable. OpenShell provider wiring remains blocked on upstream evidence;
  provider mutation broadening and nested harness runtime remain premature.
  Planning the exact public-adoption and recovery boundary is therefore the
  highest-value next step. No implementation, schema, CLI, provider, release,
  or package behavior changed during queue selection. See the [selection
  report](docs/concepts/POST_0_3_ROADMAP_QUEUE_SELECTION_REPORT.md).

- **Prepared and locally validated `0.3.0-preview.1`.** Aligned Rust and npm
  package versions, lockfiles, CLI identity, changelog, current product
  contract, security posture, known limitations, release notes, GitHub release
  draft, checklist, and final review around the governed-execution and
  continuity preview. The required Rust, Node, contract, integration,
  documentation, and dependency-audit gates pass. Six vulnerable or yanked
  transitive dependencies were updated to fixed compatible versions; the
  already documented `serde_yaml` / `unsafe-libyaml` maintenance warning
  remains accepted for trusted local specs. No tag, GitHub Release, package
  publication, provider broadening, production claim, or new runtime lane is
  authorized by this preparation.

- **Implemented and accepted the first-run quiet-success artifact
  composition.** Added the separately identified generated artifact workflow,
  low-sensitivity artifact skill, enforced observe-and-report policy, and
  additive scaffold upgrade. Bound a CLI-private one-use local artifact
  request into immutable run context, kept Core authoritative for route
  selection, created no approval proof on proceed routes, preserved
  proof-backed escalation and denial, and retained legacy run replay/recovery.
  Ordinary first-run and the existing approval/audit demo remain unchanged.
  The closed acceptance matrix, full workspace suite, and
  disposable-repository proof pass. No arbitrary command, provider, network,
  external SideEffect, schema, SDK, hosted, or release broadening was added.
  See the accepted [plan](docs/implementation-plans/first-run-quiet-success-artifact-composition-plan.md),
  [implementation report](docs/concepts/FIRST_RUN_QUIET_SUCCESS_ARTIFACT_IMPLEMENTATION_REPORT.md),
  and [focused review](docs/concepts/FIRST_RUN_QUIET_SUCCESS_ARTIFACT_IMPLEMENTATION_REVIEW.md).
  Review found and resolved one compatibility blocker: shared project reload
  now validates the exact contract selected by durable run identity, so legacy
  pre-upgrade runs remain admissible while a new artifact identity still fails
  closed without the complete new contract.

- **Accepted the first-run quiet-success artifact composition plan after two
  corrections.** The artifact path now has a separate exact low-sensitivity
  skill rather than reusing the medium-sensitivity demo skill. A CLI-private,
  one-use local artifact request supplies narrowly scoped request authority and
  is committed into immutable run context without becoming approval, reusable
  capability, or an authority receipt. The enforced `allow_local` policy,
  Core-owned routing, monotonic escalation, compatibility, and proof-free
  proceed-route artifact gates are accepted. See the [focused review](docs/concepts/FIRST_RUN_QUIET_SUCCESS_ARTIFACT_COMPOSITION_PLAN_REVIEW.md).

- **Planned the first-run quiet-success artifact composition.** New starts use
  a distinct exact generated artifact workflow and observe-and-report policy;
  the existing approval-gated workflow remains the mock audit demo. Core alone
  derives quiet, visible, approval-required, or denied posture from complete
  current facts. Existing in-flight artifact runs retain their durable legacy
  contract, and missing new scaffold files produce an additive-upgrade next
  action rather than fallback or force replacement. Focused plan review is
  required before implementation. See the [plan](docs/implementation-plans/first-run-quiet-success-artifact-composition-plan.md).

- **Selected the first-run quiet-success artifact planning lane.** The accepted
  next move composes the newly accepted first-run governed artifact path with
  existing proportional-governance and quiet-success foundations. Explicit
  invocation expresses intent to request the local artifact but is not itself
  approval or authority. Human interruption may be omitted only for a distinct
  exact contract whose complete authoritative inputs produce `Proceed`; any
  explicit approval minimum, incomplete fact, drift, or escalation remains
  blocking or denied. The existing approval/audit demo and ordinary
  non-mutating first-run command remain unchanged. See the [queue-selection
  report](docs/concepts/FIRST_RUN_QUIET_SUCCESS_ARTIFACT_QUEUE_SELECTION_REPORT.md).

- **Accepted the first-run governed WorkReport artifact implementation.**
  Focused maintainer/security review found and fixed one blocker: current
  repository drift could prevent a proof-backed denial. Denial is now
  source-free and artifact-free while grants still require exact current
  analysis and fresh authoritative check evidence. The review accepts immutable
  context binding, exact contract admission, two distinct approval gates,
  decision-time check citation, create-only persistence, exact replay, and
  check-citing crash recovery. No provider, network, repository-command,
  hosted, publication, or default-mutation behavior was added. See the
  [review](docs/concepts/FIRST_RUN_GOVERNED_WORK_REPORT_ARTIFACT_REVIEW.md).

- **Implemented the first-run governed WorkReport artifact vertical slice.**
  Ordinary `workflow-os first-run` remains non-mutating. The explicit
  `--write-report-artifact` path accepts only the exact generated contract,
  binds the bounded analysis commitment into immutable run identity, executes
  the declared authoritative project-validation check, routes through
  proportional governance, requires proof-backed aggregate and authored-step
  approvals, cites the decision-time check result, and persists one create-only
  local artifact. Exact replay and commitment-gated missing-artifact recovery
  are implemented. No repository command, provider, network, external write,
  OpenShell, hosted, or default-mutation behavior was added. See the
  [implementation report](docs/concepts/FIRST_RUN_GOVERNED_WORK_REPORT_ARTIFACT_IMPLEMENTATION_REPORT.md).

- **Accepted the first-run governed WorkReport artifact implementation plan.**
  Focused maintainer/security review rejected post-approval-only analysis and
  a nonrecoverable artifact crash gap. The accepted design binds a versioned
  bounded analysis commitment into the immutable run before approval, permits
  only exact allowlisted generated contracts to receive the trusted local
  handler, and allows missing-artifact recovery only when current bounded
  analysis exactly matches the immutable commitment. See the [implementation
  plan](docs/implementation-plans/first-run-governed-work-report-artifact-plan.md).

- **Selected the first-run governed WorkReport artifact planning lane.** The
  accepted next move composes existing real first-run posture analysis with
  existing report and local artifact foundations instead of adding another
  model family or crossing a blocked provider boundary. Default first-run
  remains non-mutating, and the generated mock workflow remains explicitly an
  approval/audit demonstration. Entry conditions, exit criteria,
  alternatives, and strict non-goals are recorded in the [queue-selection
  report](docs/concepts/FIRST_RUN_GOVERNED_WORK_REPORT_ARTIFACT_QUEUE_SELECTION_REPORT.md).

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
  exact-request-bound observation result through the accepted Core constructor
  and validator. Repeated construction is deterministic; non-fixture,
  incomplete, stale, or substituted input fails closed. No live host or
  process access exists. Focused proof review is next. See the [implementation
  report](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_FIXTURE_PROOF_REPORT.md).

- **Accepted trusted-host input observer freshness-authority fix.** Focused
  maintainer/security review confirms that request and result bind one exact
  ordering-authority commitment, every compared revision is defined in that
  shared domain, authority substitution and stale observation fail closed,
  canonical commitments include the authority, and privacy remains intact.
  The Core model is accepted with no remaining blocker. See the [focused
  review](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_FRESHNESS_AUTHORITY_FIX_REVIEW.md).

- **Closed trusted-host input observer freshness-authority blocker fix.** The
  request and result now bind one exact ordering-authority commitment, and all
  static-discovery completion and observation revisions are explicitly defined
  in that shared domain. Request and result commitments include the authority;
  omission and substitution fail closed. Focused review is next. See the
  [freshness-authority fix
  report](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_FRESHNESS_AUTHORITY_FIX_REPORT.md).

- **Closed trusted-host input observer Core model blocker-fix review with one
  remaining blocker.** Six original findings are closed: producer identity and
  version, homogeneous per-fact provenance, closed non-proof reasons,
  non-observed false-proof rejection, producer/method coherence, and nested
  wire privacy. Freshness is still not proven because the implementation
  compares a static-discovery revision to a producer-local revision without
  binding a shared ordering authority. A narrow freshness-authority fix is
  next. See the [blocker-fix
  review](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_BLOCKER_FIX_REVIEW.md).

- **Closed trusted-host input observer Core model blocker fix.** The request
  now binds the exact expected producer identity/version and a static-discovery
  completion revision. Results bind homogeneous per-fact provenance and reject
  stale observations. Non-proven facts require closed reasons and cannot retain
  a proven binding axis; producer, posture, and method combinations fail closed.
  Every nested enum reachable through the wire boundary now rejects unknown
  values without echoing caller input. Focused blocker-fix review is next. See
  the [blocker-fix
  report](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_BLOCKER_FIX_REPORT.md).

- **Closed trusted-host input observer Core model review with blocker
  findings.** The review accepted the model-only scope, closed nine-obligation
  set, repository-root composition, deterministic commitments, and
  non-authorizing boundary. It found seven focused gaps in producer binding,
  per-fact provenance, non-proof reasons, fact coherence, producer/method
  coherence, nested deserialization privacy, and freshness relative to static
  discovery. A narrow blocker fix is next. See the [focused
  review](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_REVIEW.md).

- **Closed provider-neutral trusted-host input observer Core model
  implementation.** Core now validates exact observation requests, producer
  claims, all nine payload-free host-input facts, and one reused read-only
  repository-root observation. Canonical request/result commitments bind every
  semantic field with pinned vectors; malformed, incomplete, substituted, or
  incoherent values fail closed. The model remains process-free and
  non-authorizing. Focused maintainer/security review is next. See the
  [implementation
  report](docs/concepts/TRUSTED_HOST_INPUT_OBSERVER_CORE_MODEL_REPORT.md).

- **Accepted provider-neutral trusted-host input observer planning blocker
  fix.** Focused re-review verified exact reuse of the existing repository-root
  observation, static reposito