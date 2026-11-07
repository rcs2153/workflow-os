# Cross-Harness Fixture Continuation Bridge Review

Status: not accepted; blocker correction required before the independent
Codex/LangGraph fixture corpus

## Blocker Correction In Progress

The active correction is governed by `dg/blocker`, run
`run-1794026064962845000-2`, approval
`approval/run-1794026064962845000-2/fix-approved`, with persisted presentation
`presentation/070b2c76f5e656cd`. Its state store is
`/private/tmp/workflow-os-cross-harness-bridge-blocker-state`.
The 39-event mock-backed governance run is Completed; this does not establish
completion of external implementation work or acceptance of this review.

Commit `adcfdf95` rejects non-fixture modes and malformed optional fixture
references. The subsequent worktree correction moves continuation to SQLite,
rejects terminal runs, and adds a read-only exact pre-dispatch attempt check.
The generic continuity disposition is deliberately not used: an Executing
window returns Blocked there, rather than ResumeNow. Fixture validation must
not reinterpret that result as dispatch permission.

Remaining acceptance work includes a successful public CLI continuation,
declared evidence/check obligation coverage, stale/expired/revoked/consumed
window cases, exact action binding, complete validation, and matching product
contract/report updates. Do not begin the independent harness corpus or claim
live cross-harness continuation until those requirements are proven. No
provider dispatch authority is issued by this validation-only path.

Candidate checkpoint validation: warnings-denied Clippy for Core and CLI with
all targets/features passed; the three focused local-executor continuation
tests passed, including terminal rejection and the approved Running prefix
without an execution window; docs validation passed. These are rejection-path
and compile/lint results, not evidence of a successful active continuation.
The candidate additionally compares the supplied SideEffect to its stored
record and checks cited results against required commands on the immutable
workflow prefix. Attestation, freshness, exact-action authority, and the
successful public path still require the acceptance work listed above.

The next candidate also compares the latest operational opening's operation
commitment to a domain-separated hash of the complete typed comment request,
including its body and preflight. Matching the target and SideEffect IDs alone
was insufficient to establish exact-action continuity. No existing generic
local-check window is thereby upgraded to comment authority. A fixture
preparation path must establish the corresponding binding through existing
current-authority admission before the positive test can pass; that path is
not implemented by this checkpoint.

## Verdict

The first public continuation bridge establishes useful durable identity,
approval-presentation, approval-linkage, actor, SideEffect, idempotency, and
immutable-bundle checks without performing a provider call or mutating runtime
state. Those are meaningful pieces of the preregistered cross-harness contract.

The bridge is not accepted yet. Two P0 contract gaps prevent it from
representing the preregistered interruption point, and three bounded public
boundary defects must be corrected before a second harness may depend on this
surface.

## Findings

### P0: The bridge requires a terminal run

`validate_github_pr_comment_fixture_continuation` reuses
`validate_live_sandbox_approval_authority_run`, which requires the run to be
terminal. The accepted milestone interrupts after authority, evidence, current
execution window, request commitment, and attempted SideEffect are durable but
before provider dispatch. That is active work, not a completed, failed, or
canceled run.

The current success test therefore constructs a completed run and inserts the
attempted SideEffect afterward. It proves post-run validation, not lawful
continuation of an interrupted execution. Requiring terminal state would teach
the replacement harness the wrong lifecycle contract and could allow a
terminal run to appear executable.

Required correction:

- define a continuation-specific run-state gate;
- require the exact non-terminal posture admitted by the interruption
  protocol;
- reject terminal replay as non-executable while preserving an inspectable
  terminal outcome; and
- prove that no provider call or state mutation occurs during fixture
  validation.

### P0: Required evidence/check and execution-window facts are not reassessed

The preregistered public continuation contract requires the replacement
executor to reassess required evidence, checks, and the current execution
window from kernel-owned durable state. The current bridge validates approval
presentation and approval-to-SideEffect linkage, but it accepts no durable
evidence/check or execution-window facts and performs no equivalent lookup.

