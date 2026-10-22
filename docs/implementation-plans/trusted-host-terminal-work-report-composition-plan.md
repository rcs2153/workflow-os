# Trusted-Host Terminal WorkReport Composition Plan

Status: planning complete; implementation not started.

Related foundations:

- [Engineering Standard](../ENGINEERING_STANDARD.md)
- [WorkReport Contract Plan](work-report-contract-plan.md)
- [Terminal Local Report Generation Plan](terminal-local-report-generation-plan.md)
- [Trusted-Host Project-Validation Profile Plan](trusted-host-project-validation-profile-plan.md)
- [Trusted-Host Project-Validation Profile Implementation Review](../concepts/TRUSTED_HOST_PROJECT_VALIDATION_PROFILE_IMPLEMENTATION_REVIEW.md)

## 1. Executive Summary

The unpublished trusted-host application now executes two closed no-write
profiles and reconciles their terminal workflow state. It does not yet compose
a terminal `WorkReport`. This leaves the project-validation path with durable
workflow and invocation events but without the final governed handoff model
that the broader architecture expects.

The next implementation should add one private, in-memory terminal
`WorkReport` composition path for trusted-host operations. Core must compose
the report only after terminal events are durably committed and the exact run
is rehydrated. The report must cite the actual same-call local-check result,
not a caller assertion, reconstructed command output, or fabricated evidence.

This plan does not implement reports, artifacts, persistence, released CLI
behavior, provider access, writes, scheduling, arbitrary commands, nested
harnesses, or hosted expansion.

## 2. Goals

The implementation should:

- compose one validated in-memory `WorkReport` for terminal trusted-host runs;
- preserve the exact workflow, run, step, profile, command, and immutable-bundle
  binding;
- cite the actual same-call local-check result through a bounded stable
  reference;
- support successful and concrete non-passing check outcomes honestly;
- preserve workflow pass/fail semantics when report generation fails;
- reconcile exact terminal replay without rerunning the process or appending
  events;
- disclose no-write SideEffect posture and known preview limitations; and
- keep report construction inside a private Core-owned boundary.

## 3. Non-Goals

This phase must not add:

- report persistence or a `WorkReportArtifact`;
- automatic reports for every executor or workflow path;
- released CLI report rendering or export;
- report configuration, workflow schema fields, or runtime configuration;
- arbitrary report IDs, contract IDs, summaries, citations, or metadata from
  the host caller;
- raw stdout, stderr, command transcripts, parser payloads, spec contents, or
  source contents;
- another local-check profile, arbitrary command execution, scheduling, or
  default activation;
- provider calls, network access, credentials, SideEffects, or writes;
- automatic approval, proportional-governance broadening, or policy changes;
- hosted parity, OpenShell, nested harnesses, reasoning lineage, or release
  posture changes.

## 4. Current Runtime Boundary

The foreground `workflow-local-host` admits an exact operation through Core
and receives one of these bounded outcomes:

- prepared operation;
- waiting posture;
- terminal replay;
- blocked posture; or
- denied posture.

Prepared execution consumes an opaque session. Core invokes one resolved
closed profile, appends invocation lifecycle events, terminalizes the workflow,
and returns fixed host posture. The host does not construct events, choose
authority facts, or own terminal truth.

The existing `TrustedHostLocalCheckAttemptExecutor` obtains the concrete
same-call `LocalCheckResult`, converts a passing result into bounded
`SkillOutput`, and converts failed, timed-out, or internal outcomes into
terminal failure. The concrete result is not currently carried into terminal
report composition. That loss is the principal implementation seam this plan
must correct.

## 5. Composition Boundary

Add the smallest private Core-owned composition result around the existing
trusted-host terminalization path. The boundary should return:

```text
trusted-host terminal workflow result
+ optional generated WorkReport
+ optional stable non-leaking report-generation error
```

The exact Rust shape is an implementation choice, but it must remain private or
crate-private until a separate compatibility review authorizes broader
exposure. `workflow-local-host` may render only fixed report posture such as
`report.generated` or `report.generation_failed`; it must not print report
content, references, paths, or summaries.

