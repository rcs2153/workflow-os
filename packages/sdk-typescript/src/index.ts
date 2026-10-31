export const schemaVersion = "workflowos.dev/v0";

export type LifecycleStatus = "experimental" | "stable" | "deprecated";
export type AutonomyLevel = "level_1" | "level_2" | "level_3" | "level_4";
export type TriggerKind = "manual" | "file" | "schedule" | "external_event";
export type FieldType = "string" | "boolean" | "number" | "object" | "array";
export type RedactionBehavior = "full" | "summary_only" | "reference_only";
export type ApprovalSensitivity = "low" | "medium" | "high";
export type RetryCompatibility = "compatible" | "not_compatible" | "requires_policy";
export type TerminalBehavior = "fail_workflow" | "escalate" | "continue";
export type CancellationBehavior = "stop" | "compensate";
export type GovernanceStrictnessProfile =
  | "observe_and_report"
  | "agent_assisted_gated"
  | "human_approval_gated"
  | "strict_enterprise";
export type ExplicitLocalCheckProfileId = "workflow_os_project_validation";
export type ReportArtifactHighAssuranceApprovalRequirement =
  | "not_required"
  | "disclosure_required"
  | "validated_disclosure_required"
  | "validated_fail_closed_disclosure_required";
export type ReportArtifactApprovalProofMarkerRequirement =
  | "not_required"
  | "projection_required"
  | "marker_required";

export interface OwnerMetadata {
  owning_team?: string;
  maintainer?: string;
  escalation_contact?: string;
  lifecycle_status: LifecycleStatus;
}

export interface ProjectManifestInput {
  project: {
    id: string;
    name: string;
    description?: string;
  };
  layout?: ProjectLayout;
  config?: ConfigOverlay[];
  governance?: ProjectGovernanceConfiguration;
}

export interface ProjectLayout {
  workflows?: string;
  skills?: string;
  policies?: string;
  tests?: string;
}

export interface ConfigOverlay {
  environment: string;
  vars: ConfigVar[];
}

export interface ConfigVar {
  name: string;
  value: string;
}

export interface ProjectGovernanceConfiguration {
  authoritative_execution?: AuthoritativeExecutionConfiguration;
  provider_reads?: ProviderReadConfiguration;
}

export interface AuthoritativeExecutionConfiguration {
  profile: "observe_and_report";
  local_check_profile: ExplicitLocalCheckProfileId;
}

export interface ProviderReadConfiguration {
  github_repository_metadata: GitHubRepositoryMetadataReadConfiguration;
}

export interface GitHubRepositoryMetadataReadConfiguration {
  enabled: true;
  endpoint: "github.com";
  access_reference: "env/WORKFLOW_OS_GITHUB_TOKEN";
  resources: GitHubRepositoryMetadataResource[];
}

export interface GitHubRepositoryMetadataResource {
  owner: string;
  repository: string;
}

export interface ProjectManifest {
  schema_version: typeof schemaVersion;
  project: ProjectManifestInput["project"];
  layout: Required<ProjectLayout>;
  config?: ConfigOverlay[];
  governance?: ProjectGovernanceConfiguration;
}

export interface ContractFieldInput {
  name: string;
  field_type: FieldType;
  description?: string;
  sensitive?: boolean;
  redaction?: RedactionBehavior;
}

export interface ContractField extends ContractFieldInput {
  sensitive?: boolean;
  redaction?: RedactionBehavior;
}

export interface DataContractInput {
  fields: ContractFieldInput[];
  required?: string[];
  examples?: ContractExample[];
}

export interface DataContract {
  fields: ContractField[];
  required: string[];
  examples?: ContractExample[];
}

export interface ContractExample {
  name: string;
  values: Record<string, string | number | boolean>;
}

export interface AuditRequirements {
  required: boolean;
  events: string[];
}

export interface ObservabilityRequirements {
  metrics: string[];
  tracing?: boolean;
  latency_tracking?: boolean;
}

export interface ReportArtifactRequirements {
  high_assurance_approval?: ReportArtifactHighAssuranceApprovalRequirement;
  approval_proof_markers?: ReportArtifactApprovalProofMarkerRequirement;
}

