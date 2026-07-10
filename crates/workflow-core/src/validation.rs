use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::work_report::WorkReportArtifactApprovalProofMarkerRequirement;
use crate::{
    with_spec_file_evidence_from_source_location, ApprovalSensitivity, AutonomyLevel, Diagnostic,
    DiagnosticSeverity, IdempotencyKeyStrategy, LifecycleStatus, LoadedSpec, PolicyEffect,
    PolicyEffectSet, PolicyReference, PolicySpecDocument, ProjectBundle, ProjectLoadResult,
    RedactionBehavior, SkillDefinition, SourceLocation, StepDefinition, TerminalBehavior,
    WorkReportArtifactHighAssuranceRequirement, WorkflowDefinition, WorkflowId,
    SUPPORTED_SCHEMA_VERSION,
};

/// Result of deterministic project validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationResult {
    /// Structured validation diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

/// Validation capability posture for callers that can prove scoped runtime enforcement.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ProjectValidationCapability {
    /// Default validation posture used by CLI and normal executor paths.
    #[default]
    Default,
    /// Validation posture for one explicitly selected report-artifact-capable workflow.
    ReportArtifactCapable {
        /// Workflow whose report artifact requirements are enforced by the caller.
        workflow_id: WorkflowId,
    },
}

impl ValidationResult {
    /// Returns true when any diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() == DiagnosticSeverity::Error)
    }
}

/// Validates a loaded-project result, preserving loader diagnostics.
#[must_use]
pub fn validate_loaded_project(load_result: &ProjectLoadResult) -> ValidationResult {
    validate_loaded_project_with_capability(load_result, ProjectValidationCapability::Default)
}

/// Validates a loaded-project result with an explicit runtime capability posture.
#[must_use]
pub fn validate_loaded_project_with_capability(
    load_result: &ProjectLoadResult,
    capability: ProjectValidationCapability,
) -> ValidationResult {
    let mut diagnostics = load_result.diagnostics.clone();
    if let Some(bundle) = &load_result.bundle {
        diagnostics.extend(validate_project_bundle_with_capability(bundle, capability).diagnostics);
    }
    ValidationResult { diagnostics }
}

/// Validates a loaded project bundle without executing workflows.
#[must_use]
pub fn validate_project_bundle(bundle: &ProjectBundle) -> ValidationResult {
    validate_project_bundle_with_capability(bundle, ProjectValidationCapability::Default)
}

/// Validates a loaded project bundle with an explicit runtime capability posture.
#[must_use]
pub fn validate_project_bundle_with_capability(
    bundle: &ProjectBundle,
    capability: ProjectValidationCapability,
) -> ValidationResult {
    let mut validator = Validator::new(bundle, capability);
    validator.validate();
    ValidationResult {
        diagnostics: validator.diagnostics,
    }
}

struct Validator<'a> {
    bundle: &'a ProjectBundle,
    diagnostics: Vec<Diagnostic>,
    skills: BTreeMap<(&'a str, &'a str), &'a LoadedSpec<SkillDefinition>>,
    skill_versions: BTreeMap<&'a str, BTreeSet<&'a str>>,
    policies: BTreeMap<&'a str, &'a LoadedSpec<PolicySpecDocument>>,
    capability: ProjectValidationCapability,
}

impl<'a> Validator<'a> {
    fn new(bundle: &'a ProjectBundle, capability: ProjectValidationCapability) -> Self {
        let mut skills = BTreeMap::new();
        let mut skill_versions: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for skill in &bundle.skills {
            let id = skill.definition.id.as_str();
            let version = skill.definition.version.as_str();
            skills.insert((id, version), skill);
            skill_versions.entry(id).or_default().insert(version);
        }

        let mut policies = BTreeMap::new();
        for policy in &bundle.policies {
            policies.insert(policy.definition.id.as_str(), policy);
        }

        Self {
            bundle,
            diagnostics: Vec::new(),
            skills,
            skill_versions,
            policies,
            capability,
        }
    }

    fn validate(&mut self) {
        self.validate_project();
        self.validate_unique_ids();
        self.validate_policies();
        for skill in &self.bundle.skills {
            self.validate_skill(skill);
        }
        for workflow in &self.bundle.workflows {
            self.validate_workflow(workflow);
        }
    }

