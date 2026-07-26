# Generic Explicit Local-Check Profile Source Review

## 1. Executive Verdict

**Needs blocker fixes.**

The implementation is otherwise narrow, compatible, well tested, and aligned
with the accepted authoritative proportional-governance path. One public
construction boundary does not yet prove that the supplied command contract is
the complete canonical `workflow-os validate` contract.

## 2. Scope Verification

The phase stayed within the approved Core-only profile and runtime-composition
scope.

It did not add:

- CLI or UI behavior;
- default profile selection or registration;
- repository command inference;
- workflow schema, SDK, scaffold, or example changes;
- result persistence or report artifacts;
- providers, OpenShell, containers, or credentials;
- network access, SideEffect execution, or writes;
- hosted behavior, reasoning lineage, or release changes.

## 3. Model And Profile Assessment

The closed profile vocabulary is appropriately small:

- `ExplicitLocalCheckProfileId` has one supported value;
- `ExplicitLocalCheckProfileSelection` performs no execution;
- `ResolvedExplicitLocalCheckProfile` binds the profile ID, handler, command
  contract, skill identity, and immutable declaration inventory; and
- ordinary registry behavior remains empty by default.

The profile requires an explicit executable and project root. It does not
derive execution authority from repository metadata or discovery.

## 4. Command And Handler Authority Assessment

The canonical model-only contract correctly specifies:

- command ID `local-check/workflow-os-validate`;
- kind `workflow_os_project_validation`;
- arguments `[validate]`;
- repository-root working directory;
- sanitized environment;
- disabled network;
- no source writes;
- bounded output; and
- bounded citation kinds.

The selection-based resolver constructs this canonical contract and is safe.

However, `WorkflowOsProjectValidationLocalHandler::new(...)` and
`new_with_process_runner(...)` are public and accept a caller-supplied
`LocalCheckCommandContract`. Contract validation already proves that the
executable and arguments match the command kind's fixed template, and the
constructor checks broad safety posture. It does not compare every field with
`workflow_os_project_validation_model_only()`.

That means a caller can retain the accepted command template and posture while
changing timeout, output bounds, citation kinds, command ID, or other accepted
contract fields. This contradicts the public claim that the profile exposes
one complete canonical contract and permits profile identity or evidence
posture to drift from the built-in definition.

This is a blocker because the next phase would expose this profile through an
operator-facing CLI path.

## 5. Immutable Identity And Preflight Assessment

The resolved profile correctly derives its immutable declaration inventory
from the same contract used by the handler. Authoritative preflight compares
the selected declaration's command ID and fingerprint before process
execution.

The implementation review also confirmed the selected workflow step is
resolved from the frozen immutable bundle before the actual local-check
handler identity is bound. The full workspace suite initially exposed a
regression where an unknown step could reach process execution; that ordering
was corrected and its regression test passes.

Once the public constructor is canonicalized, this identity chain will be
appropriately closed.

## 6. Runtime And Report Assessment

The private `AuthoritativeLocalCheckHandler` trait is crate-owned and is not
exported as public arbitrary handler authority.

The additive route and report helpers reuse the accepted authoritative
pipeline. The project-validation profile:

- executes one check;
- derives a source-bound assessment;
- selects an existing proportional-governance route;
- preserves durable run-event equality; and
- cites the same-call result in the terminal report without a second check.

Existing DocsCheck APIs retain their public signatures and behavior.

## 7. Privacy And Failure Assessment

Resolution errors and `Debug` output do not expose executable or project
paths. Process output remains bounded and redacted through existing local
check result handling.

The phase does not accept or persist raw source contents, spec contents,
parser payloads, provider payloads, environment values, credentials,
authorization headers, private keys, or tokens.

The constructor blocker is an authority-integrity issue rather than an
observed data leak. The blocker fix must retain stable non-leaking errors and
must not include supplied contract fields in error messages.

## 8. Compatibility Assessment

The ordinary registry remains unchanged and profile registration is explicit
and collision rejecting. Existing DocsCheck, immutable-run, approval,
WorkReport, EvidenceReference, adapter, CLI, and runtime tests pass.

No schema, serialization, scaffold, example, persistence, or release posture
changed.

## 9. Test Quality Assessment

Focused tests cover:

- canonical contract posture;
- explicit resolution without execution;
- path-safe `Debug` and invalid-root errors;
- collision-rejecting registration;
- immutable declaration inventory;
- authoritative quiet routing;
- source-bound assessment;
- same-call report citation;
- one-check execution; and
- durable run-event equality.

The missing blocking regression is:

- direct public handler construction must reject any valid contract that
  differs from the complete canonical contract, such as a modified timeout.

The blocker-fix phase should add a representative modified-timeout test and
prove the rejection occurs before process execution. Full equality provides
coverage for every canonical field without requiring one test per field.

Profile-specific visible, approval, and denial tests are not blocking because
those routes are exercised through the same private dispatcher. The first CLI
phase should add operator-level route coverage for the explicit profile.

## 10. Documentation Assessment

The plan, report, and roadmap accurately preserve the narrow product boundary.
Their claim that the handler accepts no arbitrary arguments is not fully true
until the constructor blocker is fixed. The original implementation report is
preserved; this review records the discrepancy explicitly.

Fresh-pull user evaluation continues to support the phase direction:
Workflow OS is a credible local governance kernel, and the next product
pressure is lower-friction proportional governance backed by real checks. The
Node 24 integration-check behavior and duplicate missing-manifest diagnostic
remain separate non-blocking onboarding issues.

## 11. Blockers

1. Make public project-validation handler construction fail closed unless the
   supplied contract equals the complete canonical
   `workflow-os validate` contract.
2. Add a regression proving a caller-modified accepted contract field is rejected
   before process execution and that the error is stable and non-leaking.

## 12. Non-Blocking Follow-Ups

- Add explicit-profile visible, approval, denial, and concise operator-output
  coverage during the CLI preview phase.
- Keep Node 20 as the documented integration-check baseline until Node 24
  behavior is separately diagnosed.
- Remove the duplicate missing-manifest diagnostic in a bounded CLI polish
  phase.
- Do not infer project-specific tests or commands from safe metadata.

## 13. Recommended Next Phase

Perform the generic explicit local-check profile canonical-contract blocker
fix.

After focused review accepts that fix, proceed to the explicit authoritative
quiet-success CLI preview. Do not expose this profile through the CLI while
its public constructor can accept a non-canonical contract.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785103980137246000-2`
- approval:
  `approval/run-1785103980137246000-2/review-scope-approved`
- presentation: `presentation/07566da42f711e96`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- approval-presentation enforcement: proof enforced
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation:
  - `cargo fmt --all --check` passed
  - `cargo clippy --workspace --all-targets -- -D warnings` passed
  - `cargo test --workspace` passed
  - `npm run check:docs` passed
  - `git diff --check` passed
- out-of-kernel work: source inspection, focused maintainer review,
  documentation, and validation
- missing coverage: the kernel coordinated governance but did not inspect
  code, edit files, run tests, or perform git and PR actions

## 15. Fix-Forward Status

The blocker was fixed in the immediately following bounded phase.

Public project-validation handler construction now compares the supplied
contract with the complete canonical contract after normal model validation.
A model-valid contract with a changed timeout fails closed before process
execution with stable code
`local_check.profile.handler.contract_non_canonical`.

The original finding and verdict above are preserved as the review record.
See the
[Generic Explicit Local-Check Profile Canonical Contract Blocker Fix Report](GENERIC_EXPLICIT_LOCAL_CHECK_PROFILE_CANONICAL_CONTRACT_BLOCKER_FIX_REPORT.md).
