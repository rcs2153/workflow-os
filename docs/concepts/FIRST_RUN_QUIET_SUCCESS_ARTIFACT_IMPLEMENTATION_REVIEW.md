# First-Run Quiet-Success Artifact Implementation Review

## 1. Disposition

Accepted after one narrow compatibility fix.

The new exact low-sensitivity contract, Core-owned route selection,
proof-free quiet completion, create-only artifact persistence, replay,
recovery, additive scaffold upgrade, and ordinary first-run/demo separation
are coherent and covered by focused and workspace tests. Durable legacy run
identity now selects the legacy contract during approval and artifact
persistence, while new artifact-contract identity still requires the complete
new scaffold.

## 2. Resolved Finding: Legacy Resume Required The New Contract

The accepted plan requires durable legacy artifact runs to continue by their
immutable workflow identity rather than current scaffold defaults. The start
path selects the legacy or new contract from the rehydrated run correctly.

Before the fix, the approval-resume path did not preserve that boundary. It
called the shared
`load_valid_first_run_project(...)` helper, and that helper unconditionally
called `validate_first_run_generated_contract(...)`. A legacy
`local/first-run-governance` artifact run created before the new generated
workflow, skill, policy, and test were installed therefore fails with the new
contract-upgrade requirement before its legacy contract can be validated and
resumed.

This was a compatibility blocker because an in-flight proof-backed run must not
become dependent on a later scaffold default. Requiring an additive scaffold
upgrade before resume is operationally convenient but violates the accepted
immutable-run boundary.

## 3. Fix Accepted

Project loading and validation are now contract-aware for an already
rehydrated run:

- legacy run identity validates only the exact legacy workflow, skill, and
  policies committed by that path;
- new run identity validates only the complete new artifact contract;
- unknown workflow identity remains rejected;
- current project validation still runs before either contract is admitted;
- no built-in fallback or caller-selected contract was introduced; and
- the artifact and approval paths continue to derive the contract from durable
  run identity.

The focused regression generates the canonical scaffold, removes the new
additive contract files to reproduce a pre-upgrade repository, and proves that
the shared reload path admits only the durable legacy identity. The same
pre-upgrade repository is rejected for the new artifact workflow identity with
`cli.first_run.artifact.contract_upgrade_required`. Existing integration
coverage retains the separate proof that a genuinely new start without the
new contract fails before runtime state creation with the upgrade diagnostic.

## 4. Non-Blocking Review Results

- The new contract is distinct from the medium-sensitivity approval demo.
- Exact content fingerprints bind workflow, skill, both policies, and test.
- The CLI-private local request is committed to actor, canonical project root,
  workflow, run, analysis, artifact root, and operation version.
- Quiet completion creates no approval request, approval event, presentation
  record, or proof marker.
- Recovery requires exact immutable analysis and a fresh authoritative project
  validation result.
- Mock mode and customized generated contracts fail closed.
- Ordinary `first-run` and the existing approval/audit demo remain separate.

## 5. Validation Reviewed

- focused durable-contract reload regression: passed.
- `cargo test -p workflow-cli --test cli`: passed, 176 tests.
- `cargo test --workspace --all-targets`: passed; opt-in live tests remained
  ignored as designed.
- pinned Node 20 `npm run check:docs`: passed.
- `git diff --check`: passed.
- disposable external-repository scaffold, quiet completion, exact replay,
  missing-artifact recovery, and state-health proof: passed.

The compatibility blocker is resolved and no release, provider, network,
schema, SDK, hosted, or publication behavior was broadened by the fix.

## 6. Governed Review Record

- workflow: `dg/review`;
- run: `run-1793439918872898000-2`;
- approval: `approval/run-1793439918872898000-2/review-scope-approved`;
- presentation: `presentation/188e641ea59d9575`;
- decision: granted under delegated-maintainer authority; and
- outcome: completed with proof-enforced approval.

## 7. Scope Boundary

This review does not authorize release version changes, publication, provider
or network access, arbitrary command execution, schema or SDK changes, hosted
behavior, or unrelated refactoring. The blocker fix must remain limited to
contract-aware legacy resume admission and its regression proof.

## 8. Governed Blocker-Fix Record

- workflow: `dg/blocker`;
- run: `run-1793440007990548000-2`;
- approval: `approval/run-1793440007990548000-2/fix-approved`;
- presentation: `presentation/c660b96a736b6944`;
- decision: granted under delegated-maintainer authority with persisted
  presentation proof; and
- outcome: completed with 39 events, one approval, zero retries, zero
  escalations, and proof-enforced presentation.
