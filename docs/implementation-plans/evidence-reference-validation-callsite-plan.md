# EvidenceReference Validation Call-Site Attachment Plan

Status: first implementation target complete. Schema-version diagnostics with safe source/spec context attach `SpecFile` evidence. Broader loader or validator call-site attachment remains planning only.

## 1. Executive Summary

The core `Diagnostic` model can now carry validated `EvidenceReference` values. Existing validation behavior remains deterministic, local, and unchanged when no evidence is attached.

The next question is where loader and validator call sites should continue attaching evidence references. This plan recommended starting with low-risk diagnostics that already have stable `SourceLocation` values, especially schema-version diagnostics and other source-location-backed spec-file diagnostics. That first target is now implemented for schema-version diagnostics only.

Evidence at validation call sites should help future operator reviews and `WorkReportContract` work cite validation checks without copying raw spec contents, command output, parser payloads, provider data, or environment values.

## 2. Goals

- Attach useful evidence to selected diagnostics.
- Preserve existing diagnostic behavior.
- Preserve deterministic validation.
- Avoid raw spec contents.
- Avoid raw command output.
- Preserve source locations.
- Preserve diagnostic codes.
- Prepare for future `WorkReportContract` citations.
- Keep evidence attachment reference-first, redacted, and local.

## 3. Non-Goals

This plan does not authorize:

- changing validation semantics;
- changing pass/fail behavior;
- creating `ValidationReport` or `ValidationSummary`;
- adding persistence;
- adding CLI rendering;
- updating examples;
- adding work reports;
- adding reasoning lineage;
- adding schemas;
- adding writes;
- adding live provider behavior;
- calling adapters from validation;
- storing raw specs, parser payloads, command output, or provider payloads.

## 4. Candidate Call-Site Inventory

Current validation surfaces are split between project loading/parsing and semantic validation.

| Surface | Current behavior | First implementation decision | Rationale |
| --- | --- | --- | --- |
| Project loader diagnostics | `ProjectLoadResult` accumulates `Diagnostic` values for discovery, parse, schema, secret, and duplicate-ID issues. | Defer as a group. | Loader has multiple diagnostic families with different privacy profiles. Start with one low-risk family instead of adding evidence broadly. |
| YAML parse diagnostics | `yaml.parse` includes parser-provided line and column when available. | Defer. | Parser error text can include user-provided context. Evidence must not preserve parser payloads or excerpts by accident. |
| Schema version diagnostics | `schema_version.missing`, `schema_version.unsupported`, and `validation.schema_version.unsupported` use stable document paths such as `$.schema_version`. | Implemented for the first call-site phase. | These diagnostics already have stable source locations, stable codes, and do not require copying spec contents. |
| Project manifest validation | Semantic checks include project name and manifest schema rules. | Attach only schema-version evidence first; defer other manifest diagnostics. | Manifest source paths are useful, but non-schema project metadata should wait until source path policy is exercised on schema-version diagnostics. |
| Workflow spec validation | Semantic checks cover triggers, steps, lifecycle, audit, observability, timeout, autonomy, references, retry, escalation, approval, and safety. | Defer. | Many messages include workflow, step, policy, or skill IDs. Add evidence after the source-location helper is reviewed. |
| Skill spec validation | Semantic checks cover contract fields, failure modes, evaluation, adapter references, capabilities, and sensitive redaction rules. | Defer. | Sensitive-field diagnostics need a separate safety pass so evidence never copies field values or parser context. |
| Policy spec validation | Semantic checks cover schema version, rule presence, effects, and referenced behavior. | Attach only schema-version evidence first; defer behavior diagnostics. | Policy behavior diagnostics can cite IDs and effects. Schema-version evidence is lower risk. |
| Semantic validation diagnostics | `validate_project_bundle` emits `Diagnostic` values through the validator helper. | Defer broad attachment. | Start with one diagnostic family and preserve ordering/message behavior before expanding. |
| Duplicate ID diagnostics | Loader and semantic validation report duplicate IDs with source locations. | Defer. | Messages include IDs and sometimes first-declaration paths. Evidence should not increase path or identifier leakage. |
| Missing reference diagnostics | Validator reports unknown skill and policy references. | Defer. | Messages include identifiers. Useful later, but not the safest first target. |
| Unsupported autonomy-level diagnostics | Validator reports Level 3/4 unsafe declarations. | Defer. | Useful governance evidence later, but not necessary for the first call-site exercise. |
| Invalid retry/escalation/approval diagnostics | Validator reports policy effect, bounded retry, approval, escalation, and unsafe retry exhaustion issues. | Defer. | These are governance-critical, but they sit near future side-effect and approval modeling. |
| Sensitive/secret-in-spec diagnostics | Loader rejects secret-looking keys or values with `spec.secret_disallowed`. | Defer, then consider as a second target. | A reference-only evidence attachment may be useful, but the first implementation should avoid any chance of copying secret-like parser context. |
| Source-location diagnostics | Most validator diagnostics use file path and document path through `SourceLocation`. | Use as foundation for first implementation. | A helper can construct spec-file evidence from existing source locations without changing diagnostic semantics. |
| Docs/integration check outputs | Represented by scripts and docs, not core validation structures. | Reject for this core call-site phase. | Command output evidence belongs in separate release/review tooling or a later command-output plan. |

