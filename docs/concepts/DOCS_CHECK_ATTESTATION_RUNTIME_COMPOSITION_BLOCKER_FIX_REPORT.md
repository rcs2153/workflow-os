# DocsCheck Attestation Runtime Composition Blocker Fix Report

## 1. Executive Summary

The immutable attribution blocker found during phase review is fixed.
`DocsCheckAttestationExecutionInput` no longer accepts caller-supplied skill ID
or version. Core resolves the selected step from the stored canonical workflow,
resolves its step-scoped skill reference and canonical skill record, and derives
the execution-binding skill identity before clock sampling or process launch.

## 2. Blocker Fixed

The original implementation compared workflow and run identity with the stored
manifest but allowed the caller to choose step, skill ID, and skill version.
Those values were copied consistently into the handler selection, execution
binding, observation, and candidate, so a real execution could be relabelled
without invalidating proof.

The fix removes caller authority over skill identity and rejects a selected
step that cannot be resolved exactly from the stored workflow.

## 3. Implementation Approach

The crate-private helper now:

1. validates workflow/run identity against the stored manifest;
2. resolves exactly one canonical workflow record matching manifest identity
   and content hash;
3. resolves exactly one requested step from that stored workflow;
4. resolves exactly one manifest skill reference bound to that step;
5. resolves the exact canonical skill record by identity and content hash;
6. verifies workflow version and step-to-skill ID/version consistency;
7. derives the handler selection and execution-binding skill identity from the
   resolved record; and
8. only then samples the binding clock and continues execution.

## 4. Authority Boundary

Callers still select the step to execute, invocation ID, idempotency key, result
ID, and attestation ID as explicit runtime inputs. They cannot assert which
skill that stored step resolves to. Workflow, run, step existence, skill
identity, skill version, and record content are checked against the validated
stored bundle before execution authority advances.

The accepted proof remains crate-constructed, read-only, payload-free, and
in-memory only.

## 5. Failure And Privacy Posture

Missing, duplicate, or inconsistent workflow, step, skill reference, or skill
record posture fails with stable static error codes and messages. Errors do not
echo IDs, versions, hashes, paths, source content, output, or credentials.

All immutable attribution failures occur before clock use, request
construction, runner invocation, result creation, observation, candidate, or
proof.

## 6. Tests Added

- unknown stored step fails before clock or runner use;
- a valid stored step resolves its canonical skill ID and version without
  caller-supplied skill authority; and
- the existing success, no-proof, clock, runner, freshness, identity, and Debug
  tests continue to pass.

The test fixture also gained a deterministic atomic uniqueness component after
parallel focused tests exposed a temporary-directory collision in create-only
bundle storage.

## 7. Governed Phase

- workflow: `dg/blocker`
- run: `run-1784835989653690000-2`
- approval: `approval/run-1784835989653690000-2/fix-approved`
- presentation: `presentation/d73090a631d481f7`
- approval outcome: granted by delegated maintainer through proof enforcement
- kernel boundary: governance coordination only; implementation, tests,
  documentation, and validation ran outside the kernel

## 8. Validation

Focused composition tests pass: seven passed, zero failed.

Phase-close validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 9. Remaining Limitations

- the helper has no executor or other runtime consumer;
- accepted proof is not persisted, evented, cited, or reused;
- handler implementation provenance remains registered-unattested;
- freshness must be reevaluated by a later consumer; and
- only the explicit `DocsCheck` path is supported.

## 10. Recommended Next Phase

Perform a focused blocker-fix review. Do not add consumer integration,
automatic checks, persistence, events, evidence, reports, artifacts, schemas,
CLI behavior, providers, SideEffects, writes, hosted behavior, or release
changes before review acceptance.