    fn validate_project(&mut self) {
        let manifest = &self.bundle.manifest.definition;
        if manifest.schema_version.as_str() != SUPPORTED_SCHEMA_VERSION {
            self.error(
                "validation.schema_version.unsupported",
                "project manifest schema_version is unsupported",
                &self.bundle.manifest.path,
                "$.schema_version",
            );
        }
        if manifest.project.name.trim().is_empty() {
            self.error(
                "validation.project.name_missing",
                "project name must not be empty",
                &self.bundle.manifest.path,
                "$.project.name",
            );
        }
    }

    fn validate_unique_ids(&mut self) {
        report_duplicates(
            &self.bundle.workflows,
            |workflow| workflow.definition.id.as_str(),
            "validation.workflow.duplicate_id",
            "workflow id is duplicated",
            &mut self.diagnostics,
        );
        report_duplicates(
            &self.bundle.skills,
            |skill| skill.definition.id.as_str(),
            "validation.skill.duplicate_id",
            "skill id is duplicated",
            &mut self.diagnostics,
        );
        report_duplicates(
            &self.bundle.policies,
            |policy| policy.definition.id.as_str(),
            "validation.policy.duplicate_id",
            "policy id is duplicated",
            &mut self.diagnostics,
        );
    }

    fn validate_policies(&mut self) {
        for policy in &self.bundle.policies {
            if policy.definition.schema_version.as_str() != SUPPORTED_SCHEMA_VERSION {
                self.error(
                    "validation.schema_version.unsupported",
                    "policy schema_version is unsupported",
                    &policy.path,
                    "$.schema_version",
                );
            }
            if policy.definition.rules.is_empty() {
                self.error(
                    "validation.policy.rules_missing",
                    "policy must declare at least one rule",
                    &policy.path,
                    "$.rules",
                );
            }
            for rule in &policy.definition.rules {
                if rule.effect.trim().is_empty() {
                    self.error(
                        "validation.policy.effect_missing",
                        "policy rule effect must not be empty",
                        &policy.path,
                        "$.rules",
                    );
                    continue;
                }
                if let Err(error) = PolicyEffect::parse(&rule.effect) {
                    self.error(
                        error.code(),
                        "policy rule effect is not supported by the v0 runtime",
                        &policy.path,
                        "$.rules",
                    );
                }
                if rule.actor.is_some() {
                    self.error(
                        "validation.policy.actor_unsupported",
                        "policy rule actor binding is not enforced by the v0 runtime",
                        &policy.path,
                        "$.rules",
                    );
                }
            }
        }
    }

    fn validate_skill(&mut self, skill: &LoadedSpec<SkillDefinition>) {
        let definition = &skill.definition;
        if definition.schema_version.as_str() != SUPPORTED_SCHEMA_VERSION {
            self.error(
                "validation.schema_version.unsupported",
                "skill schema_version is unsupported",
                &skill.path,
                "$.schema_version",
            );
        }
        warn_lifecycle(
            &mut self.diagnostics,
            definition.owner.lifecycle_status,
            &skill.path,
        );
        self.validate_contract(skill, "input_contract", false);
        self.validate_contract(skill, "output_contract", true);
        if definition.failure_modes.is_empty() {
            self.error(
                "validation.skill.failure_modes_missing",
                "skill must declare failure modes",
                &skill.path,
                "$.failure_modes",
            );
        }
        if !definition.adapter_requirements.is_empty() && definition.evaluation_criteria.is_empty()
        {
            self.error(
                "validation.skill.evaluation_missing",
                "adapter-backed skills must declare evaluation criteria",
                &skill.path,
                "$.evaluation_criteria",
            );
        }

        let allowed: BTreeSet<&str> = definition
            .allowed_capabilities
            .iter()
            .map(|capability| capability.name.as_str())
            .collect();
        for adapter in &definition.adapter_requirements {
            let adapter_id = adapter.adapter_id.as_str();
            if !(adapter_id.starts_with("local/") || adapter_id.starts_with("symbolic/")) {
                self.error(
                    "validation.adapter.unknown",
                    "adapter reference is not a known v0 symbolic adapter",
                    &skill.path,
                    "$.adapter_requirements",
                );
            }
            for capability in &adapter.capabilities {
                if !allowed.contains(capability.as_str()) {
                    self.error(
                        "validation.skill.undeclared_capability",
                        format!("adapter capability {capability} is not declared in allowed_capabilities"),
                        &skill.path,
                        "$.adapter_requirements",
                    );
                }
            }
        }
    }

