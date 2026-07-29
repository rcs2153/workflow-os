# Single-Tenant Hosted Alpha Plan

Status: phase-ready planning; not implemented

## 1. Executive Summary

Workflow OS has accepted local and shared durable-state foundations. The local
kernel can validate, execute governed sequential workflows, pause for approval,
record durable events, preserve immutable run identity, govern selected
SideEffects, and produce report-ready records. The explicit opt-in
`PostgreSQL` backend now implements the accepted shared-state transaction,
revision, lease, consumer, rebuild, and recovery semantics.

The next implementation milestone should compose those foundations into one
narrow **single-tenant hosted alpha**:

- one authenticated remote governance API;
- one organization and one administrative trust domain;
- `PostgreSQL` as the shared durable source of truth;
- stateless API and worker processes;
- fenced claim, retry, cancellation, and recovery behavior;
- one explicit execution-provider boundary;
- reference-only access-material delivery for one reviewed execution path;
- durable audit, evidence, approval, SideEffect, and WorkReport posture;
- health, metrics, bounded diagnostics, deployment, and recovery guidance.

This is one integrated implementation milestone. Internal code slices may be
reviewed together and should not restart separate planning cycles unless a
security or correctness blocker requires a narrower fix.

This plan does not implement hosted behavior. It does not claim production
readiness, multi-tenant isolation, enterprise administration, general agent
execution, OpenShell integration, or additional provider mutations.

## 2. Goals

The implementation milestone should:

1. expose the existing governed runtime through a bounded authenticated API;
2. preserve Core as the owner of validation, policy, approvals, state
   transitions, idempotency, evidence, audit, and reporting;
3. let stateless workers claim durable work through database-time leases and
   fencing tokens;
4. execute only through an explicit provider boundary that cannot mutate Core
   state directly;
5. keep credentials and other access material out of workflow specs, API
   payloads, `PostgreSQL`, events, logs, diagnostics, and reports;
6. preserve immutable run inputs and approval-presentation proof across remote
   process boundaries;
7. make cancellation, retry, worker loss, ambiguous execution outcomes, and
   recovery explicit;
8. provide enough health, metrics, audit, and recovery behavior to evaluate one
   deployed trust domain honestly;
9. keep the current local CLI and local executor behavior compatible;
10. reduce the gap between documented governance and runtime-enforced
    governance without broadening external writes.

## 3. Non-Goals

The hosted-alpha implementation must not add or claim:

- multi-tenancy or tenant isolation;
- organizations with independent trust domains;
- enterprise RBAC, SCIM, SSO administration, or IdP lifecycle management;
- a hosted user interface;
- recursive agents, agent swarms, or nested harness execution;
- general shell, filesystem, network, browser, or model execution;
- OpenShell, container, Kubernetes, or virtual-machine integration;
- broad provider-write adapters or a new provider mutation family;
- automatic execution of arbitrary `SkillHandler` implementations;
- workflow-spec schema or TypeScript SDK expansion unless separately approved;
- automatic workflow authoring or activation;
- a general distributed queue;
- exactly-once execution claims;
- credential persistence in Workflow OS state;
- production TLS, connection pooling, replication, high availability,
  point-in-time recovery, capacity, or SLO claims;
- production compliance, certification, SIEM, DLP, or enterprise audit export;
- collaborative workflow/catalog administration;
- reasoning lineage or claim-graph implementation;
- release-posture changes.

## 4. Alpha Product Boundary

The alpha is one deployment serving one organization under one administrative
trust domain. All authenticated callers are provisioned by the deployment
operator. The alpha may distinguish human and service actors for audit, but it
does not claim general role management or tenant isolation.

The hosted service governs work. It is not itself the general-purpose sandbox,
agent runtime, build system, or provider. An execution provider may run bounded
work only after Core has authorized an exact immutable request.

The alpha is successful when a remote caller can:

1. submit a bounded workflow bundle for validation;
2. create an idempotent run bound to that exact bundle;
3. inspect run state and ordered events;
4. receive and decide a proof-enforced approval request;
5. cancel eligible non-terminal work;
6. let a stateless worker claim and process authorized work;
7. inspect a terminal report and payload-free execution receipts;
8. recover safely from an API or worker restart.

## 5. Architecture

The first topology should contain:

