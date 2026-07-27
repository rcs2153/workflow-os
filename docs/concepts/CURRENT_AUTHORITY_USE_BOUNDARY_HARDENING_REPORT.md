# Current-Authority Use-Boundary Hardening Report

## 1. Executive Summary

The accepted private current-authority same-call use boundary now has direct
later-use invalidation and stable bounded-outcome regression coverage.

The implementation behavior did not change. New tests exercise the complete
`use_current_authority` call and prove that expired or revoked grants,
unresolved independent prerequisites, changed contract/binding pairs, and a
mismatched contract cannot invoke the consumer. A fixed bounded vector covers
success, blocked, stale-source, and ambiguous completion.

This phase does not establish durable replay prevention, consumer idempotency,
runtime integration, provider execution, sandbox execution, or writes.

## 2. Scope Completed

- Added direct use-boundary expiry regression coverage.
- Added direct use-boundary revocation regression coverage.
- Added coherent changed contract/binding substitution coverage.
- Added mismatched contract rejection with a stable source-request error code.
- Added one exact reason vector for all unresolved independent prerequisites.
- Added one stable bounded outcome vector.
- Kept every blocked or invalid path non-invoking.
- Updated the accepted plan and roadmap status.
- Added this implementation report and focused review.

## 3. Scope Explicitly Not Completed

The phase did not add or change:

- public authority APIs or reusable authority handles;
- production source or executor integration;
- persistence or durable replay records;
- consumer idempotency or ambiguity reconciliation;
- providers, OpenShell, sandboxes, SideEffects, or writes;
- events, audit projection, reports, artifacts, schemas, SDKs, CLI, or UI;
- dependencies, hosted behavior, or release posture.

## 4. Hardening Behavior

Every regression enters through `use_current_authority`.

- An expired exact grant returns `BlockedBeforeUse`.
- A revoked exact grant returns `BlockedBeforeUse`.
- A coherent changed contract and execution binding cannot reuse grants scoped
  to the prior harness contract.
- A contract that does not match the supplied execution binding fails with
  `current_authority.source.request.contract_mismatch`.
- Policy, approval, evidence, and check prerequisites remain an exact ordered
  blocking reason vector.
- A stale source remains a bounded `SourceFailure`.

Every failure path asserts that the consumer invocation count remains zero.

## 5. Stable Outcome Vector

The private bounded vector is:

1. ready plus successful consumer -> `ConsumerSucceeded`, reason `Ready`;
2. revoked authority -> `BlockedBeforeUse`, reason `RequiredContextGap`;
3. stale source -> `SourceFailure`, retryable after source change; and
4. ready plus uncertain consumer completion -> `ConsumerOutcomeAmbiguous`,
   reason `Ready`.

The vector contains only typed posture, bounded reasons, and bounded source
failure categories. It contains no IDs, targets, timestamps, commitments,
payloads, paths, commands, provider responses, or secret-like values.

## 6. Privacy And Error Posture

The production types and Debug behavior remain unchanged. The new tests use
only validated model constructors and assert that the mismatched-contract
error does not disclose the contract identifier.

No raw source, provider, command, parser, CI, Jira, GitHub, sandbox,
credential, environment, token, or target content is stored or emitted.

## 7. Tests

The registered-source test set increased from 18 to 24 tests. New coverage
proves:

- expired grant non-invocation;
- revoked grant non-invocation;
- changed contract/binding non-invocation;
- mismatched-contract stable error and non-invocation;
- complete independent-prerequisite reason ordering; and
- stable bounded outcomes.

Existing current-authority, capability, context, approval, local-check,
proportional-governance, runtime, provider, and workspace tests continue to
pass.

## 8. Validation

- focused registered-source tests: passed, 24 tests;
- focused and workspace clippy with warnings denied: passed;
- `cargo fmt --all --check`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 9. Remaining Limitations

- The boundary remains private and in-memory.
- The callback remains a private test seam.
- No concrete read-only consumer exists.
- No durable use identity or atomic consumption exists.
- Cross-process replay prevention remains unproved.
- Ambiguous consumer completion remains unreconciled.
- OpenShell remains a future optional execution-provider concern.

## 10. Recommended Next Phase

Plan one concrete Core-owned opt-in read-only consumer.

The plan must replace or specialize the generic callback at the real
integration boundary. It must not expose general authority methods, introduce
provider or sandbox execution, persist authority, claim durable replay
prevention, or enable SideEffects or writes.

## 11. Governed Phase Record

- workflow: `dg/implement`;
- run ID: `run-1785177713039932000-2`;
- approval ID:
  `approval/run-1785177713039932000-2/implementation-approved`;
- approval presentation ID: `presentation/1d2548271b206c89`;
- approval presentation content hash:
  `1d2548271b206c89afaa8645d28d50e2da7ea9f4283e023ecdf2a5daa76e29bb`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- phase status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- approval-presentation enforcement: proof persisted before approval;
- out-of-kernel work: the delegated maintainer edited tests and documentation
  and ran validation; the kernel governed scope and approval but did not edit
  files, execute checks, or mutate git.