    fn validate_contract(
        &mut self,
        skill: &LoadedSpec<SkillDefinition>,
        contract_name: &'static str,
        is_output: bool,
    ) {
        let contract = if is_output {
            &skill.definition.output_contract
        } else {
            &skill.definition.input_contract
        };

        let fields: BTreeSet<&str> = contract
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect();
        if fields.is_empty() {
            self.error(
                "validation.skill.contract_fields_missing",
                format!("{contract_name} must declare fields"),
                &skill.path,
                format!("$.{contract_name}.fields"),
            );
        }
        for required in &contract.required {
            if !fields.contains(required.as_str()) {
                self.error(
                    "validation.skill.required_field_unknown",
                    format!("required field {required} is not declared"),
                    &skill.path,
                    format!("$.{contract_name}.required"),
                );
            }
        }
        for field in &contract.fields {
            if field.sensitive && field.redaction.is_none() {
                self.error(
                    "validation.skill.sensitive_redaction_missing",
                    format!("sensitive field {} must declare redaction", field.name),
                    &skill.path,
                    format!("$.{contract_name}.fields"),
                );
            }
            if is_output
                && field.sensitive
                && matches!(field.redaction, Some(RedactionBehavior::SummaryOnly))
            {
                self.error(
                    "validation.skill.sensitive_output_redaction",
                    format!(
                        "sensitive output field {} must be full redaction or reference-only",
                        field.name
                    ),
                    &skill.path,
                    format!("$.{contract_name}.fields"),
                );
            }
        }
    }

    fn validate_workflow(&mut self, workflow: &LoadedSpec<WorkflowDefinition>) {
        let definition = &workflow.definition;
        if definition.schema_version.as_str() != SUPPORTED_SCHEMA_VERSION {
            self.error(
                "validation.schema_version.unsupported",
                "workflow schema_version is unsupported",
                &workflow.path,
                "$.schema_version",
            );
        }
        warn_lifecycle(
            &mut self.diagnostics,
            definition.owner.lifecycle_status,
            &workflow.path,
        );
        if definition.triggers.is_empty() {
            self.error(
                "validation.workflow.triggers_missing",
                "workflow must declare at least one trigger",
                &workflow.path,
                "$.triggers",
            );
        }
        if definition.steps.is_empty() {
            self.error(
                "validation.workflow.steps_missing",
                "workflow must declare at least one step",
                &workflow.path,
                "$.steps",
            );
        }
        if definition.cancellation_behavior.is_none() {
            self.error(
                "validation.workflow.cancellation_missing",
                "workflow cancellation_behavior must be explicit",
                &workflow.path,
                "$.cancellation_behavior",
            );
        }
        if !definition.audit_requirements.required
            || definition.audit_requirements.events.is_empty()
        {
            self.error(
                "validation.workflow.audit_missing",
                "workflow must declare audit requirements for runtime behavior",
                &workflow.path,
                "$.audit_requirements",
            );
        }
        if definition.observability_requirements.metrics.is_empty()
            && !definition.observability_requirements.tracing
            && !definition.observability_requirements.latency_tracking
        {
            self.error(
                "validation.workflow.observability_missing",
                "workflow must declare observability requirements",
                &workflow.path,
                "$.observability_requirements",
            );
        }
        self.validate_report_artifact_requirements(workflow);
        if matches!(
            definition.autonomy_level,
            AutonomyLevel::Level3ConditionalAutonomy | AutonomyLevel::Level4ScaledAutomation
        ) && !(definition.owner.lifecycle_status == LifecycleStatus::Experimental
            && definition.disabled_by_default)
        {
            self.error(
                "validation.workflow.autonomy_level_unsafe",
                "Level 3/4 workflows must be experimental and disabled by default in v0",
                &workflow.path,
                "$.autonomy_level",
            );
        }

        self.validate_steps(workflow);
        self.validate_branches(workflow);
        if definition
            .triggers
            .iter()
            .any(|trigger| matches!(trigger.kind, crate::TriggerKind::ExternalEvent))
            && definition.timeout_policy.is_none()
        {
            self.error(
                "validation.workflow.timeout_missing",
                "external-event workflows must declare timeout_policy",
                &workflow.path,
                "$.timeout_policy",
            );
        }
    }

