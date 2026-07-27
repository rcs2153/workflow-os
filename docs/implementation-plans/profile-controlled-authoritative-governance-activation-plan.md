# Profile-Controlled Authoritative Governance Activation Plan

Status: Implemented and accepted with non-blocking follow-ups.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Governance Strictness Profiles And Stewardship Plan](governance-strictness-profiles-and-stewardship-plan.md)
- [Proportional Governance And Quiet Success Plan](proportional-governance-quiet-success-plan.md)
- [Generic Explicit Local-Check Profile Source Plan](generic-explicit-local-check-profile-source-plan.md)
- [Authoritative Quiet-Success CLI Preview Plan](authoritative-quiet-success-cli-preview-plan.md)
- [Authoritative Quiet-Success CLI Preview Review](../concepts/AUTHORITATIVE_QUIET_SUCCESS_CLI_PREVIEW_REVIEW.md)

## 1. Executive Summary

The explicit authoritative quiet-success CLI preview is implemented and
accepted. It proves that Workflow OS can run one closed local validation
profile, derive proportional governance in Core, select quiet, visible,
approval-required, or denied behavior, preserve approval-presentation proof,
and produce an in-memory terminal WorkReport.

The preview still requires an operator to repeat
`--authoritative-governance` on `run` and `approve`. That is useful for proving
the boundary, but it is not an acceptable long-term source of runtime
governance. An operator can omit the flag, and the flag does not explain which
profile or local-check authority the project intended to use.

The next phase should add one typed project declaration that activates the
already accepted authoritative path:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

The declaration is explicit, validated, immutable-run-bound, and executable.
It does not accept command strings or provider configuration. If the
declaration is absent, ordinary CLI behavior remains unchanged. If it is
present but unsupported or incomplete, validation and execution fail closed.

This plan defined that implementation. The closed v0 activation path is now
implemented and documented in the
[Profile-Controlled Authoritative Governance Activation Report](../concepts/PROFILE_CONTROLLED_AUTHORITATIVE_GOVERNANCE_ACTIVATION_REPORT.md).
Phase-level maintainer review accepts the implementation after fixing two
compatibility blockers found by validation and review; see the
[Profile-Controlled Authoritative Governance Activation Review](../concepts/PROFILE_CONTROLLED_AUTHORITATIVE_GOVERNANCE_ACTIVATION_REVIEW.md).

## 2. Product Decision

Workflow OS should infer proportional-governance requirements from validated
workflow, skill, policy, check, authority, sensitivity, and SideEffect facts.
Users should configure reviewed constraints and activation posture, not
manually classify every decision.

The first project-level declaration therefore selects:

- the minimum governance strictness profile; and
- the only local-check execution authority available to this phase.

It does not let users:

- select the final route;
- provide precomputed risk or governance decisions;
- supply commands;
- weaken policy, approval, evidence, or check requirements; or
- suppress disclosure selected by Core.

Core remains responsible for deriving the actual execution and disclosure
axes. The project declaration is a reviewed input and minimum, not a caller
route override.

## 3. Goals

The implementation phase should:

1. add one typed, optional project-level authoritative-execution declaration;
2. bind `GovernanceStrictnessProfile` and
   `ExplicitLocalCheckProfileId` through the public project contract;
3. initially permit only `observe_and_report` with
   `workflow_os_project_validation`;
4. validate the declaration in Rust, JSON Schema, and the TypeScript SDK;
5. reject incomplete, unknown, or unsupported combinations fail closed;
6. activate the existing authoritative `run` path when the declaration is
   present;
7. activate the matching proof-enforced authoritative `approve` path when the
   waiting run was created under that declaration;
8. bind the exact declaration to the immutable run input and approval/resume
   integrity boundary;
9. retain `--authoritative-governance` as an explicit compatibility preview
   during this phase;
10. preserve ordinary CLI behavior for projects without the declaration;
11. preserve complete evidence, disclosure, audit, and report posture for
    quiet success; and
12. document the feature as local, experimental, closed-profile, and
    non-provider.

## 4. Strict Non-Goals

Do not add:

- automatic authoritative execution for undeclared projects;
- inferred activation from repository contents, `AGENTS.md`, environment
  variables, or first-run recommendations;
- arbitrary command strings, shell parsing, command discovery, or ambient
  executable lookup;
