# Capability-Gated GitHub Read-Only Core-Private Authority Lifecycle Review

## Verdict

Accepted after blocker correction. Core owns the complete immutable operation
interpretation and the exact wrapper has database-backed lifecycle evidence.
Phase 2 remains private, no-write, and unreachable from ordinary CLI execution.

## Accepted Findings

- Core derives the grant, availability, source commitment, generation, and
  watermark rather than accepting caller-built authority inventories.
- The grant binds actor, workflow, run, step, harness, resource, sensitivity,
  and expiry.
- Missing access produces declared-not-connected availability.
- The only accepted reference is `env/WORKFLOW_OS_GITHUB_TOKEN`; generic host
  fallback is absent.
- Access values are closure-scoped, non-cloneable, non-serializable, and
  Debug-redacted.
- Provider success, terminal failure, and may-have-started ambiguity map to
  distinct accepted supervisor outcomes.
- The helper remains crate-private, feature-gated, no-write, and unreachable
  from ordinary CLI execution.

## Corrected Contract-Closure Finding

The original top-level helper accepted project configuration, target, and
`SkillInput` separately from `RequiredContextExecutionBinding`. That was the
wrong authority boundary.

The corrected helper reloads the exact stored bundle and proves that:

- the bound step selects the exact GitHub repository metadata skill;
- the authored operation is exactly `github.repository.metadata`;
- the complete capability, input, and policy contract is present;
- owner and repository come from immutable literal mappings; and
- the target is present in the immutable project allowlist.

Core constructs the target, skill input, and operation commitment from those
records. Callers retain only ephemeral execution dependencies and lifecycle
identities. A provider implementation can no longer choose its own governance
binding commitment.

## Closed Evidence Gap

The exact `prepare_atomic_github_repository_metadata_read` wrapper is now
exercised through SQLite. Focused tests prove:

1. successful preparation and execution invokes the operation once and reaches
   terminal posture;
2. terminal replay does not re-enter the provider;
3. may-have-started ambiguity is persisted without blind redispatch; and
4. the forbidden access marker is absent from durable files and bounded output.

The correction reuses the existing trusted-host operational-window, attempt,
dispatch, terminal, and ambiguity state machine. It does not add a public
admission API or a parallel lifecycle.

## Validation

- `cargo test -p workflow-core --all-features github_repository --lib`: 43
  passed;
- `cargo clippy -p workflow-core --all-features --all-targets -- -D warnings`:
  passed;
- `cargo fmt --all -- --check`: passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

## Governance

- implementation workflow: `dg/implement`;
- resumed implementation run: `run-1793523827842712000-2`;
- initial review run: `run-1793524069028290000-2`;
- blocker workflow: `dg/blocker`;
- replacement blocker run: `run-1793524717964440000-2`;
- blocker approval:
  `approval/run-1793524717964440000-2/fix-approved`;
- blocker presentation proof: `presentation/ca1cf667087895fe`;
- focused re-review run: `run-1793524741210861000-2`;
- focused re-review approval:
  `approval/run-1793524741210861000-2/review-scope-approved`;
- focused re-review presentation proof: `presentation/7b22badc4a3a2e1f`;
- approval outcome: granted under delegated maintainer authority;
- kernel freshness: rebuilt from the exact synchronized checkout before each
  governed run.

## Next Step

Proceed to Phase 3 only through a new governed scope. Phase 2 acceptance does
not authorize public provider activation, network access, mutations, schema or
SDK exposure, or broader adapter families.
