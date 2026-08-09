# OpenShell Pinned CLI Compatibility Transport Review

## 1. Executive Verdict

Phase accepted with security-critical follow-ups before live integration.

The OpenShell v0.0.101 CLI transport is an appropriately narrow,
fail-closed compatibility boundary. It does not implement
`OpenShellNoWriteClient`, does not execute a governed operation, and does not
convert requested configuration or incomplete CLI output into runtime
attestation. That is the correct result for this phase.

The transport is not yet sufficient for live provider wiring. The next phase
must harden the compatibility boundary and resolve upstream observation gaps
before Workflow OS can claim a sandboxed execution proof.

## 2. Scope Verification

The phase stayed within its approved compatibility scope.

Implemented:

- exact reviewed CLI version and upstream commit constants;
- explicit absolute binary path, workspace, and digest-pinned image input;
- fixed argv invocation without a shell or caller-selected operation;
- manual approval mode and disabled provider auto-creation;
- bounded process duration, stdout, and stderr;
- strict structured sandbox and effective-policy parsing;
- bounded, non-leaking errors and redacted Debug output; and
- focused compatibility fixtures and tests.

Not introduced:

- OpenShell installation, gateway, driver, or live sandbox use;
- `OpenShellNoWriteClient` implementation;
- automatic provider selection;
- arbitrary commands, environment values, credentials, or access material;
- provider writes or new mutation families;
- OCSF, artifact, or cleanup evidence fabrication;
- workflow schemas, examples, broad CLI behavior, or release changes; or
- an OpenShell fork.

## 3. Architecture Boundary Assessment

The boundary preserves the intended architecture:

```text
Agent executes. Workflow OS governs.
```

OpenShell remains a prospective optional execution substrate. Workflow OS
retains responsibility for governed request identity, policy and approval
obligations, evidence acceptance, receipt validation, and reporting. The
transport does not collapse those responsibilities into the sandbox runtime.

Keeping the compatibility transport disconnected from
`OpenShellNoWriteClient` is the most important design decision in this phase.
The existing provider contract requires facts that the reviewed CLI cannot
currently prove. The implementation fails closed instead of weakening that
contract.

## 4. Version And Supply-Chain Assessment

The transport executes `openshell --version` before each reviewed operation
and rejects any output other than `openshell 0.0.101`. This is useful protocol
compatibility validation.

It is not binary provenance or supply-chain attestation. An absolute path can
still be replaced between version verification and operation, and a different
binary can claim the expected version. The upstream commit constant records
the reviewed source but does not prove that the invoked executable was built
from that commit.

Before live integration, Workflow OS needs an explicit binary trust posture,
such as an installer-owned immutable location plus digest/signature
verification. Version output must continue to be described as compatibility
evidence only.

## 5. Command And Subprocess Safety Assessment

The subprocess boundary is conservative:

- `Command` receives a fixed argument vector directly;
- no shell is involved;
- stdin is closed;
- the caller cannot select an arbitrary command;
- provider auto-creation is disabled;
- approval mode is manual;
- stdout and stderr are captured with bounds; and
- nonzero exits, timeouts, malformed output, and process errors return bounded
  errors without copying subprocess output.

Two hardening issues remain before live use:

1. Successful stderr is captured but ignored. A zero-exit warning or degraded
   posture emitted only on stderr could therefore be lost. The live boundary
   must reject nonempty stderr or define a reviewed structured warning channel.
2. Stream overflow is safe but can become diagnostically imprecise if the
   child blocks after a bounded reader stops. The result may surface as a
   timeout instead of an explicit output-bound failure.

The second issue is non-blocking for this disconnected phase. The first is a
live-integration blocker.

## 6. Structured Response And Policy Assessment

`deny_unknown_fields` is appropriate for the pinned security-sensitive
response shapes. Identity, lifecycle, effective status, version, policy
source, policy body, and configuration revision are validated without parsing
human-oriented CLI text.

The effective-policy response correctly requires `version == active_version`
and computes a Workflow OS canonical hash over the full parsed policy payload.
The upstream provider hash is retained separately and not presented as the
Workflow OS canonical hash.

The following coherence gaps block live client implementation:

- detailed sandbox output carries both `current_policy_version` and
  `revision`, but the transport does not require them to match;
- sandbox inspection and effective-policy inspection are separate subprocess
  calls, so policy state can change between observations;
- no composite snapshot binds sandbox identity, lifecycle resource version,
  effective policy revision, configuration revision, and policy hash at one
  accepted observation boundary; and
- the absolute policy path is not yet bound to reviewed bytes or a canonical
  policy hash before sandbox creation.

The next implementation should create one reconciled inspection result and
fail closed on any cross-surface mismatch. It must not treat two independently
valid responses as an atomic attestation.

