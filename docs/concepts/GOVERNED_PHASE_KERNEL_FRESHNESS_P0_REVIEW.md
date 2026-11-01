# Governed Phase Kernel Freshness P0 Review

## Verdict

Accepted. The implementation satisfies the P0 invariant for the repo-local
governed phase runner. Terminal-green required GitHub checks remain the merge
boundary.

## Reviewed Scope

- live material phase-start build ordering;
- `--no-build` bypass resistance;
- build-failure behavior before kernel invocation;
- dry-run and non-phase compatibility;
- focused fixture evidence;
- roadmap, runbook, and implementation-report truth.

## Findings

No blocking finding remains.

The initial focused tests proved that the live path invoked both Cargo and the
kernel but used separate logs. Review tightened that evidence to one shared
event log. The accepted proof now observes this exact order:

1. `cargo build -p workflow-cli --bin workflow-os`;
2. project validation;
3. governed run creation; and
4. approval-presentation persistence.

The failure test proves that a non-zero build exit invokes no kernel command.
Because run creation, approval creation, and presentation persistence are
kernel operations, no governed phase state can be created through this helper
after a failed build.

## Security And Correctness Assessment

- An existing binary cannot bypass the build on a live material phase start.
- `--no-build` fails before validation or run creation on that path.
- Work-context validation still runs before the build and creates no governed
  state.
- Dry-run behavior remains non-authorizing and does not require a build.
- Non-phase helper behavior is unchanged.
- The freshness disclosure is emitted only after the build returns success.

The contract intentionally relies on Cargo resolving the current workspace and
source graph. It does not add an embedded commit identity, binary attestation,
clean-room build, artifact signature, or reproducible-build claim. Those are
not required for this repo-local P0 invariant.

## Validation

- `npm run test:dogfood-helper`: 35 passed;
- `npm run check:docs`: passed;
- `git diff --check`: passed.

The review phase used:

- workflow: `dg/review`;
- run: `run-1793523009624484000-2`;
- approval: `approval/run-1793523009624484000-2/review-scope-approved`;
- presentation: `presentation/e3bb31f01a05cac5`;
- approval outcome: granted under delegated maintainer authority.

## Non-Goals Confirmed

No kernel runtime semantics, provider access, schemas, releases, automatic
approval, unrelated refactor, or paused Phase 2 implementation was added.

## Next Step

Commit the exact reviewed tree, run governed PR hygiene, require terminal-green
checks for that commit, merge, synchronize the original checkout, and only then
resume the preserved Phase 2 implementation.
