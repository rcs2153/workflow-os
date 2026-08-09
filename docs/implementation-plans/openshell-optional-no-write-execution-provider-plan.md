# OpenShell Optional No-Write Execution Provider Vertical Slice Plan

Status: accepted for implementation in the
[focused plan review](../concepts/OPENSHELL_OPTIONAL_NO_WRITE_EXECUTION_PROVIDER_PLAN_REVIEW.md).
The first implementation slice now adds provider-neutral attestation,
provider-agnostic worker injection, and an optional OpenShell lifecycle
provider behind an injected client boundary. A real OpenShell CLI/SDK client,
installation, pinned compatibility fixture, and live sandbox smoke proof remain
unimplemented and are required before this complete milestone can be accepted.

The pinned CLI compatibility transport has since received a bounded hardening
slice: caller configuration now binds an expected executable digest, every
subprocess invocation checks that digest before and after execution, successful
stderr fails closed, detailed policy revision/source coherence is enforced,
and a before/policy/after reconciliation helper rejects observable drift. This
is compatibility hardening only. It is not atomic attestation and does not
resolve exact policy-file byte binding, driver-observed image identity, complete
structured observations, or machine-readable cleanup proof.

## 1. Executive Summary

Workflow OS should integrate NVIDIA OpenShell as an optional execution
provider, not fork it and not make it the default runtime.

Workflow OS remains the governance and audit authority. It decides whether an
exact immutable step may execute, which capability and policy obligations
apply, whether approval is required, and which evidence and report disclosures
must result. OpenShell owns the bounded execution environment: process,
filesystem, network, sandbox lifecycle, credential routing, and local security
telemetry.

The first implementation should be one explicit, no-write, deterministic
vertical slice behind the existing `HostedExecutionProvider` boundary. It must
strengthen the provider-neutral receipt so the result proves the effective
loaded sandbox policy and enforcement posture, not merely the policy Workflow
OS requested. It must remain local or single-tenant evaluation software and
must not broaden provider mutations.

This plan follows the accepted
[Expansion Readiness Review](../concepts/EXPANSION_READINESS_REVIEW.md).

## 2. Architecture Decision

Choose **optional execution provider**.

Do not choose a Workflow OS-specific OpenShell fork. A fork would transfer a
large runtime and security maintenance surface into Workflow OS: container and
VM lifecycle, kernel controls, process isolation, network mediation,
credential injection, platform compatibility, vulnerability response, and
enterprise hardening.

Do not continue with native local handlers only. Native handlers remain useful
and supported, but they cannot prove runtime containment. An optional
OpenShell provider lets users choose a stronger execution boundary when policy
requires it without making every low-risk local check pay sandbox startup cost.

## 3. Goals

- Execute one already-authorized, immutable, no-write workflow step inside an
  OpenShell sandbox.
- Keep Workflow OS as the only source of workflow, policy, approval, authority,
  SideEffect, evidence, event, and WorkReport truth.
- Bind provider invocation to the exact hosted request and durable attempt.
- Prove requested and effective policy identity, revision, enforcement, and
  degradation posture.
- Record stable sandbox, log, denied-action, telemetry, artifact, cleanup, and
  reconciliation references without copying raw payloads into Core.
- Preserve restart safety and avoid blind repeat after ambiguous invocation.
- Produce report-ready evidence from the exactly bound receipt.
- Keep the provider explicit, optional, replaceable, and testable through an
  injected client boundary.
- Demonstrate useful execution rather than only model vocabulary.

## 4. Non-Goals

The first slice does not implement:

- an OpenShell fork or Workflow OS runtime distribution;
- automatic or default sandboxing;
- provider writes or external mutations;
- GitHub, Jira, or other provider mutation expansion;
- access-material or credential injection;
- inference-provider routing;
- arbitrary agent execution;
- interactive sandbox shells;
- hot policy broadening during execution;
- workflow schema or SDK fields;
- a new CLI command;
- hosted multi-tenancy or enterprise administration;
- production identity, RBAC, IdP, OIDC, SSO, or SCIM;
- cryptographic attestation or trusted-computing claims;
- automatic retries after a possibly started invocation;
- raw log, command-output, source-content, or provider-payload persistence;
- reasoning lineage, nested harness execution, or agent teams;
- production-readiness or release-posture changes.

