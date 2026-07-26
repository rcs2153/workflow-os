# Authoritative Proportional Governance Routing Review

## 1. Executive Verdict

**Phase accepted with non-blocking composition follow-ups.**

The four explicit authoritative local `DocsCheck` routes are monotonic,
fail-closed, source-bound, and compatible:

- `Proceed + Quiet`;
- `Proceed + Visible`;
- `RequireApproval + Visible`; and
- `Denied + Visible`.

Proceed next to a narrow authoritative dispatcher/composition plan. The
derived assessment should select among these accepted routes without allowing
a caller to choose a weaker outcome.

## 2. Scope Verification

The routing phases stayed within their approved explicit local executor scope.
They did not add:

- default or automatic proportional routing;
- CLI, UI, workflow-schema, or example exposure;
- automatic approval or a second approval system;
- providers, OpenShell, sandbox execution, or credentials;
- SideEffect execution or new provider mutation families;
- report artifacts or automatic report generation;
- hosted behavior, enterprise administration, reasoning lineage, or release
  changes.

The denial phase added no broad WorkReport, event, failure, or approval
redesign.

## 3. Authoritative Input Assessment

All four routes reuse the same preparation boundary:

1. validate a fresh explicit execution request;
2. build and validate the immutable run bundle;
3. create-only claim the bundle;
4. execute the canonical `DocsCheck`;
5. derive complete governance facts from stored bundle declarations and
   current typed runtime facts;
6. construct a source-bound aggregate assessment; and
7. persist or validate the exact assessment binding.

No route accepts a caller-selected governance disposition, detached
assessment projection, check output, or public fingerprint as authority.

The exact route checks require:

- complete assessment posture;
- the expected execution and disclosure pair; and
- authoritative source binding.

## 4. Routing Matrix Assessment

### Quiet Proceed

The accepted quiet route persists the exact assessment and continues through
ordinary execution. It creates no visible disclosure or approval.

### Visible Proceed

The accepted visible route requires one explicit injected disclosure surface.
Core constructs the exact delivery request, accepts only a bounded timestamp
from the surface, constructs and validates the receipt, and continues only
after acceptance.

### Approval Required

The accepted approval route constructs an aggregate approval subject inside
Core, pauses before step scheduling, and reuses the existing durable approval
and proof-enforced presentation lifecycle. Grant and denial both require fresh
exact reassessment. An aggregate grant is not step or SideEffect authority.

### Denied

The denied route persists the exact assessment, starts the ordinary run
lifecycle, and fails with a distinct stable code and `PolicyDenied` failure
class before step scheduling. It creates no approval or skill activity.

## 5. Monotonicity Assessment

The routing model preserves monotonic escalation:

- a denied or approval-required execution disposition cannot be consumed by a
  proceed route;
- blocking dispositions require visible disclosure in the normalized model;
- caller visibility hints cannot suppress the derived visible obligation;
- approval grant requires current facts to reproduce the durable assessment;
- changed retry or resume facts fail or escalate rather than downgrade; and
- a route mismatch fails closed before ordinary run events.

The executor does not repair invalid `RequireApproval + Quiet` or
`Denied + Quiet` states.

## 6. Event And Ordering Assessment

The ordinary event boundary is deterministic:

- quiet and visible proceed append the persisted assessment binding and
  continue into execution;
- approval appends the binding, starts the run, requests approval, and pauses
  before `StepScheduled`;
- denial appends the binding, starts the run, and emits `RunFailed` before
  `StepScheduled`.

Visible disclosure acceptance occurs before run events and skill execution.
Approval presentation proof occurs before decision mutation. Denial requires
no synthetic approval or disclosure-success event.

Existing event vocabulary is sufficient. The assessment-binding event carries
the exact posture; ordinary approval and terminal events carry lifecycle
truth.

## 7. Crash And Recovery Assessment

The routes retain one documented limitation: immutable bundle, assessment, or
visible delivery residue can exist if a later persistence or event append
fails. This residue is bounded and does not grant execution authority.

The visible route's receipt remains in memory, so a delivery accepted before a
later local failure is not yet durably recoverable. The explicit consumers are
fresh-run-only. Retry and resume require separate freshness and recovery work.

These are non-blocking for the bounded local slices because no stale residue
can silently authorize execution.

## 8. Privacy And Error Assessment

