# Trusted-Host Terminal WorkReport Composition Plan Blocker Fix Report

## 1. Executive Summary

The terminal WorkReport composition plan's replay-source blocker is fixed. The
plan now requires one Core-authored, payload-free
`LocalCheckResultRecorded` workflow event for every concrete passing, failed,
or timed-out trusted-host result.

The event commits atomically with invocation and run terminalization and is the
only authoritative source for fresh report composition and exact replay. A
missing result binding can no longer be downgraded to a known limitation.

No runtime code was implemented in this phase.

## 2. Original Blocker

The original plan required deterministic exact replay but allowed an
unreconstructible check-result reference to become either a durable record or
an explicit report limitation. Those outcomes are not equivalent. A
limitation-only replay could omit evidence present during fresh execution,
especially for failed and timed-out checks whose current terminal events carry
no bounded result reference.

## 3. Fix Approach

The amended plan selects one mechanism:

- derive a validated `LocalCheckResultReference` from the actual same-call
  result;
- project its payload-free binding into one
  `LocalCheckResultRecorded` workflow event;
- append that event in the same compare-and-append lifecycle batch as the
  matching invocation terminal event and run terminal event; and
- reconstruct the exact reference from that event for both fresh composition
  and replay.

No caller may supply or replace the binding, and output-reference text is not
an evidence source.

## 4. Minimum Binding

The planned event binds validated result identity and status to the exact run,
workflow, version, step, invocation attempt, trusted-host profile, canonical
command, immutable bundle root, optional bounded output reference, terminal
event relationship, and Core-provided result timestamp.

Missing, duplicate, mismatched, or conflicting bindings fail closed.

## 5. Atomicity And Recovery

A trusted-host operation with a concrete check result cannot commit only part
of its terminal truth. Result binding, invocation terminal event, and run
terminal event must commit atomically. Crash recovery after terminal commit but
before in-memory report composition rehydrates the committed result event and
derives the same report identity without process execution or event append.

## 6. Privacy And Redaction

The event is payload-free. It excludes stdout, stderr, summaries, command
transcripts, paths, executable fingerprints, environment values, source/spec
contents, and provider payloads. It is durable execution evidence, not a raw
check result, WorkReport, or report artifact.

## 7. Test Additions

The plan now explicitly requires tests for:

- atomic result, invocation, and run terminal commit;
- post-terminal pre-report crash recovery;
- passing, failed, and timed-out result reconstruction;
- missing, duplicate, ID-conflicting, status-conflicting, and
  binding-conflicting replay rejection; and
- absence of output, summaries, paths, and payloads from the durable event.

## 8. Scope Preserved

The fix does not authorize WorkReport generation code, report persistence,
report artifacts, released CLI behavior, raw result persistence, another
profile, arbitrary commands, providers, writes, hosted expansion, schemas,
examples, or release changes.

## 9. Validation

The blocker fix ran under:

- workflow ID: `dg/blocker`
- run ID: `run-1792664143494542000-2`
- approval ID: `approval/run-1792664143494542000-2/fix-approved`
- approval outcome: granted by `user/delegated-maintainer`
- presentation proof: `presentation/7f202a0fda216b70`

Validation results:

- `npm run check:docs`: passed
- `git diff --check`: passed
- governed phase close: passed with terminal `Completed` status

The governed event trail contains 39 events, one approval request and grant,
zero retries, and zero escalations. Approval presentation was proof-enforced
and its event marker is present.

Repository reads, documentation edits, shell validation, and git inspection
occurred outside the kernel under the approved blocker-fix scope. Rust and
TypeScript checks were skipped because no runtime code changed. The kernel did
not edit files, commit, push, open a pull request, generate a WorkReport, or
create an artifact. The future runtime binding is documented, not simulated.

## 10. Recommended Next Phase

Perform a focused maintainer/security re-review of the corrected plan. If
accepted, proceed to the bounded in-memory implementation vertical slice.