The composition sequence must be:

1. invoke the exact closed profile once;
2. derive a bounded `LocalCheckResultReference` from that same-call result when
   a concrete result exists;
3. append and commit invocation and terminal workflow events atomically under
   the existing lifecycle boundary;
4. rehydrate the exact terminal run and immutable bundle;
5. verify that the result reference and terminal events bind to the same run,
   workflow, step, profile, command, and bundle;
6. call the existing validated WorkReport constructors; and
7. return report posture without rewriting terminal workflow truth.

The host must not be able to inject a local-check result, stable reference,
terminal status, report section, or evidence claim.

## 6. Same-Call Check Truth

The actual `LocalCheckResult` is the authoritative check result for this path.
Implementation must capture it before it is reduced to `SkillOutput` or a
terminal failure.

For `Passed`, Core should derive one `LocalCheckResultReference` from the exact
result and use the existing bounded output reference where available.

For `Failed` and `TimedOut`, Core should retain enough bounded result data to
derive an honest reference before terminal failure. The report must preserve
the failed workflow status and describe the check as non-passing. It must not
convert failure into a completed run or a successful validation claim.

For an internal handler error that produces no concrete `LocalCheckResult`,
the report must state that the check reference is unavailable and include a
bounded known limitation. It must not synthesize a passing, failed, or timed-out
result.

The implementation must never rerun the command to produce report evidence.

## 7. Report Identity And Contract

There is no repository-wide canonical local terminal report contract identity
to reuse. The hosted `hosted-terminal-no-write` contract describes a distinct
runtime and must not be borrowed.

The first implementation should use a private Core-owned contract identity:

```text
trusted-host-terminal-no-write
version: v1
```

The report ID must be deterministically derived from the immutable trusted-host
binding, including the run ID, bundle root, selected profile, step ID, command
ID, and concrete check-result identity when available. Exact replay must derive
the same report ID. The derivation must not contain raw paths, command output,
payloads, or secrets.

The generated timestamp must come from the already-bound terminal invocation
or terminal workflow event. It must not use a new ambient clock read during
replay. The generated actor should be a fixed validated system actor for the
local trusted host. Sensitivity should default to `Internal`, and redaction
metadata should use the validated empty representation unless Core has actual
bounded redaction facts to disclose.

## 8. Required Report Sections

The report must use existing `WorkReport`, `WorkReportSection`, and
`WorkReportCitation` constructors and include every v1 section:

- work performed: bounded terminal profile and workflow posture;
- evidence considered: the exact local-check result reference when available;
- decisions made: existing policy and approval event references only;
- policy gates evaluated: exact durable policy event references;
- approvals: exact durable approval references when the workflow required one;
- validation and quality checks: the concrete local-check reference and status;
- side effects: explicit none, skipped, or unsupported;
- incomplete or deferred work: no persistence, artifact, or released surface;
- known limitations: unpublished host and bounded evidence limitations;
- risks: report generation is not workflow success and raw output is absent;
- operator handoff notes: exact replay and inspection posture only.

Absent references must become explicit bounded unavailable text. The
implementation must not fabricate IDs or copy diagnostic messages, command
output, policy text, approval reasons, or source material into section text.

## 9. Citation Policy

Allowed citations are limited to stable references already produced by Core:

- the exact `LocalCheckResultReference` derived from the same call;
- workflow event IDs from the rehydrated terminal run;
- policy decision event IDs;
- approval decision references where a stable model already exists; and
- existing bounded output references that do not contain payload data.

The report must not create an `EvidenceReference` implicitly. Evidence
promotion remains a separate governed phase. Caller-supplied citation arrays
are not accepted by this private path.

## 10. Terminal Status And Failure Semantics

Report composition occurs only for a terminal `Completed`, `Failed`, or
`Canceled` run supported by the current runtime. Waiting, blocked, prepared,
and denied admission postures do not produce terminal reports.

Report-generation failure must not:

- change a completed run to failed;
- change a failed run to completed;
- append a replacement terminal event;
- retry the process;
- alter approval or policy decisions; or
- mutate the immutable bundle.

