# Evidence Provenance And Report Integrity P0 Plan

Status: P0 plan. Truth-boundary correction is active; runtime hardening remains
unimplemented until separately reviewed.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Current Product Contract](../user-guide/current-product-contract.md)
- [Threat Model](../security/THREAT_MODEL.md)
- [Independent Local Check Attestation Plan](independent-local-check-attestation-plan.md)
- [Immutable Local Check Execution Binding Core Model Review](../concepts/IMMUTABLE_LOCAL_CHECK_EXECUTION_BINDING_CORE_MODEL_REVIEW.md)
- [Trusted-Host Project-Validation Profile Plan](trusted-host-project-validation-profile-plan.md)
- [Trusted-Host Terminal WorkReport Artifact Plan](trusted-host-terminal-work-report-artifact-plan.md)

## 1. Executive Decision

Workflow OS must distinguish evidence that the kernel can verify from claims it
accepts from a host, producer, or storage owner.

Two live tests expose the same missing product invariant:

1. the trusted host content-binds and dispatches an operator-selected executable,
   but an operator can supply a different executable that returns success; and
2. a storage owner can edit a persisted `WorkReport` summary into another
   structurally valid value, after which current inspection and replay accept
   the edited record.

The first result proves dispatch integrity inside the selected host boundary. It
does not prove that the executable is an official Workflow OS validator or that
its success is independently trustworthy. The second result proves that current
artifact reads validate structure and identity fields. It does not provide a
commitment over the complete report body or cryptographic tamper evidence
against the storage owner.

This is a P0 evidence-credibility boundary. It preempts publication, public
positioning changes that broaden evidence claims, and broader evidence-consuming
runtime work. It does not invalidate the existing event, approval, dispatch, or
report models. It requires those models to carry and enforce honest assurance.

## 2. Required Product Invariant

Every evidence-bearing result must disclose and preserve the strongest assurance
the kernel can actually establish, without promotion by naming, caller choice,
or structurally valid serialization.

Specifically:

- content-bound dispatch is not producer provenance;
- producer provenance is not execution attestation;
- structural validity is not artifact-body integrity;
- artifact-body integrity is not storage-owner tamper resistance;
- local durability is not independent evidence;
- replay may reuse only the original assurance posture and may never upgrade it;
- inspection must expose these distinctions in bounded, machine-readable form;
- policies and report consumers may require a minimum assurance and fail closed
  when it is unavailable.

## 3. Assurance Taxonomy

The implementation should reuse existing check-attestation, execution-binding,
evidence, receipt, and report vocabulary. It must not create a parallel evidence
system. The following conceptual levels are required even if existing type names
are reused:

| Level | What Workflow OS can claim | What it cannot claim |
| --- | --- | --- |
| Structurally valid | The record satisfies its schema and internal field invariants. | That the content is original or true. |
| Caller asserted | A caller supplied the claim through an accepted interface. | That the producer or execution is trusted. |
| Host observed | A named host observed a bounded action or result. | That the host or selected executable is independently trustworthy. |
| Content bound | The exact executable, request, result, or artifact bytes are committed. | Who produced the bytes or whether the implementation is approved. |
| Registered producer | A reviewed producer identity and version are selected by Core policy. | That the producer ran in an independently attested environment. |
| Attested execution | Accepted attestation binds producer, invocation, result, time, and immutable run context. | Storage-owner tamper resistance unless the attestation is anchored outside that store. |
| Externally verifiable | A signature, transparency witness, or equivalent trust anchor can be verified independently of the mutable state owner. | Universal correctness of the underlying work. |

Assurance is monotonic only when new proof is validated. A result must not move
up this table because a field is named `authoritative`, because a closed profile
was selected, or because a replayed record remains parseable.

## 4. Validator Provenance Boundary

### Current behavior

The project-validation profile accepts an explicit `--workflow-os-executable`
path. Core fixes the command arguments, working directory, environment, timeout,
and command contract. It hashes the selected executable and rejects replacement
after resolution. The dispatch record can therefore prove which bytes were
selected and invoked.