- additional local-check profile families;
- automatic approval or model self-approval;
- enterprise profile stewardship, RBAC, IdP integration, or hosted policy;
- provider execution or OpenShell integration;
- provider writes, SideEffect execution, credential injection, or network
  access;
- report persistence or report artifacts;
- new event or audit vocabularies unless a focused review proves existing
  binding vocabulary cannot represent the declaration truthfully;
- scaffold defaults or example updates;
- automatic workflow generation or promotion;
- reasoning lineage, nested harness execution, recursive agents, or agent
  swarms;
- release posture changes; or
- Level 3/4 autonomy claims.

## 5. Current Boundary

The accepted CLI preview currently:

- recognizes `--authoritative-governance` on `run` and `approve`;
- loads and validates the selected Workflow OS project;
- resolves the fixed `workflow_os_project_validation` profile;
- hardcodes `GovernanceStrictnessProfile::ObserveAndReport`;
- delegates route selection to the accepted Core dispatcher;
- persists complete approval-presentation proof when approval is required;
- performs fresh decision-time reassessment on approval;
- keeps aggregate governance approval separate from authored workflow
  approvals;
- emits bounded human and JSON output; and
- generates an in-memory terminal WorkReport.

The next implementation should reuse this path. It should not create a second
executor or governance classifier.

## 6. Source-Of-Truth Contract

Add one optional field to `ProjectManifest`:

```text
governance: Option<ProjectGovernanceConfiguration>
```

The first public shape should be:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

Candidate Rust types:

- `ProjectGovernanceConfiguration`
- `AuthoritativeExecutionConfiguration`

The configuration must use the existing closed enums:

- `GovernanceStrictnessProfile`
- `ExplicitLocalCheckProfileId`

It must not duplicate those values as free-form strings inside Core.

The complete `authoritative_execution` object is the activation boundary.
`governance` without `authoritative_execution` may remain absent, but a
partially specified authoritative-execution object is invalid.

## 7. Supported V0 Combination

The first implementation supports exactly:

```text
profile = observe_and_report
local_check_profile = workflow_os_project_validation
```

Other existing profile labels remain valid model vocabulary but are not valid
activation values in this slice:

- `agent_assisted_gated`
- `human_approval_gated`
- `strict_enterprise`

Those values must produce a stable validation error if declared under
`authoritative_execution`. They must not silently fall back to
`observe_and_report`, ordinary execution, or the CLI flag.

This closed combination keeps the implementation honest:

- the runtime can execute the selected local-check profile;
- Core can still select approval or denial when other facts require it;
- the profile does not authorize writes or external capabilities; and
- enterprise stewardship is not implied.

## 8. Schema, Parser, And SDK Contract

The implementation should update together:

- `ProjectManifest` and parser tests;
- `schemas/v0/project.schema.json`;
- TypeScript `ProjectManifestInput` and generated manifest contracts;
- schema/SDK synchronization fixtures; and
- user-facing field documentation.

Required schema behavior:

- `governance` is optional;
- `authoritative_execution` is optional;
- when present, both required fields are mandatory;
- values are closed enums;
- unknown fields are rejected;
- secret-like values remain rejected by existing spec validation;
- the supported-v0 combination receives semantic validation beyond schema
  shape; and
- serialization remains stable and explicit.

This is a public experimental contract. It must be documented and tested as
such.

## 9. Validation Semantics

Stable non-leaking errors should distinguish:

- malformed declaration shape;
- unsupported governance profile;
- unsupported local-check profile;
- unsupported profile/check combination; and
- runtime declaration mismatch with the immutable run.

Errors must not contain:

- absolute paths;
- raw spec contents;
- command output;
- environment values;
- credentials or tokens;
- approval reasons; or
- provider payloads.

Validation must fail before local process execution or run creation.

## 10. CLI Resolution Semantics

### `run`

For `workflow-os run <workflow-id>`:

1. load and validate the project;
2. resolve the project authoritative-execution declaration;
3. if absent, use ordinary existing execution;
4. if present and supported, use the existing authoritative CLI path;
5. if present but invalid or unsupported, fail before run creation; and
6. never infer activation from first-run recommendations or repository
   metadata.

### `approve`

For `workflow-os approve <run-id> <approval-id>`:

1. rehydrate the waiting run;
2. resolve the immutable project/run declaration that created the aggregate
   governance gate;
3. require the matching authoritative approval-resume path;
4. perform the existing fresh local validation and proportional-governance
   reassessment;