    fn validate_report_artifact_requirements(&mut self, workflow: &LoadedSpec<WorkflowDefinition>) {
        if matches!(
            &self.capability,
            ProjectValidationCapability::ReportArtifactCapable { workflow_id }
                if workflow_id == &workflow.definition.id
        ) {
            return;
        }
        if matches!(
            workflow
                .definition
                .report_artifact_requirements
                .high_assurance_approval,
            WorkReportArtifactHighAssuranceRequirement::DisclosureRequired
                | WorkReportArtifactHighAssuranceRequirement::ValidatedDisclosureRequired
                | WorkReportArtifactHighAssuranceRequirement::ValidatedFailClosedDisclosureRequired
        ) {
            self.error(
                "validation.workflow.report_artifact_requirement.runtime_not_enforced",
                "workflow-declared report artifact high-assurance requirements are not yet enforced by runtime artifact paths",
                &workflow.path,
                "$.report_artifact_requirements.high_assurance_approval",
            );
        }
        if matches!(
            workflow
                .definition
                .report_artifact_requirements
                .approval_proof_markers,
            WorkReportArtifactApprovalProofMarkerRequirement::ProjectionRequired
                | WorkReportArtifactApprovalProofMarkerRequirement::MarkerRequired
        ) {
            self.error(
                "validation.workflow.report_artifact_requirement.approval_proof_marker.runtime_not_enforced",
                "workflow-declared approval proof-marker artifact requirements are not yet enforced by runtime artifact paths",
                &workflow.path,
                "$.report_artifact_requirements.approval_proof_markers",
            );
        }
    }

    fn validate_steps(&mut self, workflow: &LoadedSpec<WorkflowDefinition>) {
        let mut step_ids = BTreeSet::new();
        let steps = &workflow.definition.steps;
        for step in steps {
            if !step_ids.insert(step.id.as_str()) {
                self.error(
                    "validation.workflow.duplicate_step_id",
                    format!("step id {} is duplicated within workflow", step.id),
                    &workflow.path,
                    "$.steps",
                );
            }
            self.validate_step(workflow, step);
        }

        if let Some(last_step) = steps.last() {
            if matches!(
                last_step.terminal_behavior,
                Some(TerminalBehavior::Continue)
            ) {
                self.error(
                    "validation.workflow.silent_dead_end",
                    "last step cannot continue without a following step",
                    &workflow.path,
                    "$.steps",
                );
            }
        }
    }

    fn validate_step(&mut self, workflow: &LoadedSpec<WorkflowDefinition>, step: &StepDefinition) {
        let Some(skill) = self.resolve_skill(workflow, step) else {
            return;
        };

        if step.terminal_behavior.is_none() {
            self.error(
                "validation.workflow.terminal_behavior_missing",
                format!("step {} must declare terminal_behavior", step.id),
                &workflow.path,
                "$.steps",
            );
        }

        self.validate_step_approval(workflow, step, skill);
        self.validate_step_retry(workflow, step);
        self.validate_step_policies(workflow, step);
        self.validate_side_effecting_step(workflow, step, skill);
    }

    fn validate_step_approval(
        &mut self,
        workflow: &LoadedSpec<WorkflowDefinition>,
        step: &StepDefinition,
        skill: &LoadedSpec<SkillDefinition>,
    ) {
        let has_approval_requirement = !workflow.definition.approval_requirements.is_empty()
            || step.approval_policy.is_some()
            || matches!(
                skill.definition.approval_sensitivity,
                ApprovalSensitivity::Medium | ApprovalSensitivity::High
            );
        if has_approval_requirement && step.approval_policy.is_none() {
            self.error(
                "validation.workflow.approval_policy_missing",
                format!("step {} requires an explicit approval_policy", step.id),
                &workflow.path,
                "$.steps",
            );
        }
    }

