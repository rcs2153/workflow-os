use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::{
    validate_project_bundle, ActorId, ImmutableRunBundleDefinitionRecord,
    ImmutableRunBundleExecutionPosture, ImmutableRunBundleHandlerReference, ImmutableRunBundleId,
    ImmutableRunBundleManifest, ImmutableRunBundleSensitivity, ImmutableRunBundleVersion,
    LoadedSpec, PolicyId, PolicySpecDocument, ProjectBundle, SkillDefinition, SkillId,
    SkillVersion, SpecContentHash, Timestamp, WorkflowDefinition, WorkflowId, WorkflowOsError,
    WorkflowRunId,
};

/// Explicit inputs for constructing one immutable run bundle in memory.
pub struct ImmutableRunBundleBuildRequest<'a> {
    /// Already loaded project whose validation is rechecked before construction.
    pub project: &'a ProjectBundle,
    /// Workflow selected for the future run.
    pub workflow_id: &'a WorkflowId,
    /// Caller-selected immutable bundle identity.
    pub bundle_id: ImmutableRunBundleId,
    /// Version of the bundle manifest and canonical definition records.
    pub bundle_version: ImmutableRunBundleVersion,
    /// Future run identity bound by the manifest without creating runtime state.
    pub run_id: WorkflowRunId,
    /// Accepted resolved execution-context commitment.
    pub resolved_execution_context_hash: SpecContentHash,
    /// Explicit bounded execution posture.
    pub execution_posture: ImmutableRunBundleExecutionPosture,
    /// Honest handler posture for every unique resolved skill.
    pub handlers: Vec<ImmutableRunBundleHandlerReference>,
    /// Manifest creation time supplied by the caller.
    pub created_at: Timestamp,
    /// System or human actor constructing the in-memory bundle.
    pub created_by: ActorId,
    /// Conservative bundle and definition-record sensitivity.
    pub sensitivity: ImmutableRunBundleSensitivity,
    /// Whether downstream handling must redact bundle material.
    pub redaction_required: bool,
}

impl fmt::Debug for ImmutableRunBundleBuildRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleBuildRequest")
            .field("project", &"[REDACTED]")
            .field("workflow_id", &"[REDACTED]")
            .field("bundle_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("handler_count", &self.handlers.len())
            .field("sensitivity", &self.sensitivity)
            .field("redaction_required", &self.redaction_required)
            .finish_non_exhaustive()
    }
}

/// Validated in-memory immutable run bundle and its canonical definition records.
#[derive(Clone, Eq, PartialEq)]
pub struct ImmutableRunBundleBuildResult {
    manifest: ImmutableRunBundleManifest,
    definition_records: Vec<ImmutableRunBundleDefinitionRecord>,
}

impl ImmutableRunBundleBuildResult {
    /// Returns the validated immutable bundle manifest.
    #[must_use]
    pub const fn manifest(&self) -> &ImmutableRunBundleManifest {
        &self.manifest
    }

    /// Returns canonical records required by the manifest.
    #[must_use]
    pub fn definition_records(&self) -> &[ImmutableRunBundleDefinitionRecord] {
        &self.definition_records
    }

    /// Consumes the result into its manifest and canonical records.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ImmutableRunBundleManifest,
        Vec<ImmutableRunBundleDefinitionRecord>,
    ) {
        (self.manifest, self.definition_records)
    }
}

impl fmt::Debug for ImmutableRunBundleBuildResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmutableRunBundleBuildResult")
            .field("manifest", &self.manifest)
            .field("definition_record_count", &self.definition_records.len())
            .finish_non_exhaustive()
    }
}