- **API service:** authenticates requests, validates bounded transport models,
  invokes Core services, and returns metadata-only responses;
- **worker service:** claims one runnable unit under a fenced lease, rehydrates
  the immutable run, reassesses current authority where required, invokes the
  selected execution provider, and commits the outcome under the same fence;
- **`PostgreSQL`:** authoritative shared event, projection, approval,
  presentation, immutable-bundle, idempotency, lease, SideEffect, telemetry,
  artifact, and report state;
- **execution provider:** injected interface for one reviewed execution path;
- **access-material resolver:** injected reference-only boundary that resolves
  access material at time of use without persisting it in Core state;
- **telemetry sinks:** bounded metrics, traces, and structured logs that do not
  become the authoritative audit record.

The API and worker must be stateless with respect to governed run state. Process
memory may cache non-authoritative data, but restart correctness must come from
`PostgreSQL` and immutable run bundles.

The implementation should remain inside the existing Rust workspace unless a
reviewed dependency or deployment boundary requires a new crate. A likely
shape is a new hosted service crate plus Core-owned transport-neutral service
interfaces. HTTP parsing, authentication middleware, and deployment wiring
must not move domain invariants out of `workflow-core`.

## 6. Source-Of-Truth Boundaries

| Concern | Source of truth |
| --- | --- |
| Workflow/run identity | Immutable run bundle and canonical content hash |
| Current run state | Projection reconciled from authoritative events |
| Approval authority | Persisted request, presentation proof, decision, and current-authority reassessment |
| Worker ownership | Database-time lease with fencing token |
| Idempotency | Durable reservation and completion records |
| SideEffect lifecycle | Core `SideEffect` records and authoritative events |
| Evidence and citations | Stable references; never copied provider payloads |
| WorkReport | Validated report/artifact record linked to the run |
| Execution containment | Execution provider; never inferred from Core state |
| Credentials | External resolver/provider; never Workflow OS durable state |
| Operational telemetry | Non-authoritative metrics, traces, and logs |

An API response, worker cache, telemetry backend, or execution-provider log must
not silently become a competing source of truth.

## 7. Remote API Contract

The first API should be versioned under a stable preview namespace and support:

- validation submission and result retrieval;
- idempotent run creation;
- run summary retrieval;
- ordered event retrieval with bounded pagination;
- approval request retrieval;
- proof-enforced approval grant or denial;
- eligible run cancellation;
- WorkReport and report-artifact metadata retrieval;
- health, readiness, and build identity.

An illustrative route set is:

```text
POST /v1/validations
POST /v1/runs
GET  /v1/runs/{run_id}
GET  /v1/runs/{run_id}/events
GET  /v1/runs/{run_id}/approvals/{approval_id}
POST /v1/runs/{run_id}/approvals/{approval_id}:decide
POST /v1/runs/{run_id}:cancel
GET  /v1/runs/{run_id}/report
GET  /health/live
GET  /health/ready
GET  /version
```

The final implementation may adjust route names, but it must preserve these
operations and boundaries.

Every state-changing request must require:

- authenticated actor identity;
- correlation ID;
- caller-supplied idempotency key;
- bounded request size;
- explicit schema/API version;
- stable non-secret error code;
- authorization against the single trust-domain policy;
- audit context before mutation.

Repeated requests with the same idempotency key and equivalent identity must
return deterministic replay posture. Conflicting reuse must fail closed.

The API must not return raw provider payloads, credentials, command output,
arbitrary source contents, parser payloads, or unbounded event/report text.

## 8. Authentication And Authority

The alpha should use one deployment-configured authentication mechanism with:

- one operator-controlled trust domain;
- explicit human or service actor identity;
- audience and issuer binding where tokens are used;
- bounded expiration;
- no anonymous mutation;
- payload-free actor references in durable records;
- redacted authentication failures.

The implementation must not invent a broad role model. It should support a
minimal operator and service-worker distinction sufficient to prevent an
untrusted API caller from impersonating the worker. Approval decisions must
still pass Core approval, presentation-proof, current-authority, policy, and
separation checks; transport authentication alone is not approval authority.

Long-lived static secrets should not be the preferred architecture. If a
deployment-bound token is used for the first alpha proof, it must be injected
externally, hashed or verified outside durable domain state, rotatable without
state migration, and documented as an alpha limitation.

## 9. Worker Claim And Execution Lifecycle

