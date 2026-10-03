# Trusted-Host Explicit Reinvocation Vertical Slice Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; proceed to private host
scheduling-boundary planning.**

The crate-private local SQLite composition preserves the intended authority
boundary. A previously observed wait handoff remains inert orientation input,
the exact `TimeWindow` transition remains owned by the accepted trusted-time
verifier, and executor entry remains owned by fresh directive consumption and
atomic dispatch reservation through the existing trusted-host supervisor
path.

No blocker was found. Direct composition-level coverage of concurrent
reinvocation and the crash boundary between wait transition and operational
entry remains a required follow-up before any scheduler implementation.

## 2. Scope Verification

The implementation stays within the accepted slice:

- it is crate-private, explicit, local, and SQLite-only;
- it composes one already-registered exact `TimeWindow` wait;
- it adds no scheduler, poller, timer service, queue, daemon, or background
  worker;
- it adds no automatic model turn, agent reinvocation, or approval;
- it adds no provider execution, OpenShell, sandbox, nested harness, or
  provider mutation;
- it adds no public Rust API, CLI, SDK, schema, workflow configuration, UI, or
  example; and
- it changes no hosted, distributed, persistence-backend, or release posture.

## 3. Composition Assessment

`reinvoke_after_time_window_wait` composes two accepted operations rather than
creating a parallel authority path. It first calls the exact trusted-host
`TimeWindow` wake boundary with a Core-derived handoff commitment. Only after
a committed transition or exact replay does it call the existing-window-only
operational entry helper.

The composition never receives caller-authored revision, cursor, generation,
trusted-time observation, resume directive, dispatch capability, or authority
decision. Those values remain freshly derived inside the existing Core and
SQLite boundaries.

## 4. Handoff And Durable Replay Assessment

Fresh transition validates the supplied payload-free commitment against a
newly derived handoff from current authoritative state before mutation. The
derivation covers the exact window revision, cursor commitment, active yield
generation, and complete ordered unsatisfied wait set. Locator validation
separately binds workflow, run, step, window, actor, and immutable run bundle.

Handoff-bound transitions use a distinct v2 request commitment and a distinct
private durable envelope. The envelope retains the exact window revision,
window binding commitment, cursor, wait revision, generation, and handoff
commitment required to distinguish this operation from the legacy direct-wake
path. Exact replay checks that domain and commitment before continuing.

The handoff does not become a bearer capability. Its only exposed private
operation is a payload-free commitment clone used for stale detection and
replay binding.

## 5. Source-Specific Transition Assessment

The source-specific verifier remains responsible for current trusted time,
deadline satisfaction, exact dependency binding, atomic transition, and
ambiguous-commit replay. The composition does not use generic continuation
classification as proof that a deadline elapsed.

The transaction rechecks exact window binding, revision, cursor, active yield,
wait revision, dependency, and trusted-time posture. A race after the initial
coherent read therefore cannot commit against stale expectations.

## 6. Operational Entry And Authority Assessment

`TrustedHostExistingOperationalEntryInput` removes fresh-opening context and
opening-persistence identities from the reinvocation path. It cannot be used
to open a missing window or manufacture initial authority.

Existing-window entry reloads authoritative state, verifies exact scope and
invocation plus executor binding, derives current continuation disposition,
and obtains a fresh one-use resume directive only for `ResumeNow`. Directive
consumption and dispatch reservation remain the accepted one-winner
transactional boundary before executor entry. Current authority and required
context continue to be reassessed by the reused path.

## 7. Restart, Replay, And Concurrency Assessment

Focused coverage proves that a handoff-bound transition can be replayed after
backend reopen and that a substituted handoff commitment conflicts without
leaking its marker. The end-to-end test proves one elapsed transition, one
executor entry, terminal persistence, backend reopen, exact replay, and zero
additional executor entries.

The reused primitives retain direct concurrent proofs for one winning
`TimeWindow` transition and one winning operational dispatch. No process-local
lock was added. A direct test with two callers racing through the complete new
composition is still missing. That is non-blocking for this private explicit
slice because both mutation boundaries are independently transactional, but
it is required before a scheduler may invoke the composition concurrently.

The crash boundary after a committed transition but before operational entry
is structurally restart-safe: exact replay recovers the transition and
existing-window entry reloads current state. A dedicated fault-injection test
for that exact seam remains a non-blocking follow-up before scheduler work.