The path does not currently prove that those bytes came from a Workflow OS
release, the current checkout, a reviewed build, or another trusted producer.
An executable that accepts `validate`, performs unrelated writes, and exits zero
can satisfy the current process-result contract. Calling that outcome an
independently authoritative validation would be false.

### Required correction

1. Preserve the existing exact executable-content and invocation commitment.
2. Add or project an explicit producer-assurance posture from the existing local
   check execution-binding and attestation models.
3. Classify the current public/operator path as operator-selected,
   content-bound, and unattested unless stronger proof is supplied.
4. Prevent that posture from satisfying policies that require a registered or
   independently attested validator.
5. Bind any future trusted-validator upgrade to one reviewed provenance source,
   such as a release manifest, current-checkout build record, package signature,
   or externally verified attestation. Executable name and path are never proof.
6. Persist the assurance posture with the check result and expose it through
   inspection and WorkReport citation metadata.
7. Preserve exact assurance on replay; replay cannot rediscover or infer a
   stronger producer.

The first implementation should be a model and projection correction over the
existing closed profile. It must not add arbitrary commands, a plugin registry,
hidden executable discovery, package installation, or a new trust service.

## 5. WorkReport Artifact Integrity Boundary

### Current behavior

`WorkReportArtifactRecord` validates the report and checks that duplicated
metadata fields match report identity, run identity, status, generation time,
sensitivity, and redaction posture. Filesystem storage is create-only through
supported APIs, and SQLite/PostgreSQL rows preserve indexed identity.

The artifact metadata does not commit to the complete serialized report body.
A storage owner can edit report prose while preserving all validated identity
fields. Deserialization, `inspect`, list, and terminal replay then accept the
new structurally valid record. The current threat model already states that v0
local state lacks cryptographic tamper evidence, but inspection does not expose
this report-specific assurance limit.

### Required correction

1. Define one versioned, domain-separated canonical commitment over the complete
   report body, including sections, citations, incomplete work, limitations,
   risks, handoff notes, high-assurance disclosure, sensitivity, and redaction
   metadata.
2. Store the report-body commitment in an artifact envelope field that cannot be
   derived from mutable prose during ordinary read validation without comparison
   to an independently persisted expected value.
3. Anchor the expected commitment in terminal report-artifact persistence state
   outside the report body. Prefer an existing immutable event, receipt, or
   create-only artifact index rather than a second report-owned self-assertion.
4. On read, list, inspect, metadata projection, duplicate reconciliation, missing
   artifact recovery, and replay, compare the complete body against the expected
   commitment before returning report content or accepting equality.
5. Reject report-only edits with one stable, non-leaking integrity error. Do not
   print the expected hash, actual hash, edited summary, path, or payload.
6. Version the envelope and define explicit legacy posture. Existing artifacts
   without a body commitment must be reported as legacy/unverified or migrated
   through a separately reviewed process; they must not be silently labeled
   verified.
7. Expose bounded integrity posture in human and JSON inspection.

This correction detects report-body substitution when the expected commitment
remains intact. It does not make a local database cryptographically
tamper-resistant against an actor who can rewrite both the artifact and every
local anchor.

## 6. Storage-Owner Trust And High-Assurance Follow-Up

The local filesystem and SQLite backends run inside one administrative trust
domain. PostgreSQL can separate process and database roles but still requires a
deployment-owned trust decision. None currently supplies an independent
signature or transparency witness for WorkReports.

After the body-commitment correction, the product must still say:

- ordinary local posture detects corruption and report-only substitution against
  the retained anchor;
- a privileged owner of all state remains capable of rewriting the record and
  its local anchor;
- high-assurance workflows may require a signing key outside the state owner,
  hardware-backed identity, append-only external witness, or equivalent
  independently verifiable receipt;
- adding that trust anchor is a separate reviewed security phase, not a hidden
  property of hashing.

## 7. Inspection, Reporting, And Policy

Inspection and report surfaces should expose bounded fields equivalent to:

```text
producer_assurance: operator_selected_content_bound_unattested
execution_assurance: host_observed
artifact_integrity: body_commitment_verified
storage_owner_posture: trusted_local_owner
external_verifiability: unavailable
```

