# Trusted-Host Project-Validation Profile Plan Review

## 1. Executive Verdict

**Needs planning blocker fixes.**

`WorkflowOsProjectValidation` is the correct second closed profile, and the
plan is otherwise narrow and security-conscious. Implementation must not begin
until the plan defines how a non-passing `LocalCheckResult` becomes truthful
trusted-host workflow failure and resolves the contradiction between explicit
profile selection and existing docs-check invocation compatibility.

## 2. Scope Verification

The plan stays within one unpublished, local, synchronous, SQLite-backed,
no-write profile. It does not authorize released CLI behavior, arbitrary
commands, automatic approval, providers, network access, source writes,
SideEffect execution, report artifacts, OpenShell, nested harnesses, hosted
expansion, schemas, SDK changes, examples, or release changes.

## 3. Profile Selection Assessment

The existing `WorkflowOsProjectValidation` profile is the smallest meaningful
generalization of the accepted docs-check proof. Its fixed `workflow-os
validate` contract, stable skill identity, disabled network, no-source-write
classification, explicit executable, and explicit project root are suitable
for the next closed slice.

No new profile vocabulary or generic command registry is justified.

## 4. Identity And Root Binding Assessment

The plan correctly requires the profile's working directory to be the exact
governance project root used by the immutable execution request. That prevents
one project from authorizing validation of another project.

Canonical path equality, executable content hashing, fixed arguments,
sanitized environment, timeout, command-contract fingerprint, immutable skill
identity, and actor/run/bundle bindings form an appropriate substitution
boundary. Errors remain bounded and must not expose paths or hashes.

## 5. Current-Authority And Required-Context Assessment

The plan correctly refuses to relax the docs-check-specific explicit-zero
authorization into an open profile rule. A separate exhaustive Core decision
must prove that the project-validation operation requires no separately
modeled capability grants, availability facts, or governed-context references.

The plan should retain one important distinction during its blocker fix: the
closed handler and immutable command binding authorize the bounded local-read
operation. An empty current-authority inventory means no additional runtime
authority facts are required; it does not mean filesystem access is absent or
that missing authority is treated as granted.

Distinct source and harness identities are required and correctly planned.

## 6. Blocker 1: Failed Check Would Complete The Workflow

The plan states that the actual same-call `LocalCheckResult` is check truth and
that validation failure must produce the existing terminal-failure path. The
current trusted-host execution boundary cannot satisfy that claim unchanged.

`WorkflowOsProjectValidationLocalHandler::invoke(...)` converts every valid
`LocalCheckResult`, including `Failed` and `TimedOut`, into
`Ok(SkillOutput)`. `DocsCheckLocalHandler` has the same behavior. The accepted
trusted-host `LocalSkillAttemptExecutor` treats `Ok(SkillOutput)` as a
successful attempt. Required output validation then sees `summary` and
`local_check_status`, appends `SkillInvocationSucceeded`, and completes the
one-step workflow even when the bounded status says `failed` or `timed_out`.

That is not only a future project-validation concern. It is a latent truth-gap
in the existing trusted-host docs-check path.

### Required correction

The plan must define one Core-owned closed-profile execution adapter that:

1. obtains the exact `LocalCheckResult` once;
2. accepts only `Passed` as successful skill output;
3. maps `Failed`, `TimedOut`, and internal check failure to stable non-leaking
   terminal failure before lifecycle terminalization;
4. does not rerun the process;
5. does not persist stdout, stderr, or command transcripts;
6. preserves the existing authoritative proportional-governance APIs that
   consume non-passing check results as data; and
7. hardens both docs-check and project-validation trusted-host execution with
   regression tests.

Changing the general `LocalCheckResult` model to reject failed results is not
acceptable because authoritative governance routing legitimately needs a
structured non-passing result. The correction belongs at the trusted-host
skill-attempt adaptation boundary.

## 7. Blocker 2: Profile Selection Compatibility Is Ambiguous

The plan requires an explicit `--profile` selector, says the selector must be
an exhaustive closed match, and also requires existing docs-check arguments to
remain compatible. The current unpublished binary has no profile argument and
implicitly selects docs-check from its npm-specific inputs.

