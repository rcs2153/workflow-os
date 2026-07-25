# Canonical Local-Check Declaration And Immutable-Bundle Derivation Plan

Status: planning complete. Typed step-scoped declaration vocabulary and the
canonical declaration-set record/pure resolver are implemented. Immutable-run
bundle publication remains next. The implemented slices do not execute checks,
convert structural coverage into aggregate governance posture, reassess
proportional governance, or add an executor checkpoint.

Related foundations:

- [Evidence And Check Obligation-Set Aggregation Plan](evidence-check-obligation-set-aggregation-plan.md)
- [Independent Local Check Attestation Plan](independent-local-check-attestation-plan.md)
- [Immutable Run Bundle Boundary Plan](immutable-run-bundle-boundary-plan.md)
- [Local Check Governance Structural Coverage Blocker Fix Review](../concepts/LOCAL_CHECK_GOVERNANCE_STRUCTURAL_COVERAGE_BLOCKER_FIX_REVIEW.md)

## 1. Executive Summary

Workflow OS can independently verify one `DocsCheck` execution and evaluate
exact coverage of a caller-supplied local-check candidate set. That coverage
is intentionally non-authoritative because no validated definition currently
declares the complete set of checks a step must satisfy.

The next boundary is a canonical, typed, step-scoped local-check requirement
that is validated with the workflow and deterministically frozen into the
immutable run bundle before execution. The bundle, not a runtime caller,
repository inference result, mutable registry, or model opinion, must become
the source of the obligation set used by later coverage and governance gates.

This plan does not implement that boundary. It defines the declaration fields,
source hierarchy, canonical record, deterministic derivation, compatibility,
privacy, and small implementation sequence needed to make it safe.

## 2. Goals

- Give workflow authors one typed way to declare local-check obligations for a
  specific step.
- Resolve each declaration against an allowlisted local-check command contract.
- Represent required and optional obligations without weakening executed
  failures.
- Bind independent-attestation requirements, accepted statuses, freshness,
  truncation, network, and SideEffect maxima.
- Derive one canonical declaration-set record per step.
- Freeze the declaration record and exact command-contract fingerprints into
  the immutable run bundle before `RunCreated`.
- Make ordering irrelevant and every decision-relevant change invalidate the
  declaration-set fingerprint and bundle root.
- Preserve safe onboarding inference as recommendation input, never runtime
  authority.
- Prepare a later authoritative adapter from frozen declarations to the
  existing structural coverage evaluator.

## 3. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- local-check execution, handler registration, or shell command expansion;
- automatic or default checks;
- aggregate `GovernanceWorkloadEvidenceCheckPosture` conversion;
- proportional-governance reassessment;
- executor checkpoints, approval gates, retry behavior, or run semantics;
- treating skill evaluation prose, policy strings, package scripts, CI files,
  repository metadata, or model inference as declarations;
- persistence or event changes beyond the existing immutable-bundle store in
  the later derivation phase;
- evidence creation, report attachment, artifacts, CLI rendering, or UI;
- provider calls, SideEffects, writes, hosted behavior, enterprise identity,
  or release posture changes.

## 4. Current Foundation And Confirmed Gap

Current workflow steps declare skills, policy requirements, approval, retry,
escalation, timeout, mappings, and terminal behavior. They do not declare local
checks.

Current skill `evaluation_criteria` are human-readable name/description pairs.
They are documentation, not executable or enforceable check requirements.

Current policy effects cover local access, external read, approval, bounded
retry, escalation, and attempt caps. Arbitrary policy strings are rejected;
none of the supported effects declares a local check.

Current governance strictness profiles are disclosed but not enforced. They
cannot supply authoritative check obligations.

Current immutable bundles freeze canonical workflow, resolved skill, and
referenced policy records plus bounded execution and handler posture. They do
not freeze local-check declaration records or command-contract fingerprints.

Current attestation composition can prove one exact `DocsCheck` result. Current
structural coverage can prove exactness only relative to a supplied unresolved
candidate set. Promoting that candidate to aggregate governance authority
would therefore be false governance.

## 5. Source-Of-Truth Hierarchy

The future source hierarchy is:

1. **Workflow step declarations** select concrete local-check obligations.
2. **Typed skill, policy, profile, or steward constraints** may later make a
   selected obligation stricter or require an additional typed obligation.
3. **Canonical resolution** validates the complete merged set against the
   allowlisted command-contract inventory.
4. **Immutable-bundle derivation** freezes the resolved set and exact command
   contract fingerprints before execution.
5. **Runtime consumers** read only that frozen set.

