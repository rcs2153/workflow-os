# Local Check Evidence Assurance Durable Projection Review

## Decision

Accepted after one construction-provenance blocker was corrected.

The implementation durably preserves the existing weak assurance posture for
the closed trusted-host project-validation route. It does not upgrade that
posture or treat a deterministic projection fingerprint as proof of producer,
execution, or storage-owner independence.

## Blocker Corrected

The initial implementation exposed the create-only assurance sidecar writer as
a public store method. Although caller-created projections remain weak and
non-authorizing, that API would have allowed downstream code to populate the
same persistence namespace used by the Core-owned executor.

The writer is now crate-only. The executor remains the only production caller.
Validated read and list methods stay public so CLI and downstream inspection
can project bounded posture without gaining write authority.

## Reviewed Properties

The review confirmed:

1. Fresh execution persists the projection produced in the same Core-owned
   trusted-host composition call as the exact result.
2. Accepted reassessment resolves the first create-only record instead of
   replacing it with observation-time fingerprints from a later execution.
3. Durable subject comparison binds immutable run, workflow, run, step,
   command, selected handler, result, posture, accepted attestation, and
   external-verifiability fields.
4. Semantic result drift is rejected by the existing aggregate governance
   reassessment before sidecar reconciliation, preserving the established error
   and event ordering.
5. Report result references use the Core-derived result identity rather than a
   caller-authored label when carrying assurance.
6. Corrupt or misaddressed assurance blocks human and JSON inspection before
   posture is rendered.
7. Human and JSON output disclose bounded posture and a projection fingerprint
   without paths, process payloads, environment values, or raw output.
8. The real first-run report citation describes the current weak posture and
   does not call it independent evidence.

## Residual Risks

`LocalCheckEvidenceAssuranceProjection` remains public model vocabulary, and a
serialized projection remains a claim rather than proof of Core construction.
The durable namespace and closed route provide construction provenance for this
specific path; consumers must not infer equivalent provenance from an arbitrary
projection embedded elsewhere.

The local store remains inside the storage owner's trust domain. A privileged
owner able to rewrite all local records and anchors is outside this slice's
tamper-detection guarantee.

No policy currently requires a minimum assurance. The persisted weak posture
therefore improves truthfulness, replay stability, and inspection but does not
yet block a workflow that requires registered-producer or attested-execution
evidence.

## Validation Reviewed

- Complete Core all-feature tests passed, including 792 unit tests and 348
  local-executor tests.
- Complete CLI tests passed, including 180 CLI integration tests and all example
  suites.
- Focused replay, approval reassessment, semantic-drift, inspection, report
  citation, and corrupt-sidecar tests passed.
- Warnings-denied all-target/all-feature Core and CLI Clippy passed after the
  visibility-only blocker correction.
- Formatting, docs validation, and diff checks passed after the correction.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793896874279344000-2`
- Approval: `approval/run-1793896874279344000-2/review-scope-approved`
- Presentation: `presentation/b96dc22c42ef5745`
- Presentation hash:
  `b96dc22c42ef57456e3623d804db32e2a4d304006da76d00a26e02670684dbc9`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Review phase status: completed with 39 events, one approval, zero retries,
  and zero escalations; approval-presentation proof enforcement was present

## Next Phase

Proceed to one narrow minimum-assurance enforcement gate over the Core-owned
durable projection. Do not add a broad policy DSL, new producer registry,
arbitrary command execution, provider mutation, hosted behavior, or release
claims in that phase.