Requiring `--profile` breaks existing invocations. Omitting it while claiming
selection is always explicit contradicts the planned boundary. Inferring the
profile from whichever executable flag is present would make argument shape a
hidden authority selector.

### Required correction

Choose and document one exact compatibility rule. The smallest acceptable rule
is:

- `--profile workflow-os-project-validation` is mandatory for the new path;
- `--profile docs-check` is accepted for explicit new invocations; and
- omission is a temporary, tested compatibility alias for the already
  accepted docs-check path only.

Supplying project-validation arguments without its explicit profile, mixing
profile-specific argument families, or supplying an unknown profile must fail
before backend opening or state creation. The compatibility alias must be
documented as unpublished and must not become a released default-activation
promise.

Alternatively, the plan may intentionally break the unpublished invocation
contract and update every caller, but it must say so and remove the
compatibility claim. It may not infer profile authority from ambient files or
argument coincidence.

## 8. Lifecycle And Replay Assessment

Subject to blocker 1, reusing the accepted atomic lifecycle is correct:
invocation request with opening, invocation start with dispatch, and invocation
terminal plus workflow terminal with continuity outcome. Exact replay must
remain non-executing and non-appending.

The plan correctly rejects conversion of retryable, yielded, ambiguous, stale,
mismatched, or incomplete state into completion.

## 9. Evidence And Report Assessment

The plan accurately distinguishes:

- bounded local-check result and output reference;
- durable invocation/workflow events;
- `EvidenceReference`; and
- `WorkReport` or report artifact.

It does not overclaim command output as evidence or claim a terminal report
exists. Explicitly deferring trusted-host report composition is acceptable for
this unpublished second-profile proof, provided host output and documentation
continue to disclose that absence.

## 10. Privacy And Error Assessment

The planned redaction boundary is sufficient. The blocker fix must ensure its
non-passing-result adapter emits only stable failure codes and fixed messages.
It must not put stdout, stderr, paths, executable hashes, environment values,
or local-check summaries into workflow failure events or host errors.

## 11. Test Assessment

The planned binary-level tests are strong, but blocker 1 requires more precise
assertions:

- nonzero project-validation exit yields `SkillInvocationFailed` and
  `RunFailed`, never success/completion;
- timeout yields the same truthful terminal failure posture;
- nonzero docs-check exit is fixed by the same trusted-host adapter;
- failure events contain fixed non-leaking metadata; and
- exact replay of a failed run executes no process and appends no event.

Blocker 2 requires tests for explicit selection, the temporary docs-check
compatibility alias, mixed argument rejection, and no state creation on
selection failure.

## 12. Blockers

1. Define truthful non-passing local-check adaptation before trusted-host
   lifecycle terminalization, covering both closed profiles.
2. Define one exact profile-selection compatibility rule without hidden
   inference.

## 13. Non-Blocking Follow-Ups

- Plan trusted-host terminal WorkReport composition before released or broader
  operator adoption.
- Keep final process-entry race hardening separate from this profile slice.
- Consider removal of the unpublished docs-check compatibility alias only in a
  later explicit breaking cleanup.

## 14. Recommended Next Phase

Perform a focused documentation-only blocker fix to the plan and planning
report. Then re-review the corrected plan before any Rust implementation.

## 15. Validation

- `npm run check:docs`: passed.
- `git diff --check`: passed.
- Rust checks are not required because this review changes documentation only.

## 16. Governed Review Evidence

- Dogfood workflow: `dg/review`.
- Run: `run-1792661793646317000-2`.
- Approval:
  `approval/run-1792661793646317000-2/review-scope-approved`.
- Approval presentation: `presentation/761970ff5b3038c6`.
- Approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof.
- Phase status: `Completed`.
- Event summary: 39 events, one approval, zero retries, zero escalations, and
  proof-enforced approval presentation.
- Out-of-kernel work: source, plan, tests, and prior review inspection; review
  authoring; validation; git; and later pull-request operations are performed
  by the delegated maintainer while Workflow OS governs scope and records the
  review trail.
