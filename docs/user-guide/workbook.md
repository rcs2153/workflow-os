# Workflow OS Workbook

This workbook converts operating intent into reviewable artifacts before a team writes or changes Workflow OS specs.

Use it for RC1 internal evaluation, customer discovery, internal workflow design, and implementation planning. It is not a production-readiness certification. Completing the workbook does not prove a workflow is safe for production, write-capable adapters, distributed execution, or Level 3/4 autonomy.

## How To Use This Workbook

1. Fill out the relevant sections before authoring specs.
2. Keep sensitive payloads out of the workbook if it will be committed or shared.
3. Convert stable decisions into Workflow OS project files.
4. Run `workflow-os validate`.
5. Run local or fixture-backed examples only within the current supported boundary.
6. Record gaps honestly.

## Status Terms

| Term | Meaning |
| --- | --- |
| Implemented | Supported in the current local kernel path. |
| Internal fixture-backed | Supported for internal Phase 2 adapter evaluation with offline fixtures. |
| Future | Proposed direction or deferred implementation. |
| Unsupported | Must not be claimed or used as current behavior. |

## A. Workflow Qualification Sheet

### Purpose

Decide whether a workflow is a good candidate for Workflow OS evaluation.

### When To Use It

Use before writing specs, running discovery workshops, or selecting examples for pilot evaluation.

### Fields

| Field | Notes |
| --- | --- |
| Workflow candidate | Name the business workflow in domain-neutral language. |
| Business pain | Describe delay, risk, rework, handoff cost, or quality issue. |
| Current owner | Team or person accountable today. |
| Systems involved | Local files, GitHub, Jira, CI, document systems, HRIS, CRM, finance system, or other systems. |
| Data sensitivity | Public, internal, confidential, regulated, secret, or unknown. |
| Baseline metric | Cycle time, defect rate, approval wait, cost, SLA breach, rework, or escalation count. |
| Human judgment required | Where decisions, approvals, exceptions, or reviews happen. |
| External writes required? | If yes, mark unsupported for current RC1 evaluation. |
| Candidate autonomy level | Level 1 or Level 2 for current evaluation. Level 3/4 are future and denied by default. |
| Decision | Evaluate now, defer, reject, or redesign. |
| Notes | Non-secret operating notes. |

### Mapping To Workflow OS Artifacts

| Workbook field | Workflow OS mapping |
| --- | --- |
| Workflow candidate | Workflow ID and display name. |
| Systems involved | Adapter requirements or explicit non-goals. |
| Data sensitivity | Redaction rules, sensitive fields, audit requirements. |
| Human judgment required | Approval policies and escalation plan. |
| Baseline metric | Future evaluation criteria and observability plan. |

### RC1 Boundary Notes

Good RC1 candidates are local-first, approval-gated, Level 1/2 workflows that can run with deterministic local handlers or fixture-backed read-only adapters. Workflows requiring external writes, production credentials, distributed workers, or high autonomy are not RC1 candidates.

### Workflow Candidate Scoring Rubric

Use this rubric before investing in specs or examples.

| Score | Meaning | Signals |
| --- | --- | --- |
| Green | Suitable for RC1 evaluation. | Level 1/2 workflow; clear owner; measurable baseline; local or fixture-backed path; no external writes; no sensitive live data required; approval boundary is understandable; expected outcome can be inspected locally. |
| Yellow | Needs redesign or additional evidence before RC1 evaluation. | Owner is unclear; baseline metric is weak; sensitive data handling needs review; workflow needs live read-only data but no approved smoke resources exist; approval boundary is ambiguous; fixture path is incomplete; validation or handoff evidence is missing. |
| Red | Not suitable for current RC1 evaluation. | Requires GitHub/Jira/CI writes, CI reruns/dispatch, production credentials, distributed workers, production database behavior, Level 3/4 autonomy, unapproved sensitive live data, hosted service behavior, or unsupported domain-pack behavior. |

Default to Yellow or Red when the workflow would require behavior outside the current local kernel or fixture-backed read-only adapter boundary.

## B. Current-State Work Map

### Purpose

Capture how the work actually moves today.

### When To Use It

Use during discovery before designing the target workflow.

### Fields

| Stage | Current actor | System of record | Delay or friction | Decision latency / handoff latency | Failure mode | Evidence available | Sensitive data? | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | | |

### Mapping To Workflow OS Artifacts

| Workbook entry | Workflow OS mapping |
| --- | --- |
| Stage | Workflow step or future state model element. |
| Current actor | Owner metadata, actor attribution, approval actor. |
| System of record | Adapter requirement or local context reference. |
| Failure mode | Skill failure mode, retry policy, escalation policy. |
| Evidence available | Future `evidence_reference`; current docs/fixtures/references. |

