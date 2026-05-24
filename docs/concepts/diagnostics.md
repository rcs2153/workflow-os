# Diagnostics

Diagnostics are structured findings emitted while Workflow OS loads, parses, or later validates project files.

Diagnostics are separate from process exit behavior and CLI rendering. A loader may accumulate several diagnostics and still return a partial project bundle when doing so is safe.

## Fields

Each diagnostic contains:

- Severity.
- Stable code.
- Human-readable message.
- Optional source location.

Source locations can include:

- File path.
- One-based line.
- One-based column.
- Document path such as `$.schema_version` or `$.id`.

## Severity

`error` means the affected file or operation cannot safely proceed.

`warning` means the project can still be loaded, but the user should review the condition.

`info` is reserved for future non-blocking loader and validator messages.

## Loader Diagnostics

The project loader emits diagnostics for:

- Missing `workflow-os.yml`.
- File read or discovery failures.
- Invalid YAML.
- Missing or unsupported schema version.
- Parse failures.
- Forbidden secret-like fields or values.
- Missing spec directories.
- Duplicate declared IDs.

Missing spec directories are warnings. Malformed discovered files are errors.

## Validation Diagnostics

The semantic validator emits diagnostics for deterministic project correctness and safety checks. Validation diagnostics use stable `validation.*` codes and point at the relevant file and document path where practical.

Validation errors mean the loaded project is not valid for future execution. Validation warnings identify declarations that are allowed but should be reviewed, such as experimental or deprecated definitions.

## CLI-Friendly Formatting

Diagnostics format compactly for future CLI output:

```text
workflows/example.workflow.yml:12:7 $.steps[0]: error[spec.parse]: failed to parse spec document
```

The formatted string is for humans. Tools should consume the structured fields directly.

## Redaction

Diagnostics must not leak secrets. If a diagnostic involves sensitive data, it should point to a file or document path and use a redacted message rather than including the raw value.
