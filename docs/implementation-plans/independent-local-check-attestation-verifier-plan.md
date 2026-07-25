# Independent Local Check Attestation Verifier Plan

Status: Pure verifier implemented and accepted after the phase-level
stored-bundle integrity blocker was fixed by requiring validated
`StoredImmutableRunBundle` input and deriving the trusted binding from its
manifest. The planning
blocker fix and focused re-review are accepted, and the prerequisite immutable
local-check execution binding core model is implemented and accepted with
non-blocking provenance follow-ups. Focused planning review found
that the current immutable run bundle does not freeze a local check command
contract or trusted handler implementation identity. This fix defines a
separate pre-execution immutable binding rather than claiming current bundle
membership. See
[Independent Local Check Attestation Verifier Plan Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_VERIFIER_PLAN_REVIEW.md).
The independent local check attestation requirement and
unverified binding models are implemented, their deterministic-binding blockers
are fixed, and focused review accepts the fix. The future verifier must require
Core-owned binding provenance and must not treat public fingerprint
recomputation as authenticity. The implemented verifier enforces that boundary
through a crate-private observation and verifier API plus a public read-only
accepted record. Runtime integration, persistence, event projection, schema,
and CLI behavior remain unimplemented. See
[Independent Local Check Attestation Verifier Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_VERIFIER_REVIEW.md).
The focused correction is documented in
[Independent Local Check Attestation Verifier Blocker Fix Report](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_VERIFIER_BLOCKER_FIX_REPORT.md).
Focused acceptance is documented in
[Independent Local Check Attestation Verifier Blocker Fix Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_VERIFIER_BLOCKER_FIX_REVIEW.md).
Runtime-composition inspection later found that workflow and run identity were
not compared directly to the validated stored manifest. The focused fix is
documented in
[Independent Local Check Attestation Stored Manifest Identity Blocker Fix Report](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_STORED_MANIFEST_IDENTITY_BLOCKER_FIX_REPORT.md).
Focused acceptance is recorded in
[Independent Local Check Attestation Stored Manifest Identity Blocker Fix Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_STORED_MANIFEST_IDENTITY_BLOCKER_FIX_REVIEW.md).
The first explicit runtime-composition slice is now planned in
[DocsCheck Attestation Runtime Composition Plan](docs-check-attestation-runtime-composition-plan.md).
Its crate-internal in-memory helper is implemented in
[DocsCheck Attestation Runtime Composition Report](../concepts/DOCS_CHECK_ATTESTATION_RUNTIME_COMPOSITION_REPORT.md).
No executor integration, persistence, events, evidence, reports, schemas, or
CLI behavior was added.

## 1. Executive Summary

Workflow OS can now describe the exact proof a local check would need and can
construct a payload-free candidate binding. Every candidate remains explicitly
`Unverified`. Shape validation, a stable fingerprint, or a caller assertion is
not independent proof.

The next implementation must first add an
`ImmutableLocalCheckExecutionBinding` core model. Core creates this
content-addressed binding before observation or execution. It commits the exact
immutable run context, canonical command contract, selected registered handler
identity and honest posture, and effective execution policy. A later pure,
crate-private verifier consumes that binding, the current command contract only
to recompute its commitment, a crate-private kernel observation, the structured
result derived from that observation, and an evaluation time.

The verifier does not run a process, read mutable project definitions, persist a
record, append an event, satisfy a workflow gate, or create evidence. Runtime
composition remains a separately reviewed phase.

## 2. Goals

- Create exactly one authority boundary that can upgrade an unverified candidate.
- Ensure only Core-owned observation paths can invoke that boundary.
- Recompute requirement, command-contract, immutable-run, and candidate bindings.
- Require exact workflow, run, step, invocation, idempotency, handler, and result alignment.
- Evaluate temporal ordering and freshness at the supplied evaluation time.
- Reject caller, mock, external, stale, ambiguous, changed, or policy-exceeding inputs.
- Return stable non-leaking error codes.
- Keep accepted records payload-free, bounded, deterministic, and safe to inspect.
- Preserve future compatibility with proportional-governance evidence/check facts.

