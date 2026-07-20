# Independent Local Check Attestation Plan

Status: Core model and deterministic-binding blocker fix accepted. Pure
verifier planning is next; runtime integration remains unimplemented. Existing local check results and references remain bounded
outcome/citation models, not independent proof. The new binding model is always
explicitly `unverified`; no verifier, persistence, event, executor enforcement,
schema, or CLI behavior is implemented. Focused planning review is recorded in
[Independent Local Check Attestation Plan Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_PLAN_REVIEW.md).
The model review blockers are recorded in
[Independent Local Check Attestation Core Model Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_CORE_MODEL_REVIEW.md).
The fix is recorded in
[Independent Local Check Attestation Core Model Blocker Fix Report](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_CORE_MODEL_BLOCKER_FIX_REPORT.md).
Focused fix review is recorded in
[Independent Local Check Attestation Core Model Blocker Fix Review](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_CORE_MODEL_BLOCKER_FIX_REVIEW.md).

## 1. Executive Summary

Workflow OS can model allowlisted local check contracts, execute selected
handlers through explicit registration, return bounded redaction-safe results,
and cite stable result references in reports. Those foundations do not prove
that a real check ran against the immutable definitions and execution context
claimed by a workflow run.

This plan defines an independent local check attestation boundary. A future
accepted attestation will bind a kernel-observed check invocation and structured
result to the exact command contract, immutable run bundle, workflow/run/step,
handler posture, execution policy, and freshness window. Raw command output is
not evidence, mock success is not attestation, and caller-supplied success cannot
satisfy an independent-check requirement.

The first implementation added core requirement and payload-free binding models
only. It does not execute checks or claim independent proof. Implementation
details are recorded in
[Independent Local Check Attestation Core Model Report](../concepts/INDEPENDENT_LOCAL_CHECK_ATTESTATION_CORE_MODEL_REPORT.md).

## 2. Goals

- Distinguish a check outcome from proof of how that outcome was produced.
- Define typed independent-check requirements.
- Bind attestations to exact immutable run and command-contract context.
- Bind the observed invocation, handler posture, result, and freshness policy.
- Make mock, caller-asserted, stale, mismatched, and unverifiable results
  deterministically insufficient.
- Reuse existing local check, immutable bundle, event, evidence, report,
  approval, capability, and proportional-governance foundations.
- Keep records payload-free and redaction-safe.
- Prepare one explicit local DocsCheck proof path without enabling it yet.

## 3. Non-Goals

This plan does not authorize:

- implementation during planning;
- automatic or default local check execution;
- arbitrary shell commands or user-supplied command text;
- new command families;
- ambient handler registration;
- treating mock handlers as real execution;
- treating caller assertions as independent proof;
- raw stdout, stderr, logs, transcripts, or command-output evidence;
- workflow schema changes;
- CLI or UI behavior;
- persistence, report artifacts, or automatic evidence attachment;
- provider calls or writes;
- network-enabled checks;
- cryptographic signing, remote attestation, TPM claims, or notarization;
- hosted runners, distributed workers, enterprise identity, RBAC, or IdP;
- automatic approvals, reasoning lineage, or release posture changes.

## 4. Current Foundation And Gap

Implemented foundations include:

- `LocalCheckCommandContract` with allowlisted command kind, fixed executable and
  arguments, working-directory, environment, network, timeout, SideEffect,
  output-capture, redaction, and citation posture;
- explicit non-default handler registration;
- `LocalCheckProcessRunner` and selected real/injected runner boundaries;
- `LocalCheckResult` with bounded summaries and structured status;
- `LocalCheckResultReference` with workflow/run/event context;
- WorkReport local check citation vocabulary;
- immutable run bundles and explicit executor binding;
- approval/resume resolved-context commitments;
- opt-in proportional-governance retry/resume reassessment.

The gap is provenance. Today a valid `LocalCheckResult` can be constructed from
caller-supplied fields, and a valid reference can be derived from it. Validation
proves shape and privacy, not that the kernel observed the intended process,
that the correct handler ran, or that the result belongs to the immutable run
context. A report citation preserves that same limitation.

## 5. Source-Of-Truth Boundaries

The following concepts must remain distinct:

| Concept | Meaning | Not sufficient for |
| --- | --- | --- |
| `LocalCheckCommandContract` | Validated declaration of an allowed check | Proof that it ran |
| `LocalCheckResult` | Bounded structured outcome | Independent provenance |
| `LocalCheckResultReference` | Stable citation pointer | Independent provenance |
| `EvidenceReference` | Pointer to separately governed evidence | Proof merely because it exists |
| `WorkReportCitation` | Report reference to an existing record | Recreating or upgrading evidence |
| Immutable run bundle | Frozen run definitions and selected posture | Proof of external/local execution |
| Check attestation requirement | Minimum proof required at a gate | Proof by itself |
| Accepted check attestation | Verified payload-free binding to observed execution | Raw output storage or universal truth |