export interface SkillDefinitionInput {
  id: string;
  version: string;
  display_name: string;
  description?: string;
  owner: OwnerMetadata;
  input_contract: DataContractInput;
  output_contract: DataContractInput;
  allowed_capabilities?: CapabilityRequirement[];
  adapter_requirements?: AdapterRequirement[];
  failure_modes: FailureMode[];
  evaluation_criteria: EvaluationCriterion[];
  retry_compatibility?: RetryCompatibility;
  approval_sensitivity?: ApprovalSensitivity;
  audit_requirements: AuditRequirements;
  observability_requirements: ObservabilityRequirements;
  tags?: string[];
}

export interface SkillDefinition {
  schema_version: typeof schemaVersion;
  id: string;
  version: string;
  display_name: string;
  description?: string;
  owner: OwnerMetadata;
  input_contract: DataContract;
  output_contract: DataContract;
  allowed_capabilities?: CapabilityRequirement[];
  adapter_requirements?: AdapterRequirement[];
  failure_modes: FailureMode[];
  evaluation_criteria: EvaluationCriterion[];
  retry_compatibility?: RetryCompatibility;
  approval_sensitivity?: ApprovalSensitivity;
  audit_requirements: AuditRequirements;
  observability_requirements: ObservabilityRequirements;
  tags?: string[];
}

export interface CapabilityRequirement {
  name: string;
}

export interface AdapterRequirement {
  adapter_id: string;
  operation?: string;
  integration_id?: string;
  capabilities: string[];
}

export interface FailureMode {
  code: string;
  description: string;
  retryable?: boolean;
}

export interface EvaluationCriterion {
  name: string;
  description: string;
}

export interface WorkflowDefinitionInput {
  id: string;
  version: string;
  display_name: string;
  description?: string;
  owner: OwnerMetadata;
  autonomy_level: AutonomyLevel;
  triggers: TriggerDefinition[];
  steps: StepDefinition[];
  approval_requirements?: ApprovalRequirement[];
  cancellation_behavior: CancellationBehavior;
  audit_requirements: AuditRequirements;
  observability_requirements: ObservabilityRequirements;
  report_artifact_requirements?: ReportArtifactRequirements;
  timeout_policy?: TimeoutPolicy;
  disabled_by_default?: boolean;
  tags?: string[];
}

export interface WorkflowDefinition {
  schema_version: typeof schemaVersion;
  id: string;
  version: string;
  display_name: string;
  description?: string;
  owner: OwnerMetadata;
  autonomy_level: AutonomyLevel;
  triggers: TriggerDefinition[];
  steps: StepDefinition[];
  approval_requirements?: ApprovalRequirement[];
  cancellation_behavior: CancellationBehavior;
  audit_requirements: AuditRequirements;
  observability_requirements: ObservabilityRequirements;
  report_artifact_requirements?: ReportArtifactRequirements;
  timeout_policy?: TimeoutPolicy;
  disabled_by_default?: boolean;
  tags?: string[];
}

export interface TriggerDefinition {
  id: string;
  kind: TriggerKind;
}

export interface SkillReference {
  id: string;
  version?: string;
}

export interface StepDefinition {
  id: string;
  skill_ref: SkillReference;
  input_mapping?: ValueMapping[];
  output_mapping?: ValueMapping[];
  policy_requirements?: PolicyReference[];
  local_check_requirements?: LocalCheckRequirementDeclaration[];
  retry_policy?: PolicyReferenceWrapper;
  escalation_policy?: PolicyReferenceWrapper;
  approval_policy?: PolicyReferenceWrapper;
  timeout?: DurationSpec;
  terminal_behavior: TerminalBehavior;
}

export type LocalCheckRequirementLevel = "required" | "optional";

export type LocalCheckAttestationFreshnessPolicy =
  | { mode: "no_reuse" }
  | { mode: "max_age_seconds"; seconds: number };

