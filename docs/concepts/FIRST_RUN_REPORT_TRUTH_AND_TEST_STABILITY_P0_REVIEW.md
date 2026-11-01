# First-Run Report Truth And Test Stability P0 Review

## Decision

Accepted for pull-request validation. Final blocker acceptance still requires
all configured GitHub checks to be terminal-green for the exact committed head.

The focused maintainer/security review found no correctness, security, privacy,
or scope blocker in the implementation.

## Governed Review

- Workflow: `dg/review`
- Run: `run-1793521482992281000-2`
- Approval:
  `approval/run-1793521482992281000-2/review-scope-approved`
- Presentation: `presentation/087cddf80d5908d3`
- Outcome: granted under delegated maintainer authority after the complete
  review handoff was presented

## Findings

No blocking findings.

The review verified:

- aggregate-governance and authored-step presentation text is selected from
  the typed approval subject rather than inferred from prose or call order;
- aggregate approval explicitly does not authorize an authored step;
- terminal approval summaries derive request, grant, denial, and undecided
  counts from durable run state;
- the first-run artifact report and generic terminal report retain
  high-assurance disclosure while reporting all run approvals;
- operator handoff uses the supported `workflow-os inspect <run-id>` command;
- generated guidance accurately distinguishes unsupported automatic artifact
  generation from the explicit bounded first-run artifact path;
- test fixture roots include process, time, and an in-process atomic sequence,
  and unique root creation fails closed on collision; and
- the implementation does not broaden provider, write, runtime selection,
  hosted, schema, or approval-authority surfaces.

## Validation Reviewed

- focused CLI end-to-end coverage proves two distinct approval-presentation
  records, two requests, two grants, truthful report text, and a valid inspect
  handoff;
- focused artifact-path coverage proves the valid inspect handoff;
- the formerly flaky local-host module passed six concurrent repetitions, 102
  executed tests total, with only the explicit opt-in smoke ignored;
- `npm run check` passed;
- `cargo test --workspace --all-targets` passed;
- `cargo clippy --workspace --all-targets -- -D warnings` passed;
- `cargo fmt --all -- --check` passed; and
- `git diff --check` passed.

The earlier failed first-run release-worktree probes are unrelated to this P0
branch: they lacked required local CLI and Cargo executable setup, then passed
without source changes once the documented execution environment was supplied.

## Remaining Acceptance Gate

Commit the reviewed tree, push one pull-request head, and require every
configured GitHub check to complete successfully on that exact head. A rerun
must not be used to waive a deterministic failure. If any required check fails,
the blocker remains open until the cause is fixed and the new exact head is
reviewed and green.