The exact vocabulary should reuse existing types where possible. Output must be
payload-free and stable. Policies may later require minimum assurance for a
specific action, but this P0 must not introduce a broad policy DSL.

WorkReports must not describe an operator-selected executable result as
independent evidence. Citations should retain the originating check's assurance
posture. If a report body fails integrity validation, inspection and replay must
fail closed rather than omitting the report or presenting stale event truth as
if the artifact were valid.

## 8. Implementation Sequence

1. **Truth correction and proof taxonomy.** Update the roadmap, product contract,
   threat model, and public positioning so current claims name the host and
   storage-owner boundaries.
2. **Focused model review.** Map the taxonomy onto existing local-check
   attestation and execution-binding types. Add only missing assurance fields or
   projections.
3. **Validator-assurance projection.** Persist and expose the current
   operator-selected, content-bound, unattested posture. Add fail-closed minimum
   assurance comparison without broadening executable input.
4. **Report-body commitment model.** Specify canonicalization, versioning,
   domain separation, legacy posture, and non-leaking errors.
5. **Persistence anchor.** Bind the expected body commitment into existing
   terminal artifact state and all supported stores atomically or with explicit
   crash reconciliation.
6. **Read/replay enforcement.** Enforce the commitment on every artifact read,
   list, inspect, metadata projection, duplicate reconciliation, recovery, and
   replay path.
7. **Product projection.** Expose bounded producer, execution, artifact, storage,
   and external-verifiability posture in human and JSON output.
8. **Independent security review.** Exercise executable substitution, malicious
   success, report-only edits, anchor edits, legacy artifacts, restart, replay,
   migration, and privacy behavior before release admission resumes.
9. **High-assurance trust anchor planning.** Only after the local correction is
   accepted, decide whether signatures or an external witness are required for a
   later assurance profile.

## 9. Required Tests

### Validator provenance

- the real validator succeeds and records exactly one dispatch;
- an operator-selected alternate executable can produce only the explicitly
  unattested posture, even if it exits zero;
- executable bytes changed after resolution fail before dispatch;
- path or executable name cannot upgrade assurance;
- an attestation or registered-producer requirement rejects the unattested path;
- replay preserves the original assurance without redispatch or promotion;
- marker files or other side effects produced by a selected executable are not
  misreported as kernel-controlled behavior.

### Report integrity

- changing only a summary or section body fails read, list, inspect, and replay;
- changing citations, limitations, risks, handoff notes, or disclosures fails;
- changing metadata without matching report identity still fails as today;
- recomputing a report-owned field without the external expected anchor fails;
- missing, duplicate, conflicting, and legacy anchors have explicit postures;
- exact unchanged replay remains idempotent;
- crash recovery cannot write an artifact under an uncommitted expectation;
- all failures use stable errors without payload or hash leakage;
- filesystem, SQLite, and PostgreSQL stores satisfy the same conformance suite.

## 10. Strict Non-Goals

This P0 does not authorize:

- arbitrary local commands or executable discovery;
- treating the host as hostile while it executes a process;
- a new sandbox, container runtime, or OpenShell integration;
- release signing, Sigstore, hardware keys, or a transparency service in the
  first slice;
- encryption at rest, DLP, retention, enterprise identity, or hosted admin;
- broad provider writes, nested harnesses, agent teams, or autonomous scheduling;
- storing raw command output or report prose in events;
- retroactively claiming old evidence was independently verified; or
- publication before the truth correction and required runtime slices are
  reviewed.

## 11. Acceptance

The P0 is complete only when:

- every selected validator result carries honest producer and execution
  assurance;
- operator-selected success cannot satisfy a stronger assurance requirement;
- every new WorkReport artifact has a complete body commitment anchored outside
  its mutable report body;
- report-only mutation fails inspection and replay;
- legacy artifacts have an explicit non-verified posture;
- local storage-owner trust remains disclosed and is not mislabeled as
  cryptographic tamper resistance;
- human and JSON inspection expose bounded assurance posture;
- all supported stores and replay paths pass the required integrity tests;
- public docs use the same taxonomy; and
- independent security review accepts the exact release candidate.