## 8. Failure And Security Assessment

- A stale or substituted handoff commitment fails before fresh mutation.
- Locator, immutable bundle, actor, invocation, and executor substitution
  continue to fail closed through existing binding checks.
- Trusted-time unavailability cannot fabricate deadline satisfaction.
- A committed security rejection becomes a stable reinvocation error and does
  not expose current disposition to the composition caller.
- Blocked, waiting, or terminal operational posture produces zero executor
  entries.
- Exact replay cannot reconstruct consumed authority or duplicate terminal
  work.

The lower-level wake result still contains disposition for its established
direct callers. The new reinvocation composition strips that posture when the
transition status is a security rejection, which satisfies the approved
non-oracle boundary without changing the older private caller contract.

## 9. Privacy Assessment

The new input and outcome are crate-private, non-serializable, and
Debug-redacted. Durable additions are bounded identities and payload-free
commitments in the existing private operation envelope. No deadline, prompt,
command, source content, provider payload, evidence body, credential, token,
or raw executor input/output is added to public output or a new store.

Errors use stable static messages and focused tests confirm that substituted
secret-like handoff markers are not echoed.

## 10. Test Quality Assessment

Added coverage proves:

- fresh stale-handoff rejection before mutation;
- domain-separated exact replay after backend reopen;
- conflicting replay for a substituted handoff commitment;
- one elapsed `TimeWindow` transition and one executor entry;
- terminal replay with no duplicate executor entry; and
- unchanged legacy direct-wake behavior.

The full workspace suite and required GitHub checks remained green.

Missing but non-blocking focused coverage:

- two callers racing through `reinvoke_after_time_window_wait` itself;
- injected failure after transition commit and before operational entry;
- direct composition-level unelapsed-deadline and trusted-time failure cases;
- direct composition-level current-authority or required-context denial; and
- a remaining-wait handoff case if the supervisor later supports more than one
  declared wait.

The first two are mandatory acceptance criteria for a future scheduler
implementation. The latter three should be added when the corresponding
composition surface becomes representable rather than by widening this
single-wait slice.

## 11. Documentation Assessment

The roadmap, implementation plan, and implementation report accurately state
that the helper is explicit and private. They do not claim automatic
reinvocation, scheduling, public configuration, provider execution, nested
harnesses, hosted runtime, or production readiness.

## 12. Blockers

None for the private explicit reinvocation slice.

## 13. Non-Blocking Follow-Ups

1. Add full-composition concurrent-caller coverage before scheduler
   implementation.
2. Add transition-to-entry crash fault injection before scheduler
   implementation.
3. Keep direct wake and handoff-bound reinvocation durably distinguishable.
4. Do not expose the handoff, wake input, or existing-window entry as public
   authority-bearing contracts.
5. Add remaining-wait handoff projection only if multi-wait supervisor output
   becomes an accepted runtime behavior.

## 14. Recommended Next Phase

Plan the smallest private trusted-host scheduling boundary around the accepted
explicit reinvocation composition. Planning must define how a host stores or
receives an inert handoff, waits without busy polling, invokes exactly one
source-specific reinvocation attempt, and preserves a genuine typed wait or
blocked posture without manual conversational restart.

The plan must require the concurrency and transition-to-entry crash proofs
above before implementation. It must not authorize automatic approval,
model-turn creation by Core, provider or sandbox execution, OpenShell, nested
harnesses, public runtime configuration, CLI, SDK, schema, hosted execution,
provider writes, or release changes.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791069179433211000-2`
- approval:
  `approval/run-1791069179433211000-2/review-scope-approved`
- presentation: `presentation/511ba79a940d6947`
- presentation hash:
  `511ba79a940d69470ec86aad2e89115fc3f020d74ccfe967413b1bb6bc446222`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed implementation commit: `b51319d`
- approved boundary: focused review only; no implementation fixes or runtime
  broadening
- phase status: `Completed`
- event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, zero retries, and zero escalations; approval-presentation proof
  enforced with one matching record and event marker
- validation summary: formatting, strict workspace clippy, full workspace
  tests, documentation checks, and diff checks passed
- out-of-kernel work: source inspection, security analysis, review authoring,
  validation, and git actions are performed by the delegated trusted host;
  Workflow OS governs scope and approval but does not perform those actions
