# Authoritative Governance Scaffold Runtime Contract Blocker Fix Review

## 1. Executive Verdict

**Blocker fixed; the explicit authoritative scaffold path is accepted.**

The generated authoritative project now contains the exact project-level
profile selection and workflow-step check declaration required by the existing
runtime consumer. A clean disposable repository reaches the authoritative
check, preserves two independent approval gates, completes, and persists one
terminal WorkReport artifact. Default scaffolding remains unchanged.

## 2. Scope Verification

The fix stayed within the approved blocker boundary:

- only explicit `--authoritative-governance` scaffolds gain the canonical
  project-validation requirement;
- default scaffold content and runtime behavior remain unchanged;
- no new check profile or command vocabulary was introduced;
- no workflow schema, provider, sandbox, SideEffect, or external-write path was
  added; and
- no OpenShell integration or execution-provider decision was made.

## 3. Contract Completeness Assessment

The scaffold now generates both halves of the existing closed contract:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

and:

```yaml
local_check_requirements:
  - id: project-validation
    command_id: local-check/workflow-os-validate
    requirement_level: required
    minimum_assurance: kernel_observed_local_process
    accepted_statuses: [passed]
    freshness:
      mode: no_reuse
    exact_immutable_run_binding_required: true
    truncation_allowed: false
    network_maximum: disabled
    side_effect_maximum: no_source_writes
```

This matches the current authoritative CLI consumer and Core command contract.
The implementation reuses fixed vocabulary rather than inferring repository
commands.

## 4. Runtime And Approval Assessment

The disposable-repository proof confirms:

- project validation succeeds;
- the fixed local check executes before mock skill invocation;
- proportional governance requests its own approval;
- the workflow step retains its separate approval;
- granting one approval does not grant the other;
- the run completes only after both decisions; and
- terminal completion persists exactly one WorkReport artifact.

The fix does not change workflow pass/fail semantics or add execution
authority.

## 5. Compatibility Assessment

Default scaffold output contains neither an authoritative project declaration
nor a local-check requirement. The explicit flag remains additive. Existing
agent guidance is preserved, dry-run remains non-writing, and unknown options
continue to fail before writes.

The string-template insertion is narrow and covered by public CLI regression
tests. A future scaffold-template refactor may replace it with a structured
builder, but that is not required for this fixed static template.

## 6. Privacy And Failure Assessment

The generated requirement contains only fixed identifiers and closed policy
values. It does not include source contents, command output, provider payloads,
paths, credentials, or tokens.

The original incomplete contract failed closed with
`cli.authoritative_governance.check_profile_missing` before run creation. The
fix removes the setup defect without weakening that runtime preflight.

## 7. Test Quality Assessment

Focused tests cover:

- default omission of the authoritative declaration and check;
- exact explicit requirement generation;
- validation and first-run posture;
- authoritative execution through both approvals;
- terminal completion; and
- one persisted WorkReport artifact.

Full Rust, CLI, TypeScript, contract, documentation, dogfood-helper, and
integration suites pass. The automated end-to-end test does not separately
assert every local-check event field; existing authoritative executor tests and
the disposable inspect proof cover those details. Adding a direct event
assertion later would be useful but is non-blocking.

## 8. Documentation Assessment

The roadmap, external evaluation, opt-in report, and blocker-fix report now
agree that:

- the initial external evaluation found a runtime-contract blocker;
- the blocker was fixed narrowly;
- repeated external execution passes;
- explicit opt-in remains required;
- default activation is unchanged; and
- providers, OpenShell, SideEffects, and external writes remain unsupported by
  this phase.

## 9. Blockers

No blockers remain.

## 10. Non-Blocking Follow-Ups

- Consider asserting the persisted local-check reference directly in the CLI
  end-to-end regression.
- Continue gathering external-repository onboarding evidence before changing
  the default.
- Keep Node 20 as the supported repository validation toolchain while the
  opaque Node 24 integration-helper failure is investigated separately.
- Use the accepted proportional-governance and quiet-success lane to reduce
  low-risk ceremony without weakening evidence or disclosure.

## 11. Governed Review Record

- workflow id: `dg/review`
- run id: `run-1785221870807757000-2`
- approval id:
  `approval/run-1785221870807757000-2/review-scope-approved`
- presentation id: `presentation/6a7c7b1bca7dca22`
- approval outcome: granted by delegated maintainer
- approval proof: persisted
- out-of-kernel work: source review, test assessment, documentation edits, and
  validation were performed by Codex under the kernel-governed review scope

## 12. Recommended Next Phase

Commit and merge this accepted blocker fix. Then return to the accepted
Risk-Proportional Governance and Quiet Success sequence, beginning with the
model-only decision contract already positioned in the roadmap. Do not broaden
provider execution or external writes as part of that phase.
