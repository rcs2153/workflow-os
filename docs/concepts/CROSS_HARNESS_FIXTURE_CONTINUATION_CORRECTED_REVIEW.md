# Corrected Fixture Continuation Security Review

## Final Maintainer Recheck

Exact implementation head `47947650` is accepted for the fixture-only bridge.
The earlier rejection below applies to `9d6f6466`. This is delegated maintainer
review, not independent security assurance or acceptance of the harness corpus.

The final SQLite read compares the full event history and exact SideEffect
against the same transaction's operational window and dispatch reservation.
The interleaving regression invalidates a stale result when a check failure
commits after the initial read. Approval presentation and immutable bundle
material are create-only. The verdict is non-authorizing and applies at the
read snapshot, not to future dispatch.

Required check declarations in the selected prefix fail closed when their
attestation cannot be verified. The synthetic fixture success case remains
valid only within that boundary. No attested-check support, live provider
effect, or cross-principal delegation is accepted.

Next: separately govern exact-head PR delivery and local sync, then the
independent fixture harness corpus. No publication or merge was performed
under this review grant.

### Recheck Governance

- Workflow: `dg/review`
- Run: `run-1794031321127705000-2`
- Approval: `approval/run-1794031321127705000-2/review-scope-approved`
- Presentation: `presentation/7e9e59f1b60292ae`
- Outcome: granted under standing delegation with persisted proof
- Closeout: Completed, 39 events, one grant, no retries or escalations
- Validation: three focused continuation tests with the rebuilt CLI probe,
  35 fixture-filtered Core unit tests, docs check, and diff check passed
- Out-of-kernel work: source inspection, rebuilt CLI probe, tests, docs and
  Git commit. No WorkReport, live calls, release, or PR mutations.

## Earlier Review

Reviewed head: `9d6f6466`. Verdict: not accepted for the independent harness
corpus. The fixture success path is real; two authority checks need correction.

## Correction Candidate

The final read transaction now compares the exact event history and durable
SideEffect against the facts used by validation in the same snapshot as the
continuity tables and dispatch reservation. The approval proof and immutable
bundle are create-only. The returned fixture verdict is valid at that read
snapshot; it is not a capability or a promise about later dispatch.

A regression commits a later failed check after the initial run read, during
approval linkage lookup, while the execution window remains open. The old
verdict is rejected at the final snapshot boundary. Rereading the updated run
also rejects the superseded pass.

The fixture bridge refuses workflows with required check declarations in
the selected prefix when the durable record cannot prove their attestation
and constraints. A parsed declaration regression covers that refusal. The
existing positive fixture has no required check declaration and does not
prove support for declared attested checks. No broader authority is issued.

Correction governance: `dg/blocker`, run `run-1794030899775173000-2`,
grant `approval/run-1794030899775173000-2/fix-approved`, presentation
`presentation/34e30d508a7f667e`, proof-backed delegated approval. Closeout:
Completed, 39 events, one grant, no retries or escalations. Three focused
continuation tests with the rebuilt CLI probe, 35 fixture-filtered Core unit
tests, warnings-denied Core and CLI Clippy, format, docs, and diff checks
passed. Source edits, test commands, docs and Git work occurred outside the
kernel. No WorkReport, live call, PR, or merge was performed.

## Findings

### P0: Current authority is assembled from separate database snapshots

The Core entry point rehydrates the run and reads the SideEffect before
`fixture_predispatch_attempt_is_current` starts its own SQLite transaction.
That transaction checks the current continuity tables and dispatch reservation
but does not compare the run cursor or SideEffect to the values already read.
A later check failure or cancellation committed between the reads can leave
the old run evidence paired with a still-open window. The CLI could then
report a valid fixture after the evidence used for that decision changed.
The result is non-authorizing and does not dispatch, but it is an incorrect
continuation verdict.

Use one consistent read snapshot for the run, proof, SideEffect, evidence,
and window, or prove a bounded equivalent with cursor and record commitments.
Add a deterministic interleaving test that commits drift between the reads.
Do not use retries that can silently reuse the earlier approval evidence.

### P1: Declared check assurance and freshness are not fully verified

`validate_fixture_continuation_check_evidence` checks result ID, status,
bundle, latest step/command result and, for a declared obligation, the linked
success event. It does not resolve the declaration's freshness, minimum
assurance, evidence-assurance minimum, truncation, network, or side-effect
constraints. The positive fixture appends a synthetic check result to an echo
workflow; it does not exercise a declaration that demands those properties.
An obligation with stronger constraints can appear satisfied by a matching
pass event even when the required provenance is absent.

Use the existing resolved check/attestation facts to enforce declared
requirements. Where the fixture path cannot verify a requirement, fail closed
with a bounded diagnostic. Test one declared requirement with authentic
supporting facts and one with insufficient or stale facts; do not claim the
synthetic fixture proves attestation.

## Verified And Preserved

The focused Core/CLI probe accepted the exact request from an existing
read-only database and rejected altered body, actor, mode, and malformed
reference input. The prior regression evidence covers terminal runs, missing
or superseded check evidence, and expired/revoked/consumed windows at their
respective boundaries. None of this grants live provider dispatch or
cross-principal delegation. This was a maintainer review, not independent
security assurance.

## Governance

- Workflow: `dg/review`
- Run: `run-1794030804801701000-2`
- Approval: `approval/run-1794030804801701000-2/review-scope-approved`
- Presentation: `presentation/866328e2d083f7d3`
- Outcome: granted with persisted proof under delegated review-only authority
- Closeout: Completed, 39 events, one grant, no retries or escalations
- Validation: three focused continuation tests with the CLI probe, 74
  operational-opening tests, docs check, and diff check passed
- Outside the kernel: source inspection, test commands, docs edits, and Git
  commit; no live provider, harness, PR, merge, or release action
- No WorkReport was persisted by the review runner.
