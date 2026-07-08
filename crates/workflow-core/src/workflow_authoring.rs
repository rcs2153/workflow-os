use std::collections::BTreeSet;
use std::fmt;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::{ActorId, SpecContentHash, WorkflowId, WorkflowOsError};

const REVIEW_TEXT_MAX_BYTES: usize = 256;
const REVIEW_PATH_MAX_BYTES: usize = 192;
const REVIEW_CODE_MAX_BYTES: usize = 96;
const REVIEW_CODE_MAX_COUNT: usize = 32;

/// Preflight status supplied to steward review.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowDraftPromotionPreflightStatus {
    /// Preflight found no deterministic blockers.
    Passed,
    /// Preflight found blockers and promotion review must fail closed.
    Blocked,
}

/// Steward decision for an inactive workflow draft.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowDraftStewardReviewDecision {
    /// Draft is approved for a future active-promotion step if unchanged.
    ApprovedForPromotion,
    /// Draft must not be promoted.
    Denied,
    /// Draft should remain inactive until changed and re-reviewed.
    NeedsChanges,
    /// No promotion decision has been made.
    Deferred,
}

/// Whether a steward-review result authorizes a future promotion step.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowDraftStewardReviewAuthorization {
    /// The unchanged draft may proceed to a separately implemented promotion step.
    AuthorizedForPromotion,
    /// The draft is not authorized for promotion.
    NotAuthorized,
}

/// Explicit input for pure, in-memory steward review of a workflow draft.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDraftStewardReviewInput {
    /// Relative draft path under `workflows/drafts/`.
    pub draft_path: String,
    /// Candidate workflow id found in the draft.
    pub candidate_workflow_id: WorkflowId,
    /// Draft content hash that preflight evaluated.
    pub preflight_draft_content_hash: SpecContentHash,
    /// Current draft content hash at steward-review time.
    pub current_draft_content_hash: SpecContentHash,
    /// Preflight status for the draft.
    pub preflight_status: WorkflowDraftPromotionPreflightStatus,
    /// Bounded preflight blocker codes.
    pub preflight_blockers: Vec<String>,
    /// Bounded preflight warning codes.
    pub preflight_warnings: Vec<String>,
    /// Bounded owner/maintainer posture summary.
    pub owner_summary: String,
    /// Bounded escalation posture summary.
    pub escalation_summary: String,
    /// Bounded policy posture summary.
    pub policy_summary: String,
    /// Bounded evidence/report posture summary.
    pub evidence_report_summary: String,
    /// Bounded side-effect posture summary.
    pub side_effect_summary: String,
    /// Whether an active workflow id/path conflict is present.
    pub active_workflow_conflict: bool,
    /// Steward or delegated maintainer actor.
    pub reviewer: ActorId,
    /// Steward review decision.
    pub decision: WorkflowDraftStewardReviewDecision,
    /// Bounded approval or review reason.
    pub approval_reason: String,
}

impl fmt::Debug for WorkflowDraftStewardReviewInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowDraftStewardReviewInput")
            .field("draft_path", &self.draft_path)
            .field("candidate_workflow_id", &self.candidate_workflow_id)
            .field(
                "preflight_draft_content_hash",
                &self.preflight_draft_content_hash,
            )
            .field(
                "current_draft_content_hash",
                &self.current_draft_content_hash,
            )
            .field("preflight_status", &self.preflight_status)
            .field("preflight_blockers", &self.preflight_blockers)
            .field("preflight_warnings", &self.preflight_warnings)
            .field("owner_summary", &"[REDACTED]")
            .field("escalation_summary", &"[REDACTED]")
            .field("policy_summary", &"[REDACTED]")
            .field("evidence_report_summary", &"[REDACTED]")
            .field("side_effect_summary", &"[REDACTED]")
            .field("active_workflow_conflict", &self.active_workflow_conflict)
            .field("reviewer", &self.reviewer)
            .field("decision", &self.decision)
            .field("approval_reason", &"[REDACTED]")
            .finish()
    }
}