## 3. Non-Goals

This plan does not authorize:

- verifier implementation in this planning phase;
- process or shell execution;
- automatic or default local checks;
- public construction of kernel observations;
- public construction or deserialization of accepted attestations;
- mock or caller-asserted success as independent proof;
- persistence, cache reuse, events, audit projection, evidence attachment, or reports;
- executor or approval-gate integration;
- workflow, policy, schema, SDK, CLI, or UI changes;
- provider access, network-enabled checks, side effects, or writes;
- cryptographic signing, remote attestation, hosted runners, RBAC, or IdP integration;
- weakening explicit workflow, policy, authority, evidence/check, or steward minima;
- broader provider mutation families or release posture changes.

## 4. Current Boundary

Implemented:

- `LocalCheckAttestationRequirement` with a deterministic complete fingerprint;
- `LocalCheckAttestationBinding` with payload-free immutable-run and result context;
- assurance and source vocabulary;
- bounded freshness policy;
- source/assurance and result/exit-posture validation;
- candidate fingerprints that exclude caller-selected record identity;
- an `Unverified`-only candidate posture.

Still missing:

- a trusted observation boundary;
- command-contract fingerprinting owned by Core;
- a pre-execution immutable local-check execution binding;
- verification against the stored immutable bundle and that execution binding;
- time-of-use freshness evaluation;
- a non-constructible accepted result;
- runtime enforcement or durable proof.

## 5. Source-Of-Truth Boundaries

| Input | Source of truth | What the verifier must prove |
| --- | --- | --- |
| Requirement | Validated `LocalCheckAttestationRequirement` | Fingerprint recomputes and requirement is eligible for independent v0 proof |
| Immutable context | `StoredImmutableRunBundle` plus durable run binding | Bundle integrity and exact workflow/run/step definition membership |
| Execution binding | `ImmutableLocalCheckExecutionBinding` created by Core before observation | Exact bundle/run/step/skill, command commitment, handler selection, and execution policy were frozen before work |
| Command | Exact validated `LocalCheckCommandContract` supplied for recomputation | Canonical fingerprint matches the pre-execution binding; current mutable command state is not authority by itself |
| Observation | Crate-private kernel observation created by Core execution code | Invocation facts were not supplied through public model APIs |
| Result | Structured `LocalCheckResult` derived from the same observation | Status, timing, exit, truncation, and identity agree |
| Handler | Core-derived identity and `RegisteredUnattested` posture frozen in the execution binding | No mock, caller, missing, or substituted handler satisfies v0 kernel-observed-process proof; implementation integrity is not claimed |
| Evaluation time | Explicit kernel time input | Observation is not future-dated and satisfies the freshness policy now |
| Candidate | `LocalCheckAttestationBinding` | Canonical binding recomputed from all trusted inputs matches exactly |

Repository metadata, mutable project files, natural-language claims, report
citations, and model judgment are not verifier authority sources.

The execution binding is separate from `ImmutableRunBundleDefinitionKind` in
the first phase. Current bundle definition records remain workflow, skill, and
policy only. The binding references and validates the immutable bundle root; it
does not silently widen the bundle taxonomy.

## 6. Pre-Execution Immutable Binding

The first code phase should add a payload-free
`ImmutableLocalCheckExecutionBinding` with a deterministic fingerprint over:

- immutable run bundle binding/root;
- workflow, run, step, and skill ID/version;
- command ID and the canonical full command-contract fingerprint;
- Core-derived handler selection fingerprint over command kind, skill
  ID/version, and registration mode;
- honest handler posture, initially only `RegisteredUnattested`;
- registration profile or mode fingerprint;
- effective working-directory, environment, network, timeout, SideEffect,
  output-capture, redaction, and citation policy commitments;
- binding algorithm/version and creation time;
- bounded creator/system-actor reference where repository primitives support it.

