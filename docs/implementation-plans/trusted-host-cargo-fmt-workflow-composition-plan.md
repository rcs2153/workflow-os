# Trusted-Host Cargo Format Workflow Composition Plan

Status: implemented after accepted focused maintainer/security plan review;
focused implementation/security review found blocker fixes are required.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [Trusted-Host Two-Step No-Write Workflow Plan](trusted-host-two-step-no-write-workflow-plan.md)
- [Trusted-Host Two-Step No-Write Workflow Review](../concepts/TRUSTED_HOST_TWO_STEP_NO_WRITE_WORKFLOW_REVIEW.md)
- [Trusted-Host Cargo Format Check Profile Plan](trusted-host-cargo-fmt-profile-plan.md)
- [Trusted-Host Cargo Format Check Profile Blocker Fix Review](../concepts/TRUSTED_HOST_CARGO_FMT_PROFILE_BLOCKER_FIX_REVIEW.md)

## 1. Executive Summary

Workflow OS has two accepted private trusted-host proofs that are not yet
composed together:

1. an exact ordered two-step no-write workflow that runs project validation
   followed by the docs check and persists one v3 terminal report artifact;
2. an exact one-step Cargo format profile that runs
   `cargo fmt --all --check` with bounded toolchain identity, repository
   final-state commitments, deterministic result truth, terminal reporting,
   and exact replay.

The next implementation should add one additive exact three-step workflow:

```text
workflow-os project validation
  -> docs check
  -> cargo format check
  -> terminal WorkReport
  -> private SQLite report artifact
```

Appending Cargo format preserves the accepted two-step workflow as the exact
prefix. The existing two-step mode, registry, v3 report commitment, artifact
identity, and tests remain unchanged. The new path uses a distinct closed
three-profile registry, exact three-step topology, bounded three-operation host
loop, and private v4 terminal report commitment.

The implementation described by this plan is complete. It does not authorize
clippy, tests, builds, generic Cargo execution, source-writing format mode,
providers, external writes, public CLI behavior, schemas, examples, hosted expansion,
parallelism, nested harnesses, or release changes.

## 2. Why Composition Is Next

The Cargo format profile has completed plan review, implementation, blocker
fix, and focused security review. The accepted two-step workflow already owns
ordered progression, immutable bundle binding, causal-history validation,
restart behavior, report composition, and artifact reconciliation.

The remaining gap is runtime composition, not a new primitive family. A third
closed check proves that the existing sequential boundary can grow by one
reviewed profile without becoming arbitrary command execution or weakening
the accepted two-step compatibility surface.

## 3. Goals

- Add exactly one private three-step no-write trusted-host workflow.
- Preserve the existing project-validation then docs-check prefix.
- Append the accepted Cargo format profile as the terminal check.
- Keep Core authoritative for topology, current-step, and profile selection.
- Bind all three exact command contracts into the immutable run bundle and
  bind each resolved trusted-host execution identity through its durable
  dispatch executor commitment.
- Commit each result, invocation terminal, and successor schedule atomically.
- Re-admit from durable state before each later step.
- Fail honestly at any step without scheduling later work.
- Resume after process loss without rerunning a committed successful step.
- Produce one deterministic v4 terminal report over exact durable history.
- Persist one create-only private SQLite artifact through the accepted gate.
- Preserve existing one-step and two-step behavior and identity unchanged.

## 4. Strict Non-Goals

This phase must not add:

- another local-check profile;
- `cargo clippy`, `cargo test`, `cargo build`, or arbitrary Cargo arguments;
- source-writing `cargo fmt` behavior;
- a generic profile registry, command registry, or caller-authored ordering;
- parallel steps, branches, loops, fan-out, or nested harness execution;
- providers, network authority, credentials, SideEffect execution, or writes;
- automatic approval, approval bypass, or broader delegated authority;
- a scheduler, queue poller, daemon, or workflow discovery behavior;
- released `workflow-os` CLI behavior, report rendering, or export;
- workflow schema, SDK, onboarding scaffold, or example changes;
- hosted or PostgreSQL parity claims;
- an operating-system sandbox or an independent toolchain attestation claim;
- migration or replacement of the v3 two-step report contract; or
- release-posture changes.

## 5. Compatibility Decision

