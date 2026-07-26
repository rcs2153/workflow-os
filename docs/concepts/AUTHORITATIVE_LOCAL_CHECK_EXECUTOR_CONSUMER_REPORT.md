# Authoritative Local-Check Executor Consumer Report

## 1. Executive Summary

Workflow OS now has one explicit, opt-in executor path that turns a canonical
local `DocsCheck` result into enforceable quiet-success authority for a fresh
local workflow run.

The path builds and validates the immutable run bundle, derives all check and
attestation identities inside Core, executes the accepted same-call
local-check reassessment, persists the exact fact-source commitment, and starts
the existing sequential workflow only when the complete multi-step assessment
set resolves to complete quiet `Proceed`.

This does not make checks automatic. Existing executor APIs and defaults remain
unchanged.

## 2. Scope Completed

- Added `LocalExecutionWithAuthoritativeDocsCheckGovernanceRequest`.
- Added `LocalExecutionWithAuthoritativeDocsCheckGovernanceResult`.
- Added `execute_with_authoritative_docs_check_governance(...)`.
- Required an explicit fresh run ID and rejected existing event or bundle
  state before process use.
- Built the immutable bundle with canonical local-check declaration records.
- Performed pure reassessment preflight against an in-memory validated bundle
  before bundle persistence or local process use.
- Derived requirement, invocation, idempotency, result, and attestation
  identities inside Core with a versioned length-framed algorithm.
- Executed the explicit `DocsCheckLocalHandler` through the accepted private
  composition and reassessment path.
- Consumed the private fact-bound assessment directly.
- Added a backward-readable V2 `GovernanceAssessmentBinding` with an optional
  authoritative source commitment.
- Bound the governance event idempotency key to the aggregate assessment and
  exact source commitment.
- Projected only bounded source kind or absence into audit text.
- Enforced aggregate complete quiet `Proceed` before `RunCreated`.
- Returned the terminal run, immutable binding, governance binding, and
  bounded local-check result.

## 3. Scope Explicitly Not Completed

This phase did not add:

- default, automatic, background, parallel, or repository-wide checks;
- more than one selected check-bearing step;
- check families other than the accepted `DocsCheck` command;
- retry, rehydration, approval resume, or cancellation for the new path;
- visible-disclosure continuation or persistence;
- proportional approval creation;
- reports, artifacts, evidence attachment, CLI, UI, SDK, or schemas;
- providers, OpenShell, SideEffects, external writes, or network access;
- hosted or distributed behavior;
- reasoning lineage;
- enterprise administration; or
- release changes.

## 4. Executor API And Fresh-Run Boundary

The request reuses the immutable-bundle execution request and accepts:

- one selected step;
- one governance profile;
- exactly one runtime-fact record per immutable workflow step; and
- an optional expected aggregate fingerprint.

It does not accept local-check requirements, invocation identities, result
identities, attestation identities, a check posture, a detached fact, or a
prior source binding.

The selected step must resolve to the one valid canonical `DocsCheck`
obligation allowed by the current workflow validator's one-obligation-per-
command rule. Broader check families and multi-handler batches remain future
work.

Any prior workflow events, bundle manifest, or governance binding for the run
ID produces `executor.authoritative_local_check.existing_run_unsupported`
before another local check can run.

## 5. Preflight And Execution Ordering

The implementation orders work as follows:

1. require a fresh explicit run ID;
2. prepare the existing execution plan and evaluate pre-run policy;
3. load and validate the project;
4. build the immutable bundle in memory;
5. validate plan and bundle identity;
6. resolve the selected canonical declaration and explicit handler contract;
7. derive Core-owned identities;
8. preflight the complete reassessment context without process use;
9. persist and reload the immutable bundle;
10. execute the canonical check and derive the private fact-bound assessment;
11. validate the optional expected aggregate fingerprint;
12. enforce aggregate complete quiet `Proceed`;
13. persist the exact source-bound governance binding;
14. append the existing binding and run-start events; and
15. execute the existing sequential workflow.

No workflow event exists when check execution, attestation, reassessment,
expected fingerprint, or unsupported governance posture fails.

