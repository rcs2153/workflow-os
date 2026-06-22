# Workflow Discovery Field Coverage Integration Plan

Status: Implemented as bounded first-run recommendation output in [Workflow Discovery Field Coverage Integration Report](../concepts/WORKFLOW_DISCOVERY_FIELD_COVERAGE_INTEGRATION_REPORT.md). The implementation emits review-only workflow discovery recommendation records from existing first-run posture, ownership/escalation, and spec-field coverage signals. It does not implement catalog storage, workflow generation, workflow registration, schema changes, command execution, provider calls, writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.

## 1. Executive Summary

Workflow OS now has three local first-run onboarding signals:

- governance field posture;
- ownership and escalation warnings;
- spec-field coverage posture.

Those signals make rich YAML fields legible, but they are not yet connected to workflow discovery recommendations. The next question is how discovery should use these signals to recommend workflow additions, workflow changes, workflow splits, workflow retirements, ownership fixes, evidence/check requirements, approval posture, side-effect posture, and report obligations.

This plan defines a conservative, review-only integration path. Workflow OS should use first-run posture and coverage signals to produce better recommendations without creating workflow files, registering workflows, mutating specs, changing validation behavior, executing commands, calling providers, or writing catalog state.

## 2. Goals

- Connect first-run governance posture to workflow discovery recommendations.
- Use spec-field coverage to explain why a workflow recommendation exists.
- Use ownership/escalation warnings to recommend stewardship fixes.
- Keep recommendations bounded, deterministic, and redaction-safe.
- Preserve the distinction between observed posture and enforced behavior.
- Avoid treating advisory/deferred fields as active governance.
- Prepare future workflow catalog/store design without implementing it.
- Make manual workflow authoring less central over time while keeping humans in review and stewardship roles.

## 3. Non-Goals

Do not implement beyond the bounded first-run recommendation integration:

- automatic workflow generation;
- automatic workflow registration;
- catalog storage;
- workflow proposal persistence;
- schema changes;
- CLI commands;
- command execution;
- local check execution;
- provider calls;
- write-capable adapters;
- RBAC, IdP integration, paging, or enterprise notification;
- approval routing;
- hosted/distributed runtime behavior;
- recursive agents, agent swarms, or Level 3/4 autonomy.

## 4. Current Inputs

The future integration should consume existing bounded signals only.

### Governance Field Posture

Source: `workflow-os first-run` governance field posture output.

Useful signals:

- ownership configured, placeholder, or missing;
- escalation configured, placeholder, or missing;
- approval posture;
- policy gate posture;
- evidence/check posture;
- side-effect posture;
- audit/observability posture;
- deferred/advisory fields.

### Ownership And Escalation Check

Source: `workflow-os first-run` ownership/escalation check.

Useful signals:

- missing owner;
- placeholder owner;
- missing escalation contact;
- placeholder escalation contact;
- lifecycle warning;
- authority context warning.

These signals should produce stewardship recommendations, not authorization decisions or notification routing.

### Spec Field Coverage Check

Source: `workflow-os first-run` spec-field coverage check.

Useful signals:

- enforced fields;
- validated fields;
- disclosed fields;
- advisory fields;
- deferred fields;
- stable item codes such as `spec_field.triggers.not_background_executed`.

These signals should explain current coverage and recommendation rationale. They must not be treated as proof that unsupported automation exists.

## 5. Recommendation Categories

Workflow discovery should produce review-only recommendations in bounded categories:

| Category | Purpose |
| --- | --- |
| Create workflow | A repeated or important work pattern should become a governed workflow. |
| Change workflow | Existing workflow needs additional gates, evidence, checks, owners, or reports. |
| Split workflow | Existing workflow mixes separable planning, implementation, approval, verification, or reporting concerns. |
| Merge or relate workflows | Multiple workflows overlap and should be merged, linked, or clarified. |
| Retire workflow | Workflow appears stale, superseded, unsafe, or no longer useful. |
| Assign ownership | Workflow or skill has missing/placeholder ownership or unclear stewardship. |
| Add evidence/check requirements | Work relies on unverified claims or skipped checks. |
| Add approval posture | Work crosses sensitive authority, side-effect, or risk boundaries. |
| Add side-effect posture | Workflow touches or may touch external resources without clear side-effect disclosure. |
| Add report/handoff obligations | Work needs auditable closure or typed transfer to downstream actors. |

Recommendations should include rationale codes, not raw values.

## 6. Recommendation Shape

Future recommendation records should be structured and redaction-safe.

Candidate fields:

- recommendation id;
- recommendation kind;
- target surface;
- target ordinal or stable ID if safe;
- rationale codes;
- related coverage item codes;
- related ownership/escalation issue codes;
- suggested lifecycle posture;
- suggested owner/steward posture, without raw owner values;
- suggested evidence/check posture;
- suggested approval posture;
- suggested side-effect posture;
- suggested report/handoff posture;
- conflict hints;
- status, such as `review_only`, `needs_human_review`, or `deferred`.

The recommendation must not contain raw spec values, source snippets, command output, provider payloads, parser payloads, environment values, credentials, tokens, private keys, or raw owner/escalation values.

## 7. Rationale Policy

Every recommendation should cite bounded rationale.

Allowed rationale inputs:

- first-run posture labels;
- ownership/escalation issue codes;
- spec-field coverage item codes;
- workflow/skill/policy/test counts;
- known schema field names;
- lifecycle status categories;
- existing validation diagnostics by code where safe.

Forbidden rationale inputs:

- raw YAML field values;
- raw mapping literals;
- raw config values;
- raw owner or escalation contact values;
- source file contents;
- command output;
- provider payloads;
- parser payloads;
- environment variable values;
- credentials, tokens, or secret-like strings.

