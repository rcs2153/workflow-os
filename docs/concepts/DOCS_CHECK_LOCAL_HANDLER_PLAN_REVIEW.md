# DocsCheck Local Handler Plan Review

Review date: 2026-06-15

## 1. Executive Verdict

Plan accepted; proceed to explicit `DocsCheck` local handler implementation.

The plan is ready to generate a narrow implementation prompt for a non-default, explicitly constructed `DocsCheck` handler. The implementation must remain local, in-memory/test-scoped, non-shell, non-CLI, non-schema, non-artifact, non-evidence, and non-writing.

The first implementation should rely on injected-runner tests. Any real npm execution test must be separately bounded, deterministic, and must not depend on ambient credentials or network access.

## 2. Scope Verification

The plan stayed within planning-only scope.

No accidental authorization was found for:

- `DocsCheck` handler implementation in the planning phase;
- production local check handler registration;
- default handler registration;
- CLI behavior;
- workflow schema fields;
- automatic check execution;
- report artifact writing;
- persistence;
- evidence attachment;
- command-output evidence;
- side-effect boundary implementation;
- source writes;
- broader cargo/npm command families;
- live provider access;
- release posture changes.

## 3. Governance Check

This review was governed by the self-governance dogfood workflow.

- State directory: `/tmp/workflow-os-docs-check-handler-plan-review`
- Run ID: `run-1781503249922787000-2`
- Workflow ID: `dg/d`
- Approval ID: `approval/run-1781503249922787000-2/d`
- Final status: `Completed`

Inspection confirmed the expected event history through `RunCompleted`.

## 4. Candidate Assessment

`DocsCheck` is the right first non-dogfood local check candidate.

Reasons:

- it is project-owned;
- it maps to a fixed canonical command template: `npm run check:docs`;
- it does not require provider credentials;
- it validates repository documentation rather than mutating source;
- it can exercise the local check handler boundary without introducing cargo build/cache complexity.

The plan correctly identifies that Node/npm still requires explicit environment and cache posture before implementation.

## 5. Command Authority Assessment

The command authority rules are sufficient for implementation.

The future handler must:

- accept only `LocalCheckCommandKind::DocsCheck`;
- execute only the canonical executable/argument template;
- use executable plus argument vector;
- never invoke a shell;
- reject extra caller arguments;
- use explicit working directory and sanitized environment;
- return stable non-leaking errors.

The plan correctly rejects arbitrary package scripts, shell features, command strings, and live provider credentials.

## 6. Environment And Tooling Assessment

The plan correctly treats Node/npm as a reviewed local tool boundary, not ambient runtime behavior.

Required implementation constraints:

- start from an empty environment;
- pass only reviewed values such as `PATH` and, if needed, `NPM_CONFIG_CACHE`;
- reject secret-like environment keys and values;
- do not inherit npm tokens, registry credentials, provider tokens, authorization headers, or private keys;
- prefer explicit npm executable path injection in tests;
- do not run install or dependency mutation commands.

The implementation prompt should resolve the open question by using explicit executable and cache inputs where real process execution is constructed.

## 7. Cache And Side-Effect Assessment

The plan is appropriately cautious about npm cache behavior.

Implementation should treat any npm cache write as cache-only behavior and should keep it out of the normal unit-test path where possible.

Confirmed non-goals:

- no source writes;
- no lockfile changes;
- no package metadata mutation;
- no dependency installation;
- no report artifacts;
- no state writes beyond ordinary explicit workflow execution events in tests.

## 8. Output And Redaction Assessment

The output policy is ready for implementation.

Confirmed:

- use `LocalCheckResult` for bounded summaries;
- mark truncation explicitly;
- reject secret-like stdout/stderr;
- do not persist raw output;
- do not store command transcripts;
- do not copy raw docs content or parser payloads;
- do not attach command-output evidence.

The previous `LocalCheckProcessOutput` debug blocker is fixed and reviewed, so this plan can safely build on the process-runner boundary.

## 9. Handler Registration Assessment

The registration posture is correctly conservative.

Implementation must keep:

- explicit construction only;
- no default registry entry;
- no CLI flag;
- no workflow schema field;
- no ambient replacement of `--mock-all-local-skills`;
- serialized `AllowlistedHandlerOnly` still failing closed unless a separate authorization review changes it.

This is important because even a narrow docs check should not become an unreviewed production command runner.

## 10. Runtime And Event Boundary Assessment

The plan preserves the existing runtime boundary.

Allowed implementation behavior:

- explicit test registration through existing `SkillHandler` and `LocalExecutor` mechanics;
- normal skill invocation events from explicit workflow runs;
- bounded `SkillOutput` derived from `LocalCheckResult`.

Still not allowed:

- new runtime event kinds;
- automatic check execution;
- post-terminal event appending;
- result persistence;
- report artifact writes;
- CLI rendering;
- schema changes.

## 11. Evidence And Work Report Boundary Assessment

The plan correctly defers evidence and work-report integration.

Implementation must not:

- create `EvidenceReference` values;
- attach command-output evidence;
- attach local check results to reports;
- create report artifacts;
- turn docs check output into a durable evidence ledger.

Future work can plan local check result citations separately.

## 12. Test Plan Assessment

The planned tests are strong and implementation-ready.

They cover:

- unsupported command kind rejection;
- canonical command vector;
- repository root working directory;
- sanitized environment;
- secret-like environment rejection;
- injected runner success/failure/timeout/spawn failure;
- secret-like stdout/stderr rejection;
- bounded output behavior;
- redaction-safe debug output;
- executor event boundary;
- no post-terminal events;
- no report artifacts;
- no CLI output;
- existing regression suites.

Implementation should add a test proving the handler remains non-default and is only used when explicitly registered.

## 13. Documentation Review

Docs accurately state:

- `DocsCheck` is planned, not implemented;
- production handler registration is not implemented;
- CLI exposure is not implemented;
- workflow schema fields are not implemented;
- automatic check execution is not implemented;
- report artifact writing is not implemented;
- evidence attachment is not implemented;
- side-effect boundary modeling is not implemented;
- writes remain unsupported.

## 14. Planning Blockers

None.

## 15. Non-Blocking Follow-Ups

- Decide later whether `AllowlistedHandlerOnly` should become valid for a reviewed subset of local check contracts.
- Decide later whether `DocsCheck` results should be cited in work reports.
- Decide later whether command-output evidence belongs in core, and under what redaction policy.
- Reassess public export posture of local check process-runner types before production API stabilization.

## 16. Recommended Next Phase

Recommended next phase: **explicit `DocsCheck` local handler implementation, non-default and test-scoped**.

Implementation should add the smallest handler that reuses existing local check infrastructure, supports injected-runner tests, preserves dogfood governance, and does not create CLI, schema, artifact, evidence, side-effect, write, or release posture changes.

## 17. Validation

Validation commands run for this review:

- `npm run check:docs`
  - Passed.
