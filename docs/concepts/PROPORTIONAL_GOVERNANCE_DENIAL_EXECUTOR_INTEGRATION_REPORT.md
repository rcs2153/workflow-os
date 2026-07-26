# Proportional Governance Denial Executor Integration Report

## 1. Executive Summary

Workflow OS now has one explicit local executor path for a complete,
source-bound proportional-governance result whose route is
`Denied + Visible`.

The path persists the exact authoritative assessment, starts the ordinary run
lifecycle, and terminates with a distinct policy-denied failure before any
workflow step is scheduled. It reuses existing durable event vocabulary
without inventing a second denial subsystem.

This is additive and opt-in. Existing executor, approval, disclosure,
provider, SideEffect, and report behavior remains unchanged.

## 2. Scope Completed

- Added an explicit result type for the authoritative denied route.
- Added an explicit fresh-run executor method that consumes the existing
  authoritative `DocsCheck` governance request.
- Required an exact complete source-bound `Denied + Visible` assessment.
- Persisted the exact immutable bundle and governance assessment binding.
- Appended ordinary run-start events before terminal denial.
- Failed with a distinct stable code and `PolicyDenied` failure class.
- Stopped before step scheduling, skill invocation, approval, provider, or
  SideEffect activity.
- Reused the existing `GovernanceAssessmentBound` and `RunFailed` events.
- Added focused route, ordering, privacy, and rejection tests.

## 3. Scope Explicitly Not Completed

The phase did not add:

- a new denial model, event, store, or approval behavior;
- retry, resume, or existing-run support for the denied route;
- automatic proportional-governance routing from default executor methods;
- disclosure delivery that claims a denied decision was observed;
- CLI, UI, workflow-schema, or example exposure;
- report or artifact generation;
- providers, OpenShell, sandbox execution, or credentials;
- SideEffect execution or a new mutation family;
- hosted behavior, reasoning lineage, or release changes.

## 4. API And Runtime Route

The implementation adds:

- `LocalExecutionWithAuthoritativeDocsCheckDeniedGovernanceResult`; and
- `execute_with_authoritative_docs_check_denied_governance(...)`.

The result exposes the failed run, immutable bundle binding, exact governance
assessment, and bounded local-check results through read-only accessors.
`Debug` omits run identity, check contents, paths, and fingerprints.

The route accepts the same explicit request used by the other authoritative
`DocsCheck` consumers. It does not accept a caller-selected denial enum or
detached projection as authority.

## 5. Event, Failure, And Audit Semantics

The successful denied route emits:

1. `RunCreated`;
2. `GovernanceAssessmentBound`;
3. `RunValidated`;
4. `RunStarted`; and
5. `RunFailed`.

No `StepScheduled`, skill, approval, hook, provider, or SideEffect event is
appended.

The terminal error code is:

```text
executor.authoritative_local_check.governance_denied
```

The failure class is `PolicyDenied`. Together with the exact durable
assessment binding, this distinguishes authoritative governance denial from
incomplete assessment, check failure, disclosure-delivery failure, approval
denial, missing handlers, and ordinary execution failure.

No new event kind is needed. `GovernanceAssessmentBound` truthfully records
the denied decision and `RunFailed` truthfully records the terminal lifecycle.

## 6. Ordering And Crash Posture

The explicit path orders work as follows:

1. require a fresh run and empty durable event state;
2. prepare and validate the execution plan;
3. build and create-only claim the immutable run bundle;
4. execute the canonical `DocsCheck`;
5. derive the complete source-bound aggregate assessment;
6. require exact `Denied + Visible`;
7. persist the exact governance assessment binding;
8. append ordinary run-start events; and
9. append the denial-specific terminal failure before step scheduling.

As with the accepted authoritative consumers, a failure after immutable bundle
or assessment persistence can leave bounded create-only residue. No execution
authority is created by that residue. Retry and recovery semantics remain
separately deferred.

## 7. Privacy And Error Behavior

The route fails closed when the assessment is incomplete, lacks authoritative
source binding, or is not exactly `Denied + Visible`.

Errors use stable static codes and do not include:

- raw source or spec contents;
- local-check output;
- commands, environment values, or paths;
- assessment fingerprints or reason payloads;
- approval or disclosure prose;
- provider payloads; or
- credentials, authorization headers, private keys, or tokens.

Unknown or caller-inconsistent route inputs fail before run events.

## 8. Test Coverage

Focused tests prove:

- an authoritative denied assessment creates a failed in-memory run;
- the exact assessment binding is durable;
- the failure uses the denial-specific code and `PolicyDenied` class;
- the event sequence is deterministic;
- no step, skill, approval, hook, provider, or SideEffect event appears;
- quiet proceed, visible proceed, and approval-required assessments are
  rejected before events by this route;
- a caller's quiet disclosure hint cannot suppress the derived visible
  obligation for denial;
- result `Debug` does not expose run identity; and
- the complete `workflow-core` suite remains compatible.

## 9. User Feedback Reconciliation

Current external evaluation describes Workflow OS as a coherent and honest
local governance kernel while identifying ceremony as the remaining product
constraint. This phase completes the high-risk half of proportional routing:
an authoritative denial now reaches a real terminal executor boundary instead
of remaining model vocabulary.

It also reinforces the distinction raised in feedback about visible
disclosure. The caller does not select a separate governance mode. A denied
assessment monotonically derives `Visible`, even when the caller supplies no
visibility minimum. Product surfaces may display quiet evidence live, but
they may not weaken a required visible obligation or manufacture proof of
delivery.

Repo-specific onboarding, concise first-run presentation, Node 20 integration
tooling, and duplicate pre-scaffold diagnostics remain separate product lanes.
They do not weaken this executor boundary.

## 10. Validation

The following passed:

- focused authoritative denial route tests;
- `cargo test -p workflow-core`;
- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

## 11. Governed Phase Record

- workflow: `dg/runtime-composition`
- run: `run-1785051023156147000-2`
- approval:
  `approval/run-1785051023156147000-2/composition-approved`
- presentation: `presentation/a69761d54300b2c8`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, and 0 escalations
- approval-presentation event marker: present
- out-of-kernel work: source inspection, Rust implementation, tests,
  documentation, and later git/PR work
- missing coverage: the kernel coordinated governance only; it did not edit
  files, run checks, create a WorkReport artifact, or perform git actions

## 12. Remaining Limitations And Recommendation

The path is fresh-run-only, local, `DocsCheck`-only, and explicit. It does not
automatically select among routes, persist disclosure-delivery receipts, or
expose proportional governance through CLI or schemas.

Proceed next to the combined authoritative routing review. Review quiet
proceed, visible proceed, approval-required, and denied routes together for
monotonicity, crash ordering, non-leakage, and compatibility before adding
operator UX, optional execution providers, or broader mutation families.