The first implementation should support item 1 and the deterministic parts of
items 3 and 4. Existing skill criteria, policy effects, profiles, and steward
models must contribute nothing until they gain separately reviewed typed
fields.

Repository metadata, package scripts, CI configuration, and model analysis may
recommend declarations during onboarding. A recommendation becomes authority
only after governed authoring, validation, and immutable-bundle publication.
Pure inference must never silently add, remove, or downgrade an enforcement
obligation.

## 6. Candidate Step Declaration

Add a future `local_check_requirements` collection to `StepDefinition`. Each
entry should contain the smallest complete v1 declaration:

- `id`: stable identifier unique within the step;
- `command_id`: exact allowlisted `LocalCheckCommandId`;
- `requirement_level`: `required` or `optional`;
- `minimum_assurance`: initially only `kernel_observed_local_process` may
  satisfy an independent requirement;
- `accepted_statuses`: non-empty unique bounded set, initially normally
  `passed` only;
- `freshness`: explicit maximum age or no-cache posture;
- `exact_immutable_run_binding_required`: must be true in v1;
- `truncation_allowed`: explicit boolean;
- `network_maximum`: disabled in v1; and
- `side_effect_maximum`: bounded by the selected command contract and never
  source-write-capable.

Do not accept executable text, arguments, working directories, environment
values, output paths, raw evidence, summaries, or handler names in the workflow
declaration. Those belong to separately validated allowlisted contracts and
runtime selection boundaries.

The declaration ID is a source locator, not proof identity. Canonical proof
identity must derive from the validated declaration, command contract, step,
and immutable bundle.

## 7. Validation Rules

Validation must reject:

- empty or duplicate requirement IDs within a step;
- duplicate semantic obligations under different IDs;
- unknown or unresolved command IDs;
- empty or duplicate accepted statuses;
- assurance below the independent requirement;
- missing exact immutable-run binding;
- zero or unbounded freshness windows;
- network posture broader than the command contract;
- SideEffect posture broader than the command contract;
- source-write or unclassified SideEffect posture;
- optional declarations that attempt to turn an executed failure into success;
- secret-like IDs or metadata; and
- unsupported future enum values.

Validation errors must use stable codes and must not echo workflow IDs, step
IDs, command IDs, paths, arguments, hashes, environment values, output, or
secret-like input.

## 8. Canonical Resolution

The canonical resolver should accept explicit inputs only:

- one already validated workflow and exact step;
- an explicit validated allowlisted command-contract inventory;
- the immutable bundle model version; and
- future typed constraint sets only when separately implemented.

For every declaration it should:

1. resolve exactly one command contract by `command_id`;
2. validate declaration maxima against the contract;
3. compute the canonical command-contract fingerprint with the existing shared
   helper;
4. construct the independent attestation requirement fingerprint;
5. derive the exact obligation identity from step, requirement, and command
   context;
6. sort by canonical obligation identity; and
7. reject duplicates or ambiguity.

Resolution must not inspect repository files, discover commands, register
handlers, execute checks, or infer missing declarations.

## 9. Canonical Bundle Record

Add a versioned canonical local-check declaration-set record rather than
embedding a naked list or opaque hash in the manifest. The record should bind:

- workflow ID and version;
- exact step ID;
- declaration algorithm/version;
- every validated declaration field;
- command ID, kind, and canonical contract fingerprint;
- independent requirement fingerprint;
- required/optional level;
- deterministic obligation identity; and
- deterministic declaration-set fingerprint.

The record should contain no executable text, arguments, absolute paths,
environment values, raw output, source content, provider payload, token,
credential, or evidence body. Existing canonical workflow records already bind
the authored declaration text after the schema field exists; the dedicated
record binds the resolved command and requirement meaning used at runtime.

The declaration-set record should be content-addressed and validated on read.
Deserialization must recompute its fingerprint and fail closed on mismatch.

## 10. Immutable Manifest Integration

Extend immutable-bundle definition vocabulary with a dedicated local-check
declaration-set record kind or an equivalently explicit typed manifest
collection. Do not hide it inside `resolved_execution_context_hash`.

For each workflow step, the builder must publish exactly one canonical
declaration-set record, including an authoritative empty set when the validated
step declares no local checks. This distinction is essential:

- authoritative empty set: the validated definitions prove no checks were
  declared for that step;
- missing record: bundle invalid or legacy/unbundled posture;
- unresolved record: construction failure, never `Satisfied`.

The manifest root must change when any declaration, requirement, command
contract fingerprint, step binding, or algorithm version changes. Unreferenced
command contracts must not churn the bundle.