A stateless worker should:

1. discover one runnable durable work item through a bounded query;
2. acquire a database-time lease with a monotonically increasing fence;
3. read and verify the immutable run bundle;
4. rehydrate the run from authoritative events;
5. confirm that the run remains eligible for the exact work item;
6. reassess policy, proportional-governance posture, approval authority,
   required evidence/check facts, capability, and SideEffect requirements;
7. reserve idempotency and, for external effects, persist pre-effect intent;
8. resolve access material at time of use;
9. invoke the configured execution provider;
10. persist a bounded execution receipt and outcome under the valid fence;
11. append authoritative events and update projections atomically where the
    accepted transaction family permits;
12. release the lease or allow it to expire safely after failure.

The worker must not poll or execute automatically until its full claim,
eligibility, fence, and outcome contracts are tested.

Worker loss before provider invocation should permit takeover after lease
expiry. Worker loss after possible external effect must produce an ambiguous
outcome that requires provider reconciliation; it must not retry blindly or
claim that nothing happened.

## 10. Execution-Provider Boundary

Hosted execution must use a new explicit provider interface. The existing
`SkillHandler` remains a deterministic local development/test interface and
must not become an implicit hosted shell, network, provider, or credential
boundary.

The provider request should contain only:

- immutable run and step identity;
- exact input/artifact references;
- authorized capability set;
- approved SideEffect intent where applicable;
- execution policy reference and hash;
- timeout and resource budget;
- correlation and idempotency identity;
- access-material references, never resolved values.

The provider response should contain only:

- provider and sandbox/execution identity;
- policy/configuration hash;
- start and terminal timestamps;
- terminal status and bounded error category;
- exit or provider status where meaningful;
- stable log, artifact, denied-action, and telemetry references;
- SideEffect outcome/reconciliation reference where applicable.

The provider must not append Core events, mutate snapshots, grant approvals,
change policy, or create evidence claims. Core validates the response and
decides which events, evidence references, SideEffect transitions, and report
citations are authoritative.

An optional sandbox such as NVIDIA OpenShell may later implement this interface,
but it is not a dependency or deliverable of the first hosted alpha. A fork is
not justified unless an upstream substrate blocks required containment,
receipt, or governance hooks and the project explicitly accepts ownership of
that security runtime.

## 11. Access-Material Isolation

The alpha may exercise one reviewed access-material path only. It must:

- accept opaque references, not credentials, in workflow/run inputs;
- resolve values only after exact current authorization;
- scope values to one provider request;
- avoid durable storage, logs, errors, telemetry, evidence, and reports;
- support expiration and revocation detection;
- clear or drop resolved values after invocation;
- fail closed when the resolver is unavailable or posture is stale;
- record only payload-free resolver/provider receipts.

Workflow OS does not become a secret manager. The resolver is injected by the
deployment and remains outside Core durable state.

## 12. Governance Composition

The hosted path must use existing Core constructors and accepted boundaries for:

- deterministic project/spec validation;
- immutable run-bundle publication and read;
- policy evaluation;
- proportional-governance decision and read-only posture;
- approval request and exact presentation proof;
- approval grant/denial with current-authority reassessment;
- required evidence and check facts;
- capability and authority projection where implemented;
- SideEffect pre-effect intent, outcome, reconciliation, and linkage;
- WorkReport generation and artifact integrity;
- event, audit, and telemetry projection.

Quiet capture may remove an unnecessary human interruption only when the
deterministic governance decision permits it. It must not remove durable
evidence, audit, disclosure, or report obligations. Visible disclosure is a
delivery posture over a governance decision, not permission to weaken a
blocking approval or denial.

## 13. Cancellation, Retry, And Failure Semantics

Cancellation is cooperative and state-driven:

- API cancellation appends the accepted cancellation transition only for an
  eligible non-terminal run;
- workers must check cancellation before claiming and before provider
  invocation;
- a provider may expose a cancellation capability, but the alpha must not claim
  hard interruption when it cannot prove it;
- in-flight external outcomes remain completed, failed, or ambiguous rather
  than being erased.

Retries must be bounded and category-aware:

- database serialization/deadlock retries remain within the state adapter;
- worker takeover follows lease expiry and fencing;
- provider retries require the same idempotency identity and must respect
  SideEffect reconciliation;
