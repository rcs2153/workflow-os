use serde::{Deserialize, Serialize};

use crate::{
    ActorId, ApprovalSensitivity, AutonomyLevel, CorrelationId, SkillId, StepId, WorkflowId,
    WorkflowRunId,
};

/// Runtime capability required for an action.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// Local deterministic read capability.
    LocalRead,
    /// Local deterministic write capability.
    LocalWrite,
    /// External read capability.
    ExternalRead,
    /// External write capability.
    ExternalWrite,
    /// Request human approval.
    ApprovalRequest,
    /// Cancel workflow runs.
    WorkflowCancel,
    /// Resume workflow runs.
    WorkflowResume,
    /// Invoke an adapter.
    AdapterInvoke,
    /// Read a secret.
    SecretRead,
    /// Write audit records.
    AuditWrite,
    /// Unknown capability, denied by default.
    Unknown(String),
}

impl Capability {
    /// Parses one declared capability name into the internal runtime vocabulary.
    #[must_use]
    pub(crate) fn from_declared_name(name: &str) -> Self {
        match name {
            "local.read" => Self::LocalRead,
            "local.write" => Self::LocalWrite,
            "external.read" => Self::ExternalRead,
            "external.write" => Self::ExternalWrite,
            "approval.request" => Self::ApprovalRequest,
            "workflow.cancel" => Self::WorkflowCancel,
            "workflow.resume" => Self::WorkflowResume,
            "adapter.invoke" => Self::AdapterInvoke,
            "secret.read" => Self::SecretRead,
            "audit.write" => Self::AuditWrite,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

/// Runtime action being evaluated by policy.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Start a workflow run.
    StartWorkflow,
    /// Request approval.
    RequestApproval,
    /// Resume a workflow run.
    ResumeWorkflow,
    /// Invoke a local skill handler.
    InvokeSkill,
    /// Invoke an adapter.
    InvokeAdapter,
    /// Cancel a workflow run.
    CancelWorkflow,
    /// Inspect workflow state.
    InspectWorkflow,
    /// Unknown action, denied by default.
    Unknown(String),
}

/// Violation returned by policy evaluation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolicyViolation {
    /// Stable reason code.
    pub code: String,
    /// Human-readable non-secret message.
    pub message: String,
}

/// Typed v0 policy effect parsed from policy rule declarations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyEffect {
    /// Allows local execution capability declarations.
    AllowLocal,
    /// Allows supported read-only external adapter access.
    AllowExternalRead,
    /// Requires an approval checkpoint before the scoped action.
    RequireApproval,
    /// Declares bounded retry behavior.
    BoundedRetry,
    /// Compatibility retry declaration. Retry remains bounded in v0.
    Retry,
    /// Declares local escalation behavior.
    Escalate,
    /// Retry attempt cap.
    MaxAttempts(u32),
}

impl PolicyEffect {
    /// Parses a policy rule effect into the supported v0 vocabulary.
    ///
    /// # Errors
    ///
    /// Returns a stable non-secret code for unsupported or malformed effects.
    pub fn parse(effect: &str) -> Result<Self, PolicyEffectParseError> {
        match effect {
            "allow_local" => Ok(Self::AllowLocal),
            "allow_external_read" => Ok(Self::AllowExternalRead),
            "require_approval" | "approval" | "approval_policy" => Ok(Self::RequireApproval),
            "bounded_retry" => Ok(Self::BoundedRetry),
            "retry" | "retry_policy" => Ok(Self::Retry),
            "escalate" | "escalation" | "escalation_policy" => Ok(Self::Escalate),
            other => parse_max_attempts_effect(other),
        }
    }
}

/// Stable error returned when parsing policy effect vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolicyEffectParseError {
    code: &'static str,
}

impl PolicyEffectParseError {
    /// Stable non-secret error code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        self.code
    }
}