## 5. Existing Workflow OS Foundations

The slice should reuse, not duplicate:

- `HostedExecutionRequest` and deterministic request fingerprints;
- `HostedExecutionProvider` as the injected execution boundary;
- durable `HostedExecutionAttempt` posture;
- immutable run-bundle identity and integrity roots;
- scheduled skill invocation and Core-owned hosted work creation;
- exact provider receipt validation and terminal result projection;
- provider failure and reconciliation-required projection;
- scoped capability/current-authority foundations;
- proof-enforced approval presentation and decision records;
- proportional-governance assessment and route selection;
- payload-free hosted reference vocabulary;
- EvidenceReference, WorkReport, and report-artifact foundations;
- SQLite/PostgreSQL state semantics and hosted recovery proof.

OpenShell must not append workflow events, mutate snapshots, grant authority,
construct approvals, or determine terminal workflow status directly.

## 6. Source-Of-Truth Boundaries

| Concern | Source of truth | Not sufficient |
| --- | --- | --- |
| Workflow and step identity | Stored immutable run bundle | Sandbox labels or command text |
| Permission to invoke | Fresh Workflow OS authority, policy, approval, and proportional-governance result | Tool visibility or sandbox availability |
| Requested containment | Workflow OS execution policy binding | Provider defaults |
| Effective containment | OpenShell effective policy revision plus enforcement/degradation observations | Requested policy hash echoed in a receipt |
| Invocation identity | Durable hosted attempt and request fingerprint | Sandbox name alone |
| Execution outcome | Exactly bound provider receipt validated by Core | CLI exit text or log presence |
| Security observations | OpenShell structured events referenced by the receipt | Human summary |
| Workflow events and status | Workflow OS Core projection | OpenShell lifecycle state |
| Evidence and report posture | Workflow OS references and WorkReport | Raw sandbox logs |

## 7. Target Runtime Flow

```text
validated immutable run bundle
  -> current authority and capability resolution
  -> proportional-governance decision
  -> required approval/policy/check gates
  -> Core-owned scheduled no-write invocation
  -> durable hosted request and attempt
  -> optional OpenShell provider adapter
  -> sandbox creation with pinned image and requested policy
  -> effective policy and enforcement attestation
  -> deterministic no-write command
  -> bounded artifact and OCSF reference collection
  -> sandbox cleanup or reconciliation-required posture
  -> exactly bound hosted receipt
  -> Core-owned terminal workflow event projection
  -> EvidenceReference and WorkReport artifact composition
```

The adapter receives a request only after Workflow OS has authorized the exact
operation. A successful sandbox launch does not imply the step succeeded. A
successful command does not imply the requested containment was enforced.

## 8. Minimal Prototype

Use one deterministic repository validation/check operation with no network or
external SideEffect requirement.

The prototype should:

1. Require explicit provider selection through an internal API or test fixture.
2. Pin an accepted OpenShell release/commit and sandbox image by immutable
   identity or digest.
3. Materialize one immutable input workspace as read-only.
4. Provide one bounded writable output directory for generated validation
   artifacts only.
5. Configure hard-requirement filesystem/process controls.
6. Configure default-deny network policy with no allowed endpoint for the
   primary command.
7. Enable OCSF JSON export before command execution.
8. Wait for the policy revision to be loaded, fetch the full effective policy,
   and bind its digest and revision.
9. Execute one fixed command selected by the injected provider fixture, not
   caller-supplied arbitrary shell text.
10. Capture exit posture and bounded artifact metadata.
11. Exercise one deliberate denied-egress probe as separate security proof.
12. Collect stable OCSF/log references and selected bounded counts.
13. Delete the sandbox and record cleanup completion.
14. Return one exactly bound receipt for Core validation and projection.

