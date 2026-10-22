# Scoped Capability GitHub Read-Only Integration Plan

Status: implemented as a crate-private, opt-in, no-write runtime vertical slice;
focused implementation review required before any broader adoption.

Related foundations:

- [Scoped Runtime Authority And Capability Projection Plan](scoped-runtime-authority-capability-projection-plan.md)
- [Current Authority In-Memory Source Plan](current-authority-in-memory-source-plan.md)
- [Production Current Authority Source Boundary Plan](production-current-authority-source-boundary-plan.md)
- [Local Unsigned Authority Receipt Plan](local-unsigned-authority-receipt-plan.md)
- [Scoped Capability Read-Only Integration Queue Selection Report](../concepts/SCOPED_CAPABILITY_READ_ONLY_INTEGRATION_QUEUE_SELECTION_REPORT.md)

## 1. Executive Summary

This plan defines the first runtime composition of source-backed capability
authority with an existing provider adapter. The implementation should add one
internal, explicit, opt-in helper that authorizes and performs exactly one
GitHub repository-metadata read through an injected `GitHubReadOnlyAdapter`.

The helper must read a complete current-authority fact set from a registered
Core-owned source, resolve the exact `github.read` capability for the current
actor, workflow, run, step, harness, repository, and sensitivity, and call the
adapter only when that fresh resolution is authorized. After one successful
bounded read, it may issue a local unsigned evidence-only authority receipt.

This is a no-write runtime vertical slice. It does not add provider mutations,
live-network acceptance requirements, CLI or workflow-schema configuration,
ambient tool exposure, automatic grants, hosted identity, or another adapter.

## 2. Why This Phase Is Next

Workflow OS already has the required primitives:

- validated capability grants and availability records;
- deterministic resolution over exact execution and resource scope;
- step-scoped capability projection;
- a registered in-memory source with complete-inventory, freshness, and
  commitment semantics;
- an injected GitHub read-only adapter with bounded normalized output and
  redaction-safe telemetry; and
- a trusted local unsigned receipt producer for one required-context read.

Those primitives have not yet been composed around an adapter invocation. The
next useful proof is therefore not another model family. It is one real
no-write call boundary where source failure or insufficient authority prevents
the client call and successful use produces bounded proof.

## 3. Selected Operation

The first and only supported operation is:

```text
GitHubReadOnlyAdapter::read_repository_metadata
```

The integration must require:

- `AdapterKind::GitHub`;
- `AdapterCapability::GitHubRead`;
- action `github.repository.metadata`;
- capability reference `github.read`;
- resource kind `adapter_resource`; and
- canonical resource reference `github/<owner>/<repository>`.

Owner and repository segments must be validated independently before the
canonical resource reference is formed. Owner is limited to 39 ASCII
alphanumeric or hyphen characters and cannot begin or end with a hyphen.
Repository is limited to 100 ASCII alphanumeric, dot, underscore, or hyphen
characters. Empty segments, separators, URL syntax, traversal, control text,
credentials, and secret-like values are rejected.

GitHub resolves owner and repository names case-insensitively. Core must
therefore normalize both validated segments to ASCII lowercase before creating
`github/<owner>/<repository>`. The same canonical lowercase values must be
used for the source query, authority resolution, adapter request metadata,
adapter call, and successful-operation commitment. Original caller spelling
must not be retained in errors, Debug, or receipt evidence. The caller must not
be able to authorize one case alias and invoke another.

Default-branch, file-content, pull-request, comment, diff, changed-file, and
check-status reads remain out of scope. Their different payload and sensitivity
postures require separate adoption decisions.

## 4. Source-Of-Truth Boundary

The runtime helper must consume a registered Core-owned current-authority
source. Caller-supplied grants, availability records, fact sets, projections,
or receipts are not authoritative inputs.

The existing `CurrentAuthoritySourceRequest` is intentionally derived from a
`RequiredContextContractBinding`. A GitHub repository read is a capability use,
not a governed-context requirement. The implementation must not fabricate a
required-context contract or `GovernedContextReference` to reuse that request
shape.

Instead, add dedicated crate-private `CapabilityUseCurrentAuthorityQuery`,
`CapabilityUseCurrentAuthorityQuerySet`, and
`CapabilityUseCurrentAuthorityFactSet` types plus the smallest private source
request and assessment boundary. These types support exactly one
capability/resource query and are not a new public general-query framework.
They should reuse existing source registration, read-window, freshness,
completeness, consistency, snapshot commitment, and capability-resolution
primitives while querying exactly:

