# Local Check Evidence Assurance Projection Review

## Decision

Accepted for the model-only boundary with one explicit runtime-composition
guardrail: a serialized weak projection is representational data, not proof
that Core observed the execution it describes.

The implementation may proceed to a separately governed trusted-host
composition phase. That phase must establish the projection from Core-owned
resolved execution and result context before any minimum-assurance decision is
used as a gate. A caller-created or deserialized projection must never become
execution authority merely because its deterministic fingerprints validate.

## Reviewed Boundary

The review covered:

- the closed producer, execution, attestation, and external-verifiability
  vocabulary;
- exact binding to immutable run, execution, handler-selection,
  resolved-execution, result identity, and result status commitments;
- construction from the existing crate-private accepted-attestation proof;
- validated serialization and deserialization behavior;
- deterministic domain-separated framing;
- dimension-specific minimum comparison;
- safe `Debug` and stable non-leaking errors; and
- the absence of runtime, persistence, event, CLI, schema, SDK, report,
  provider, signing, witness, hosted, or publication behavior.

## Accepted Properties

The stronger `kernel_accepted_local_process` posture is not caller-selectable.
It requires the existing verifier-owned `AcceptedLocalCheckAttestation` object,
and construction checks exact execution binding, immutable run, workflow, run,
step, handler selection, result identity, and result status agreement.

The accepted posture can be serialized for bounded disclosure, but it cannot
be deserialized back into verifier-owned assurance. A stored proof fingerprint
is therefore a reference, not a substitute proof.

The current weak posture remains honest and dimensionally separate:

- producer: `operator_selected_content_bound`;
- execution: `trusted_host_observed`;
- attestation: `unattested`; and
- external verifiability: `unavailable`.

Pass status, zero exit, executable name, paths, or recomputable public
fingerprints do not upgrade those dimensions. The projection stores no raw
paths, command output, environment values, summaries, credentials, provider
payloads, or report prose.

## Required Runtime Guardrail

Validated weak projections intentionally round-trip. Their fingerprint proves
only deterministic internal consistency. It does not prove that Core created
the projection or that the trusted host actually observed the represented
process.

Therefore the later runtime phase must:

1. construct the projection inside the Core-owned trusted-host result path;
2. bind it to the exact immutable execution, resolved-execution, result, and
   run context already held by that path;
3. persist or expose it only with source provenance that cannot be replaced by
   a caller-supplied serialized projection;
4. reject any minimum-assurance gate that lacks that Core-owned source binding;
5. preserve the accepted-attestation verifier boundary; and
6. keep minimum comparison as classification, not standalone authority.

This guardrail is now explicit in the public constructor and minimum-comparison
documentation. Runtime composition is not authorized to infer provenance from
the projection fingerprint alone.

## Validation

Passed:

- focused projection tests;
- accepted-attestation verifier integration test;
- `cargo fmt --all --check`;
- `cargo clippy -p workflow-core --all-targets --all-features -- -D warnings`;
- `cargo test -p workflow-core --all-features`;
- `npm run check:docs`; and
- `git diff --check`.

Opt-in live credential and external-tool smoke tests remained ignored under
their declared contracts. No live provider, write, publication, or arbitrary
command behavior was exercised.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793853776405636000-2`
- Approval: `approval/run-1793853776405636000-2/review-scope-approved`
- Presentation: `presentation/14d941a53f02dd5d`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Repository review, validation commands, and this record were produced outside
  the kernel under the approved review scope

## Next Phase

Compose the weak projection into the closed trusted-host project-validation
result path only. Do not add a generic evidence producer, arbitrary command
execution, report-body commitment, new provider mutations, or broader release
claims in that phase.
