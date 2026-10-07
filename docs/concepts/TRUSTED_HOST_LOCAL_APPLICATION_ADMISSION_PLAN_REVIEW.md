# Trusted-Host Local Application Admission Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

The plan substantially improves the runtime boundary. It selects SQLite,
restricts the first handler to a closed no-write profile, keeps executor and
`SkillInput` derivation in Core, and preserves the opaque prepared-pair
handoff. It does not yet satisfy the prerequisite established by the prior
blocker review because two load-bearing ownership decisions remain open:

1. the plan does not select the actual foreground process entry; and
2. it does not identify the production provenance of the complete registered
   current-authority inventories required for operational-window opening.

Implementation must not begin until both are resolved in the plan.

## 2. Review Scope

This review assessed:

- the selected foreground owner and SQLite lifecycle;
- the proposed Core admission transition;
- project, workflow, run, policy, approval, and immutable-input semantics;
- closed-profile handler, executor, and `SkillInput` provenance;
- registered current-authority and required-context provenance;
- fresh and existing operation posture;
- the cross-crate opaque handoff;
- failure, privacy, workflow, cancellation, and replay behavior;
- end-to-end proof requirements; and
- scope discipline.

It did not implement code or authorize a runtime surface.

## 3. Foreground Owner Assessment

Selecting `workflow-local-host` rather than the filesystem CLI, PostgreSQL
hosted worker, or Core lifecycle is correct. Keeping the owner unpublished and
foreground-only is also appropriate for the first slice.

The plan is nevertheless incomplete. Section 5 leaves the concrete entry as
either a binary target or a package-owned executable wrapper and delegates the
choice to review. A wrapper with no process caller is the exact class of
unreachable production abstraction rejected in the preparation-source phase.
The implementation plan, not its review or implementation, must select:

- the exact package target and entry function;
- how the process is invoked in the production-shaped proof;
- who owns process exit and cancellation custody; and
- why the target is operationally real despite remaining unpublished.

This is a blocker.

## 4. SQLite Lifecycle Assessment

One `SqliteStateBackend` retained by the foreground owner is the correct state
boundary. The plan properly rejects filesystem-to-SQLite translation and
PostgreSQL reuse.

The fix-forward plan should add:

- database creation/open ownership;
- schema compatibility and health checks before admission;
- behavior for an unavailable, unhealthy, or incompatible database;
- explicit close/drop posture; and
- proof that the same backend instance is used from admission through
  consumed execution.

These details are necessary but can be resolved together with the foreground
entry blocker.

## 5. Core Admission Transition Assessment

The proposed single Core transition is directionally correct. It keeps
project loading, run identity, immutable bindings, policy, approval, step
selection, executor construction, input construction, operational posture,
and preparation on the authoritative side of the boundary.

The selection-input distinction is also sound: project root, workflow ID,
optional run ID, actor, correlation ID, profile, and trusted time may identify
what the application requests, but they cannot prove eligibility.

The corrected plan should make the transition outcome explicit. It must
distinguish at least:

- prepared operation;
- waiting for approval or another supported condition;
- terminal replay;
- blocked or denied; and
- bounded failure.

Returning every non-prepared posture as `TrustedHostLocalApplicationFailure`
would collapse lawful workflow state into error handling.

## 6. Current-Authority Provenance Blocker

Operational opening requires a `RegisteredInMemoryCurrentAuthoritySource`.
That source is built from complete inventories of capability grants,
capability availability records, and governed-context references, plus source
identity, configuration commitment, freshness, sensitivity, and time bounds.

The plan says Core will resolve registered current authority in the admission
call, but it does not identify where those inventories originate or how Core
proves completeness and freshness. The existing project spec, closed local-
check profile, and SQLite run snapshot do not automatically supply that
complete source.

Implementation would therefore have to choose among three unacceptable paths:

- fabricate sufficient authority for the docs-check profile;
- accept caller-authored authority inventory; or
- add another test-only source.

The blocker fix must select one existing accepted authority-registration path
and show how the foreground application causes it to be populated from
authoritative local inputs. If no production-shaped source exists, the next
prerequisite is current-authority registration/admission planning, not local-
host implementation.

## 7. Handler And Input Provenance Assessment

Restricting the first slice to the existing explicit docs-check profile is a
good choice. It is closed, deterministic, no-write, and already binds a fixed
command contract.

The plan correctly keeps `LocalSkillAttemptExecutor` private and derives
`SkillInput` from the validated immutable workflow step. It correctly rejects
caller-supplied command arguments, arbitrary JSON, environment values,
payloads, and mock handlers.

