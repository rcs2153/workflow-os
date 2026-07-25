# DocsCheck Attestation Runtime Composition Plan Report

## 1. Executive Summary

Planning is complete for one explicit, in-memory, opt-in `DocsCheck` runtime
composition helper. The future helper will freeze the immutable execution
binding before process launch, derive the structured result and Core-owned
observation from one bounded process output, construct the unverified candidate
internally, and invoke the accepted independent attestation verifier.

No runtime behavior was implemented in this phase.

Focused review identified and the planning blocker fix resolves two details:
the helper now owns an injected clock for all binding and observation time, and
typed requirement eligibility determines honest no-proof outcomes before the
verifier is called.

## 2. Scope Completed

- inspected the current explicit `DocsCheckLocalHandler`, process-runner,
  structured-result, registry, executor, execution-binding, and verifier
  boundaries;
- selected an additive internal helper rather than changing executor defaults;
- defined explicit inputs, ordering, observation ownership, outcome semantics,
  privacy rules, tests, and implementation sequence; and
- linked the plan from the attestation roadmap documentation.

## 3. Scope Explicitly Not Completed

No check execution path, executor integration, default registration,
persistence, events, audit projection, evidence, reports, artifacts, schemas,
SDK, CLI, UI, providers, SideEffects, writes, hosted execution, or release
changes were implemented.

## 4. Core Decision

The first runtime slice will be one explicit helper for the canonical
`DocsCheck` contract. It will not parse `SkillOutput` or treat an output
reference as proof. Core will create the execution binding before launch and
derive the observation, result, candidate, and accepted proof from the same
bounded process execution.

## 5. Failure Posture

Passed checks may return accepted proof. Honest failed or timed-out checks will
retain a structured result and return no accepted proof. Internal execution,
binding, redaction, or verification failures return stable non-leaking errors
and no partial outcome.

## 6. Governed Phase

- workflow: `dg/d`
- run: `run-1784521724003437000-2`
- approval: `approval/run-1784521724003437000-2/planning-approved`
- presentation: `presentation/db0c74f0fb4d4f2a`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- kernel boundary: governance coordination only; repository inspection,
  planning, documentation, and validation ran outside the kernel

## 7. Validation

- `npm run check:docs` - passed.
- `git diff --check` - passed.
- governed phase close - completed with 39 events, one granted approval,
  zero retries, and zero escalations.

The phase-close helper reported the known
`approval_presentation_enforcement: proof_record_read_error` after reaching the
250-record disclosure cap. The exact presentation proof was persisted and used
for approval; this is a bounded phase-close read defect, not evidence that the
approval lacked presentation proof.

## 8. Remaining Limitations

- no runtime composition exists yet;
- no independent proof is produced by the current handler path;
- handler implementation provenance remains registered-unattested;
- freshness must later be reevaluated at time of use; and
- the dogfood phase-close presentation-list cap defect remains open.

## 9. Recommended Next Phase

Perform a focused re-review of the corrected plan. If accepted, implement the
one explicit in-memory `DocsCheck` composition helper only.
