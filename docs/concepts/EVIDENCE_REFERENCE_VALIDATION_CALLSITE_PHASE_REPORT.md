# EvidenceReference Validation Call-Site Phase Report

## 1. Selected Call-Site Target

The selected target is schema-version diagnostics with existing source/spec context:

- loader-produced `schema_version.missing`;
- loader-produced `schema_version.unsupported`;
- semantic validator `validation.schema_version.unsupported`.

These diagnostics now attach `EvidenceKind::SpecFile` evidence when a `SourceLocation` is already present.

## 2. Why It Was Selected

Schema-version diagnostics were selected because they are the lowest-risk validation call-site family:

- they already have stable diagnostic codes;
- they already use stable source locations;
- their document path is normally `$.schema_version`;
- they do not require copying raw spec contents;
- they do not involve command output;
- they do not invoke adapters or providers;
- they do not require a new aggregate validation model.

## 3. Behavior Added

The implementation adds a small internal helper that constructs reference-first spec-file evidence from an existing `SourceLocation`.

For selected schema-version diagnostics:

- evidence is attached through the validated `Diagnostic` attachment API;
- evidence kind is `SpecFile`;
- evidence scope is `Project`;
- evidence source component is `Validator`;
- sensitivity is conservative;
- redaction metadata records that the target is reference-only;
- raw file contents are not read or copied;
- diagnostic code, severity, message, source location, and ordering are preserved.

## 4. Behavior Explicitly Not Added

This phase does not add:

- attachment to all diagnostics;
- attachment to diagnostics without source location;
- attachment to YAML parser payloads;
- attachment to command output;
- secret-in-spec evidence attachment;
- duplicate-ID evidence attachment;
- missing-reference evidence attachment;
- approval/retry/escalation diagnostic evidence attachment;
- `ValidationReport`;
- `ValidationSummary`;
- aggregate `ValidationResult` evidence;
- validation success evidence;
- persistence;
- CLI rendering;
- example updates;
- approval evidence;
- work reports;
- reasoning lineage;
- side-effect modeling;
- writes;
- schemas;
- release posture changes.

## 5. Privacy And Redaction Posture

The implementation remains reference-first:

- no raw spec contents are copied;
- no raw YAML parser output is copied;
- no command output is copied;
- no provider payloads are copied;
- no environment values are read;
- file-path targets are treated as confidential evidence;
- evidence summaries are not populated from diagnostic messages by default;
- `SourceLocation` remains the source of truth for file path, line, column, and document path.

If evidence construction or attachment fails, invalid evidence is not attached and the original diagnostic remains unchanged.

## 6. Tests Added

Tests were added for:

- schema-version diagnostics receiving `SpecFile` evidence;
- evidence kind, scope, sensitivity, redaction metadata, and file target behavior;
- diagnostic code, severity, message, and source location preservation;
- evidence summary remaining empty by default;
- diagnostics outside the selected family remaining evidence-free;
- YAML parse diagnostics remaining evidence-free;
- secret-in-spec diagnostics remaining evidence-free;
- semantic validation diagnostic ordering remaining stable;
- semantic diagnostics outside the selected family remaining evidence-free.

## 7. Validation Commands Run

Focused validation run during implementation:

- `cargo fmt --all --check` passed after formatting.
- `cargo test -p workflow-core --test project_loader --test project_validation` passed.

Full required validation should be recorded by the final implementation report.

## 8. Remaining Limitations

Remaining limitations:

- only schema-version diagnostics attach evidence;
- broad automatic loader/validator evidence generation is not implemented;
- aggregate `ValidationResult` evidence is not implemented;
- validation success evidence is not implemented;
- parser errors and command output remain out of scope;
- secret-in-spec diagnostics remain out of scope pending a targeted safety review;
- persistence and CLI rendering are not implemented;
- work reports and reasoning lineage are not implemented.

## 9. Recommended Next Phase

Recommended next phase: validation call-site phase review.

After review, the next implementation candidate should be either:

- a targeted secret-in-spec reference-only evidence plan; or
- approval evidence attachment planning if maintainers prefer moving toward future governed work reports.

Do not expand evidence attachment to all diagnostics without a maintainer review.
