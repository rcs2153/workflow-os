# First-Run Metadata Provenance Clarity Review

## 1. Executive Verdict

Phase accepted; proceed to the next first-run product bridge phase.

The implementation fixes the specific first-run ambiguity where a freshly
scaffolded Workflow OS project could report generated `tests/` content as if it
were user repository test metadata. The phase stayed narrow, preserved bounded
metadata behavior, and did not add raw source inspection, command execution,
workflow generation, schemas, examples, hosted behavior, writes, or release
posture changes.

## 2. Scope Verification

The phase stayed within approved scope.

Implemented:

- scaffold-only `tests/` detection for the generated
  `tests/first-run-governance.test.yml` file;
- exclusion of scaffold-only `tests/` from conventional repository
  `test_dirs`;
- `workflow_os_scaffold_dirs` in verbose first-run text output;
- `workflow_os_scaffold_dirs` in preview JSON first-run output;
- focused tests and documentation updates;
- an end-of-phase report.

Not implemented:

- raw source inspection;
- raw manifest-body copying;
- command execution;
- local check execution;
- provider calls;
- workflow generation or registration;
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

The selected behavior is appropriate for the first-run product boundary.

Before this phase, a scaffolded repository with only the generated
`tests/first-run-governance.test.yml` file could display `test_dirs: tests`,
which looked like a detected user repository test directory. The new behavior
reports that case as:

```text
test_dirs: none
workflow_os_scaffold_dirs: tests
```

That is the right user-facing distinction: Workflow OS generated governance
files are not confused with user repository implementation or validation
metadata.

## 4. Bounded Metadata Assessment

The implementation remains bounded.

`workflow_os_scaffold_dirs` checks file names in the generated scaffold
directory and does not read file contents. It ignores dotfiles, rejects unknown
non-dot entries as scaffold-only proof, and only classifies `tests/` as a
Workflow OS scaffold directory when the visible entries are the expected
generated first-run test spec.

This preserves the existing first-run posture:

- no source-content reads;
- no package script body copying;
- no command execution;
- no provider calls;
- no runtime state creation;
- no local check execution.

## 5. User Metadata Preservation Assessment

User repository metadata remains preserved.

The existing package metadata tests now assert that a user-created `test/`
directory is still reported as `test_dirs: test` while the generated Workflow OS
`tests/` directory is separately reported as `workflow_os_scaffold_dirs: tests`.

The JSON test with user-created `tests/lib.test.ts` still reports:

```json
"conventional_test_dirs": ["tests"],
"workflow_os_scaffold_dirs": []
```

This verifies that the implementation does not hide real user test directories.

## 6. Privacy And Redaction Assessment

The privacy posture is acceptable.

The phase does not introduce storage or output of:

- raw source contents;
- raw spec contents;
- raw package script bodies;
- raw parser payloads;
- raw command output;
- provider payloads;
- environment values;
- credentials or token-like values.

Existing metadata privacy tests still assert that secret-like package script
payloads and dependency markers do not appear in first-run output.

## 7. Test Quality Assessment

The test coverage is focused and sufficient for this phase.

Covered:

- scaffold-only `tests/` is not reported as conventional user test metadata;
- scaffold-only `tests/` is reported as `workflow_os_scaffold_dirs`;
- preview JSON reports scaffold-only provenance;
- user `test/` remains conventional test metadata;
- user `tests/` remains conventional test metadata;
- existing package metadata privacy behavior remains intact.

No blocker-level test gaps were found.

Non-blocking future test opportunity: if Workflow OS adds more generated
directories that overlap common user repository metadata names, add similarly
focused provenance tests for those directories at the time they become
first-run metadata candidates.

## 8. Documentation Review

Documentation is honest and aligned.

The first-run CLI docs now explain that `safe_repo_metadata.test_dirs` reports
conventional user repository test directories only, while a scaffold-only
generated `tests/` directory is reported under `workflow_os_scaffold_dirs`.

The roadmap links the phase report and states the implemented boundary. The
phase report clearly lists completed scope, explicit non-scope, safety posture,
tests, dogfood governance, validation commands, limitations, and recommended
next phase.

## 9. Validation Assessment

The implementation report records the full validation suite as passing:

```sh
cargo test -p workflow-cli --test cli first_run_separates_scaffold_only_test_dir_from_repo_metadata
cargo test -p workflow-cli --test cli first_run
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run check:docs
git diff --check
```

For this review, documentation validation was rerun:

```sh
npm run check:docs
```

Result: passed.

Broader Rust validation was not rerun during review because this review found
no implementation blocker and the full suite was already run during the
implementation phase.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- Add scaffold provenance handling for any future generated directories only
  when those directories become first-run metadata candidates.
- Continue the product bridge from first-run recommendations into reviewed
  workflow authoring and promotion, keeping generated or scaffolded content
  clearly separated from detected user repository metadata.

## 12. Recommended Next Phase

Proceed to the next first-run product bridge phase.

The sharpest remaining user-facing gap from recent real-repo testing is still
the transition from useful first-run recommendations to concrete, reviewed
workflow authoring. Metadata provenance is now clearer; the next work should
continue making the recommendation-to-authoring path explicit, bounded,
non-mutating by default, and honest about what is generated versus detected.

## 13. Dogfood Governance

- Workflow: `dg/review`
- Run ID: `run-1783742548785262000-2`
- Approval ID:
  `approval/run-1783742548785262000-2/review-scope-approved`
- Approval presentation ID: `presentation/e31fd9f0dfa03a8e`
- Approval presentation hash:
  `e31fd9f0dfa03a8e7389758b4cc6e1aab3bf6de860562b1c3c5deaa24d284114`
- Approval outcome: delegated maintainer approved.
- Work performed outside the kernel: documentation review file creation and
  validation command execution.
