# Current-Authority Proportional-Governance Runtime Composition Report

## 1. Executive Summary

Workflow OS now has a private Core bridge between registered current authority
and the existing authoritative proportional-governance executor route.

The bridge accepts one unclassified single-step runtime fact, validates its
exact immutable execution and harness-contract binding, freshly resolves
current authority, and injects `Sufficient` only inside the same-call consumer.
Callers cannot preclassify sufficient authority on this path. Blocked or stale
authority never reaches the governance route.

The bridge remains private and is not used by the CLI yet. The CLI compatibility
path still supplies a hardcoded authority fact until a reviewed production
local current-authority source and configuration boundary exists.

## 2. Scope Completed

- Added a private exact execution-and-contract input for governance routing.
- Added a private same-call current-authority governance consumer.
- Reused the registered source and existing `FnOnce` use boundary.
- Reused the existing authoritative local-check proportional-governance route.
- Rejected preclassified authority, mismatched step identity, multiple runtime
  facts, and mismatched execution bindings.
- Injected sufficient authority only inside the ready callback.
- Preserved underlying executor errors without relabeling them as authority
  success.
- Added focused ready, blocked, stale-source, caller-preclassification, and
  privacy regressions.

## 3. Scope Explicitly Not Completed

The phase did not add:

- automatic approval or ambient authority;
- a public or reusable capability/authority object;
- CLI activation, schema fields, or runtime configuration;
- a production current-authority source;
- arbitrary commands or additional local-check profiles;
- providers, OpenShell, sandbox execution, SideEffects, or writes;
- persistence, hosted execution, enterprise identity, or release changes.

## 4. Runtime Boundary

The private route requires:

- the exact immutable required-context execution binding;
- the matching required-context contract identity, version, and content hash;
- one selected step and exactly one runtime-fact record;
- an absent authority field;
- a registered source, explicit evaluation time, and redaction metadata; and
- the existing closed local-check profile and executor dependencies.

The source reruns current-authority resolution and required-context consumption.
Only a ready result invokes the callback. The callback clones the request,
injects `GovernanceWorkloadAuthorityPosture::Sufficient`, and immediately calls
the existing authoritative route. The bound fact is not returned or persisted
as a reusable authorization result.

## 5. Failure And Privacy Posture

Stable errors distinguish:

- execution-binding mismatch;
- invalid runtime-fact count or step;
- caller-preclassified authority;
- blocked authority;
- source failure; and
- inconsistent consumer outcomes.

Errors do not include actor, workflow, run, step, source, grant, contract,
context, path, command, provider, or secret-like values. Blocked and
source-failure paths prove zero consumer invocations.

## 6. Test Coverage

Focused tests prove:

- ready authority invokes the consumer once with sufficient authority;
- revoked authority blocks before invocation;
- stale source fails before invocation;
- caller-preclassified authority fails before source use;
- stable errors do not expose governed actor or run identity; and
- existing registered-source and current-authority behavior remains intact.

The full workspace validation remains the regression boundary for
proportional-governance, local-check, executor, WorkReport, provider, hosted,
and persistence behavior.

## 7. Validation

- focused registered current-authority tests: passed, 37 tests;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed; and
- `cargo test --workspace`: started and all reached suites passed, but the
  local command was stopped because macOS imposed roughly two minutes of
  process-launch latency per one of 66 test executables. The required GitHub
  Rust check remains the authoritative complete workspace-test gate before
  merge.

## 8. Remaining Limitations

- The bridge is private and currently unused outside focused Core tests.
- The registered source is in-memory and not configured by the CLI.
- The CLI still hardcodes sufficient authority on its compatibility path.
- The first bridge supports one selected step only.
- Durable replay prevention and transactional source/executor coupling remain
  deferred.
- OpenShell may later be an optional execution provider, but it is not an
  authority source and is not part of this phase.

## 9. Recommended Next Phase

Define and review the first production local current-authority source and
configuration boundary for the closed project-validation profile. Then replace
the CLI compatibility shortcut with this source-bound route.

Do not broaden provider mutation families, sandbox execution, or authority
vocabulary first.

## 10. Governed Phase Record

- workflow: `dg/runtime-composition`;
- run ID: `run-1785403735124792000-2`;
- approval ID:
  `approval/run-1785403735124792000-2/composition-approved`;
- approval presentation ID: `presentation/dffdf3e53f66e336`;
- approval presentation content hash:
  `dffdf3e53f66e336d0ac9a44a442e4bec430e4eab994b0033968dd9cde6f22d3`;
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted handoff was presented;
- phase status: completed;
- event summary: 39 events, 1 approval, 0 retries, 0 escalations;
- approval-presentation enforcement: proof enforced with the approval event
  marker present;
- out-of-kernel work: the delegated maintainer edited implementation, tests,
  and documentation and ran validation; the kernel governed scope and approval
  but did not edit files, execute checks, mutate git, or invoke providers.