The denied-egress probe must not make the primary validation command depend on
network access. Its expected denial should be represented as security evidence,
not as workflow failure.

## 9. Upstream Version And API Boundary

OpenShell is alpha software and its APIs may change without notice. The
implementation phase must:

- choose one reviewed release or commit after a compatibility spike;
- pin the CLI/SDK/gateway and image identities used by tests;
- record the exact versions in provider configuration identity;
- prefer a documented SDK or gateway API;
- use version-pinned machine-readable CLI output only if no stable SDK/API
  covers the first slice;
- prohibit human CLI text parsing in the reviewed adapter;
- add compatibility fixtures for every consumed response shape;
- fail closed on unknown fields that alter security posture and on missing
  required attestation fields;
- document the upgrade and rollback process.

The first implementation must not use a remote install script at runtime. CI
or development installation must be separately explicit and checksum/pin
aware.

The compatibility spike selected OpenShell v0.0.101 at upstream commit
`8ddd98c3dff62619a3963f99ba1e055b67650e72`. A bounded CLI compatibility
transport now consumes the reviewed create, sandbox-get, and full effective
policy JSON shapes. It verifies the exact CLI version and requires a
digest-pinned image. It is intentionally not an `OpenShellNoWriteClient`:
v0.0.101 structured CLI output does not expose the driver-observed immutable
image identity, complete OCSF observations, or machine-readable cleanup
confirmation required by the provider contract. Requested image identity is
not relabeled as observed runtime evidence. The integrated milestone therefore
remains blocked on an upstream/API attestation surface and a live smoke proof.

The compatibility hardening does not change that milestone status. The
reconciled CLI snapshot detects drift visible across separate observations but
cannot make them atomic. Live provider wiring must continue to fail closed
until the remaining upstream and policy-input binding facts are trustworthy.
The required evidence boundary is now specified in the
[OpenShell Upstream API Attestation Contract Plan](openshell-upstream-api-attestation-contract-plan.md),
which remains planning-only and requires review before any implementation.

That contract is now accepted by focused review, and the
[OpenShell v0.0.101 Evidence-Sufficiency Matrix](openshell-v0-0-101-evidence-sufficiency-matrix.md)
has evaluated the exact pinned upstream protobuf and implementation surfaces.
The matrix confirms that effective-policy and sandbox-identity facts are
usable, but provider wiring remains blocked on invocation idempotency,
driver-observed image identity, durable operation outcome, complete
observation export, exact cleanup, reconciliation, and capability negotiation.

## 10. Provider-Neutral Contract Hardening

The current request/receipt contract binds requested policy, provider identity,
configuration, timing, terminal status, exit status, and stable references. It
needs a bounded attestation addition before OpenShell use.

Candidate domain-neutral types:

- `HostedExecutionEnvironmentAttestation`
- `HostedExecutionPolicyAttestation`
- `HostedExecutionControlPosture`
- `HostedExecutionCleanupPosture`
- `HostedExecutionObservationSummary`

The exact names should follow repository conventions after implementation
inspection. The contract should remain provider-neutral and payload-free.

Required receipt additions:

- runtime environment/image identity;
- effective policy revision and canonical digest;
- requested/effective policy relationship;
- filesystem, process, and network enforcement posture;
- explicit degraded, skipped, unavailable, or unsupported controls;
- structured observation/log reference and bounded event counts;
- cleanup status and timestamp/reference;
- ambiguity and reconciliation posture.

The request fingerprint must commit every new requested security field. The
receipt must bind every attested field to the exact execution ID, request,
provider identity/version/configuration, and durable attempt.

## 11. Policy Translation And Attestation

Workflow OS should not mirror the complete OpenShell policy language.

Define one narrow internal provider profile for the prototype:

- read-only input workspace;
- one writable output workspace;
- non-root process identity;
- hard-requirement Landlock/filesystem posture where supported;
- default-deny network;
- no provider credentials or inference routes;
- OCSF JSON enabled;
- fixed CPU, memory, and timeout budget where the selected driver exposes them.

