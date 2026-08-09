# Optional OpenShell No-Write Provider

Workflow OS can now host an optional OpenShell no-write provider behind the
existing `HostedExecutionProvider` boundary. Workflow OS remains the source of
truth for workflow identity, immutable inputs, authority, policy and approval
decisions, durable attempts, terminal events, evidence references, and reports.
OpenShell is the intended containment substrate for process, filesystem, and
network controls.

## Current Status

Implemented:

- provider-neutral runtime attestation in `workflow-core`;
- exact effective-policy revision and digest posture;
- runtime image, enforcement mode, hard-control, observation, and cleanup
  posture;
- provider-required attestation validation;
- provider-agnostic injection in the hosted worker;
- an optional OpenShell no-write lifecycle provider;
- an injected `OpenShellNoWriteClient` transport boundary;
- one provider-owned fixed-operation path;
- denied-egress proof and bounded observation references;
- fail-closed policy drift, degraded-control, and cleanup behavior;
- scripted-client conformance tests.

Not implemented:

- a complete system OpenShell CLI, Rust SDK, or Python SDK provider client;
- OpenShell installation or gateway configuration;
- a live pinned image or complete provider response fixture;
- a live sandbox smoke test;
- automatic provider selection;
- caller-selected commands;
- credentials, inference routing, provider writes, or external SideEffects;
- a new CLI, workflow schema, or SDK surface;
- a production-hosted or multi-tenant claim.

The default `workflow-os-hosted` binary continues to use the inert
`NoWriteHostedExecutionProvider`. Merely compiling the OpenShell provider does
not start a gateway, create a sandbox, or alter existing execution behavior.

## Lifecycle Contract

The injected client must implement four operations:

1. create a sandbox for the exact hosted request;
2. execute the provider-owned fixed no-write operation;
3. inspect the effective sandbox posture after execution;
4. delete the sandbox and return a stable cleanup reference.

The provider validates before and after execution that:

- the runtime image digest is the configured digest;
- the effective policy digest equals the requested policy digest;
- the policy revision did not change;
- enforcement mode is `enforce`;
- filesystem, process, and network controls are enforced;
- allowed network, policy-change, and security-finding counts are zero;
- process start and terminal observations exist;
- at least one denied network observation and denied-action reference exist;
- cleanup completes with a telemetry reference.

A mismatch after sandbox creation is a possibly-started failure. Cleanup is
attempted. Cleanup uncertainty becomes an ambiguous outcome requiring
reconciliation; it is never converted into success.

## Transport Requirements

A future real client must pin a reviewed OpenShell release and image, consume
machine-readable interfaces, canonicalize the full effective policy including
provider-composed entries, and map only bounded observations into Workflow OS.
It must not parse human-oriented CLI prose or copy raw policy, source, command,
environment, URL, stdout, stderr, or provider payload content into Core.

## Pinned CLI Compatibility Boundary

Workflow OS includes `OpenShellCliTransport` for the reviewed OpenShell
v0.0.101 release at commit
`8ddd98c3dff62619a3963f99ba1e055b67650e72`. The transport:

- requires an absolute CLI path and digest-pinned image;
- verifies the exact `openshell 0.0.101` version;
- invokes fixed argv directly without a shell;
- bounds process time plus stdout/stderr;
- disables provider auto-creation and selects manual sandbox policy approval;
- strictly parses reviewed sandbox create/get and effective-policy JSON; and
- returns stable, non-leaking transport/protocol failures.

It is not wired as `OpenShellNoWriteClient`. The pinned structured CLI does not
currently expose the driver-observed immutable image identity, complete OCSF
observations, or machine-readable cleanup confirmation required by that
contract. Operator-supplied image text, labels, annotations, and human CLI
output are not accepted as substitutes for provider-observed evidence.

OpenShell installation changes the developer machine by installing a gateway
service and requiring a compute driver. Installation and the live smoke proof
therefore require a separate explicit governed phase; they are not hidden test
setup.

## Relationship To Proportional Governance

Sandbox selection and approval remain separate. A policy may require OpenShell
containment while proportional governance still permits quiet execution for a
fully authorized no-write operation. Conversely, sandbox containment never
bypasses a blocking approval, denial, evidence obligation, or policy gate.

## Next Proof

The next phase should review the pinned compatibility transport and resolve the
missing runtime-image, OCSF, and cleanup attestation surfaces through an
upstream/API-compatible boundary. Only then should one explicit local sandbox
smoke test exercise a fixed no-write command, hard controls, denied egress,
effective policy reinspection, evidence collection, cleanup, and exact receipt
binding.
