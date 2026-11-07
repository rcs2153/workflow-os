# Corrected Fixture Continuation Security Review

Reviewed head: `9d6f6466`. Verdict: not accepted for the independent harness
corpus. The fixture success path is real; two authority checks need correction.

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