export interface LocalCheckRequirementDeclaration {
  id: string;
  command_id: string;
  requirement_level: LocalCheckRequirementLevel;
  minimum_assurance: "kernel_observed_local_process";
  accepted_statuses: ["passed"];
  freshness: LocalCheckAttestationFreshnessPolicy;
  exact_immutable_run_binding_required: true;
  truncation_allowed: boolean;
  network_maximum: "disabled";
  side_effect_maximum: "no_source_writes" | "build_or_cache_writes";
}

export interface ValueMapping {
  from: MappingExpression;
  to: string;
}

export type MappingExpression =
  | { type: "field"; path: string }
  | { type: "literal"; value: string }
  | { type: "config_ref"; name: string };

export interface PolicyReference {
  id: string;
}

export interface PolicyReferenceWrapper {
  policy: PolicyReference;
}

export interface ApprovalRequirement {
  id: string;
  reason: string;
  expires_after?: DurationSpec;
}

export interface DurationSpec {
  duration: string;
}

export interface TimeoutPolicy {
  max_duration: DurationSpec;
  on_timeout: "fail" | "escalate" | "cancel";
}

export interface PolicyDefinitionInput {
  id: string;
  name: string;
  description?: string;
  rules: PolicyRule[];
}

export interface PolicyDefinition {
  schema_version: typeof schemaVersion;
  id: string;
  name: string;
  description?: string;
  rules: PolicyRule[];
}

export interface PolicyRule {
  id: string;
  effect: string;
}

export interface ProjectFilesInput {
  manifest: ProjectManifest;
  workflows: WorkflowDefinition[];
  skills: SkillDefinition[];
  policies: PolicyDefinition[];
}

export type ProjectFiles = Record<string, string>;

