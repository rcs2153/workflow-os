# Errors And Diagnostics

Workflow OS separates structured errors from diagnostics.

## Errors

`WorkflowOsError` represents a failed operation. It includes:

- Stable kind.
- Stable code.
- Human-readable message.
- Optional diagnostics.

Errors are intended for CLI output, SDK surfaces, tests, and future editor integrations. Error messages must avoid leaking secrets or sensitive payloads.

Current error kinds include:

- Parse.
- Validation.
- Unsupported.
- Policy denied.
- Invalid state.
- Security.
- Internal.

## Diagnostics

`Diagnostic` represents a specific issue discovered while loading, parsing, or validating project files.

Diagnostics include:

- Severity.
- Stable code.
- Message.
- Optional source location.

Diagnostic severities are:

- Info.
- Warning.
- Error.

## Source Locations

`SourceLocation` supports:

- File path.
- One-based line.
- One-based column.
- JSON Pointer, YAML path, or similar document-local path.

This shape is intended to support high-quality CLI output now and editor integration later.

## Formatting

Diagnostics should format in a compact CLI-friendly form:

```text
workflow.yaml:12:7 $.steps[0]: error[workflow.missing_skill]: referenced skill does not exist
```

The formatted string is for humans. Tools should use structured fields.

## Redaction

Errors and diagnostics must not include raw secrets. When sensitive data must be associated with an error, use references, summaries, or redacted wrappers.
