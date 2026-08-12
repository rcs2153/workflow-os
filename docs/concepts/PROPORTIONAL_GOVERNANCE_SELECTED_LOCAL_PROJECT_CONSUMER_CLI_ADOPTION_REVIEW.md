# Proportional-Governance Selected Local Project Consumer CLI Adoption Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups. The manifest-controlled CLI
cutover is compatible, bounded, source-backed, and ready for pull-request
review. No blocker was found.

## 2. Scope Verification

The phase stayed within the approved compatibility-sensitive CLI adoption
scope. It replaced internal composition for the already-declared `run` and
`approve` paths, added one focused regression test, and updated the roadmap,
plan, and implementation report.

It did not add commands, flags, declarations, schemas, workflow defaults,
provider execution, provider writes, SideEffect execution, hosted behavior,
runtime configuration, approval policy, or another mutation family. Ordinary
undeclared workflow execution remains unchanged.

## 3. Run Path Assessment

The declared run path now calls
`execute_selected_project_validation_governance_report`. Core owns the
canonical check result, fixed source registration, current facts, evaluation
time, source-backed governance assessment, and stable report reference. The
CLI does not author authority-bearing facts or route disposition.

Quiet, visible, approval-required, denied, failed-check, and
existing-terminal routes retain the existing result and artifact boundary.
Fresh-run artifact persistence remains in the CLI, as planned, and still
prints truthful run output before enforcing the artifact obligation.

## 4. Approval Path Assessment

The CLI no longer inspects approval binding state to select separate aggregate
and authored implementations. It calls one selected Core approval envelope.
Core derives the bounded gate kind from durable approval state, enforces
presentation proof before reassessment, reloads the immutable workflow,
rechecks current authority only for grants, and closes terminal outcomes
through the selected artifact gates.

Aggregate grants remain non-terminal. Their authority receipt remains
transient and no report artifact is written. Authored terminal grants cite the
exact decision-time check reference, persist one trusted receipt record,
validate receipt referential integrity, project proof markers, and persist the
terminal report artifact. Both aggregate and authored denials create truthful
terminal reports without rerunning the decision-time check or executing a
skill.

## 5. Output Compatibility Assessment

The public command surface and line-oriented human output remain stable. The
implementation preserves:

- quiet-success minimal output;
- Rust-cased human status and lowercase JSON status;
- top-level `run_id` and `approval_id` fields;
- complete persisted approval handoffs;
- `approval_decision` for aggregate approval JSON;
- `authored_approval_decision` for authored approval JSON;
- report and artifact posture labels;
- bounded error-code fields; and
- output-before-artifact-obligation ordering.

No source identity, current facts, authority receipt contents, report text,
paths, command output, environment values, or provider payloads were added to
CLI output.

## 6. Durable State And Retry Assessment

The selected Core envelope creates deterministic local stores under the
planned state-root paths. Exact duplicate report artifacts reconcile as
already persisted. Conflicting or ambiguous outcomes fail closed and prevent
blind retry. Existing-terminal run retry remains idempotent and does not append
events or rerun workflow skills.

Approval proof is preflighted before current-fact reassessment or approval
mutation. Relevant immutable definition drift and source-backed fact drift
continue to fail closed. The phase does not mutate kernel state by hand.

## 7. Test Quality Assessment

Existing authoritative CLI tests cover quiet, visible, approval-required,
denied, failed-check, drift, retry, artifact, JSON privacy, handoff, and
ordinary-run behavior. The existing two-gate CLI test verifies two
proof-marked grants and terminal artifact persistence. The new focused test
drives both approvals through JSON, proves the frozen route labels, verifies
aggregate non-terminal artifact deferral, and confirms exactly one terminal
authority-receipt record.

Core tests separately prove aggregate and authored denial without a check
rerun, proof-before-recheck behavior, one check per grant, no transient receipt
persistence, proof-marker projection, and terminal receipt/artifact closure.

## 8. Privacy And Error Assessment

The adoption removes CLI construction of decision-time check references and
therefore reduces the authority-bearing surface. Debug and public output remain
bounded. Stable errors do not include facts, paths, command output, report
text, provider payloads, credentials, or secret-like values. Post-decision
report or persistence failure does not rewrite the truthful workflow result.

## 9. Documentation Assessment

The roadmap, implementation plan, and phase report accurately state that the
selected CLI adoption is implemented. They preserve the product boundary:
declaration-controlled local behavior only, with no provider execution,
writes, hosted behavior, generic default, or new schema exposure.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add a CLI-level aggregate and authored denial compatibility test when the
  next approval-envelope test-depth phase is opened.
- Add exact JSON key-set assertions, rather than selected key assertions, in a
  future compatibility-hardening phase.
- Review retirement of the old public Core compatibility APIs separately; do
  not remove them as part of this adoption.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed under the
  repository-local toolchain during review. Hosted Rust 1.97 subsequently
  identified `clippy::too_many_lines` in the 101-line run adapter. A bounded
  fix-forward extraction removed the hosted lint blocker without changing the
  accepted behavior; hosted required CI remained pending when this note was
  added.
- `cargo test -p workflow-cli --test cli authoritative_governance`: passed.
- Focused selected-envelope CLI regression test: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed with the repository-pinned Node 20 toolchain.
- `git diff --check`: passed.

## 13. Governed Review Record

- Dogfood workflow: `dg/review`.
- Run ID: `run-1786511665299227000-2`.
- Approval ID: `approval/run-1786511665299227000-2/review-scope-approved`.
- Approval outcome: granted with persisted presentation proof
  `presentation/7dbb8db7c01d640a`.
- Review run status: completed.
- Event summary: 39 events, one approval, zero retries, zero escalations.

The kernel governed scope, approval, and durable event history. The delegated
maintainer performed code inspection, commands, tests, documentation edits,
and Git work outside the kernel.

## 14. Recommended Next Phase

Publish the accepted implementation and review together for pull-request and
CI verification. After merge, inspect the current roadmap ordering before
choosing between compatibility cleanup and the next runtime composition lane.