## 5. First Implementation Target Recommendation

The first implementation targeted source-location-backed spec-file evidence for schema-version diagnostics.

Implemented first call sites:

- loader-produced `schema_version.missing`;
- loader-produced `schema_version.unsupported`;
- semantic validator `validation.schema_version.unsupported` for project, workflow, skill, and policy specs.

Why this target is small:

- the diagnostics already have stable codes;
- the diagnostics already have stable source locations;
- document path is already `$.schema_version`;
- no raw spec content is needed;
- no command output is involved;
- no provider context is involved;
- validation pass/fail behavior does not need to change.

Avoid first:

- raw YAML parser errors;
- command-output diagnostics;
- diagnostics without source location;
- diagnostics that require a new aggregate validation model;
- diagnostics that require reading or copying file contents;
- secret-in-spec diagnostics until a second targeted safety review.

## 6. Evidence Construction Rules

Future call-site attachment should construct evidence references using these rules:

- `EvidenceKind` should generally be `SpecFile` for file/source-location pointers.
- `EvidenceKind::ValidationResult` should be used only when the target is a validation artifact, diagnostic reference, or stable validation result reference.
- `EvidenceScope` should generally be `Project` for spec-file evidence generated during local project validation.
- `EvidenceScope::Validation` should be used only when the evidence includes the validation reference required by the core model.
- `EvidenceScope::Workflow` may be used only when workflow context is already known and no context is invented for evidence.
- The evidence target should reference the file path or local source reference safely.
- `SourceLocation` remains the source of truth for line, column, and document path.
- `EvidenceReference.summary` must not copy `Diagnostic.message` by default.
- Evidence must not copy raw file contents.
- Sensitivity should default conservatively.
- Redaction metadata must be explicit.
- Call sites must attach evidence through `Diagnostic` attachment APIs so `EvidenceReference::validate()` and diagnostic-specific kind/scope checks run.

## 7. Source Path And Privacy Policy

File paths can be useful evidence targets, but they can also expose private repository layout, usernames, temporary directories, or customer/project names.

Future implementation should follow these rules:

- Prefer project-relative spec paths when available.
- Treat absolute paths as sensitive by default.
- Default path-backed evidence to at least internal or confidential sensitivity unless the path is explicitly known to be safe.
- Do not include source excerpts.
- Do not include raw YAML/JSON content.
- Do not expand environment variables.
- Do not copy shell command lines or terminal transcripts into path evidence.
- Omit or redact source paths when path exposure would be more sensitive than useful.
- Preserve line, column, and document path on `SourceLocation`; do not duplicate them into evidence summaries by default.
- Use bounded metadata only when a future citation needs stable diagnostic code, schema version, or spec hash context.

