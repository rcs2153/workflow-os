# Trusted-Host Local Application Admission Plan

Status: planning blocker fix complete; focused blocker-fix review required
before implementation. The original focused review found that the process
entry, current-authority inventory provenance, and admission result shape were
not sufficiently specified.

## 1. Executive Summary

Workflow OS can privately prepare one current, one-shot trusted-host operation,
and the unpublished `workflow-local-host` package can synchronously consume
that operation. The missing boundary is now precisely identified: no
foreground application admits a validated local run into SQLite and asks Core
to derive the exact current operation that local host may consume.

The first implementation should add one unpublished foreground binary target
named `workflow-local-host` to the existing package and one Core-owned
admission transition. The
application owns one `SqliteStateBackend` for its foreground lifetime and
requests one explicit workflow plus one closed local-check profile. Core loads
and validates the project, creates or rehydrates the immutable run, evaluates
the existing policy and approval state, selects the exact runnable step,
derives the validated `SkillInput` and `LocalSkillAttemptExecutor`, prepares
the existing opaque session pair, and returns that pair without exposing its
private authority inputs. Local host immediately consumes it through
`LocalHostPreparedOperation` and returns the existing bounded outcome.

For this exact no-write slice, Core may register a current-authority source
with complete empty inventories only after proving that the immutable
required-context contract and selected closed profile require no capability,
availability, or governed-context facts. The source grants nothing. Any
non-empty requirement, incomplete contract, stale observation, or conflicting
fact blocks admission.

The first slice is deliberately narrow. It supports one explicit no-write
local-check profile and synchronous foreground execution. It does not add the
path to the released `workflow-os` CLI, discover runs or commands, poll queues,
schedule arbitrary work, expose public runtime configuration, mutate a
provider, or claim hosted parity.

## 2. Goals

- Establish one real admission-to-operation path owned by a foreground local
  application.
- Keep Core authoritative for project validation, run identity, immutable
  inputs, policy, approvals, current authority, step selection, executor and
  input commitments, preparation, and consumed execution.
- Use SQLite directly for the complete admitted-run lifecycle without
  translating filesystem or PostgreSQL state.
- Execute one explicit, closed, no-write local-check profile through the
  existing handler and trusted-host attempt adapter.
- Preserve one-shot ownership, same-call time-of-use validation, bounded
  failures, and payload-free application outcomes.
- Prove the production constructors and ownership path end to end.
- Make every unsupported trigger, handler, wait, and state posture fail closed.

## 3. Non-Goals

This plan does not authorize:

- implementation during this planning phase;
- adoption by the released `workflow-os` CLI, SDKs, or hosted worker;
- a stable or generally supported operator interface;
- workflow, policy, handler, or authority discovery;
- directory scanning, queue polling, background daemons, detached execution,
  worker pools, or automatic startup replay;
- arbitrary commands, ambient tools, live adapters, provider access, provider
  mutation, or broader write behavior;
- caller-authored current authority, locators, immutable bindings, executor
  commitments, or validated skill input;
- automatic approvals, delegated-authority broadening, or approval bypass;
- general retry policy, parallelism, branching, nested harnesses, OpenShell,
  or distributed scheduling;
- state-model translation, hosted parity, schemas, examples, or release
  posture changes; or
- process-loss recovery, active-attempt interruption, or a production service
  availability claim.

## 4. Repository Findings

### 4.1 Generic executor compatibility is not admission

`LocalExecutor` is generic over `StateBackend`, and `SqliteStateBackend`
implements that contract. This makes SQLite mechanically usable by the
established executor, but it does not provide the trusted-host admission
boundary. The current executor invokes handlers directly and does not yield
the private operational-window input, locator, executor adapter, and
`SkillInput` required by the trusted-host supervisor.

### 4.2 The released CLI is the wrong owner

`workflow-cli` constructs `LocalStateBackend` for ordinary execution. Its
SQLite behavior is limited to explicit migration and health operations.
Changing an existing command to SQLite would silently replace its state model;
adding the trusted-host path there now would create a public product contract
before the application boundary is reviewed.

### 4.3 The hosted worker is the wrong owner

Hosted execution uses PostgreSQL claims, leases, and fencing. A SQLite
admission path would bypass those contracts rather than prove hosted parity.

### 4.4 Core has the required authority but not the process lifetime

