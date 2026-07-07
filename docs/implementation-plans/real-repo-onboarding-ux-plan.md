# Real-Repo Onboarding UX Plan

Status: In progress. This follows external real-repository onboarding feedback against a public TypeScript package. The first implementation slice preserves existing `AGENTS.md` content by default in `workflow-os init-repo-governance` and `workflow-os init-agent-harness`. The second implementation slice adds bounded `package.json`/TypeScript metadata detection and concrete review-only first-run recommendations. The third implementation slice adds concise first-run summary output and labels the generated mock workflow as an optional approval/audit demo. The fourth implementation slice adds `workflow-os first-run --verbose`, making the default human output concise while preserving the full bounded posture matrix for audit-minded users. The fifth implementation slice adds bounded Rust, Python, Go, and GitHub Actions metadata labels plus review-only first-run recommendations without reading manifest bodies, executing commands, or generating workflows.

This plan is planning only. It does not implement source-content inspection, command execution, provider calls, automatic workflow generation, schema changes, examples, hosted behavior, writes, or release posture changes.

## 1. Executive Summary

Workflow OS now has a credible local first-run product loop:

1. `workflow-os validate` explains that a normal repo is missing `workflow-os.yml`.
2. `workflow-os init-repo-governance` creates a valid governance envelope.
3. `workflow-os first-run` emits bounded governance posture and workflow recommendations without creating runtime state or pretending to inspect more than it did.
4. The generated approval-gated workflow demonstrates pause/resume and durable event history.

External testing against a real repository confirmed this is valuable. It also exposed three P0 product gaps:

- existing `AGENTS.md` files are handled safely but too sharply;
- `first-run` needed safe repo metadata awareness beyond generic governance posture;
- the useful posture output is dense and blurs the difference between real first-run analysis and the mock approval/audit demo.

The next implementation should improve real-repo adoption without changing the product boundary. Workflow OS should preserve existing agent guidance by default, inspect only safe repository metadata, produce concrete first-run recommendations, and keep mock runtime demos clearly labeled as demos.

## 2. Goals

- Preserve existing repository agent guidance by default.
- Make `init-repo-governance` coexist with existing `AGENTS.md` instead of forcing replacement.
- Keep replacement available only through explicit `--force`.
- Add safe metadata-aware first-run detection for common repository shapes.
- Recommend concrete governed workflows/checks from metadata without executing commands.
- Keep recommendations review-only.
- Improve default human-readable `first-run` output so the most important next actions are obvious.
- Keep full posture output available for audit-minded users and machine readers.
- Clarify that `first-run` is real bounded governance posture and the generated mock workflow is an approval/audit-state demonstration.
- Preserve redaction-safe, local-only behavior.

## 3. Non-Goals

Do not implement in this lane:

- raw source file inspection;
- raw README, issue, PR, or code content analysis;
- arbitrary shell command execution;
- automatic local check execution;
- provider calls;
- GitHub, Jira, CI, npm, cargo, Python, Go, or other external writes;
- automatic workflow generation;
- automatic workflow registration or promotion;
- workflow schema changes;
- examples that imply production automation;
- hosted or distributed runtime behavior;
- recursive agents or agent swarms;
- Level 3/4 autonomy expansion;
- release posture changes.

## 4. User Feedback Summary

The external evaluator reported that Workflow OS already guided a normal repository from missing-manifest validation to scaffold, validation, first-run posture, approval-gated run, durable state, and inspectable event history.

Strong product signals:

- missing-manifest validation was actionable;
- scaffold generated a valid project;
- `first-run` produced useful bounded governance posture without overclaiming;
- approval pause/resume and event history made the product thesis tangible;
- unsupported behavior was honestly disclosed.

Gaps:

- existing `AGENTS.md` handling fails closed but pushes users toward `--force`;
- `first-run` does not yet detect safe metadata such as `package.json` scripts;
- first-run text is useful but dense;
- generated mock workflow can be mistaken for actual repository analysis.

## 5. Existing Agent Guidance Preservation

Real repositories increasingly contain `AGENTS.md`, `CLAUDE.md`, Cursor rules, Copilot instructions, or other agent guidance. Workflow OS should not make users choose between preserving that context and adopting governance.

Implemented first slice:

- If `AGENTS.md` does not exist, create the managed Workflow OS file as today.
- If `AGENTS.md` exists with a Workflow OS managed block, update only the managed block and preserve surrounding text.
- If `AGENTS.md` exists without a Workflow OS managed block, append a new managed block by default and preserve existing content.
- If `--force` is supplied, replace the file as today, but print a clearer bounded warning that existing repo-specific agent guidance will be replaced.
- In `--dry-run`, report that existing unmanaged content would be preserved and a Workflow OS managed block would be appended.

The managed block markers should remain deterministic:

```text
<!-- BEGIN WORKFLOW OS AGENT HARNESS -->
...
<!-- END WORKFLOW OS AGENT HARNESS -->
```

Errors and warnings must not echo existing file contents.

## 6. Safe Repo Metadata Inspection

`workflow-os first-run` should inspect safe repository metadata, not raw source contents.

Allowed first slice:

- `package.json` presence;
- `package.json` bounded metadata fields:
  - package manager posture if inferable from lockfiles;
  - script names and bounded command labels for common script keys such as `test`, `build`, `lint`, `typecheck`, `format`, `prepare`, `release`;
  - dependency counts, not dependency values if that risks noisy output;
- `Cargo.toml` presence and workspace/package posture;
- `pyproject.toml` presence and common tool sections by name;
- `go.mod` presence;
- `.github/workflows/*.yml` presence and count;
- source/test directory presence by conventional names only;
- README, license, contributing, security policy, code of conduct presence.

Disallowed:

- arbitrary source content;
- raw command output;
- raw test output;
- raw lockfile contents;
- raw package dependency lists in default human output;
- raw CI logs;
- private paths beyond bounded relative names;
- environment values;
- credentials or token-like values.

## 7. Concrete Recommendation Policy

First-run recommendations should remain review-only, but they should become more concrete when metadata supports it.

Examples:

- Detected TypeScript/npm package:
  - recommend implementation workflow using declared test/build scripts as validation obligations;
  - recommend PR review workflow requiring type/lint/test evidence when scripts are present;
  - recommend release readiness workflow requiring build and package metadata review;
  - recommend dependency update workflow requiring lockfile posture and declared tests.
- Detected Rust crate/workspace:
  - recommend implementation workflow with `cargo fmt`, `cargo clippy`, and `cargo test` obligations as review-only suggestions;
  - recommend release readiness workflow for crate metadata and changelog posture.
- Detected GitHub Actions:
  - recommend CI evidence workflow referencing workflow presence without reading logs.
- Missing license/security/contributing:
  - recommend governance setup tasks, not automatic fixes.

The output must say recommendations are not active workflows until reviewed and authored.

## 8. Human Output Shape

Default `first-run` output should lead with a concise operator summary:

```text
Workflow OS found a TypeScript package and a local governance envelope.

Ready now:
- validate the Workflow OS project
- run the approval/audit demo workflow if desired
- review suggested governed workflows

Needs setup:
- assign owner and escalation contact
- choose required checks
- decide side-effect posture before writes
```

The full field posture matrix is available through `workflow-os first-run --verbose`. JSON output remains bounded and machine-readable and continues to include the detailed posture fields.

## 9. Mock Workflow Demo Separation

The CLI should clearly separate:

- `workflow-os first-run`: real bounded repo governance posture and recommendation output;
- `workflow-os --mock-all-local-skills run local/first-run-governance`: approval/audit-state demonstration using a mock local skill.

Recommended wording:

```text
first-run: real bounded governance posture; no runtime state created
mock workflow: optional approval/audit demo; does not perform repository analysis
```

This avoids implying that the mock workflow produced the real repository insight.

## 10. First Implementation Sequence

1. Preserve existing `AGENTS.md` by default in `init-repo-governance` and `init-agent-harness`. Implemented in the existing agent-instruction preservation slice.
2. Add focused tests for unmanaged `AGENTS.md` preservation, dry-run messaging, managed block update, and explicit `--force` replacement warning. Implemented in the existing agent-instruction preservation slice.
3. Add safe metadata detection model/helper for `first-run`. Implemented for bounded `package.json`, package-manager lockfile posture, TypeScript markers, GitHub workflow count, conventional source/test directories, and common repo-document presence.
4. Add metadata-aware first-run output for npm/TypeScript first, because real-repo feedback supplied a concrete package case. Implemented as review-only recommendations; script command bodies and dependency values are not copied.
5. Improve default human summary while preserving bounded full detail and JSON output. Implemented through concise default text and `workflow-os first-run --verbose`.
6. Add richer Rust/Python/Go/GitHub Actions metadata in small follow-up slices. Implemented as bounded presence/lockfile/count labels and review-only recommendations.
7. Review before adding automatic workflow generation, schema changes, real local check execution, or provider integration.

## 11. Test Plan

Future implementation tests should cover:

- existing unmanaged `AGENTS.md` is preserved by default;
- Workflow OS managed block is appended when absent;
- Workflow OS managed block is updated when present;
- `--force` still replaces and prints a clear warning without leaking file content;
- `--dry-run` writes no files and says unmanaged content would be preserved;
- generated downstream `AGENTS.md` remains portable;
- `first-run` detects `package.json` safely;
- `first-run` reports common script keys without executing scripts;
- `first-run` does not copy raw source contents;
- `first-run` does not copy raw dependency lists into default output;
- TypeScript/npm metadata produces concrete review-only recommendations;
- absent metadata keeps generic recommendations;
- JSON output remains bounded;
- mock workflow next-step wording is clearly labeled as an approval/audit demo;
- existing validate, scaffold, first-run, runtime, state doctor, and docs tests still pass.

## 12. Privacy And Security

This lane must remain local and metadata-only.

Allowed output should be bounded labels, counts, relative file names, and script keys. Any script command text that appears should be treated as metadata and bounded; future implementation may prefer script key labels over full command strings in default output.

Do not print existing `AGENTS.md` content, source contents, token-like strings, private absolute paths, environment values, provider payloads, command outputs, CI logs, or raw dependency lists.

## 13. Documentation Updates

Update:

- `ROADMAP.md`;
- `docs/implementation-plans/existing-repo-governance-onboarding-plan.md`;
- `docs/implementation-plans/first-run-governed-ledger-report-plan.md` if needed;
- user guide material after implementation.

Docs must keep saying:

- first-run does not execute commands;
- mock local skills are preview/demo tooling;
- recommendations are review-only;
- workflow generation/registration is not automatic;
- source-content inspection is not implemented;
- provider writes, hosted behavior, schemas, examples, and release posture changes are not implemented by this lane.

## 14. Open Questions

- Should `--merge-agent-instructions` exist as an explicit alias even if preservation becomes default?
- Should full script command text appear in human output, JSON only, or neither by default?
- Should `.github/workflows` detection parse job names or only count workflow files in the first slice?
- Should `first-run` recommend package-manager-specific checks by name before local check handlers are configured?

## 15. Final Recommendation

Proceed next to safe `package.json`/TypeScript first-run metadata detection and concrete review-only recommendations.

Do not implement source-content inspection, command execution, automatic workflow generation, provider calls, schemas, examples, hosted behavior, writes, or release posture changes.
