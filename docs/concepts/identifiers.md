# Identifiers

Workflow OS core uses strongly typed identifiers. IDs must not be passed around as interchangeable strings inside the Rust core.

## Why Strong Types

Strong identifier types prevent accidental misuse such as passing a `SkillId` where a `WorkflowId` is required, or using an `EventId` as a `WorkflowRunId`.

The Rust core defines canonical identifier types for:

- `ProjectId`
- `WorkflowId`
- `WorkflowVersion`
- `WorkflowRunId`
- `SkillId`
- `SkillVersion`
- `SkillInvocationId`
- `SkillAttemptId`
- `EventId`
- `CorrelationId`
- `IdempotencyKey`
- `ActorId`
- `AdapterId`
- `IntegrationId`
- `PolicyId`
- `SchemaVersion`
- `SpecContentHash`

## String Identifiers

Human-authored identifiers such as project, workflow, skill, actor, adapter, integration, version, schema version, and idempotency keys are stored as validated strings.

Canonical string identifiers:

- Must not be empty.
- Must not exceed 128 bytes.
- May contain ASCII letters, digits, `.`, `_`, `-`, and `/`.
- Serialize as strings.
- Display as their raw identifier text.

These rules keep identifiers readable in specs, CLI output, diagnostics, and audit records.

## Generated Identifiers

Runtime-generated identifiers include workflow run IDs, skill invocation IDs, skill attempt IDs, event IDs, and correlation IDs.

Generated IDs:

- Are strongly typed.
- Serialize as strings.
- Display as readable prefixed text.
- Are suitable for logs, events, audit records, and correlation.

The v0 foundation intentionally avoids a UUID dependency for generated IDs. Generated IDs use a stable type-specific prefix plus timestamp and monotonic process counter data. Future implementations may replace the internal generation strategy only if serialized contracts, uniqueness expectations, and migration impact are documented.

## Spec Content Hashes

`SpecContentHash` is a deterministic SHA-256 digest over the exact canonical spec bytes supplied to the hasher.

The hash:

- Is lowercase hexadecimal text.
- Must be 64 characters.
- Must be recorded when a run is created.
- Must remain bound to the run even if local project files later change.

Future canonicalization rules for workflow specs must be documented before they are used for run creation.

## Compatibility

Identifier serialization is part of the public contract. Future SDKs and schema artifacts must preserve the same serialized representation unless a documented major-version change intentionally breaks compatibility.