/// Step-scoped typed policy effects used by runtime policy evaluation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PolicyEffectSet {
    effects: Vec<PolicyEffect>,
    max_attempts: Option<u32>,
}

impl PolicyEffectSet {
    /// Adds one parsed effect.
    pub fn insert(&mut self, effect: PolicyEffect) {
        match effect {
            PolicyEffect::MaxAttempts(value) => self.max_attempts = Some(value),
            other => {
                if !self.effects.contains(&other) {
                    self.effects.push(other);
                }
            }
        }
    }

    /// Returns whether local access is explicitly allowed by policy.
    #[must_use]
    pub fn allows_local(&self) -> bool {
        self.effects.contains(&PolicyEffect::AllowLocal)
    }

    /// Returns whether read-only external adapter access is explicitly allowed.
    #[must_use]
    pub fn allows_external_read(&self) -> bool {
        self.effects.contains(&PolicyEffect::AllowExternalRead)
    }

    /// Returns whether approval is explicitly required by policy.
    #[must_use]
    pub fn requires_approval(&self) -> bool {
        self.effects.contains(&PolicyEffect::RequireApproval)
    }

    /// Returns whether retry is explicitly bounded by policy.
    #[must_use]
    pub fn has_bounded_retry(&self) -> bool {
        self.effects.contains(&PolicyEffect::BoundedRetry)
            || self.effects.contains(&PolicyEffect::Retry)
            || self.max_attempts.is_some()
    }

    /// Returns whether escalation is explicitly enabled by policy.
    #[must_use]
    pub fn allows_escalation(&self) -> bool {
        self.effects.contains(&PolicyEffect::Escalate)
    }

    /// Returns the parsed retry attempt cap, if any.
    #[must_use]
    pub const fn max_attempts(&self) -> Option<u32> {
        self.max_attempts
    }
}

/// Runtime policy decision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolicyDecision {
    /// Whether the action is allowed.
    pub allowed: bool,
    /// Whether human approval is required before the action.
    pub requires_approval: bool,
    /// Stable reason codes.
    pub reason_codes: Vec<String>,
    /// Policy violations, if any.
    pub violations: Vec<PolicyViolation>,
    /// Evaluated action.
    pub action: Action,
    /// Required capabilities.
    pub capabilities: Vec<Capability>,
    /// Actor evaluated by policy.
    pub actor: Option<ActorId>,
    /// Workflow ID evaluated by policy.
    pub workflow_id: Option<WorkflowId>,
    /// Run ID evaluated by policy, when a run exists.
    pub run_id: Option<WorkflowRunId>,
    /// Correlation ID for audit linkage.
    pub correlation_id: Option<CorrelationId>,
}

/// Context supplied to the policy engine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyEvaluationContext {
    /// Action being evaluated.
    pub action: Action,
    /// Capabilities required by the action.
    pub capabilities: Vec<Capability>,
    /// Actor requesting the action.
    pub actor: Option<ActorId>,
    /// Workflow ID.
    pub workflow_id: Option<WorkflowId>,
    /// Run ID when available.
    pub run_id: Option<WorkflowRunId>,
    /// Step ID when step-scoped.
    pub step_id: Option<StepId>,
    /// Skill ID when skill-scoped.
    pub skill_id: Option<SkillId>,
    /// Workflow autonomy level.
    pub autonomy_level: Option<AutonomyLevel>,
    /// Skill approval sensitivity.
    pub approval_sensitivity: Option<ApprovalSensitivity>,
    /// Whether an explicit approval policy exists.
    pub has_approval_policy: bool,
    /// Typed effects resolved from referenced policy specs.
    pub policy_effects: PolicyEffectSet,
    /// Whether an adapter is referenced.
    pub adapter_id: Option<String>,
    /// Correlation ID.
    pub correlation_id: Option<CorrelationId>,
}

/// Deterministic conservative policy engine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConservativePolicyEngine {
    kill_switch_enabled: bool,
    level_3_4_enabled: bool,
    secret_read_enabled: bool,
}

