# Local Project Authority Source Runtime Composition Review

## 1. Executive Verdict

Phase accepted with the standalone CLI compatibility flag retained as an
explicit non-blocking limitation.

The project-declared path now has a concrete, immutable, Core-owned authority
source instead of trusting a CLI-supplied classification.

## 2. Scope Verification

The phase stayed within the closed project-validation composition boundary. It
did not add RBAC, enterprise identity, automatic approval, public authority
APIs, OpenShell, sandbox execution, credentials, providers, SideEffects,
writes, schemas, hosted behavior, or release changes.

## 3. Source Assessment

The validated project declaration is appropriate as the first production local
source because:

- it is explicit rather than inferred;
- the supported vocabulary is closed;
- project validation rejects unsupported combinations; and
- the declaration is committed into the immutable run bundle.

The source is workload-level authority for one local profile. It must not be
described as actor-specific authorization or general tool authority.

## 4. Runtime Assessment

The CLI leaves authority absent for project-declared execution. Core verifies
the immutable activation and supplies sufficient authority only inside the
existing governance composition.

Approval reassessment repeats the same binding from the stored immutable
bundle. Approval does not create or replace authority.

## 5. Failure And Privacy Assessment

Missing or mismatched activation fails closed. Caller-preclassified authority
also fails before local-check execution, skill invocation, or workflow event
creation.

The stable errors do not expose governed IDs, paths, commands, configuration
values, credentials, payloads, or secret-like input.

## 6. Compatibility Assessment

Existing public executor APIs are unchanged. Project-declared CLI behavior is
strengthened without changing its user-facing invocation.

The standalone `--authoritative-governance` flag remains a compatibility path
and still uses caller-classified authority when no project declaration exists.
That limitation is explicit and should be retired or replaced in a separate
reviewed phase.

## 7. Test Quality Assessment

Focused tests cover immutable source derivation, exact activation persistence,
preclassification rejection with zero execution, and approval-time rebinding.
Existing CLI and executor tests remain the regression boundary for the legacy
flag and report-bearing routes.

The local test executable encountered the repository's known macOS launch
stall after compilation. GitHub CI must pass before merge.

## 8. Blockers

None, subject to passing canonical CI.

## 9. Non-Blocking Follow-Ups

- Decide whether the standalone flag should be deprecated or require an
  explicit trusted source.
- Add actor-specific delegated authority only through the scoped capability
  lane, not by widening this declaration.
- Preserve the distinction between execution containment and authority when
  evaluating OpenShell.

## 10. Recommended Next Phase

Plan and review standalone compatibility-flag retirement, then continue the
scoped runtime authority and capability projection sequence already recorded
in the roadmap.

## 11. Governed Review Record

This review is part of the approved `dg/runtime-composition` phase:

- run ID: `run-1785412215467666000-2`;
- approval ID:
  `approval/run-1785412215467666000-2/composition-approved`;
- approval presentation ID: `presentation/6a50b9e8b9229921`;
- approval outcome: granted under delegated-maintainer authority; and
- approval-presentation enforcement: proof persisted before approval.

## 12. Validation

Canonical validation results are recorded in the implementation report and
must be green in GitHub CI before merge.