## 7. Runtime Attestation Gap

The phase report correctly identifies the decisive upstream gaps:

- no driver-observed immutable runtime-image identity in the reviewed
  structured CLI output;
- no complete machine-readable OCSF observation surface for the fixed
  operation; and
- no machine-readable cleanup confirmation suitable for the provider
  contract.

The configured digest-pinned image is a request, not execution evidence. A
sandbox ID, effective policy, or zero exit status cannot substitute for those
missing facts. `OpenShellNoWriteClient` must remain unimplemented until the
facts can be obtained from a trustworthy upstream API or independently
verified observation surface.

## 8. Privacy And Error Assessment

No payload-leakage blocker was found.

Debug output redacts binary paths, workspace names, image references, sandbox
identities, and hashes. Errors expose stable categories and attempt posture,
not subprocess output, policy payloads, paths, tokens, provider values, or
credentials. Structured output remains inside the hosted transport and is not
copied into Core events, evidence summaries, or reports.

The current image-reference validator is sufficient to prevent shell
injection because no shell is used, but it is not a complete OCI reference
parser. A stricter parser is a non-blocking compatibility follow-up and should
be completed before external configuration reaches this API.

## 9. Test Quality Assessment

The seven focused tests cover the most important current claims:

- reviewed response shapes parse;
- exact version mismatch fails before sandbox creation;
- fixed manual/no-provider argv is used;
- mutable image and relative binary configuration fail safely;
- unknown response fields fail closed;
- stale effective-policy versions fail closed; and
- Debug output does not expose configured or returned identities.

Missing tests align with the live-integration blockers:

- sandbox `current_policy_version` and detailed `revision` mismatch;
- nonempty stderr on a successful exit;
- policy drift between sandbox and effective-policy observations;
- binary digest/provenance verification;
- exact policy-file byte/hash binding;
- output-bound behavior at the real process boundary;
- driver-observed image identity;
- OCSF observation reduction; and
- cleanup proof.

The final three cannot be added honestly until OpenShell exposes trustworthy
machine-readable facts.

## 10. Documentation Assessment

The implementation report and runtime documentation are accurate. They
describe the transport as a compatibility building block, identify the pinned
version and upstream commit, disclose the attestation gaps, and explicitly
deny live execution and production-readiness claims.

No dangerous false claim requiring a documentation correction was found.

## 11. Current-Phase Blockers

None.

The current phase is acceptable precisely because the transport remains
disconnected and makes no execution-evidence claim.

## 12. Blockers Before Live Integration

1. Establish binary provenance beyond self-reported CLI version.
2. Reconcile sandbox and effective-policy observations into one accepted,
   drift-detecting snapshot.
3. Reject or explicitly model successful stderr warnings.
4. Bind the sandbox policy input to reviewed bytes and a canonical hash.
5. Obtain driver-observed immutable runtime-image identity.
6. Obtain complete structured execution observations, including denied egress.
7. Obtain machine-readable cleanup confirmation.

None of these blockers should be bypassed by caller assertion, requested
configuration, human-formatted output, or synthetic evidence.

## 13. Non-Blocking Follow-Ups

- use a complete OCI image-reference parser;
- validate structured timestamps if they become evidence-bearing;
- preserve the configured per-run stream bound through join validation;
- distinguish output-bound failures from timeout failures; and
- document the relationship, if any, between OpenShell's provider hash and
  Workflow OS canonical policy hash.

## 14. Recommended Next Phase

Proceed to OpenShell compatibility hardening and upstream attestation contract
resolution.

The next phase should remain local, optional, and non-executing. It should
define the reconciled snapshot and warning policy, add the locally testable
coherence regressions, and document or prototype the upstream API facts needed
for image, observations, and cleanup. Do not implement
`OpenShellNoWriteClient`, run a live sandbox, add credentials, enable provider
writes, select OpenShell automatically, or fork OpenShell in that phase.

## 15. Governed Review Evidence

- Workflow: `dg/review`.
- Run ID: `run-1786256462178763000-2`.
- Approval ID:
  `approval/run-1786256462178763000-2/review-scope-approved`.
- Approval presentation ID: `presentation/08db1eb77aac02cd`.
- Approval outcome: granted by delegated maintainer.
- Approved scope: focused maintainer/security review of version pinning, fixed
  argv, subprocess bounds, strict parsing, error safety, tests, documentation,
  and the intentionally disconnected provider boundary.
- Out-of-kernel work: Codex inspected Workflow OS and pinned upstream source,
  wrote this review, and ran validation. The kernel governed scope and
  approval but did not inspect source, edit files, execute tests, or perform
  git and pull-request actions.
