# Current-Authority WorkReport Artifact Metadata Read Plan Report

## 1. Executive Summary

Planning is complete for the first concrete Core-owned consumer of the private
current-authority same-call boundary.

The selected consumer is an exact, metadata-only read of one immutable
`WorkReport` artifact from an explicit caller-supplied
`WorkReportArtifactStore`. The future helper must resolve fresh authority and
consume the exact required-context contract before the store is reachable.

No runtime implementation was added.

## 2. Scope Completed

- Inspected the private registered current-authority source and use boundary.
- Inspected required-context contract and execution-binding semantics.
- Inspected governed WorkReport target and bounded-metadata vocabulary.
- Inspected the existing exact WorkReport artifact store read contract.
- Selected one concrete read-only consumer.
- Defined exact target, access-level, source, store, output, privacy, failure,
  and test boundaries.
- Updated the roadmap with the planning result.

## 3. Scope Explicitly Not Completed

This phase did not add:

- implementation;
- public authority APIs;
- generic callbacks;
- WorkReport body access;
- executor integration;
- runtime defaults;
- persistence changes;
- local-check or skill execution;
- providers or OpenShell;
- sandbox execution;
- SideEffect execution or writes;
- events or audit appends;
- schemas, SDKs, CLI behavior, examples, dependencies, or release changes.

## 4. Selected Consumer

The future operation will read one exact `WorkReportArtifactRecord` only after
the immutable required-context contract declares the same WorkReport target as
required bounded metadata and fresh current authority resolves ready.

It will project only:

- report ID;
- run ID;
- terminal report status;
- sensitivity.

The contained WorkReport will not be returned.

## 5. Why It Was Selected

The existing store already supports exact immutable artifact lookup by run and
report identity. This gives the authority lane a real read boundary without
introducing process execution, network access, provider behavior, mutable
records, or arbitrary payload dereference.

It also connects authority work to the governed handoff layer while preserving
the distinction between metadata visibility and report-content access.

## 6. Validation Boundary Summary

The future helper must fail before source or store access unless the request is
bound to:

- one exact WorkReport target;
- required obligation;
- bounded-metadata access;
- matching immutable execution and contract identity.

After that shape check, the existing same-call resolver remains responsible for
fresh source selection, capability resolution, prerequisite posture,
sensitivity bounds, and required-context consumption.

## 7. Privacy And Security Summary

The plan rejects report-body exposure, generic dereference, raw redaction
metadata, raw errors, paths, payloads, logs, command output, environment
values, credentials, and tokens.

Blocked and source-failure paths must prove zero store reads. IDs remain
redacted in Debug and errors. Store errors must map to one stable bounded code.

## 8. Future Test Coverage

The plan requires focused tests for ready reads, exact field bounds,
reference-only and optional-only rejection, identity mismatch, unavailable
targets, expired/revoked grants, unresolved prerequisites, source failures,
not-found posture, store failure, corrupt data, sensitivity mismatch, repeated
fresh resolution, Debug non-leakage, private visibility, and absence of writes
or events.

## 9. Governed Planning Record

- workflow ID: `dg/d`
- run ID: `run-1785179257082667000-2`
- approval ID:
  `approval/run-1785179257082667000-2/planning-approved`
- approval-presentation ID: `presentation/ef8ad15e59185dcc`
- approval outcome: granted by delegated maintainer
- governed status: completed
- out-of-kernel work: repository reading, documentation edits, validation, git,
  and later PR operations remain external execution coordinated by the kernel

## 10. Validation Commands

- `npm run check:docs`: passed
- `git diff --check`: passed

## 11. Remaining Limitations

- No concrete metadata read exists yet.
- The current registered source is in-memory and private.
- No transactional source/store snapshot exists.
- No durable replay prevention exists.
- No public or serialized view exists.
- No executor, handler, provider, OpenShell, sandbox, or write integration
  exists.

## 12. Recommended Next Phase

The focused maintainer review is complete and accepted in
[Current-Authority WorkReport Artifact Metadata Read Plan Review](CURRENT_AUTHORITY_WORK_REPORT_ARTIFACT_METADATA_READ_PLAN_REVIEW.md).

Implement only the private exact-target WorkReport artifact metadata read. Keep
arbitrary context dereference and every execution or mutation integration
deferred.
