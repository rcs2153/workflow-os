# Trusted-Host Project-Validation Profile Implementation Review

## 1. Executive Verdict

**Accepted with non-blocking follow-ups.**

The implementation satisfies the bounded second-profile objective and fixes
the shared trusted-host result-truth defect without creating a generic command
host. Core remains the owner of profile resolution, immutable binding,
current-authority proof, operational preparation, lifecycle events, and
terminal workflow truth.

No blocker prevents acceptance. One test-depth follow-up remains: timed-out
result terminalization is proven through the private trusted-host adapter with
an injected runner, not through an actual foreground process waiting for the
closed profile's 120-second timeout.

## 2. Scope Verification

The phase stayed within the approved unpublished no-write scope. It added:

- one already-modeled closed profile;
- one exact host selector;
- one shared private local-check attempt adapter;
- profile-specific explicit-zero source identities;
- focused binary and Core tests; and
- status documentation and an implementation report.

It did not add arbitrary commands, new command contracts, automatic approval,
runtime configuration, scheduler behavior, provider access, network access,
source writes, SideEffects, WorkReports, artifacts, schemas, examples,
released CLI behavior, hosted parity, OpenShell, or nested harness execution.

## 3. Selector And Host Boundary Assessment

The selector contract is exact and conservative:

- project validation requires
  `--profile workflow-os-project-validation`;
- docs-check accepts explicit `--profile docs-check`;
- omission remains a temporary docs-check-only compatibility alias; and
- unknown selectors, implicit project-validation arguments, and mixed
  profile-specific argument families fail before SQLite state creation.

The host accepts only the closed profile-specific executable inputs. It does
not accept command text, arbitrary arguments, environment overrides, timeout
overrides, output bounds, handlers, or authority facts.

The parser and actual-binary tests make the compatibility posture explicit
rather than relying on argument coincidence.

## 4. Governed-Root And Execution Binding Assessment

Project validation resolves its working directory from the governance project
root already used by the execution request. Core then canonicalizes and
compares the resolved profile root with the governed root before operational
preparation.

The direct substitution test proves that distinct valid project roots fail
with `local_check.profile.governed_root_mismatch` without leaking either path.
The existing executable-content and resolved-execution commitments continue to
bind executable identity, fixed arguments, working directory, environment,
timeout, and command contract. Same-path executable replacement remains
covered and fails closed.

This phase does not claim to eliminate the final validation-to-process-entry
filesystem race, and the implementation report states that limitation.

## 5. Current-Authority Assessment

The implementation does not broaden the old docs-check condition into an open
profile rule. `explicit_zero_required_context_authorization` exhaustively
matches the two closed profile identifiers and requires each profile's exact
canonical command contract.

Source registration now includes the profile identifier in its configuration
commitment. Fresh project-validation runs receive distinct stable harness and
source identities. Empty grant, availability, and governed-context inventories
remain proof that this exact closed operation requires no additional current
authority; they are not synthetic authorization and cannot satisfy other
governance gates.

## 6. Result Truth And Lifecycle Assessment

`TrustedHostLocalCheckAttemptExecutor` invokes the resolved profile exactly
once. Only `LocalCheckResultStatus::Passed` maps to successful skill output.
`Failed`, `TimedOut`, and internal handler failure map to the existing terminal
failure outcome.

The supervisor and workflow lifecycle remain unchanged. Binary tests prove a
nonzero docs-check and a failed project-validation process produce durable
`SkillInvocationFailed` and `RunFailed` state, never
`SkillInvocationSucceeded` or `RunCompleted`. Failed-run replay returns the
durable terminal posture without another invocation or appended event.

The timeout unit test uses the same private adapter with an injected timed-out
process result and proves terminal failure. This is sufficient to accept the
behavioral boundary, but an eventual deterministic clock or shorter reviewed
test profile would provide stronger foreground-process coverage without adding
a 120-second ordinary test.

## 7. Privacy And Error Assessment

The implementation preserves fixed host output and stable bounded errors.
Errors and durable failure events do not include:

- project or executable paths;
- executable hashes or execution fingerprints;
- stdout, stderr, summaries, or command transcripts;
- source or parser payloads;
- environment values;
- policy text or approval reasons; or
- credentials or token-like values.

