# Dogfood Phase-Close Proof-Enforcement Disclosure Report

## 1. Executive Summary

This phase implemented bounded dogfood `phase-close` disclosure for approval-presentation proof posture.

Material dogfood `phase-start` already persists approval-presentation proof and prints a proof-enforced approval command. `phase-close` now scans the local approval-presentation store for records matching the governed run and reports whether proof records are present, ambiguous, missing, or unreadable.

The implementation is intentionally repo-local helper behavior only. It does not change public approval semantics, add approval-card UI, add schemas, enable writes, add hosted behavior, implement reasoning lineage, or change release posture.

## 2. Scope Completed

- Added approval-presentation proof-record discovery to `scripts/self-governed-benchmark.mjs`.
- Updated `phase-close` output with:
  - `approval_presentation_enforcement`;
  - `approval_presentation_records`;
  - bounded `approval_presentation_id`;
  - bounded `approval_presentation_content_hash`;
  - bounded `approval_presentation_approval_id`;
  - `approval_presentation_event_marker`;
  - `approval_presentation_note`.
- Added focused dogfood helper tests for:
  - a matching proof record with a granted approval;
  - an absent proof store;
  - ambiguous matching proof records.
- Updated roadmap and self-governed build benchmark documentation.
- Added this end-of-phase report.

## 3. Scope Explicitly Not Completed

- No public `workflow-os approve` behavior change.
- No automatic approval.
- No hidden approval.
- No public approval-card UI.
- No workflow schema changes.
- No examples.
- No provider writes.
- No side effects.
- No report artifact writes.
- No hosted or distributed runtime behavior.
- No reasoning lineage.
- No release posture changes.

## 4. Helper Behavior Summary

`phase-close` now reads status and inspect output as before, then scans:

```text
<state-dir>/approval_presentations/records
```

The scanner matches approval-presentation records by `run_id`.

If exactly one matching record is found, `phase-close` prints bounded identifiers for the presentation, content hash, and approval ID. If no store exists, no matching record exists, records cannot be read, or more than one record matches, it reports that condition explicitly instead of simulating proof.

## 5. Proof Disclosure Boundary

The current CLI inspect output reports event kinds but does not expose approval IDs or presentation proof-use fields on approval events.

Because of that, `phase-close` does not overclaim that the event trail independently proves presentation use. When a matching proof record and a granted approval are both visible, it reports:

```text
approval_presentation_enforcement: proof_record_present_granted_approval_seen
approval_presentation_event_marker: not_available
approval_presentation_note: proof record matched the run; inspect output does not yet expose a presentation proof-use marker
```

That is the honest boundary for this phase. A later phase may add a durable approval-event proof marker if needed.

## 6. Redaction And Privacy

The new disclosure only prints bounded identifiers and hashes from approval-presentation records. It does not print work summaries, approved scope text, strict non-goals, validation expectations, provider payloads, command output, raw source/spec contents, chat transcripts, screenshots, tokens, private keys, or secret-like metadata.

Read/parse failures are reported with stable non-leaking posture text rather than raw file paths or record contents.

## 7. Test Coverage Summary

Focused dogfood helper tests cover:

- proof discovery for a matching record;
- proof discovery when the approval-presentation store is absent;
- proof discovery when matching records are ambiguous;
- existing phase-start proof persistence and proof-enforced approval command shape;
- existing approval handoff/copy-safe request behavior;
- existing secret-like input rejection.

## 8. Governed Phase Summary

- dogfood workflow ID: `dg/implement`
- run ID: `run-1783603001038540000-2`
- approval ID: `approval/run-1783603001038540000-2/implementation-approved`
- approval-presentation proof: persisted before approval
- presentation ID: `presentation/99f31dfaa34dab09`
- approval outcome: granted by delegated maintainer through the proof-enforced dogfood approval command
- phase-close status: `Completed`
- phase-close event summary: 39 events total; 1 approval request; 1 approval grant; 6 scheduled steps; 6 skill invocations requested, started, and succeeded; 0 retries; 0 escalations
- phase-close proof disclosure: `proof_record_present_granted_approval_seen`

The dogfood runner coordinated governance only. Repo edits, validation commands, git operations, PR actions, and this report were performed by the executor outside the kernel and are disclosed here.

## 9. Commands Run

- `npm run test:dogfood-helper` - passed
- `npm run dogfood:benchmark -- phase-close run-1783603001038540000-2 --phase implementation` - passed
- `npm run check:docs` - passed
- `git diff --check` - passed

## 10. Remaining Known Limitations

- CLI inspect output does not yet expose approval IDs or presentation proof-use markers on approval events.
- `phase-close` can report matching proof-record posture, but it intentionally does not claim stronger event-level proof than the current inspect output exposes.
- The hidden dogfood approval command remains repo-local dogfood tooling, not public approval-card UX.
- Dogfood approval freshness/max-age policy remains deferred.

## 11. Recommended Next Phase

Recommended next phase: dogfood phase-close proof-enforcement disclosure review.

The review should verify the disclosure is accurate, bounded, non-leaking, and honest about the current event-marker limitation before any broader approval event, UI, or public approval-card work.
