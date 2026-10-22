# Trusted-Host Terminal WorkReport Composition Plan Report

## 1. Executive Summary

Planning is complete for one private in-memory terminal `WorkReport`
composition path over the accepted trusted-host no-write profiles. The plan
requires Core to preserve same-call local-check truth, commit terminal workflow
state, rehydrate the exact run and immutable bundle, and only then construct a
validated report.

No runtime behavior was implemented in this phase.

## 2. Scope Completed

This phase:

- inspected the trusted-host application, supervisor, executor, lifecycle, and
  WorkReport generation boundaries;
- identified where the concrete same-call `LocalCheckResult` is currently
  reduced before report composition;
- defined a private Core-owned terminal composition result;
- defined deterministic report identity and fixed contract posture;
- defined success, non-passing check, internal-error, and replay semantics;
- defined bounded citations, sections, privacy requirements, and tests; and
- reconciled the active delivery queue after acceptance of the second closed
  no-write profile.

## 3. Scope Explicitly Not Completed

The phase did not add:

- WorkReport generation code;
- report persistence or artifacts;
- released CLI report rendering or export;
- workflow schemas or report configuration;
- another local-check profile or arbitrary commands;
- provider calls, network access, credentials, SideEffects, or writes;
- scheduling, automatic approval, nested harnesses, OpenShell, hosted parity,
  reasoning lineage, or release posture changes; or
- execution of the dependency-blocked draft-pull-request live sandbox smoke.

## 4. Runtime Boundary Finding

The trusted-host executor obtains the authoritative same-call
`LocalCheckResult`, but the current terminal path retains only bounded skill
output or terminal failure. A trustworthy report cannot be reconstructed from
an output-reference string alone. The implementation must carry or durably
reconstruct a bounded Core-derived `LocalCheckResultReference` while never
persisting raw stdout, stderr, summaries, or command transcripts.

## 5. Identity And Contract Decision

No canonical local terminal report contract exists across the repository. The
plan therefore selects one private Core-owned contract identity,
`trusted-host-terminal-no-write` at `v1`, instead of borrowing the distinct
hosted contract or accepting caller-provided report metadata.

Report identity must be derived deterministically from the exact run, immutable
bundle, selected profile, step, command, and concrete check result when one
exists. Exact replay must derive the same identity without running a process.

## 6. Workflow Semantics

Report construction follows terminal event commit and exact run rehydration.
Report-generation failure is secondary posture: it cannot rewrite a completed
run as failed, a failed run as completed, or append replacement terminal
events. Waiting, blocked, prepared, and denied operations do not produce a
terminal report.

## 7. Privacy And Redaction

The plan forbids raw command output, paths, source/spec/parser/provider
payloads, environment values, credentials, policy source text, approval
reasons, and caller-authored unbounded prose. Debug, serde, fixed host output,
and errors must remain bounded and non-leaking. Reports default to `Internal`.

## 8. Test Posture

The future implementation must cover both closed profiles, passing and
non-passing concrete results, missing-result internal errors, deterministic
identity, exact replay, substitution rejection, required sections, workflow
semantic preservation, non-leakage, absence of artifacts, and complete
regression suites.

## 9. Commands Run And Results

The planning phase ran under:

- workflow ID: `dg/d`
- run ID: `run-1792663912502635000-2`
- approval ID:
  `approval/run-1792663912502635000-2/planning-approved`
- approval outcome: granted by `user/delegated-maintainer`
- presentation proof: `presentation/c74244be9f5065ac`

Validation results:

- `npm run check:docs`: passed
- `git diff --check`: passed
- governed phase close: passed with terminal `Completed` status

The governed event trail contains 39 events, one approval request and grant,
zero retries, and zero escalations. Approval presentation enforcement was
proof-enforced and the approval event trail contains the presentation marker.

Repository reads, documentation edits, shell validation commands, and git
inspection occurred outside the kernel under the approved planning scope. No
Rust or TypeScript code check was run because this phase changes documentation
only. No git commit, push, pull request action, provider call, runtime report,
or report artifact was performed by the kernel. Missing report composition was
documented rather than simulated.

## 10. Remaining Limitations

- The trusted-host application still creates no WorkReport.
- No WorkReport artifact or durable report exists.
- A bounded same-call check-result reference is not yet available across exact
  terminal replay.
- The trusted host remains unpublished and supports only two closed no-write
  profiles.
- The dedicated GitHub draft-pull-request live sandbox smoke remains blocked
  on a dedicated sandbox credential and does not block this no-write lane.

## 11. Recommended Next Phase

Implement **trusted-host terminal WorkReport composition, in-memory only**, then
perform focused maintainer/security review before considering persistence,
artifacts, released surfaces, another profile, or any write-capable expansion.