    fn validate_step_retry(
        &mut self,
        workflow: &LoadedSpec<WorkflowDefinition>,
        step: &StepDefinition,
    ) {
        if step.retry_policy.is_some() {
            if !matches!(
                step.terminal_behavior,
                Some(TerminalBehavior::FailWorkflow | TerminalBehavior::Escalate)
            ) && step.escalation_policy.is_none()
            {
                self.error(
                    "validation.workflow.retry_exhaustion_unsafe",
                    format!(
                        "step {} retry exhaustion must lead to escalation or terminal failure",
                        step.id
                    ),
                    &workflow.path,
                    "$.steps",
                );
            }
            if let Some(retry) = &step.retry_policy {
                self.validate_policy_ref(
                    &retry.policy,
                    "retry",
                    "validation.policy.retry_invalid",
                    &workflow.path,
                    "$.steps",
                );
                self.validate_policy_ref_context(
                    &retry.policy,
                    PolicyEffectContext::Retry,
                    &workflow.path,
                    "$.steps",
                );
            }
        }
    }

    fn validate_step_policies(
        &mut self,
        workflow: &LoadedSpec<WorkflowDefinition>,
        step: &StepDefinition,
    ) {
        if let Some(policy) = &step.approval_policy {
            self.validate_policy_ref(
                &policy.policy,
                "approval",
                "validation.policy.approval_invalid",
                &workflow.path,
                "$.steps",
            );
            self.validate_policy_ref_context(
                &policy.policy,
                PolicyEffectContext::Approval,
                &workflow.path,
                "$.steps",
            );
        }
        if let Some(policy) = &step.escalation_policy {
            self.validate_policy_ref(
                &policy.policy,
                "escalation",
                "validation.policy.escalation_invalid",
                &workflow.path,
                "$.steps",
            );
            self.validate_policy_ref_context(
                &policy.policy,
                PolicyEffectContext::Escalation,
                &workflow.path,
                "$.steps",
            );
        }
        for policy in &step.policy_requirements {
            self.validate_policy_exists(policy, &workflow.path, "$.steps");
            self.validate_policy_ref_context(
                policy,
                PolicyEffectContext::Requirement,
                &workflow.path,
                "$.steps",
            );
        }
    }

    fn validate_side_effecting_step(
        &mut self,
        workflow: &LoadedSpec<WorkflowDefinition>,
        step: &StepDefinition,
        skill: &LoadedSpec<SkillDefinition>,
    ) {
        if !skill.definition.adapter_requirements.is_empty() {
            if skill.definition.allowed_capabilities.is_empty() {
                self.error(
                    "validation.safety.capabilities_missing",
                    format!(
                        "external side-effecting step {} must use a skill with capabilities",
                        step.id
                    ),
                    &workflow.path,
                    "$.steps",
                );
            }
            if step.policy_requirements.is_empty() {
                self.error(
                    "validation.safety.policy_missing",
                    format!(
                        "external side-effecting step {} must declare policy checks",
                        step.id
                    ),
                    &workflow.path,
                    "$.steps",
                );
            }
            if skill_requests_external_write(&skill.definition) {
                self.error(
                    "validation.policy.external_write_unsupported",
                    "external write policy effects are not supported by the v0 runtime",
                    &workflow.path,
                    "$.steps",
                );
            }
            if skill_requests_secret_read(&skill.definition) {
                self.error(
                    "validation.policy.secret_read_unsupported",
                    "secret read policy effects are not supported by the v0 runtime",
                    &workflow.path,
                    "$.steps",
                );
            }
            if skill_requests_external_read(&skill.definition)
                && !self.policy_requirement_effects(step).allows_external_read()
            {
                self.error(
                    "validation.policy.external_read_missing",
                    "read-only adapter step requires allow_external_read policy effect",
                    &workflow.path,
                    "$.steps",
                );
            }
            if matches!(
                step.idempotency_key_strategy,
                IdempotencyKeyStrategy::Derived
            ) {
                // Derived is explicit in the model and safe for v0.
            }
            if step.timeout.is_none() && workflow.definition.timeout_policy.is_none() {
                self.error(
                    "validation.workflow.timeout_missing",
                    format!(
                        "external side-effecting step {} must declare timeout",
                        step.id
                    ),
                    &workflow.path,
                    "$.steps",
                );
            }
        }
        if matches!(
            workflow.definition.autonomy_level,
            AutonomyLevel::Level1Assistive
        ) && !skill.definition.adapter_requirements.is_empty()
        {
            self.error(
                "validation.workflow.autonomy_incompatible",
                "Level 1 workflows cannot declare adapter-backed side-effecting steps",
                &workflow.path,
                "$.autonomy_level",
            );
        }
    }

