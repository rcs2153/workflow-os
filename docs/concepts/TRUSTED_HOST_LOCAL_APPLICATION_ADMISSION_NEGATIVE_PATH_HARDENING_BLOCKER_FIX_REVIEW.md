# Trusted-Host Admission Negative-Path Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; trusted-host roadmap work may proceed.**

The executable-identity implementation fixes the substantive security defect.
The private commitment now preserves exact encoded path bytes, binds a
streaming executable content identity, and rejects same-path replacement before
the process runner is invoked. One regression required by the original review
was initially absent: executable identity becoming unavailable after
resolution was implemented but not tested. The governed fix adds that direct
pre-runner regression without changing production behavior.

## 2. Scope Verification

The fix remains within the accepted unpublished docs-check slice. It adds no
profile, public CLI, discovery, scheduling, provider behavior, write,
automatic approval, workflow completion, hosted parity, nested harness,
OpenShell, schema, example, or release behavior.

## 3. Exact Identity Assessment

The revised `resolved_execution_fingerprint` uses domain-separated SHA-256 and
fixed-width framing for every private field. Executable and working-directory
paths contribute `OsStr::as_encoded_bytes`, avoiding the lossy Unicode
conversion identified by the original review. Arguments, ordered environment
entries, timeout, and the resolved executable content hash remain bound into
the same private fingerprint.

Each supported handler computes the executable content hash when it is
resolved and revalidates the hash before constructing every process request.
The trusted-host executor commitment therefore changes when executable bytes
change at the same path. Only the resulting `SpecContentHash` crosses the
admission boundary.

## 4. Replay And Execution Assessment

The foreground application proof now replaces executable contents at the same
path and demonstrates fail-closed replay. The direct handler proof confirms
the injected runner receives no request after replacement. Existing unchanged
replay remains idempotent, and actor, correlation, approval-wait, topology,
SQLite, and bounded-output proofs remain intact.

The implementation still has a narrow trusted-host race between final content
validation and operating-system process entry. The report discloses that
limitation and makes no platform-attestation claim. Descriptor-backed or
sandbox-attested launch identity remains separate future work.

## 5. Privacy And Error Assessment

Handler Debug output redacts executable paths and content hashes. Changed
identity returns the stable
`local_check.profile.handler.executable_identity_changed` code without paths,
bytes, digests, environment values, or command output. Unavailable identity is
mapped to the similarly bounded
`local_check.profile.handler.executable_identity_unavailable` code.

The public model and serialization shape are unchanged.

## 6. Test Assessment

The following required proofs are present:

- same-path executable replacement fails before runner invocation;
- exact unchanged application replay remains idempotent;
- distinct non-UTF-8 path byte sequences produce distinct commitments;
- application errors do not expose executable contents or local paths; and
- the original six application-boundary postures remain covered.

The final required proof is now present:

- after a valid handler resolves an executable, making that executable identity
  unavailable and prove invocation fails with
  `local_check.profile.handler.executable_identity_unavailable`, before runner
  invocation and without path or content leakage.

The regression removes the executable after successful handler construction,
then proves the stable code, zero runner invocation, and bounded error text.

## 7. Validation Assessment

The implementation passed format, clippy, focused tests, documentation checks,
integration checks, and a serialized full workspace test run. Unrelated
timing-sensitive scheduler tests failed in ordinary parallel runs and each
passed immediately in isolation. The serialized workspace run passed in full;
the final test-only follow-up also passed its focused test, format, clippy,
documentation, and integration gates. A deliberately concurrent integration build changed the executable
under test and correctly triggered the new identity guard; independent runs of
both gates passed.

## 8. Blockers

None.

## 9. Non-Blocking Follow-Ups

- Keep the final validation-to-process-entry race explicit until a stronger
  trusted-host launch boundary is separately designed.
- Avoid compiling or replacing the admitted executable concurrently with a
  test intended to prove its immutable identity; the resulting rejection is
  correct behavior, not a reason to weaken the guard.

## 10. Recommended Next Phase

Proceed to the next trusted-host roadmap phase identified after PR integration.
Do not treat this acceptance as authorization for public runtime surfaces or
for removing the disclosed final validation-to-process-entry race.

## 11. Fix Verification

The one-test governed blocker phase added no production behavior. Focused and
repository validation passed, and the new regression proves post-resolution
unavailability fails before runner invocation without leaking the path or
fixture contents.

- Workflow: `dg/blocker`
- Run: `run-1791501089129242000-2`
- Approval: `approval/run-1791501089129242000-2/fix-approved`
- Presentation proof: `presentation/4013c801a6dad79f`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations

## 12. Governed Review Record

- Workflow: `dg/review`
- Run: `run-1791500256676192000-2`
- Approval: `approval/run-1791500256676192000-2/review-scope-approved`
- Presentation proof: `presentation/e1daf8fd9931ae3d`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations

Repository inspection, testing, review writing, and git/PR work were performed
by the delegated maintainer outside the kernel. The kernel governed scope,
approval, sequencing, and durable review evidence.