Core owns run validation, immutable bundle binding, registered current
authority, operational-window opening, dispatch reservation, supervisor
execution, continuation, and SQLite state. It does not own foreground process
lifetime or application cancellation custody.

### 4.5 Local host has the process shape but not admission

`workflow-local-host` owns a synchronous one-shot operation and scoped
cooperative cancellation. It cannot construct the prepared pair, open the
backend, load a project, choose a step, resolve a handler, or establish current
authority. Its current library-only shape therefore remains a consumer, not an
admission owner.

## 5. Selected Foreground Owner

The first owner is an unpublished binary target named `workflow-local-host` in
the existing package. It is invoked directly from that package in the
production-shaped integration proof. It is not added to the released
`workflow-os` command, installed as a product binary, or documented as a
supported user CLI.

The application entry owns:

- opening and retaining one `SqliteStateBackend`;
- retaining one resolved explicit local-check profile and its handler for the
  complete operation lifetime;
- invoking one Core admission request;
- immediately converting the returned opaque pair into
  `LocalHostPreparedOperation`;
- retaining cancellation custody while `run` is active; and
- returning one bounded local-application result before process exit.

The binary owns `main`, process exit, the backend lifetime, and cancellation
custody. Its parsing layer may delegate to a package-private process-facing
function, but that function is not accepted as the owner by itself. The end-
to-end proof must invoke the binary target rather than call only the function.

## 6. Explicit Application Input

The foreground entry may accept only selection and lifecycle input, not
authority:

- project root;
- SQLite database path;
- workflow ID;
- optional idempotent run ID;
- requesting actor and correlation ID;
- one enum-valued closed local-check profile;
- bounded trusted timestamps required by the existing opening contract; and
- optional cooperative cancellation supplied by the owning process.

These values identify what the operator is asking Core to evaluate. They do
not prove that a run, step, approval, capability, or handler is eligible.

The target's first interface is explicitly unstable and package-local. Its
arguments exist to prove application ownership, not to create a supported
general CLI. Unknown flags, unsupported profiles, missing values, and
additional workflow topology fail closed with fixed payload-free exit
posture.

Before admission, the binary opens or creates exactly one database through
`SqliteStateBackend::open`, validates the adapter schema and health check, and
retains that instance until the operation and cancellation custody are
dropped. An unavailable, unhealthy, or incompatible database returns a fixed
failure before project admission. The binary performs no migration,
translation, repair, or fallback to another backend.

The first slice should select the existing docs-check profile because it is
closed, deterministic, no-write, and already represented by a validated
command contract and `SkillHandler`. If review finds that its process contract
cannot be retained safely at this boundary, implementation must stop rather
than substitute an arbitrary or mock handler.

## 7. Core Admission Transition

Add one Core-owned admission function behind the existing unstable trusted-
host feature. Its conceptual shape is:

```text
admit_trusted_host_local_application_operation(request, backend, profile)
  -> Result<TrustedHostLocalApplicationPreparedSession,
            TrustedHostLocalApplicationFailure>
```

The exact Rust API should follow repository conventions. It must not expose
the private preparation input or accept preassembled authority-bearing facts.

In one bounded call, Core must:

1. load and validate the selected project and workflow;
2. reject unsupported workflow topology or non-selected skill kinds;
3. create a new immutable run or rehydrate the exact supplied run ID;
4. preserve idempotent replay without re-executing terminal work;
5. evaluate the existing policy and approval posture;
6. stop with a bounded non-runnable outcome when approval or evidence remains
   unsatisfied;
7. identify exactly one currently runnable step;
8. bind workflow, run, step, actor, immutable bundle, and snapshot cursor;
9. derive the exact handler from the resolved closed profile;
10. construct `SkillInput` from the validated immutable step definition;
11. construct `LocalSkillAttemptExecutor` and its deterministic binding
    commitment inside Core;
12. resolve registered current authority and required context in the same
    call;
13. derive fresh-opening or existing-window posture from current SQLite state;
14. construct all operation, receipt, attempt, persistence, and trusted-time
    inputs inside Core; and
15. call the existing preparation function and return only its opaque pair.

No intermediate locator, current-authority source, opening input, executor,
skill input, capability, or preparation source crosses to the application.

### 7.1 Current-authority source for the first slice

There is no production registration call for
`RegisteredInMemoryCurrentAuthoritySource` today. The first implementation
must add one private Core constructor dedicated to the selected no-authority
profile. It may register complete empty inventories only when all of the
following are proven from current immutable inputs:

- the selected required-context contract contains zero obligations;
- the selected closed profile requires zero capability grants;
- the selected closed profile requires zero capability-availability facts;
- the selected closed profile requests zero governed-context references;
- independent policy, approval, evidence, and check requirements have already
  been evaluated by their owning boundaries; and
- observed time, validity, source generation, sensitivity, configuration
  commitment, and exact execution binding are current and deterministic.

The source configuration commitment must bind the immutable run bundle,
required-context contract, closed-profile command contract, exact execution
binding, and explicit empty-inventory posture. Its validity is bounded to the
same admission call and prepared-session lifetime.

This source is not a synthetic grant. It is an authoritative statement that
the exact admitted no-write operation requires no current authority facts. If
any requirement is non-empty, missing, unknown, stale, optional-but-policy-
significant, or conflicting, registration or resolution returns blocked. The
application cannot supply, amend, or override the inventories.

Additional profiles that require grants, tool availability, or governed
context need a separately reviewed durable/current authority source and are
not authorized by this plan.

## 8. Fresh And Existing Admission

### 8.1 Fresh run

For a new run, Core validates the project, writes the established run-start
history and immutable binding, evaluates policy and approval requirements, and
leaves the run in the exact eligible running posture before preparing the
first operational window. If approval is required, admission stops before
preparation and returns bounded waiting posture; the application cannot grant
or synthesize approval.

### 8.2 Existing run

For an explicit existing run ID, Core rehydrates the run and derives behavior
from durable current state:

- terminal runs return terminal posture without handler invocation;
- waiting runs return their bounded wait posture;
- exactly one eligible existing operational window may resume;
- stale, ambiguous, mismatched, or duplicated windows fail closed; and
- a caller-supplied workflow, actor, profile, or invocation mismatch cannot
  replace the durable binding.

The first slice does not scan for resumable runs. Existing-run use is explicit
and idempotent. A waiting run may re-enter admission only after a separately
governed approval or evidence operation has changed durable state and the
foreground binary is invoked again with the same run ID. Admission does not
wait interactively, grant approval, or poll for the change.

## 9. Executor And Skill-Input Provenance

The application supplies the closed profile choice, not a handler object or
command. Core resolves the profile through the existing explicit-profile
resolver, verifies that the workflow step's skill ID and version match the
profile, and retains the resolved handler for the operation lifetime.

Core derives `SkillInput` only from the validated immutable workflow step. It
must not accept arbitrary JSON, environment values, command arguments, source
contents, or copied event payloads from the application.

`LocalSkillAttemptExecutor` remains private. Its binding commitment must cover
the selected profile's canonical command contract and the validated skill
identity. The same commitment participates in preparation and consumed
execution revalidation.

## 10. Cross-Crate Handoff

The only authority-bearing value crossing from Core to local host is the
existing `TrustedHostLocalApplicationPreparedSession`. The foreground owner
must immediately consume it into `LocalHostPreparedOperation`.

The application may retain the corresponding redacted cancellation control.
It must not serialize, cache, clone, persist, or reconstruct the prepared
pair. Dropping the pair or operation remains zero-write and makes no completion
claim.

Core remains responsible for all workflow and execution truth. Local host
does not reinterpret `AwaitCondition`, `Blocked`, `Terminal`, cancellation,
or wake-budget outcomes as a different workflow status.

### 10.1 Admission result

Core returns `Result<TrustedHostLocalApplicationAdmissionOutcome,
TrustedHostLocalApplicationFailure>`, where the bounded outcome has exactly
these first-slice postures:

- `Prepared(TrustedHostLocalApplicationPreparedSession)`;
- `Waiting(TrustedHostLocalApplicationWaitReason)`;
- `TerminalReplay`;
- `Blocked(TrustedHostLocalApplicationBlockReason)`; and
- `Denied`.

Wait and block reasons are closed payload-free enums. They may distinguish
approval, evidence/check, unsupported wait, incomplete authority, and
ambiguous current state only where the existing workflow model already makes
that distinction. They expose no identifiers, policy text, diagnostics, or
payloads.

Only `Prepared` crosses into `LocalHostPreparedOperation`. `Waiting`,
`TerminalReplay`, `Blocked`, and `Denied` are lawful workflow observations and
must not enter the error channel. Parse, validation, security, unsupported,
invalid-state, and internal failures remain the fixed application failures.

