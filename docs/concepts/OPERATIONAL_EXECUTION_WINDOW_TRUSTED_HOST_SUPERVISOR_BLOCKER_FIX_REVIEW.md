# Operational Execution Window Trusted-Host Supervisor Blocker Fix Review

## 1. Executive Verdict

**Blocker fix accepted with non-blocking follow-ups.**

The repair satisfies the accepted private one-shot trusted-host boundary. A
conforming caller must transfer the attempt capability into one supervisor
call, current durable attempt state is checked before executor entry, the
complete skill invocation and selected executor commitment are bound before
authority issuance, and ambiguous result persistence is reconciled against
the exact durable operation on a fresh connection.

This acceptance does not authorize repeated scheduling or a public/provider
execution boundary. The current dispatchability check is a read, not an atomic
dispatch reservation. Before multiple supervisors, repeated scheduling, or an
untrusted host can be introduced, Core must provide one-winner dispatch
reservation so concurrent reconstructed internal authority cannot pass the
same pre-dispatch read.

## 2. Scope Verification

The fix stayed inside the approved private, local, SQLite-only blocker scope.
It did not add:

- a scheduler, daemon, queue, polling loop, or repeated dispatch;
- model-turn creation or conversational auto-resume;
- automatic approval, evidence bypass, or check bypass;
- provider execution, provider mutation, OpenShell, or live adapters;
- nested harness execution, recursive agents, or agent swarms;
- public runtime configuration, CLI, SDK, schema, UI, or example behavior;
- filesystem or PostgreSQL operational-opening support;
- prompts, transcripts, source bodies, command output, or provider payloads;
- hosted execution, Reasoning Lineage, or release posture changes; or
- a sixth public continuity operation.

## 3. Original Blockers

The original review identified four blocking areas:

1. borrowed attempt authority could enter the executor more than once;
2. opening authority did not bind the complete `SkillInput` or selected
   executor;
3. ambiguous result persistence was not reconciled on a fresh connection; and
4. the result, substitution, replay, yield, and persistence-fault matrix was
   incomplete.

All four areas received bounded repairs and executable regression coverage.

## 4. One-Use Dispatch Authority Assessment

`TrustedHostSupervisorInput` now owns its `SkillInput` and its opened or
resumed capability. `supervise_one_local_skill_attempt(...)` consumes that
input by value. The private capability types are not `Clone`, so a conforming
host cannot ordinarily call the supervisor twice with the same authority.

Before executor entry, the SQLite backend reloads current continuity state and
requires the exact live trusted-time posture, executing window, workflow, run,
step, actor, immutable bundle, governance and authority commitments, cursor,
window revision, started attempt, attempt revision, attempt origin, and
capability binding. A stale reconstructed capability after a recorded result
fails with `trusted_host_supervisor.attempt_not_dispatchable`, and the executor
call count remains unchanged.

This closes the original sequential replay defect for the declared private
one-shot boundary. It is not an atomic claim against concurrent reconstructed
internal capabilities: two non-conforming in-crate callers could both read the
attempt as started before either records a result. That limitation is
non-blocking only because the accepted slice has one injected caller and no
repeated scheduler. It becomes a blocker before any broader dispatch topology.

## 5. Invocation And Executor Binding Assessment

The opening authority now commits:

- workflow ID and version;
- schema version and spec hash;
- run ID and step ID;
- skill ID and version;
- correlation ID;
- canonical ordered input names and values; and
- an explicit executor binding commitment.

The operation binding incorporates that invocation commitment before the
opening capability is issued. The supervisor recomputes it from the owned
input and injected executor before executor entry. Domain-separated,
length-framed hashing prevents field-boundary ambiguity, and `BTreeMap`
iteration gives deterministic value ordering.

The focused substitution table covers every `SkillInput` field and executor
identity. A separate test covers step substitution. All substitutions fail
before the executor callback.

The executor commitment is trusted-host selection consistency, not binary,
process, hardware, or remote attestation. The implementation and report state
that limitation honestly.

## 6. Persistence And Reconciliation Assessment

Yield, outcome, and ambiguity persistence construct an exact reconciliation
request before mutation. If the projected write returns an error, the
supervisor reopens SQLite through the existing reconciliation boundary and
requires the same operation ID, request commitment, receipt ID, committed
disposition, and valid projection binding.

The supervisor distinguishes:

- durably committed success, which is accepted;
- durably committed security rejection, which remains a bounded security
  error;
