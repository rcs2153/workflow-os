# OpenShell No-Write Provider Threat Model

## Boundary

Workflow OS authorizes and records governed work. OpenShell contains execution.
Neither boundary substitutes for the other.

This threat model covers the optional injected provider implementation. It
does not claim that a live OpenShell transport or sandbox has been tested.

## Protected Invariants

- No request with approved SideEffects or access material reaches sandbox
  creation.
- Only read capabilities are accepted.
- The executed operation is provider-owned and fixed, not caller-supplied.
- Requested policy identity is not treated as effective-policy attestation.
- A completed receipt requires enforce mode and enforced filesystem, process,
  and network controls.
- Effective policy revision and digest remain stable through execution.
- Denied egress is supported by a bounded count and stable denied-action
  reference.
- Sandbox cleanup is confirmed or the result becomes ambiguous.
- Core receives references and bounded counts, not raw provider payloads.
- The provider cannot append Workflow OS events or mutate workflow state.

## Threats And Responses

### Requested Policy Is Echoed Without Enforcement

Risk: a provider repeats the requested policy hash while loading a different
or weaker policy.

Response: the client must return the full effective-policy digest and revision;
the provider compares the digest with the request and validates hard-control
posture. A real transport must canonicalize the effective provider-composed
policy rather than trusting request echo.

### Policy Changes During Execution

Risk: dynamic network policy broadens after preflight attestation.

Response: the provider reinspects after execution and requires identical
sandbox identity, policy revision, and policy digest. Drift fails closed as a
possibly-started policy failure.

### Control Degradation Is Hidden

Risk: audit, best-effort, skipped, unavailable, or unsupported controls are
reported as success.

Response: completed receipts require `enforce` and `enforced` posture for all
three control families. Other values remain representable for failure and
diagnostic posture but cannot satisfy hard requirements.

### Egress Denial Is Fabricated

Risk: a provider claims default-deny behavior without evidence.

Response: the fixed outcome requires a non-zero denied-network count and a
stable denied-action reference. The real transport must derive both from
structured OpenShell observations.

### Cleanup Is Lost Or Ambiguous

Risk: a successful command leaves a live sandbox or retained data.

Response: no receipt is returned until deletion succeeds. Deletion failure
becomes `Ambiguous` with `MayHaveStarted`, blocking ordinary retry and requiring
reconciliation.

### Raw Data Leaks Through Governance Records

Risk: source, paths, commands, URLs, environment values, policy bodies, or logs
appear in errors, Debug output, events, evidence, or reports.

Response: public models contain validated identifiers, hashes, enum posture,
counts, and stable references. Debug implementations redact identity. Errors
use stable codes and bounded messages.

### Sandbox Containment Is Overclaimed

Risk: scripted-client tests are described as live OpenShell enforcement.

Response: runtime documentation and the implementation report explicitly state
that the real transport, installation, compatibility pin, and smoke proof are
not implemented. The provider is not selected by the default binary.

## Deferred Threats

Credential injection, inference routing, provider mutations, arbitrary agent
commands, multi-tenancy, remote identity, hosted administration, cryptographic
attestation, and production vulnerability response remain out of scope. They
require separate threat models and reviews.
