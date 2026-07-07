# Broader Ecosystem First-Run Metadata Review

## 1. Executive Verdict

Phase accepted with non-blocking follow-ups.

The broader ecosystem first-run metadata slice stays within the approved real-repo onboarding boundary. It makes `workflow-os first-run` more concrete for Rust, Python, Go, and GitHub Actions repositories while preserving the local, metadata-only, review-only posture.

## 2. Scope Verification

The phase stayed within scope.

Confirmed in scope:

- bounded Rust metadata labels for `Cargo.toml` and `Cargo.lock` presence;
- bounded Python metadata labels for `pyproject.toml` and allowlisted lock/requirements file names;
- bounded Go metadata labels for `go.mod` and `go.sum` presence;
- bounded GitHub Actions workflow count and detection flag;
- review-only workflow discovery recommendations for detected ecosystems;
- focused text and JSON tests for bounded output and non-leakage;
- docs and roadmap updates.

No accidental implementation found for:

- source-content inspection;
- manifest-body interpretation for Rust, Python, Go, or GitHub Actions;
- command execution;
- local check execution;
- provider calls;
- automatic workflow generation or registration;
- workflow schema changes;
- examples;
- hosted/distributed runtime;
- writes;
- recursive agents, agent swarms, or Level 3/4 autonomy;
- release posture changes.

## 3. Implementation Assessment

The implementation is appropriately small and product-shaped.

`SafeRepoMetadata` now records presence/count/allowlisted labels for additional ecosystem files. The detection helpers are intentionally shallow: `rust_detected`, `python_detected`, `go_detected`, and `github_actions_detected` are based on file presence or workflow count, not file bodies.

The recommendation helpers keep the new output review-only. Detected ecosystem posture can add implementation-workflow and validation-obligation recommendations, but those recommendations do not activate local checks, mark commands as required, or create workflow files.

## 4. Privacy And Redaction Assessment

The privacy boundary is preserved.

Verified behavior:

- Rust, Python, Go, and GitHub Actions file contents are not read into output.
- Lockfile contents are not copied.
- Source and test contents are not copied.
- GitHub Actions workflow names and bodies are not copied.
- Output uses bounded labels such as `cargo_toml`, `uv_lock`, `go_mod`, and workflow counts.
- JSON output mirrors bounded labels and booleans.

The tests intentionally place secret-like strings inside manifests, lockfiles, workflow files, source files, and test files, then assert those values do not appear in text or JSON output.

## 5. Recommendation Quality Assessment

The new recommendations are useful without overclaiming.

The recommendations are concrete enough to help a maintainer see what governed workflow structure might come next:

- Rust implementation and validation obligations;
- Python implementation and validation obligations;
- Go implementation and validation obligations;
- GitHub Actions CI evidence obligations.

They remain explicitly `review_only`, which is important because Workflow OS still does not know the correct commands, authority model, check policy, or workflow ownership for the repository from metadata alone.

## 6. Diagnostic And Behavior Preservation

The phase does not change first-run execution semantics.

`workflow-os first-run` still:

- validates the local Workflow OS project;
- emits a report-ready context rather than a terminal `WorkReport`;
- does not create runtime state;
- does not append workflow events;
- does not run checks;
- does not execute workflows;
- does not call providers.

The default output remains concise, while `--verbose` and `--json` carry the detailed bounded metadata.

## 7. Test Quality Assessment

Test coverage is focused and meaningful.

Reviewed coverage includes:

- verbose output for Rust, Python, Go, and GitHub Actions labels;
- review-only recommendation IDs for each detected ecosystem;
- JSON output for bounded booleans/counts/allowlisted labels;
- non-leakage of manifest, lockfile, workflow, source, and test payloads;
- no runtime state creation.

Existing first-run tests continue to cover package/TypeScript posture, default/verbose output behavior, JSON shape, ownership/escalation warnings, field coverage, and scaffold behavior.

Non-blocking gap: follow-up tests could cover mixed repositories where only a lockfile exists without its manifest, to confirm output remains purely descriptive and does not infer an ecosystem too aggressively.

## 8. Documentation Review

Docs accurately state that the slice is implemented and bounded.

Reviewed docs say:

- first-run detects bounded safe repository metadata;
- Rust/Python/Go/GitHub Actions recommendations are review-only;
- first-run does not execute commands;
- first-run does not read raw source, manifest, lockfile, workflow, parser, provider, or command-output payloads;
- workflow generation/registration is not automatic;
- schemas, examples, hosted behavior, writes, recursive agents, agent swarms, Level 3/4 autonomy, and release posture changes remain unimplemented.

## 9. Governed Dogfood Summary

- Workflow: `dg/review`.
- Run ID: `run-1783390194995234000-2`.
- Approval ID: `approval/run-1783390194995234000-2/review-scope-approved`.
- Approval outcome: granted by delegated maintainer after the complete approval handoff block was surfaced.
- Scope: broader ecosystem first-run metadata implementation review.

## 10. Validation Commands Run

- `npm run dogfood:benchmark -- phase-start --phase review ...`: passed after the runner rejected an overlong strict-non-goals field and the field was shortened.
- `./target/debug/workflow-os --project-dir ./dogfood/workflow-os-self-governance --state-dir /var/folders/r9/y7_mqmq108z94yhyt702h2b80000gn/T/workflow-os-self-governance-state --mock-all-local-skills approve run-1783390194995234000-2 approval/run-1783390194995234000-2/review-scope-approved --actor user/delegated-maintainer --reason approved-broader-ecosystem-first-run-metadata-review`: passed.
- Focused implementation/test/doc inspection: passed.
- `npm run check:docs`: passed.

## 11. Blockers

None.

## 12. Non-Blocking Follow-Ups

- Add mixed-metadata tests for lockfiles without corresponding manifests.
- Consider a future bounded recommendation for missing ecosystem lockfiles, but only as review-only posture and only after deciding whether that creates noisy or misleading guidance.
- Keep the next product work focused on first-run user guidance, not automatic workflow generation.

## 13. Recommended Next Phase

Recommended next phase: first-run workflow recommendation next-action refinement.

The onboarding loop is now metadata-aware across common ecosystems. The next useful product step is to make those recommendations easier for a user or agent to act on without generating workflows automatically: clearer next-action grouping, explicit “review before authoring” posture, and possibly a bounded `first-run` recommendation detail view. This should remain local, review-only, metadata-only, and non-executing.