- capability grants matching the exact execution identity and `github.read`;
- capability availability for `github.read`; and
- the canonical GitHub repository resource.

No governed-context reference family is requested or copied into this fact
set. Existing context-derived `CurrentAuthorityQuerySet`,
`CurrentAuthorityFactSet`, and `CurrentAuthoritySourceRequest` must not be
reused. The source must still prove that the returned grant and availability
inventories are complete for the exact query. An empty complete inventory is
authoritative absence and must fail closed.

## 5. Candidate Internal API

The implementation should remain internal to `workflow-core`. Candidate types
may include:

- `CurrentAuthorityGitHubRepositoryReadInput<'a>`;
- `CurrentAuthorityGitHubRepositoryReadOutcome`;
- `GitHubRepositoryReadExecutionBinding`; and
- a private successful-use proof consumed by the receipt producer.

The input should contain only explicit values:

- registered current-authority source;
- exact actor, workflow ID, run ID, step ID, and harness contract identity;
- owner and repository;
- maximum/requested sensitivity;
- evaluation timestamp supplied by a trusted caller;
- adapter request fields required by the existing adapter contract;
- one already-evaluated adapter policy precheck;
- correlation ID and redaction metadata; and
- the injected adapter/client boundary.

The helper must not read environment variables, discover credentials, invent
runtime configuration, or select a repository from ambient Git state.

The helper must return an explicit outcome enum with these semantic postures:

- `Succeeded { outcome, projection, receipt }`;
- `ReadSucceededReceiptFailed { outcome, projection, error }`;
- `BlockedBeforeRead { error }`; and
- `ReadFailed { error }`.

Names may be adjusted to existing conventions, but the domains must remain
distinct. A pre-read block contains no projection or receipt. An adapter
failure contains no receipt. A receipt failure after provider success retains
the bounded read outcome and projection so callers cannot mistake the call for
not attempted and retry it blindly.

## 6. Exact Authority Resolution

The helper must build a fresh `CapabilityResolutionInput` from source facts and
the exact call context. Authorization requires all of the following:

- actor equality;
- workflow equality;
- exact run equality;
- exact step equality;
- exact harness contract equality;
- capability equality with `github.read`;
- exact canonical repository-resource equality;
- current grant posture;
- non-expired authority at the trusted evaluation time;
- current availability posture `available`;
- requested sensitivity within the grant and source ceilings; and
- one unambiguous authorizing grant.

Existing generic resolution remains unchanged. After it selects the most
specific matching grant, this integration must inspect that grant from the
fresh fact set and require its stored scope to contain the exact run and step
used by the call plus the exact harness posture. Workflow-wide or run-wide
grants remain valid model vocabulary but are insufficient for this first
adapter-use proof. A future integration may separately review broader grant
inheritance.

Any grant carrying independent policy, approval, evidence, or check
requirements remains non-authorizing in this slice. The helper must return the
existing independent-evaluation posture before adapter invocation. It must not
infer that those prerequisites passed and must not accept caller booleans that
claim they passed.

## 7. Freshness, Completeness, And Consistency

Source evaluation must use existing registered-source commitments and bounded
read-window rules. Before authority resolution, the helper must reject:

- unregistered or mismatched source identity;
- unsupported source contract version;
- source sensitivity below the request;
- incomplete grant or availability inventory;
- unordered, duplicate, conflicting, or ambiguous facts;
- stale observations;
- an expired source-valid-through boundary;
- rollback or invalid generation posture; and
- a snapshot or fact-set commitment mismatch.

The source snapshot, fact set, and resolution assessment must be committed into
the successful-use proof. A previously serialized snapshot, projection, or
receipt cannot be supplied to skip this same-call source read.

## 8. Deterministic Failure Ordering

The implementation must preserve this order:

1. validate explicit input and exact execution identity;
2. validate operation mode and policy-precheck provenance;
3. validate and canonicalize owner/repository resource scope;
4. validate source registration and construct the exact capability-use query;
5. read and validate a complete fresh source snapshot;
6. construct and validate the exact capability-use fact set;
7. resolve capability authority and verify exact selected-grant scope;
8. reject independent-prerequisite posture;
9. derive the step-scoped projection from the fresh authorized resolution;
10. construct and validate the existing adapter request;
11. call `read_repository_metadata` exactly once;
12. validate and commit the bounded successful outcome; and
13. issue the evidence-only receipt.

Failure at steps 1 through 10 must prove that the adapter client was not
called. Adapter failure at step 11 must return `ReadFailed` with the classified
adapter failure and no receipt. Receipt-construction failure after a successful
read must return `ReadSucceededReceiptFailed`; it must not relabel the provider
read as failed or authorize a retry.

