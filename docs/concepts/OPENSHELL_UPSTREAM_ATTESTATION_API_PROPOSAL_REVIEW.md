# OpenShell Upstream Attestation API Proposal Review

## 1. Executive Verdict

Proposal accepted; proceed to one bounded upstream architectural discussion
draft for human review.

The proposal preserves the correct division of responsibility. NVIDIA
OpenShell would expose authoritative sandbox lifecycle, applied-control,
operation, observation, and cleanup facts. Workflow OS would remain the system
that authorizes governed execution, accepts evidence, records audit state, and
produces reports. The proposal does not turn OpenShell into a workflow
governance system or Workflow OS into a sandbox runtime maintainer.

This verdict authorizes drafting only. It does not authorize upstream
submission, OpenShell installation or execution, provider wiring, access
material, writes, schemas, examples, a fork, or a production claim.

## 2. Scope Verification

The proposal stayed within the accepted planning scope. It defines:

- provider-neutral lifecycle and attestation gaps;
- idempotent creation and restart-safe lookup;
- canonical policy commitment and applied-state relationships;
- driver-observed image and applied-control posture;
- durable operation identity and terminal semantics;
- complete operation-bound observation manifests;
- exact resource-version-bound deletion and cleanup receipts;
- typed capability negotiation;
- authorization, privacy, ambiguity, retention, and rollout posture; and
- a strict no-fork threshold.

It does not authorize runtime implementation, provider selection, credentials,
external mutation, agent orchestration, hosted administration, schema changes,
or upstream submission.

## 3. Product And Architecture Assessment

The optional execution-provider boundary remains strategically correct.
Workflow OS should decide whether execution is allowed and which evidence must
return. OpenShell should enforce and report the containment facts only its
gateway, driver, supervisor, and platform can know authoritatively.

The proposal also avoids the wrong abstraction of treating OpenShell as an
ordinary skill handler. Sandbox creation, policy application, execution,
observation, and cleanup form a typed security lifecycle. That lifecycle needs
capability negotiation, durable identities, ambiguity semantics, and exact
evidence bindings that a generic invocation interface would obscure.

The integration must remain optional. Fresh-pull product evaluation confirms
that Workflow OS already has a useful local governance experience and that the
next product priority is reducing low-risk ceremony through quiet success.
OpenShell should strengthen explicitly selected execution paths; it must not
become a prerequisite for first-run posture, workflow authoring, or ordinary
local governance.

## 4. Upstream Generality Assessment

The proposal is useful beyond Workflow OS. CI systems, compliance tools,
security products, and other sandbox consumers also need to answer:

- which resource one request created;
- which policy, image, and controls were applied;
- which durable operation ran and how it terminated;
- whether observations completely cover that operation; and
- whether the exact resource reached terminal cleanup.

The resource names and semantics remain sandbox-native. They do not expose
Workflow OS approvals, policy gates, EvidenceReference values, SideEffects, or
WorkReports in OpenShell. That makes the proposal appropriate for upstream
discussion.

The complete model should be presented as one architectural direction, not as
one indivisible implementation request. After upstream feedback, the work may
be split into independently useful issues or patches. Opening multiple broad
issues before maintainers confirm the preferred resource boundaries would be
premature.

## 5. Resource Model Assessment

The five proposed records form a coherent lifecycle:

```text
SandboxCreationRecord
  -> SandboxAppliedStateSnapshot
  -> SandboxExecOperation
  -> ObservationExportManifest
  -> SandboxDeletionOperation
```

The proposal previously called this set four records while listing five. That
editorial inconsistency was corrected during review. It did not change the
semantic design.

Stable gateway IDs, resource versions, request commitments, timestamps, typed
postures, and schema versions are the right shared join keys. Equivalent
upstream types are acceptable; exact candidate names are not requirements.

## 6. Creation And Reconciliation Assessment

Creation idempotency is correctly treated as a durable provider mutation, even
for a no-write governed workload. A transport failure after create cannot be
reported as not-started without authoritative proof.

The proposed rules are sufficient:

- idempotency scope is documented;
- the request commitment binds retries to one request;
- matching retries return the original result;
- conflicting reuse fails closed;
- the creation record is persisted before or atomically with intent; and
- lookup survives caller and gateway restart within a declared retention
  boundary.

Names and labels remain metadata rather than reconciliation identity. This
prevents accidental substitution or name-reuse races.

## 7. Policy And Applied-State Assessment

Committing canonical structured policy is the right provider-neutral
correction to the earlier exact-YAML assumption. Original YAML formatting is
not a security fact once OpenShell has parsed and persisted typed policy.

The proposal still requires the load-bearing relationships:

- gateway-computed canonical input commitment;
- provenance generated by enforcing components;
- complete composed effective-policy commitment;
- accepted and loaded revision identity;
- driver/platform control posture;
- operation binding; and
- explicit drift and degradation state.

A final hash without input lineage, composition revisions, and load state would
remain insufficient. Caller annotations cannot substitute for provider-owned
attestation.

## 8. Runtime Image And Control Assessment

The applied-state snapshot correctly distinguishes requested image from the
immutable image observed by the compute driver after resolution. A digest-like
request is intent; it is not proof that the runtime launched that image.

Typed control families and effective modes are also necessary. Required hard
controls must not disappear into opaque driver configuration or human-readable
conditions. Unsupported, unavailable, skipped, degraded, and best-effort
postures must remain distinguishable so Workflow OS can fail before execution
when a required capability is absent.

## 9. Durable Operation Assessment

The durable exec operation is appropriately separate from a transient stream.
It binds one canonical request to the sandbox resource version and applied
snapshot, preserves an operation ID across restart, and distinguishes timeout,
signal, cancellation, terminal failure, success, and ambiguous may-have-started
outcomes.

