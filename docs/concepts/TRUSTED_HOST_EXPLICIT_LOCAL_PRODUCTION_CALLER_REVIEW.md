# Explicit Local Trusted-Host Production Caller Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups. Proceed to planning one explicit
internal adoption site.**

The implementation is a narrow crate-private production-shaped caller over the
accepted local timer and bounded repeated-scheduling driver. It creates no
authority, performs no run discovery, and adds no automatic scheduling or
public runtime surface.

## 2. Scope Verification

The phase stayed within the corrected implementation plan. It added one
crate-private synchronous caller, one private production identity source, one
direct entropy dependency compatible with the caller's Rust 1.78 requirement,
focused tests, roadmap status, and an implementation report.

It did not add a caller adoption site, public API, CLI, SDK, schema, daemon,
background thread, queue, runtime configuration, provider mutation, OpenShell,
nested harness execution, automatic approval, hosted scheduling, or release
change.

## 3. Caller Boundary Assessment

`run_trusted_host_local_production_caller` owns only private identity material,
the fixed finite wake budget, and delegation to the accepted local timer. The
input contains an already-selected durable locator, exact executor and skill
input, owner-created cancellation receiver, and backend reference.

The caller cannot accept claimed eligibility, approval, authority, wait
satisfaction, workflow disposition, or a user-selected wake budget. Core still
reloads and verifies durable state before every executor entry.

## 4. Identity Construction Assessment

The production source uses one all-or-nothing entropy fill for each typed
identity set. Redispatch obtains 96 bytes partitioned into six independent
128-bit values; a wake obtains 32 bytes partitioned into two values. Fixed
domain prefixes and lowercase hexadecimal keep identifiers distinct and inside
the existing length limit.

Entropy failure returns the stable
`trusted_host_local_production_caller.identity_generation_failed` error. No
partial set is returned, and the caller does not retry a replay conflict.
Identity values remain idempotency material rather than execution authority.

The internal `RefCell` is acceptable for this synchronous, crate-private
boundary. It would need reconsideration before a shared, reentrant, detached,
or multithreaded identity source were introduced.

## 5. Wake Budget And Scheduling Assessment

The caller constructs exactly `TrustedHostRepeatedWakeBudget::new(2)` and
accepts no external budget. It invokes `run_trusted_host_local_timer` once and
returns the existing bounded outcome unchanged.

The integrated two-wake test demonstrates lawful repeated waiting and exactly
two executor entries for a fixture that yields once before reaching terminal
state. Budget exhaustion, unsupported waits, blocking, replay conflict, and
concurrent caller behavior remain covered at the accepted repeated-driver and
timer boundaries rather than reimplemented in the thin caller.

## 6. Cancellation And Restart Assessment

The owner constructs the existing cancellation pair and retains the handle.
Pre-cancellation reaches the production caller, produces the typed canceled
stop, enters no executor, and leaves continuity state unchanged. The lower
timer boundary separately proves cancellation during a blocked wait and
idempotent signaling.

The reopen test reconstructs the caller from a fresh SQLite backend handle,
the durable locator, fresh process-local cancellation, fresh identities, and a
fresh fixed budget. The implementation does not claim automatic restart,
remaining-budget restoration, owner-loss detection, or durable host jobs.

## 7. Authority And Workflow Semantics Assessment

The caller does not mutate workflow semantics. Eligibility, trusted time,
current authority, wait satisfaction, reservation, dispatch, execution, and
terminal state remain inside the previously reviewed Core path.

Host cancellation and budget exhaustion do not become workflow failure or
completion. A returned outcome is bounded operator posture and contains no
resume recommendation or reusable authority.

## 8. Failure, Privacy, And Debug Assessment

Identity generation fails closed with a fixed code and message. Timer, state,
binding, replay, and executor failures pass through without automatic retry or
reinterpretation.

The identity source and caller input have bounded custom `Debug`
implementations. They omit random material, locators, skill input, executor
bindings, paths, commands, payloads, credentials, evidence bodies, and approval
reasons. The new types do not derive serialization.

## 9. Dependency And MSRV Assessment

The direct caller dependency is `getrandom` 0.2.17, whose API supplies the
required operating-system entropy without introducing the Rust 1.85 floor of
the reviewed 0.4 line. This correction is appropriate and the caller does not
widen the repository's declared Rust 1.78 requirement.

The locked workspace still contains a pre-existing transitive `getrandom`
0.4.3 path through `postgres-protocol` / `tokio-postgres` / `rand` 0.10. That
does not originate in this phase, but it means the repository should not infer
workspace-wide Rust 1.78 compatibility solely from the caller's direct
dependency choice. A separate dependency/MSRV audit should reconcile the
declared workspace floor with the full resolved graph.

## 10. Test Quality Assessment

Focused caller tests prove:

- one atomic fill per redispatch or wake identity set;
- independent, bounded, domain-separated identifiers;
- deterministic test-only entropy;
- stable non-leaking entropy failure;
- redaction-safe identity-source Debug output;
- one eligible caller entry;
- canceled caller zero-write behavior;
- backend reopen reconstruction; and
- the exact fixed two-wake path.

The accepted lower layers continue to prove during-wait cancellation,
idempotent cancellation, duplicate identity rejection, competing callers,
budget exhaustion, unsupported waits, stale bindings, corruption, and
aggregate at-most-once executor entry. This is valid compositional coverage for
a thin private caller.

Non-blocking coverage follow-ups:

- add a direct caller-input Debug assertion before any adoption site carries
  more sensitive bindings;
- add a caller-level during-wait cancellation test if the caller gains a
  longer-lived owner; and
- keep real 500 millisecond waits as a small smoke boundary rather than a
  pattern for broader scheduling tests.

## 11. Documentation Assessment

The plan, implementation report, and roadmap accurately describe the caller as
private, synchronous, local, finite, and unadopted. They explicitly deny
automatic scheduling, public exposure, provider mutation, hosted behavior,
OpenShell, nested harnesses, and release readiness.

The MSRV fix-forward notes preserve the original planning finding while
explaining why the direct dependency changed from the reviewed 0.4 line to
0.2.17.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Audit the complete locked dependency graph against the declared Rust 1.78
  workspace floor.
- Add direct caller-input Debug coverage before adoption expands binding
  sensitivity.
- Add caller-level cancellation-during-wait coverage if ownership becomes
  longer-lived or detached.
- Preserve the fixed two-wake budget and non-retrying replay posture.

## 14. Recommended Next Phase

Plan one explicit internal adoption site for this private caller. The plan must
identify an already-authorized local path, preserve synchronous owner control,
define how non-terminal stop posture reaches the owner, and prove that adoption
does not become run discovery or automatic scheduling.

Do not implement a daemon, general scheduler, startup scanning, public runtime
configuration, provider mutation, OpenShell, nested harnesses, automatic
approval, CLI, SDK, schema, hosted scheduling, or release change.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791365405829577000-2`
- approval: `approval/run-1791365405829577000-2/review-scope-approved`
- presentation: `presentation/b91022622e2554ad`
- presentation hash:
  `b91022622e2554ad631b59ccacf5b8cb0d804712c958cd86121f842f52478a7d`
- approval outcome: granted by delegated maintainer through proof enforcement
- reviewed merge commit: `b30ad3d69e03eb79425b0ef7d0e521e2b8671520`
- approved boundary: focused maintainer/security implementation review only
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted record
- validation summary: focused and workspace Rust checks, docs check, dependency
  graph inspection, and diff hygiene
- out-of-kernel work: source and test inspection, review authoring, shell
  validation, and later git and pull-request actions
