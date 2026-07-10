# Approval-Resume Artifact Projection Composition Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a narrow, explicit approval-resume artifact/projection
composition helper for the proof-enforced approval-presentation path. It
preserves existing approval APIs, keeps report/artifact behavior opt-in, and
composes already-reviewed primitives without changing default executor
semantics.

## 2. Scope Verification

The phase stayed within the approved explicit helper scope.

Implemented scope:

- `LocalApprovalResumeWithProjectedProofMarkerArtifactRequest`;
- `decide_approval_with_report_artifact_and_projected_proof_markers(...)`;
- proof-enforced approval resume through the existing presentation path;
- caller-supplied approval proof-marker projection store;
- caller-supplied report artifact store and side-effect record store;
- terminal WorkReport generation only after successful projection persistence;
- artifact gate evaluation before artifact write;
- focused tests and documentation.

No accidental implementation was found for:

- default approval behavior changes;
- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI approval-resume artifact behavior;
- workflow schema changes;
- examples;
- runtime config;
- provider calls;
- provider writes;
- side-effect execution;
- hosted or distributed runtime behavior;
- reasoning lineage;
- recursive agents or agent swarms;
- Level 3/4 autonomy changes;
- release posture changes.

## 3. API Assessment

The helper API is appropriately explicit and executor-adjacent. It requires the
caller to supply:

- the executor;
- artifact store;
- side-effect record store;
- proof-marker projection inputs;
- approval-presentation decision request;
- report inputs;
- optional side-effect discovery inputs;
- artifact gate inputs.

The result reuses `LocalExecutionWithProjectedProofMarkerArtifactResult`, which
keeps the implementation small and consistent with the existing execution-time
artifact composition path. That reuse is acceptable because the helper name and
request type clearly identify the approval-resume origin.

## 4. Runtime Semantics Assessment

The composition order is appropriate:

1. approval decision is applied through
   `LocalExecutor::decide_approval_with_presentation(...)`;
2. workflow report artifact policies are derived from immutable workflow/run
   identity;
3. non-terminal resumed runs return report status posture without projection or
   artifact writes;
4. terminal resumed runs persist proof-marker projections first;
5. projection failure returns the completed run plus projection error and writes
   no report artifact;
6. successful projection persistence allows report generation and artifact gate
   evaluation;
7. artifact write happens only through the existing artifact gate helper.

This preserves workflow pass/fail semantics. Post-processing failures do not
retroactively change the resumed run status.

## 5. Projection And Artifact Gate Assessment

Projection persistence remains caller-supplied-store bounded. The helper does
not discover hidden stores or infer projection paths from workflow specs.

The artifact path uses the existing store-backed proof-marker gate and derives
workflow-declared proof-marker requirements before writing. This means authored
requirements can strengthen caller policy in the explicit artifact-capable path
without changing default validation or approval behavior.

The implementation correctly avoids artifact writes when projection persistence
fails.

## 6. Privacy And Redaction Assessment

The implementation does not copy or persist approval handoff text, approval
presentation payloads, approval reasons, command output, provider payloads, raw
source/spec contents, environment values, credentials, tokens, authorization
headers, private keys, or secret-like values.

Debug implementations redact stores, projection/redaction metadata, and
payload-bearing report/artifact fields through existing result/request
redaction behavior. Focused tests assert that projection/result debug output
does not expose approval or presentation identifiers.

## 7. Test Quality Assessment

The added tests cover the highest-risk behavior:

- proof-enforced approval resume completes and writes a gated artifact;
- proof-marker projection persistence is exposed;
- a durable projection record is persisted;
- artifact store receives one artifact only after gates pass;
- projection failure is returned as result posture;
- projection failure writes no artifact and generates no report;
- workflow events are unchanged by projection/artifact post-processing;
- debug output does not expose approval or presentation identifiers.

Non-blocking follow-up: add a focused test for a non-terminal resumed run if a
future approval-resume scenario can naturally remain non-terminal. The current
coverage is enough for the first terminal proof-enforced path.

## 8. Documentation Review

The phase report and plan accurately state that the helper is implemented and
that default approval behavior remains unchanged.

Docs continue to say the following remain unimplemented:

- automatic report generation;
- automatic projection persistence;
- automatic artifact writing;
- CLI approval-resume artifact behavior;
- workflow schema changes;
- examples;
- provider calls/writes;
- side-effect execution;
- hosted/distributed runtime behavior;
- reasoning lineage;
- release posture changes.

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Add non-terminal approval-resume coverage if a natural workflow shape emerges.
- Plan high-assurance approval-resume artifact/projection composition separately
  instead of folding it into this proof-enforced presentation path.
- Keep CLI and default executor behavior out of scope until the explicit helper
  boundary has more usage evidence.

## 11. Recommended Next Phase

Recommended next phase: high-assurance approval-resume artifact/projection
planning.

Why: the proof-enforced presentation path is now accepted. High-assurance
approval controls are the adjacent sensitive-action boundary, but they should
be planned before implementation so the project does not accidentally weaken
the nuclear-key posture or broaden default approval behavior.

## 12. Validation

- `cargo fmt --all --check` - passed.
- `cargo clippy --workspace --all-targets -- -D warnings` - passed.
- `cargo test -p workflow-core --test local_executor decide_approval_with_report_artifact_projected_proof_markers -- --nocapture` - passed.
- `cargo test --workspace` - passed.
- `npm run check:docs` - passed.
- `git diff --check` - passed.
- `npm run dogfood:benchmark -- phase-close run-1783693658301111000-2 --phase review` - passed.

## 13. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1783693658301111000-2`.
- Approval ID: `approval/run-1783693658301111000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after proof-enforced handoff.
- Approval presentation ID: `presentation/fc5d6e93a3b75745`.
- Approval presentation hash: `fc5d6e93a3b75745ec9eae90c5f3761d0e5ee1f678bb7208bd59fa8f9e858667`.
- Event summary: 39 events; `ApprovalGranted:1`,
  `ApprovalRequested:1`, `PolicyDecisionRecorded:8`, `RunCompleted:1`,
  `RunCreated:1`, `RunResumed:1`, `RunStarted:1`, `RunValidated:1`,
  `SkillInvocationRequested:6`, `SkillInvocationStarted:6`,
  `SkillInvocationSucceeded:6`, `StepScheduled:6`.
- Out-of-kernel work: repository inspection, documentation edits, validation
  commands, git operations, and GitHub actions are performed by the
  maintainer/Codex execution layer outside the Workflow OS runtime.