## 11. Failure And Privacy Boundary

- Project, workflow, policy, approval, authority, state, and handler failures
  map to stable bounded application failure or stop vocabulary.
- Application-visible Debug, Display, and errors contain no project paths,
  identifiers, policy text, commands, arguments, environment values, payloads,
  outputs, credentials, or Core diagnostics.
- Failed validation and failed admission cannot invoke a handler.
- Failure before preparation may write only the established durable run and
  governance events justified by the admitted request; it must not write an
  operational window or dispatch reservation.
- Failed preparation, canceled-before-entry, and dropped operation retain
  their existing zero-dispatch guarantees.
- Handler output remains governed by the existing bounded result path and is
  not printed by the application entry.

## 12. Workflow Semantics

The foreground application must not change existing workflow semantics:

- approval-required remains a wait, not a failure or implicit grant;
- policy denial remains denial;
- missing or mismatched handlers fail closed;
- terminal replay does not duplicate work;
- executor failure is recorded through existing outcome semantics;
- an operational wait remains non-terminal; and
- application return is not itself workflow completion.

The application may exit after returning bounded posture. Persistent
background continuation and process restart are later work.

## 13. Test Plan

The implementation phase must add production-shaped tests that prove:

1. one fresh eligible docs-check run reaches Core admission, preparation,
   local-host consumption, exactly one handler admission, and bounded return;
2. one explicit existing eligible window resumes through the same path;
3. terminal replay invokes no handler and duplicates no events;
4. required approval stops before preparation and cannot be bypassed;
5. policy denial stops before preparation;
6. unsupported workflow topology and non-docs-check skills fail closed;
7. profile/skill, actor, workflow, immutable-bundle, and run substitution fail
   closed;
8. ambiguous runnable steps or windows fail closed;
9. dropped admission/prepared operation performs no dispatch write;
10. pre-entry cancellation admits no executor attempt;
11. state drift between preparation and consumption fails closed;
12. executor admission occurs at most once across idempotent replay;
13. no prepared pair, locator, executor, handler, or skill input is reusable or
    serializable;
14. Debug and errors remain payload-free;
15. no filesystem state bridge, PostgreSQL path, provider call, or mock handler
    is used;
16. the exact empty current-authority inventories are accepted only for an
    obligation-free contract, while missing, stale, incomplete, conflicting,
    or non-empty requirements block admission;
17. backend open, health, and schema failure stop before admission; and
18. package-specific and workspace-wide tests preserve current CLI and hosted
    behavior.

The end-to-end test must invoke the actual foreground entry and production
Core admission function. A test-only issuer or direct private preparation call
does not satisfy acceptance.

## 14. Proposed Implementation Sequence

1. Focused maintainer/security review of this plan.
2. Add the private Core admission request/result and deterministic admission
   function for the one closed profile.
3. Add the unpublished foreground local-host entry and SQLite lifecycle.
4. Wire the opaque pair directly into `LocalHostPreparedOperation`.
5. Add fresh, existing, approval-wait, denial, cancellation, drift, replay,
   privacy, and end-to-end tests.
6. Run full Rust, docs, integration, and repository validation.
7. Perform a focused implementation/security review before any CLI adoption,
   additional profile, scheduling, provider, or write-capable expansion.

The implementation should be one vertical slice. Do not ship a Core admission
function without its foreground caller or a caller with only a test issuer.

## 15. Open Questions For Review

- Which existing durable events should represent a request that lawfully
  stops at approval before operation preparation?
- Can the docs-check process contract be retained for the full borrowed
  session lifetime without widening public handler APIs?
- Which bounded outcome best distinguishes terminal replay from an operation
  that reached terminal state during this call?
- Should a fresh run ID be generated only inside Core, while existing-run
  selection remains an explicit optional hint?
- What minimal operational metric can be emitted without exposing bound
  identities or creating a second audit truth?

## 16. Validation For This Planning Phase

- `npm run check:docs`
- `git diff --check`

No Rust implementation or Rust validation is authorized in this phase.

## 17. Final Recommendation

Proceed to a focused maintainer/security review. If accepted, implement one
unpublished SQLite-backed foreground admission path for the explicit docs-
check profile. Keep Core as the sole authority for admission and return only
the existing opaque prepared pair to local host.

Do not add the path to the released CLI, broaden handler families, add
discovery or scheduling, expose authority-bearing inputs, mutate providers,
or claim production local-host support.