5. require complete persisted approval-presentation proof; and
6. reject declaration drift or missing authoritative context before approval
   mutation.

The operator must not need to remember the activation flag on resume when the
run already proves the authoritative declaration.

## 11. Explicit Flag Compatibility

Keep `--authoritative-governance` for this phase as an explicit experimental
compatibility path.

Rules:

- the flag may activate the current preview when the project declaration is
  absent;
- when the declaration is present, the flag must resolve to the exact same
  profile/check combination;
- the flag cannot override or weaken a declaration;
- conflicting sources fail closed; and
- CLI output identifies whether activation came from the project declaration
  or the explicit preview flag.

Flag deprecation is a later compatibility decision backed by usage and
operator testing.

## 12. Immutable Run And Resume Integrity

The exact authoritative-execution declaration must participate in the
immutable run-input boundary.

At minimum, the runtime must bind:

- declaration presence;
- strictness profile;
- local-check profile;
- project manifest content identity; and
- the existing local-check command/declaration inventory identity.

Approval resume must reject:

- a removed declaration;
- a changed profile;
- a changed local-check profile;
- a changed project manifest identity not represented by the accepted
  immutable-input rules; or
- a waiting run that lacks authoritative activation provenance.

The implementation must not re-read a changed manifest and treat it as the
authority that created an existing gate.

## 13. Quiet-Success Operator Contract

For an eligible low-risk run selected as quiet `Proceed`:

- execute the fixed project-validation check once;
- retain the bounded check result for report citation;
- execute the workflow through the accepted route;
- produce one concise completion result;
- preserve an inspectable run and event trail;
- generate the in-memory terminal WorkReport; and
- avoid approval language or interruption.

Quiet success does not mean:

- no governance;
- no evidence;
- no audit;
- no report posture;
- ignored check failures; or
- authority to write.

Visible disclosure remains an operator-presentation axis selected by Core, not
a different execution permission.

## 14. Approval And Denial Contract

If Core selects approval:

- persist and render the exact approval presentation record;
- include scope, non-goals, touched surfaces, validation expectations,
  why-now, next action, presentation ID, and content hash;
- pause before workflow step execution;
- require explicit proof-enforced approval; and
- reassess with fresh current check evidence before mutation.

If Core selects denial:

- fail before workflow step execution;
- record bounded denial posture through existing authoritative lifecycle
  behavior; and
- do not offer an approval command that could bypass denial.

The implementation should decide explicitly whether a failed project
validation check can still render a governance denial result or remains a
validation prerequisite failure. It must not conflate those outcomes.

## 15. Evidence, Audit, And Report Posture

The implementation must continue to:

- cite the exact same-call local-check result;
- avoid raw command output;
- avoid fabricating `EvidenceReference` values;
- preserve aggregate governance and authored workflow approvals as distinct;
- preserve event ordering and workflow status semantics;
- keep report-generation failure separate from workflow execution status; and
- disclose that WorkReports remain in memory only.

No new persistence or artifact claim is authorized.

## 16. First-Run And Onboarding Posture

This phase should not change scaffold defaults.

`workflow-os first-run` should, however, disclose:

- whether authoritative execution is undeclared or declared;
- the declared strictness profile;
- the declared local-check profile;
- whether the combination is supported and enforced;
- that repository recommendations remain review-only; and
- the exact bounded command needed to opt in if no declaration exists.

The concise output should stay short. Detailed field posture belongs in
verbose and JSON output.

A later onboarding phase may offer an explicit command to write this
declaration after review. It must not silently mutate existing projects.

## 17. Privacy And Security

- Use existing validated constructors and closed enums.
- Never accept shell commands, arguments, environment values, or paths in the
  declaration.
- Never read credentials for this profile.
- Keep network disabled and source access read-only.
- Keep Debug, Display, JSON, diagnostics, and deserialization errors bounded.
- Do not serialize raw check output into events, evidence, or reports.
- Treat declaration drift as an integrity failure, not a fallback condition.
- Do not let CLI source precedence weaken project-declared governance.

## 18. Test Plan

Future tests should prove:

