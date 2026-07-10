# High-Assurance Approval-Resume Artifact Projection Hardening Report

## 1. Executive Summary

This phase hardens the explicit high-assurance approval-resume artifact/projection helper that was accepted in the previous review. The helper remains opt-in, local, and executor-adjacent.

The hardening does two things:

- derives effective report-artifact policy before approval mutation where immutable run identity is already available;
- adds exact-helper regression coverage for denied approvals, projection persistence failure, same-actor rejection, and high-assurance disclosure posture conflict.

No default approval behavior, automatic report generation, automatic artifact writing, CLI behavior, schemas, examples, provider writes, side-effect execution, hosted behavior, reasoning lineage, RBAC/IdP/quorum/revocation, or release posture changed.

## 2. Scope Completed

Completed scope:

- `decide_approval_with_high_assurance_report_artifact_and_projected_proof_markers(...)` now derives workflow-declared/caller-composed artifact policies before `apply_approval_decision(...)`.
- Policy derivation now fails before approval mutation when that derivation fails.
- Exact-helper denial behavior is covered: denied high-assurance approval fails the run, persists the proof-marker projection, carries denied disclosure, and writes the local report artifact when gates pass.
- Exact-helper projection failure is covered: approval succeeds, projection failure is returned as projection posture, and no report artifact is written.
- Exact-helper same-actor rejection is covered: the run remains waiting for approval, no decision events are appended, and no projection/artifact is written.
- Exact-helper disclosure posture conflict is covered: conflicting high-assurance disclosure posture fails closed before approval mutation, projection, or artifact writing.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- default approval behavior changes;
- automatic high-assurance approval enforcement;
- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI behavior;
- workflow schema changes;
- examples;
- provider calls or provider writes;
- side-effect execution;
- approval evidence attachment;
- RBAC, IdP, SSO, SCIM, teams, groups, quorum, or external directory integration;
- role-bound approval authority;
- revocation enforcement;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy changes;
- release posture changes.

## 4. Implementation Summary

The helper previously derived effective artifact policy after applying the approval decision. This was acceptable for the first composition slice but left a non-blocking hardening opportunity: policy derivation errors could occur after approval mutation.

The helper now builds the local policy request from the prepared paused run identity and derives `WorkflowReportArtifactPolicies` before approval mutation. It then applies the approval decision only after:

- approval-presentation proof is resolved and validated;
- high-assurance approval controls and references validate;
- report-safe high-assurance disclosure is constructed;
- approval proof marker is attached to the decision;
- effective artifact policy derives successfully.

Projection persistence still occurs after approval mutation because projection is derived from the resumed terminal run event trail. Projection failure remains a post-approval result posture and writes no report artifact.

## 5. Validation Boundary Summary

Fail-closed pre-mutation boundaries now include:

- waiting-run/approval validation;
- durable approval-presentation proof validation;
- high-assurance approval validation;
- high-assurance disclosure construction;
- effective artifact policy derivation.

Post-mutation result boundaries remain:

- non-terminal run status returns no report artifact;
- projection persistence failure returns projection error and writes no artifact;
- report generation or artifact construction failure returns the existing structured result posture;
- artifact gates still decide whether the local artifact is written.

## 6. Redaction And Privacy Summary

The hardening preserves the existing privacy posture:

- no raw provider payloads;
- no raw command output;
- no raw CI logs;
- no raw source or spec contents;
- no environment values;
- no credentials, authorization headers, private keys, or token-like values;
- no approval-presentation payload copying into reports;
- no high-assurance control payload copying into artifacts.

Regression tests assert non-leakage for relevant error/debug paths where the exact helper fails closed.

## 7. Test Coverage Summary

Added or strengthened focused coverage for:

- high-assurance approval-resume success path with projected proof markers and artifact disclosure;
- high-assurance approval-resume denial artifact behavior;
- high-assurance approval-resume projection failure writing no artifact;
- high-assurance approval-resume same-actor rejection appending no decision events;
- high-assurance approval-resume disclosure posture conflict appending no decision events;
- missing high-assurance reference appending no decision events, projection records, or artifacts.

Existing adjacent coverage continues to cover:

- proof-enforced approval-presentation paths;
- high-assurance approval validation helper behavior;
- high-assurance disclosure report behavior;
- proof-marker projection persistence;
- report artifact proof-marker gates;
- workflow-declared artifact requirement derivation.

## 8. Commands Run And Results

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test local_executor high_assurance_approval_resume -- --nocapture` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783698261719166000-2 --phase implementation` - passed.

## 9. Remaining Known Limitations

- The helper remains an explicit local API, not automatic runtime behavior.
- Projection persistence is necessarily post-approval because it is based on the resumed event trail.
- The high-assurance disclosure artifact gate is structurally satisfied by the helper on successful validated paths because the helper attaches validated disclosure before artifact writing.
- RBAC/IdP/quorum/revocation semantics remain future work.
- Write-capable adapters remain unsupported.

## 10. Governed Implementation Record

- Dogfood workflow: `dg/implement`.
- Run ID: `run-1783698261719166000-2`.
- Approval ID: `approval/run-1783698261719166000-2/implementation-approved`.
- Approval outcome: granted by delegated maintainer after proof-enforced handoff.
- Approval presentation ID: `presentation/f4e10a0e8f1e7774`.
- Approval presentation hash: `f4e10a0e8f1e77746c5b6cb9083a4f4e882062e139b53f0477295b2dffba36db`.

## 11. Recommended Next Phase

Recommended next phase: high-assurance approval-resume artifact/projection hardening review.

The phase changed safety-sensitive approval/artifact composition ordering and added regression tests. A focused maintainer review should confirm that the pre-mutation boundary is now correct and that the exact-helper tests cover the previously accepted follow-ups without broadening scope.
