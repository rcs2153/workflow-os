# Local Project Authority Source Runtime Composition Plan

## 1. Executive Summary

The closed project-validation execution profile can be declared in
`workflow-os.yml`, but the CLI currently classifies workload authority as
`Sufficient` before Core evaluates the immutable run. This weakens the intended
boundary: callers should provide workload facts, while Workflow OS should
derive authority from a validated source.

This phase makes the validated project declaration the first production local
authority source for the closed project-validation profile. Core derives
current authority only after the declaration is captured in the immutable run
bundle. The same derivation is repeated during approval reassessment.

The standalone `--authoritative-governance` flag remains a compatibility path
when no project declaration exists. Replacing or removing that path is deferred.

## 2. Goals

- treat the validated project declaration as the authority source for the
  closed project-validation profile;
- bind the declaration to the immutable run bundle;
- reject caller-preclassified authority when project authority is declared;
- derive sufficient authority inside Core for fresh execution;
- repeat the derivation during approval reassessment;
- preserve existing local-check, governance, approval, report, and artifact
  behavior; and
- use stable, bounded, non-leaking failures.

## 3. Non-Goals

- actor-specific roles or enterprise RBAC;
- external identity, group, or administrator policy;
- ambient or inferred authority;
- automatic approval;
- a generic public authority-source API;
- OpenShell or another sandbox runtime;
- credentials, providers, SideEffects, or writes;
- new schemas, commands, profiles, or release posture;
- removal of the compatibility flag in this phase.

## 4. Trusted Source

The source is the validated
`project.governance.authoritative_execution` declaration with the only
supported v0 combination:

- profile: `observe_and_report`; and
- local-check profile: `workflow_os_project_validation`.

The immutable run bundle must contain the same activation. Core verifies the
requested configuration, strictness profile, and local-check profile against
that activation before supplying authority to proportional-governance
composition.

This declaration authorizes only the closed local project-validation execution
profile. It is not general tool authority and does not authorize provider
mutations.

## 5. Runtime Composition

For a project-declared request:

1. the caller supplies runtime facts with authority absent;
2. Core validates the project and builds the immutable run bundle;
3. Core verifies the stored authoritative activation;
4. Core inserts `Sufficient` authority into a cloned fact set;
5. the existing same-call local-check and proportional-governance route
   consumes those facts; and
6. approval resume repeats the immutable activation check before reassessment.

The caller cannot override the project-declared source with a preclassified
authority fact.

## 6. Compatibility Boundary

When no project declaration exists, the existing explicit CLI flag path
continues to supply its compatibility authority facts. That path remains
honestly documented as caller-classified and is not evidence of source-backed
authority.

A later reviewed phase should decide whether to deprecate the flag, require a
project declaration, or introduce another explicit trusted local source.

## 7. Failure And Privacy Semantics

The phase fails closed when:

- the immutable activation is absent;
- the requested activation or profile does not match;
- the caller preclassifies authority; or
- existing immutable-bundle or reassessment validation fails.

Errors use stable codes and do not include actors, run IDs, workflow IDs,
paths, commands, configuration values, provider data, credentials, or
secret-like values.

## 8. Test Plan

- project-declared execution succeeds with unclassified authority facts;
- the immutable bundle contains the exact activation;
- caller-preclassified authority fails before checks or workflow events;
- approval reassessment rebinds authority from the immutable activation;
- project declaration CLI execution remains successful;
- standalone compatibility-flag behavior remains unchanged; and
- existing authority, proportional-governance, executor, report, and workspace
  tests remain green.

## 9. Recommended Follow-Up

Review the implementation, then decide the compatibility flag retirement
posture before adding broader authority sources. After that, resume scoped
capability projection and authority receipt work.

Do not broaden provider mutations or add OpenShell first.