## 6. Decision Semantics

Only this aggregate cell executes:

```text
execution=proceed
disclosure=quiet
completeness=complete
```

The selected check may satisfy only its evidence/check axis. A passing check
cannot weaken stricter authority, sensitivity, SideEffect, policy, prior-
decision, runtime-escalation, profile, or other-step requirements.

Visible disclosure, approval-required, denial, and incomplete facts fail
closed before `RunCreated`. The path does not silently discard disclosure or
invent an approval.

## 7. Durable Source Commitment

`GovernanceAssessmentBindingVersion::V2` requires a
`GovernanceAssessmentSourceBinding` containing:

- bounded source kind;
- versioned source algorithm;
- source fingerprint; and
- selected step identity.

The only constructor that claims authoritative local-check provenance is
crate-private and consumes the private same-call bound assessment. Public serde
can read integrity records, but a deserialized record is not treated as proof
that a process ran.

Legacy V1 bindings remain readable without a source field. V1 with a source or
V2 without a source fails closed. Create-only storage equality includes the
exact source commitment.

## 8. Event, Audit, Privacy, And Error Posture

The durable binding event retains the exact integrity commitment. Its
idempotency key commits to both aggregate assessment and source binding.

Human-facing audit projection exposes only:

- aggregate execution;
- disclosure;
- completeness;
- step count; and
- `source=none` or the bounded source-kind identifier.

It does not expose the selected step, run ID, workflow ID, bundle ID,
fingerprints, paths, commands, output, source contents, parser payloads,
environment values, provider data, credentials, or tokens.

Errors use stable codes and static bounded messages. `Debug` output exposes
only statuses, counts, governance posture, and source-binding presence.

## 9. Test Coverage

Focused tests cover:

- completed multi-step quiet execution in existing sequential order;
- Core-owned source-bound V2 governance persistence;
- caller-selected check posture rejected before process and events;
- an invalid earlier context winning over a deliberately failing process;
- a stricter or incomplete other step preventing execution;
- immutable-bundle residue preventing check reuse;
- visible disclosure, approval-required, and denial failures;
- failed-check behavior with no workflow events;
- legacy V1 source absence;
- V2 source presence and version mismatch;
- serialized source commitment as non-authoritative data;
- bounded source-kind audit projection;
- audit non-leakage; and
- result `Debug` non-leakage.

## 10. Governed Implementation Record

- workflow: `dg/runtime-composition`
- run: `run-1785024801710601000-2`
- approval:
  `approval/run-1785024801710601000-2/composition-approved`
- presentation: `presentation/e2d9b078d6b176bf`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- out-of-kernel work: Rust implementation, tests, documentation, validation,
  and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run engineering checks, create a WorkReport artifact, or perform git
  actions

## 11. Validation

Validation completed:

- authoritative executor tests: 5 passed;
- source-binding audit tests: 2 passed;
- source-bound serde compatibility test: passed;
- source-binding version mismatch test: passed;
- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed; and
- `git diff --check`: passed.

## 12. Remaining Limitations

- The path is explicit and fresh-run-only.
- Phase review found and the focused blocker phase fixed a non-atomic
  fresh-run claim. Create-only immutable-manifest publication is now the
  accepted authoritative claim.
- Only one selected `DocsCheck`-bearing step is accepted.
- Current workflow validation allows one obligation per command ID, so the
  accepted `DocsCheck` slice resolves one canonical declaration.
- A check or later governance failure may leave a complete immutable bundle
  without workflow events. That bounded create-only residue is intentional and
  prevents reuse; no rollback is claimed.
- Visible disclosure and proportional approval runtime behavior are absent.
- Retry and approval resume cannot reuse this same-call authority.
- The result is in memory and is not exposed through CLI or reports.

## 13. Recommended Next Phase

Proceed to the next accepted proportional-governance runtime-composition
boundary after merge and roadmap refresh.

Do not broaden to retry, approval resume, visible-disclosure continuation,
automatic checks, additional check families, reports, CLI, providers,
OpenShell, SideEffects, writes, schemas, hosted behavior, reasoning lineage,
or enterprise administration without a separately governed phase.
