# Side-Effect Boundary ADR Review

Review date: 2026-06-17

## 1. Executive Verdict

ADR accepted; proceed to SideEffect core model implementation planning.

ADR 0011 correctly defines the domain-neutral side-effect boundary Workflow OS needs before write-capable adapters. It separates mutation authority from lifecycle state, keeps writes denied, preserves the adapter boundary, and defines how future side-effect records should relate to policy decisions, approval decisions, idempotency, audit events, EvidenceReference, adapter telemetry, and WorkReport citations.

Acceptance is architecture direction only. It does not implement side-effect model code, writes, provider mutations, schemas, CLI behavior, persistence changes, examples, hosted runtime behavior, or release posture changes.

## 2. Scope Verification

The ADR and phase stayed within architecture/documentation scope.

No accidental implementation was found for:

- side-effect Rust model types;
- side-effect validation code;
- side-effect persistence;
- side-effect workflow events;
- side-effect audit projections;
- write-capable adapters;
- GitHub, Jira, CI, local filesystem, or provider mutations;
- generic runtime adapter execution;
- branch creation;
- pull request creation;
- pull request comments;
- issue updates;
- CI reruns;
- workflow dispatch;
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

## 3. Status Criteria Assessment

The ADR status criteria are sufficiently met for architecture acceptance.

Assessment:

- side-effect lifecycle vocabulary was reviewed and is acceptable for model-only v1;
- authority and lifecycle state are separated clearly;
- privacy and redaction implications are documented with concrete forbidden payload classes;
- relationships to policy, approval, audit, EvidenceReference, WorkReport, adapter telemetry, and idempotency are clear enough to guide a model-only implementation plan;
- acceptance preserves the Workflow OS product boundary;
- acceptance explicitly does not authorize writes or runtime execution.

The next phase must still produce a focused implementation plan before Rust model work begins.

## 4. Architecture Fit

The ADR fits Workflow OS architecture.

Workflow OS already has policy gates, approvals, audit records, idempotency keys, read-only adapter telemetry, EvidenceReference, WorkReport, report artifacts, and governed multi-step execution. The missing primitive is a domain-neutral record for mutation intent and lifecycle.

The ADR correctly avoids treating:

- adapter telemetry as the source of truth for mutation authority;
- WorkReport prose as the source of truth for mutation lifecycle;
- approval decisions as proof of execution;
- provider-specific IDs as core domain model structure.

That keeps Workflow OS as a governed workflow kernel rather than a provider-specific automation bot.

## 5. Authority And Lifecycle Assessment

The authority/lifecycle split is the key correct decision.

Authority answers whether a side effect may proceed under policy, approval, capability, actor, kill-switch, and runtime conditions.

Lifecycle state answers whether the side effect is proposed, attempted, completed, denied, skipped, or failed.

The ADR correctly rejects `approved` as a primary lifecycle state in v1. Approval is authority context; it does not prove that a mutation was attempted or completed. This distinction will prevent reports, audit records, and adapter telemetry from overstating what happened.

## 6. Lifecycle Vocabulary Assessment

The minimal lifecycle vocabulary is appropriate:

- `proposed`;
- `attempted`;
- `completed`;
- `denied`;
- `skipped`;
- `failed`.

This is enough to model future write readiness without implying rollback or compensation. Keeping `rolled_back` future-only is the right posture because most external systems cannot honestly guarantee rollback without adapter-specific semantics.

Non-blocking implementation guidance: the model plan should define whether `denied` can occur before a formal side-effect proposal record exists, or whether every denied action should create a denied record. The ADR does not need to answer that yet.

## 7. Policy, Approval, And Idempotency Assessment

The fail-closed policy rules are appropriate:

- credentials never imply permission;
- unknown capabilities fail closed;
- missing policy context fails closed;
- denied policy cannot be bypassed by adapter precheck, retry, CLI flag, worker restart, report generation, or SDK call;
- sensitive or ambiguous side effects require human approval;
- kill switch denies new proposals and attempts.