The route APIs use validated bounded models and stable errors. `Debug`, serde,
events, and failures do not copy:

- raw source or spec contents;
- local-check output;
- commands, environment values, or filesystem paths;
- assessment reason payloads or fingerprints in user errors;
- approval or disclosure prose;
- provider payloads; or
- credentials, authorization headers, private keys, or tokens.

Unknown or inconsistent wire values fail closed without echoing caller input.

## 9. Compatibility Assessment

The routes are additive. Existing `execute(...)`, step approvals, hooks,
reports, artifacts, providers, SideEffects, state rehydration, and CLI
behavior remain unchanged.

The full workspace suite passes, including executor, approval, immutable
bundle, local-check, proportional-governance, provider, report, event, and CLI
tests.

## 10. Product Boundary Assessment

The accepted routes prove real executor enforcement, but the current API is
still integration-oriented:

- callers select among separate exact-route functions;
- the selected function recomputes the authoritative assessment and rejects a
  mismatch;
- therefore a caller cannot downgrade governance by selecting a weaker route;
  but
- the product does not yet expose one boundary where the derived assessment
  itself selects the route.

This is not a correctness blocker. It is the next runtime-composition gap.

Visible disclosure remains an obligation axis, not a user-selected execution
mode. A UI may show quiet evidence live without changing governance. A
required visible disposition means a bounded delivery obligation exists and
must not be silently treated as ordinary quiet capture.

## 11. Test Quality Assessment

Coverage proves:

- each accepted route's positive behavior;
- exact route mismatch rejection;
- complete source-bound assessment requirements;
- visible delivery before execution and fail-closed delivery errors;
- approval pause before steps and proof-enforced reassessment;
- denied terminal failure before steps;
- deterministic event ordering;
- no duplicate execution on accepted replay paths;
- stable non-leaking error and `Debug` behavior; and
- broad compatibility through the workspace suite.

One future combined regression should exercise a single dispatcher across all
four outcomes once that dispatcher exists. Existing route-local tests should
remain as defense in depth.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

1. Add one authoritative dispatcher/composition boundary that routes the
   same-call assessment to the accepted implementation without caller mode
   selection.
2. Preserve exact per-route checks so dispatcher defects still fail closed.
3. Decide whether visible delivery receipts need durable recovery before
   retry, resume, hosted operation, or asynchronous delivery.
4. Retain the approval-route follow-ups for generic `ApprovalStore` subject
   validation and aggregate-gate-plus-later-step-approval regression coverage.
5. Add concise operator projection only after the dispatcher preserves policy
   authority outside UI code.

## 14. User Feedback Reconciliation

External evaluation says the kernel is credible and honest, while ceremony is
the remaining product constraint. The four-route matrix is the runtime
foundation for reducing that ceremony safely:

- low-risk work can execute through quiet evidence capture;
- material but non-blocking posture can disclose visibly;
- higher-risk work can pause for proof-enforced approval; and
- prohibited work fails before execution.

The user concern that visible disclosure may feel like a UI mode is partly
correct at the presentation layer but not at the governance layer. A local UI
may display quiet decisions continuously. The distinct `Visible` obligation
is still necessary when policy requires a delivery that must not be silently
dropped.

The next product move should compose the routes, then improve concise
operator UX. It should not add OpenShell, another provider mutation, or a new
primitive family first.

## 15. Validation

The review relies on the implementation phase validation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

Documentation checks and diff hygiene are rerun after this review.

## 16. Governed Review Record

- workflow: `dg/review`
- run: `run-1785053542584699000-2`
- approval:
  `approval/run-1785053542584699000-2/review-scope-approved`
- presentation: `presentation/e09cb749b7bad733`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: source inspection, review judgment, documentation,
  validation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run checks, create a WorkReport artifact, or perform git actions

## 17. Recommended Next Phase

Plan one narrow authoritative routing dispatcher/composition boundary.

The dispatcher should:

- consume the same complete source-bound assessment;
- select quiet, visible, approval-required, or denied behavior from the
  derived assessment;
- accept only route-specific dependencies that are actually required;
- preserve each existing exact route check;
- return a typed outcome without inventing execution success; and
- remain explicit, local, fresh-run-only, and free of CLI/schema/provider
  expansion.

After that boundary is accepted, proceed to concise operator UX and quiet
success integration. Optional execution providers such as OpenShell remain
later consumers of the accepted authority and routing boundary.