- policy, approval, authority, validation, and capability failures do not
  become transient retries;
- retry exhaustion becomes failed, escalated, canceled, or manual recovery
  according to the immutable run policy.

Errors must use stable codes and bounded summaries. They must not leak SQL,
connection strings, credentials, tokens, source text, command output, provider
payloads, or policy internals.

## 14. Evidence, Audit, Reports, And Artifacts

The hosted alpha must retain the distinction between:

- authoritative workflow events;
- audit projections;
- adapter/execution telemetry;
- `EvidenceReference` citations;
- `SideEffect` records;
- `WorkReport` and report artifacts;
- operational logs and traces.

Reports cite stable references instead of copying payloads. Missing evidence
must be explicit. Execution-provider receipts may become evidence only through
a reviewed Core mapping that preserves source, scope, sensitivity, and
redaction posture.

Report-generation or artifact-write failure after a terminal workflow outcome
must not silently rewrite workflow pass/fail semantics. The result must expose
the report failure separately and remain inspectable.

## 15. Observability And Operations

The alpha should expose:

- liveness and dependency-aware readiness;
- build/version identity;
- API request count, latency, and error category;
- worker claim, lease contention, takeover, and stale-fence rejection;
- run status and terminal outcome counts;
- approval wait time and decision counts;
- retry, escalation, cancellation, and ambiguous-outcome counts;
- execution-provider latency and bounded outcome category;
- database transaction retry and pool posture;
- queue/backlog age derived from durable state;
- report/artifact generation failures.

Metrics and traces must use bounded labels. Run, workflow, actor, provider, and
error identifiers must not create uncontrolled cardinality or leak sensitive
values. Structured logs should carry correlation IDs and stable references, not
payloads.

## 16. Deployment And Configuration

The first deployment should be platform-neutral and reproducible. It should
define:

- API process;
- one or more stateless worker processes;
- supported `PostgreSQL` version;
- reviewed TLS, timeout, and pooling factory outside Core;
- external authentication and access-material configuration;
- migration/schema compatibility gate;
- health and readiness checks;
- bounded concurrency and shutdown behavior;
- backup/restore and recovery commands;
- upgrade, rollback, and incompatible-schema behavior.

Kubernetes, a managed service, and a hosted control plane are not required.
Local containers or a single virtual machine may prove the topology, provided
the service and worker still use shared `PostgreSQL` state and independent
processes.

## 17. Recovery And Upgrade Posture

The implementation must include a runbook and executable proof for:

- API restart during active runs;
- worker termination before and after lease acquisition;
- worker takeover after lease expiry;
- stale worker commit rejection;
- database unavailability and reconnection;
- schema mismatch or newer-schema fail-closed behavior;
- logical backup, isolated restore, projection rebuild, and immutable-bundle
  readability;
- deployment rollback when no incompatible durable write occurred;
- operator handling of ambiguous provider outcomes.

The alpha does not claim HA or PITR. Recovery proof should be honest about the
single-database deployment and the difference between logical rehearsal and
production disaster recovery.

## 18. Security And Privacy

Before implementation acceptance, the milestone needs a focused threat review
covering:

- authentication bypass and actor spoofing;
- replay and idempotency-key collision;
- stale approval or authority reuse;
- immutable-bundle substitution;
- lease theft and stale worker commits;
- confused-deputy execution-provider calls;
- credential leakage through request, state, log, metric, trace, error, or
  report surfaces;
- SSRF, path traversal, command injection, and arbitrary process execution at
  provider boundaries;
- oversized payloads and event/report amplification;
- unauthorized event, report, and artifact reads;
- ambiguous external effects and unsafe retries;
- database privilege and network posture;
- dependency and image supply-chain risk.

The alpha should run with least-privilege database identities and separate API
and worker credentials where feasible. It must not persist authentication
tokens or provider credentials in domain records.

## 19. Implementation Sequence

Implement the milestone as one governed vertical build:

1. Define transport-neutral hosted request/result and execution-provider
   contracts in Rust.
2. Add authenticated API wiring for validation, run creation, inspect,
   approval, cancellation, and report retrieval.
3. Add bounded runnable-work discovery over `PostgreSQL`.
4. Add stateless worker claim/lease/fence processing against immutable bundles.
5. Add one no-write execution-provider proof, then one separately reviewed
   access-material path if required by the accepted scope.
