# Hacker News Launch Admission Plan Review

Status: accepted after focused maintainer/product/security review and plan
corrections.

## Decision

Accept Phase 0 of the Hacker News launch-admission lane. The corrected plan
selects one existing real operation, identifies only bounded CLI composition
gaps, and preserves the kernel's existing authority and execution boundaries.

This decision accepts planning only. It does not authorize runtime
implementation, artifact publication, a GitHub Release, a Hacker News
submission, live provider activation, arbitrary command execution, or broader
product claims.

This is owner-conducted repository assurance with an independent agent review,
not independent external security assurance.

## Findings And Corrections

### 1. The golden path implied two operations

The initial plan treated first-run artifact production and an ordinary
`run`-shaped operation as separate requirements. That invited a parallel path
or unnecessary trusted-handler broadening.

Corrected:

- the lane selects exactly
  `workflow-os first-run --write-report-artifact --run-id
  launch-first-run-v1`;
- the operation reuses the existing local executor, immutable run, policy,
  approval, proportional-governance, evidence, and report contracts; and
- the plan prohibits automatically registering its trusted handler for
  arbitrary generic `run` calls.

### 2. Initial quiet success could omit its validation citation

The real project-validation check executes on the initial quiet-success path,
but the current CLI does not pass its stable local-check reference into the
persisted WorkReport.

Corrected in the plan:

- Phase 2 must preserve the initial authoritative project-validation reference;
- clean-room admission requires a WorkReport that cites it; and
- exit criteria make the citation mandatory.

### 3. Persisted evidence is not sufficiently inspectable

Current `inspect` output exposes artifact identity and metadata but not the
bounded report sections or validation citation required for a new user to
verify what happened.

Corrected in the plan:

- `inspect` must expose bounded report and citation posture, analysis
  commitment, limitations, and skipped work;
- sensitivity and redaction remain mandatory; and
- raw command output and generic report-body rendering remain excluded.

### 4. Unconditional `next-action` conflicted with quiet success

The selected operation can complete without interruption. Terminal runs
correctly reject `next-action` with `run.not_running`.

Corrected:

- `status` and `inspect` are required after completion;
- `next-action` is required only for an approval or recoverable nonterminal
  state; and
- terminal `run.not_running` is documented as the correct result.

### 5. Default onboarding still elevated mock execution

Scaffold and verbose first-run output currently present the mock sequencing
demo as a next step even though the real artifact path exists.

Corrected:

- the real artifact operation becomes the primary governed next step; and
- mock execution may remain only as an optional, clearly labeled sequencing
  demonstration and cannot satisfy launch evidence.

### 6. Packaging order did not prove the final composed artifact

The initial phase order rehearsed release artifacts before golden-path changes
without requiring a final rebuild.

Corrected:

- Phase 1 defines the packaging contract and produces provisional artifacts;
- Phase 2 rebuilds and checksums final native candidates from the exact
  post-composition commit; and
- any later code change invalidates those candidates.

## Selected Operation

```sh
workflow-os first-run \
  --write-report-artifact \
  --run-id launch-first-run-v1
```

The operation is credential-free, network-free, ecosystem-neutral, rejects
mock mode, validates exact generated contracts, performs real Workflow OS
project validation, derives governance posture in Core, persists durable events
and a WorkReport artifact, and supports exact replay and bounded recovery.

## Smallest Missing Composition

Phase 2 may implement only:

1. initial local-check citation preservation;
2. bounded report and citation projection through `inspect`;
3. real-artifact default onboarding guidance; and
4. explicit replay and interruption disclosure.

No new Core primitive, runtime provider, generic command runner, parallelism,
provider fixture, or arbitrary trusted-handler registration is required.

## Validation

Passed after plan correction:

- `npm run check:docs`; and
- `git diff --check`.

## Residual Review Requirements

- Fault injection must remain test-only.
- Bounded inspection must honor sensitivity and redaction.
- A code change after candidate generation must invalidate and rebuild native
  archives and checksums.
- Phase 1 and Phase 2 each require separate governed implementation and focused
  review before clean-room evaluation.

## Next Authorized Increment

Phase 1 may define and rehearse the native packaging contract and provisional
archives without publication. Runtime composition remains reserved for Phase 2,
and final clean-room candidates must be rebuilt after Phase 2 acceptance.