## 9. Adapter Request And Telemetry

The helper must use the existing `AdapterRequest` and
`GitHubReadOnlyAdapter::read_repository_metadata` APIs. It must not bypass
adapter precondition validation or call the injected client directly.

The request's actor, run scope, capability, action, canonical owner and
repository, correlation ID, operation mode, and policy precheck must agree with
the governed integration input.

Capability authority does not replace policy. This slice requires one
already-evaluated `AdapterPolicyPrecheck` and passes it unchanged into the
adapter request:

- `Fixture` accepts only `Allowed` with `FixtureTest` provenance;
- `LiveReadOnly` accepts only `Allowed` with `RuntimePolicy` provenance; and
- `ApprovalGranted`, `Missing`, `Denied`, `Mock`, `Local`, and
  `LiveWriteCapable` are rejected before source access.

The helper must not synthesize `runtime_allowed` from a capability grant. The
source authenticity of an allowed policy precheck remains the existing adapter
contract boundary and is not a new guarantee of this phase. A grant that
declares an independent policy prerequisite remains non-authorizing because
this slice does not bind the supplied precheck to that prerequisite.

Existing adapter invocation and observability records remain the telemetry
source of truth. The integration must not synthesize successful telemetry on
denied or failed paths. It must not copy raw GitHub response bodies into Core
errors, source facts, projections, receipts, Debug output, or tests.

## 10. Receipt Design And Compatibility

The existing `AuthorityReceipt` V1 wire model is bound to a required-context
requirement, governed-context target kind, and access level. Those fields are
not truthful for a GitHub adapter capability use.

Implementation must therefore add a separate versioned
`CapabilityUseAuthorityReceipt` V1 type rather than modifying or wrapping the
existing required-context receipt. The new type must:

- preserves all existing V1 serialized bytes and validation behavior;
- does not add fake required-context fields;
- binds the exact execution identity, capability, resource commitment, grant,
  source snapshot, fact set, assessment, operation kind, outcome commitment,
  issuance time, validity, signature posture, effect, and redaction posture;
- use a dedicated operation kind such as
  `github_repository_metadata_read_v1`;
- be produced only from a private successful-use proof; and
- remain serialize-only as trusted evidence, with deserialized data explicitly
  treated as an unverified claim.

The receipt is point-in-time evidence. It cannot authorize another adapter
call, retry, repository, actor, workflow, run, step, or sensitivity. It must
not contain owner/repository plaintext if a bounded commitment is sufficient,
and it must never contain credentials, request headers, raw responses, or
provider payloads.

Trusted receipt construction remains crate-private. Deserialization must target
an explicitly named unverified-claim type and cannot recover the trusted
receipt. Existing `AuthorityReceipt` V1 APIs, known bytes, required-context
semantics, and validation remain unchanged. A generalized envelope is deferred
until at least two truthful operation-specific receipt models justify it.

## 11. Error And Privacy Boundary

Errors must use stable codes and bounded messages. They must not include:

- owner or repository values;
- resource references or commitments beyond a safe stable identifier;
- raw source facts;
- grant contents;
- GitHub response bodies;
- URLs, paths, file contents, command output, or parser payloads;
- credentials, authorization headers, environment values, or tokens; or
- arbitrary adapter metadata.

Debug implementations for new integration types must redact repository,
resource, source-fact, and receipt identifiers where disclosure is not needed.
Serialization is allowed only for explicitly reviewed evidence models. Private
runtime inputs and successful-use proofs should not derive serialization.

## 12. API And Product Posture

The helper should be `pub(crate)` unless an existing public adapter-composition
pattern requires otherwise. Public export, workflow configuration, CLI
commands, SDK support, and schema exposure require separate compatibility and
product review.

This phase does not make capability gating automatic for all adapters. It
proves one explicit Core-owned path. Existing direct GitHub adapter APIs remain
unchanged and do not acquire stronger governance claims merely because this
helper exists.

## 13. Strict Non-Goals

Do not add:

- GitHub comments, pull requests, merges, or provider writes;
- another GitHub read action or provider adapter;
- live GitHub access as required validation;
- token loading, credential storage, or connector installation;
- workflow-spec, schema, SDK, runtime-config, or CLI changes;
- automatic grants, inferred authority, or model-decided authority;
- independent policy, approval, evidence, or check evaluation;
- durable receipt persistence, audit export, or report-artifact integration;
- ambient tool visibility or workspace access;
- OpenShell or another sandbox runtime;
- nested harness execution or agent delegation;
- hosted identity, enterprise administration, or cryptographic signing; or
- release posture changes.