6. Compose policy, proportional governance, approval presentation, current
   authority, evidence/check facts, capability, SideEffect, and report gates.
7. Add operational telemetry, health, diagnostics, shutdown, and recovery
   behavior.
8. Add deployment and recovery proof.
9. Run phase-level security and maintainer review.

The no-write provider proof should precede any hosted provider mutation. The
existing GitHub PR-comment sandbox must not be made remotely available merely
because the hosted service exists.

## 20. Test And Validation Plan

Future implementation tests must cover:

- authenticated and unauthenticated API behavior;
- version, size, and content-type rejection;
- idempotent run creation and conflicting key reuse;
- immutable run-bundle identity across API and worker processes;
- exact approval presentation and decision proof;
- current-authority reassessment at time of use;
- quiet capture, disclosure, blocking approval, and denial boundaries;
- event ordering and projection reconciliation;
- worker contention, lease renewal, expiry, takeover, and stale-fence rejection;
- API and worker restart safety;
- cancellation before claim, before invocation, and during ambiguous execution;
- bounded retries and non-retryable governance failures;
- execution-provider request/receipt validation;
- missing provider and missing access-material resolver fail closed;
- secret-like values absent from Debug, errors, serialization, logs, metrics,
  audit, evidence, reports, and state;
- report/artifact retrieval and integrity;
- PostgreSQL concurrency and recovery against a live service;
- complete API-to-worker-to-terminal-report vertical slice;
- compatibility of existing local CLI/executor behavior;
- no provider write, file artifact, or external call in default tests;
- docs and known-limitations honesty.

Required validation should include the existing workspace suite, live
`PostgreSQL` conformance, hosted integration tests, dependency audits, strict
Clippy/Rustdoc, docs checks, and an isolated deployment/recovery rehearsal.

## 21. Acceptance Criteria

The milestone is accepted only when:

1. a separately running authenticated API creates and inspects a governed run;
2. a separately running stateless worker claims that run under a fenced lease;
3. the run remains bound to an immutable bundle;
4. approval cannot proceed without exact presentation proof and current
   authority;
5. no work executes without current policy, capability, evidence/check, and
   SideEffect posture;
6. an execution provider receives only the authorized bounded request;
7. worker loss and stale commits fail safely;
8. cancellation and ambiguous outcomes remain explicit;
9. a terminal report cites stable evidence/receipt references;
10. credentials and raw payloads do not appear in durable or diagnostic
    surfaces;
11. API/worker restart and database restore proofs pass;
12. existing local behavior remains compatible;
13. the docs call the result a single-tenant hosted alpha, not production,
    multi-tenant, or enterprise ready.

## 22. Known Limitations At Alpha

Even after implementation, the alpha will still lack:

- multi-tenant isolation;
- enterprise identity and delegated administration;
- collaborative workflow/catalog management;
- general hosted execution substrates;
- broad write-capable adapters;
- production HA, PITR, capacity, and SLO proof;
- UI and notification systems;
- production secret management;
- nested harness runtime;
- reasoning lineage.

These limitations are product boundaries, not hidden follow-ups.

## 23. Open Questions

- Which deployment-configured authentication mechanism is the smallest
  credible alpha boundary?
- Should validation accept an uploaded canonical bundle or only a
  pre-registered catalog reference?
- What durable runnable-work index can be derived without introducing a second
  queue source of truth?
- What is the first no-write execution provider used for end-to-end proof?
- Is one reviewed access-material path necessary for alpha acceptance, or
  should all credentials remain deferred until after the no-write proof?
- Which API response fields are stable preview contracts versus experimental?
- What bounded retention posture is required for events and report artifacts?
- What cancellation signal can the first execution provider honestly support?
- Which metrics are necessary for alpha operations without high-cardinality
  identifiers?
- What exact deployment topology will be used for the recovery rehearsal?

## 24. Final Recommendation

Proceed next with the **single-tenant hosted alpha implementation milestone** as
one accelerated vertical build.

Start with transport-neutral service and execution-provider contracts, then
deliver the authenticated API, stateless fenced worker, no-write provider
proof, governance composition, observability, and recovery behavior inside the
same governed milestone.

Do not begin multi-tenancy, enterprise administration, OpenShell integration,
additional provider mutations, workflow schema expansion, or production
readiness work first.