- confirmed absence, which fails closed without fabricating success; and
- unreadable, conflicting, or corrupt state, which returns a stable
  reconciliation failure.

Before-commit and after-commit fault tests prove confirmed absence versus
durable success for the outcome path. The common reconciliation implementation
also covers yield and ambiguity writes. Dedicated fault injection for those
two call sites is a useful future test extension, but is not a blocker for this
private slice.

## 7. Closed Result And Yield Assessment

Focused tests cover success, retryable failure, terminal failure,
turn-boundary yield, ambiguous-may-have-started, and missing yield-generation
identity. The supervisor records the Core-derived disposition and does not
complete the workflow run merely because one skill attempt succeeds.

The result vocabulary remains closed. No provider-specific result, scheduler
state, fake workflow completion, or automatic retry authority was introduced.

## 8. Privacy And Error Safety

The repair persists commitments rather than raw invocation values. Custom
Debug behavior remains bounded, and remapped supervisor errors do not include
identifiers, paths, input values, SQLite details, credentials, prompts,
transcripts, command output, or provider payloads.

The executor binding is payload-free. Fresh-connection reconciliation does not
turn storage errors into user-controlled diagnostics.

## 9. Test Quality Assessment

The focused test boundary now proves:

- one owned capability drives one ordinary executor call;
- stale sequential replay cannot re-enter the executor;
- complete invocation and executor substitution fail before dispatch;
- all closed executor results persist deterministically;
- missing yield identity fails closed;
- before-commit ambiguity is confirmed absent;
- after-commit ambiguity reconciles to durable success;
- opening and generic-event cursor contention retain one-winner behavior; and
- supervisor success does not complete the workflow run.

The complete Workflow Core library suite passed with 343 tests. Focused
opening/supervisor coverage passed with 20 tests. Strict all-target Core clippy
also passed.

The full workspace matrix was not rerun locally because this host exhibits a
known multi-minute startup delay for each integration binary. CI must complete
that matrix before merge; this review does not represent it as a local pass.

## 10. Documentation Assessment

The roadmap, original phase report, original review, and blocker-fix report
preserve history and accurately distinguish:

- the accepted private one-shot boundary;
- the trusted-host rather than cryptographic executor identity;
- absent scheduling, providers, OpenShell, nested harnesses, public
  configuration, CLI, SDK, and schemas; and
- the requirement for focused re-review before broadening.

No dangerous capability overclaim remains after this review's atomic-dispatch
clarification.

## 11. Remaining Blockers

There are no remaining blockers for the private one-shot supervisor slice.

Repeated scheduling, multiple supervisors, public execution configuration,
provider execution, and OpenShell remain blocked until dispatch admission is
an atomic one-winner state transition rather than a read-only eligibility
check.

## 12. Non-Blocking Follow-Ups

1. Add dedicated before/after persistence-fault tests for yield and ambiguous
   recovery, not only the shared outcome reconciliation path.
2. Keep the executor commitment language limited to trusted-host selection
   consistency unless stronger attestation is separately designed.
3. Preserve `.workflow-os/` local state outside source commits.
4. Require CI to run the complete workspace matrix before merge.

## 13. Recommended Next Phase

Plan and implement a **bounded atomic dispatch-reservation slice** before any
repeated supervisor loop. The slice should atomically move one exact started
attempt into a dispatch-admitted posture, return one private non-reusable
dispatch capability, reject concurrent claimants before executor entry, and
preserve the accepted five-operation result/yield semantics.

It must not add scheduling, provider execution, OpenShell, model turns,
automatic approval, public configuration, schemas, hosted behavior, or another
mutation family.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1790898455087311000-2`
- approval: `approval/run-1790898455087311000-2/review-scope-approved`
- presentation: `presentation/3c3dac403f9a00eb`
- presentation hash:
  `3c3dac403f9a00eb36eabc60f6787705f37e11a426937a471f06db156310949a`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed implementation commit: `15008cc`
- phase status: completed
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, and six successful skill
  invocations; no retries or escalations
- validation summary: formatting, strict all-target Workflow Core clippy, 343
  Core library tests, 20 focused supervisor/opening tests, docs, and diff
  checks passed; the complete workspace matrix remains required in CI
- out-of-kernel work: source inspection, security analysis, validation, review
  authoring, and git actions were performed by the delegated maintainer; the
  kernel governed scope and approval but did not inspect code or edit files