### RC1 Boundary Notes

Treat this as discovery input. Workflow OS does not yet implement generic evidence references or work reports. Do not put secrets or full sensitive provider payloads in the workbook.

## C. Workflow Definition Canvas

### Purpose

Design the target governed execution loop.

### When To Use It

Use before creating or editing `workflows/*.workflow.yml`.

### Fields

| Field | Notes |
| --- | --- |
| Workflow ID | Stable ID. |
| Version | Workflow definition version. |
| Display name | Human-readable name. |
| Owner | Owning team, maintainer, escalation contact. |
| Lifecycle status | Experimental, stable, or deprecated. |
| Trigger | Manual, file, schedule, or external event declaration. Current CLI starts by workflow ID. |
| Autonomy level | Level 1 or Level 2 for current supported evaluation. |
| Step ID | Stable step ID. |
| Skill invoked | Skill ID and version. |
| Required policy | Policy IDs or policy intent. |
| Approval gate | Whether approval is required before skill invocation. |
| Completion criteria | What makes the run complete. |
| Failure behavior | Fail, retry, escalate, or cancel. |
| Inspection expectation | What an operator should see in `status` or `inspect`. |

### Mapping To Workflow OS Artifacts

Maps primarily to workflow specs, policy specs, skill references, terminal behavior, and audit/observability requirements.

### RC1 Boundary Notes

The local executor currently supports a narrow single-step local path. External event triggers, distributed execution, and generic adapter execution from arbitrary workflow specs remain future work.

## D. Skill Contract Template

### Purpose

Define each skill as an operational contract, not as an informal prompt.

### When To Use It

Use before creating or editing `skills/*.skill.yml`.

### Fields

| Field | Notes |
| --- | --- |
| Skill ID | Stable ID, such as `local/rec` or symbolic adapter skill for examples. |
| Version | Skill contract version. |
| Display name | Human-readable name. |
| Owner | Owning team, maintainer, escalation contact. |
| Inputs | Field names, types, required status, sensitivity, redaction. |
| Outputs | Field names, types, required status, sensitivity, redaction. |
| Allowed capabilities | Local, external read, approval, audit, or future capability names. |
| Adapter requirements | Symbolic adapter IDs and required capabilities, where applicable. |
| Constraints | What the skill must not do. |
| Failure modes | Retryable/non-retryable categories. |
| Evaluation criteria | What quality means for this skill. |
| Implementation status | Real handler, deterministic mock, fixture handler, future, or unsupported. |

### Mapping To Workflow OS Artifacts

Maps to skill specs, validation rules, local handler registration, policy capabilities, adapter requirements, redaction behavior, failure handling, and future evaluation criteria.

### RC1 Boundary Notes

Declaring a skill does not implement it. The CLI executes only registered local handlers. `--mock-all-local-skills` is explicit example tooling, not a real skill plugin system or AI model execution.

## E. Governance And Policy Matrix

### Purpose

Turn human governance intent into executable policy and operator review rules.

### When To Use It

Use before writing policies or enabling any workflow beyond a local evaluation path.

### Fields

| Action | Capability | Allowed? | Approval required? | Denied? | Actor/owner | Reason code | Evidence required | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Start workflow | | | | | | | | |
| Invoke skill | | | | | | | | |
| Request approval | | | | | | | | |
| Resume workflow | | | | | | | | |
| Invoke adapter read | | | | | | | | |
| External write | | | | | | | | |
| Secret read | | | | | | | | |

### Mapping To Workflow OS Artifacts

Maps to policy specs, capability declarations, runtime policy decisions, approval policies, audit records, and kill switch behavior.

### RC1 Boundary Notes

The default policy posture is conservative. Unknown actions, unknown capabilities, missing context, `external.write`, and `secret.read` fail closed. Write actions remain unsupported.

## F. Retry And Escalation Plan

### Purpose

Bound recovery and make human handoff explicit.

### When To Use It

Use for any workflow where transient failure is expected or where failure should not silently dead-end.

### Fields

| Failure mode | Failure class | Retry allowed? | Retry limit | Retry evidence | Escalate when | Escalation owner | Context packet | Terminal behavior |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | | |

### Mapping To Workflow OS Artifacts

Maps to retry policies, escalation policies, runtime retry events, `RetryExhausted`, `EscalationTriggered`, observability events, and operator runbooks.

### RC1 Boundary Notes

Bounded local retry and local escalation state are implemented. Background retry schedulers, external notifications, issue creation, paging, and human escalation resolution APIs are not implemented.

## G. Integration And Adapter Map

### Purpose

Define how Workflow OS may read from or eventually act on systems of record.

### When To Use It

Use before declaring adapter requirements or running Phase 2 read-only examples.