The idempotency direction is also correct. Side-effecting operations should require an idempotency key before attempt, and duplicate keys must not reattempt mutations. The future model should reference prior side-effect records or non-secret outcome references, not raw payloads.

## 8. Source-Of-Truth Assessment

The ADR preserves source-of-truth boundaries:

- workflow events remain run-state truth;
- audit events remain governance/operational projections;
- adapter telemetry records invocation and redacted outcome summaries;
- EvidenceReference remains citation substrate, not raw evidence storage;
- WorkReport remains a governed handoff artifact;
- future side-effect records become the source of truth for side-effect intent and lifecycle.

This is the right layering. It prevents side effects from becoming hidden inside reports or telemetry while avoiding a rewrite of the event-sourced runtime model in the ADR phase.

## 9. Audit, Evidence, And Report Assessment

The ADR correctly requires side-effect records to be citeable by audit events, WorkReports, EvidenceReference values, adapter telemetry, policy decisions, approval decisions, and future reasoning lineage if separately accepted.

The WorkReport rule is especially important: side-effect sections should disclose proposed, denied, skipped, attempted, completed, and failed side effects explicitly. Absence must not imply safety or non-occurrence.

The EvidenceReference rule is also correct: evidence may cite decision inputs and redacted outcome summaries, but side-effect modeling must not turn EvidenceReference into raw provider payload storage.

## 10. Privacy And Redaction Assessment

The ADR's privacy posture is strong.

It forbids side-effect records from storing:

- raw provider payloads;
- raw command output;
- raw CI logs;
- raw Jira issue bodies or comments;
- raw GitHub file contents;
- raw spec contents;
- parser payloads;
- environment variable values;
- credentials;
- authorization headers;
- private keys;
- token-like values;
- unbounded summaries;
- secret-like target identifiers or metadata.

The future implementation plan should translate these into concrete validation tests for constructors, serde, Debug, Display, deserialization errors, and validation errors.

## 11. Non-Goals Review

The ADR's non-goals are clear and sufficient.

It does not authorize writes, write-capable adapters, generic live adapter execution, runtime adapter write routing, schemas, CLI behavior, examples, persistence changes, hosted/distributed runtime behavior, rollback claims, or release posture changes.

This keeps the project out of the dangerous leap from read-only adapters to provider mutation.

## 12. Implementation Readiness Assessment

The ADR is ready to guide a model-only implementation plan.

The next plan should specify:

- exact Rust type names;
- required fields;
- validation error codes;
- serde shape;
- redaction-safe Debug behavior;
- sensitivity and redaction metadata handling;
- reference types for policy, approval, audit, adapter telemetry, evidence, idempotency, workflow/run/step/skill, and future report citation;
- tests proving no writes, no persistence changes, no schemas, no CLI behavior, no examples, and no release posture changes.

It should not implement provider writes or runtime side-effect attempts.

## 13. Blockers

None for ADR acceptance.

## 14. Non-Blocking Follow-Ups

- Decide in the model plan whether denied side effects always create records or only records after proposal.
- Decide whether side-effect target references should reuse existing adapter/action/capability vocabulary or introduce a new narrow target-reference wrapper.
- Decide whether WorkReport side-effect citation vocabulary should be added in the same model phase or separately.
- Keep high-assurance approval controls planning parallel, because sensitive writes will need stronger approval semantics before provider mutation.
- Keep write-adapter readiness planning separate from the SideEffect core model implementation.

## 15. Recommended Next Phase

Recommended next phase: SideEffect core model implementation planning.

The plan should be short and code-directed. It should move quickly into a Rust model-only phase after review, with no writes, no provider mutations, no schemas, no CLI behavior, no persistence changes, no examples, no hosted behavior, and no release posture changes.

## 16. Validation

- `npm run check:docs` - passed.