export function projectManifest(input: ProjectManifestInput): ProjectManifest {
  checkNoSecretsInText(input.project.id);
  checkNoSecretsInText(input.project.name);
  for (const overlay of input.config ?? []) {
    for (const variable of overlay.vars) {
      checkNoSecretsInText(variable.name);
      checkNoSecretsInText(variable.value);
    }
  }
  const providerReads = input.governance?.provider_reads;
  if (providerReads && !input.governance?.authoritative_execution) {
    throw new Error("provider reads require authoritative execution");
  }
  if (providerReads) {
    const declaration = providerReads.github_repository_metadata;
    if (
      declaration.enabled !== true ||
      declaration.endpoint !== "github.com" ||
      declaration.access_reference !== "env/WORKFLOW_OS_GITHUB_TOKEN" ||
      declaration.resources.length === 0 ||
      declaration.resources.length > 32
    ) {
      throw new Error("GitHub repository metadata declaration is invalid");
    }
    const resources = new Set<string>();
    for (const resource of declaration.resources) {
      const ownerValid = /^[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/.test(resource.owner)
        && resource.owner.length <= 39;
      const repositoryValid = /^[a-z0-9](?:[a-z0-9._-]*[a-z0-9])?$/.test(resource.repository)
        && resource.repository.length <= 100;
      const key = `${resource.owner}/${resource.repository}`;
      if (!ownerValid || !repositoryValid || resources.has(key)) {
        throw new Error("GitHub repository metadata resource is invalid");
      }
      resources.add(key);
    }
  }
  return {
    schema_version: schemaVersion,
    project: input.project,
    layout: {
      workflows: input.layout?.workflows ?? "workflows",
      skills: input.layout?.skills ?? "skills",
      policies: input.layout?.policies ?? "policies",
      tests: input.layout?.tests ?? "tests"
    },
    ...(input.config ? { config: input.config } : {}),
    ...(input.governance ? { governance: input.governance } : {})
  };
}

export function skillDefinition(input: SkillDefinitionInput): SkillDefinition {
  validateExactAdapterOperations(input);
  return {
    schema_version: schemaVersion,
    ...input,
    input_contract: dataContract(input.input_contract),
    output_contract: dataContract(input.output_contract)
  };
}

function validateExactAdapterOperations(input: SkillDefinitionInput): void {
  for (const adapter of input.adapter_requirements ?? []) {
    if (adapter.operation === undefined) {
      continue;
    }
    if (adapter.operation !== "github.repository.metadata") {
      throw new Error("adapter operation is not supported by the v0 contract");
    }
    const allowed = new Set((input.allowed_capabilities ?? []).map(capability => capability.name));
    const required = new Set(input.input_contract.required ?? []);
    const fields = new Map(input.input_contract.fields.map(field => [field.name, field]));
    const owner = fields.get("owner");
    const repo = fields.get("repo");
    const exact = (input.adapter_requirements?.length ?? 0) === 1
      && adapter.adapter_id === "symbolic/github-read-only"
      && adapter.integration_id === undefined
      && adapter.capabilities.length === 1
      && adapter.capabilities[0] === "external.read"
      && allowed.size === 2
      && allowed.has("external.read")
      && allowed.has("adapter.invoke")
      && (input.allowed_capabilities?.length ?? 0) === 2
      && fields.size === 2
      && owner?.field_type === "string"
      && owner.sensitive !== true
      && repo?.field_type === "string"
      && repo.sensitive !== true
      && required.size === 2
      && required.has("owner")
      && required.has("repo")
      && (input.input_contract.required?.length ?? 0) === 2
      && (input.input_contract.examples?.length ?? 0) === 0;
    if (!exact) {
      throw new Error("adapter operation does not match its exact v0 skill contract");
    }
  }
}

export function workflowDefinition(input: WorkflowDefinitionInput): WorkflowDefinition {
  for (const step of input.steps) {
    for (const mapping of step.input_mapping ?? []) {
      if (mapping.from.type === "literal") {
        checkNoSecretsInText(mapping.from.value);
      }
    }
  }
  return {
    schema_version: schemaVersion,
    ...input
  };
}

export function policyDefinition(input: PolicyDefinitionInput): PolicyDefinition {
  for (const rule of input.rules) {
    checkNoSecretsInText(rule.effect);
  }
  return {
    schema_version: schemaVersion,
    ...input
  };
}

export function literal(value: string): MappingExpression {
  checkNoSecretsInText(value);
  return { type: "literal", value };
}

export function field(path: string): MappingExpression {
  return { type: "field", path };
}

export function configRef(name: string): MappingExpression {
  checkNoSecretsInText(name);
  return { type: "config_ref", name };
}

export function emitJsonSpec(spec: unknown): string {
  return `${JSON.stringify(spec, null, 2)}\n`;
}

export function projectFiles(input: ProjectFilesInput): ProjectFiles {
  const files: ProjectFiles = {
    "workflow-os.yml": emitJsonSpec(input.manifest)
  };
  for (const workflow of input.workflows) {
    files[`workflows/${fileStem(workflow.id)}.workflow.yml`] = emitJsonSpec(workflow);
  }
  for (const skill of input.skills) {
    files[`skills/${fileStem(skill.id)}.skill.yml`] = emitJsonSpec(skill);
  }
  for (const policy of input.policies) {
    files[`policies/${fileStem(policy.id)}.policy.yml`] = emitJsonSpec(policy);
  }
  return files;
}

function dataContract(input: DataContractInput): DataContract {
  for (const field of input.fields) {
    checkNoSecretsInText(field.name);
    if (field.sensitive === true && field.redaction === undefined) {
      throw new Error(`sensitive field ${field.name} must declare redaction`);
    }
  }
  for (const example of input.examples ?? []) {
    for (const value of Object.values(example.values)) {
      if (typeof value === "string") {
        checkNoSecretsInText(value);
      }
    }
  }
  return {
    fields: input.fields,
    required: input.required ?? [],
    ...(input.examples ? { examples: input.examples } : {})
  };
}

function fileStem(id: string): string {
  return id.replaceAll("/", "-").replaceAll(".", "-").replaceAll("_", "-");
}

function checkNoSecretsInText(value: string): void {
  const lower = value.toLowerCase();
  if (
    lower.includes("secret") ||
    lower.includes("token") ||
    lower.includes("password") ||
    lower.includes("credential") ||
    lower.includes("api_key")
  ) {
    throw new Error("spec helpers reject secret-like values");
  }
}