### Fields

| System | Adapter ID | Operation mode | Read capability | Write capability | Data sensitivity | Credential source | Live test status | Forbidden operations | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| GitHub | | fixture / live-read-only | `github.read` | unsupported | | env var only | not recorded / passed / failed | no branches, commits, PRs, comments, labels, merges | |
| Jira | | fixture / live-read-only | `jira.read` | unsupported | | env var only | not recorded / passed / failed | no issue updates, comments, transitions, assignment, labels | |
| CI/GitHub Actions | | fixture / live-read-only | `ci.read` | unsupported | | env var only | not recorded / passed / failed | no rerun, dispatch, cancellation, check mutation | |

### Mapping To Workflow OS Artifacts

Maps to adapter requirements, integration docs, provider setup docs, capabilities, policy prechecks, adapter telemetry, and live smoke evidence.

### RC1 Boundary Notes

Phase 2 read-only adapters are internal fixture-backed capabilities. Live mode is opt-in and not public-preview approved until evidence is recorded. Write-capable adapters are unsupported.

## H. Audit And Observability Plan

### Purpose

Define proof, metrics, redaction, and operating review signals.

### When To Use It

Use before a workflow is considered ready for serious evaluation.

### Fields

| Event or metric | Required? | Data stored | Redaction rule | Owner | Alert or review threshold | Inspect path | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Run started/completed/failed/canceled | | | | | | | |
| Policy allow/deny/approval-required | | | | | | | |
| Approval requested/granted/denied | | | | | | | |
| Skill success/failure | | | | | | | |
| Retry count/exhaustion | | | | | | | |
| Escalation | | | | | | | |
| Adapter read telemetry | | | | | | | |
| Backend health | | | | | | | |
| Incomplete/deferred work disclosure | | | | | | | |

### Mapping To Workflow OS Artifacts

Maps to audit sinks, observability sinks, local state artifacts, CLI inspect output, metrics docs, audit redaction rules, and future production telemetry plans.

### RC1 Boundary Notes

Local audit and observability are implemented for evaluation. Production SIEM export, OpenTelemetry integration, enterprise retention, and tamper-evident audit storage are not implemented.

## I. Autonomy Readiness Assessment

### Purpose

Decide whether a workflow has earned more authority.

### When To Use It

Use after repeated successful local or fixture-backed evaluation, and before considering any future higher-autonomy design.

### Fields

| Workflow | Current level | Target level | Required controls | Evidence | Missing controls | Decision | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | |

### Assessment Prompts

- Does validation consistently pass?
- Are policy decisions explicit and auditable?
- Are approvals required for sensitive or ambiguous steps?
- Are retries bounded?
- Does retry exhaustion escalate or fail closed?
- Are sensitive fields redacted?
- Is local state inspectable and restart-safe?
- Are adapter reads fixture-proven and, if relevant, live-smoke-proven?
- Are operators able to inspect status, history, approval, retry, escalation, audit, and telemetry signals?
- Is there evidence that the workflow improves the baseline metric without increasing risk?

### Mapping To Workflow OS Artifacts

Maps to `autonomy_level`, policy rules, approval gates, audit/observability evidence, test results, readiness reviews, and future maturity gates.

### RC1 Boundary Notes

Level 1 and Level 2 are the current supported posture. Level 3/4 are not enabled and must not be treated as available because a workbook says they are desired.

## J. RC1 Readiness / Production Readiness Assessment

### Purpose

Separate safe RC1 internal evaluation from future production readiness.

### When To Use It

Use at the end of an internal evaluation cycle or before proposing broader rollout.

### Fields

| Check | RC1 status | Evidence | Gap | Owner | Decision |
| --- | --- | --- | --- | --- | --- |
| Specs validate | | | | | |
| Local run succeeds or fails safely | | | | | |
| Approval path works | | | | | |
| Denial path works where relevant | | | | | |
| Inspect/status output is useful | | | | | |
| Audit/observability records exist | | | | | |
| Sensitive values are redacted | | | | | |
| Local state health is inspectable | | | | | |
| Fixture adapter path works if used | | | | | |
| Live smoke evidence exists if claiming live read-only readiness | | | | | |
| No unsupported write behavior is required | | | | | |
| Known limitations are accepted | | | | | |

### Mapping To Workflow OS Artifacts

Maps to validation output, CLI run/approve/status/inspect evidence, local state health, integration gate output, live smoke evidence, release readiness docs, and known limitations.

### RC1 Boundary Notes

This section is an assessment artifact, not proof of production readiness. Production readiness would require production backend design, distributed worker semantics, enterprise audit/observability export, live provider evidence where integrations are used, operational runbooks, security review, and explicit release approval.