An attestation is accepted only through a separately reviewed verifier boundary.
Constructing or deserializing an attestation-shaped value must not make it
authoritative by itself.

## 6. Candidate Core Model

The smallest first model set should be evaluated during implementation:

- `LocalCheckAttestationId`
- `LocalCheckAttestationAlgorithm`
- `LocalCheckAttestationAssurance`
- `LocalCheckAttestationSource`
- `LocalCheckAttestationRequirement`
- `LocalCheckAttestationBinding`
- `LocalCheckAttestationFreshnessPolicy`

The implementation should omit any type that does not protect an immediate
invariant. Public construction should follow existing private-field,
validated-constructor, safe-`Debug`, and validated-serde patterns.

Candidate assurance vocabulary:

- `caller_asserted`: valid disclosure vocabulary, never independently trusted;
- `mock_observed`: test/demo vocabulary, never independently trusted;
- `kernel_observed_local_process`: future accepted local assurance after a
  verifier proves the complete binding;
- `external_verifier`: future vocabulary only, not accepted in v0.

The model must not imply cryptographic, hardware-backed, or third-party trust.

## 7. Requirement Model

A future `LocalCheckAttestationRequirement` should express:

- required command/check identity;
- minimum assurance;
- accepted result statuses, normally `Passed` only;
- exact immutable-run binding requirement;
- exact workflow/run/step requirement;
- required handler identity posture;
- maximum result age or explicit no-cache posture;
- whether truncation is allowed;
- network and SideEffect maxima;
- required provenance references;
- conservative sensitivity and redaction metadata.

Requirements must be monotonic. A workflow, policy, approval, capability, or
steward minimum may make proof stricter; inference or a local caller may not
downgrade it.

## 8. Payload-Free Binding

The attestation binding should use a versioned fixed-width framed fingerprint
over every decision-relevant field, including:

- attestation algorithm and version;
- command contract identity and canonical contract fingerprint;
- immutable bundle ID, version, and integrity root;
- workflow ID/version, run ID, and step ID;
- invocation ID and idempotency key reference;
- handler identity and handler-attestation posture;
- working-directory, environment, network, timeout, output-capture, redaction,
  and SideEffect policy posture;
- result status, exit-code posture, duration, and truncation flags;
- observation start/completion time;
- requirement/freshness policy identity.

Do not include raw command output, summaries, arguments, paths, environment
values, source contents, tokens, credentials, or provider payloads. Canonical
contract and bundle fingerprints should bind those validated definitions without
copying them into the attestation.

## 9. Future Verifier Boundary

The future pure verifier should accept only explicit inputs:

- the attestation requirement;
- stored immutable run bundle and durable run binding;
- exact command contract selected for the run;
- kernel-owned invocation observation;
- structured `LocalCheckResult` produced from that observation;
- handler identity/registration posture;
- evaluation time.

It should verify exact identity alignment, accepted status, source assurance,
policy maxima, invocation idempotency, temporal ordering, and freshness before
returning an accepted binding.

The verifier must reject:

- caller-created or mock results when independent assurance is required;
- missing or changed immutable bundle binding;
- command contract or handler substitution;
- workflow/run/step mismatch;
- stale, future-dated, or temporally impossible observations;
- network or SideEffect posture beyond the requirement;
- failed, skipped, unavailable, internal-error, policy-denied, or
  redaction-failed results when `Passed` is required;
- duplicate or ambiguous provenance references.

## 10. Freshness And Cache Semantics

Check attestation should follow the accepted build-cache invariant:

- identical immutable run input, command contract, handler posture, invocation,
  result, and freshness policy produce the same binding;
- any decision-relevant change invalidates the binding;
- a valid old result does not satisfy a newer run or step;
- freshness is evaluated at time of use, not only when the record is created;
- cache reuse is allowed only when an explicit requirement permits it and the
  immutable input root and all proof context match exactly.

The first implementation should model freshness policy but not implement a
cache or automatic reuse.

## 11. Runtime And Event Semantics

Later runtime integration should be explicit and opt-in:

1. prepare the immutable run and command contract context;
2. record an invocation request before process start;
3. execute through the selected explicit handler/runner;
4. create a structured result from kernel-observed process output;
5. verify and bind the attestation;
6. append a bounded accepted/rejected event;
7. enforce the requirement before the governed action or closure gate;
8. cite the stable attestation/result references in reports without copying
   output.

