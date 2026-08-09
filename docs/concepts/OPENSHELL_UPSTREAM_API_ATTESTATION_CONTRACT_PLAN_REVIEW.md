# OpenShell Upstream API Attestation Contract Plan Review

## 1. Executive Verdict

Plan accepted; proceed to a version-pinned upstream evidence-sufficiency
matrix.

The plan preserves the correct product boundary: Workflow OS remains the
governance, evidence-acceptance, and reporting layer, while NVIDIA OpenShell
may become an optional execution-containment provider. It does not authorize a
fork, default sandboxing, live execution, access material, or provider writes.

This verdict accepts the integration architecture, not provider readiness.
`OpenShellNoWriteClient` must remain disconnected until the next phase proves
that every required security fact has an authoritative, exactly bound source.

## 2. Scope Verification

The plan stayed within planning-only scope.

It defines:

- the optional-provider boundary;
- required invocation, policy, image, operation, observation, cleanup, and
  reconciliation facts;
- evidence-sufficiency classifications;
- candidate upstream surfaces;
- capability negotiation and fail-closed behavior;
- privacy, retry, and fork posture; and
- a staged path to one fixed no-write prototype.

It does not authorize:

- OpenShell installation, gateway startup, sandbox creation, or commands;
- provider wiring, automatic selection, or runtime configuration;
- access material, inference routing, or external writes;
- arbitrary commands, interactive shells, or agent teams;
- schemas, SDKs, examples, hosted administration, or release changes;
- an OpenShell fork; or
- production security or containment claims.

## 3. Product And Architecture Assessment

The optional-provider decision is the right strategic boundary.

Workflow OS should govern whether work may execute, bind immutable authority
and run identity, define required evidence, interpret policy and approval
obligations, validate returned facts, and project accepted results into audit,
evidence, and WorkReport surfaces. OpenShell should own sandbox lifecycle and
the actual filesystem, process, network, and inference containment it can
authoritatively enforce.

Treating OpenShell as a generic skill handler would understate the trust
boundary. It should be modeled as a typed execution provider with explicit
capability negotiation and attestation requirements. Treating it as the
governance source of truth would invert the architecture. Forking it now would
make Workflow OS responsible for a large security and platform surface before
the integration has demonstrated user value.

## 4. Required-Fact Assessment

The seven required fact groups are appropriately conservative:

1. invocation identity;
2. exact policy input;
3. effective policy and control state;
4. driver-observed runtime image;
5. fixed operation outcome;
6. complete structured observations; and
7. cleanup and restart reconciliation.

The plan correctly separates requested configuration from committed
control-plane state and driver-observed runtime state. This distinction is
load-bearing. A digest-pinned image request, local policy hash, sandbox ID, CLI
exit code, or accepted delete request cannot independently prove what executed
or whether cleanup completed.

No required fact should be weakened merely to complete the first integration.
The prototype should remain blocked if an authoritative source is unavailable.

## 5. Evidence-Sufficiency Assessment

The classifications `Authoritative`, `Corroborating`, `Requested`, `Lossy`,
and `Unavailable` are clear and suitable for the next phase.

The rule that multiple weak observations cannot manufacture an authoritative
fact is essential. It prevents a control-plane request, human CLI output, and
partial log stream from being combined into a false execution attestation.

The next matrix must add, for every fact:

- exact upstream release and schema revision;
- enforcing or observing component;
- API, SDK, CLI, file, or independent-observer surface;
- binding key and observation interval;
- completeness, drop, and drift posture;
- privacy reduction before Core; and
- accepted classification with source-backed rationale.

Unknown or disputed classifications must remain `Unavailable` rather than
optimistically inferred.

## 6. Upstream Surface Priority Assessment

The proposed priority is correct:

1. typed documented SDK or gateway API;
2. gateway or driver APIs exposing committed and observed state;
3. machine-readable CLI output backed by the same typed API;
4. complete sandbox-resident OCSF records reduced before deletion; and
5. narrow gateway interceptors for validation or notification only.

Gateway interceptors are useful governance hooks but are not execution
attestation. They cannot prove the image started by the driver, a complete
observation interval, or terminal resource deletion.

The next phase should inspect version-pinned schemas and official fixtures,
not merely current prose documentation. Human-oriented CLI text and tool
self-description remain unacceptable evidence surfaces.

## 7. Attempt, Retry, And Reconciliation Assessment

The plan aligns with the existing durable attempt model:

- failures proven before provider activity may be `NotStarted`;
- uncertainty after create, execute, or delete activity is
  `MayHaveStarted`;
- cleanup ambiguity blocks receipt completion; and
- automatic retry is prohibited when a provider mutation may have occurred.

Restart reconciliation must use a durable exact identity, reject substituted
or duplicate resources, and validate policy, image, operation, and resource
revision coherence before accepting any recovered result. A sandbox label or
best-effort lookup is insufficient.

This is appropriately strict for a provider that creates and deletes runtime
resources even when the governed operation itself is no-write.

