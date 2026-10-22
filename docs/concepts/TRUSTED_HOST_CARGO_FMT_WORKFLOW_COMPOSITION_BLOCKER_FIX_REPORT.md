# Trusted-Host Cargo Format Workflow Composition Blocker Fix Report

## 1. Executive Summary

The focused implementation/security review blockers for the private
trusted-host three-step composition are fixed. Resumed admission and terminal
report composition now validate the complete scheduled-step, invocation,
authorized-dispatch, result, invocation-terminal, successor, and run-terminal
chain for each attempted step.

The fix remains local, private, explicit, and no-write. It does not add another
profile, broaden topology, expose released CLI behavior, enable providers or
writes, add schemas, or change release posture.

## 2. Blockers Fixed

The fix closes the four blocker groups recorded in the implementation review:

1. exactly one matching authorized dispatch is required for every attempted
   step, and unexpected dispatches fail closed;
2. a failed terminal run must end in either a bounded non-passing result or the
   exact pre-result failure shape;
3. pre-result failure is bound to the deterministic invocation request and its
   matching failed invocation; and
4. focused three-step corruption, substitution, artifact, and privacy
   regressions now exercise the hardened boundary directly.

## 3. Implementation Approach

The existing exact sibling implementation was retained. The private
three-step history validator now receives the stored immutable run bundle, the
closed three-profile registry, and the correlation identity needed to
reconstruct each immutable `SkillInput`.

For every scheduled step, validation now proves:

```text
schedule
  < deterministic invocation request
  < exactly one matching authorized dispatch
  < accepted local-check result, when present
  < matching invocation terminal
  < exact successor schedule or run terminal
```

Admission additionally re-resolves the current closed profile and requires its
executor commitment to reproduce the durable dispatch commitment before more
work may execute. Terminal report composition keeps the durable dispatch
executor commitment authoritative because a valid pre-result integrity
failure may intentionally make the current executable identity differ.

## 4. Exact History And Dispatch Boundary

The validator rejects missing, duplicate, substituted, or unrelated dispatch
history. Dispatch identity is checked against the immutable step input and the
stored executor commitment. Result truth is bound to the exact step, profile,
command, immutable bundle, attempt, invocation request, and invocation
terminal.

Both successor boundaries are checked causally. A later step cannot appear
before the preceding successful invocation terminal, and failed or timed-out
work cannot schedule a successor.

## 5. Pre-Result Identity Linkage

The pre-result failure path now requires exactly one deterministic invocation
request, one matching authorized dispatch, one matching failed invocation, and
one terminal `RunFailed` event in order. A substituted invocation ID, missing
failure event, duplicate failure event, or accepted-result mismatch fails
closed. The path still records no fabricated local-check result and does not
infer an unpersisted failure cause.

## 6. Report And Artifact Posture

The v4 report commitment now includes and cites each schedule event ID in
addition to dispatch, result, invocation-terminal, and run-terminal evidence.
This makes the claimed scheduled-step binding directly inspectable.

Create-only artifact persistence remains unchanged. Missing artifact recovery
is exact; a different valid v4 artifact fails closed without overwrite or run
history mutation.

## 7. Test Coverage

Focused foreground coverage now includes:

- dispatch executor-commitment substitution;
- pre-result failed-invocation identity substitution;
- causal corruption at both successor boundaries;
- result/profile substitution;
- conflicting v4 artifact rejection without overwrite;
- schedule-event citations for the three-step report;
- bounded non-leaking rejection errors; and
- the existing success, first/second/third failure, restart, replay, recovery,
  concurrency, and compatibility matrix.

The focused `foreground_admission` suite contains 40 passing tests.

## 8. Privacy And Redaction

Validation errors remain stable and payload-free. The new rejection paths do
not include repository or toolchain paths, command output, source contents,
environment values, credentials, substituted identifiers, or process
transcripts. Reports continue to cite stable references rather than copying
payloads or creating `EvidenceReference` values implicitly.

## 9. Governed Phase Record

- workflow: `dg/blocker`
- run: `run-1792683923758244000-2`
- approval: `approval/run-1792683923758244000-2/fix-approved`
- presentation: `presentation/80ee7f44a53ff199`
- presentation hash:
  `80ee7f44a53ff19967b4328cc2db2bad771dc97f44053da40cdd8891c9319dd5`
- approval outcome: granted under delegated maintainer authority
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations

## 10. Validation

All engineering-standard gates passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- focused foreground admission tests
- `cargo test --workspace`
- `npm run check`
- `npm run check:integrations`
- `npm run check:docs`
- `git diff --check`

The focused foreground suite passed 40 tests. The complete workspace test
suite, TypeScript SDK checks, contract checks, dogfood-helper tests, read-only
integration checks, and documentation checks also passed. Opt-in live provider
and sandbox tests remained skipped by their existing explicit gates.

## 11. Remaining Limitations

The composition is still a private trusted-host application path. It does not
provide an operating-system sandbox, independently attest the toolchain, prove
that a transient restored write never occurred, enable network authority, or
authorize provider mutation. It remains an exact three-step topology rather
than a generic workflow engine extension.

## 12. Recommended Next Phase

Perform a focused maintainer/security review of this blocker fix. Do not expand
profiles, topology, provider authority, write posture, public CLI behavior, or
hosted behavior before the fix is accepted.