The adapter renders that profile into the pinned OpenShell policy shape. It
then fetches the full effective policy, including provider-composed entries,
and computes a canonical digest over the machine-readable representation.

The adapter must reject:

- failed or timed-out policy load;
- a revision change between attestation and command start;
- audit-only enforcement where enforce was requested;
- best-effort degradation where hard requirement was requested;
- missing or skipped filesystem controls;
- unexpected allowed network endpoints;
- unexpected credential or inference-provider entries;
- unsupported required process controls;
- an effective policy that cannot be canonicalized deterministically.

Dynamic policy changes during execution must be disabled by ownership/config or
detected through revision/config events. A relevant revision change makes the
outcome ambiguous or failed according to whether execution may have started.

## 12. Process, Filesystem, And Network Posture

The first slice must prove, not assume:

- the child runs under the expected non-root identity;
- the input path is read-only;
- writes outside the output/temp allowance are denied;
- the output path is writable;
- the configured filesystem mechanism is active at the required compatibility
  level;
- outbound network is default-deny;
- the deliberate egress probe is denied and represented in structured events;
- no host credential files, sockets, or broad workspace mounts are exposed;
- command timeout and cancellation terminate or reconcile the child;
- sandbox deletion completes or is represented as cleanup ambiguity.

Platform-specific absence of Landlock, seccomp, or another required control is
a rejected environment for this prototype, not a warning-only success.

## 13. Access Material And Credentials

The first slice requires an empty `access_material_references` collection and
must reject approved SideEffects or write capabilities.

OpenShell credential providers, request-body rewrite, inference routing, host
environment forwarding, SSH agent forwarding, and provider secrets remain out
of scope. The adapter must scrub its own environment and allow only an explicit
non-secret variable set required by the deterministic command.

Credential integration requires a later threat model covering:

- reference-to-secret resolution;
- least-privilege provider identities;
- issuer/audience/expiry and revocation;
- credential injection proof without value disclosure;
- network endpoint and method binding;
- log and artifact leakage;
- cleanup and provider-side revocation.

## 14. Evidence, Logs, And Artifacts

OpenShell OCSF JSON is a provider observation source, not the Workflow OS event
ledger. The adapter should reduce it to bounded facts and durable references.

The first receipt/report path should preserve:

- sandbox/environment reference;
- full effective-policy artifact reference and digest;
- OCSF JSONL artifact reference and digest;
- command/result artifact references and digests;
- allowed/denied network-event counts;
- process launch/exit/timeout counts;
- policy/configuration change counts;
- security finding counts by bounded severity/class;
- cleanup reference/posture;
- exact provider and image identity.

Do not copy raw event messages, command lines, paths, URLs, source, stdout,
stderr, environment values, or provider payloads into Core errors, Debug,
events, EvidenceReference summaries, or WorkReport text by default.

The implementation must account for OpenShell's bounded in-memory gateway log
buffer. Durable evidence must come from referenced sandbox OCSF files or an
explicit durable sink before sandbox deletion. Log retrieval failure after a
possibly successful command is a report/evidence failure that follows declared
workflow requirements; it must not fabricate complete evidence.

## 15. Lifecycle, Restart, And Reconciliation

Persist the durable attempt before sandbox creation.

The adapter must expose enough stable identity to reconcile after:

- client interruption during create;
- policy load timeout;
- worker restart before command start;
- worker restart while the command may be running;
- command completion before receipt commit;
- artifact/log collection interruption;
- sandbox delete interruption;
- gateway restart.

Safe rules:

- retry before confirmed sandbox creation may use exact idempotent lookup;
- never create a second sandbox while the first invocation may exist;
- never rerun the command when it may have started;
- recover by inspecting the exact sandbox and attempt identity;
- stale or changed policy makes the execution non-successful;
- cleanup ambiguity remains visible even when command execution succeeded;
- unresolved ambiguity escalates and blocks ordinary retry.

