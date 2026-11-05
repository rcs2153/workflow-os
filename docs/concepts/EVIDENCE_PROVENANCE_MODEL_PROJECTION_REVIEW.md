# Evidence Provenance Model And Projection Review

## 1. Executive Verdict

The current evidence foundations are reusable, but they do not yet project the
assurance boundary exposed by live testing into one durable result that report,
inspection, replay, and policy consumers can interpret consistently.

Workflow OS must not create a second attestation system. The existing models
already separate three important facts:

- `ImmutableLocalCheckExecutionBinding` commits the immutable run, command,
  policy, and selected handler before execution;
- `LocalCheckAttestationBinding` is explicitly unverified candidate data; and
- `AcceptedLocalCheckAttestation` is returned only by the crate-private
  verifier after matching a Core-owned process observation.

The missing first implementation is a payload-free **evidence assurance
projection** over those facts. It must honestly represent the current trusted
host project-validation path as operator-selected, content-bound, host-observed,
and unattested. It must not promote that path to accepted local-process
attestation merely because the selected executable exits zero.

WorkReport body integrity is a separate boundary. The existing artifact record
validates structure and duplicated identity metadata, but it does not commit to
the complete report body against an independently persisted expectation. That
hardening should follow the validator-assurance projection rather than being
folded into the same model.

## 2. Reviewed Surfaces

The review traced the following existing surfaces and their consumers:

- `local_check.rs`: closed command contracts, explicit profile resolution,
  selected executable hashing, pre-dispatch hash revalidation, bounded process
  results, and stable result references;
- `local_check_execution_binding.rs`: immutable command, handler-selection,
  policy, run, and step commitments;
- `local_check_attestation.rs` and its crate-private verifier: candidate,
  requirement, observation, and accepted-proof boundaries;
- `work_report.rs`: stable local-check citations, report construction, artifact
  metadata, record validation, and governed artifact writes;
- filesystem, SQLite, and PostgreSQL state implementations: create-only artifact
  writes through supported APIs and structurally validated reads;
- CLI inspection, continuation-context collection, executor replay, hosted
  terminal persistence, and current-authority readers that consume stored
  report artifacts.

No existing type supplies a complete result-level projection of producer,
execution, attestation, artifact-integrity, storage-owner, and external-
verifiability posture.

## 3. Existing Guarantees That Must Be Reused

### 3.1 Immutable local-check execution binding

`ImmutableLocalCheckExecutionBinding` already binds:

- immutable run-bundle identity and integrity root;
- workflow, run, step, skill, command, and version identity;
- canonical command-contract and effective-policy fingerprints;
- handler-selection metadata and fingerprint; and
- creation time before observation.

Its handler posture is deliberately `RegisteredUnattested`. Deserialization
recomputes public fingerprints, so a valid serialized binding is tamper-evident
model data, not proof that Core created it or that the selected implementation
is trusted.

### 3.2 Local-check attestation

`LocalCheckAttestationBinding` always has `Unverified` verification posture.
Its assurance and source fields are claims until the crate-private verifier
accepts exact requirement, immutable-run, execution-binding, observation,
result, and freshness context.

`AcceptedLocalCheckAttestation` is the stronger proof object. The verifier only
accepts the `KernelObservedLocalProcess` assurance and
`KernelLocalProcessRunner` source, and binds the accepted proof to the execution
binding, handler selection, invocation, result, timestamps, and requirement.

This distinction is correct and must remain intact.

### 3.3 Trusted-host project validation

The closed project-validation profile fixes the command arguments, working
directory, sanitized environment, timeout, and command contract. It hashes the
operator-selected executable and rechecks those bytes before process entry. The
trusted-host admission boundary also computes a private resolved-execution
fingerprint over the executable, arguments, working directory, environment,
timeout, and executable-content hash.

These controls prove which selected process boundary was dispatched. They do
not prove that the executable is an official Workflow OS validator, a reviewed
checkout build, or an independently attested producer. The current path must
therefore remain below `AcceptedLocalCheckAttestation` assurance.

### 3.4 WorkReport artifacts

`WorkReportArtifactRecord` validates its `WorkReport` and verifies duplicated
artifact metadata against report identity, run, terminal status, generation
time, sensitivity, and redaction posture. Supported stores reject duplicate
writes and deserialize records through validated constructors.

The metadata does not commit to all report sections, citation summaries,
incomplete work, limitations, risks, handoff notes, or high-assurance
disclosures. A storage owner can edit those fields and recompute any
report-owned self-commitment. A useful integrity check therefore needs an
expected complete-body commitment anchored outside the mutable artifact body.

## 4. Assurance Dimensions Must Remain Distinct

The implementation must not collapse the following dimensions into one ordered
enum:

| Dimension | Current truthful posture | Stronger future posture |
| --- | --- | --- |
| Producer selection | operator-selected, exact bytes content-bound | reviewed or independently verified producer |
| Execution observation | trusted host observed one bounded process result | separately attested execution environment |
| Attestation | unavailable for the public validator path | crate-private accepted local-process proof |
| Report-body integrity | structurally valid and identity-consistent | complete body commitment verified against retained anchor |
| Storage owner | trusted local or deployment owner | independently anchored or signed evidence |
| External verifiability | unavailable | independently verifiable receipt or witness |

These dimensions are related but not substitutable. Content binding cannot
upgrade producer provenance. Host observation cannot upgrade attestation.
Report-body hashing cannot remove storage-owner trust when the same owner can
rewrite the artifact and its anchor.

## 5. Compatibility And Duplication Findings

### Reuse directly

