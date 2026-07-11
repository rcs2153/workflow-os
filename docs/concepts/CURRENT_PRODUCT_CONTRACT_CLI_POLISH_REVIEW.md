# Current Product Contract And CLI Polish Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The phase fixed the concrete first-use trust issues surfaced by external
real-repository testing without expanding runtime scope. `workflow-os
--version`, `workflow-os version`, and bounded JSON version output are now
available without requiring a Workflow OS project. The stale known-limitations
claim about missing project initialization is corrected, the
`init-repo-governance` generated-file docs now include `policies/local.policy.yml`,
and the new Current Product Contract gives evaluators a concise current-state
boundary.

## 2. Scope Verification

The phase stayed within the approved polish scope.

Implemented:

- CLI version reporting.
- Regression tests for version output.
- Known-limitations correction.
- `init-repo-governance` generated-file documentation correction.
- Current Product Contract user guide.
- Links from the CLI overview and user-guide index.
- End-of-phase report.

Not introduced:

- new runtime primitives;
- provider-write behavior;
- write-capable adapters;
- schema changes;
- examples;
- hosted or distributed runtime;
- report artifact automation;
- automatic local check execution;
- reasoning lineage;
- release posture changes.

## 3. CLI Behavior Assessment

The version behavior is appropriate for the current preview:

- `workflow-os --version` prints `workflow-os 0.2.0-preview.1`.
- `workflow-os version` prints `workflow-os 0.2.0-preview.1`.
- `workflow-os --json version` prints a bounded object containing `name`,
  `version`, `schema_version`, and `release_posture`.
- The command does not require `workflow-os.yml` or local runtime state.

This resolves the evaluator-reported CLI papercut without changing executor,
project loader, adapter, or runtime behavior.

## 4. Documentation Assessment

The documentation updates are accurate and bounded:

- `docs/release/V0_KNOWN_LIMITATIONS.md` no longer says project
  initialization is missing.
- The same limitations document still clearly says docs generation, generic
  live adapter execution commands, distributed worker commands, hosted
  operation, and production deployment commands are not implemented.
- `docs/cli/init-repo-governance.md` lists `policies/local.policy.yml`.
- `docs/user-guide/current-product-contract.md` separates real behavior from
  mock/demo behavior and unsupported behavior.
- `docs/cli/overview.md` and `docs/user-guide/README.md` link the current
  product contract.

The Current Product Contract does not overclaim production readiness,
write-capable adapters, hosted behavior, recursive agents, automatic local
checks, or reasoning lineage.

## 5. Product Contract Assessment

The new product contract is useful because it gives evaluators a short,
current-state truth surface instead of forcing them to infer the project
boundary from roadmap history and phase reports.

It correctly identifies the current product as a local-first governance kernel
and preserves the core operating boundary:

```text
Agent executes. Workflow OS governs.
```

It also makes the important distinction that repo-local `dg/*` workflows are
Workflow OS dogfood benchmark workflows for this repository, not downstream
community defaults or plug-and-play user workflows.

## 6. Safety And Boundary Assessment

No safety boundary was relaxed.

The phase does not:

- execute local checks automatically;
- enable shell execution;
- create reports or artifacts automatically;
- call live providers;
- write to external systems;
- change approval semantics;
- change policy evaluation semantics;
- change runtime state behavior.

The version output and docs corrections are safe, deterministic, and
non-sensitive.

## 7. Test Quality Assessment

The added CLI tests cover:

- `workflow-os --version`;
- `workflow-os version`;
- bounded JSON version output;
- project-independent execution.

The existing workspace suite also covers the surrounding CLI, validation,
runtime, adapter, report, artifact, hook, side-effect, and policy surfaces.
This is sufficient for a small CLI/docs polish phase.

## 8. Dogfood Governance

The implementation phase was governed by `dg/implement`:

- run ID: `run-1783735062078181000-2`
- approval ID: `approval/run-1783735062078181000-2/implementation-approved`
- presentation ID: `presentation/0d956fa872e4048d`
- presentation hash:
  `0d956fa872e4048d6bbeedc09364f0af87a8da7cae4ea3ca0d866354ac7f92fe`
- approval outcome: granted
- close status: completed

This review phase was governed by `dg/review`:

- run ID: `run-1783736568002315000-2`
- approval ID: `approval/run-1783736568002315000-2/review-scope-approved`
- presentation ID: `presentation/8f08039080de386a`
- presentation hash:
  `8f08039080de386a8dcbaea3f677b2ffc3eeca7401423ea7e02faae0073bc110`
- approval outcome: granted by delegated maintainer authority

## 9. Validation

Implementation validation:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.

Review spot checks:

```sh
./target/debug/workflow-os --version
./target/debug/workflow-os version
./target/debug/workflow-os --json version
```

Result: passed.

Review validation after writing this review:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

Result: passed.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Continue improving the bridge from `first-run` recommendations to reviewed
  workflow authoring and promotion.
- Consider making `first-run` distinguish detected user repository metadata
  from generated Workflow OS scaffold files more explicitly.
- Keep the Current Product Contract updated whenever CLI/runtime behavior moves
  from planned to implemented.

## 12. Recommended Next Phase

Recommended next phase: first-run recommendation to workflow authoring bridge
planning.

Reason: the current feedback loop repeatedly shows that `first-run` is the
strongest first-use product signal. The next runtime/product unlock is making
the transition from recommendation to reviewed, inactive workflow draft feel
clearer and more concrete, without turning recommendations into automatic
execution or overclaiming real handler/check coverage.