/// Bounded, human-reviewable card returned by steward review.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDraftStewardReviewCard {
    draft_path: String,
    candidate_workflow_id: WorkflowId,
    draft_content_hash: SpecContentHash,
    preflight_status: WorkflowDraftPromotionPreflightStatus,
    preflight_warnings: Vec<String>,
    owner_summary: String,
    escalation_summary: String,
    policy_summary: String,
    evidence_report_summary: String,
    side_effect_summary: String,
    approval_allows: String,
    approval_does_not_allow: String,
    next_action: String,
}

impl WorkflowDraftStewardReviewCard {
    /// Returns the relative draft path under review.
    #[must_use]
    pub fn draft_path(&self) -> &str {
        &self.draft_path
    }

    /// Returns the candidate workflow id.
    #[must_use]
    pub const fn candidate_workflow_id(&self) -> &WorkflowId {
        &self.candidate_workflow_id
    }

    /// Returns the reviewed draft content hash.
    #[must_use]
    pub const fn draft_content_hash(&self) -> &SpecContentHash {
        &self.draft_content_hash
    }

    /// Returns the preflight status supplied to review.
    #[must_use]
    pub const fn preflight_status(&self) -> WorkflowDraftPromotionPreflightStatus {
        self.preflight_status
    }

    /// Returns bounded preflight warning codes.
    #[must_use]
    pub fn preflight_warnings(&self) -> &[String] {
        &self.preflight_warnings
    }

    /// Returns the bounded owner posture summary.
    #[must_use]
    pub fn owner_summary(&self) -> &str {
        &self.owner_summary
    }

    /// Returns the bounded escalation posture summary.
    #[must_use]
    pub fn escalation_summary(&self) -> &str {
        &self.escalation_summary
    }

    /// Returns the bounded policy posture summary.
    #[must_use]
    pub fn policy_summary(&self) -> &str {
        &self.policy_summary
    }

    /// Returns the bounded evidence/report posture summary.
    #[must_use]
    pub fn evidence_report_summary(&self) -> &str {
        &self.evidence_report_summary
    }

    /// Returns the bounded side-effect posture summary.
    #[must_use]
    pub fn side_effect_summary(&self) -> &str {
        &self.side_effect_summary
    }

    /// Returns what steward approval allows.
    #[must_use]
    pub fn approval_allows(&self) -> &str {
        &self.approval_allows
    }

    /// Returns what steward approval explicitly does not allow.
    #[must_use]
    pub fn approval_does_not_allow(&self) -> &str {
        &self.approval_does_not_allow
    }

    /// Returns the next action for the reviewed draft.
    #[must_use]
    pub fn next_action(&self) -> &str {
        &self.next_action
    }
}

impl fmt::Debug for WorkflowDraftStewardReviewCard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowDraftStewardReviewCard")
            .field("draft_path", &self.draft_path)
            .field("candidate_workflow_id", &self.candidate_workflow_id)
            .field("draft_content_hash", &self.draft_content_hash)
            .field("preflight_status", &self.preflight_status)
            .field("preflight_warnings", &self.preflight_warnings)
            .field("owner_summary", &"[REDACTED]")
            .field("escalation_summary", &"[REDACTED]")
            .field("policy_summary", &"[REDACTED]")
            .field("evidence_report_summary", &"[REDACTED]")
            .field("side_effect_summary", &"[REDACTED]")
            .field("approval_allows", &self.approval_allows)
            .field("approval_does_not_allow", &self.approval_does_not_allow)
            .field("next_action", &self.next_action)
            .finish()
    }
}

/// Explicit operation boundary for steward review.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct WorkflowDraftStewardReviewBoundary {
    /// Whether files were written.
    pub files_written: bool,
    /// Whether a workflow was registered.
    pub workflow_registered: bool,
    /// Whether a workflow was promoted.
    pub workflow_promoted: bool,
    /// Whether approval state was persisted.
    pub approval_persisted: bool,
    /// Whether runtime state was created.
    pub runtime_state_created: bool,
    /// Whether commands were executed.
    pub commands_executed: bool,
    /// Whether providers were called.
    pub providers_called: bool,
}