- `ImmutableLocalCheckHandlerSelection` and its selection fingerprint;
- `ImmutableLocalCheckExecutionBinding` and its binding fingerprint;
- `LocalCheckAttestationAssurance`, candidate posture, accepted proof, and proof
  fingerprint;
- `LocalCheckResultId`, status, truncation posture, and result references;
- immutable run-bundle, invocation, command-contract, and policy fingerprints;
- existing report citation and stable-reference mechanisms;
- existing state-store validation and replay error boundaries.

### Do not reuse as stronger proof

- `RegisteredUnattested` must not be interpreted as a reviewed producer;
- the public recomputable binding fingerprint is not Core provenance;
- `AuthoritativeLocalCheckHandler` is an internal interface name, not an
  assurance claim about the selected executable;
- `LocalCheckResult` and `LocalCheckResultReference` are outcomes and pointers,
  not independent evidence;
- a structurally valid `WorkReportArtifactRecord` is not complete-body integrity
  proof.

### Missing model concepts

The smallest missing concept is a closed, payload-free projection that records:

- the exact local-check result and immutable execution-binding references;
- selected-handler and resolved-execution commitments;
- producer-selection posture;
- execution-observation posture;
- optional accepted-attestation proof reference, absent for the current path;
- a deterministic projection fingerprint; and
- an explicit statement that no external-verifiability claim exists.

This projection should reference existing proof objects. It should not copy raw
paths, command output, environment values, report prose, credentials, or
provider payloads.

## 6. First Implementation Phase

Implement a **model-only local-check evidence assurance projection** in Core.
The exact type name may follow repository conventions, but the model must:

1. represent the current operator-selected, content-bound, host-observed,
   unattested posture without ambiguity;
2. bind the projection to the immutable execution binding, handler selection,
   resolved-execution commitment, result identity/status, and immutable run;
3. optionally reference an existing `AcceptedLocalCheckAttestation` proof, but
   reject contradictory combinations and never construct that proof itself;
4. use private fields, validated constructors, safe `Debug`, validated serde,
   bounded closed vocabulary, domain separation, and deterministic framing;
5. prevent a caller-selected path, executable name, zero exit, or recomputable
   public fingerprint from upgrading assurance;
6. expose a monotonic minimum-assurance comparison suitable for a later narrow
   gate without adding a policy DSL; and
7. remain payload-free and independent of filesystem, SQLite, PostgreSQL, CLI,
   schema, SDK, executor, and hosted integration.

The model should use separate closed fields for producer, execution, and
attestation posture. It should not encode the entire table in one ordinal whose
comparison could accidentally treat different dimensions as equivalent.

## 7. Required Model Tests

- the current validator projection round-trips with the exact weak posture;
- a zero exit does not change producer or attestation posture;
- changing executable, resolved-execution, handler-selection, command,
  immutable-run, result, or observation commitments changes the projection;
- an accepted-attestation reference must match the same execution binding,
  handler, result, and immutable run;
- unattested and accepted-attestation combinations cannot be confused;
- serde tampering and unknown vocabulary fail closed;
- Debug and errors disclose no paths, hashes, payloads, summaries, or command
  output;
- minimum-assurance checks are dimension-specific and monotonic;
- the model contains no raw output, arbitrary claims, or mutable report text.

## 8. Subsequent Runtime Sequence

After model review, continue in this order:

1. compose the current trusted-host project-validation result into the weak
   assurance projection and persist it with the existing dispatch/result truth;
2. expose the projection through bounded inspection and local-check report
   citation metadata;
3. add one narrow fail-closed minimum-assurance consumer for the closed
   validation profile;
4. specify and implement the versioned complete WorkReport body commitment;
5. anchor the expected commitment in existing terminal artifact persistence
   state outside the mutable report body;
6. enforce the commitment across every read, list, inspect, continuation,
   duplicate-reconciliation, recovery, and replay path;
7. expose explicit legacy, storage-owner, and external-verifiability posture;
8. obtain independent security review before release admission resumes.

## 9. Strict Non-Goals

This review does not authorize:

- runtime code, persistence, events, CLI, schema, or SDK changes;
- arbitrary commands, executable discovery, or ambient handler registration;
- a generic producer registry or policy language;
- automatic local-check execution;
- signatures, transparency logs, external witnesses, TPM claims, or remote
  attestation;
- hosted identity, provider expansion, writes, or publication;
- claiming storage-owner tamper resistance from a local hash; or
- implementing the separate cold-recipient continuation/handoff UX finding.

## 10. Review Decision

Proceed with the model-only local-check evidence assurance projection. Do not
start WorkReport commitment persistence or broaden runtime execution first.

The current architecture is stronger than the live tests initially suggest:
the kernel already has the critical pre-execution binding and accepted-
attestation separation. The defect is that current result, report, inspection,
and replay surfaces cannot yet carry that distinction end to end. The proposed
projection closes that gap incrementally while preserving the existing trust
boundaries.

## 11. Governed Phase Record

- Workflow: `dg/review`
- Run: `run-1793849420131486000-2`
- Approval: `approval/run-1793849420131486000-2/review-scope-approved`
- Presentation: `presentation/74156a7213c3e376`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Runtime implementation: not performed in this phase
- External work: code inspection, review authoring, and validation ran outside
  the kernel under the approved scope

## 12. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Governed phase close: `Completed`; 39 events, one approval, zero retries,
  zero escalations, and one proof-enforced approval-presentation record.
- Runtime tests were not run because this phase changed documentation only and
  explicitly did not authorize runtime implementation.
