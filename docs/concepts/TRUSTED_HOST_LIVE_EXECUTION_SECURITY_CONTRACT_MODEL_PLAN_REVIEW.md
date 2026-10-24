# Trusted-Host Live-Execution Security Contract Core Model Plan Review

## 1. Executive Verdict

**Plan accepted with mandatory implementation constraints; proceed to the
provider-neutral, non-authorizing core model only.**

The plan preserves the security boundary established by the Cargo clippy
live-execution review. It separates required controls from observed facts,
keeps assessment pure, reuses the existing hosted-execution vocabulary where
the semantics match, and does not authorize a process, provider, handler,
workflow, or local-check registration path.

Implementation must close four ambiguities before the model can be accepted:

1. represent `not_started`, `started`, and `may_have_started` explicitly rather
   than inferring attempt posture from event counts;
2. bind observations to the exact provider selection or provider-selection
   commitment accepted for the attempt without coupling Core to one provider;
3. never fabricate a hosted runtime-image digest for a trusted-host attempt
   that has no image identity; and
4. define domain-separated canonical fingerprints and explicit collection and
   count limits.

These are mandatory model constraints. They do not authorize runtime work.

## 2. Scope Verification

The plan stays within the approved model-only scope. It does not authorize:

- a process runner, shell, subprocess, or live Cargo invocation;
- input discovery, filesystem inspection, sandboxing, or network policy;
- provider selection, OpenShell integration, or provider credentials;
- workflow composition, executor dispatch, profile registration, or public
  execution surfaces;
- persistence, events, reports, artifacts, schemas, SDKs, examples, or CLI;
- dependency additions, provider mutations, writes, hosted expansion, or
  release changes.

The proposed requirements, observations, and assessment are descriptive and
evaluative only.

## 3. Existing Model Reuse Assessment

The plan correctly prefers composition over a second terminal receipt
hierarchy. The following existing concepts are appropriate where their current
meaning is exact:

- `SpecContentHash` for bounded commitments;
- `HostedExecutionEnforcementMode` for enforce versus audit posture;
- `HostedExecutionControlPosture` for enforced, degraded, skipped,
  unavailable, and unsupported controls;
- `HostedExecutionCleanupPosture` for terminal cleanup facts;
- `HostedExecutionReference` for bounded typed references;
- `HostedExecutionObservationSummary` for existing aggregate observations;
  and
- `HostedExecutionAttestation` and `HostedExecutionReceipt` as later
  composition surfaces.

The implementation must not widen existing hosted wire enums merely to add an
uncertain state for this model. A narrow live-security observation-certainty
or attempt-posture type is safer because adding an enum variant changes a
public serialization contract and may force unrelated hosted code to handle a
state it did not previously admit.

`HostedExecutionAttestation` currently requires a `runtime_image_digest`.
Trusted-host execution may have no container or image identity. The new model
must not invent one, substitute a toolchain hash, or use a sentinel hash.
Composition with the hosted attestation is valid only when a real runtime
image identity exists. Otherwise the model should retain a narrow
provider-neutral environment or base commitment and defer receipt composition.

## 4. Requirement And Observation Separation

The top-level split between
`TrustedHostLiveExecutionSecurityRequirements` and
`TrustedHostLiveExecutionSecurityObservations` is correct. Construction of
either side must not imply satisfaction.

The implementation must also keep provenance explicit:

- requirements are Core-owned accepted constraints;
- observations are claims about one provider attempt;
- observation validation proves shape and coherence, not truth; and
- assessment proves only that the supplied validated observations satisfy the
  accepted requirements for the exact binding.

The model and docs must not call a structurally valid observation
"attested" unless it is actually composed with the existing attestation
boundary and its provenance requirements.

## 5. Exact Binding Assessment

Binding the immutable run bundle, workflow/run/step identity, command-contract
fingerprint, attempt identity, request fingerprint, and requirements
fingerprint is necessary and appropriately scoped.

Mandatory constraints:

- every fingerprint must use a versioned, domain-separated canonical encoding;
- unordered collections must be canonicalized before hashing;
- duplicate identities must fail rather than disappear during sorting;
- an observation must bind the exact accepted provider selection or a
  provider-selection commitment, even though the model remains provider
  neutral; and