impl Default for ConservativePolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ConservativePolicyEngine {
    /// Creates the default conservative policy engine.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            kill_switch_enabled: false,
            level_3_4_enabled: false,
            secret_read_enabled: false,
        }
    }

    /// Creates a policy engine with the kill switch enabled.
    #[must_use]
    pub const fn kill_switch() -> Self {
        Self {
            kill_switch_enabled: true,
            level_3_4_enabled: false,
            secret_read_enabled: false,
        }
    }

    /// Returns whether the kill switch is enabled.
    #[must_use]
    pub const fn kill_switch_enabled(&self) -> bool {
        self.kill_switch_enabled
    }

    /// Evaluates a policy context deterministically.
    #[must_use]
    pub fn evaluate(&self, context: &PolicyEvaluationContext) -> PolicyDecision {
        let mut decision = PolicyDecision {
            allowed: true,
            requires_approval: false,
            reason_codes: vec!["policy.allow.default_conservative".to_owned()],
            violations: Vec::new(),
            action: context.action.clone(),
            capabilities: context.capabilities.clone(),
            actor: context.actor.clone(),
            workflow_id: context.workflow_id.clone(),
            run_id: context.run_id.clone(),
            correlation_id: context.correlation_id.clone(),
        };

        Self::check_required_context(context, &mut decision);
        self.check_kill_switch(context, &mut decision);
        Self::check_action(context, &mut decision);
        self.check_capabilities(context, &mut decision);
        self.check_autonomy(context, &mut decision);
        Self::check_approval(context, &mut decision);

        if !decision.violations.is_empty() {
            decision.allowed = false;
        }
        decision
    }

    fn check_required_context(context: &PolicyEvaluationContext, decision: &mut PolicyDecision) {
        if context.actor.is_none() {
            deny(
                decision,
                "policy.deny.missing_actor",
                "policy context is missing actor",
            );
        }
        if context.workflow_id.is_none() {
            deny(
                decision,
                "policy.deny.missing_workflow",
                "policy context is missing workflow id",
            );
        }
        if context.correlation_id.is_none() {
            deny(
                decision,
                "policy.deny.missing_correlation",
                "policy context is missing correlation id",
            );
        }
        if context.capabilities.is_empty() {
            deny(
                decision,
                "policy.deny.missing_capability",
                "policy context declares no capability",
            );
        }
    }

    fn check_kill_switch(&self, context: &PolicyEvaluationContext, decision: &mut PolicyDecision) {
        if self.kill_switch_enabled
            && !matches!(
                context.action,
                Action::CancelWorkflow | Action::InspectWorkflow
            )
        {
            deny(
                decision,
                "policy.deny.kill_switch",
                "kill switch prevents non-terminal mutating actions",
            );
        }
    }

    fn check_action(context: &PolicyEvaluationContext, decision: &mut PolicyDecision) {
        if matches!(context.action, Action::Unknown(_)) {
            deny(
                decision,
                "policy.deny.unknown_action",
                "unknown action is denied",
            );
        }
        if matches!(context.action, Action::InvokeAdapter) && !is_phase2_read_only_adapter(context)
        {
            deny(
                decision,
                "policy.deny.adapter_invoke_v0",
                "adapter invocation is denied unless it is an explicitly supported Phase 2 read-only adapter",
            );
        }
        if context.adapter_id.is_some() && !is_phase2_read_only_adapter(context) {
            deny(
                decision,
                "policy.deny.unknown_adapter",
                "adapter references are denied unless explicitly supported by Phase 2 read-only policy",
            );
        }
    }

    fn check_capabilities(&self, context: &PolicyEvaluationContext, decision: &mut PolicyDecision) {
        for capability in &context.capabilities {
            match capability {
                Capability::Unknown(_) => deny(
                    decision,
                    "policy.deny.unknown_capability",
                    "unknown capability is denied",
                ),
                Capability::ExternalWrite => deny(
                    decision,
                    "policy.deny.external_write_v0",
                    "external.write is denied in v0",
                ),
                Capability::SecretRead if !self.secret_read_enabled => deny(
                    decision,
                    "policy.deny.secret_read",
                    "secret.read requires explicit future configuration",
                ),
                Capability::AdapterInvoke if !is_phase2_read_only_adapter(context) => {
                    deny(
                        decision,
                        "policy.deny.adapter_invoke_v0",
                        "adapter.invoke is denied unless it is an explicitly supported Phase 2 read-only adapter",
                    );
                }
                Capability::LocalRead
                | Capability::LocalWrite
                | Capability::ExternalRead
                | Capability::ApprovalRequest
                | Capability::WorkflowCancel
                | Capability::WorkflowResume
                | Capability::AdapterInvoke
                | Capability::AuditWrite
                | Capability::SecretRead => {}
            }
        }
    }

    fn check_autonomy(&self, context: &PolicyEvaluationContext, decision: &mut PolicyDecision) {
        if matches!(
            context.autonomy_level,
            Some(AutonomyLevel::Level3ConditionalAutonomy | AutonomyLevel::Level4ScaledAutomation)
        ) && !self.level_3_4_enabled
        {
            deny(
                decision,
                "policy.deny.autonomy_level",
                "Level 3/4 execution is denied by default",
            );
        }
    }

    fn check_approval(context: &PolicyEvaluationContext, decision: &mut PolicyDecision) {
        if matches!(
            context.approval_sensitivity,
            Some(ApprovalSensitivity::Medium | ApprovalSensitivity::High)
        ) && !context.has_approval_policy
        {
            decision.requires_approval = true;
            deny(
                decision,
                "policy.deny.approval_required",
                "sensitive action requires explicit approval policy",
            );
        } else if context.has_approval_policy {
            decision.requires_approval = true;
            add_reason(decision, "policy.requires_approval");
        }
    }
}