An attestation failure must not be converted into a passing check, fabricated
evidence, or a misleading project diagnostic. It must fail before any action
whose gate requires the attestation.

Candidate event vocabulary belongs to a separate phase. Events should contain
typed posture and stable references only.

## 12. Persistence And Idempotency

Persistence is deferred. A future store should be create-only or exact-idempotent
by run, step, invocation, and attestation identity. Conflicting rebinding must
fail closed. Partial publication must not create an accepted record.

Restart behavior should rehydrate the durable invocation and immutable bundle,
then reverify time-of-use freshness. It must not rerun a check silently or
upgrade an unverified result during recovery.

## 13. Privacy And Redaction

- Attestations store references, typed posture, timestamps, and fingerprints.
- Raw stdout/stderr, bounded summaries, command transcripts, source contents,
  environment values, paths, and provider payloads are excluded.
- IDs, hashes, paths, actors, resources, and handler references are potentially
  sensitive and must be redacted from `Debug` and errors.
- Serialization must not carry raw output or secret-like metadata.
- Deserialization errors must use fixed non-leaking messages.
- Attestations may be sensitive even when the check is read-only.

## 14. Relationship To Evidence And Reports

An accepted attestation may later be cited by an `EvidenceReference`, audit
projection, approval, capability prerequisite, or WorkReport. Those consumers
must not recreate the attestation, copy output, or infer stronger assurance than
the accepted record declares.

`LocalCheckResultReference` remains useful for ordinary result disclosure.
Independent gates should require an accepted attestation reference in addition
to or instead of a result reference. Command-output evidence remains separately
deferred.

## 15. Failure Codes

Future stable errors should distinguish:

- requirement invalid;
- assurance insufficient;
- mock or caller assertion not accepted;
- immutable bundle binding missing or mismatched;
- command contract mismatch;
- invocation or handler mismatch;
- workflow/run/step mismatch;
- status not accepted;
- freshness missing, stale, or invalid;
- policy posture exceeded;
- binding fingerprint mismatch;
- persistence conflict or corruption.

Errors must not echo IDs, hashes, paths, command text, output, environment
values, source snippets, credentials, or payloads.

## 16. Test Plan

Future tests should prove:

- all model vocabulary is representable without claiming runtime proof;
- valid requirements and bindings round trip through serde;
- invalid serialized models fail closed;
- fingerprint vectors are stable and delimiter-safe;
- every decision-relevant field changes the binding;
- raw output and forbidden payload fields cannot be stored;
- `Debug`, serialization failures, and validation errors do not leak values;
- caller-asserted and mock results cannot satisfy independent requirements;
- exact immutable bundle, workflow/run/step, invocation, command, and handler
  alignment is required;
- stale or future-dated results fail;
- failed/skipped/unavailable/error results cannot satisfy pass requirements;
- exact retry is idempotent and changed retry context fails;
- no accepted event or record exists after verifier or persistence failure;
- existing local check, immutable bundle, executor, evidence, WorkReport,
  capability, approval, and SideEffect tests remain green.

## 17. Proposed Implementation Sequence

1. Add requirement, assurance/source, freshness, and payload-free binding core
   models only.
2. Perform focused maintainer review.
3. Add a pure verifier over explicit kernel-observed invocation input,
   structured result, immutable bundle, and command contract.
4. Perform focused maintainer review.
5. Add create-only local persistence and bounded event vocabulary.
6. Integrate one explicit non-default DocsCheck path against an immutable local
   run.
7. Add report/evidence citation support for accepted references only.
8. Review time-of-use enforcement before any default registration, schema/CLI
   exposure, or broader provider mutation.

Each item is a separate governed phase. The next implementation prompt should
target item 1 only.

## 18. Open Questions

- Should `kernel_observed_local_process` require a production-shaped runner
  implementation identity before it can satisfy a requirement?
- What canonical fingerprint should bind `LocalCheckCommandContract`?
- Should result duration participate in identity or only disclosure?
- What maximum clock skew is acceptable for local freshness checks?
- Should cached attestations ever cross run boundaries when immutable roots are
  equal?
- Which event owns the stable invocation observation reference?
- Should an accepted attestation have a distinct citation target from
  `LocalCheckResultReference`?
- When should handler binary/version identity enter the immutable run bundle?

## 19. Final Recommendation

Proceed next with **independent local check attestation core model only**.

Do not execute checks, register handlers by default, add schemas or CLI behavior,
persist records, attach command output, invoke providers, add writes, or claim
independent proof until the model and later verifier are separately reviewed.
