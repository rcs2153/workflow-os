# Side-Effect Boundary ADR Report

## 1. Executive Summary

The Side-Effect Boundary ADR phase drafted [ADR 0011: Side-Effect Boundary Core Model](../adr/0011-side-effect-boundary.md).

ADR 0011 proposes a domain-neutral architecture boundary for representing side-effect intent, authority, lifecycle state, idempotency, audit, evidence, and report citation before write-capable adapters are implemented.

The ADR is proposed, not accepted. This phase does not implement side-effect model code, writes, provider mutations, generic runtime adapter execution, schemas, CLI behavior, persistence changes, examples, hosted runtime behavior, or release posture changes.

## 2. Scope Completed

Completed:

- created proposed ADR 0011;
- defined why side-effect boundary modeling is required before writes;
- separated side-effect authority from lifecycle state;
- proposed minimal lifecycle vocabulary;
- defined fail-closed authority and policy rules;
- defined idempotency expectations for future mutation attempts;
- documented source-of-truth boundaries between workflow events, audit events, adapter telemetry, EvidenceReference, WorkReport, and future side-effect records;
- documented audit, evidence, and report interaction rules;
- documented privacy and redaction requirements;
- updated the side-effect ADR plan status;
- updated roadmap and concept docs to link ADR 0011 without overclaiming implementation.

## 3. Scope Explicitly Not Completed

Not implemented:

- side-effect model Rust code;
- side-effect validation types;
- side-effect persistence;
- side-effect workflow events;
- side-effect audit projections;
- write-capable adapters;
- GitHub, Jira, CI, local filesystem, or provider mutations;
- generic runtime adapter execution;
- branch creation, pull request creation, comments, issue updates, CI reruns, workflow dispatch, or status writes;
- schemas;
- CLI commands or rendering;
- example updates;
- automatic report artifact behavior;
- domain packs;
- hosted or distributed runtime behavior;
- production SIEM, DLP, access control, OAuth, or webhook behavior;
- rollback or compensation claims;
- Level 3/4 autonomy enablement;
- release posture changes.

## 4. ADR Summary

ADR 0011 proposes that Workflow OS add a model-only side-effect boundary before writes.

The ADR recommends that side-effect records eventually represent:

- stable identity;
- target reference;
- requested capability;
- lifecycle state;
- actor or system actor;
- workflow and run identity;
- step and skill identity when available;
- policy decision references;
- approval decision references;
- idempotency binding;
- adapter telemetry or outcome references;
- evidence references;
- audit event references;
- WorkReport citation compatibility;
- sensitivity and redaction metadata.

## 5. Authority And Lifecycle Summary

ADR 0011 keeps authority separate from lifecycle state.

Authority answers whether a side effect is allowed under policy, approval, capability, actor, and runtime conditions.

Lifecycle state answers where the side effect is in its mutation timeline.

The ADR recommends not using `approved` as a primary lifecycle state in v1 because approval is authority context, not proof of attempt or completion.

## 6. Privacy And Redaction Summary

ADR 0011 requires future side-effect records to remain reference-first and redaction-safe.

Future side-effect records must not store raw provider payloads, raw command output, raw CI logs, raw Jira bodies/comments, raw GitHub file contents, raw spec contents, parser payloads, environment variable values, credentials, authorization headers, private keys, token-like values, unbounded summaries, or secret-like target identifiers and metadata.

Debug, Display, serialization, deserialization errors, validation errors, audit projection, and report citation behavior must be bounded and non-leaking.

## 7. Documentation Updates

Updated:

- [Side-Effect Boundary ADR Plan](../implementation-plans/side-effect-boundary-adr-plan.md);
- [Roadmap](../../ROADMAP.md);
- [Governed Work Pattern](governed-work-pattern.md);
- [Evidence Reference](evidence-reference.md).

These docs now state that side-effect boundary architecture is proposed in ADR 0011 while the model, writes, schemas, CLI, persistence, adapter writes, runtime side-effect execution, and release posture changes remain unimplemented.

## 8. Commands Run And Results

- `npm run check:docs` - passed.

## 9. Remaining Known Limitations

- ADR 0011 is proposed and still requires maintainer review.
- No side-effect Rust model exists yet.
- Side-effect lifecycle vocabulary is not yet a public contract.
- WorkReport side-effect citation support for side-effect records is not implemented.
- EvidenceReference attachment for side effects is not implemented.
- High-assurance approval controls remain future work.
- Write-adapter readiness still requires separate planning and review.

## 10. Recommended Next Phase

Recommended next phase: side-effect boundary ADR review.

If accepted, the following implementation phase should be SideEffect core model only: domain-neutral Rust model types, deterministic validation, serde support, redaction-safe Debug behavior, and focused tests. That implementation must still avoid provider writes, schemas, CLI behavior, persistence changes, examples, hosted behavior, and release posture changes.
