# Workflow-Declared Proof-Marker Artifact Runtime Derivation Helper Report

## 1. Executive Summary

The pure workflow-declared approval proof-marker artifact runtime derivation helper is implemented.

The helper derives an effective approval proof-marker artifact gate policy from a selected workflow declaration and a caller-supplied policy. It preserves the invariant that callers may strengthen but cannot weaken workflow-declared requirements. It also fails closed when enforceable workflow declarations are derived outside an explicit artifact-capable posture.

## 2. Scope Completed

- Added `WorkflowReportArtifactProofMarkerDerivationMode`.
- Added `WorkflowReportArtifactProofMarkerGateDerivationInput`.
- Added `WorkflowReportArtifactProofMarkerGateDerivation`.
- Added `derive_workflow_report_artifact_approval_proof_marker_gate_policy`.
- Added strictness composition between workflow-declared and caller-supplied proof-marker policies.
- Added focused WorkReport tests for no-op, caller-stricter, workflow-stricter, non-artifact-capable rejection, and redaction-safe debug/error behavior.
- Exported the helper API from `workflow-core`.
- Updated roadmap and runtime derivation planning status.

## 3. Scope Explicitly Not Completed

This phase does not implement:

- executor artifact-path integration for this derivation helper;
- automatic report artifact writing;
- automatic approval proof-marker projection persistence;
- automatic report generation;
- default executor proof-marker enforcement;
- artifact-capable project validation;
- CLI rendering or artifact commands;
- examples;
- public approval cards;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 4. Helper API Summary

The new helper is:

```rust
derive_workflow_report_artifact_approval_proof_marker_gate_policy(
    WorkflowReportArtifactProofMarkerGateDerivationInput {
        workflow,
        caller_policy,
        derivation_mode,
    },
)
```

Inputs:

- selected `WorkflowDefinition`;
- optional caller-supplied `WorkReportArtifactApprovalProofMarkerGatePolicy`;
- derivation mode:
  - `DefaultValidation`;
  - `ArtifactCapable`.

Output:

- optional effective `WorkReportArtifactApprovalProofMarkerGatePolicy`.

`None` represents no proof-marker artifact gate. `allow_marker_free` requires projection coverage while allowing explicit marker-free approvals. `require_present_markers` requires projection coverage with present proof markers.

## 5. Composition Summary

Effective policy is the stricter of workflow declaration and caller policy:

- `not_required` plus no caller policy returns no proof-marker gate;
- `not_required` plus a stricter caller policy preserves caller policy;
- `projection_required` strengthens disabled caller policy to marker-free projection coverage;
- `marker_required` strengthens disabled or marker-free caller policy to present-marker coverage;
- caller policy may strengthen but cannot weaken workflow-declared requirements.

## 6. Artifact-Capable Boundary

`projection_required` and `marker_required` fail closed in `DefaultValidation` mode.

Stable error code:

```text
work_report_artifact.approval_proof_marker.derivation.runtime_not_artifact_capable
```

`ArtifactCapable` mode is required before enforceable workflow declarations can be composed into an effective artifact gate policy.

## 7. Redaction And Privacy Summary

The helper operates only on bounded enum posture and explicit gate policy.

It does not read, store, copy, debug-print, or serialize:

- approval presentation payloads;
- approval reasons;
- approval IDs;
- event IDs;
- projection IDs;
- presentation IDs;
- content hashes;
- report text;
- local paths;
- provider payloads;
- command output;
- source or spec contents;
- environment variable values;
- credentials, tokens, authorization headers, private keys, or secret-like metadata.

Errors use stable codes and do not include raw workflow paths or spec fragments.

## 8. Test Coverage Summary

Added focused tests proving:

- `not_required` preserves disabled proof-marker policy;
- caller can request stricter proof-marker policy;
- workflow `projection_required` strengthens weaker caller policy;
- workflow `marker_required` strengthens weaker caller policy;
- caller can strengthen a workflow projection requirement to present-marker coverage;
- non-artifact-capable derivation rejects enforceable workflow declarations with a stable non-leaking error;
- derivation debug output is bounded and does not include workflow identity or schema text;
- the helper does not mutate the selected workflow definition.

## 9. Commands Run And Results

- `cargo fmt --all` - passed.
- `cargo test -p workflow-core --test work_report workflow_proof_marker_derivation` - passed.
- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783676300145858000-2 --phase implementation` - passed.

## 10. Remaining Known Limitations

- Executor artifact-path integration for this derivation helper is not implemented.
- Artifact-capable project validation is not implemented.
- Automatic report artifact writing is not implemented.
- Automatic approval proof-marker projection persistence is not implemented.
- Default executor behavior remains unchanged.
- CLI artifact behavior remains unimplemented.

## 11. Recommended Next Phase

Recommended next phase: workflow-declared proof-marker artifact runtime derivation helper review.

This helper is a runtime-adjacent policy composition boundary. It should be reviewed before executor artifact-path derivation uses it.