## 16. Proportional Governance And Operator UX

OpenShell selection and approval are separate decisions.

A policy may require sandbox containment while proportional governance still
selects quiet execution for a low-risk, fully authorized, no-write check. The
existence of a sandbox must not force human approval. Conversely, a blocking
approval cannot be bypassed merely because OpenShell contains execution.

Quiet success should disclose, durably and without interruption:

- provider and sandbox posture;
- effective policy attestation;
- control degradation (which must be absent for this prototype);
- denied-action summary;
- artifact/evidence completeness;
- cleanup result.

Visible disclosure remains an operator presentation obligation independent of
whether execution proceeds. Denial and blocking approval remain monotonic over
lower-friction modes.

## 17. Failure Semantics

Use stable, non-leaking classifications for:

- provider unavailable;
- version/configuration mismatch;
- sandbox create rejected;
- sandbox creation ambiguous;
- policy validation/load rejected;
- effective policy mismatch;
- required control degraded or unavailable;
- policy revision changed;
- process start rejected;
- process outcome ambiguous;
- timeout or cancellation;
- artifact/log collection incomplete;
- cleanup failed or ambiguous;
- receipt/request/attempt binding invalid;
- unsupported OpenShell response version.

Errors must never include raw command text, source paths, policy bodies, log
messages, URLs, environment values, access material, sandbox bootstrap data, or
provider response payloads.

## 18. Threat Model

The implementation plan must test these threats:

- caller substitutes a weaker policy or provider configuration;
- requested policy is accepted but a different effective policy loads;
- policy changes after attestation and before/during execution;
- unsupported kernel controls silently degrade;
- sandbox image changes under a mutable tag;
- command or artifact path escapes allowed roots;
- agent/process bypasses proxy or reaches link-local metadata services;
- logs or artifacts expose secrets or raw source;
- provider returns a receipt for another sandbox/request/attempt;
- worker crash causes duplicate sandbox or command execution;
- cleanup failure leaves a live sandbox with retained data;
- gateway log loss is mistaken for absence of denied activity;
- expected denial is fabricated without a structured observation;
- OpenShell or its adapter mutates Workflow OS state directly;
- sandbox containment is overclaimed for native handlers or other providers.

## 19. Implementation Milestone

After plan review, implement the first proof as one governed vertical milestone
rather than a chain of micro-phases. The milestone contains these workstreams:

1. Provider-neutral attestation and cleanup contract hardening in Core.
2. Injected OpenShell client/transport boundary in a provider-specific module or
   crate chosen to preserve current ownership.
3. Policy renderer, effective-policy verifier, and security-observation reducer.
4. No-write provider implementation with durable attempt and reconciliation.
5. Core-owned terminal event and report-artifact composition.
6. Fake-client conformance tests and one explicit real OpenShell smoke test.
7. Security/privacy review, documentation, operational runbook, and phase
   review.

If contract review finds a safety blocker, fix it inside the milestone before
live execution. Do not broaden scope into credentials or writes.

## 20. Test Plan

Core contract tests:

- valid attested no-write receipt;
- request, attempt, provider, configuration, image, policy, sandbox, or cleanup
  substitution fails closed;
- unknown wire values fail safely;
- deterministic serde and Debug/error non-leakage;
- no raw payload fields.

Adapter tests with injected client:

- create, attest, execute, collect, delete success;
- exact effective-policy digest and revision binding;
- audit-vs-enforce mismatch rejection;
- best-effort or skipped required control rejection;
- unexpected network/provider entry rejection;
- policy revision TOCTOU rejection;
- fixed command and path boundary enforcement;
- empty access material and SideEffect rejection;
- denied-egress observation proof;
- OCSF/artifact reduction and bounds;
- create/start/result/collection/delete ambiguity;
- restart reconciliation without duplicate execution;
- cleanup uncertainty remains visible;
- provider never mutates Core state.

End-to-end tests:

- completed governed no-write run yields exact terminal events and report
  artifact;