/// Constructs one validated immutable run bundle without persistence or runtime mutation.
///
/// # Errors
///
/// Returns a stable bounded error when the project is invalid, the selected workflow or one of
/// its references cannot be resolved, canonical record construction fails, or manifest
/// invariants do not hold.
pub fn build_immutable_run_bundle(
    request: ImmutableRunBundleBuildRequest<'_>,
) -> Result<ImmutableRunBundleBuildResult, WorkflowOsError> {
    if validate_project_bundle(request.project).has_errors() {
        return Err(builder_error(
            "immutable_run_bundle.builder.project_invalid",
            "immutable run bundle requires a valid project",
        ));
    }

    let workflow = resolve_workflow(request.project, request.workflow_id)?;
    let record_version = request.bundle_version.clone();
    let workflow_record = ImmutableRunBundleDefinitionRecord::from_workflow(
        record_version.clone(),
        workflow.definition.clone(),
        workflow.content_hash.clone(),
        request.sensitivity,
        request.redaction_required,
    )?;

    let mut definition_references = vec![workflow_record.definition_reference(None)?];
    let mut skill_records = BTreeMap::<(SkillId, SkillVersion), _>::new();
    for step in &workflow.definition.steps {
        let skill = resolve_skill(&request.project.skills, step)?;
        let key = (
            skill.definition.id.clone(),
            skill.definition.version.clone(),
        );
        if !skill_records.contains_key(&key) {
            let record = ImmutableRunBundleDefinitionRecord::from_skill(
                record_version.clone(),
                skill.definition.clone(),
                skill.content_hash.clone(),
                request.sensitivity,
                request.redaction_required,
            )?;
            skill_records.insert(key.clone(), record);
        }
        let record = skill_records.get(&key).ok_or_else(|| {
            builder_error(
                "immutable_run_bundle.builder.skill_record_missing",
                "resolved immutable run bundle skill record is unavailable",
            )
        })?;
        definition_references.push(record.definition_reference(Some(step.id.clone()))?);
    }

    let policy_ids = referenced_policy_ids(&workflow.definition);
    let mut policy_records = BTreeMap::<PolicyId, ImmutableRunBundleDefinitionRecord>::new();
    for policy_id in policy_ids {
        let policy = resolve_policy(&request.project.policies, &policy_id)?;
        let record = ImmutableRunBundleDefinitionRecord::from_policy(
            record_version.clone(),
            &policy.definition,
            policy.content_hash.clone(),
            request.sensitivity,
            request.redaction_required,
        )?;
        definition_references.push(record.definition_reference(None)?);
        policy_records.insert(policy_id, record);
    }

    let manifest = ImmutableRunBundleManifest::new(
        request.bundle_id,
        request.bundle_version,
        request.run_id,
        workflow.definition.id.clone(),
        workflow.definition.version.clone(),
        workflow.definition.schema_version.clone(),
        workflow.content_hash.clone(),
        request.resolved_execution_context_hash,
        definition_references,
        request.execution_posture,
        request.handlers,
        request.created_at,
        request.created_by,
        request.sensitivity,
        request.redaction_required,
    )?;

    let mut definition_records = Vec::with_capacity(1 + skill_records.len() + policy_records.len());
    definition_records.push(workflow_record);
    definition_records.extend(skill_records.into_values());
    definition_records.extend(policy_records.into_values());

    Ok(ImmutableRunBundleBuildResult {
        manifest,
        definition_records,
    })
}

fn resolve_workflow<'a>(
    project: &'a ProjectBundle,
    workflow_id: &WorkflowId,
) -> Result<&'a LoadedSpec<WorkflowDefinition>, WorkflowOsError> {
    project
        .workflows
        .iter()
        .find(|workflow| &workflow.definition.id == workflow_id)
        .ok_or_else(|| {
            builder_error(
                "immutable_run_bundle.builder.workflow_not_found",
                "immutable run bundle workflow was not found",
            )
        })
}

fn resolve_skill<'a>(
    skills: &'a [LoadedSpec<SkillDefinition>],
    step: &crate::StepDefinition,
) -> Result<&'a LoadedSpec<SkillDefinition>, WorkflowOsError> {
    let matches = skills
        .iter()
        .filter(|skill| skill.definition.id == step.skill_ref.id)
        .filter(|skill| {
            step.skill_ref
                .version
                .as_ref()
                .map_or(true, |version| &skill.definition.version == version)
        })
        .collect::<Vec<_>>();
    let [skill] = matches.as_slice() else {
        return Err(builder_error(
            "immutable_run_bundle.builder.skill_unresolved",
            "immutable run bundle skill reference could not be resolved uniquely",
        ));
    };
    Ok(*skill)
}

fn referenced_policy_ids(workflow: &WorkflowDefinition) -> BTreeSet<PolicyId> {
    let mut policy_ids = BTreeSet::new();
    policy_ids.extend(
        workflow
            .retry_policy_refs
            .iter()
            .chain(&workflow.escalation_policy_refs)
            .map(|reference| reference.id.clone()),
    );
    for step in &workflow.steps {
        policy_ids.extend(
            step.policy_requirements
                .iter()
                .map(|reference| reference.id.clone()),
        );
        if let Some(reference) = &step.approval_policy {
            policy_ids.insert(reference.policy.id.clone());
        }
        if let Some(reference) = &step.retry_policy {
            policy_ids.insert(reference.policy.id.clone());
        }
        if let Some(reference) = &step.escalation_policy {
            policy_ids.insert(reference.policy.id.clone());
        }
    }
    policy_ids
}

fn resolve_policy<'a>(
    policies: &'a [LoadedSpec<PolicySpecDocument>],
    policy_id: &PolicyId,
) -> Result<&'a LoadedSpec<PolicySpecDocument>, WorkflowOsError> {
    policies
        .iter()
        .find(|policy| &policy.definition.id == policy_id)
        .ok_or_else(|| {
            builder_error(
                "immutable_run_bundle.builder.policy_not_found",
                "immutable run bundle policy was not found",
            )
        })
}

fn builder_error(code: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