## 8. Diagnostic Behavior Preservation

Future call-site evidence attachment must preserve existing validation behavior:

- diagnostics without evidence behave exactly as before;
- adding evidence must not change diagnostic code;
- adding evidence must not change severity;
- adding evidence must not change message;
- adding evidence must not change source location;
- adding evidence must not change pass/fail decisions;
- adding evidence must not change diagnostic ordering;
- adding evidence must not require adapters, live credentials, remote systems, or secret reads.

Evidence attachment failure should be treated as an internal evidence-construction failure, not as a user project diagnostic. The conservative behavior for the first call-site implementation should be:

- never attach invalid evidence;
- never emit partially attached evidence;
- preserve the original diagnostic without evidence if optional evidence construction fails in a non-user-controlled way;
- add tests proving the selected call sites construct valid evidence;
- do not add a new user-facing validation diagnostic for evidence construction failures unless a later design explicitly introduces internal diagnostic reporting.

## 9. Error Handling

If evidence construction or attachment fails:

- errors must not include raw paths beyond the existing diagnostic source-location policy;
- errors must not include raw evidence titles, targets, summaries, metadata, source excerpts, parser payloads, or secret-like values;
- the original validation diagnostic should remain available without evidence;
- invalid evidence must not be stored;
- validation result ordering should remain stable;
- the failure should be covered by unit tests around helper behavior rather than surfaced as a project-spec error.

If future persistence or CLI rendering is added later, this error-handling posture must be revisited before evidence-bearing diagnostics are persisted or displayed as stable output.

## 10. Test Plan

Future implementation should include tests for:

- diagnostics with source location attach spec-file evidence;
- schema-version diagnostics attach spec-file evidence;
- diagnostics without source location remain unchanged;
- secret-in-spec diagnostics are not attached in the first implementation;
- diagnostic code remains unchanged;
- diagnostic severity remains unchanged;
- diagnostic message remains unchanged;
- source location remains unchanged;
- validation ordering remains unchanged;
- invalid evidence attachment fails safely;
- no raw spec content is copied;
- no command output is copied;
- diagnostic message is not copied into evidence summary by default;
- source paths follow sensitivity policy;
- absolute paths are treated conservatively;
- secret-like path, target, title, summary, and metadata values do not leak through debug output;
- existing validation tests continue to pass;
- docs checks pass.

## 11. Proposed Implementation Sequence

Completed first implementation sequence:

1. Add a helper for constructing source-location-backed spec-file evidence from `SourceLocation`. Completed.
2. Attach evidence to schema-version diagnostics only. Completed.
3. Add tests for behavior preservation, source path sensitivity, no raw spec copies, and stable ordering. Completed.
4. Run maintainer review. Next.
5. Expand to one additional low-risk diagnostic family only after review. Deferred.

The likely second target should be secret-in-spec diagnostics, but only if the implementation proves reference-only behavior and never copies secret-like values, parser payloads, source excerpts, or raw YAML.

## 12. Deferred Work

This plan explicitly defers:

- aggregate validation report evidence;
- validation success evidence;
- command-output evidence;
- CLI evidence display;
- persistence;
- example updates;
- work reports;
- reasoning lineage;
- approval evidence;
- write-side effects;
- schemas;
- live provider behavior;
- production evidence storage.

## 13. Final Recommendation

The first call-site implementation targeted source-location-backed spec-file evidence for schema-version diagnostics.

Do not start with secret-in-spec diagnostics, parser errors, command output, duplicate IDs, missing references, approval/retry/escalation diagnostics, or a new aggregate validation result model.

The implementation must not change validation semantics, persistence, CLI output, examples, reports, reasoning lineage, schemas, writes, or release posture.