- request, attempt, requirements, manifest, and provider-selection
  substitution must have distinct stable error codes.

Provider-neutral means the contract is not designed around OpenShell or any
other provider. It does not mean provider identity can be omitted from exact
attempt binding.

## 6. Attempt-Start And Ambiguity Assessment

The proposed assessment outcomes preserve the important distinction between
safe pre-start denial and possibly-started reconciliation. The plan does not,
however, name an explicit observation that can prove this distinction.
`process_start_events == 0` is not proof that a process did not start when
telemetry may be incomplete or unavailable.

The implementation must add a closed attempt-start posture with semantics
equivalent to:

- `NotStarted`;
- `Started`; and
- `MayHaveStarted`.

It must be coherent with process-start counts, process-tree terminality,
provider failure posture, and observation availability. `DeniedBeforeStart`
is valid only with affirmative `NotStarted` evidence. Any uncertainty after
the provider call boundary maps to reconciliation, never safe retry.

## 7. Build-Time Input Assessment

The proposed input kinds cover the material Cargo and platform surfaces. The
payload-free manifest shape is appropriate, and the plan correctly says that
model validation cannot prove that a discovery implementation deserves to
claim `Complete`.

Mandatory constraints:

- completeness is an observed claim, never derived merely from a non-empty or
  internally valid manifest;
- the discovery method is closed vocabulary with an explicit unsupported or
  unavailable posture;
- aggregate counts and any per-kind or per-input collections have named hard
  maximums enforced during construction and deserialization;
- total and per-kind counts must reconcile deterministically; and
- no file name, path, package name, source content, environment value, or raw
  inventory may enter the model, errors, Debug, or serialization.

Only `Complete` may satisfy mandatory closure, and later input-discovery work
must independently justify when it may emit that value.

## 8. Root-Binding Assessment

Typed root roles and separate required versus observed facts are the right
model. The roles cover repository, toolchain, dependency, cache, build-output,
and temporary roots without exposing paths.

Implementation must require exactly one observation for every required role,
reject extra or duplicate roles, and distinguish read-only from writable
access. Invocation-scoped creation must apply to writable roots where required.
Object continuity must span admission through terminal process-tree and
observation collection, not merely parent-process exit.

No public inode, path, device number, mount label, or provider-specific object
identifier should become the portable contract. Stable commitments and closed
observation methods are sufficient for this phase.

## 9. Filesystem, Network, And Transient-Write Assessment

The plan correctly requires enforce mode, default-deny filesystem access,
protected read-only roots, typed writable roots, zero allowed network events,
policy stability, and distinct transient-write prevention and observation.

Mandatory constraints:

- a zero count is meaningful only when the corresponding observation channel
  is affirmatively available and complete;
- count fields require explicit maximums rather than relying only on `u32`;
- policy-change count zero does not replace an explicit revision-stability
  observation;
- the denied-egress proof probe remains separate from the normal invocation;
  and
- prevented, observed-and-denied, unobserved, unobservable, unsupported, and
  uncertain write posture remain distinct.

No denied path, endpoint, payload, command output, or sandbox log may be
stored.

## 10. Process-Tree And Cleanup Assessment

The process requirements are complete enough for the model slice: provider
ownership, descendant containment, timeout and cancellation propagation,
terminal-tree confirmation, post-terminal mutation prevention, and explicit
resource-budget posture.

The model must preserve three separate facts:

1. the parent command returned;
2. the complete process tree is terminal; and
3. cleanup and writable-output quiescence completed.

A successful parent exit cannot satisfy the assessment when descendants are
uncertain, termination is incomplete, cleanup is ambiguous, or output remains
mutable. Unsupported mandatory resource controls also fail closed rather than
becoming advisory.

## 11. Pure Assessment And Coherence Assessment

One pure assessment function is the correct boundary. It may validate and
compare values but must not inspect the host, call a provider, append events,
change workflow state, persist data, or grant authority.

The coherence matrix in the plan is strong. Implementation tests must cover
all cross-field combinations, especially:

