# WorkReport Artifact Identity Binding Independent Rerun

## Verdict

Pass. The independent clean-room backend-misplacement rerun satisfies the final
acceptance item for continuity-of-authority Gate 2, **Verified Means Correctly
Bound**.

Combined with the accepted focused review, exact-head CI, credential-backed
PostgreSQL execution, unchanged-head merge, and maintainer-operated post-merge
evidence, this rerun closes Gate 2.

## Bound Revision And Environment

- exact `main`: `b2cfdbec33efe6bdf8be709e464d38c5b373f5f1`
- disposable checkout:
  `/private/tmp/workflow-os-gate2-rerun.VoLwhP/WorkflowOS`
- platform: Darwin ARM64
- Rust: `rustc 1.99.0`
- Cargo: `cargo 1.99.0`
- final detached-checkout status: clean
- primary workspace: untouched
- commits or pushes: none

The evaluator verified the remote `main` identity, cloned into the disposable
location, and detached at the exact revision before running tests.

## Commands Executed

```sh
git ls-remote origin refs/heads/main
git clone --no-hardlinks /Users/rsegar/Documents/WorkflowOS WorkflowOS
git switch --detach b2cfdbec33efe6bdf8be709e464d38c5b373f5f1
git rev-parse HEAD
cargo test -p workflow-core --test work_report
cargo test -p workflow-cli --test cli \
  inspect_and_terminal_replay_reject_valid_work_report_from_another_run_slot \
  -- --exact --nocapture
cargo test -p workflow-core --test postgres_state_backend --no-run
git status --short --untracked-files=all
git diff --check
```

## Results

- complete WorkReport target: 243 passed, 0 failed;
- filesystem wrong run/report slot: rejected;
- filesystem wrong immutable identity plus corrupt body: rejected with
  `work_report_artifact.read.location_mismatch` before body mismatch;
- SQLite wrong relational keys: rejected;
- SQLite wrong immutable identity plus corrupt body: rejected with
  `work_report_artifact.read.location_mismatch` before body mismatch;
- human and JSON inspect: wrong-slot substitution rejected before projection;
- terminal replay: substitution rejected without `already_persisted`;
- PostgreSQL adversarial scenarios: present and compiled successfully; and
- checked errors: payload-free.

## Skipped And Evidence Boundary

The evaluator did not run credential-backed PostgreSQL because
`WORKFLOW_OS_TEST_POSTGRES_URL` was unset. It did not claim live database
execution. The exact reviewed head's successful Shared PostgreSQL State CI job
supplies that separate evidence.

The evaluator also skipped the full workspace suite, a second GitHub CI run,
and opt-in live provider tests. Those checks were not required for this bounded
independent rerun and their omission does not broaden the verdict.

This is an independent agent-context rerun, not an external human security
audit. It satisfies the plan's independent backend-misplacement rerun
criterion; it does not replace the repository's broader independent-assurance
requirements before a v1 claim.

## Governed Rerun Evidence

- workflow: `dg/review`
- run: `run-1793885544423249000-2`
- approval:
  `approval/run-1793885544423249000-2/review-scope-approved`
- presentation: `presentation/b60b1023089ec0a2`
- presentation content hash:
  `b60b1023089ec0a22921c6c26f3ddf4d5a975479879c71e58f4a19a064c0a0c1`
- approval outcome: granted
- phase status: completed
- events: 39
- approvals: 1
- retries: 0
- escalations: 0

The kernel governed the review boundary and retained its audit trail. The
independent evaluator performed repository reads, test execution, and clean-
room verification outside the kernel under that approved scope.

## Gate Status

Gate 2 is closed. Gate 3, project-resource authorization continuity, is now the
next reviewed implementation gate. This report does not authorize or implement
Gate 3 behavior.