The proposal includes an environment map in the candidate request because that
matches a general execution API, but durable records and errors must not retain
or echo environment values. The bounded upstream discussion should ask how
OpenShell commits secret-bearing requests without turning sensitive inputs into
retrievable operation metadata. The first Workflow OS prototype remains
access-material-free and should send no environment secrets.

## 10. Observation Manifest Assessment

An operation-bound manifest is the correct abstraction. Complete individual
OCSF records do not prove a complete observation interval. The manifest must be
finalized by the observing subsystem and include interval watermarks, final
flush posture, event and drop counts, integrity commitment, completeness, and
stable retrieval posture.

Bounded counts and stable references let Workflow OS accept useful facts
without copying raw security logs into Core. A deliberate denied-egress event
is compelling prototype evidence only when it falls inside the exact complete
operation interval.

## 11. Deletion And Cleanup Assessment

Resource-version-bound deletion and a durable terminal deletion operation are
required. Delete acceptance or later name absence cannot prove exact cleanup;
both can race with replacement or name reuse.

The proposed receipt correctly separates gateway absence, driver absence,
observation finalization, process teardown, and temporary access-material purge
posture. Unresolved cleanup must remain explicit and must block a successful
execution receipt.

## 12. Capability Negotiation Assessment

Typed runtime capability negotiation is necessary because support varies by
gateway, driver, platform, and version. Compile-time claims or known-version
tables cannot prove that the active combination exposes the required facts.

The proposed capability snapshot allows a caller to reject unsupported,
degraded, unavailable, or unknown required capabilities before sandbox
creation. That reduces unnecessary lifecycle activity and supports the broader
Workflow OS quiet-success principle: successful eligible work should remain
concise, while missing containment facts fail early and visibly.

## 13. Authorization, Privacy, And Retention Assessment

The proposal preserves the payload-minimizing Core boundary. Stable IDs,
commitments, versions, typed postures, timestamps, bounded counts, and governed
references may cross into Workflow OS. Raw policy, observations, stdout,
stderr, environment values, provider payloads, and access material may not.

The bounded upstream discussion should ask OpenShell maintainers to identify:

- authorization scopes for each record and raw retrieval surface;
- which component owns each asserted fact;
- retention guarantees for reconciliation records and manifests;
- behavior after retention expires; and
- whether commitments over sensitive input classes create metadata risk.

No caller-controlled field may masquerade as driver-observed image, control,
drop-count, or cleanup evidence.

## 14. Compatibility And Delivery Assessment

The incremental rollout sequence is practical. Optional request fields and new
lookup resources can be added without silently changing existing callers, and
partial upstream delivery remains useful to other consumers.

Workflow OS must not equate partial delivery with provider readiness. After
each reviewed upstream release or commit, the complete evidence matrix must be
rerun. Provider wiring remains blocked until every required capability is
authoritative for the selected gateway, driver, and platform.

Every accepted schema needs explicit versioning, support and retention notes,
fixtures, compatibility posture, and rollback behavior.

## 15. Fork Decision

Do not fork OpenShell.

The proposal has not encountered an upstream refusal, exhausted extension
options, or proven that a narrow independent observer cannot supply a missing
fact. The accepted threshold remains appropriate: only a security-critical
unavailable fact, explicit upstream rejection, no trustworthy observer, a
narrow sustainable patch, and a new Workflow OS ADR accepting lifecycle and
vulnerability burden could justify reconsideration.

## 16. Blockers

None for preparing one bounded upstream discussion draft for human review.

Provider wiring, live sandbox proof, and production claims remain blocked by
the unavailable authoritative facts documented in the v0.0.101 matrix.

## 17. Non-Blocking Follow-Ups

- Ask upstream which proposed records should be first-class resources and
  which should extend existing resources.
- Ask where durable mutation and observation state should live across gateway
  restart and declared retention expiry.
- Clarify commitment semantics for secret-bearing general exec requests while
  keeping the first Workflow OS prototype free of access material.
- Confirm how driver/platform capability variation should be represented.
- Present the full architecture in one discussion, then split implementation
  only with upstream maintainer guidance.
- Keep the current v0.0.101 CLI transport disconnected as compatibility
  regression coverage.

## 18. Recommended Next Phase

One concise
[OpenShell Trustworthy Sandbox Attestation Discussion Draft](../implementation-plans/openshell-upstream-attestation-discussion-draft.md)
is now prepared for human review.

The draft should lead with the general trustworthy-sandbox-attestation problem,
include the exact v0.0.101 evidence gaps, propose the five-record lifecycle as
candidate semantics, and ask maintainers which boundaries align with upstream
direction. Candidate protobuf remains non-final.

Do not submit the draft automatically. Do not install or execute OpenShell,
change Workflow OS Rust, wire a provider, add access material or writes, expose
schemas or examples, fork OpenShell, or make production claims.

## 19. Governed Review Evidence

- workflow ID: `dg/review`;
- run ID: `run-1786269258642204000-2`;
- approval ID:
  `approval/run-1786269258642204000-2/review-scope-approved`;
- approval presentation ID: `presentation/59e757179cd3298d`;
- approval outcome: granted by delegated maintainer with persisted presentation
  proof;
- event summary: 39 events, one approval, zero retries, and zero escalations;
- validation summary: `npm run check:docs` passed and `git diff --check`
  passed;
- provider/runtime activity: none;
- out-of-kernel work: source-backed document review, documentation edits,
  documentation validation, Git operations, and GitHub pull-request actions.

Required phase-close validation:

- `npm run check:docs`: passed;
- `git diff --check`: passed;
- governed event-trail inspection: completed through `phase-close`; and
- changed-surface inspection: only documentation changed, excluding local
  untracked `.workflow-os/` state.
