# Hook Disclosure Model Plan Report

## 1. Executive Summary

Hook disclosure model planning is complete. The plan defines a conservative model-only path for bounded, validated, redaction-safe hook disclosures before any `Warning` or `SkippedWithDisclosure` runtime continuation is considered.

The plan keeps current executor behavior unchanged: `Passed` may continue, explicit `FailedClosed` blocks, and `Warning`, `SkippedWithDisclosure`, and `Blocked` remain unsupported.

## 2. Scope Completed

- Created [Hook Disclosure Model Plan](../implementation-plans/hook-disclosure-model-plan.md).
- Defined candidate hook disclosure model concepts.
- Defined allowed disclosure kinds and severity vocabulary.
- Defined validation, redaction, serde, Debug, and non-leakage requirements.
- Clarified relationship to hook statuses, WorkReports, audit/events, policy, and optionality.
- Defined a model-only future test plan.
- Recommended the next implementation phase.

## 3. Scope Explicitly Not Completed

This planning phase did not implement:

- hook disclosure model types;
- warning continuation;
- skipped-with-disclosure continuation;
- blocked runtime behavior;
- automatic hook invocation;
- broad executor hook checkpoints;
- workflow-declared hook configuration;
- runtime hook configuration;
- hook optionality semantics;
- policy-controlled continuation;
- post-terminal workflow events;
- dedicated hook audit sink emission;
- hook persistence;
- CLI behavior;
- schemas;
- local check execution;
- command execution;
- adapter invocation;
- external provider calls;
- evidence creation or attachment;
- approval attachment;
- report artifact writes;
- reasoning lineage;
- side-effect boundary implementation;
- writes;
- recursive agents or agent swarms;
- hosted or distributed runtime claims;
- release posture changes.

## 4. Planning Boundary Summary

The plan positions hook disclosures as a prerequisite for future warning/skipped behavior, not as authorization to continue execution.

Future disclosures must be structured, bounded, validated, and redaction-safe. They must not be copied from raw logs, command output, provider payloads, parser output, spec contents, diagnostics, or unbounded agent prose by default.

## 5. Privacy Summary

The plan forbids raw provider payloads, raw command output, raw CI logs, Jira/GitHub bodies, raw spec contents, parser payloads, environment values, credentials, authorization headers, private keys, token-like values, unbounded notes, and secret-like values in hook disclosures.

Errors must use stable codes and avoid raw disclosure content.

## 6. Test Coverage Plan Summary

The future model-only test plan covers valid warning/skipped disclosures, invalid IDs, unbounded and secret-like text rejection, stable reference validation, duplicate reference rejection, redaction metadata validation, serde round trips, invalid serialized failure, Debug non-leakage, serialization non-leakage, and existing regression suites.

## 7. Commands Run And Results

- `npm run check:docs`: failed in the desktop shell because `npm` is not on PATH.
- `PATH=/Users/rsegar/Documents/WorkflowOS/.tools/node-v20.19.5-darwin-arm64/bin:$PATH NPM_CONFIG_CACHE=/Users/rsegar/Documents/WorkflowOS/.tools/npm-cache npm run check:docs`: passed.

## 8. Remaining Known Limitations

- No hook disclosure code exists yet.
- Warning/skipped continuation remains unsupported.
- Hook optionality remains unmodeled.
- Policy-controlled warning/skipped continuation remains unplanned beyond prerequisites.
- WorkReport disclosure integration remains deferred.
- Hook event/audit persistence remains deferred.

## 9. Recommended Next Phase

Recommended next phase: **hook disclosure core model, model-only**.

That implementation should add bounded, validated, redaction-safe hook disclosure types and focused tests only. It must not broaden hook runtime status behavior or introduce automatic hooks, persistence, CLI behavior, schemas, command execution, adapter invocation, side effects, writes, or release posture changes.