Bundle publication remains create-only and failure-atomic. A missing,
ambiguous, corrupt, or mismatched declaration record must prevent `RunCreated`
on the bundle-required path.

## 11. Legacy And Compatibility Posture

Existing specs without `local_check_requirements` remain valid and mean an
explicit empty authored list after the schema field is introduced.

Existing immutable bundles that predate the declaration-record version remain
readable for historical inspection but are **not authoritative for local-check
coverage**. A future authoritative coverage path must reject them with a typed
unsupported/missing-declaration-source outcome rather than treating them as an
empty set.

No migration should rewrite historical bundles. A new run must build a new
bundle under the new model version.

## 12. Relationship To Structural Coverage

A later private authoritative adapter may:

1. read the validated stored immutable bundle;
2. resolve the exact step declaration-set record;
3. derive the existing local-check obligation-set model without caller-supplied
   requirement levels or fingerprints;
4. consume same-call verified contributions; and
5. evaluate complete structural coverage.

Only that separately reviewed adapter may mark declaration provenance as
canonical. The current caller-supplied candidate constructor must remain
non-authoritative and must not be reused as the runtime source boundary.

Aggregate workload conversion, proportional-governance reassessment, and
executor enforcement remain later phases even after canonical derivation is
implemented.

## 13. Proportional Governance And Presentation

Canonical declarations provide enforcement constraints; they do not choose the
operator presentation mode. Quiet capture, visible disclosure, and blocking
approval remain separate execution/disclosure decisions.

Inference may recommend likely checks and may raise attention when repository
facts change. It may not silently lower required declarations or convert a
missing authoritative source into satisfaction. Relevant changes invalidate
the bundle and therefore any prior assessment, following the accepted
build-cache-style invariant.

## 14. Privacy And Redaction

- Store typed declarations, references, enum posture, counts, and fingerprints.
- Exclude command text and process output from the declaration record.
- Treat identifiers and fingerprints as potentially sensitive in `Debug` and
  errors.
- Use fixed non-leaking serde errors.
- Reject secret-like free-form metadata.
- Keep file paths out of the v1 declaration and record.
- A read-only check record may still be confidential and must inherit the
  bundle sensitivity and redaction posture.

## 15. Error Posture

Future stable errors should distinguish:

- declaration invalid or duplicate;
- command contract missing or ambiguous;
- declaration broader than command policy;
- requirement fingerprint mismatch;
- declaration-set fingerprint mismatch;
- bundle record missing, corrupt, or incompatible;
- step or workflow binding mismatch; and
- legacy bundle lacks authoritative declaration source.

Construction failure is an internal bundle-preparation error. It must not be
converted into a misleading user-project diagnostic, passing check, fabricated
evidence, or partial run.

## 16. Test Plan

Future tests must prove:

- valid required and optional step declarations;
- empty authored declaration list remains valid;
- duplicate IDs and duplicate semantic obligations fail;
- unknown command IDs fail resolution;
- accepted status, assurance, freshness, truncation, network, and SideEffect
  validation;
- exact contract fingerprint derivation;
- deterministic ordering and stable fingerprints;
- every decision-relevant field invalidates the set and bundle root;
- unreferenced contracts do not churn the bundle;
- one authoritative record exists for every step, including empty sets;
- missing, duplicate, corrupt, or cross-step records fail closed;
- cross-workflow, cross-run, cross-bundle, and cross-step relabeling fail;
- legacy bundles remain readable but cannot claim authoritative coverage;
- safe serde, `Debug`, and error behavior;
- no raw command, path, environment, output, source, provider, or secret data is
  stored;
- existing immutable-bundle, local-check, attestation, structural-coverage,
  executor, policy, evidence, and report tests remain green.

## 17. Proposed Implementation Sequence

1. Add typed step-scoped local-check declaration vocabulary, schema-facing
   validation, and focused serialization/privacy tests. Do not resolve or run
   checks yet.
2. Review the declaration model and compatibility posture.
3. Add the canonical declaration-set record and pure resolver against an
   explicit allowlisted command-contract inventory.
4. Review deterministic identity, contract resolution, and privacy.
5. Extend immutable-bundle construction and create-only storage to publish and
   validate one declaration-set record per step.
6. Review bundle invalidation, atomicity, legacy posture, and corruption tests.
7. Add a private authoritative adapter from stored records to structural
   coverage.
8. Only after review, plan aggregate evidence/check posture conversion and one
   explicit executor gate.

Each item is a separate governed implementation or review phase. Items 1
through 5 are implemented. Item 6 is the next review boundary. Structural
coverage adaptation and executor integration remain later, separately governed
code phases.