Do not rename or generalize the accepted two-step types in the first
implementation. Add an exact sibling boundary, likely:

- `TrustedHostThreeStepNoWriteProfileRegistry`;
- `TrustedHostThreeStepNoWriteAdmissionRequest`;
- `admit_trusted_host_three_step_no_write_operation(...)`;
- `compose_trusted_host_three_step_terminal_work_report(...)`; and
- `persist_trusted_host_three_step_terminal_work_report_artifact(...)`.

The existing two-step function and report path remain byte-for-byte compatible
unless a minimal shared private helper can be extracted without changing their
observable result, error, event, or artifact behavior. A generic N-step
abstraction is not required and must not be introduced merely to reduce local
duplication.

## 6. Exact Workflow Shape

The only new topology is:

| Order | Step | Skill | Closed profile | Terminal behavior |
| --- | --- | --- | --- | --- |
| 1 | `project-validation` | `local/workflow-os-validate@v0` | `workflow_os_project_validation` | continue |
| 2 | `docs-check` | `local/check-docs@v0` | `docs_check` | continue |
| 3 | `cargo-format-check` | `local/check-cargo-fmt@v0` | `cargo_fmt_check` | terminal |

Appending the third step preserves the accepted two-step prefix. Each step
must retain explicit-zero required context and authority, local/read-only
policy posture, no adapter, no hook, and no SideEffect events.

The implementation may use a test fixture or dogfood-only project definition.
It must not enter onboarding scaffolds, released examples, or public schemas.

## 7. Closed Three-Profile Registry

The host resolves all three accepted profiles before admission. The registry
key set is exactly:

- `local/workflow-os-validate@v0`;
- `local/check-docs@v0`; and
- `local/check-cargo-fmt@v0`.

Construction must reject missing, duplicate, substituted, identity-mismatched,
or authority-bearing profiles. Inventory generation must contain exactly the
three accepted command contracts in workflow order. Profile selection remains
derived from the immutable current step inside Core.

The registry exposes no arbitrary map, generic constructor, or caller-selected
command. Its `Debug` output reveals only fixed bounded posture such as profile
count.

## 8. Host Input And Toolchain Boundary

Add one explicit unpublished application mode, likely
`three-step-no-write`. It may accept the existing two-step inputs plus the
already reviewed Cargo-profile inputs:

- explicit Cargo executable;
- explicit cargo-fmt executable;
- explicit rustfmt executable; and
- canonical toolchain root.

The host resolves the Cargo profile through the accepted constructor. It must
not accept a step ID, profile ID, command, argument list, successor, event
cursor, result, output, or report identity.

Mixed mode arguments fail before state mutation. Repository and toolchain
roots remain disjoint in both directions. The exact toolchain inventory and
Core-owned repository exclusion policy remain committed and revalidated under
the accepted Cargo profile contract.

## 9. Admission And Immutable Binding

Fresh admission must:

1. validate the project and exact three-step topology;
2. prepare the complete execution plan;
3. validate all three profiles and roots;
4. build one immutable run bundle containing the exact workflow, handler
   posture, and all three command contracts;
5. persist or validate that exact bundle;
6. append run start and the first schedule;
7. derive policy and approval posture without broadening authority; and
8. prepare only the first opaque operation.

The immutable bundle root is not, by itself, the resolved Cargo toolchain and
protected-source identity. Each admitted operation must also durably record the
existing trusted-host executor commitment in its dispatch event. That
commitment includes the command-contract fingerprint and private resolved
execution fingerprint; for Cargo format, the latter includes the accepted
formatter-chain inventory and protected-source commitment. Re-admission must
reproduce and verify the exact dispatch commitment before execution. Terminal
report composition must verify the durable dispatch executor commitment
against the immutable step input and use that durable commitment as the
authoritative admitted execution identity. This matters for an honest
pre-result identity failure: the current mutated executable or source state
intentionally may no longer reproduce the admitted identity at report time. A
command-contract match without the matching durable dispatch executor
commitment is insufficient.

Re-admission must load the exact stored bundle, validate request identity,
validate the complete causal history, derive the latest scheduled step, derive
its skill and profile from the immutable bundle and closed registry, and then
reuse the accepted trusted-host preparation boundary.