Core must create the binding before observation or process execution. Its
fingerprint is content addressed and excludes caller-selected record identity.
Public callers must not be able to relabel `MockSelected`, `DeclaredOnly`,
`Unavailable`, or an unbound handler as eligible.

The first honest assurance name is `KernelObservedLocalProcess`. It proves that
Core observed a local process under the exact pre-bound registered handler and
execution policy. It does not prove handler source integrity, cryptographic
identity, binary provenance, trusted-host posture, or independent third-party
execution.

## 7. Visibility And Authority Design

The smallest safe first design should use:

- `pub(crate) struct KernelObservedLocalCheck`;
- a `pub(crate)` constructor used only by future Core-owned runner composition;
- `pub(crate) fn verify_local_check_attestation(...)`;
- `pub struct AcceptedLocalCheckAttestation` with private fields and read-only accessors.

`AcceptedLocalCheckAttestation` must have no public constructor. It should not
implement `Deserialize` in the first phase. Serialization may be added only if
the shape is payload-free and needed by a concrete reviewed boundary; otherwise
defer it with persistence.

This visibility is intentional. A public verifier accepting a publicly
constructible "kernel observation" would let callers manufacture the very
authority the verifier is meant to protect.

## 8. Candidate Verifier Input

Use one explicit borrowed input structure, likely
`LocalCheckAttestationVerificationInput<'a>`, carrying:

- the validated requirement;
- the unverified candidate binding;
- the validated stored immutable run bundle;
- the validated pre-execution immutable local-check execution binding;
- the exact command contract;
- the crate-private kernel observation;
- the structured local check result;
- the evaluation timestamp.

Do not read hidden global state, the filesystem, mutable specs, environment
variables, wall-clock time, a state backend, or a provider from the pure helper.

## 9. Kernel Observation Contract

The crate-private observation should carry only decision-relevant bounded facts:

- workflow, run, and step identity;
- invocation identity;
- idempotency-key reference;
- command identity and canonical contract fingerprint;
- immutable run bundle binding;
- handler selection fingerprint and non-mock registration posture;
- observation start and completion time;
- result identity and status;
- exit-code posture;
- duration and timeout posture;
- stdout/stderr truncation flags;
- effective working-directory, environment, network, output-capture,
  redaction, and SideEffect policy posture as enums/fingerprints;
- optional stable provenance references when the requirement permits them.

It must not contain raw stdout, stderr, arguments, paths, source contents,
environment values, tokens, credentials, provider payloads, or free-form claims.

## 10. Command Contract Fingerprint

The verifier requires a deterministic canonical fingerprint over every
decision-relevant `LocalCheckCommandContract` field. The future implementation
should add one Core-owned helper using a new versioned domain separator and the
existing fixed-width length framing convention.

The fingerprint must cover command identity, kind, executable/argument
commitment, working-directory policy, environment-name policy, network policy,
timeout, SideEffect boundary, output capture, redaction, and citation posture.
Values that must not be copied into an attestation should still be committed by
the canonical fingerprint where the command contract legitimately stores them.

Tests must include a stable known vector, delimiter-collision resistance, field
change sensitivity, and canonical ordering where order is semantically irrelevant.

## 11. Verification Algorithm

The pure verifier should perform this order:

1. Validate every supplied model.
2. Recompute the requirement fingerprint and compare it with the stored value.
3. Validate stored immutable bundle manifest and definition-record integrity.
4. Confirm exact workflow/run/step identity and bundle binding.
5. Validate the pre-execution binding and match its bundle/run/step/skill context.
6. Recompute the command-contract fingerprint and match the binding exactly.
7. Match the Core-derived handler selection fingerprint and honest
   registered-unattested posture frozen by the binding.
8. Reject any source or assurance other than kernel-observed local process.
9. Reject mock, caller-supplied, external, missing, substituted, or ambiguous
   handler posture without implying handler implementation attestation.