## 14. Implementation Sequence

1. Add the private exact capability-use query, fact set, source request, and
   source-backed assessment for `github.read` repository metadata.
2. Add focused source tests proving freshness, completeness, ambiguity, and
   no-invocation failure behavior.
3. Add the internal GitHub repository-read composition helper using the
   existing adapter request and injected client.
4. Add the private successful-use proof and separate
   `CapabilityUseAuthorityReceipt` V1 without changing existing receipt bytes.
5. Add operation and privacy tests, then run existing authority, receipt,
   adapter, telemetry, and workspace regressions.
6. Complete a focused maintainer/security review before considering another
   operation or public API.

Implementation may occur in one governed milestone after this plan is
accepted. A demonstrated security or compatibility blocker must be fixed and
re-reviewed; ordinary implementation detail should not create an open-ended
documentation cycle.

## 15. Test Plan

Future tests must prove:

1. one exact authorized GitHub repository-metadata read succeeds;
2. the source is read in the same call before the adapter client;
3. the exact fresh resolution produces the step-scoped projection;
4. the injected client is called exactly once on success;
5. a receipt is issued only after a successful bounded adapter outcome;
6. source failure prevents the client call;
7. incomplete, stale, expired, rollback, duplicate, conflicting, and ambiguous
   source facts prevent the client call;
8. missing, revoked, expired, unavailable, or sensitivity-exceeding authority
   prevents the client call;
9. actor, workflow, run, step, harness, capability, owner, repository, resource,
   and correlation substitution fail closed;
10. a prerequisite-bearing grant prevents the client call;
11. adapter request validation failure prevents the client call;
12. adapter failure produces no receipt and no fake success telemetry;
13. receipt failure after provider success is represented as a completed read
   with missing evidence, not as permission to retry;
14. the receipt cannot be used as runtime authority;
15. deserialized receipt data remains an unverified claim;
16. existing V1 receipt fixture bytes remain unchanged;
17. Debug, errors, and serialization do not leak repository names, raw payloads,
   source facts, credentials, headers, tokens, or secret-like markers;
18. no live network is required; and
19. existing capability, projection, context, receipt, GitHub adapter,
   telemetry, and workspace tests pass.

Additional required regressions are:

- fixture/live policy-precheck provenance mismatch fails before source access;
- mixed-case repository aliases resolve to one lowercase authority identity;
- invalid or oversized owner/repository segments fail without source or client
  use;
- a selected workflow-wide or run-wide grant is insufficient;
- no context-derived query or fake context requirement is constructed;
- existing `AuthorityReceipt` V1 known bytes remain unchanged; and
- read success followed by receipt failure returns the explicit non-retry
  posture with no receipt.

## 16. Review Questions

The focused review decisions are now incorporated:

- canonical resource: lowercase `github/<owner>/<repository>` from separately
  validated segments;
- source model: crate-private single-query capability-use types;
- selected grant: exact run and step required for this first integration;
- policy: explicit already-evaluated precheck with operation-mode provenance;
- receipt: separate `CapabilityUseAuthorityReceipt` V1;
- post-read evidence failure: explicit `ReadSucceededReceiptFailed` posture;
  and
- operation outcome proof: commitment over the existing bounded normalized
  response/invocation fields, never the raw provider body.

The blocker-fix review should verify these decisions are internally consistent.

## 17. Governed Planning Record

- workflow ID: `dg/d`
- run ID: `run-1792689341041273000-2`
- approval ID: `approval/run-1792689341041273000-2/planning-approved`
- presentation ID: `presentation/c49e765dc84576f5`
- approval outcome: granted by delegated maintainer with persisted presentation
  proof
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation enforcement: proof enforced with one matching persisted
  presentation record and event marker
- kernel boundary: governance coordination only; source inspection,
  documentation, validation, Git, and pull-request work remain out of kernel

## 18. Validation

The planning phase required:

- `npm run check:docs`; and
- `git diff --check`.

Both planning checks passed. The subsequently approved implementation is
documented in the [Scoped Capability GitHub Read-Only Integration
Report](../concepts/SCOPED_CAPABILITY_GITHUB_READ_ONLY_INTEGRATION_REPORT.md).
The implementation remains internal and does not make existing direct adapter
APIs capability-gated.

## 19. Final Recommendation

Proceed next to a focused maintainer and security review of the implemented
vertical slice. Do not expose the helper publicly or broaden to another read
or write operation before that review accepts the source, scope, failure,
privacy, and receipt boundaries.
