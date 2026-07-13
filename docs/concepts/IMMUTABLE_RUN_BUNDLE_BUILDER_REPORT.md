# Immutable Run Bundle Builder Report

Report date: 2026-07-13

## 1. Executive Summary

The pure in-memory immutable run-bundle builder is implemented. It consumes a
loaded `ProjectBundle`, rechecks deterministic project validation, selects one
workflow and only its resolved skills and referenced policies, constructs
canonical definition records, and returns those records with a matching
validated manifest.

The helper does not persist records, create or resume a run, append events,
execute handlers or checks, call providers, write artifacts, or change current
executor behavior.

## 2. Scope Completed

- Added `ImmutableRunBundleBuildRequest` with explicit run, bundle, context,
  execution, handler, actor, sensitivity, and redaction posture.
- Added `build_immutable_run_bundle` as a pure construction helper.
- Added `ImmutableRunBundleBuildResult` with read-only manifest/record accessors
  and `into_parts`.
- Revalidated the supplied project before construction.
- Resolved the selected workflow's skills using existing local version rules.
- Selected workflow-level and step-level policy references.
- Sourced every workflow, skill, and policy hash from loader-owned
  `LoadedSpec` records.
- Deduplicated canonical skill and policy records while retaining per-step
  skill references in the manifest.
- Reused existing definition-record and manifest constructors for
  canonicalization, hashing, validation, and redaction boundaries.

## 3. Scope Explicitly Not Completed

This phase did not add:

- definition-record or manifest stores;
- persistence, create-only writes, or atomic write sequencing;
- executor integration or automatic bundle generation;
- run identity, run creation, approval, resume, event, or snapshot changes;
- executable replay or historical inspection;
- handler or check attestation;
- scoped authority or capability enforcement;
- provider calls or writes;
- CLI, schemas, SDK changes, examples, hosted behavior, or release changes.

## 4. Builder API Summary

`ImmutableRunBundleBuildRequest<'a>` borrows one `ProjectBundle` and selected
workflow ID while owning the explicit manifest identity and posture values.
The request does not read hidden global state.

`build_immutable_run_bundle(request)` returns
`Result<ImmutableRunBundleBuildResult, WorkflowOsError>`. The result contains
one validated manifest and the unique canonical records required to satisfy its
definition references. It remains memory-only.

## 5. Selection And Resolution Summary

The builder first rejects any project with deterministic validation errors. It
then selects exactly one workflow, resolves every ordered step's skill and
version, and gathers referenced policy IDs from workflow retry/escalation
references plus step policy, approval, retry, and escalation references.

Unreferenced project skills, policies, tests, the project manifest, and source
paths are excluded. A repeated skill used by multiple steps produces one
canonical record and one manifest reference per step.

## 6. Integrity Boundary Summary

The builder does not accept caller-supplied source hashes. Workflow, skill, and
policy source hashes come from the corresponding `LoadedSpec`. Existing record
constructors canonicalize typed definitions and existing manifest construction
validates workflow identity, deterministic ordering, handler/skill alignment,
execution posture, and the root hash.

The resolved execution-context commitment remains an explicit caller input.
The builder does not recompute executor request posture or claim that it can
resume from the resulting bundle.

## 7. Privacy And Redaction Summary

The request and result use bounded manual Debug implementations. They do not
format project contents, local paths, workflow IDs, bundle IDs, run IDs,
definition contents, or hashes. Canonical records exclude raw YAML, source
comments, and source locations through the accepted record canonicalization
boundary.

Serialized definition records remain sensitive storage shapes rather than safe
operator output. No raw provider payloads, command output, environment values,
credentials, tokens, prompts, or transcripts are accepted or copied.

## 8. Test Coverage Summary

Focused tests prove:

- a valid project produces a matching manifest and deduplicated records;
- repeated step use retains per-step references;
- loader-owned hashes are used for every selected definition;
- unreferenced skills and policies are excluded;
- identical explicit inputs produce identical root hashes;
- missing workflows and invalid projects fail with stable non-leaking errors;
- handler/skill mismatches fail closed;
- Debug and serialized record shapes exclude paths, comments, and raw YAML;
- construction creates no runtime state or filesystem artifacts.

## 9. Validation Commands And Results

The phase requires:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `npm run check:docs`;
- `git diff --check`.

All required commands passed. The workspace test run included all eight focused
builder tests and the existing executor, report, policy, adapter, and runtime
regression suites.

## 10. Remaining Known Limitations

- A `ProjectBundle` has no validated-marker type, so the builder conservatively
  rechecks default project validation.
- The resolved execution-context commitment remains caller supplied.
- Definition references currently commit loader source hashes; canonical record
  hashes are available on records but are not yet store keys in a persisted
  relationship.
- Handler posture is declarative and explicitly unattested.
- No immutable store, runtime binding, inspection, or resume behavior exists.

## 11. Governed Phase Evidence

- Workflow: `dg/implement`.
- Run: `run-1783922692457860000-2`.
- Approval: `approval/run-1783922692457860000-2/implementation-approved`.
- Presentation: `presentation/e4bf4b3a0a7c6780`.
- Approval outcome: granted through the proof-enforced presentation path under
  delegated-maintainer authority after the full handoff was relayed.
- Out-of-kernel work: Codex inspected the accepted models, edited code and
  documentation, and ran validation. The kernel governed scope and approval;
  it did not edit files, execute checks, or mutate Git.
- Report posture: this document is the phase report. No WorkReport artifact was
  generated or persisted.

## 12. Recommended Next Phase

Perform a focused maintainer review of the in-memory builder. Review selection
completeness, cross-definition validation, deterministic record/reference
alignment, source-hash provenance, privacy, Debug behavior, and non-mutation.

Do not begin local stores, executor integration, workload inference, scoped
authority, provider writes, CLI, schemas, hosted behavior, or release changes
until the builder is accepted.
