# Profile-Controlled Authoritative Governance Activation Plan Review

## 1. Executive Verdict

Plan accepted; proceed to the complete profile-controlled authoritative
governance activation implementation.

The plan closes the correct product gap after the accepted explicit CLI
preview. It replaces repeated operator flag selection with one typed,
reviewable project declaration while preserving ordinary behavior for
undeclared projects. It also avoids another model-only sequence by requiring
the public contract, runtime activation, resume integrity, first-run
disclosure, and compatibility tests to land together.

## 2. Scope Verification

The plan remains bounded to:

- one optional project-level authoritative-execution declaration;
- one supported strictness/check-profile combination;
- Rust, JSON Schema, and TypeScript SDK contract synchronization;
- existing authoritative `run` and `approve` path activation;
- immutable-run and resume binding;
- first-run disclosure;
- compatibility and privacy tests; and
- accurate product documentation.

It does not authorize:

- activation for undeclared projects;
- arbitrary or inferred commands;
- additional check-profile families;
- automatic approval;
- provider or OpenShell execution;
- SideEffect execution or writes;
- report persistence or artifacts;
- scaffold defaults or examples;
- hosted or enterprise controls;
- reasoning lineage or nested harness runtime; or
- release changes.

No scope blocker was found.

## 3. Product Alignment

Fresh-pull evaluation identifies ceremony reduction as the next user problem.
The accepted CLI preview proves the runtime behavior, but requiring the user to
repeat `--authoritative-governance` is not a durable governance source.

The plan correctly distinguishes:

- deterministic workload assessment from user configuration;
- project-declared minimums from Core-selected routes;
- execution disposition from operator disclosure; and
- quiet operation from missing evidence or audit.

Users declare one bounded activation contract. Core still derives quiet,
visible, approval-required, or denied behavior from the validated workload and
runtime facts.

## 4. Source-Of-Truth Assessment

The proposed complete declaration:

```yaml
governance:
  authoritative_execution:
    profile: observe_and_report
    local_check_profile: workflow_os_project_validation
```

is appropriate for the first slice.

It is better than:

- reusing free-form `config.vars`;
- inferring activation from repository metadata;
- treating first-run recommendations as authority;
- reading `AGENTS.md`;
- using an environment variable; or
- retaining a CLI flag as the only source of truth.

Both fields are required inside the activation object. That prevents a
standalone profile label or check-profile label from looking enforceable while
doing nothing.

## 5. Runtime Integrity Assessment

The plan correctly requires:

- validation before process execution and run creation;
- exact closed profile resolution;
- immutable binding of declaration presence and identity;
- declaration-aware `run` routing;
- existing-run-aware `approve` routing;
- fresh decision-time check reassessment;
- approval-presentation proof;
- rejection of declaration removal or drift; and
- separation of aggregate governance approval from authored workflow
  approvals.

The implementation must not re-read changed ambient project configuration and
treat it as the authority that originally requested approval. The waiting run's
durable immutable context must establish whether authoritative activation was
in force. Current project validation may contribute fresh evidence, but it
cannot rewrite historical authorization provenance.

## 6. Compatibility Assessment

Compatibility behavior is explicit and conservative:

- no declaration means ordinary current behavior;
- the existing flag remains an experimental opt-in;
- flag and declaration agreement resolve to one path;
- conflicts fail closed; and
- missing resume flags cannot silently bypass authoritative approval behavior.

The plan should preserve this source precedence exactly. A CLI flag may tighten
an undeclared invocation into the preview path, but it must never weaken,
replace, or reinterpret a project declaration.

## 7. Schema And SDK Assessment

Updating the Rust manifest, JSON Schema, and TypeScript SDK in one phase is
necessary. A partial contract would create incompatible or decorative
governance surfaces.

The plan correctly requires:

- optional outer fields for backward compatibility;
- required closed fields inside the activation object;
- unknown-field rejection;
- stable serde behavior;
- semantic validation of the one supported v0 combination; and
- schema/SDK synchronization tests.

Existing profile vocabulary is broader than the supported activation slice.
Therefore deserialization success cannot imply runtime support. Semantic
validation must reject unsupported profile values with fixed non-leaking
errors.

## 8. Operator Experience Assessment

The planned experience is coherent:

- users may retain explicit flag activation;
- reviewed projects may declare the closed profile once;
- low-risk eligible work can complete quietly with evidence and a report;
- visible posture remains non-blocking presentation;
- approval posture presents complete proof-bound context;
- denial remains terminal; and
- first-run explains declared, supported, and enforced posture.

The plan does not silently modify scaffolds. That is appropriate for this
phase. A later explicit onboarding write command can be considered after real
operator testing.

## 9. Security And Privacy Assessment

The plan preserves the accepted security boundary:

- no command strings or arguments in YAML;
- no credential or environment discovery;
- network disabled;
- source read only;
- fixed local-check identity;
- no raw check output in events, evidence, or reports;
- bounded errors and output; and
- no fallback on declaration mismatch.

This remains governance plus one fixed local validation handler. It is not a
general process sandbox or provider runtime.

## 10. Test Plan Assessment

The planned coverage is sufficient and behavior-oriented. It covers:

- absent, valid, incomplete, unknown, unsupported, and conflicting
  declarations;
- flag compatibility;
- all four authoritative routes;
- proof-enforced resume without repeated flag input;
- authored approval separation;
- declaration drift;
- exact check execution count;
- schema/SDK synchronization;
- first-run disclosure;
- privacy/non-leakage; and
- ordinary CLI compatibility.

Full workspace and integration checks are required, including the supported
Node version for the integration harness.

## 11. Blockers

None.

## 12. Non-Blocking Implementation Constraints

- Keep CLI/declaration precedence fail closed and cover every combination.
- Bind authoritative activation to durable immutable run context before
  relying on current project configuration during approval resume.
- Use existing event and binding vocabulary if it can represent the new source
  truthfully; add vocabulary only if a focused implementation finding proves
  it cannot.
- Keep the first supported combination closed. Do not opportunistically enable
  the other strictness profiles.
- Keep first-run concise output short and place full activation detail in
  verbose/JSON output.

## 13. Recommended Next Phase

Implement the complete profile-controlled authoritative governance activation
phase described by the plan.

Do not insert another model-only or helper-only phase first. The relevant
model, selector, local-check profile, dispatcher, approval-resume path, report
consumer, and explicit CLI preview already exist.

After implementation and focused phase review, consider explicit onboarding
ergonomics for writing the declaration. Do not broaden providers, OpenShell,
writes, check profiles, enterprise controls, or scaffold defaults first.

## 14. Governed Review Record

- workflow: `dg/review`
- run: `run-1785117275064940000-2`
- approval:
  `approval/run-1785117275064940000-2/review-scope-approved`
- presentation: `presentation/08cfe5b8f756c41f`
- approval outcome: granted by delegated maintainer through presentation-proof
  enforcement
- phase status: completed
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- validation: `npm run check:docs` and `git diff --check` passed
- skipped checks: Rust formatting, clippy, and workspace tests were not required
  for this documentation-only review phase
- out-of-kernel work: plan inspection, architecture review, review authoring,
  and validation command execution
- kernel boundary: the kernel governed review scope and approval but did not
  inspect documents, write the review, run checks, or perform git actions
- report posture: this review is the bounded phase report; no runtime
  WorkReport or report artifact was generated or persisted