The implementation report defers those gates to a later live slice. That is too
late. The fixture corpus must be able to prove stale window, missing evidence,
and changed check posture fail closed before any harness adapter or live
provider work begins.

Required correction:

- bind the bridge to the existing authoritative execution-window and
  evidence/check records rather than caller-supplied booleans;
- require an open, current, unconsumed window for the exact run/action;
- require the exact preregistered evidence/check obligations to remain
  satisfied;
- return stable non-payload errors for absent, stale, changed, expired,
  revoked, or consumed authority; and
- add adversarial tests for every rejected posture.

### P1: Dry-run input is accepted by a fixture-only command

The command is documented and rendered as `mode: fixture`, but the underlying
preflight and fixture validators also accept `DryRun`. A caller can therefore
supply matching dry-run values, receive a dry-run outcome, and still see the
command report `mode: fixture`.

Required correction: require `Fixture` at the continuation boundary and reject
all other modes with a continuation-specific error before fixture validation.

### P1: A malformed optional fixture reference is silently discarded

When `fixture_reference` is present with a non-string JSON value, the parser
treats it as absent. A security-sensitive continuation boundary must
distinguish omitted or null from malformed input.

Required correction: accept only absent, null, or a valid string, and reject a
present value of any other type without echoing it.

### P1: The public success path is not tested end to end

Core tests prove one helper-level success and two actor/delegation failures.
CLI tests prove discoverability, a required request path, and non-leaking
missing-input behavior. No test creates the durable state, invokes the public
CLI command successfully, and proves its bounded JSON output plus state
immutability.

Required correction: add one successful public CLI fixture-continuation test
and public-boundary adversarial tests for mode, malformed optional input,
terminal replay, execution-window posture, evidence/check posture, and exact
identity drift.

## Accepted Elements

The review accepts these implementation choices as foundations, not as final
milestone acceptance:

- exact durable-run equality;
- create-only immutable-bundle binding;
- exact workflow, version, schema, spec hash, run, step, target, SideEffect,
  and idempotency matching;
- same-actor recovery with explicit rejection of unsupported delegation;
- proof-enforced approval-presentation validation;
- store-backed approval-to-SideEffect linkage;
- no authentication input, provider call, event append, SideEffect mutation,
  or report write in the fixture slice; and
- bounded non-payload CLI output.

Same-actor recovery is acceptable for the fixture-first slice because harness
identity and authority identity are distinct. It does not establish delegated
actor portability. A later authenticated delegation contract remains required
before productization or a live cross-principal claim.

## Required Blocker-Fix Scope

The next phase is a narrow blocker fix, not the independent harness corpus. It
must:

1. model and validate the actual non-terminal interruption posture;
2. consume durable current execution-window and evidence/check facts;
3. enforce fixture-only mode and strict optional fixture parsing;
4. add a successful public CLI test plus the preregistered fail-closed cases
   now representable at this boundary;
5. preserve the no-provider, no-authentication, no-mutation boundary; and
6. update the implementation report and current product contract to match the
   corrected public behavior.

Do not add live transport, a generic provider-write command, another mutation
family, harness adapters, hosted orchestration, or release work in this fix.

## Validation

The review examined commit `b7c6b9f4`, the Core continuation helper, CLI input
and output boundary, focused tests, implementation report, current product
contract, roadmap, and accepted milestone preregistration. The implementation
phase's warnings-denied Rust checks, full local-executor and CLI suites, docs
check, and diff check were green; the findings above are contract and coverage
failures that those tests do not currently exercise.

## Governance Record

- workflow: `dg/review`
- run: `run-1794025801779552000-2`
- approval: `approval/run-1794025801779552000-2/review-scope-approved`
- approval presentation: `presentation/24d9b4bb9c6871ba`
- approval outcome: granted under delegated maintainer authority after the
  complete scope and non-goals were presented
- review result: not accepted; blocker correction required
- kernel role: governance coordination and durable approval record only
- execution disclosure: repository inspection and review writing occurred
  outside the kernel; no provider call or runtime mutation occurred
