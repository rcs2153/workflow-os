# High-Assurance Approval-Resume Artifact Projection Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation provides the intended explicit local composition helper for
high-assurance approval resume, durable presentation proof, proof-marker
projection persistence, terminal WorkReport generation, high-assurance
disclosure gates, and local report artifact writing. It stays opt-in and does
not alter default approval, execution, CLI, schema, provider-write, side-effect,
hosted, reasoning-lineage, or release behavior.

## 2. Scope Verification

The phase stayed within the approved runtime-composition scope.

Confirmed absent:

- default approval behavior changes;
- automatic high-assurance approval enforcement;
- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI behavior;
- workflow schema changes;
- examples;
- runtime configuration;
- provider calls or provider writes;
- side-effect execution;
- approval evidence attachment;
- RBAC, IdP, SSO, SCIM, teams, groups, quorum, or external directory integration;
- role-bound approval authority or revocation enforcement;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents, agent swarms, or Level 3/4 autonomy changes;
- release posture changes.

## 3. Helper/API Assessment

The new API is appropriately explicit:

- `LocalHighAssuranceApprovalResumeWithProjectedProofMarkerArtifactRequest`;
- `decide_approval_with_high_assurance_report_artifact_and_projected_proof_markers(...)`.

The request requires caller-supplied projection inputs, a high-assurance approval
decision request, durable approval-presentation proof, report inputs,
side-effect discovery inputs if any, and artifact inputs. It does not discover
hidden stores or infer runtime configuration.

The helper reuses the existing
`LocalExecutionWithProjectedProofMarkerArtifactResult`, which is acceptable for
this slice because it already separates run, report, artifact, projection
success, and projection error posture.

## 4. Composition Assessment

The composition order is correct for this phase:

1. rehydrate and validate the waiting approval run;
2. resolve durable approval-presentation proof;
3. validate approval-presentation proof;
4. validate high-assurance controls and supplied references;
5. construct report-safe high-assurance disclosure;
6. attach the approval proof marker to the approval decision;
7. apply the approval decision;
8. derive effective artifact policies;
9. persist approval proof-marker projections;
10. generate a terminal WorkReport carrying high-assurance disclosure;
11. evaluate existing artifact gates;
12. write the local report artifact only when requested gates pass.

The important correction versus a high-assurance-only approach is present:
proof-marker projection requires durable approval-presentation proof. The helper
therefore requires both high-assurance validation and presentation proof rather
than pretending high-assurance validation alone can satisfy projection gates.

## 5. Validation Semantics Assessment

Validation is deterministic and fail-closed:

- presentation proof failures occur before approval mutation;
- high-assurance validation failures occur before approval mutation;
- missing required high-assurance references append no approval decision events;
- successful grant resumes once and reaches the existing terminal path;
- projection failure after approval resume is returned as projection posture and writes no artifact;
- generated reports use existing WorkReport constructors;
- artifact writes use existing artifact gates;
- default approval paths remain unchanged.

No validation path copies raw control payloads, presentation payloads, provider
payloads, command output, source contents, spec contents, tokens, credentials,
or secret-like values.

## 6. Artifact And Projection Assessment

The helper uses the existing projection persistence helper and report artifact
finish path. That keeps proof-marker projection semantics aligned with the
previously reviewed proof-enforced approval artifact path.

The happy-path test requires:

- selected approval reference projection;
- persisted proof-marker projection record;
- high-assurance disclosure in the report;
- high-assurance disclosure artifact gate;
- artifact write success.

This is the right minimum for the grant-path implementation.

## 7. Workflow Semantics Assessment

The helper does not mutate workflow state outside the approval decision it is
explicitly asked to make. It does not append projection, report, artifact, or
side-effect workflow events. It does not call providers, touch hidden stores,
write CLI output, or change workflow pass/fail semantics.

One non-blocking hardening opportunity remains: effective artifact policy
derivation currently happens after the approval decision is applied, matching
the existing projection/artifact helper shape. A future pass should consider
deriving the policy before approval mutation where feasible, so policy
derivation errors can fail before approval resume and return a cleaner
pre-mutation error boundary.

## 8. Privacy And Redaction Assessment

Privacy posture is acceptable:

- request Debug output redacts proof mode, report payloads, stores, and approval inputs;
- result Debug output does not expose approval IDs, presentation IDs, evidence reference IDs, or project-specific names in the focused tests;
- high-assurance disclosure stores bounded posture rather than raw control payloads;
- projection records do not store presentation payloads;
- WorkReport generation uses existing redaction-safe constructors;
- artifact gates reuse existing non-leaking errors.

No raw provider payloads, command output, CI logs, Jira/GitHub bodies, source
contents, spec contents, environment values, credentials, authorization headers,
private keys, token-like values, approval reasons, or secret-like values are
stored by this helper.

## 9. Test Quality Assessment

The added tests are focused and useful:

- successful high-assurance approval grant writes a proof-marker-projected report artifact;
- missing high-assurance reference fails before approval mutation;
- failed validation writes no projection records or artifacts;
- debug output does not leak approval, presentation, or evidence identifiers.

Existing suites cover adjacent behavior:

- presentation-proof validation;
- high-assurance same-actor rejection;
- high-assurance denial disclosure;
- projection persistence failure in the existing proof-enforced approval path;
- workflow-declared high-assurance and proof-marker artifact gate derivation;
- WorkReport and artifact redaction behavior.

Missing or deferred coverage is acceptable for this phase but should be tracked:

- denial-result artifact behavior through this exact helper;
- projection persistence failure through this exact helper;
- same-actor rejection through this exact helper;
- high-assurance disclosure gate failure through this exact helper.

## 10. Documentation Review

Documentation is honest:

- the plan now marks the explicit local grant path implemented;
- the report states the helper requires durable approval-presentation proof plus high-assurance validation;
- the report names completed and explicitly incomplete scope;
- the roadmap links the implementation report;
- automatic enforcement, automatic report generation, automatic artifact writing, CLI, schemas, examples, provider writes, side-effect execution, hosted behavior, reasoning lineage, RBAC/IdP/quorum/revocation, and release posture remain explicitly unimplemented.

## 11. Blockers

No blockers.

## 12. Non-Blocking Follow-Ups

- Consider moving effective artifact policy derivation before approval mutation where feasible.
- Add denial-result artifact behavior tests before claiming denial artifact support.
- Add projection persistence failure coverage through this exact helper.
- Add same-actor rejection coverage through this exact helper.
- Add explicit high-assurance disclosure gate failure coverage through this exact helper.

## 13. Recommended Next Phase

Recommended next phase: high-assurance approval-resume artifact/projection hardening follow-up, focused on pre-mutation policy derivation and the missing exact-helper regression tests.

This should be a narrow blocker-prevention hardening phase, not a new primitive
family. CLI behavior, schemas, examples, provider writes, side-effect execution,
hosted behavior, reasoning lineage, RBAC/IdP/quorum/revocation, and release
posture should remain out of scope.

## 14. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783697620746773000-2`.
- Approval ID: `approval/run-1783697620746773000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after proof-enforced handoff.
- Approval presentation ID: `presentation/eaeebcbf75e6844a`.
- Approval presentation hash: `eaeebcbf75e6844a2aca7701216a2f796786e8eb904c28faaf77e0ddb3d4453a`.

## 15. Validation

- `cargo fmt --all --check` - passed.
- `cargo test -p workflow-core --test local_executor high_assurance_approval_resume -- --nocapture` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783697620746773000-2 --phase review` - passed.
