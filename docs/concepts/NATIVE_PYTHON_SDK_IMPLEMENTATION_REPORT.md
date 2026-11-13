# Native Python SDK Implementation Report

## Files Changed

- New `crates/workflow-python` binding crate and workspace/lockfile entries.
- New `packages/sdk-python` package, types, example agent, and integration tests.
- New `.github/workflows/python-sdk.yml` and Python build artifact ignores.
- SDK guide/index, changelog entry, scoped implementation plan, and this report.
- No runtime or CLI implementation was changed. The Core build-input
  discovery parser and workspace fixture cover the native SDK's manifest
  and locked dependencies.

## Behavior Added or Changed

Python calls Rust Core in process through a PyO3 extension. Explicit local
callbacks run only when Core reaches their governed skill. The existing
GitHub metadata fixture path preserves proof-bound approval, input drift
checks, adapter execution, durable inspection, and terminal replay. The example
agent no longer invokes subprocesses or parses CLI text.

## Contracts Added or Changed

Experimental `WorkflowOS`, `Run`, and `WorkflowError` Python API; synchronous
string-map callback inputs/outputs; stable error codes; explicit local and
fixture modes. Local approval also resumes callbacks; fixture approval requires
a subsequent run. Existing Rust Core/CLI contracts and schemas are unchanged.

## Tests Added or Changed

18 Python integration tests run against the real compiled extension, including
separate Python process reopening, both directions of CLI/native fixture state
interoperability, and disabled Python process-launch entry points. Linux
CPython 3.10/3.14 and macOS CPython 3.12 wheel tests are configured in CI.

## Docs Added or Changed

The SDK guide documents install, callback registration, native agent flow,
mode-specific approval semantics, ephemeral outputs, trusted-code boundaries,
concurrency, versioning, and unsupported capabilities. The implementation plan
records the approved scope and dependency rationale.

## Validation Performed

- 18 Python integration tests passed against both the editable native package
  and a built wheel installed in a fresh virtual environment; no tests skipped
  when the CLI/native interoperability environment variable was supplied.
- All 15 existing `github_read_only_example` Rust integration tests passed.
- `cargo test -p workflow-python` passed linking and doc-test compilation;
  behavior coverage resides in the Python integration suite, not Rust unit tests.
- Rust formatting, SDK-only strict clippy (`--no-deps`), and SDK rustdoc with
  warnings denied passed.
- Repository documentation checker, release-state checker, and git whitespace
  check passed.
- A macOS arm64 ABI3 wheel was built and installed without network dependencies.
- The generated source distribution also built successfully offline. Maturin
  prunes workspace members in that archive, so its copied workspace lockfile
  required normal offline reconciliation; an initial `--locked` sdist build
  was rejected. Repository wheel builds use `--locked` successfully.
- Full strict clippy including Core was attempted and stopped on five existing
  documentation-markdown warnings in Core under local Rust 1.96. No unrelated
  Core edits or lint suppressions were added. Upstream base commit's published
  Rust CI was green; local SDK-only strict lint passed.
- The full upstream suite was not rerun locally; required exact-head hosted CI
  and maintainer review remain pending at PR preparation. No acceptance claim.

## Security and Privacy Considerations

The binding does not implement alternate policy or expose a generic dispatch
bypass. It rejects unsupported local governance profiles, unknown request fields,
non-local/duplicate callback registrations, and reentrant/concurrent SDK calls.
Python exception text is not stored in durable events. Inspection never creates
missing approval proof. Callbacks, the embedding app, and the state owner remain
trusted. Actor labels and delivered presentation records do not authenticate a
human or prove UI viewing. Callback identities are not code attestations.

## Assumptions Made

The embedding app preserves a run ID, displays full approval context, collects
human decisions outside the model's tool registry, and registers reviewed local
handlers. CPython and a supported Unix Core environment are available. The
GitHub fixture uses synthetic data and no credentials.

## Risks and Follow-Ups

In-process callbacks can block or crash the host and have its OS permissions.
The SDK operation lock does not serialize unrelated CLI/Core writers. This is
not an exactly-once external write protocol, callback sandbox, live generic
tool adapter, autonomous scheduler, or production SDK. Review native packaging,
callback trust, and per-mode approval semantics before adoption.

## Incomplete or Placeholder Work

No PyPI publication, production wheel release, general live provider expansion,
async callback execution, callback attestation, or forceful callback timeout is
included. The agent is a deterministic fixture harness, not an LLM integration.
No automatic persisted WorkReport is claimed. CI acceptance is pending review.

## Review Corrections

- Upgrade the native binding to PyO3 0.29.0 and refresh the lockfile for the
  two RustSec advisories reported against the original pin.
- Accept the SDK's explicit `cdylib` and `rlib` artifact declarations in the
  strict workspace build-input parser without admitting unknown types.
- Bind SDK approval to the explicit presentation ID returned by the human
  surface. The record identifies the workflow version and gated step and
  skill when present. Returning an ID is not evidence of UI viewing or
  authenticated identity; the embedding application owns that interaction.
- Validate without creating state and inspect an existing database read-only
  without taking the SDK writer lock. SQLite's WAL coordination is unchanged.
- Add regression tests for missing or mismatched presentation IDs before
  callback entry and for non-mutating inspection and validation.

The correction run is `run-1794441295441028000-2`, with delegated approval
`approval/run-1794441295441028000-2/implementation-approved` and
presentation `presentation/2ae3f86ed267d3a7`. This authorizes a draft update,
not merge or publication. The validation results above describe the original
submission; exact corrected-head CI is tracked on the PR.

## Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1794374156366948000-2`.
- Approval: `approval/run-1794374156366948000-2/implementation-approved`.
- Presentation: `presentation/8924e93805150fda`.
- Outcome: explicitly approved by the user in chat and recorded with
  `user/eliasdefaria`; phase-close status `Completed`.
- Event summary: 39 events, one approval request, one granted decision, eight
  policy decisions, six mock skill invocations, zero retries/escalations; the
  approval presentation proof marker was present.
- The dogfood workflow used deterministic mock handlers for governance only.
  Editing, dependency installation, builds, tests, packaging, git operations,
  and PR preparation were executed outside the kernel and are not represented
  as kernel-executed implementation or validation. This report is a repository
  document, not a generated durable WorkReport artifact.
