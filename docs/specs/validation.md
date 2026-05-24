# Semantic Validation

The Workflow OS validator operates on a loaded `ProjectBundle`. It never executes workflows, invokes skills, evaluates live policy decisions, calls adapters, or reads secrets.

The validator is deterministic: diagnostics are derived from loaded project content, sorted discovery order, and canonical Rust models.

## Entry Points

Rust exposes:

- `validate_project_bundle(&ProjectBundle) -> ValidationResult`
- `validate_loaded_project(&ProjectLoadResult) -> ValidationResult`

`validate_loaded_project` preserves loader diagnostics and then validates the bundle when one exists. This is the intended foundation for a future `workflow-os validate` command.

## Diagnostic Contract

Validation diagnostics include severity, stable code, message, file path, and document path where practical.

Validation errors block a project from being considered valid. Validation warnings indicate risky or transitional declarations such as experimental or deprecated definitions.

## Project Rules

The validator checks:

- Project metadata is present and non-empty.
- Schema versions are supported.
- IDs are unique within workflow, skill, and policy namespaces.
- References resolve deterministically.
- Experimental and deprecated lifecycle statuses produce warnings.

## Workflow Rules

The validator checks:

- Workflow has at least one trigger.
- Workflow has at least one step.
- Step IDs are unique within a workflow.
- Referenced skills exist.
- Referenced skill versions exist, or an omitted version resolves only when exactly one version exists.
- Branch targets point to known steps.
- Each step declares terminal behavior.
- The final step cannot silently `continue`.
- Approval-sensitive steps declare approval policy.
- Retry policies are bounded.
- Retry exhaustion leads to escalation or terminal failure.
- Timeout behavior exists for external-event workflows and adapter-backed steps.
- Cancellation behavior is explicit.
- Level 1 workflows do not declare adapter-backed side-effecting steps.
- Level 3/4 workflows fail validation unless they are experimental and disabled by default.
- Runtime workflows declare audit and observability requirements.

## Skill Rules

The validator checks:

- Input and output contracts declare fields.
- Required fields exist in the contract field list.
- Sensitive fields declare redaction.
- Sensitive output fields use full redaction or reference-only behavior.
- Failure modes are declared.
- Adapter-backed skills declare evaluation criteria.
- Adapter capabilities are declared in `allowed_capabilities`.
- Adapter IDs are symbolic v0 references. Real adapter implementations are not validated here.

## Policy Rules

The validator checks:

- Referenced policies exist.
- Unknown policies fail validation.
- Approval policies declare approval behavior.
- Escalation policies declare escalation behavior.
- Retry policies declare bounded retry behavior and do not declare unbounded retry.
- Policy rules have non-empty effects.

## Safety Rules

Adapter-backed steps are treated as external side-effecting declarations for validation. They must have:

- Declared capabilities on the referenced skill.
- Policy requirements on the step.
- Idempotency strategy from the model.
- Timeout behavior on the step or workflow.

Secrets in specs are rejected by the loader and preserved by `validate_loaded_project` diagnostics.

## Non-Goals

The validator does not:

- Execute workflows.
- Validate runtime event application.
- Evaluate policy decisions against real actors.
- Call adapters.
- Resolve remote packages.
- Read secrets.
- Implement CLI behavior.
