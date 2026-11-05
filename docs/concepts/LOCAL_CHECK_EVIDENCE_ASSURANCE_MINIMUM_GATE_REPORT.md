# Local Check Evidence Assurance Minimum Gate Report

## Status

Implemented and accepted after focused maintainer/security review.

## Summary

Workflow authors can now declare an optional, dimension-specific
`evidence_assurance_minimum` on a local-check requirement. The declaration is
validated through the public schema and TypeScript SDK, included in the
content-addressed canonical declaration set, recovered from the exact immutable
run bundle, and enforced against the Core-owned durable evidence-assurance
projection before workflow execution begins.

Existing declarations without the new field preserve their previous behavior.
The gate does not reinterpret the older `minimum_assurance` attestation-candidate
field and does not accept a caller-supplied runtime minimum outside immutable
workflow identity.

## Declaration

The optional declaration keeps assurance dimensions separate:

```yaml
local_check_requirements:
  - id: project-validation
    command_id: local-check/workflow-os-validate
    minimum_assurance: kernel_observed_local_process
    evidence_assurance_minimum:
      producer: registered_content_bound
      execution: trusted_host_observed
      attestation: kernel_accepted_local_process
```

The closed producer vocabulary now distinguishes:

- `operator_selected_content_bound`; and
- `registered_content_bound`.

`registered_content_bound` is requirement vocabulary only. This phase does not
add a producer registry or any mechanism that can establish that stronger
posture. The current closed validator route remains operator-selected,
content-bound, trusted-host-observed, and unattested.

## Enforcement Order

For the closed authoritative project-validation path, Core now:

1. resolves the canonical local-check declaration from the immutable bundle;
2. executes and evaluates the existing bounded validator path;
3. persists or resolves the first create-only evidence-assurance projection;
4. recovers the optional minimum from the matching immutable declaration; and
5. rejects the route before workflow events or skill execution when the durable
   projection does not satisfy every declared dimension.

The stable denial is:

```text
executor.local_check_evidence_assurance.minimum_not_satisfied
```

The error is payload-free. Failed enforcement leaves no workflow events and
does not invoke workflow skills. The bounded local check still runs because its
result and assurance projection are the evidence being classified by the gate.

## Compatibility

- The field is optional in YAML and the TypeScript SDK.
- Existing canonical declaration fingerprints remain unchanged when the field
  is absent.
- When present, all three dimensions participate in the canonical declaration
  set fingerprint and therefore immutable run identity.
- Existing routes preserve their behavior when no evidence minimum is declared.
- A runtime caller cannot lower or replace the immutable declaration by passing
  a different minimum to the executor.

## Validation

Focused validation passed for:

- optional declaration serde and accessor behavior;
- canonical declaration-set resolution and fingerprint validation;
- the existing no-minimum route;
- an exact current weak minimum;
- rejection of a registered-producer minimum;
- rejection of a kernel-accepted-attestation minimum;
- durable projection persistence before the fail-closed decision;
- zero workflow events and zero skill calls after denial;
- schema/SDK contract synchronization; and
- TypeScript typecheck, lint, and tests.

Complete Core and CLI test suites, warnings-denied Core and CLI Clippy,
formatting, contract synchronization, TypeScript checks, documentation checks,
and diff validation passed. Validation found one CLI exhaustiveness gap for the
new registered-producer label; the public projection and direct label coverage
were corrected before review acceptance.

## Strict Non-Goals

This phase does not add:

- a broad policy DSL;
- a producer registry or producer authentication;
- arbitrary command discovery or execution;
- new provider mutations;
- hosted behavior;
- signatures, external witnesses, or storage-owner tamper resistance;
- automatic assurance upgrades;
- WorkReport commitment expansion; or
- release publication or broader readiness claims.

## Governed Implementation

- Workflow: `dg/runtime-composition`
- Run: `run-1793897912937365000-2`
- Approval: `approval/run-1793897912937365000-2/composition-approved`
- Presentation: `presentation/7019434c7b6eca6b`
- Presentation hash:
  `7019434c7b6eca6b985b4761219336cfe4666d11ee174a2036fc23962ec6cb3c`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start

Repository edits, tests, and documentation were performed outside the kernel
under the approved scope. The kernel coordinated the phase and approval; it did
not execute commands or manufacture validation evidence.

## Review Outcome

Focused maintainer/security review accepted the slice after replacing implicit
enum declaration-order comparison with an explicit closed producer-assurance
table. The review confirms that this gate evaluates the provenance and
attestation claims durably projected by the local kernel; it does not establish
host independence, storage-owner tamper resistance, signatures, or external
witnessing. See [Local Check Evidence Assurance Minimum Gate
Review](LOCAL_CHECK_EVIDENCE_ASSURANCE_MINIMUM_GATE_REVIEW.md).

## Next Phase

Run independent adversarial assurance review before adding any producer
registry, assurance upgrade path, or wider authorization use.