Caller memory is never progression authority.

## 10. Atomic Progression

Core and SQLite own the following transitions:

- Project-validation success commits its result, invocation success, and the
  exact docs-check schedule in one transaction.
- Docs-check success commits its result, invocation success, and the exact
  cargo-format-check schedule in one transaction.
- Cargo-format success commits its result, invocation success, and
  `RunCompleted` in one transaction.
- Failure or timeout at any step commits the result, invocation failure, and
  `RunFailed` without scheduling a successor.
- Integrity failures before result acceptance append no false check result or
  success event.

The foreground host must not append lifecycle events, patch snapshots, select
the successor, or turn an integrity failure into a formatter result.

## 11. History Validation

The exact legal scheduled prefixes are:

```text
[project-validation]
[project-validation, docs-check]
[project-validation, docs-check, cargo-format-check]
```

The exact legal result prefixes use the same order. For every attempted step,
validation must prove:

```text
step scheduled
  < local-check result recorded
  < matching invocation terminal
  < next step scheduled, when one exists
```

Each result must bind the expected step, profile, command, immutable bundle,
invocation, and terminal event. A non-passing result may not have a successor.
A completed run requires exactly three passing results. A failed run requires
the last attempted result to be non-passing or the exact pre-result failure
shape defined below. Each attempted step must also have exactly one matching
authorized dispatch whose executor commitment reproduces from the closed
profile and immutable step identity. Duplicate, skipped, reordered,
contradictory, or substituted history fails closed.

The accepted pre-result failure shape is deliberately narrow:

```text
step scheduled
  < matching authorized dispatch admitted
  < matching skill invocation failed
  < run failed
```

It contains no `LocalCheckResultRecorded` event for the failed step and no
successor schedule. This is the existing fail-closed shape when handler
identity, protected-source, or another pre-result execution check returns an
error. Because the current supervisor intentionally normalizes raw handler
errors, the report may disclose only that execution failed before a bounded
check result was accepted. It must not infer or claim a more specific cause
that durable history does not prove. Tests may induce a known Cargo integrity
failure to prove this shape, but report text remains bounded to persisted
truth.

## 12. Bounded Host Loop

The private process may execute at most three prepared operations. After each
committed continuation it must update trusted observation time and call Core
admission again with the same run and bundle identities. It must not construct
the next operation from memory or retain a reusable capability.

The process stops immediately on waiting, blocked, denied, terminal replay,
window expiry, integrity failure, or any unexpected continuation posture.

## 13. Terminal WorkReport And Artifact

Use a distinct private report identity and commitment, proposed as:

- contract ID: `trusted-host-three-step-terminal-no-write`;
- contract version: `v4`; and
- commitment domain: `workflow-os/trusted-host-terminal-report/v4`.

The report commitment must bind:

- run and immutable bundle identity;
- exact three-step topology and order;
- exact closed profile and command identity for each attempted step;
- the matching authorized dispatch event and resolved executor commitment for
  each attempted step;
- each concrete local-check result;
- each matching invocation terminal event; and
- the terminal run event.

Completed reports require three passing result citations and three matching
dispatch bindings. Failed reports cite only accepted results, bind every
attempted step's dispatch and invocation-terminal events, and disclose later
work explicitly:

- project-validation failure: docs and Cargo format were not attempted;
- docs-check failure: Cargo format was not attempted;
- Cargo-format failure: all three checks were attempted and the final check
  did not pass.

When a step fails before result acceptance, the report must not fabricate a
local-check result citation. It binds the scheduled step, matching dispatch,
failed invocation, and terminal run event; states that no bounded result was
accepted; and discloses every later step as not attempted. The v4 report
commitment includes the durable executor commitment so Cargo's private
toolchain/source execution identity is covered without serializing paths,
inventory entries, or source material.

The artifact path remains create-only and uses the accepted report artifact
gate. Exact duplicate reconciliation may return already persisted; conflicting
or unreadable artifacts fail closed. Report or artifact failure cannot rewrite
terminal workflow truth.

## 14. Replay And Recovery

Required restart points include:

- before the first operation;
- after project-validation commit;
- after docs-check commit;
- after Cargo-format terminal commit but before artifact creation; and
- after exact artifact persistence.