    fn validate_policy_ref_context(
        &mut self,
        policy: &PolicyReference,
        context: PolicyEffectContext,
        file: &Path,
        document_path: impl Into<String>,
    ) {
        let Some(loaded) = self.validate_policy_exists(policy, file, document_path) else {
            return;
        };
        let invalid = loaded
            .definition
            .rules
            .iter()
            .filter_map(|rule| PolicyEffect::parse(&rule.effect).ok())
            .any(|effect| !context.allows(effect));
        if invalid {
            self.error(
                "validation.policy.effect_context_invalid",
                "policy effect is not supported in this reference context",
                file,
                "$.policy",
            );
        }
    }

    fn policy_requirement_effects(&self, step: &StepDefinition) -> PolicyEffectSet {
        let mut effects = PolicyEffectSet::default();
        for policy_ref in &step.policy_requirements {
            let Some(policy) = self.policies.get(policy_ref.id.as_str()) else {
                continue;
            };
            insert_policy_effects(&mut effects, &policy.definition);
        }
        effects
    }

    fn resolve_skill(
        &mut self,
        workflow: &LoadedSpec<WorkflowDefinition>,
        step: &StepDefinition,
    ) -> Option<&'a LoadedSpec<SkillDefinition>> {
        let skill_id = step.skill_ref.id.as_str();
        let Some(versions) = self.skill_versions.get(skill_id) else {
            self.error(
                "validation.reference.skill_missing",
                format!("referenced skill {skill_id} does not exist"),
                &workflow.path,
                "$.steps",
            );
            return None;
        };
        let version = if let Some(version) = &step.skill_ref.version {
            version.as_str()
        } else if versions.len() == 1 {
            versions.iter().next().copied().unwrap_or_default()
        } else {
            self.error(
                "validation.reference.skill_version_ambiguous",
                format!("referenced skill {skill_id} has multiple versions; declare one"),
                &workflow.path,
                "$.steps",
            );
            return None;
        };
        if let Some(skill) = self.skills.get(&(skill_id, version)) {
            return Some(*skill);
        }
        self.error(
            "validation.reference.skill_version_missing",
            format!("referenced skill {skill_id}@{version} does not exist"),
            &workflow.path,
            "$.steps",
        );
        None
    }

    fn validate_branches(&mut self, workflow: &LoadedSpec<WorkflowDefinition>) {
        let step_ids: BTreeSet<&str> = workflow
            .definition
            .steps
            .iter()
            .map(|step| step.id.as_str())
            .collect();
        for branch in &workflow.definition.branches {
            if !step_ids.contains(branch.target_step.as_str()) {
                self.error(
                    "validation.workflow.invalid_state_transition",
                    format!(
                        "branch {} targets unknown step {}",
                        branch.id, branch.target_step
                    ),
                    &workflow.path,
                    "$.branches",
                );
            }
        }
    }

    fn validate_policy_ref(
        &mut self,
        policy: &PolicyReference,
        expected_effect: &str,
        code: &'static str,
        file: &Path,
        document_path: impl Into<String>,
    ) {
        let Some(loaded) = self.validate_policy_exists(policy, file, document_path) else {
            return;
        };
        let has_expected = loaded
            .definition
            .rules
            .iter()
            .any(|rule| policy_effect_matches(&rule.effect, expected_effect));
        if !has_expected {
            self.error(
                code,
                format!(
                    "policy {} does not declare required {expected_effect} behavior",
                    policy.id
                ),
                file,
                "$.policy",
            );
        }
        if expected_effect == "retry" {
            let unbounded = loaded
                .definition
                .rules
                .iter()
                .any(|rule| rule.effect == "unbounded_retry");
            let mut effects = PolicyEffectSet::default();
            insert_policy_effects(&mut effects, &loaded.definition);
            if unbounded || !effects.has_bounded_retry() {
                self.error(
                    "validation.policy.retry_unbounded",
                    format!("retry policy {} must be bounded", policy.id),
                    file,
                    "$.policy",
                );
            }
        }
    }

    fn validate_policy_exists(
        &mut self,
        policy: &PolicyReference,
        file: &Path,
        document_path: impl Into<String>,
    ) -> Option<&'a LoadedSpec<PolicySpecDocument>> {
        let id = policy.id.as_str();
        if let Some(policy) = self.policies.get(id) {
            return Some(*policy);
        }
        self.error(
            "validation.reference.policy_missing",
            format!("referenced policy {id} does not exist"),
            file,
            document_path,
        );
        None
    }

    fn error(
        &mut self,
        code: &'static str,
        message: impl Into<String>,
        file: &Path,
        document_path: impl Into<String>,
    ) {
        let diagnostic = Diagnostic::error(code, message.into())
            .with_source_location(SourceLocation::new(file).with_document_path(document_path));
        if code == "validation.schema_version.unsupported" {
            self.diagnostics
                .push(with_spec_file_evidence_from_source_location(diagnostic));
        } else {
            self.diagnostics.push(diagnostic);
        }
    }
}