Tests use a private stderr marker and assert that it is absent from both host
output and rehydrated run Debug output. Root-substitution tests likewise prove
that neither private path appears in the error.

## 8. Replay And Compatibility Assessment

Fresh execution uses the new profile-specific current-authority registration
and private result adapter. Existing-run admission reconstructs immutable
workflow and skill identity, checks actor and invocation bindings, verifies the
current executor commitment, and returns terminal replay before any process
entry when the run is already terminal.

Passing docs-check behavior and omitted-profile compatibility remain active.
The intentional correction is limited to non-passing docs-check outcomes,
which can no longer be represented as successful trusted-host attempts.

## 9. Report And Evidence Assessment

The host still does not generate or persist a `WorkReport` or report artifact.
Workflow and invocation events plus bounded output references remain audit
state, not copied evidence or a final governed handoff artifact. The
implementation report says this clearly and recommends separate terminal
report composition before released or broader trusted-host use.

## 10. Test Quality Assessment

Coverage is strong for the bounded slice:

- actual foreground project-validation success and failure;
- docs-check failure truth correction;
- failed terminal replay;
- explicit and omitted-selector compatibility;
- implicit, unknown, and mixed-profile rejection before state creation;
- canonical governed-root substitution rejection;
- same-path executable replacement;
- waiting approval, unsupported topology, required-output failure, actor/run
  substitution, and incompatible SQLite posture; and
- complete existing Core, local-host, and workspace regression suites.

Non-blocking gaps:

- the actual binary does not wait through the canonical 120-second timeout;
- no new profile-specific binary policy-denial test was added because the
  unchanged lower-level admission policy boundary already covers denial; and
- there is no terminal WorkReport composition test because that behavior is
  explicitly absent.

These gaps do not weaken the accepted command, authority, or terminal-truth
boundaries.

## 11. Documentation Assessment

The plan, roadmap, and implementation report consistently state that:

- exactly two closed unpublished profiles are supported;
- project validation is bound to the governed project;
- non-passing checks fail the trusted-host attempt;
- the docs-check omission alias is temporary and unpublished;
- no WorkReport artifact exists;
- no arbitrary command, provider, write, scheduler, hosted, OpenShell, or
  nested-harness behavior is implemented; and
- the final filesystem race is not claimed as solved.

No dangerous overclaim was found.

## 12. Blockers

None.

## 13. Non-Blocking Follow-Ups

- Plan terminal WorkReport composition before making the trusted-host path a
  released operator surface or expanding it materially.
- Add deterministic end-to-end timeout coverage if a reviewed test clock or
  shorter closed test profile becomes available.
- Remove the unpublished docs-check omission alias only through an explicit
  compatibility cleanup phase.
- Keep final validation-to-process-entry race hardening separate.

## 14. Recommended Next Phase

Return to the authoritative Active Delivery Queue after this review is merged.
The dependency-tracked draft-pull-request live sandbox proof may run when its
credential is available. Otherwise, select the next already-documented
no-write runtime-composition item rather than adding another trusted-host
profile by default.

Do not begin arbitrary local commands, generic scheduling, default activation,
provider mutation broadening, nested harness execution, or hosted expansion as
an implied consequence of this acceptance.

## 15. Validation

The accepted implementation commit was reviewed against:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- focused Core and local-host tests;
- `cargo test --workspace`;
- `npm run check:docs`; and
- `git diff --check`.

All commands passed. Documentation-only review changes are revalidated before
phase close.

## 16. Governed Review Evidence

- Workflow: `dg/review`.
- Run: `run-1792662814338683000-2`.
- Approval:
  `approval/run-1792662814338683000-2/review-scope-approved`.
- Approval presentation: `presentation/1932b7a4b6faf2ef`.
- Approval outcome: granted under delegated-maintainer authority with
  persisted presentation proof.
- Phase status: completed.
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations.
- Approval-presentation enforcement: proof enforced with one persisted
  presentation record and an event proof marker.
- Out-of-kernel work: code, test, plan, report, and diff inspection;
  validation commands; documentation authoring; git; and later PR operations
  are performed by the delegated maintainer while Workflow OS governs review
  scope and approval.

No `.workflow-os` runtime state is part of the review change set.