Fresh admission derives the only lawful posture from durable state. It never
reruns a committed check. Missing exact terminal artifacts may be recreated
through the accepted create-only path. Exact terminal replay appends no events
and performs zero process invocations.

## 15. Failure And Privacy Posture

Preserve stable fixed error families for registry, topology, history, report,
artifact, root, toolchain, and integrity failures. Errors, Debug, events,
reports, artifacts, and host output must not expose:

- source or toolchain paths;
- stdout, stderr, or command transcripts;
- source/spec/parser/provider payloads;
- environment values or Cargo configuration;
- credentials, authorization headers, private keys, or token-like values; or
- caller-authored unbounded prose.

The report cites stable local-check and event references. It does not create
`EvidenceReference` values implicitly. Final-state source equivalence still
does not prove that a transient write never occurred, and disabled-network
posture still is not operating-system egress isolation.

## 16. Test Plan

Focused implementation tests must prove:

1. exact successful order: project validation, docs check, Cargo format;
2. exact three-profile registry and immutable command inventory;
3. caller step/profile/command selection is rejected before state mutation;
4. first-step failure never schedules docs or Cargo format;
5. second-step failure never schedules Cargo format;
6. third-step failure records all three results and fails honestly;
7. non-zero and timeout Cargo outcomes remain bounded result truth;
8. Cargo source or toolchain integrity failure cannot become success;
9. a Cargo pre-result integrity failure produces no check result, binds the
   exact dispatch and failed invocation, and yields an honest v4 report;
10. dispatch executor-commitment substitution fails closed during admission,
    history validation, and report composition;
11. restart after first commit runs only steps two and three;
12. restart after second commit runs only step three;
13. terminal replay runs no process and appends no event;
14. causal-history corruption at either successor boundary fails closed;
15. profile, result, command, bundle, or invocation substitution fails closed;
16. completed v4 report cites all three stable results in order;
17. each failure report cites only accepted results, binds attempted work, and
    discloses skipped work;
18. missing artifact recovery is create-only and exact;
19. conflicting artifact posture fails closed;
20. Debug, errors, output, events, report, artifact, and serialization do not
    leak paths or raw process output;
21. existing one-step Cargo behavior is unchanged;
22. existing one-step project-validation/docs behavior is unchanged; and
23. existing two-step v3 workflow, report, artifact, and replay behavior is
    unchanged.

Before acceptance, run focused Core and foreground tests plus the complete
workspace validation required by the engineering standard. The real Cargo
format smoke remains opt-in and must use the pre-provisioned bundled stable
toolchain without installation or network access.

## 17. Implementation Sequence

1. Add the exact private three-profile registry and request types.
2. Add exact three-step topology and causal-history validation.
3. Extend the existing outcome-aware lifecycle progression only as needed for
   the second continuation boundary.
4. Add the private three-step admission function and immutable bundle binding.
5. Add the unpublished bounded host mode and three-operation loop.
6. Add v4 report composition and create-only artifact persistence.
7. Add focused success, failure, restart, corruption, replay, privacy, and
   compatibility tests.
8. Update implementation documentation and run full validation.
9. Perform focused maintainer/security review before any further profile or
   topology expansion.

## 18. Implementation Review Questions

The focused implementation review must decide:

- whether an additive three-step sibling is preferable to any private shared
  helper extraction;
- whether appending Cargo format is the correct compatibility-preserving order;
- whether v4 identity binds every new profile/toolchain commitment needed for
  exact replay;
- whether history validation remains complete at both continuation boundaries;
- whether Cargo integrity failures are represented without false result truth;
- whether artifact recovery remains exact and create-only; and
- whether the proposed tests prove two-step and one-step non-regression.

## 19. Recommended Next Phase

Implement the blocker fixes from
[Trusted-Host Cargo Format Workflow Composition Implementation Review](../concepts/TRUSTED_HOST_CARGO_FMT_WORKFLOW_COMPOSITION_IMPLEMENTATION_REVIEW.md).
Complete exact attempted-step dispatch validation, failed-terminal history
validation, pre-result invocation linkage, and the mandatory corruption and
privacy regressions. Do not broaden the profile registry, topology, public
surface, provider access, or write posture before focused re-review accepts
the fixed boundary.
