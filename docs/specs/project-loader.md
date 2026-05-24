# Project Loader

The Workflow OS project loader turns a local project directory into a structured `ProjectBundle`.

The loader is a discovery and parsing layer only. It never executes workflows, invokes skills, evaluates policy, calls adapters, reads secrets, or performs external side effects.

## Inputs

The loader starts from a project root directory and locates:

```text
workflow-os.yml
workflows/*.workflow.yml
skills/*.skill.yml
policies/*.policy.yml
tests/*.test.yml
```

Directory names may be overridden by `workflow-os.yml`.

## Loading Order

The v0 loader:

1. Locates `workflow-os.yml`.
2. Reads and parses the project manifest.
3. Uses the manifest layout to discover workflow, skill, policy, and test files.
4. Sorts discovered file paths for deterministic loading.
5. Parses every discovered file.
6. Computes canonical content hashes for successfully parsed files.
7. Builds a `ProjectBundle` with raw loaded definitions.
8. Accumulates diagnostics for discovery, parsing, schema-version, secret, and duplicate-ID errors.

Malformed discovered files are not silently ignored.

## Bundle Shape

`ProjectBundle` contains:

- Project root path.
- Loaded project manifest.
- Loaded workflow definitions.
- Loaded skill definitions.
- Loaded policy specs.
- Loaded test specs.

Each loaded spec includes:

- File path.
- Canonical content hash.
- Parsed definition.

Workflow and skill definitions preserve file-level source locations where practical.

## Diagnostics

The loader returns `ProjectLoadResult`, which contains:

- `bundle`: the loaded bundle when the manifest can be parsed.
- `diagnostics`: accumulated diagnostics.

`bundle` is absent only when `workflow-os.yml` is missing or cannot be parsed. If later files fail to parse, the loader still returns the bundle with successfully loaded definitions and diagnostics for failed files.

Diagnostics include severity, stable code, message, and source location where practical. Invalid YAML diagnostics include line and column when the YAML parser provides them. Schema-version diagnostics include `$.schema_version` as the document path.

Missing spec directories are warnings. Malformed files, duplicate declared IDs, unsupported schema versions, invalid YAML, and forbidden secrets are errors.

## Duplicate IDs

Duplicate IDs are reported after parsing successful files. v0 reports duplicate workflow, skill, policy, and test IDs as loader-level diagnostics.

Duplicate detection is not semantic validation. It only compares declared IDs within each spec kind.

## Secret Handling

The loader uses the same parser-level secret rejection as direct YAML parsing. Secrets must not be stored in specs. Secret-like keys or inline values produce diagnostics and prevent that file from being loaded.

The loader does not read environment variables or secret providers.

## Determinism

Discovery is deterministic because file paths are sorted before parsing. The same project content should produce the same bundle ordering and content hashes.

## Non-Goals

The v0 project loader does not:

- Validate cross-file references.
- Validate policy semantics.
- Execute workflows.
- Invoke skills.
- Open adapters.
- Resolve remote packages.
- Read secrets.
- Implement CLI behavior.
