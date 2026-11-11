# Native Python SDK implementation scope

Status: user-authorized implementation submitted for review; not an accepted
release or production SDK contract.

## Problem and scope

Python harnesses currently shell out to the CLI and parse presentation text.
Add a native extension linked to Rust Core, a small typed Python facade, and
explicit Python callback registration through the existing `SkillHandler` SPI.
Port the GitHub metadata fixture agent without weakening its approval, input
commitment, or replay behavior.

The user approved PyO3 bindings, Python packaging and a typed API, explicitly
registered local callbacks, existing GitHub fixture run/approval/inspection
parity, tests, documentation, and a draft PR. No automatic approvals, arbitrary
tool interception, new live writes, sandbox claims, policy bypasses, package
publication, merge, or release were authorized.

## Architecture and dependencies

- Keep every policy decision and durable state transition in Rust Core.
- Add `workflow-python` as a separate workspace crate, with an optional
  `extension-module` feature used by maturin. Ordinary Rust test linking does
  not enable that feature.
- Pin PyO3 0.27.2 for the CPython ABI boundary instead of hand-written unsafe
  FFI. The binding crate denies unsafe code. Transitive FFI remains inside
  PyO3, not a claim of an unsafe-free dependency graph.
- Use maturin 1.x for mixed Python/Rust packaging and an ABI3 CPython 3.10+
  wheel. The package targets the existing Unix Core; no Windows support claim.
- Reuse existing serde/serde_json, fs2, and time dependencies for an internal
  typed request/result boundary, nonblocking operation lock, and the existing
  fixture execution window. JSON is an in-memory FFI encoding, not a subprocess
  protocol or a second policy model.
- Expose local callbacks and the exact GitHub repository-metadata fixture as
  distinct modes. Reject unsupported project-level governance in local mode.
- Preserve existing approval semantics: local approval also resumes execution;
  fixture approval grants and a subsequent run performs the read.
- Treat callbacks as trusted, synchronous, registered-unattested code. Return
  callback outputs ephemerally, never represent them as persisted WorkReports.

## Validation and acceptance

Exercise approval, denial, policy rejection, wrong approval identity, input
mutation before/after approval, missing handlers, Python callback exceptions
and invalid results, separate-process reopen, terminal replay, reentrancy,
CLI/native interoperability, native wheel installation, source distribution
build, Rust lint/doc checks, and existing fixture tests. Add Linux and macOS
Python CI coverage. Do not call the change accepted until exact-head required
CI and maintainer review pass.