The blocker fix should name the existing resolver method and define ownership
of the resolved profile and handler across the borrowed prepared-session
lifetime. This is required for implementability but is not independently a
security blocker if the ownership model remains entirely inside Core.

## 8. Fresh And Existing Posture Assessment

The fresh/existing separation is conservative and appropriate:

- fresh admission may establish one durable run before preparing a window;
- approval and policy stops remain pre-preparation;
- existing admission is explicit rather than discovered;
- terminal replay performs no work; and
- ambiguous, stale, or substituted state fails closed.

The corrected plan should state how a waiting run later re-enters admission
after a separately governed approval decision. Admission itself must not
grant approval or wait interactively.

## 9. Cross-Crate Handoff Assessment

Returning only `TrustedHostLocalApplicationPreparedSession` is the correct
cross-crate authority boundary. The plan preserves its one-shot,
non-cloneable, non-serde, lifetime-bound, and redacted properties and requires
immediate conversion to `LocalHostPreparedOperation`.

No new standalone source is needed if the accepted Core admission transition
can return the pair directly. Adding a source remains unjustified unless the
blocker fix proves a concrete ownership need inside the selected path.

## 10. Failure, Privacy, And Workflow Semantics Assessment

The plan correctly preserves:

- approval waits as waits rather than implicit grants;
- denial as denial;
- terminal replay without duplicate execution;
- at-most-once dispatch;
- application return as distinct from workflow completion;
- payload-free application errors and Debug output;
- zero-dispatch failure and drop posture; and
- no raw commands, paths, identifiers, payloads, outputs, or credentials in
  application-visible diagnostics.

The corrected plan must keep lawful non-error admission outcomes separate from
fixed application failures. Otherwise an operator could not distinguish a
healthy approval wait from invalid state.

## 11. Test Plan Assessment

The planned proof is strong. It requires the actual foreground path,
production Core admission, fresh and existing runs, approval wait, policy
denial, terminal replay, substitution rejection, cancellation, drift,
at-most-once dispatch, privacy, and state-model isolation.

Add direct tests for:

- current-authority inventory completeness and freshness;
- missing, stale, incomplete, and conflicting authority source posture;
- backend open/health/schema failure before admission;
- lawful waiting and terminal admission outcomes remaining distinct from
  failures; and
- exact process entry invocation rather than only an internal function call.

## 12. Scope Verification

The planning phase stayed within documentation-only scope. It did not add
Rust, executables, sources, commands, runtime configuration, discovery,
scheduling, provider access, mutations, hosted behavior, OpenShell, nested
harnesses, schemas, examples, or release changes.

## 13. Blockers

1. Select the exact real foreground process entry and production-shaped
   invocation. Do not leave binary-versus-wrapper ownership open.
2. Identify the accepted authoritative source and registration path for the
   complete current-authority inventories required by operational opening.
3. Define an admission outcome that preserves prepared, waiting, terminal,
   blocked/denied, and failure posture without collapsing workflow state.

## 14. Non-Blocking Follow-Ups

- Name the exact closed-profile resolver and borrowed ownership strategy.
- State SQLite health, schema, and close/drop behavior.
- Clarify how a separately approved waiting run explicitly re-enters
  admission.
- Preserve an unpublished, non-release posture until implementation review.

## 15. Recommended Next Phase

Perform a documentation-only planning blocker fix. Update the admission plan
to select the process entry, current-authority source, and admission result
shape. Then re-review the corrected plan before Rust implementation.

Do not add an admission function, foreground executable, preparation source,
test authority factory, CLI path, hosted path, discovery loop, provider
behavior, or broader handler family yet.

## 16. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 17. Governed Review Record

- workflow: `dg/review`
- run: `run-1791390721103079000-2`
- approval: `approval/run-1791390721103079000-2/review-scope-approved`
- presentation: `presentation/f9a7f6d71a609216`
- presentation hash:
  `f9a7f6d71a609216473a109641605090f30feade2a249d476030acdcaccb5c78`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused documentation-only maintainer/security review
- phase status: completed
- event summary: 39 ordered events, one approval, zero retries, and zero
  escalations; approval-presentation proof was enforced with one presentation
  record
- validation summary: documentation and diff checks passed; Rust checks were
  not run because the reviewed phase changed documentation only
- out-of-kernel work: source inspection, review authoring, validation commands,
  and later git or pull-request work
- report posture: no runtime WorkReport artifact was generated or persisted