fn report_duplicates<T>(
    specs: &[LoadedSpec<T>],
    id: impl Fn(&LoadedSpec<T>) -> &str,
    code: &'static str,
    message: &'static str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut seen = BTreeSet::new();
    for spec in specs {
        let id = id(spec);
        if !seen.insert(id.to_owned()) {
            diagnostics.push(
                Diagnostic::error(code, format!("{message}: {id}")).with_source_location(
                    SourceLocation::new(&spec.path).with_document_path("$.id"),
                ),
            );
        }
    }
}

fn warn_lifecycle(
    diagnostics: &mut Vec<Diagnostic>,
    lifecycle_status: LifecycleStatus,
    file: &Path,
) {
    match lifecycle_status {
        LifecycleStatus::Experimental => diagnostics.push(
            Diagnostic::warning(
                "validation.lifecycle.experimental",
                "experimental definition is not stable API",
            )
            .with_source_location(
                SourceLocation::new(file).with_document_path("$.owner.lifecycle_status"),
            ),
        ),
        LifecycleStatus::Deprecated => diagnostics.push(
            Diagnostic::warning(
                "validation.lifecycle.deprecated",
                "deprecated definition should not be used for new work",
            )
            .with_source_location(
                SourceLocation::new(file).with_document_path("$.owner.lifecycle_status"),
            ),
        ),
        LifecycleStatus::Stable => {}
    }
}

fn policy_effect_matches(effect: &str, expected_effect: &str) -> bool {
    let Ok(effect) = PolicyEffect::parse(effect) else {
        return false;
    };
    matches!(
        (effect, expected_effect),
        (PolicyEffect::RequireApproval, "approval")
            | (PolicyEffect::Escalate, "escalation")
            | (
                PolicyEffect::Retry | PolicyEffect::BoundedRetry | PolicyEffect::MaxAttempts(_),
                "retry",
            )
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PolicyEffectContext {
    Requirement,
    Approval,
    Retry,
    Escalation,
}

impl PolicyEffectContext {
    fn allows(self, effect: PolicyEffect) -> bool {
        match self {
            Self::Requirement => matches!(
                effect,
                PolicyEffect::AllowLocal | PolicyEffect::AllowExternalRead
            ),
            Self::Approval => matches!(effect, PolicyEffect::RequireApproval),
            Self::Retry => matches!(
                effect,
                PolicyEffect::Retry | PolicyEffect::BoundedRetry | PolicyEffect::MaxAttempts(_)
            ),
            Self::Escalation => matches!(effect, PolicyEffect::Escalate),
        }
    }
}

fn insert_policy_effects(effects: &mut PolicyEffectSet, policy: &PolicySpecDocument) {
    for rule in &policy.rules {
        if let Ok(effect) = PolicyEffect::parse(&rule.effect) {
            effects.insert(effect);
        }
    }
}

fn skill_requests_external_read(skill: &SkillDefinition) -> bool {
    skill_allowed_capability(skill, "external.read") || adapter_capability(skill, "external.read")
}

fn skill_requests_external_write(skill: &SkillDefinition) -> bool {
    skill_allowed_capability(skill, "external.write") || adapter_capability(skill, "external.write")
}

fn skill_requests_secret_read(skill: &SkillDefinition) -> bool {
    skill_allowed_capability(skill, "secret.read") || adapter_capability(skill, "secret.read")
}

fn skill_allowed_capability(skill: &SkillDefinition, capability: &str) -> bool {
    skill
        .allowed_capabilities
        .iter()
        .any(|declared| declared.name == capability)
}

fn adapter_capability(skill: &SkillDefinition, capability: &str) -> bool {
    skill
        .adapter_requirements
        .iter()
        .flat_map(|adapter| adapter.capabilities.iter())
        .any(|declared| declared == capability)
}