1. a project without the declaration keeps ordinary `run` behavior;
2. a valid declaration activates the authoritative path without the flag;
3. the explicit flag still activates the preview when no declaration exists;
4. flag and declaration agreement produce one authoritative path;
5. flag/declaration conflict fails before run creation;
6. incomplete declarations fail validation;
7. unknown declaration fields fail validation;
8. unsupported profiles fail with stable non-leaking codes;
9. unsupported local-check profiles fail closed;
10. the valid v0 combination round-trips through serde;
11. Rust, JSON Schema, and TypeScript SDK shapes remain synchronized;
12. quiet `Proceed` completes with all required report sections;
13. visible `Proceed` delivers disclosure before execution;
14. approval-required posture pauses with a complete persisted presentation;
15. `approve` resumes authoritatively without requiring the flag;
16. authored workflow approvals remain distinct;
17. denial remains terminal and cannot be approved;
18. changed or removed declarations cannot resume an existing run;
19. the local check executes exactly once per authoritative decision point;
20. check failure does not silently downgrade to ordinary execution;
21. no raw command output, paths, tokens, or spec contents leak;
22. first-run concise, verbose, and JSON output disclose accurate posture;
23. existing ordinary CLI tests remain unchanged;
24. existing authoritative CLI tests still pass; and
25. workspace validation passes.

## 19. Implementation Sequence

Implement as one bounded feature phase with focused internal checkpoints:

1. add and validate the typed project declaration in Rust;
2. update JSON Schema and TypeScript SDK contracts;
3. bind the declaration to existing project/immutable-run identity;
4. resolve activation for `run`;
5. resolve existing-run activation for `approve`;
6. add first-run disclosure;
7. add focused compatibility, integrity, privacy, and route tests;
8. update product-contract documentation;
9. run full validation; and
10. perform a phase-level maintainer review before changing scaffold defaults
    or adding profiles.

Do not split this into model-only, projection-only, and helper-only roadmap
phases. The user value is the complete declared-to-enforced local path.

## 20. Validation

The implementation phase must run:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check:docs`
- `npm run check:integrations` under the repository-supported Node version
- `git diff --check`

Run focused schema, SDK, project parser, CLI, immutable-run, approval-resume,
and authoritative-governance tests while iterating.

## 21. Documentation Requirements

Update:

- `ROADMAP.md`
- `docs/user-guide/current-product-contract.md`
- project-manifest/spec field documentation
- CLI reference for `run` and `approve`
- the proportional-governance plan status
- the authoritative quiet-success preview plan

Docs must state:

- project-controlled authoritative activation is implemented only for the
  closed v0 combination;
- absent declarations preserve ordinary behavior;
- the CLI flag remains experimental compatibility;
- first-run recommendations do not activate execution;
- WorkReports remain in memory;
- no providers, OpenShell, writes, SideEffect execution, artifacts,
  enterprise controls, or hosted behavior are implemented.

## 22. Open Questions

- Should failed project validation prevent all route selection, or should an
  otherwise valid immutable run be able to produce an authoritative denial
  result?
- Should `first-run` recommend the declaration only when every assessed step
  is complete enough for an operator to review?
- When should the explicit CLI flag become deprecated?
- Should later profiles remain project-level minimums, or require a separate
  steward-controlled source before runtime enforcement?
- What future configuration source may tighten project posture without
  allowing local files to weaken enterprise minimums?

None of these questions block the first closed `observe_and_report` activation
slice.

## 23. Final Recommendation

Proceed next to one implementation phase for the complete
project-declared-to-runtime path.

Do not create another model-only planning chain. The accepted Core selector,
closed local-check profile, executor dispatcher, approval-resume integrity,
report consumer, and CLI preview already exist. The next value is composing
them behind one typed, explicit project declaration while preserving ordinary
behavior everywhere else.

After focused review, consider scaffold/onboarding opt-in ergonomics. Do not
broaden local-check profiles, provider mutations, OpenShell, automatic
approvals, enterprise stewardship, or hosted behavior first.

## 24. Governed Planning Record

- workflow: `dg/d`
- run: `run-1785117051268555000-2`
- approval: `approval/run-1785117051268555000-2/planning-approved`
- presentation: `presentation/60426edfcdae0c47`
- approval outcome: granted by delegated maintainer through presentation-proof
  enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: `npm run check:docs` and `git diff --check` passed
- skipped checks: Rust formatting, clippy, and workspace tests were not required
  for this documentation-only planning phase
- out-of-kernel work: architecture inspection, plan authoring, roadmap editing,
  and validation command execution
- kernel boundary: the kernel governed scope and approval but did not inspect
  code, edit files, run documentation checks, or perform git actions
- report posture: this planning document is the bounded phase report; no
  runtime WorkReport or report artifact was generated or persisted