- affirmative `NotStarted` versus process-start observations;
- `Started` or `MayHaveStarted` versus pre-start denial;
- terminal process tree versus cleanup ambiguity;
- complete input posture versus count disagreement;
- zero network events versus unavailable network observation;
- enforced controls versus audit enforcement mode; and
- a satisfied assessment with any unavailable, unsupported, degraded, or
  uncertain mandatory fact.

The assessment result should carry bounded stable reason codes or a bounded
set of reason enums, not caller-authored summaries.

## 12. Authority Assessment

The plan preserves the authority boundary. A satisfied assessment is evidence
about one exact attempt. It is not a capability, approval, policy decision,
dispatch grant, retry grant, handler registration, or reusable provider
credential.

Public exports should be the minimum needed for model construction and focused
tests. Keep assessment orchestration crate-private unless a concrete later
composition phase proves an external need. Existing Cargo clippy executor
denial and registration exclusion must remain unchanged and tested.

## 13. Privacy, Serde, And Error Assessment

Validated wire types, private fields, custom Debug, and stable generic errors
match repository practice. Unknown enum values and impossible wire states must
fail closed.

Mandatory constraints:

- IDs, hashes, references, provider identities, and infrastructure commitments
  are redacted from Debug;
- deserialization errors contain no rejected value;
- fingerprints and references cannot use secret-like caller strings;
- serialization contains no extension map or untyped payload field; and
- invalid wire data is reconstructed only through validated constructors.

The proposed stable error families are appropriate. Distinct failures may use
distinct codes, but messages should remain generic and payload-free.

## 14. Test-Plan Assessment

The 31 planned test families are proportionate to a security boundary. Add
focused cases for:

- domain separation between each fingerprint family;
- canonical ordering and duplicate rejection;
- explicit collection and count bounds;
- trusted-host execution without a fabricated runtime-image digest;
- provider-selection substitution;
- `NotStarted`, `Started`, and `MayHaveStarted` coherence; and
- zero observed events with unavailable telemetry failing closed.

Tests must remain process-free, provider-free, network-free, filesystem-free,
and sandbox-free. Existing hosted receipt behavior must not be changed to make
new tests pass.

## 15. Blockers

No planning blocker remains if the implementation treats the constraints in
this review as acceptance requirements.

The following remain blockers to any live execution phase:

- reviewed deterministic build-input discovery;
- a selected provider meeting the contract;
- operating-system filesystem and zero-egress containment;
- full process-tree ownership and terminal reconciliation;
- transient-write prevention and observation;
- effective-control evidence independent from request echo;
- focused implementation and security review; and
- a separately approved live smoke.

## 16. Non-Blocking Follow-Ups

- Decide during implementation whether a narrow observation-certainty enum or
  separate posture types produce the smallest coherent API.
- Evaluate whether existing hosted references are sufficient for every new
  observation before adding any reference kind.
- Keep eventual hosted receipt composition in a separate plan after model
  review.

## 17. Recommended Next Phase

Implement the provider-neutral trusted-host live-execution security contract
core model only.

The implementation should add validated requirements, observations, explicit
attempt-start posture, bounded input and root facts, transient-write and
process-tree posture, domain-separated fingerprints, and one pure assessment.
It must not add a runner, live Cargo, discovery, provider selection, OpenShell,
composition, registration, persistence, public execution surfaces,
dependencies, writes, or release changes.

## 18. Governed Review Record

- Workflow ID: `dg/review`.
- Run ID: `run-1792811921538731000-2`.
- Approval ID:
  `approval/run-1792811921538731000-2/review-scope-approved`.
- Approval-presentation ID: `presentation/e893e4c7a04c7e2b`.
- Approval-presentation content hash:
  `e893e4c7a04c7e2b73df39824c89fc8400bf7052d7530611c623a9cda3eb56d2`.
- Approval outcome: granted under standing delegated-maintainer authority for
  the exact review-only scope.
- Phase status: `Completed` with 39 events, one approval, zero retries, and
  zero escalations.
- Validation: `npm run check:docs` and `git diff --check` passed.
- Out-of-kernel work: the delegated maintainer inspected the plan and existing
  hosted contracts, authored this review and roadmap update, and ran
  documentation and diff validation. The kernel governed scope and approval
  but did not edit files, run checks, or perform git and PR operations.