## 8. Initial Discovery Policies

The first implementation should use conservative policies:

- `review_only`: emit recommendations without changing project files.
- `no_auto_registration`: do not create active workflow specs.
- `no_catalog_write`: do not persist recommendations outside explicit reports or output.
- `bounded_rationale`: use stable codes and posture labels only.
- `no_raw_values`: never include caller-supplied field values.
- `schema_vocabulary_only`: field names are allowed only when they are known Workflow OS vocabulary.

If a recommendation cannot be expressed without raw values, it should be omitted or replaced with a bounded `recommendation_redacted` disclosure.

## 9. Relationship To First-Run

First-run should remain the natural place to surface initial recommendations for a newly scaffolded repo.

The first implementation now:

- group recommendations by category;
- add rationale codes;
- cite spec-field coverage item codes;
- cite ownership/escalation issue codes;
- emit review-only or needs-human-review status;
- keep legacy human-readable recommendation text for operator readability.

Future first-run recommendation improvements may:

- say which recommendations are dogfood-specific versus portable;
- explain which items are advisory or deferred;
- add conflict hints once a workflow catalog/store exists.

First-run must still not:

- run workflows;
- create runtime state;
- create report artifacts;
- auto-register workflows;
- execute commands;
- call providers;
- write files beyond explicitly requested scaffold commands.

## 10. Relationship To Dogfood Workflow Discovery

`dg/workflow-discovery` is a dogfood workflow for Workflow OS development. It is not a community-default workflow.

The future integration should learn from its pattern:

- review repeated work;
- identify missing workflow boundaries;
- flag overlap or conflicts;
- recommend splits, merges, retirements, or new workflows;
- require human review before adoption.

The portable user-facing path should not ask users to adopt `dg/*` workflows. It should generate bounded first-run recommendations and later feed a governed catalog/store.

## 11. Conflict And Overlap Policy

Workflow discovery recommendations should avoid creating conflicting workflow advice.

Potential conflict dimensions:

- same workflow ID;
- overlapping purpose;
- overlapping owner/steward responsibility;
- overlapping authority scope;
- overlapping side-effect/resource boundary;
- incompatible approval posture;
- incompatible policy gates;
- incompatible evidence/report obligations;
- stale lifecycle or supersession relationship;
- unsafe dependency cycle.

The first integration should report conflict hints only. It should not resolve conflicts automatically, mutate workflow definitions, or choose winners.

## 12. Human Stewardship Policy

Humans should remain the review and stewardship layer.

The integration should help humans:

- see why a recommendation exists;
- inspect bounded rationale;
- decide whether to create, change, split, merge, or retire a workflow;
- assign owner/steward and escalation responsibility;
- decide whether a recommendation is local-only, team-worthy, or enterprise-worthy;
- reject recommendations that are redundant, unsafe, or too broad.

The kernel should not assume humans will manually author every workflow forever, but it also should not promote recommendations without review.

## 13. Privacy And Redaction

Recommendation output should be safe for public logs and PR review.

It may disclose:

- recommendation category;
- bounded posture labels;
- stable issue codes;
- stable coverage item codes;
- schema vocabulary;
- counts;
- target ordinals or safe stable IDs.

It must not disclose:

- raw spec contents;
- raw config values;
- raw mapping literals;
- raw owner/maintainer/escalation values;
- raw source snippets;
- command output;
- provider payloads;
- parser payloads;
- environment values;
- credentials, tokens, private keys, or authorization headers.

## 14. Test Plan For Future Implementation

Future implementation should test:

- first-run recommendations include bounded rationale codes;
- recommendations can cite spec-field coverage item codes;
- recommendations can cite ownership/escalation issue codes;
- advisory and deferred fields are not presented as enforced;
- missing owner produces ownership recommendation without printing owner values;
- missing escalation produces escalation recommendation without routing or notification claims;
- trigger deferred execution produces a recommendation without implying background execution;
- test assertion deferred execution produces a recommendation without running tests;
- side-effect/adapter deferred posture produces a recommendation without provider calls or writes;
- recommendation output omits raw config values and mapping literals;
- secret-like spec values still fail closed through existing validation;
- recommendation ordering is deterministic;
- no files, catalog records, runtime state, events, or report artifacts are created.

## 15. Proposed Implementation Sequence

1. Add an internal recommendation taxonomy for first-run workflow discovery output.
2. Add bounded recommendation records derived from existing first-run posture, ownership/escalation, and spec-field coverage signals.
3. Add first-run text output with category, target, rationale codes, and review-only status.
4. Add preview JSON output with the same bounded shape.
5. Add tests for deterministic ordering and non-leakage.
6. Run maintainer review.
7. Only after review, plan catalog/store models or workflow proposal records.

## 16. Deferred Work

Explicitly defer:

- persistent workflow catalog;
- workflow proposal model;
- workflow promotion/review state;
- conflict-resolution engine;
- automatic workflow generation;
- automatic workflow registration;
- schema changes;
- CLI workflow-discovery command;
- command execution and local check execution;
- provider calls and write-capable adapters;
- enterprise RBAC, IdP, paging, notification, or escalation routing;
- hosted collaboration registry.

## 17. Final Recommendation

The next implementation prompt should be:

```text
First-run workflow discovery recommendations from existing posture signals.
```

It should add bounded, review-only recommendations to `workflow-os first-run`, derived from existing governance posture, ownership/escalation warnings, and spec-field coverage codes.

It must not build workflow generation, workflow registration, catalog storage, schema changes, command execution, provider calls, writes, hosted behavior, recursive agents, agent swarms, or Level 3/4 autonomy.