impl WorkflowDraftStewardReviewBoundary {
    /// Returns the prohibited-operation boundary for pure steward review.
    #[must_use]
    pub const fn prohibited() -> Self {
        Self {
            files_written: false,
            workflow_registered: false,
            workflow_promoted: false,
            approval_persisted: false,
            runtime_state_created: false,
            commands_executed: false,
            providers_called: false,
        }
    }
}

/// Result of pure steward review.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDraftStewardReviewResult {
    card: WorkflowDraftStewardReviewCard,
    reviewer: ActorId,
    decision: WorkflowDraftStewardReviewDecision,
    authorization: WorkflowDraftStewardReviewAuthorization,
    approval_reason: String,
    boundary: WorkflowDraftStewardReviewBoundary,
}

impl WorkflowDraftStewardReviewResult {
    /// Returns the bounded approval card.
    #[must_use]
    pub const fn card(&self) -> &WorkflowDraftStewardReviewCard {
        &self.card
    }

    /// Returns the reviewer actor.
    #[must_use]
    pub const fn reviewer(&self) -> &ActorId {
        &self.reviewer
    }

    /// Returns the steward review decision.
    #[must_use]
    pub const fn decision(&self) -> WorkflowDraftStewardReviewDecision {
        self.decision
    }

    /// Returns whether this review authorizes future promotion.
    #[must_use]
    pub const fn authorization(&self) -> WorkflowDraftStewardReviewAuthorization {
        self.authorization
    }

    /// Returns the bounded approval reason.
    #[must_use]
    pub fn approval_reason(&self) -> &str {
        &self.approval_reason
    }

    /// Returns the non-mutation boundary.
    #[must_use]
    pub const fn boundary(&self) -> WorkflowDraftStewardReviewBoundary {
        self.boundary
    }
}

impl fmt::Debug for WorkflowDraftStewardReviewResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkflowDraftStewardReviewResult")
            .field("card", &self.card)
            .field("reviewer", &self.reviewer)
            .field("decision", &self.decision)
            .field("authorization", &self.authorization)
            .field("approval_reason", &"[REDACTED]")
            .field("boundary", &self.boundary)
            .finish()
    }
}

/// Reviews an inactive workflow draft for future promotion without mutating repository or runtime state.
///
/// # Errors
///
/// Returns a stable, non-leaking validation error if the supplied review context
/// is incomplete, stale, blocked by preflight, or unsafe.
pub fn review_workflow_draft_for_promotion(
    input: WorkflowDraftStewardReviewInput,
) -> Result<WorkflowDraftStewardReviewResult, WorkflowOsError> {
    validate_draft_path(&input.draft_path)?;
    if input.candidate_workflow_id.as_str().starts_with("draft/") {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.draft_namespace",
            "steward review requires a non-draft candidate workflow id",
        ));
    }
    if input.preflight_draft_content_hash != input.current_draft_content_hash {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.stale_preflight",
            "steward review requires a fresh preflight result for the current draft",
        ));
    }
    validate_code_list("preflight blocker", &input.preflight_blockers)?;
    validate_code_list("preflight warning", &input.preflight_warnings)?;
    if input.preflight_status == WorkflowDraftPromotionPreflightStatus::Blocked
        || !input.preflight_blockers.is_empty()
    {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.preflight_blocked",
            "steward review cannot approve a draft with preflight blockers",
        ));
    }
    if input.active_workflow_conflict {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.active_conflict",
            "steward review cannot approve a draft with active workflow conflicts",
        ));
    }
    validate_summary("owner summary", &input.owner_summary)?;
    validate_summary("escalation summary", &input.escalation_summary)?;
    validate_summary("policy summary", &input.policy_summary)?;
    validate_summary(
        "evidence and report summary",
        &input.evidence_report_summary,
    )?;
    validate_summary("side-effect summary", &input.side_effect_summary)?;
    validate_summary("approval reason", &input.approval_reason)?;

    let authorization =
        if input.decision == WorkflowDraftStewardReviewDecision::ApprovedForPromotion {
            WorkflowDraftStewardReviewAuthorization::AuthorizedForPromotion
        } else {
            WorkflowDraftStewardReviewAuthorization::NotAuthorized
        };
    let next_action =
        if authorization == WorkflowDraftStewardReviewAuthorization::AuthorizedForPromotion {
            "future_active_promotion_may_proceed_if_draft_unchanged"
        } else {
            "keep_draft_inactive_until_steward_decision_changes"
        };

    let card = WorkflowDraftStewardReviewCard {
        draft_path: input.draft_path,
        candidate_workflow_id: input.candidate_workflow_id,
        draft_content_hash: input.current_draft_content_hash,
        preflight_status: input.preflight_status,
        preflight_warnings: dedupe_codes(input.preflight_warnings),
        owner_summary: input.owner_summary,
        escalation_summary: input.escalation_summary,
        policy_summary: input.policy_summary,
        evidence_report_summary: input.evidence_report_summary,
        side_effect_summary: input.side_effect_summary,
        approval_allows: "future promotion of this exact unchanged draft through a separately implemented promotion step".to_owned(),
        approval_does_not_allow: "file movement, workflow registration, runtime execution, command execution, provider calls, side effects, writes, schemas, examples, hosted behavior, or approval of future draft changes".to_owned(),
        next_action: next_action.to_owned(),
    };

    Ok(WorkflowDraftStewardReviewResult {
        card,
        reviewer: input.reviewer,
        decision: input.decision,
        authorization,
        approval_reason: input.approval_reason,
        boundary: WorkflowDraftStewardReviewBoundary::prohibited(),
    })
}

