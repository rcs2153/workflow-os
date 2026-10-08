# Trusted-Host Local Application Admission Negative-Path Hardening Report

## 1. Executive Summary

The unpublished trusted-host local application admission slice now has focused
application-boundary negative-path coverage. The real `workflow-local-host`
foreground binary proves bounded approval wait, unsupported-topology rejection,
durable identity replay rejection, resolved executable substitution rejection,
incompatible SQLite rejection, and non-leaking failure output.

The tests exposed one real security defect: the durable executor commitment
bound the logical command contract but not the exact resolved executable path.
The implementation now adds a private resolved-execution fingerprint covering
the executable, arguments, working directory, sanitized environment, and
timeout. Only its digest enters the executor commitment.

## 2. Scope Completed

- Added application tests through the actual unpublished foreground binary.
- Proved approval-gated work returns a bounded waiting posture without
  preparing or executing an operational window.
- Proved unsupported multi-step topology fails before run creation.
- Proved actor and correlation substitutions fail without new durable events.
- Proved replay with a different resolved executable fails before execution.
- Proved incompatible SQLite input returns a bounded state error without path
  disclosure.
- Preserved the existing successful admission and exact idempotent replay proof.
- Bound the exact resolved local process request into the private trusted-host
  executor commitment.

## 3. Scope Explicitly Not Completed

This phase does not add another profile, public CLI or SDK exposure, automatic
discovery, scheduling, background execution, provider calls, writes, automatic
approval, workflow completion, hosted parity, nested harnesses, OpenShell,
workflow schema changes, examples, or release posture changes.

Policy denial and pre-entry cancellation are not fabricated at the binary
boundary. The foreground application has no caller-controlled policy engine or
cancellation injection seam. Existing lower-level policy, cancellation,
preparation, and ownership tests remain the authoritative proofs for those
postures.

## 4. Resolved Execution Commitment Fix

`LocalCheckCommandContract` intentionally describes the canonical logical
command, such as `npm run check:docs`. The application resolves that token to
an explicit executable and repository boundary. Previously, replay compared a
commitment derived only from the logical contract, so two distinct executable
paths with the same logical contract produced the same executor commitment.

`ResolvedExplicitLocalCheckProfile::resolved_execution_fingerprint` now builds
a domain-separated private commitment over:

- the exact executable path;
- the fixed argument vector;
- the exact working directory;
- the sanitized environment names and values; and
- the timeout.

The trusted-host executor commitment combines this digest with the existing
logical command-contract fingerprint and skill identity. Raw paths and
environment values are not serialized, displayed, included in errors, or
placed in reports.

## 5. Application Proofs

The focused binary suite now proves:

- a valid approval requirement yields `admission.waiting_approval` and a
  durable `WaitingForApproval` run with no operational opening;
- an unsupported dogfood workflow topology yields the fixed unsupported
  failure category and no run events;
- changed actor and correlation values fail closed, do not append events, and
  are absent from output;
- a changed resolved executable fails closed even when the logical docs-check
  contract is unchanged;
- invalid SQLite bytes yield only `state.unavailable`, without the database
  path; and
- the established success case still executes once and replays idempotently.

## 6. Privacy And Error Posture

Application stdout and stderr remain closed posture strings. Tests use
secret-like actor text, changed correlation text, alternate executable names,
and a local database path as non-leakage markers. None appear in failure
output. The new resolved-execution material exists only long enough to derive a
`SpecContentHash`; only the hash participates in durable admission checks.

## 7. Validation

The focused application suite passed with six tests. Full repository validation
also passed:

- `cargo fmt --all --check`: passed;
- `cargo clippy --workspace --all-targets -- -D warnings`: passed;
- `cargo test --workspace`: passed;
- `npm run check:docs`: passed;
- `npm run check:integrations`: passed; and
- `git diff --check`: passed.

Cargo validation used an isolated target directory to avoid the repository's
known shared-target contention. No test was skipped beyond the suite's existing
opt-in live provider proof.

## 8. Remaining Limitations

- The application surface remains unpublished and unstable.
- Only the exact docs-check workflow/profile is admitted.
- Policy denial and pre-entry cancellation remain lower-level proofs.
- The operational result still does not complete the workflow.
- No public product surface or broader trusted-host adoption is authorized.

## 9. Governed Phase Record

- Workflow: `dg/implement`
- Run: `run-1791460465864366000-2`
- Approval: `approval/run-1791460465864366000-2/implementation-approved`
- Presentation proof: `presentation/a6378143ff3f89f0`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations

Repository edits, shell commands, validation, and the forthcoming git/PR
operations were performed by the delegated maintainer outside the kernel. The
kernel governed scope, approval, sequencing, and durable phase evidence; it did
not execute those external operations.

No `.workflow-os` runtime state is part of the implementation change set.

## 10. Recommended Next Phase

Perform a focused maintainer/security review of this negative-path hardening.
The review should verify the resolved-execution commitment, replay behavior,
bounded output, absence of false binary-level proofs, and regression coverage.

Do not broaden profiles, runtime surfaces, providers, writes, scheduling,
automatic approval, workflow completion, hosted behavior, nested harnesses,
OpenShell, schemas, examples, or release posture before that review.
