# Governed Workflow Authoring Promotion Plan Report

## 1. Executive Summary

This planning phase defines the next boundary after inactive workflow draft file output: governed workflow authoring promotion and steward review.

The plan treats promotion as a control-plane change. A draft should not become active merely because a file exists. Promotion should require deterministic preflight, owner and escalation posture, policy/evidence/check/report posture, conflict checks, and explicit steward or delegated maintainer approval.

This phase is planning only. It does not implement promotion, active workflow registration, file movement, command execution, provider calls, runtime state creation, schemas, examples, hosted behavior, write-capable adapters, or release posture changes.

## 2. Scope Completed

- Created [Governed Workflow Authoring Promotion And Steward Review Plan](../implementation-plans/governed-workflow-authoring-promotion-plan.md).
- Defined promotion as the boundary where an inactive draft becomes an active workflow spec.
- Defined steward review posture for local and future enterprise settings.
- Defined promotion preconditions.
- Defined conflict checks.
- Defined a candidate CLI shape for preflight-only promotion review.
- Defined validation, privacy, error-handling, and test requirements.
- Recommended the next implementation as promotion preflight only.
- Updated roadmap and umbrella authoring plan references.

## 3. Scope Explicitly Not Completed

This phase did not implement:

- workflow promotion;
- active workflow registration;
- file movement from `workflows/drafts/` to `workflows/`;
- workflow mutation;
- workflow catalog persistence;
- runtime state creation;
- command execution;
- local check execution;
- provider calls;
- report artifacts;
- schemas;
- examples;
- hosted or distributed runtime behavior;
- RBAC, IdP, admin UI, paging, or notification systems;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- write-capable adapters;
- release posture changes.

## 4. Planning Summary

The plan recommends a preflight-first approach:

```sh
workflow-os author workflow promote \
  --draft workflows/drafts/<name>.workflow.yml \
  --dry-run
```

or:

```sh
workflow-os author workflow preflight \
  --draft workflows/drafts/<name>.workflow.yml
```

The first implementation should inspect a draft and return a bounded promotability result. It should not move files, register workflows, write active specs, create state, append events, run commands, or call providers.

## 5. Steward Review Summary

The plan keeps local use simple while preserving the future enterprise path.

For local v0, the steward can be a human maintainer or explicitly delegated maintainer.

Future enterprise stewardship can add administrators, policy stewards, workflow owners, escalation owners, security reviewers, and audit/release stewards without changing the core principle: promotion is explicit governed authorization.

## 6. Validation Boundary Summary

Promotion preflight should verify:

- draft path safety;
- candidate workflow id validity;
- no active workflow id conflict;
- owner and escalation posture;
- triggers and steps;
- skill and policy references;
- supported policy effects;
- approval policy alignment;
- evidence/check posture;
- side-effect posture;
- audit/observability posture;
- report/handoff posture;
- supported autonomy level;
- no validation errors.

Purpose and authority conflict checks may start as warnings if deterministic taxonomy is not ready.

## 7. Privacy And Redaction Summary

Promotion preflight must use bounded metadata and structured diagnostics only.

It must not copy raw source contents, manifest bodies, package script bodies, dependency values, CI logs, provider payloads, issue or pull request bodies, parser payloads, absolute private paths, environment values, credentials, authorization headers, private keys, token-like strings, or existing agent instruction bodies.

Errors must not echo unsafe ids, paths, owner values, escalation values, notes, or raw payloads.

## 8. Test Coverage Planned

The plan defines future tests for:

- valid promotable drafts;
- incomplete generated drafts blocked;
- missing owner/escalation posture;
- missing triggers/steps;
- unresolved skill/policy references;
- duplicate workflow ids;
- unsafe paths and secret-like values;
- unsupported policy effects;
- side-effect posture;
- report/handoff posture;
- no runtime state;
- no file writes;
- no command execution;
- no provider calls;
- docs check.

## 9. Governed Dogfood Summary

- Workflow: `dg/d`.
- Run ID: `run-1783404280566835000-2`.
- Approval ID: `approval/run-1783404280566835000-2/planning-approved`.
- Approval outcome: granted by delegated maintainer after the full approval handoff was emitted.
- Scope: create promotion/steward-review planning document and phase report.
- Event summary: 39 events total; 1 approval; 0 retries; 0 escalations; terminal status `Completed`.
- Out-of-kernel work disclosed: documentation editing, docs validation, diff check, git commit, PR creation, and PR merge remain agent/GitHub actions outside the kernel.

## 10. Validation Commands Run

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- `npm run dogfood:benchmark -- phase-close run-1783404280566835000-2 --phase planning`: passed.

## 11. Remaining Known Limitations

- Promotion is not implemented.
- No preflight helper exists yet.
- No active workflow registration exists.
- No steward approval integration exists.
- No catalog/store-backed proposal state exists.
- No typed draft lifecycle schema value exists.
- No active workflow file movement is authorized.

## 12. Recommended Next Phase

Recommended next phase: governed workflow authoring promotion preflight implementation.

The first implementation should inspect a draft and return a bounded promotability result only. It must not move files, register workflows, activate drafts, create runtime state, execute commands, call providers, add schemas, add examples, enable writes, or change release posture.