10. Match observation, result, invocation, idempotency, status, exit, duration,
   timeout, and truncation facts.
11. Enforce accepted statuses and every policy maximum.
12. Validate binding/start/completion/evaluation ordering.
13. Evaluate `NoReuse` or maximum-age freshness at evaluation time.
14. Reconstruct the expected unverified binding from trusted inputs.
15. Recompute and compare the complete candidate binding fingerprint.
16. Return a distinct accepted record containing only bounded proof context.

Validation order should reject structural and identity failures before time or
status decisions so error behavior remains deterministic.

## 12. Accepted Record

The accepted record should preserve:

- algorithm and version;
- requirement and command-contract fingerprints;
- accepted binding fingerprint;
- immutable local-check execution binding fingerprint;
- immutable run bundle binding;
- workflow/run/step, invocation, result, and handler references;
- kernel-observed local-process assurance;
- accepted result status and exit-code posture;
- observation completion and verification timestamps;
- freshness policy and evaluated age posture;
- truncation posture;
- bounded provenance references;
- conservative sensitivity and validated redaction metadata if needed.

Record identity, if added, must remain separate from canonical proof identity.
Two accepted records over identical proof facts should share proof identity even
when stored later under distinct record IDs. Store-level record-ID uniqueness is
a later persistence invariant.

## 13. Freshness And Invalidation

- `NoReuse` means the observation must belong to the exact current invocation;
  it does not mean an old record can be relabeled current.
- Maximum age is measured from observation completion to evaluation time.
- Future evaluation, future observation, completion-before-start, overflow, and
  boundary ambiguity fail closed.
- A later consumer must reevaluate freshness at time of use; creation-time
  acceptance alone is not permanent authority.
- Any requirement, command, bundle, handler, invocation, result, policy, or
  truncation change invalidates the candidate.

The first verifier implementation does not add a cache or reusable persistence.

## 14. Error Policy

Use stable bounded codes, including distinct families for:

- `local_check_attestation.verify.requirement_mismatch`;
- `local_check_attestation.verify.bundle_mismatch`;
- `local_check_attestation.verify.command_mismatch`;
- `local_check_attestation.verify.handler_mismatch`;
- `local_check_attestation.verify.observation_mismatch`;
- `local_check_attestation.verify.result_mismatch`;
- `local_check_attestation.verify.status_not_accepted`;
- `local_check_attestation.verify.assurance_insufficient`;
- `local_check_attestation.verify.time_invalid`;
- `local_check_attestation.verify.freshness_expired`;
- `local_check_attestation.verify.policy_exceeded`;
- `local_check_attestation.verify.provenance_ambiguous`;
- `local_check_attestation.verify.binding_mismatch`.

Errors must not include raw IDs, paths, arguments, output, environment values,
timestamps supplied as secret-like strings, tokens, provider payloads, or source
contents. Verification failure must not return a partial accepted record.

## 15. Relationship To Proportional Governance

An accepted attestation may later supply one typed evidence/check fact to the
accepted proportional-governance reassessment boundary. It must not directly
select quiet, visible, approval, or denial posture.

The product remains constraint-first:

- ordinary posture should be derived from validated workload facts;
- explicit workflow, profile, policy, authority, check, SideEffect, sensitivity,
  and steward minima remain authoritative;
- relevant changes invalidate prior assessments;
- inference may recommend or escalate but may never weaken a minimum.

Operator presentation remains separate. A UI may display a quiet accepted-check
decision without changing execution disposition, while a required disclosure
remains durable even when execution does not pause.

## 16. Relationship To Capability Authority

An accepted check proves bounded check execution context. It does not grant a
tool capability, authorize context access, approve a SideEffect, or establish
actor authority. A future step-scoped capability or authority receipt may cite
an accepted attestation where policy requires it, but each owning boundary must
validate freshness and exact run/step linkage independently.

## 17. Test Plan

Future focused tests must prove:

1. exact kernel-owned inputs return one accepted record;
2. public callers cannot construct kernel observations or accepted records;
3. accepted records cannot be deserialized in the first phase;
4. requirement fingerprint tampering fails;
5. stored bundle and durable run binding mismatch fail;
6. execution binding is created before observation and commits command, handler,
   and effective policy fingerprints;
7. workflow, run, step, skill, and execution-binding cross-combinations fail;
8. a current but unbound command contract fails;
9. command identity or any command-contract field change fails;
10. a current but unbound handler fingerprint fails;
11. handler substitution, mock posture, and caller posture fail;
12. registered-unattested posture cannot be presented as implementation attestation;
13. invocation and idempotency mismatch fail;
14. result identity, status, exit, duration, timeout, or truncation mismatch fail;
15. failed, skipped, unavailable, denied, internal-error, and redaction-failed
    statuses cannot satisfy a passed-only requirement;
16. future-dated, impossible, stale, and boundary-overflow times fail;
17. exact freshness boundary behavior is deterministic;
18. policy maxima cannot be exceeded;
19. duplicate or ambiguous provenance fails;
20. candidate or execution-binding tampering fails;
21. changing the execution binding invalidates the candidate even when command
    ID and result status remain identical;
22. stable execution-binding, command, and accepted-proof vectors are pinned;
23. Debug and errors do not leak forbidden values;
24. serialization, if included, remains payload-free;
25. no execution, state, events, artifacts, CLI output, providers, or writes occur;
26. existing attestation, immutable bundle, local check, executor, report,
    proportional-governance, capability, and workspace tests continue to pass.

Compile-fail coverage should be considered for constructor/privacy guarantees.
If repository tooling does not support it without a new dependency, document
Rust privacy plus crate-internal unit coverage and defer compile-fail tooling.

## 18. Proposed Implementation Sequence

1. Add the immutable local-check execution binding core model, canonical
   command-contract fingerprinting, honest registered-handler posture, and stable
   vectors. Do not execute checks.
2. Perform a phase-level review of that binding model.
3. Add crate-private kernel observation and pure verification input over the
   accepted binding model.
4. Add the crate-private verifier and public read-only accepted record.
5. Add focused positive, mismatch, freshness, privacy, and non-leakage tests.
6. Run the full workspace validation suite.
7. Perform a phase-level verifier review.
8. Plan one opt-in DocsCheck runtime composition only after acceptance.

Keep the first code phase in `workflow-core`; do not touch executor call sites.

## 19. Deferred Work

- executor and handler integration;
- automatic checks or default handler registration;
- accepted/rejected workflow events and audit projection;
- persistence, cache reuse, record-ID conflict handling, and recovery;
- evidence/report citation and artifact gates;
- proportional-governance runtime fact composition;
- approval, capability, or authority-receipt consumption;
- schema, SDK, CLI, UI, examples, and migration behavior;
- remote, cryptographic, hardware-backed, or third-party attestation;
- hosted/distributed runners and enterprise identity.

## 20. Open Questions

- Should the first accepted record implement `Serialize`, or remain entirely
  in-memory until persistence is separately planned?
- Whether a later composition phase should incorporate accepted execution
  bindings into the immutable bundle manifest or retain a separately rooted
  execution-input ledger.
- What stronger provenance would justify an assurance beyond
  `KernelObservedLocalProcess` for handler implementation integrity?
- What is the narrowest future event vocabulary that can disclose rejection
  without turning a rejected candidate into evidence?
- Should freshness reevaluation return a separate time-of-use decision rather
  than mutating or invalidating the original accepted record?

## 21. Final Recommendation

Perform a phase-level review of the implemented pure verifier. If accepted,
plan one explicit opt-in `DocsCheck` runtime composition path. Runtime
composition must create the immutable execution binding before execution,
derive the crate-private observation from Core-owned execution, and invoke the
verifier afterward.

Do not enable automatic checks or implement persistence, events, schemas, CLI,
evidence attachment, broader providers, side effects, or writes in the review.
