# First-Run Authoring Command Guidance Implementation Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The implementation adds a bounded, review-only bridge from `workflow-os
first-run` recommendations to recommendation detail and authoring dry-run
commands. It stays within the approved product boundary: no automatic workflow
generation, no file writing by default, no promotion, no runtime execution, no
local checks, no provider calls, no schemas, no examples, no hosted behavior,
and no writes.

## 2. Scope Verification

The phase stayed within the approved implementation scope.

Implemented:

- default human `first-run` output includes `authoring_command_guidance`;
- guidance selects one already-computed recommendation;
- guidance prints `workflow-os first-run --recommendation <id>`;
- guidance prints `workflow-os author workflow --from-recommendation <id>
  --dry-run` only for workflow-creation candidates;
- CLI docs, roadmap, focused tests, and an implementation report were updated.

Not introduced:

- automatic workflow generation;
- active workflow registration;
- automatic draft file writing;
- automatic workflow promotion;
- workflow execution from first-run;
- approval execution;
- local check execution;
- provider calls;
- runtime state creation;
- report artifact writing;
- schema changes;
- examples;
- hosted or distributed behavior;
- write-capable adapters;
- recursive agents or agent swarms;
- Level 3/4 autonomy;
- release posture changes.

## 3. Behavior Assessment

The selected behavior is appropriate for the first-use UX gap it targets.

The implementation makes the next safe commands visible without turning
recommendations into active workflows. The default output now gives operators a
copyable path:

- inspect one recommendation;
- preview inactive workflow authoring with `--dry-run`;
- remain in `review_only_non_mutating` posture.

The selector is deterministic and bounded. It prioritizes concrete ecosystem
implementation recommendations, then generic implementation, validation
obligations, ownership, and report handoff. It does not fabricate
recommendation IDs; the selected ID must already exist in the computed
recommendation set.

## 4. Safety And Privacy Assessment

The implementation preserves the established first-run safety boundary.

Verified:

- command guidance is built from static command tokens and existing bounded
  recommendation IDs;
- package script bodies are not printed;
- dependency values are not printed;
- source file contents are not read or printed;
- command output is not printed;
- provider payloads are not used;
- environment values, credentials, and token-like values are not used;
- guidance does not suggest draft output, promotion, workflow execution,
  approval, local check execution, or provider commands.

The new output remains a disclosure and authoring-guidance surface, not an
execution surface.

## 5. Test Quality Assessment

Focused test coverage is appropriate for this small CLI UX slice.

Tests cover:

- default output includes authoring guidance for the generic repository
  recommendation;
- default output includes recommendation detail guidance;
- default output includes authoring dry-run guidance for workflow-creation
  recommendations;
- TypeScript metadata selects `first_run.typescript_implementation`;
- secret-like package script command bodies are not copied;
- default guidance does not include `--output workflows/drafts`;
- default guidance does not include `author workflow promote`;
- first-run does not create runtime state;
- existing first-run tests continue to cover concise and verbose behavior.

No blocker-grade test gaps were found.

Non-blocking follow-up: preview JSON intentionally remains unchanged. If
machine consumers need this command guidance, add a separately scoped JSON
surface with tokenized/static command guidance and compatibility notes.

## 6. Documentation Assessment

Documentation is consistent with the implemented behavior.

Verified:

- `docs/cli/first-run.md` documents `authoring_command_guidance`;
- the docs state that recommendation guidance is non-mutating;
- file output remains separate and explicit;
- promotion remains separate;
- recommendations remain review-only;
- no runtime execution, provider calls, local check execution, schemas,
  examples, writes, hosted behavior, or release posture changes are claimed;
- `ROADMAP.md` marks the implementation complete and links the implementation
  report.

## 7. Dogfood Governance

- Workflow: `dg/review`
- Run ID: `run-1783740360302165000-2`
- Approval ID: `approval/run-1783740360302165000-2/review-scope-approved`
- Approval presentation ID: `presentation/7fea551db4cfbdc6`
- Approval presentation hash:
  `7fea551db4cfbdc60e0ad39af0c041addf5170b0cc1aa0827893b6b94338761d`
- Approval outcome: delegated maintainer approved.
- Approval-presentation proof: persisted and enforced.

## 8. Validation

Required validation for this review phase:

```sh
npm run check:docs
git diff --check
```

The implementation phase already ran:

```sh
cargo test -p workflow-cli --test cli first_run_default_authoring_guidance_selects_typescript_recommendation
cargo test -p workflow-cli --test cli first_run
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

## 9. Blockers

None.

## 10. Non-Blocking Follow-Ups

- Consider a separately scoped preview JSON command-guidance surface if
  machine consumers need the selected recommendation and command tokens.
- Continue product polish around distinguishing detected repository metadata
  from scaffold-generated governance files in `first-run` output.

## 11. Recommended Next Phase

First-run detected-vs-scaffolded metadata clarity implementation.

Recent real-repository feedback identified a small but important ambiguity:
some first-run metadata can read as repository detection even when it comes from
the generated Workflow OS scaffold. The next implementation should make that
distinction explicit without reading raw source contents, executing commands,
generating workflows automatically, or changing runtime behavior.