The operation result should preserve the terminal run and carry one stable
non-leaking report error. Errors must use fixed codes and must not include raw
identifiers, paths, output, hashes, payloads, policy text, approval reasons, or
secret-like values.

## 11. Replay And Recovery

Exact terminal replay must:

- rehydrate the existing terminal run and exact immutable bundle;
- reconstruct or retrieve the same bounded check-result reference without
  process execution;
- derive the same report identity and bounded contents;
- append no workflow or invocation events; and
- return the same generated or generation-failed posture deterministically.

If the concrete bounded check-result reference is not durably reconstructible
from accepted state, implementation must not infer it from output-ref text.
That condition is an implementation blocker for replay and requires a narrow
payload-free durable reference record or an explicit report limitation. It
does not authorize storing raw `LocalCheckResult` output.

Crash after terminal commit but before report composition must reconcile from
terminal state. Because this phase is in-memory only, it does not claim durable
report availability across process restarts.

## 12. Privacy And Redaction

The path must never store or copy into a report:

- stdout, stderr, or command transcripts;
- source, spec, parser, provider, or network payloads;
- project or executable paths;
- environment values;
- credentials, authorization headers, private keys, or tokens;
- policy source text or approval reasons; or
- caller-authored unbounded report prose.

Debug, Display, errors, fixed host output, serde, and replay failures must
remain safe. Reports are `Internal` even though the supported profiles are
read-only and no-write.

## 13. Contract Enforcement

The private composition path enforces one Core-owned contract, not a
workflow-declared contract. Full `WorkReportContract` instance validation,
workflow schema exposure, and user-selectable contract requirements remain
deferred.

Existing model constructors remain the final shape and redaction gates. The
private boundary must additionally verify trusted-host identity and
same-call-result bindings that the generic WorkReport model cannot know.

## 14. Test Plan

Focused implementation tests must prove:

- passing docs-check and project-validation runs produce valid in-memory
  terminal reports;
- failed and timed-out concrete results preserve failed workflow truth and are
  disclosed as non-passing checks;
- an internal error with no concrete result creates no fake check citation;
- all required v1 sections are present;
- report identity is deterministic and bound to the exact immutable operation;
- exact replay performs no process invocation and appends no events;
- run, workflow, step, profile, command, result, bundle, and actor substitution
  fail closed;
- report failure preserves workflow pass/fail status;
- waiting, blocked, prepared, and denied postures create no terminal report;
- no raw output, paths, payloads, environment values, or secrets enter report,
  Debug, serde, host output, or errors;
- no report artifact, filesystem report, or new persistence record is created;
- existing docs-check and project-validation behavior remains unchanged; and
- the complete Core, local-host, and workspace suites pass.

## 15. Implementation Sequence

Implement in one bounded vertical slice:

1. retain or immediately derive the bounded same-call result reference inside
   `TrustedHostLocalCheckAttemptExecutor`;
2. add one private terminal composition result carrying terminal workflow truth
   and report posture;
3. compose through existing WorkReport constructors after terminal commit and
   exact rehydration;
4. reconcile exact terminal replay without process execution;
5. expose only fixed report posture from `workflow-local-host`;
6. add focused substitution, replay, privacy, and regression tests;
7. run full repository validation; and
8. perform focused maintainer/security review before any artifact or released
   surface is considered.

## 16. Blockers To Resolve During Implementation

Before implementation can be accepted, the code must resolve these questions
without widening scope:

- how a bounded check-result reference survives terminal commit for exact
  replay without persisting raw output;
- which exact terminal event timestamp is the report generation timestamp;
- which existing approval reference can be cited without reconstructing
  approval reason text; and
- how a report-generation error is returned through the opaque operation
  result without weakening fixed host output.

These are implementation design obligations, not authorization for a new
general persistence model.

## 17. Final Recommendation

The next implementation phase should be **trusted-host terminal WorkReport
composition, in-memory only**, followed by focused maintainer/security review.

Do not implement report artifacts, report persistence, released CLI report
output, another profile, provider access, network access, scheduling, arbitrary
commands, automatic approval, nested harnesses, hosted parity, or writes.
