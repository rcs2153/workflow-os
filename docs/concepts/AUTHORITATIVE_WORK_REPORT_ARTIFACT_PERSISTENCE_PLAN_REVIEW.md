# Authoritative WorkReport Artifact Persistence Plan Review

## 1. Executive Verdict

**Plan accepted; proceed to authoritative WorkReport artifact persistence
implementation.**

The plan selects the correct next runtime-composition boundary. It connects the
already accepted authoritative report result to the already accepted local
artifact store and governance gates without introducing a new primitive family,
provider, sandbox, schema, or opt-in control.

The implementation must complete deterministic durable-run-derived report
identity before the first artifact write. That is an implementation
prerequisite, not a planning blocker.

## 2. Scope Verification

The plan stays within an additive local authoritative path.

It does not authorize:

- persistence for ordinary undeclared executor paths;
- automatic artifacts for every run;
- provider execution or mutation expansion;
- OpenShell integration;
- new SideEffect or approval semantics;
- hosted or remote storage;
- report export, signing, or publication;
- schema expansion;
- post-terminal workflow events;
- snapshot mutation;
- examples; or
- release posture changes.

Treating artifact persistence as part of the already explicit authoritative
contract is preferable to another command flag. Projects without accepted
authoritative activation remain unchanged.

## 3. Current-Foundation Assessment

The plan accurately identifies the implemented foundations:

- the authoritative dispatcher-plus-report result produces a validated
  terminal `WorkReport`;
- `WorkReportArtifactRecord` validates and binds report metadata;
- `LocalStateBackend` implements artifact and SideEffect stores;
- governed artifact helpers compose SideEffect integrity, approval linkage,
  high-assurance disclosure, and proof-marker gates;
- workflow definitions already declare high-assurance and proof-marker
  artifact requirements; and
- approval proof-marker projection helpers already exist.

The planned implementation should reuse these surfaces. Direct CLI writes to
the artifact store would bypass reviewed gates and must be rejected in review.

## 4. Activation Assessment

The proposed activation boundary is appropriately narrow:

- explicit authoritative CLI invocation; or
- accepted project-controlled authoritative execution.

No extra artifact flag is needed. Such a flag would make evidence retention an
operator memory problem and conflict with quiet-success goals.

This does not make persistence global. Ordinary execution, onboarding posture,
mock demos, and undeclared projects retain current behavior.

## 5. Deterministic Identity And Retry Assessment

The plan correctly identifies a real prerequisite. Current authoritative report
inputs use a stable report ID but fresh timestamp and correlation values.
Persisting that shape directly would create byte-different artifacts on exact
retry.

The implementation must:

- derive generation time and correlation from durable run events;
- document one deterministic fallback rule;
- preserve stable generated-by posture;
- accept an exactly equal existing artifact as idempotent success;
- reject different content at the same run/report identity;
- re-read after a concurrent duplicate rejection; and
- never overwrite or repair a conflicting artifact.

Tests must exercise fresh completion, completed-run retry, approval-resume
retry, and concurrent duplicate handling.

## 6. Gate Composition Assessment

The gate order is correct:

1. terminal authoritative route;
2. same-call report generation;
3. artifact validation;
4. immutable workflow policy derivation;
5. proof-marker projection;
6. SideEffect integrity and approval linkage;
7. high-assurance disclosure;
8. proof-marker enforcement; and
9. create-only artifact write.

Quiet execution must not weaken artifact gates. If a workflow requires approval
proof that does not exist, the artifact obligation fails visibly even when the
workflow run itself is otherwise terminal.

## 7. Workflow Semantics Assessment

The plan preserves the correct source-of-truth boundary:

- workflow events and snapshot retain the terminal execution result;
- report artifacts remain separate governed handoff records;
- no post-terminal event is appended;
- report or artifact failure does not rewrite run status;
- the authoritative operation still returns non-success when its required
  handoff fails; and
- exact retry may complete the artifact obligation later.

This distinction is important. A completed workflow with a failed report
artifact is not a failed workflow, but it is not a successful authoritative
handoff either.

## 8. Terminal And Approval Assessment

The plan correctly defers report/artifact creation for approval-pending runs and
composes persistence only after terminal report generation.

It also correctly includes terminal denial reports when the authoritative
consumer generated one. Failed and canceled outcomes should be persisted only
where the existing report consumer produces a valid terminal report; the
implementation must not manufacture new status support.

## 9. CLI And Inspect Assessment

The proposed quiet output remains concise while improving truth:

- completed run;
- quiet governance;
- persisted report identity; and
- inspect command.

Verbose and JSON modes should expose bounded artifact posture and stable error
codes without report body text.

Adding bounded artifact metadata to existing `inspect` is necessary for the
phase to be useful. Persisting a report that the operator cannot discover would
leave the product loop incomplete. Metadata-only inspection preserves privacy
and keeps full report rendering/export deferred.

## 10. Privacy Assessment

The plan maintains the repository's privacy posture:

- validated report content only;
- no command output;
- no provider payloads;
- no raw source/spec/parser contents;
- no credentials or environment values;
- no secret-like approval metadata;
- no state-root path disclosure; and
- no report body in default output or metadata inspection.

Artifact stores remain sensitive local preview stores. The plan does not
overclaim encryption, retention, access control, or regulated-data readiness.

## 11. Test Quality Assessment

The planned matrix covers:

- terminal route variants;
- approval deferral and resume;
- deterministic retry;
- concurrency;
- workflow-authored artifact gates;
- SideEffect and approval linkage;
- failure separation;
- quiet/verbose/JSON behavior;
- inspect metadata and corruption;
- ordinary-path compatibility;
- privacy;
- no provider calls;
- no post-terminal mutation; and
- Node integration regressions.

One implementation detail should be added if needed during coding: prove that
the terminal event selected for deterministic report identity is identical
after state rehydration and does not depend on event-vector iteration outside
validated sequence order.

## 12. Documentation Assessment

The plan and roadmap accurately say:

- in-memory authoritative reports are implemented;
- durable authoritative artifact composition is planned, not implemented;
- ordinary execution is unchanged;
- provider expansion and OpenShell are not part of this phase;
- hosted storage and export remain deferred; and
- the next work is runtime composition, not another model-only family.

## 13. Blockers

None.

## 14. Non-Blocking Follow-Ups

- Plan full report-content read/export only after metadata inspection is
  reviewed.
- Define retention and access-control posture before shared or hosted stores.
- Consider artifact citation from future reasoning lineage only after that
  model exists.
- Evaluate an OpenShell execution provider only after Workflow OS's internal
  authority, evidence, and artifact boundaries are complete.

## 15. Recommended Next Phase

Proceed with **authoritative WorkReport artifact persistence, local and
project-controlled only**.

The implementation should begin with durable-run-derived deterministic report
identity and then compose existing artifact gates into fresh-run and
proof-enforced approval-resume authoritative results. It must finish with
bounded CLI/JSON posture and metadata-only `inspect`.

## 16. Validation

Review validation:

- `npm run check:docs`: passed;
- `git diff --check`: passed.

Governed review:

- workflow ID: `dg/review`;
- run ID: `run-1785190504794311000-2`;
- approval ID:
  `approval/run-1785190504794311000-2/review-scope-approved`;
- approval presentation:
  `presentation/6f5488f9d23d7523`;
- approval outcome: granted through proof-enforced presentation;
- runtime execution, artifact writing, provider calls, and git actions: not
  performed by the kernel.
