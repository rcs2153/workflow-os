# Authoritative Quiet-Success CLI Preview Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The explicit authoritative CLI surface composes the accepted Core governance,
local-check, immutable-input, approval-presentation, and WorkReport paths
without giving the CLI authority to choose a command, route, check outcome, or
approval outcome. One approval-rendering blocker found during review was fixed
and covered before acceptance.

## 2. Scope Verification

The phase stayed within its approved experimental CLI scope.

It added:

- additive `--authoritative-governance` paths for `run` and `approve`;
- one fixed project-validation profile;
- bounded route-aware human and JSON output;
- complete persisted approval presentations;
- separate aggregate-governance and authored-workflow approval handling; and
- in-memory report completion.

It did not add:

- default or automatic authoritative governance;
- arbitrary or repository-discovered command execution;
- report artifacts, report persistence, or export;
- provider or OpenShell integration;
- SideEffect execution or writes;
- schemas, examples, hosted behavior, enterprise controls, or release changes.

## 3. CLI And Compatibility Assessment

The preview is opt-in. Ordinary `workflow-os run` and `workflow-os approve`
continue through their existing dispatch paths when the flag is absent.

The parser admits only:

```text
workflow-os run <workflow-id> --authoritative-governance
workflow-os approve <run-id> <approval-id> \
  --authoritative-governance --actor <actor> --reason <reason>
```

Unexpected command, route, or execution inputs fail before run state is
created. The preview therefore exposes composition of accepted authority; it
does not create a generic command runner.

## 4. Check Authority Assessment

The CLI resolves only
`ExplicitLocalCheckProfileSelection::workflow_os_project_validation()`.
Resolution fixes the executable, project root, process request, canonical
contract, and runner boundary before execution.

The newly public execute-once method on
`ResolvedExplicitLocalCheckProfile` cannot accept arguments or discover
commands. It builds and executes only the already resolved process request and
returns the existing bounded `LocalCheckResult`.

Fresh execution and aggregate approval reassessment remain on the accepted Core
helpers. Authored workflow approval uses the same resolved profile and creates
a payload-free reference from the exact result obtained before decision
mutation.

## 5. Route Assessment

Core remains the only route selector.

- Quiet proceed completes without an interrupt and preserves report and check
  references.
- Visible proceed requires disclosure delivery but does not request approval.
- Approval required persists a complete presentation and remains blocking.
- Denial remains terminal and inspectable.

The CLI does not accept a route override and does not infer a lower route after
Core has selected a stronger one.

## 6. Approval And Immutable-Context Assessment

Aggregate proportional-governance approval and authored workflow approval are
different approval subjects. The first grant cannot satisfy the second.

When aggregate approval exposes a later authored gate, the CLI:

1. preserves the waiting run;
2. creates a presentation bound to the new approval ID;
3. prints the new complete handoff; and
4. requires a separate proof-enforced decision.

Approval resume reloads the immutable bundle manifest and continues through the
existing resolved-context and presentation-proof enforcement paths. The CLI
does not replace or edit run state.

## 7. Approval Rendering Blocker And Fix

Review found that the persisted presentation and the displayed handoff were
constructed from duplicate string literals. They matched in the implementation
under review, but future drift could have caused the human-visible scope to
differ from the record later used as approval proof.

This was an acceptance blocker because exact approval context is a security
boundary.

The renderer now reads directly from `ApprovalPresentationRecord` and includes:

- presentation ID and content hash;
- requested action;
- work summary and approved scope;
- strict non-goals;
- expected touched surfaces;
- validation expectations;
- why-now context; and
- exact next action.

Focused CLI coverage verifies the proof context is rendered. No approval model
or runtime semantic was broadened.

## 8. Report And Evidence Assessment

Terminal successful routes produce an in-memory `WorkReport` through the
existing report helper. The report cites a stable, payload-free local-check
result reference created from the actual result.

Report failure remains distinct from durable workflow status. The CLI prints a
stable report error code and does not rewrite a completed or failed run.

No raw output, command transcript, report body, provider payload, source
content, or environment value is copied into the event stream or default
output. No report artifact is created.

## 9. Privacy And Error Assessment

Human output is bounded. Experimental JSON output contains identities, route
and disposition labels, report posture, stable references, approval posture,
error codes, and inspect guidance.

It excludes:

- raw stdout and stderr;
- executable and filesystem paths;
- environment values;
- policy payloads;
- report section bodies;
- provider data; and
- secret-like test markers.

Construction and execution errors use stable
`cli.authoritative_governance.*` codes without caller payload values.

## 10. Test Quality Assessment

Focused CLI tests cover:

- quiet completion and in-memory report posture;
- visible disclosure without approval language;
- aggregate approval followed by a distinct authored approval;
- proof markers on both granted decisions;
- terminal denial;
- fail-closed missing profile before state creation;
- bounded valid JSON and payload non-leakage;
- rejection of ambient command and route inputs; and
- record-backed presentation hash and requested-action rendering.

Workspace tests retain the accepted Core coverage for:

- exact-once fresh and decision-time local checks;
- immutable and resolved-context integrity;
- handler substitution rejection;
- report failure separation;
- disclosure delivery;
- approval-presentation proof; and
- ordinary CLI and executor behavior.

## 11. Blockers

None remain.

The duplicate approval-rendering source was fixed during review and the
focused suite passed afterward.

## 12. Non-Blocking Follow-Ups

- Decide whether authoritative denial should remain available when the fresh
  project-validation check fails. The current preview applies the same
  validation prerequisite to grants and denials.
- Complete the separate P0 presentation-delivery proof work so durable proof
  can establish that exact context was presented, not only persisted and
  rendered by the CLI process.
- Add CLI-level fault injection for report-generation failure if a stable test
  seam is introduced; accepted Core tests already cover status preservation.
- Keep the preview explicit until profile-controlled defaults and proportional
  governance configuration have their own accepted phases.

## 13. Documentation Assessment

The roadmap, plan, product contract, implementation report, and this review
consistently state:

- the preview is implemented and experimental;
- ordinary command behavior is unchanged;
- only the closed project-validation profile is executable;
- reports are in memory only; and
- providers, OpenShell, SideEffect execution, writes, schemas, examples,
  hosted behavior, and release posture remain unsupported.

## 14. Recommended Next Phase

Return to the accepted proportional-governance roadmap and select the next
runtime implementation phase from current `main` after merge.

The next phase must prefer runtime composition over new vocabulary and must not
broaden provider mutation families before the roadmap's immutable-input,
authority, and approval prerequisites.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1785115943459084000-2`
- approval: `approval/run-1785115943459084000-2/review-scope-approved`
- presentation: `presentation/d871911c8d871983`
- approval outcome: granted by delegated maintainer through proof enforcement
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval presentation enforcement: proof enforced
- validation: `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `npm run check:docs`, `git diff --check`, and the
  focused authoritative CLI suite passed
- out-of-kernel work: code inspection, blocker analysis, renderer fix, test and
  documentation edits, command execution, and review authoring
- missing coverage: the kernel coordinates governance only; it did not inspect
  code, edit files, execute validation, or perform git and pull-request actions
