# High-Assurance Approval-Resume Artifact Projection Hardening Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The hardening correctly narrows a previously accepted follow-up: effective artifact policy is now derived before approval mutation, and the exact high-assurance approval-resume artifact/projection helper has direct regression coverage for denial, projection failure, same-actor rejection, and disclosure posture conflict. The phase stays within the explicit local helper boundary and does not change default approval behavior.

## 2. Scope Verification

The phase stayed within the approved hardening scope.

Confirmed absent:

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

## 3. Implementation Assessment

The implementation moves `workflow_report_artifact_policy_for_request_with_proof_marker_policy(...)` ahead of `apply_approval_decision(...)` in `decide_approval_with_high_assurance_report_artifact_and_projected_proof_markers(...)`.

That is the right boundary. The helper already has immutable paused-run identity available after `prepare_approval_decision(...)`, so deriving workflow-declared and caller-composed artifact policies before approval mutation is feasible and safer. Policy derivation can now fail without appending approval decision events.

The post-mutation projection boundary remains appropriate. Projection persistence depends on the resumed run event trail and should continue to return structured projection posture after approval mutation rather than pretending projection can be derived before the decision exists.

## 4. Exact-Helper Regression Coverage

The added tests cover the previously accepted follow-ups against the exact helper:

- `high_assurance_approval_resume_denial_writes_failed_artifact_with_disclosure`;
- `high_assurance_approval_resume_projection_failure_writes_no_artifact`;
- `high_assurance_approval_resume_same_actor_rejection_appends_no_events`;
- `high_assurance_approval_resume_disclosure_conflict_appends_no_events`.

The existing success and missing-reference tests remain in place.

The disclosure-conflict test is the correct substitute for a missing-disclosure artifact-gate test on this helper. Successful high-assurance helper paths structurally attach validated disclosure before artifact writing; the realistic fail-closed disclosure path is therefore disclosure construction conflict before approval mutation.

## 5. Validation Semantics Assessment

Validation semantics are improved:

- presentation proof validation remains pre-mutation;
- high-assurance control/reference validation remains pre-mutation;
- high-assurance disclosure construction remains pre-mutation;
- effective artifact policy derivation is now pre-mutation;
- projection failure remains post-mutation result posture and writes no artifact;
- denial behavior is explicitly covered through artifact/report disclosure;
- same-actor rejection appends no decision events, projections, or artifacts.

No workflow pass/fail behavior changes outside this explicit helper.

## 6. Privacy And Redaction Assessment

Privacy posture remains sound:

- errors do not include approval IDs, presentation IDs, project paths, or secret-like values in the tested failure paths;
- reports carry bounded high-assurance disclosure rather than raw control payloads;
- projection records do not copy approval-presentation payloads;
- no raw provider payloads, command output, CI logs, source contents, spec contents, environment values, credentials, authorization headers, private keys, or token-like values are stored.

## 7. Documentation Review

Documentation is accurate:

- the plan status links to the hardening report;
- the previous review preserves its original finding and adds a fix-forward note;
- the roadmap states the hardening follow-up is implemented;
- the hardening report names completed scope, explicit non-scope, validation, remaining limitations, and the governed implementation record.

Docs do not overclaim default enforcement, automatic report generation, automatic artifact writing, write-capable adapters, hosted behavior, RBAC/IdP/quorum/revocation, or release posture.

## 8. Test Quality Assessment

The tests are focused and meaningful. They verify exact-helper behavior instead of relying only on adjacent lower-level or non-high-assurance helper coverage.

Remaining non-blocking test opportunities:

- add a dedicated policy-derivation-failure pre-mutation regression if a small fixture can trigger that path without synthetic corruption;
- add an assertion that the denial artifact includes the expected high-assurance approval section summary, not only the denied disclosure decision.

These are not blockers because the current ordering change is direct and the denial/disclosure tests exercise the main safety boundaries.

## 9. Validation

Local validation run during implementation:

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test local_executor high_assurance_approval_resume -- --nocapture` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783698261719166000-2 --phase implementation` - passed.

Review validation:

- GitHub PR #250 mergeability - mergeable, no conflicts.
- GitHub PR #250 checks - all required checks passed.
- `npm run dogfood:benchmark -- phase-close run-1783699498020514000-2 --phase review` - passed.

## 10. Blockers

No blockers.

## 11. Non-Blocking Follow-Ups

- Add a policy-derivation-failure pre-mutation regression if a clean fixture is available.
- Add more detailed denied-artifact report-section assertions.

## 12. Recommended Next Phase

Recommended next phase: merge PR #250 after CI completes, then proceed to the next roadmap runtime-composition item.

This hardening closes the immediate review follow-ups for the high-assurance approval-resume artifact/projection helper. The next implementation work should continue reducing the gap between existing primitives and explicit runtime composition, while keeping write-capable adapters deferred until the approval/artifact/report gates are review-clean.

## 13. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783699498020514000-2`.
- Approval ID: `approval/run-1783699498020514000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after proof-enforced handoff.
- Approval presentation ID: `presentation/b87213f195cb9166`.
- Approval presentation hash: `b87213f195cb91669a0b0c758a5caa14755900e12e309e81c3276f28cba1d8ca`.
- Event summary: 39 events, 1 approval, 0 retries, 0 escalations.
