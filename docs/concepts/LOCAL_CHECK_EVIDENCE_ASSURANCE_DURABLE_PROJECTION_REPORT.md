# Local Check Evidence Assurance Durable Projection Report

## Summary

Workflow OS now persists the existing weak local-check evidence-assurance
projection for the closed trusted-host project-validation path and carries that
exact posture through reassessment, inspection, and report citation metadata.

This implementation does not strengthen the evidence. It prevents a later
observation, replay, or consumer from silently replacing or upgrading what Core
first recorded about the check result.

## Implemented Boundary

The immutable-run bundle store now maintains a create-only assurance sidecar
addressed by the exact run and local-check result. A stored projection must bind
the same:

- immutable run bundle;
- workflow, run, and step;
- local-check command identity and kind;
- selected-handler commitment;
- local-check result identity and status;
- producer, execution, attestation, and external-verifiability posture; and
- accepted verifier-owned attestation, when one exists.

Observation-specific execution and projection fingerprints are not used to
replace the first durable record. A later accepted reassessment of the same
evidence subject returns the original projection. A changed result status or
other semantic governance drift still fails through the existing governance
assessment binding before assurance reconciliation.

Missing projection directories list as empty for compatibility. Corrupt,
misaddressed, duplicate-conflicting, or immutable-bundle-mismatched records fail
closed with bounded errors.

## Runtime And Report Projection

The closed authoritative route persists its Core-owned projection before
returning it. Fresh execution, approval reassessment, and terminal reassessment
therefore resolve to one durable assurance record for the exact result.

`LocalCheckResultReference` can now carry that validated projection. Its result,
command, status, workflow, and run must match exactly. Report consumers use the
Core-derived durable result identity instead of treating a caller-supplied
label as evidence identity.

Human and JSON `inspect` output expose only bounded fields:

- result identity and status;
- producer posture;
- execution posture;
- attestation posture;
- external-verifiability posture; and
- the deterministic projection fingerprint.

The real first-run WorkReport citation summary carries the same bounded posture.
The exact projection remains a separately validated durable record; this phase
does not claim that the complete projection is a new public WorkReport schema.

## Replay And Integrity Properties

- Accepted replay returns the first persisted assurance projection without
  upgrading producer, execution, attestation, or external verifiability.
- A fresh observation may have different observation-time fingerprints, but it
  cannot replace the durable projection for the same evidence subject.
- Semantic result drift is rejected by the existing aggregate governance
  reassessment before the sidecar is reconciled.
- Stored assurance corruption blocks human and JSON inspection before bounded
  assurance output is rendered.
- The sidecar participates in the immutable-run store's writer guard and remains
  separate from workflow event mutation.

## Current Assurance

The persisted closed project-validation posture remains:

- producer: `operator_selected_content_bound`;
- execution: `trusted_host_observed`;
- attestation: `unattested`; and
- external verifiability: `unavailable`.

Durability does not authenticate the selected executable, make the host
independent, or make a local storage owner unable to rewrite all local state.

## Explicit Non-Goals

This phase did not add:

- minimum-assurance enforcement or a policy DSL;
- registered-producer identity or executable discovery;
- signatures, external witnesses, remote attestation, or storage-owner tamper
  resistance;
- arbitrary commands, provider writes, or hosted execution;
- schemas, SDK projections, or a migration mechanism for unrelated evidence;
- a new event family or a claim that persistence itself is independent proof;
  or
- release publication or broader public evidence claims.

## Governed Phase

- Workflow: `dg/runtime-composition`
- Run: `run-1793894754295967000-2`
- Approval: `approval/run-1793894754295967000-2/composition-approved`
- Presentation: `presentation/d31097e0a065fac3`
- Presentation hash:
  `d31097e0a065fac3c1b683d571be746ea291b7ac7e4e2e9bc331d08264aca704`
- Approval outcome: granted by the delegated maintainer after complete handoff
  presentation
- Kernel freshness: rebuilt from the exact checkout before phase start
- Phase close: completed with 39 events, one approval, zero retries, and zero
  escalations; approval-presentation proof enforcement was present
- Repository edits and validation ran outside the kernel under the approved
  execution scope

## Validation

- Focused durable persistence and accepted-replay tests: passed.
- Focused approval reassessment and semantic-drift ordering tests: passed.
- Focused human and JSON inspection projection tests: passed.
- Focused corrupted-assurance inspection rejection test: passed.
- `cargo test -p workflow-core --all-features`: passed, including 792 unit
  tests, 348 local-executor tests, all remaining integration suites, and doc
  tests; declared opt-in live and external-tool tests remained ignored.
- `cargo test -p workflow-cli`: passed, including 180 CLI integration tests and
  all example suites.
- `npm run check:docs`: passed.
- `cargo clippy -p workflow-core -p workflow-cli --all-targets --all-features
  -- -D warnings`: passed.
- `cargo fmt --all --check`: passed.
- `git diff --check`: passed.

## Next Phase

Implement one narrow fail-closed minimum-assurance gate over the Core-owned
durable projection. Preserve existing behavior when no minimum is declared. Do
not broaden producer discovery, executable selection, policy language, or
external execution in that phase.
