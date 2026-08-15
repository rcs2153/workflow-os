# Authoritative Agent Continuation Vertical Slice Report

## 1. Executive Summary

Workflow OS now has one opt-in local continuation enforcement path for an
immutable-bundle-backed `BeforeSkillInvocation` skill call. The agent-facing
brief is orientation only. Core rehydrates current durable state, derives the
exact continuation binding, creates one durable cursor-bound claim, rereads the
cursor, and enters the existing hook-plus-skill consumer only when the binding
is still current.

This is the first runtime proof of the invariant:

```text
The agent may remember or propose the next step.
Only the kernel may declare and authorize the next material action.
```

## 2. Scope Completed

- Added `GovernedNextAction` with only `invoke_current_step_skill`.
- Added a redaction-safe, serializable `GovernedContinuationBrief` and exact
  `GovernedContinuationBinding`.
- Bound continuation to immutable bundle root, durable last event sequence and
  event ID, current step, invocation idempotency key, action code, and a
  payload-free governance commitment.
- Added a private Core consumer that rehydrates before claim, atomically claims
  first use through `IdempotencyStore`, rereads the cursor, and then invokes one
  concrete consumer.
- Added `LocalExecutor::with_authoritative_continuation()` as an explicit opt-in
  builder. Existing constructors and execution behavior remain unchanged.
- Integrated the claim around the existing side-effect disclosure,
  `BeforeSkillInvocation` hook, and local skill invocation path.

## 3. Scope Explicitly Not Completed

- No public `next-action` CLI or JSON endpoint.
- No typed child-harness start or result-acceptance runtime.
- No interception of shell, editor, git, browser, or provider actions outside
  the integrated executor path.
- No provider mutation broadening, nested harness execution, schemas, hosted
  behavior, or reusable authority object.
- No new workflow events. The durable idempotency claim is the first-slice
  consumption record.
- No independently configured current-authority source was added to the
  executor. This first consumer is limited to local immutable steps whose
  authority, context, evidence, and check posture does not require such a
  source.

## 4. API Summary

Public model vocabulary:

- `GovernedNextAction`
- `GovernedContinuationBinding`
- `GovernedContinuationBrief`

Public opt-in executor configuration:

- `LocalExecutor::with_authoritative_continuation()`

The brief exposes stable posture and identity accessors but has no execution or
authorization method. Projection and consumption remain private to Core.

## 5. Atomicity And Failure Behavior

The claim key commits the exact immutable root, durable cursor, step,
invocation key, governance commitment, and action. The state backend's existing
create-first idempotency contract selects one writer. A duplicate returns
`executor.governed_continuation.claim.already_consumed` before the handler.

Core rereads the cursor after the claim. A changed cursor returns
`executor.governed_continuation.cursor.stale`; the old claim remains burned and
a caller must rehydrate a fresh binding. Terminal, non-running, or unbundled
runs fail closed. Errors are stable and do not echo identifiers or payloads.

## 6. Governance Commitment

The first commitment includes the immutable root, resolved execution context,
accepted proportional-governance fingerprint when present, approval posture,
approval sensitivity, parsed policy effects, bounded capability kinds,
required hook posture, hook identity/status, and SideEffect identities and
lifecycle states.

Unknown capability text is never copied into the commitment input vocabulary;
it is represented only as `unknown`. The commitment is stored and exposed only
as a SHA-256 value.

## 7. Test Coverage

Focused tests prove:

- exact brief projection and serde round trip;
- redaction-safe `Debug`;
- first use succeeds and repeat use fails closed;
- two concurrent consumers produce one durable first writer;
- a cursor change after claim blocks before the consumer and burns the stale
  claim;
- one immutable-bundle `BeforeSkillInvocation` executor path completes once;
- replay of the completed run does not reinvoke the handler; and
- opt-in use against an unbundled run fails before handler invocation.

## 8. Workflow Semantics And Privacy

Ordinary executor behavior is unchanged. The path is opt-in and requires an
immutable run bundle. It does not create provider calls, files, CLI output, or
new runtime configuration. The brief and errors exclude raw prompts, source
contents, command output, provider payloads, credentials, environment values,
paths, and model reasoning.

## 9. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1786770878257514000-2`
- approval ID:
  `approval/run-1786770878257514000-2/implementation-approved`
- presentation ID: `presentation/6406c367404e4154`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events, including one approval request and grant, six
  scheduled steps, six successful skill invocations, no retries, and no
  escalations
- approval presentation enforcement: `proof_enforced`, with one matching
  durable presentation record and an approval-event proof marker
- out-of-kernel work: code inspection, edits, tests, documentation, and git
  operations remained external executor work governed procedurally

## 10. Validation

The following validation passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

The full Rust workspace includes the focused continuation model and
local-executor tests. The integration gate includes the GitHub, Jira, and CI
read-only adapter contracts and examples.

## 11. Remaining Limitations

- Crash recovery after a first-write claim but before consumer completion is
  intentionally fail closed and requires a separately designed recovery
  posture.
- The idempotency claim is durable but no continuation-specific event or report
  projection exists yet.
- The first slice does not inject the registered current-authority source into
  arbitrary skill paths. Any step requiring independent authority or governed
  context must remain blocked until a source-backed consumer is implemented.
- External agent actions outside the integrated executor path remain
  procedurally governed rather than mechanically intercepted.

## 12. Recommended Next Phase

Add the read-only `next-action` preview and continuation outcome projection only
after this implementation passes focused maintainer/security review. Typed
child-handoff runtime behavior should remain a separate reviewed phase before
nested harness execution.