fn validate_draft_path(value: &str) -> Result<(), WorkflowOsError> {
    validate_bounded_text("draft path", value, REVIEW_PATH_MAX_BYTES)?;
    if value.starts_with('/') || value.starts_with('\\') {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.draft_path",
            "steward review draft path must be relative and under workflows/drafts",
        ));
    }
    let path = Path::new(value);
    let has_parent_or_prefix = path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        )
    });
    if has_parent_or_prefix
        || !value.starts_with("workflows/drafts/")
        || !value.ends_with(".workflow.yml")
    {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.draft_path",
            "steward review draft path must be relative and under workflows/drafts",
        ));
    }
    Ok(())
}

fn validate_code_list(type_name: &'static str, values: &[String]) -> Result<(), WorkflowOsError> {
    if values.len() > REVIEW_CODE_MAX_COUNT {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.code.too_many",
            format!("steward review {type_name} list contains too many codes"),
        ));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        validate_code(type_name, value)?;
        if !seen.insert(value) {
            return Err(steward_review_error(
                "workflow_authoring.steward_review.code.duplicate",
                format!("steward review {type_name} list contains duplicate codes"),
            ));
        }
    }
    Ok(())
}

fn validate_code(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    validate_bounded_text(type_name, value, REVIEW_CODE_MAX_BYTES)?;
    let valid = value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':' | b'/')
    });
    if !valid {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.code.invalid",
            format!("steward review {type_name} contains invalid characters"),
        ));
    }
    Ok(())
}

fn validate_summary(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    validate_bounded_text(type_name, value, REVIEW_TEXT_MAX_BYTES)
}

fn validate_bounded_text(
    type_name: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.text.empty",
            format!("steward review {type_name} cannot be empty"),
        ));
    }
    if value.len() > max_bytes {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.text.too_long",
            format!("steward review {type_name} exceeds the bounded text limit"),
        ));
    }
    validate_not_secret_like(type_name, value)
}

fn validate_not_secret_like(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    let secret_like = lowercase.contains("authorization")
        || lowercase.contains("bearer")
        || lowercase.contains("private_key")
        || lowercase.contains("private-key")
        || lowercase.contains("api_token")
        || lowercase.contains("api-token")
        || lowercase.contains("secret")
        || lowercase.contains("token");

    if secret_like {
        return Err(steward_review_error(
            "workflow_authoring.steward_review.secret_like_value",
            format!("steward review {type_name} contains sensitive-looking text"),
        ));
    }
    Ok(())
}

fn dedupe_codes(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn steward_review_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(code, message)
}