## 8. Privacy And Reporting Assessment

The proposed Core boundary is sound: stable references, commitments, bounded
counts, typed postures, and timestamps may cross into Workflow OS. Raw policy,
OCSF, stdout, stderr, environment values, provider material, host paths, and
arbitrary artifacts may not.

The plan also correctly states that a reference is not proof of completeness
unless production, integrity, retention, and access posture are known. The
next matrix should make that distinction explicit for every proposed log or
artifact reference.

No provider error should copy OpenShell payloads, internal endpoints, sandbox
labels, or policy details into workflow diagnostics, audit summaries, Debug,
or serialized reports.

## 9. Fork Policy Assessment

The five-part fork threshold is appropriate and strategically disciplined.

A fork is not justified by convenience, release cadence, branding, CLI
stability, or Workflow OS-specific defaults. It should be reconsidered only
when a security-critical authoritative fact is unavailable, upstream cannot or
will not expose it, no trustworthy independent observer exists, the patch is
narrow, and Workflow OS explicitly accepts the maintenance and vulnerability
burden through a new ADR.

An upstream contribution, documented extension point, or narrowly maintained
adapter-side observer remains preferable even when the threshold is met.

## 10. Minimal Prototype Assessment

The proposed prototype is both compelling and appropriately small:

- one pinned upstream release, schema, driver, and immutable image;
- one provider-owned fixed no-write repository check;
- no access material and default-deny networking;
- exact policy and effective-policy binding;
- driver-observed image and hard-control verification;
- one expected denied-egress observation;
- complete structured-observation reduction;
- terminal deletion confirmation; and
- one exactly bound receipt or an explicit ambiguous result.

That prototype would prove the architectural value without turning Workflow OS
into a sandbox runtime. It must remain opt-in and non-production.

## 11. Risks And Missing Detail

No planning blocker was found. The next phase must resolve these uncertainties
before implementation:

- whether a current OpenShell driver exposes the image digest it actually ran;
- whether accepted policy bytes and fully composed effective policy share an
  exact revision boundary;
- whether complete OCSF records can be exported with integrity and drop
  posture before deletion;
- whether terminal absence can be proven rather than inferred from delete
  acceptance;
- whether a stable idempotency or client-request identity supports restart
  reconciliation; and
- which controls are truly hard requirements across supported drivers.

OpenShell's alpha maturity and moving APIs also create upgrade, supply-chain,
and compatibility risk. The plan correctly requires a complete re-evaluation
for each reviewed pin.

## 12. Test-Plan Assessment

The future test plan covers the important security and correctness claims,
including policy substitution and drift, image mismatch, degraded controls,
dropped observations, denied egress, cleanup ambiguity, restart
reconciliation, attempt posture, non-leaking errors, and non-fabrication.

The version-pinned matrix phase should add fixture provenance checks so a
recorded schema or response cannot silently drift from its documented upstream
release. Live proof remains separately scoped and must not be simulated by
fixtures.

## 13. Blockers

None for accepting this plan.

Provider implementation and live sandbox proof remain blocked until the
evidence-sufficiency matrix identifies an authoritative source for every
required fact or records it as unavailable.

## 14. Non-Blocking Follow-Ups

- Decide whether the first reviewed pin should track a release or exact commit.
- Record fixture acquisition and integrity posture in the matrix.
- Define how long evidence references must remain resolvable for the prototype.
- Clarify which component owns reduction of complete OCSF records before
  sandbox deletion.
- Keep compatibility-only CLI transport tests as regression coverage without
  treating them as attestation.

## 15. Recommended Next Phase

Proceed to a version-pinned OpenShell upstream evidence-sufficiency matrix
using official API schemas and fixtures, without live sandbox execution.

The matrix should classify every required fact, record exact binding and
completeness posture, identify unavailable facts, and recommend only the
smallest provider-neutral model amendments supported by evidence. Do not wire
`OpenShellNoWriteClient`, run a sandbox, add access material, enable writes,
select OpenShell automatically, expose schemas or examples, fork OpenShell, or
make production claims.

## 16. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1786266525844611000-2`.
- Approval ID:
  `approval/run-1786266525844611000-2/review-scope-approved`.
- Approval presentation ID: `presentation/ac4478234becc9e4`.
- Approval presentation hash:
  `ac4478234becc9e44e3f073a6126a2fedd9fb03e2f7f2ac21215d05f60488c38`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: focused maintainer review of the optional-provider boundary,
  authoritative facts, evidence classifications, candidate upstream surfaces,
  retry and reconciliation posture, privacy, fork threshold, and sequencing.
- Validation requirement: documentation check and diff check only; Rust
  validation was outside scope because no Rust change was authorized.
- Out-of-kernel work: Codex inspected the accepted plan and supporting
  contracts, authored this review, and ran documentation validation. The
  kernel governed scope and approval but did not inspect source, edit files,
  run checks, or perform git and pull-request actions.