## 18. Open Questions

- Should command-contract declarations eventually be project specs or remain a
  kernel allowlist plus plugin-owned validated inventory?
- Should a skill be able to require a check class that each workflow maps to a
  concrete command, or should only workflows select commands in v1?
- Should policy/profile/steward constraints add obligations or only strengthen
  already selected obligations?
- What schema-version transition best distinguishes historical absent fields
  from authoritative empty lists?
- Should freshness be duration-based only or also support exact no-cache
  posture as a distinct enum?
- When should handler implementation identity enter the declaration-set record
  versus the separate execution binding?

## 19. Final Recommendation

Review the implemented immutable-bundle publication boundary. If accepted,
proceed to the private authoritative adapter from validated stored records to
structural coverage. The declaration model review is documented in
[Local Check Requirement Declaration Model Review](../concepts/LOCAL_CHECK_REQUIREMENT_DECLARATION_MODEL_REVIEW.md),
and the publication implementation is documented in
[Canonical Local-Check Declaration Immutable-Bundle Publication Report](../concepts/CANONICAL_LOCAL_CHECK_DECLARATION_IMMUTABLE_BUNDLE_PUBLICATION_REPORT.md).
No current implementation converts these records to aggregate posture or
enforces runtime gates.

Do not execute checks, add default handlers, derive authoritative coverage,
convert aggregate posture, reassess proportional governance, add executor
checkpoints, persist runtime results, call providers, add writes, or change
release posture in that first implementation.

## 20. Governed Planning Record

- workflow: `dg/d`
- run: `run-1784967114974906000-2`
- approval: `approval/run-1784967114974906000-2/planning-approved`
- presentation: `presentation/70c403fd636aae51`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; inspection, writing, and
  validation run outside the kernel

## 21. First Implementation Status

The first implementation adds:

- `StepDefinition.local_check_requirements`, defaulting to an authoritative
  empty authored list for compatibility;
- validated requirement ID, required/optional level, command reference,
  independent assurance, passed-only result posture, freshness, exact bundle
  binding, truncation, network, and SideEffect fields;
- fail-closed deserialization and redaction-safe `Debug` behavior; and
- deterministic duplicate ID and duplicate semantic-obligation diagnostics.

Command IDs remain unresolved declaration references in this slice. Unknown or
ambiguous command-contract resolution, contract-maxima comparison, canonical
records, fingerprints, immutable-bundle publication, structural-coverage
authority, aggregate posture, and runtime enforcement remain deferred.

## 22. Canonical Resolution Implementation Status

The second implementation adds:

- an explicit validated allowlisted command-contract inventory;
- a pure resolver that selects one exact workflow step and resolves every
  declaration by command identity;
- fail-closed missing and ambiguous identity handling;
- declaration-to-contract network and SideEffect maximum checks;
- canonical command-contract and independent attestation-requirement
  fingerprints;
- deterministic obligation identities and order-independent declaration-set
  fingerprints;
- a content-addressed declaration-set record, including authoritative empty
  sets; and
- fail-closed record deserialization with fingerprint recomputation and
  redaction-safe `Debug`.

The record excludes executable text, arguments, working directories,
environment values, raw output, source content, provider payloads, credentials,
and evidence bodies. It is returned in memory only. Immutable-bundle
publication, storage, runtime authority, structural-coverage adaptation,
aggregate posture, executor enforcement, providers, and writes remain
unimplemented.

## 23. Immutable-Bundle Publication Implementation Status

The third implementation adds:

- a payload-free typed declaration-set reference bound to workflow, version,
  step, immutable-bundle version, algorithm, and declaration-set fingerprint;
- an explicit enriched immutable-bundle builder that resolves exactly one
  canonical record for every workflow step from a caller-supplied validated
  command-contract inventory;
- root-hash participation for canonical declaration-set references;
- content-addressed, create-only declaration-set storage before manifest
  publication;
- read-time reference, address, binding, and fingerprint validation; and
- legacy compatibility through an omitted/default-empty reference collection
  whose absence remains non-authoritative rather than meaning authoritative
  empty coverage.

The existing builder remains the legacy path and does not fabricate
declaration coverage. The enriched path is explicit, and unreferenced inventory
contracts do not affect the bundle root. Missing, corrupt, mismatched, or
unreferenced records fail closed with stable non-leaking errors.

This phase does not execute checks, register handlers, inspect repository
metadata, adapt records into structural coverage, convert aggregate governance
posture, add executor gates, expose CLI or schema behavior, call providers,
model SideEffect execution, or authorize writes.