- failed command maps to bounded failure, not fabricated success;
- ambiguous invocation escalates and blocks retry;
- required evidence failure follows contract policy;
- quiet successful execution produces no approval unless another requirement
  demands it;
- native local handler behavior remains unchanged;
- no filesystem report outside approved stores and no provider mutation occurs.

Validation for implementation should include:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused Core and provider tests;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`;
- explicit pinned OpenShell smoke test where the reviewed environment is
  available.

## 21. Acceptance Criteria

The vertical slice is accepted only when:

- it is opt-in and no-write;
- one immutable governed step executes in OpenShell;
- Core derives and persists the exact request/attempt before invocation;
- effective policy revision/digest and required controls are attested;
- any degraded or unavailable required control fails closed;
- no access material or external SideEffect is accepted;
- a structured denied-egress observation is retained by stable reference;
- artifacts and OCSF evidence are bounded, integrity-bound, and report-ready;
- cleanup is complete or explicitly reconciliation-required;
- restart does not duplicate ambiguous execution;
- exact terminal events and WorkReport artifact are produced through Core;
- native handlers and existing defaults remain unchanged;
- no raw payload or secret-like value leaks;
- the pinned real smoke test and workspace regressions pass;
- a focused maintainer/security review accepts the complete boundary.

## 22. Fork Reconsideration Criteria

Forking remains prohibited unless a future ADR proves all of the following:

- upstream cannot expose required effective-policy, degradation, lifecycle,
  structured-event, artifact, cleanup, or reconciliation hooks;
- upstream contribution or plugin work has been attempted and rejected or is
  structurally impossible;
- the missing surface is a governance invariant, not convenience;
- Workflow OS has dedicated runtime-security ownership, vulnerability response,
  release engineering, and cross-platform capacity;
- the long-term cost is lower than maintaining an adapter or alternative
  provider.

## 23. Open Questions For Plan Review

- Which exact OpenShell release/commit and image digest should the proof pin?
- Does the selected release expose a stable SDK/gateway API for every required
  operation, or is a machine-readable CLI compatibility boundary needed?
- What canonical representation should bind the full effective policy?
- Can policy updates be disabled or exclusively owned for the sandbox lifetime?
- Which compute driver gives the most reproducible local/CI proof?
- Which required controls are consistently available on supported developer and
  CI platforms?
- Where should OCSF and artifacts live long enough to survive sandbox deletion?
- Does the existing hosted reference taxonomy cover policy, OCSF, and cleanup
  proof without adding provider-specific vocabulary?
- Should command selection come from a reviewed handler registry or one
  provider fixture in the first slice?
- What artifact/evidence failure should do to workflow terminal semantics when
  command execution itself succeeded?

## 24. Final Recommendation

Review this plan, then implement the optional OpenShell no-write provider as one
integrated vertical milestone. Keep it explicit, pinned, no-write,
access-material-free, and provider-neutral at the Core boundary.

Do not fork OpenShell. Do not add another provider mutation, automatic
sandboxing default, credential flow, schema exposure, multi-tenancy, or
production claim in the implementation phase.

## 25. Governed Planning Evidence

- Dogfood workflow: `dg/d`.
- Run ID: `run-1786226457510018000-2`.
- Approval ID:
  `approval/run-1786226457510018000-2/planning-approved`.
- Approval presentation ID: `presentation/256a7a1de482c324`.
- Approval outcome: granted by the delegated maintainer through the
  proof-enforced path.
- Event summary: 39 ordered events, including one approval request and grant,
  eight policy decisions, six scheduled steps, six successful skill
  invocations, and one completed run; no retries or escalations.
- Approved scope: planning and documentation only.
- Validation: `npm run check:docs` passed; `git diff --check` passed. Runtime
  tests were not run because this phase changed documentation only.
- Out-of-kernel work: Codex inspected repository and upstream OpenShell
  documentation, authored this plan, and will perform validation and git/PR
  operations. The kernel governed scope and approval but did not browse,
  install software, edit files, execute validation, or perform git/PR actions.
