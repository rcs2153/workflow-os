# Authoritative Agent Continuation Preview Report

## 1. Executive Summary

Workflow OS now exposes the current bounded local continuation as a read-only
Core projection and `workflow-os next-action <run-id>` CLI surface. The preview
is orientation only. It neither grants nor consumes authority and cannot invoke
the selected action.

This implements the operator-facing half of the invariant:

```text
The agent may remember or propose the next step.
Only the kernel may declare and authorize the next material action.
```

## 2. Scope Completed

- Added `preview_governed_continuation` for a running immutable-bundle-backed
  local run.
- Reconstructed the exact current scheduled step, skill, referenced policy
  effects, capabilities, approval posture, invocation identity, hook
  requirement, and governance commitment from durable state.
- Added `workflow-os next-action <run-id>` human and `--json` output.
- Marked human output explicitly as `authoritative: false` and
  `consumed: false`.
- Reused `GovernedContinuationBrief`, `GovernedContinuationBinding`, and
  `GovernedNextAction`; no parallel preview model was introduced.

## 3. Scope Explicitly Not Completed

- No handler, command, provider, tool, or child-harness execution.
- No authority grant, durable continuation claim, event append, state change,
  report artifact, or filesystem output.
- No automatic next-action invocation or generic callback.
- No typed child runtime, provider mutation broadening, nested harness runtime,
  workflow schema, hosted behavior, or release change.
- No preview for hook or SideEffect context that the immutable bundle records
  as supplied but does not preserve reconstructably.

## 4. API And CLI Summary

Core exports:

- `preview_governed_continuation(&WorkflowRun, &StoredImmutableRunBundle)`

CLI exposes:

- `workflow-os next-action <run-id>`
- `workflow-os --json next-action <run-id>`

The existing consuming path remains private and independently rehydrates and
claims current state. A serialized preview cannot be passed back as authority.

## 5. Validation Boundary

Projection requires a currently `Running` run, an exact immutable bundle
binding and run identity, one resolvable current scheduled step, one exact
immutable skill definition, and exact referenced policy definitions with valid
effects. Terminal, unbundled, mismatched, unresolved, already-invoked, or
non-reconstructable context fails closed with stable non-leaking errors.

## 6. Read-Only Semantics

The Core function accepts borrowed run and bundle values. It has no state
backend, registry, handler, artifact store, or provider dependency. Tests prove
that projection leaves the durable event list unchanged and invokes no skill
handler. The CLI reads the run and exact bundle only.

## 7. Privacy And Redaction

The preview contains stable IDs, immutable root, event cursor, action code, and
payload-free governance commitment. It contains no source content, prompts,
command output, provider payload, credentials, environment values, paths,
model reasoning, or raw policy text. Existing redaction-safe `Debug` behavior
remains unchanged.

## 8. Test Coverage

Focused tests cover:

- exact projection for a durable running immutable run;
- current step and action vocabulary;
- zero handler calls and unchanged event history;
- terminal-run rejection through the CLI without event mutation;
- command help and no state-directory creation;
- explicit non-authoritative human rendering; and
- bounded JSON rendering without authority or raw-payload fields.

Existing continuation consumption, executor, immutable-run, approval, policy,
state, report, adapter, and runtime tests remain part of workspace validation.

## 9. Dogfood Governance

- workflow: `dg/implement`
- run ID: `run-1786773352456740000-2`
- approval ID:
  `approval/run-1786773352456740000-2/implementation-approved`
- presentation ID: `presentation/e2b9214266250756`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented
- phase status: completed
- event summary: 39 events, including one approval request and grant, six
  scheduled steps, six successful skill invocations, no retries, and no
  escalations
- approval presentation enforcement: `proof_enforced`, with one matching
  durable presentation record and an approval-event proof marker
- out-of-kernel work: inspection, code edits, tests, documentation, and git
  operations remained external executor work governed procedurally

## 10. Validation

The completed phase runs:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

The standard workspace suite kept environment-gated live provider smoke tests
ignored. No live provider call was required or performed for this read-only
local phase.

## 11. Remaining Limitations

- Preview supports only the exact reconstructable local immutable-run posture.
- It does not independently resolve a registered current-authority source or
  consume governed context.
- It does not project a continuation outcome event or report record.
- It cannot govern external shell, editor, git, browser, or provider actions
  outside an integrated Core consumer.
- Typed child launch and result acceptance remain unimplemented.

## 12. Recommended Next Phase

Add a registered-current-authority-backed continuation consumer for one narrow
local operation. Keep provider mutation broadening and nested harness execution
blocked until that source-backed boundary is implemented and focused-review
accepted.