fn deny(decision: &mut PolicyDecision, code: &str, message: &str) {
    add_reason(decision, code);
    decision.violations.push(PolicyViolation {
        code: code.to_owned(),
        message: message.to_owned(),
    });
}

fn add_reason(decision: &mut PolicyDecision, code: &str) {
    if !decision
        .reason_codes
        .iter()
        .any(|existing| existing == code)
    {
        decision.reason_codes.push(code.to_owned());
    }
}

fn is_phase2_read_only_adapter(context: &PolicyEvaluationContext) -> bool {
    matches!(context.action, Action::InvokeAdapter)
        && context.policy_effects.allows_external_read()
        && matches!(
            context.adapter_id.as_deref(),
            Some(
                "symbolic/github-read-only"
                    | "symbolic/jira-read-only"
                    | "symbolic/ci-read-only"
                    | "symbolic/github-actions-read-only"
            )
        )
        && context
            .capabilities
            .iter()
            .any(|capability| matches!(capability, Capability::ExternalRead))
        && !context.capabilities.iter().any(|capability| {
            matches!(
                capability,
                Capability::ExternalWrite | Capability::SecretRead | Capability::Unknown(_)
            )
        })
}

fn parse_max_attempts_effect(effect: &str) -> Result<PolicyEffect, PolicyEffectParseError> {
    let Some(value) = effect
        .strip_prefix("max_attempts=")
        .or_else(|| effect.strip_prefix("max_attempts:"))
    else {
        return Err(PolicyEffectParseError {
            code: "validation.policy.effect_unsupported",
        });
    };
    let attempts = value.parse::<u32>().map_err(|_| PolicyEffectParseError {
        code: "validation.policy.effect_parameter_invalid",
    })?;
    if attempts == 0 {
        return Err(PolicyEffectParseError {
            code: "validation.policy.effect_parameter_invalid",
        });
    }
    Ok(PolicyEffect::MaxAttempts(attempts))
}
