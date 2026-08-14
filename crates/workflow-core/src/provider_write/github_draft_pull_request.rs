use std::fmt;

use sha2::{Digest, Sha256};

use super::{
    github_write_error, validate_github_name, validate_not_secret_like,
    validate_redaction_metadata, GitHubPullRequestCommentProviderAuth,
};
use crate::{
    preflight_adapter_write, validate_approval_presentation_for_request,
    validate_side_effect_approval_linkage_from_store, Action, ActorId, AdapterId, AdapterKind,
    AdapterWriteCapability, AdapterWritePreflightRequest, AdapterWriteTargetKind,
    ApprovalDecisionKind, ApprovalDecisionProofEnforcementMode,
    ApprovalDecisionProofValidationPolicy, ApprovalPresentationRecord,
    ApprovalPresentationValidationInput, ApprovalRequest, Capability, CapabilityResolution,
    CapabilityResolutionPosture, CapabilityResourceKind, CorrelationId, EvidenceKind,
    EvidenceRedactionMetadata, EvidenceReference, EvidenceReferenceId,
    EvidenceReferenceRequiredFields, EvidenceReferenceTarget, EvidenceScope, EvidenceSensitivity,
    EvidenceSourceComponent, GovernanceAssessmentBinding, GovernanceAssessmentCompleteness,
    GovernanceDisclosureRequirement, GovernanceExecutionDisposition, IdempotencyKey, IntegrationId,
    RedactionMetadata, SideEffectAuthority, SideEffectAuthorityDecision, SideEffectCapability,
    SideEffectCompleteTransitionStoreInput, SideEffectFailTransitionStoreInput, SideEffectId,
    SideEffectIdempotencyBinding, SideEffectIdempotencyScope, SideEffectLifecycleState,
    SideEffectLifecycleTransitionResult, SideEffectMissingRecordPolicy, SideEffectOutcomeReference,
    SideEffectOutcomeReferenceKind, SideEffectRecord, SideEffectRecordDefinition,
    SideEffectRecordStore, SideEffectReference, SideEffectReferenceKind, SideEffectSensitivity,
    SideEffectTargetKind, SideEffectTargetReference, StepId, Timestamp, WorkReportArtifactRecord,
    WorkReportCitation, WorkReportCitationDefinition, WorkReportCitationTarget,
    WorkReportSensitivity, WorkflowOsError, WorkflowRun, WorkflowRunEventKind,
};

const BRANCH_MAX_BYTES: usize = 255;
const TITLE_MAX_BYTES: usize = 256;
const BODY_MAX_BYTES: usize = 16 * 1024;
const MARKER_MAX_BYTES: usize = 128;
const PROVIDER_REFERENCE_MAX_BYTES: usize = 256;
const PROVIDER_ERROR_CODE_MAX_BYTES: usize = 128;
const EXACT_CAPABILITY: &str = "github.pull_request.create";

macro_rules! draft_pr_error {
    ($suffix:literal, $message:expr $(,)?) => {
        github_write_error(concat!("github_draft_pull_request.", $suffix), $message)
    };
}

/// Exact capability reference required for draft GitHub pull request creation.
pub const GITHUB_DRAFT_PULL_REQUEST_CREATE_CAPABILITY: &str = EXACT_CAPABILITY;

/// Bounded target for one draft GitHub pull request creation attempt.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubDraftPullRequestTarget {
    owner: String,
    repository: String,
    head_owner: String,
    head_branch: String,
    expected_head_sha: String,
    base_branch: String,
    observed_base_sha: String,
}

impl GitHubDraftPullRequestTarget {
    /// Creates a validated, draft-only target.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when provider identity, branch, or
    /// commit observations are invalid.
    pub fn new(
        owner: impl Into<String>,
        repository: impl Into<String>,
        head_owner: impl Into<String>,
        head_branch: impl Into<String>,
        expected_head_sha: impl Into<String>,
        base_branch: impl Into<String>,
        observed_base_sha: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let target = Self {
            owner: owner.into(),
            repository: repository.into(),
            head_owner: head_owner.into(),
            head_branch: head_branch.into(),
            expected_head_sha: expected_head_sha.into(),
            base_branch: base_branch.into(),
            observed_base_sha: observed_base_sha.into(),
        };
        target.validate()?;
        Ok(target)
    }

    /// Validates the target without exposing its values.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when any target component is invalid.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_github_name("owner", &self.owner)?;
        validate_github_name("repository", &self.repository)?;
        validate_github_name("head owner", &self.head_owner)?;
        validate_branch("head branch", &self.head_branch)?;
        validate_branch("base branch", &self.base_branch)?;
        validate_commit_sha(&self.expected_head_sha)?;
        validate_commit_sha(&self.observed_base_sha)
    }

    /// Returns the repository owner for the injected provider only.
    #[must_use]
    pub fn owner_for_provider(&self) -> &str {
        &self.owner
    }

    /// Returns the repository name for the injected provider only.
    #[must_use]
    pub fn repository_for_provider(&self) -> &str {
        &self.repository
    }

    /// Returns the head owner for the injected provider only.
    #[must_use]
    pub fn head_owner_for_provider(&self) -> &str {
        &self.head_owner
    }

    /// Returns the head branch for the injected provider only.
    #[must_use]
    pub fn head_branch_for_provider(&self) -> &str {
        &self.head_branch
    }

    /// Returns the expected head SHA observation.
    #[must_use]
    pub fn expected_head_sha(&self) -> &str {
        &self.expected_head_sha
    }

    /// Returns the base branch for the injected provider only.
    #[must_use]
    pub fn base_branch_for_provider(&self) -> &str {
        &self.base_branch
    }

    /// Returns the approved base SHA observation.
    #[must_use]
    pub fn observed_base_sha(&self) -> &str {
        &self.observed_base_sha
    }

    /// Returns an exact repository authority scope reference.
    #[must_use]
    pub fn repository_reference(&self) -> String {
        format!("github/{}/{}", self.owner, self.repository)
    }

    /// Returns the distinct provider operation reference used by preflight and `SideEffect` state.
    #[must_use]
    pub fn operation_reference(&self) -> String {
        format!(
            "github/{}/{}/draft-pull-request/{}/{}",
            self.owner, self.repository, self.head_branch, self.base_branch
        )
    }
}

impl fmt::Debug for GitHubDraftPullRequestTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestTarget")
            .field("owner", &"[REDACTED]")
            .field("repository", &"[REDACTED]")
            .field("head_owner", &"[REDACTED]")
            .field("head_branch", &"[REDACTED]")
            .field("expected_head_sha", &"[REDACTED]")
            .field("base_branch", &"[REDACTED]")
            .field("observed_base_sha", &"[REDACTED]")
            .field("draft", &true)
            .finish()
    }
}

/// Versioned bounded content passed only to the injected GitHub provider.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubDraftPullRequestContent {
    template_version: String,
    title: String,
    body: String,
    managed_marker: String,
    commitment: String,
}

impl GitHubDraftPullRequestContent {
    /// Creates bounded, non-secret draft pull request content and its SHA-256 commitment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when content is invalid, unbounded, or secret-like.
    pub fn new(
        template_version: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        managed_marker: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let template_version = template_version.into();
        let title = title.into();
        let body = body.into();
        let managed_marker = managed_marker.into();
        validate_text("template version", &template_version, MARKER_MAX_BYTES)?;
        validate_text("title", &title, TITLE_MAX_BYTES)?;
        validate_text("body", &body, BODY_MAX_BYTES)?;
        validate_text("managed marker", &managed_marker, MARKER_MAX_BYTES)?;
        let commitment = content_commitment(&template_version, &title, &body, &managed_marker);
        Ok(Self {
            template_version,
            title,
            body,
            managed_marker,
            commitment,
        })
    }

    /// Returns the template version.
    #[must_use]
    pub fn template_version(&self) -> &str {
        &self.template_version
    }

    /// Returns the rendered title only to the injected provider.
    #[must_use]
    pub fn title_for_provider(&self) -> &str {
        &self.title
    }

    /// Returns the rendered body only to the injected provider.
    #[must_use]
    pub fn body_for_provider(&self) -> &str {
        &self.body
    }

    /// Returns the bounded managed marker used for reconciliation.
    #[must_use]
    pub fn managed_marker_for_provider(&self) -> &str {
        &self.managed_marker
    }

    /// Returns the payload-free content commitment.
    #[must_use]
    pub fn commitment(&self) -> &str {
        &self.commitment
    }
}

impl fmt::Debug for GitHubDraftPullRequestContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestContent")
            .field("template_version", &"[REDACTED]")
            .field("title", &"[REDACTED]")
            .field("body", &"[REDACTED]")
            .field("managed_marker", &"[REDACTED]")
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

/// Bounded pre/post provider observation of the mutable branch refs.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubDraftPullRequestRefObservation {
    head_sha: String,
    base_sha: String,
}

impl GitHubDraftPullRequestRefObservation {
    /// Creates a validated bounded head/base ref observation.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when either commit SHA is invalid.
    /// Creates a validated branch observation.
    pub fn new(
        head_sha: impl Into<String>,
        base_sha: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let observation = Self {
            head_sha: head_sha.into(),
            base_sha: base_sha.into(),
        };
        validate_commit_sha(&observation.head_sha)?;
        validate_commit_sha(&observation.base_sha)?;
        Ok(observation)
    }

    /// Returns the observed head SHA.
    #[must_use]
    pub fn head_sha(&self) -> &str {
        &self.head_sha
    }

    /// Returns the observed base SHA.
    #[must_use]
    pub fn base_sha(&self) -> &str {
        &self.base_sha
    }
}

impl fmt::Debug for GitHubDraftPullRequestRefObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestRefObservation")
            .field("head_sha", &"[REDACTED]")
            .field("base_sha", &"[REDACTED]")
            .finish()
    }
}

/// Bounded provider identity for one existing or newly-created pull request.
#[derive(Clone, Eq, PartialEq)]
pub struct GitHubDraftPullRequestObservation {
    provider_reference: String,
    draft: bool,
    refs: GitHubDraftPullRequestRefObservation,
    managed_marker_matches: bool,
}

impl GitHubDraftPullRequestObservation {
    /// Creates one bounded provider observation.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when provider identity or draft posture is invalid.
    /// Creates a validated provider observation.
    pub fn new(
        provider_reference: impl Into<String>,
        draft: bool,
        refs: GitHubDraftPullRequestRefObservation,
        managed_marker_matches: bool,
    ) -> Result<Self, WorkflowOsError> {
        let observation = Self {
            provider_reference: provider_reference.into(),
            draft,
            refs,
            managed_marker_matches,
        };
        validate_text(
            "provider reference",
            &observation.provider_reference,
            PROVIDER_REFERENCE_MAX_BYTES,
        )?;
        Ok(observation)
    }

    /// Returns the stable provider reference.
    #[must_use]
    pub fn provider_reference(&self) -> &str {
        &self.provider_reference
    }

    /// Returns whether the provider object remains a draft.
    #[must_use]
    pub const fn draft(&self) -> bool {
        self.draft
    }

    /// Returns the observed branch refs.
    #[must_use]
    pub const fn refs(&self) -> &GitHubDraftPullRequestRefObservation {
        &self.refs
    }

    /// Returns whether the managed marker matched.
    #[must_use]
    pub const fn managed_marker_matches(&self) -> bool {
        self.managed_marker_matches
    }
}

impl fmt::Debug for GitHubDraftPullRequestObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestObservation")
            .field("provider_reference", &"[REDACTED]")
            .field("draft", &self.draft)
            .field("refs", &self.refs)
            .field("managed_marker_matches", &self.managed_marker_matches)
            .finish()
    }
}

/// Bounded lookup result from the injected provider.
#[derive(Clone, Eq, PartialEq)]
pub enum GitHubDraftPullRequestLookupResult {
    /// No pull request matched the exact repository/head/base identity.
    NotFound,
    /// One managed draft matched.
    ExactManaged(GitHubDraftPullRequestObservation),
    /// A non-managed or non-draft pull request conflicts with the identity.
    Conflict,
    /// The provider state could not be reduced to one safe answer.
    Ambiguous,
}

impl fmt::Debug for GitHubDraftPullRequestLookupResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => formatter.write_str("NotFound"),
            Self::ExactManaged(observation) => formatter
                .debug_tuple("ExactManaged")
                .field(observation)
                .finish(),
            Self::Conflict => formatter.write_str("Conflict"),
            Self::Ambiguous => formatter.write_str("Ambiguous"),
        }
    }
}

/// Classified response from exactly one create call.
#[derive(Clone, Eq, PartialEq)]
pub enum GitHubDraftPullRequestCreateOutcome {
    /// GitHub returned a bounded created-draft observation.
    Created(GitHubDraftPullRequestObservation),
    /// GitHub definitively rejected the request without creating a draft.
    KnownRejected {
        /// Bounded provider classification code; Debug output always redacts it.
        code: String,
    },
    /// The call may have created a draft but no safe conclusion is available.
    Ambiguous {
        /// Bounded provider classification code; Debug output always redacts it.
        code: String,
    },
}

impl GitHubDraftPullRequestCreateOutcome {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        match self {
            Self::Created(observation) => {
                if !observation.draft() || !observation.managed_marker_matches() {
                    return Err(draft_pr_error!(
                        "provider.created_observation_invalid",
                        "GitHub draft pull request created observation is invalid",
                    ));
                }
                Ok(())
            }
            Self::KnownRejected { code } | Self::Ambiguous { code } => {
                validate_text("provider error code", code, PROVIDER_ERROR_CODE_MAX_BYTES)
            }
        }
    }
}

impl fmt::Debug for GitHubDraftPullRequestCreateOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created(observation) => {
                formatter.debug_tuple("Created").field(observation).finish()
            }
            Self::KnownRejected { .. } => formatter.write_str("KnownRejected([REDACTED])"),
            Self::Ambiguous { .. } => formatter.write_str("Ambiguous([REDACTED])"),
        }
    }
}

/// Non-serializable request passed to the injected provider.
pub struct GitHubDraftPullRequestProviderRequest {
    target: GitHubDraftPullRequestTarget,
    content: GitHubDraftPullRequestContent,
    idempotency_key: IdempotencyKey,
    auth: GitHubPullRequestCommentProviderAuth,
}

impl GitHubDraftPullRequestProviderRequest {
    /// Returns the target for the injected provider.
    #[must_use]
    pub const fn target(&self) -> &GitHubDraftPullRequestTarget {
        &self.target
    }

    /// Returns the bounded content for the injected provider.
    #[must_use]
    pub const fn content(&self) -> &GitHubDraftPullRequestContent {
        &self.content
    }

    /// Returns the idempotency key for provider correlation, not provider retry.
    #[must_use]
    pub const fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    /// Returns explicit caller-supplied auth only to the injected provider.
    #[must_use]
    pub const fn auth(&self) -> &GitHubPullRequestCommentProviderAuth {
        &self.auth
    }
}

impl fmt::Debug for GitHubDraftPullRequestProviderRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestProviderRequest")
            .field("target", &self.target)
            .field("content", &self.content)
            .field("idempotency_key", &"[REDACTED]")
            .field("auth", &self.auth)
            .finish()
    }
}

/// Injected provider boundary for one draft pull request mutation family.
pub trait GitHubDraftPullRequestProvider {
    /// Observes the current mutable head and base refs.
    ///
    /// # Errors
    ///
    /// Returns a provider-specific bounded error when observation fails.
    fn observe_refs(
        &self,
        request: &GitHubDraftPullRequestProviderRequest,
    ) -> Result<GitHubDraftPullRequestRefObservation, WorkflowOsError>;

    /// Looks up an existing pull request by exact governed identity and marker.
    ///
    /// # Errors
    ///
    /// Returns a provider-specific bounded error when lookup fails.
    fn lookup(
        &self,
        request: &GitHubDraftPullRequestProviderRequest,
    ) -> Result<GitHubDraftPullRequestLookupResult, WorkflowOsError>;

    /// Performs exactly one create call. Callers must never automatically retry it.
    ///
    /// # Errors
    ///
    /// Returns a provider-specific bounded error when the create call fails.
    fn create(
        &self,
        request: &GitHubDraftPullRequestProviderRequest,
    ) -> Result<GitHubDraftPullRequestCreateOutcome, WorkflowOsError>;
}

/// Input for the explicit local sandbox draft pull request mutation helper.
pub struct GitHubDraftPullRequestMutationInput<'a> {
    /// Coherent terminal run whose durable events are the governance source of truth.
    pub run: &'a WorkflowRun,
    /// Explicit preflight request using the draft-only sandbox readiness policy.
    pub preflight: AdapterWritePreflightRequest,
    /// Fresh exact capability resolution.
    pub capability_resolution: &'a CapabilityResolution,
    /// Accepted authoritative proportional-governance binding from the terminal run.
    pub governance_assessment: &'a GovernanceAssessmentBinding,
    /// Fresh current-runtime-fact reassessment for provider-use time.
    pub current_governance_assessment: &'a GovernanceAssessmentBinding,
    /// Approval request from the run event trail.
    pub approval_request: &'a ApprovalRequest,
    /// Proof that the exact approval scope was presented.
    pub approval_presentation: &'a ApprovalPresentationRecord,
    /// Actor making the provider mutation.
    pub actor: ActorId,
    /// Exact workflow step boundary.
    pub step_id: StepId,
    /// Adapter identity.
    pub adapter_id: AdapterId,
    /// Integration identity.
    pub integration_id: IntegrationId,
    /// Validated terminal `WorkReport` artifact supporting the provider content.
    pub work_report_artifact: &'a WorkReportArtifactRecord,
    /// Target repository and branch observations.
    pub target: GitHubDraftPullRequestTarget,
    /// Versioned bounded content.
    pub content: GitHubDraftPullRequestContent,
    /// Explicit caller-supplied auth.
    pub auth: GitHubPullRequestCommentProviderAuth,
    /// Proposed-record timestamp.
    pub proposed_at: Timestamp,
    /// Attempt and authority-evaluation timestamp.
    pub attempted_at: Timestamp,
    /// Outcome timestamp.
    pub outcome_at: Timestamp,
    /// Optional correlation identity.
    pub correlation_id: Option<CorrelationId>,
    /// Sensitivity.
    pub sensitivity: SideEffectSensitivity,
    /// Redaction metadata.
    pub redaction: RedactionMetadata,
}

impl fmt::Debug for GitHubDraftPullRequestMutationInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestMutationInput")
            .field("run", &"[REDACTED]")
            .field("preflight", &self.preflight)
            .field("capability_resolution", &self.capability_resolution)
            .field("governance_assessment", &self.governance_assessment)
            .field(
                "current_governance_assessment",
                &self.current_governance_assessment,
            )
            .field("approval_request", &"[REDACTED]")
            .field("approval_presentation", &self.approval_presentation)
            .field("actor", &"[REDACTED]")
            .field("step_id", &"[REDACTED]")
            .field("adapter_id", &self.adapter_id)
            .field("integration_id", &self.integration_id)
            .field("work_report_artifact", &"[REDACTED]")
            .field("target", &self.target)
            .field("content", &self.content)
            .field("auth", &self.auth)
            .field("proposed_at", &self.proposed_at)
            .field("attempted_at", &self.attempted_at)
            .field("outcome_at", &self.outcome_at)
            .field(
                "correlation_id",
                &self.correlation_id.as_ref().map(|_| "[REDACTED]"),
            )
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &"[REDACTED]")
            .finish()
    }
}

/// Reconciliation posture for the integrated mutation helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GitHubDraftPullRequestMutationStatus {
    /// One new draft was created and reconciled.
    Created,
    /// An exact existing managed draft was reused without a create call.
    ExistingManaged,
    /// Existing managed draft was found but refs have moved since approval.
    ExistingManagedWithRefDrift,
    /// A conflicting pull request prevents creation.
    Conflict,
    /// Provider state is ambiguous and automatic retry is blocked.
    Ambiguous,
    /// The provider definitively rejected creation.
    KnownRejected,
    /// Refs moved during the unavoidable provider-create interval.
    ConcurrentRefChange,
}

/// Bounded report disclosure for a draft pull request provider mutation.
#[derive(Clone, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct GitHubDraftPullRequestDisclosure {
    status: GitHubDraftPullRequestMutationStatus,
    lookup_performed: bool,
    create_attempted: bool,
    post_create_observation_performed: bool,
    retry_blocked: bool,
    operator_action_required: bool,
}

impl GitHubDraftPullRequestDisclosure {
    /// Returns the reconciliation status.
    #[must_use]
    pub const fn status(&self) -> GitHubDraftPullRequestMutationStatus {
        self.status
    }

    /// Returns whether provider lookup occurred.
    #[must_use]
    pub const fn lookup_performed(&self) -> bool {
        self.lookup_performed
    }

    /// Returns whether one create call was attempted.
    #[must_use]
    pub const fn create_attempted(&self) -> bool {
        self.create_attempted
    }

    /// Returns whether post-create ref observation occurred.
    #[must_use]
    pub const fn post_create_observation_performed(&self) -> bool {
        self.post_create_observation_performed
    }

    /// Returns whether automatic retry is blocked.
    #[must_use]
    pub const fn retry_blocked(&self) -> bool {
        self.retry_blocked
    }

    /// Returns whether operator reconciliation is required.
    #[must_use]
    pub const fn operator_action_required(&self) -> bool {
        self.operator_action_required
    }
}

impl fmt::Debug for GitHubDraftPullRequestDisclosure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestDisclosure")
            .field("status", &self.status)
            .field("lookup_performed", &self.lookup_performed)
            .field("create_attempted", &self.create_attempted)
            .field(
                "post_create_observation_performed",
                &self.post_create_observation_performed,
            )
            .field("retry_blocked", &self.retry_blocked)
            .field("operator_action_required", &self.operator_action_required)
            .finish()
    }
}

/// Integrated in-memory closure result.
pub struct GitHubDraftPullRequestMutationResult {
    disclosure: GitHubDraftPullRequestDisclosure,
    proposed_record: SideEffectRecord,
    attempted_transition: Option<SideEffectLifecycleTransitionResult>,
    outcome_transition: Option<SideEffectLifecycleTransitionResult>,
    evidence: Option<EvidenceReference>,
    report_citations: Vec<WorkReportCitation>,
}

impl GitHubDraftPullRequestMutationResult {
    /// Returns the bounded disclosure.
    #[must_use]
    pub const fn disclosure(&self) -> &GitHubDraftPullRequestDisclosure {
        &self.disclosure
    }

    /// Returns the originally persisted proposed record.
    #[must_use]
    pub const fn proposed_record(&self) -> &SideEffectRecord {
        &self.proposed_record
    }

    /// Returns the attempted lifecycle transition when provider mutation was possible.
    #[must_use]
    pub const fn attempted_transition(&self) -> Option<&SideEffectLifecycleTransitionResult> {
        self.attempted_transition.as_ref()
    }

    /// Returns a terminal lifecycle transition only for known outcomes.
    #[must_use]
    pub const fn outcome_transition(&self) -> Option<&SideEffectLifecycleTransitionResult> {
        self.outcome_transition.as_ref()
    }

    /// Returns bounded provider evidence only for reconciled completion.
    #[must_use]
    pub const fn evidence(&self) -> Option<&EvidenceReference> {
        self.evidence.as_ref()
    }

    /// Returns report-ready citations without populating or persisting a `WorkReport`.
    #[must_use]
    pub fn report_citations(&self) -> &[WorkReportCitation] {
        &self.report_citations
    }
}

impl fmt::Debug for GitHubDraftPullRequestMutationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubDraftPullRequestMutationResult")
            .field("disclosure", &self.disclosure)
            .field("proposed_record", &"[REDACTED]")
            .field("attempted", &self.attempted_transition.is_some())
            .field("terminal_outcome", &self.outcome_transition.is_some())
            .field("evidence", &self.evidence.is_some())
            .field("report_citation_count", &self.report_citations.len())
            .finish()
    }
}

/// Executes one explicit, local, sandbox-bound draft pull request provider mutation.
///
/// The helper does not append workflow events, write report artifacts, expose
/// CLI behavior, execute Git transport, or retry provider creation. It returns
/// validated event payloads through the lifecycle transition results and
/// report-ready evidence/citations for explicit later persistence.
///
/// # Errors
///
/// Returns a stable non-leaking error when any governance gate, provider
/// observation, `SideEffect` transition, evidence construction, or reconciliation
/// invariant fails.
// The explicit gate/provider/reconciliation sequence is intentionally kept in
// one reviewable orchestration boundary.
#[allow(clippy::too_many_lines)]
pub fn execute_github_draft_pull_request_mutation(
    store: &impl SideEffectRecordStore,
    provider: &impl GitHubDraftPullRequestProvider,
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<GitHubDraftPullRequestMutationResult, WorkflowOsError> {
    validate_mutation_input(input)?;
    let preflight = preflight_adapter_write(&input.preflight).map_err(|_| {
        draft_pr_error!(
            "preflight.denied",
            "GitHub draft pull request preflight was not satisfied",
        )
    })?;
    let expected_side_effect_id = input.preflight.side_effect_id().ok_or_else(|| {
        draft_pr_error!(
            "preflight.mismatch",
            "GitHub draft pull request preflight does not match mutation input",
        )
    })?;
    let expected_idempotency_key = input.preflight.idempotency_key().ok_or_else(|| {
        draft_pr_error!(
            "preflight.mismatch",
            "GitHub draft pull request preflight does not match mutation input",
        )
    })?;
    if preflight.capability() != AdapterWriteCapability::GitHubPullRequestCreate
        || preflight.side_effect_id() != expected_side_effect_id
        || preflight.idempotency_key() != expected_idempotency_key
    {
        return Err(draft_pr_error!(
            "preflight.mismatch",
            "GitHub draft pull request preflight does not match mutation input",
        ));
    }

    let proposed_record = compose_proposed_record(input)?;
    store
        .write_side_effect_record(&proposed_record)
        .map_err(|_| {
            draft_pr_error!(
                "side_effect.proposed_persistence_failed",
                "GitHub draft pull request proposed SideEffect could not be persisted",
            )
        })?;

    let side_effect_ids = [proposed_record.side_effect_id().clone()];
    validate_side_effect_approval_linkage_from_store(
        store,
        crate::SideEffectApprovalLinkageFromStoreInput {
            run: input.run,
            side_effect_ids: &side_effect_ids,
            load_mode: crate::SideEffectApprovalLinkageStoreLoadMode::ExplicitIds,
            missing_record_policy: SideEffectMissingRecordPolicy::RequireAll,
            require_approval_references_for_requires_approval: true,
            require_decision_for_approved_or_denied: true,
        },
    )
    .map_err(|_| {
        draft_pr_error!(
            "approval.linkage_invalid",
            "GitHub draft pull request approval linkage is invalid",
        )
    })?;

    let provider_request = GitHubDraftPullRequestProviderRequest {
        target: input.target.clone(),
        content: input.content.clone(),
        idempotency_key: preflight.idempotency_key().clone(),
        auth: input.auth.clone(),
    };
    let pre_refs = provider.observe_refs(&provider_request).map_err(|_| {
        draft_pr_error!(
            "provider.pre_observation_failed",
            "GitHub draft pull request pre-create observation failed",
        )
    })?;
    if !refs_match_target(&pre_refs, &input.target) {
        return Err(draft_pr_error!(
            "provider.pre_observation_drift",
            "GitHub draft pull request refs differ from the approved observations",
        ));
    }

    let lookup = provider.lookup(&provider_request).map_err(|_| {
        draft_pr_error!(
            "provider.lookup_failed",
            "GitHub draft pull request lookup failed",
        )
    })?;
    match lookup {
        GitHubDraftPullRequestLookupResult::Conflict => {
            return Ok(non_attempted_result(
                GitHubDraftPullRequestMutationStatus::Conflict,
                proposed_record,
            ));
        }
        GitHubDraftPullRequestLookupResult::Ambiguous => {
            return Ok(non_attempted_result(
                GitHubDraftPullRequestMutationStatus::Ambiguous,
                proposed_record,
            ));
        }
        GitHubDraftPullRequestLookupResult::ExactManaged(observation) => {
            validate_managed_observation(&observation)?;
            let attempted = transition_attempted(store, proposed_record.side_effect_id(), input)?;
            if !refs_match_target(observation.refs(), &input.target) {
                return Ok(result_without_terminal(
                    GitHubDraftPullRequestMutationStatus::ExistingManagedWithRefDrift,
                    proposed_record,
                    attempted,
                    false,
                    false,
                ));
            }
            return complete_result(
                store,
                input,
                proposed_record,
                attempted,
                &observation,
                GitHubDraftPullRequestMutationStatus::ExistingManaged,
                false,
            );
        }
        GitHubDraftPullRequestLookupResult::NotFound => {}
    }

    let attempted = transition_attempted(store, proposed_record.side_effect_id(), input)?;
    let Ok(create_outcome) = provider.create(&provider_request) else {
        return Ok(result_without_terminal(
            GitHubDraftPullRequestMutationStatus::Ambiguous,
            proposed_record,
            attempted,
            true,
            false,
        ));
    };
    create_outcome.validate()?;
    match create_outcome {
        GitHubDraftPullRequestCreateOutcome::KnownRejected { code } => {
            let failed = crate::transition_side_effect_to_failed_in_store(
                store,
                SideEffectFailTransitionStoreInput {
                    side_effect_id: proposed_record.side_effect_id(),
                    transitioned_at: input.outcome_at,
                    outcome_reference: None,
                    reason_codes: vec![code],
                    summary: Some("GitHub draft pull request creation was rejected".to_owned()),
                    additional_references: Vec::new(),
                    evidence_reference_count: 0,
                },
            )?;
            Ok(GitHubDraftPullRequestMutationResult {
                disclosure: disclosure(
                    GitHubDraftPullRequestMutationStatus::KnownRejected,
                    true,
                    true,
                    false,
                ),
                proposed_record,
                attempted_transition: Some(attempted),
                outcome_transition: Some(failed),
                evidence: None,
                report_citations: Vec::new(),
            })
        }
        GitHubDraftPullRequestCreateOutcome::Ambiguous { .. } => Ok(result_without_terminal(
            GitHubDraftPullRequestMutationStatus::Ambiguous,
            proposed_record,
            attempted,
            true,
            false,
        )),
        GitHubDraftPullRequestCreateOutcome::Created(observation) => {
            validate_managed_observation(&observation)?;
            let Ok(post_refs) = provider.observe_refs(&provider_request) else {
                return Ok(result_without_terminal(
                    GitHubDraftPullRequestMutationStatus::Ambiguous,
                    proposed_record,
                    attempted,
                    true,
                    false,
                ));
            };
            if !refs_match_target(&post_refs, &input.target) || post_refs != *observation.refs() {
                return Ok(result_without_terminal(
                    GitHubDraftPullRequestMutationStatus::ConcurrentRefChange,
                    proposed_record,
                    attempted,
                    true,
                    true,
                ));
            }
            complete_result(
                store,
                input,
                proposed_record,
                attempted,
                &observation,
                GitHubDraftPullRequestMutationStatus::Created,
                true,
            )
        }
    }
}

fn validate_mutation_input(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<(), WorkflowOsError> {
    let rehydrated = WorkflowRun::rehydrate(&input.run.events)
        .map_err(|_| draft_pr_error!("run.invalid", "GitHub draft pull request run is invalid"))?;
    if rehydrated.snapshot != input.run.snapshot || !input.run.snapshot.status.is_terminal() {
        return Err(draft_pr_error!(
            "run.not_trusted_terminal",
            "GitHub draft pull request requires a coherent terminal run",
        ));
    }
    if input.run.snapshot.identity.immutable_run_bundle.is_none() {
        return Err(draft_pr_error!(
            "run.immutable_bundle_missing",
            "GitHub draft pull request requires an immutable run bundle",
        ));
    }
    validate_report_artifact(input)?;
    input.target.validate()?;
    validate_redaction_metadata(&input.redaction)?;
    if input.preflight.capability() != AdapterWriteCapability::GitHubPullRequestCreate
        || input.preflight.target().kind() != AdapterWriteTargetKind::GitHubRepository
        || input.preflight.target().reference() != input.target.operation_reference()
    {
        return Err(draft_pr_error!(
            "preflight.target_mismatch",
            "GitHub draft pull request target does not match preflight",
        ));
    }
    validate_policy_event_references(input)?;
    validate_governance_assessment(input)?;
    validate_exact_capability_resolution(input)?;
    validate_approval_presentation_for_request(ApprovalPresentationValidationInput {
        presentation: input.approval_presentation,
        approval_request: input.approval_request,
    })
    .map_err(|_| {
        draft_pr_error!(
            "approval.presentation_invalid",
            "GitHub draft pull request approval presentation is invalid",
        )
    })?;
    if !input.run.events.iter().any(|event| {
        matches!(
            &event.kind,
            WorkflowRunEventKind::ApprovalRequested(request)
                if request.as_ref() == input.approval_request
        )
    }) {
        return Err(draft_pr_error!(
            "approval.request_not_durable",
            "GitHub draft pull request approval request is not in the durable run",
        ));
    }
    validate_approval_decision_proof(input)?;
    let expected_action = format!(
        "create draft github pull request content {}",
        input.content.commitment()
    );
    if input.approval_presentation.requested_action() != expected_action {
        return Err(draft_pr_error!(
            "approval.content_commitment_mismatch",
            "GitHub draft pull request content commitment was not presented for approval",
        ));
    }
    Ok(())
}

fn validate_report_artifact(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<(), WorkflowOsError> {
    input.work_report_artifact.validate().map_err(|_| {
        draft_pr_error!(
            "report.artifact_invalid",
            "GitHub draft pull request requires a valid terminal WorkReport artifact",
        )
    })?;
    let metadata = input.work_report_artifact.metadata();
    let identity = &input.run.snapshot.identity;
    if metadata.workflow_id() != &identity.workflow_id
        || metadata.workflow_version() != &identity.workflow_version
        || metadata.schema_version() != &identity.schema_version
        || metadata.spec_hash() != &identity.spec_content_hash
        || metadata.run_id() != &identity.run_id
    {
        return Err(draft_pr_error!(
            "report.identity_mismatch",
            "GitHub draft pull request WorkReport artifact does not match the terminal run",
        ));
    }
    Ok(())
}

fn validate_approval_decision_proof(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<(), WorkflowOsError> {
    let decision = input.run.events.iter().find_map(|event| match &event.kind {
        WorkflowRunEventKind::ApprovalGranted(decision)
            if decision.approval_id == input.approval_request.approval_id =>
        {
            Some(decision)
        }
        _ => None,
    });
    let Some(decision) = decision else {
        return Err(draft_pr_error!(
            "approval.granted_decision_missing",
            "GitHub draft pull request requires a durable granted approval decision",
        ));
    };
    let Some(marker) = decision.proof_marker.as_ref() else {
        return Err(draft_pr_error!(
            "approval.proof_marker_missing",
            "GitHub draft pull request requires proof-enforced approval",
        ));
    };
    if decision.decision != ApprovalDecisionKind::Granted
        || marker.enforcement_mode()
            != ApprovalDecisionProofEnforcementMode::ApprovalPresentationRequired
        || marker.proof_validation_policy()
            != ApprovalDecisionProofValidationPolicy::ApprovalPresentationRequestMatch
        || marker.presentation_id() != input.approval_presentation.presentation_id()
        || marker.presentation_content_hash() != input.approval_presentation.content_hash()
        || marker.proof_validated_at() != decision.decided_at
    {
        return Err(draft_pr_error!(
            "approval.proof_marker_mismatch",
            "GitHub draft pull request approval proof does not match the presented scope",
        ));
    }
    Ok(())
}

fn validate_governance_assessment(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<(), WorkflowOsError> {
    let Some(durable) = input.run.snapshot.governance_assessment_binding.as_ref() else {
        return Err(draft_pr_error!(
            "governance.assessment_missing",
            "GitHub draft pull request requires a durable proportional-governance assessment",
        ));
    };
    let Some(immutable_run_bundle) = input.run.snapshot.identity.immutable_run_bundle.as_ref()
    else {
        return Err(draft_pr_error!(
            "run.immutable_bundle_missing",
            "GitHub draft pull request requires an immutable run bundle",
        ));
    };
    if durable != input.governance_assessment
        || input.governance_assessment.workflow_id() != &input.run.snapshot.identity.workflow_id
        || input.governance_assessment.run_id() != &input.run.snapshot.identity.run_id
        || input.governance_assessment.immutable_run_bundle() != immutable_run_bundle
        || input.governance_assessment.completeness() != GovernanceAssessmentCompleteness::Complete
        || input.governance_assessment.execution()
            != GovernanceExecutionDisposition::RequireApproval
        || input.governance_assessment.disclosure() != GovernanceDisclosureRequirement::Visible
        || !input
            .governance_assessment
            .has_authoritative_fact_commitment()
    {
        return Err(draft_pr_error!(
            "governance.assessment_mismatch",
            "GitHub draft pull request proportional-governance assessment is not accepted",
        ));
    }
    input
        .governance_assessment
        .validate_current_runtime_fact_binding(input.current_governance_assessment)
        .map_err(|_| {
            draft_pr_error!(
                "governance.reassessment_mismatch",
                "GitHub draft pull request requires a matching current runtime-fact reassessment",
            )
        })?;
    if input
        .current_governance_assessment
        .runtime_fact_snapshot_binding()
        .map_or(true, |binding| binding.evaluated_at() != input.attempted_at)
    {
        return Err(draft_pr_error!(
            "governance.reassessment_stale",
            "GitHub draft pull request current runtime-fact reassessment is stale",
        ));
    }
    Ok(())
}

fn validate_policy_event_references(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<(), WorkflowOsError> {
    if input.preflight.policy_references().is_empty() {
        return Err(draft_pr_error!(
            "policy.reference_missing",
            "GitHub draft pull request requires a durable policy decision reference",
        ));
    }
    for reference in input.preflight.policy_references() {
        if reference.kind() != SideEffectReferenceKind::PolicyDecision {
            return Err(draft_pr_error!(
                "policy.reference_invalid",
                "GitHub draft pull request policy reference is invalid",
            ));
        }
        let decision = input.run.events.iter().find_map(|event| {
            if event.event_id.as_str() == reference.reference() {
                match &event.kind {
                    WorkflowRunEventKind::PolicyDecisionRecorded(decision) => Some(decision),
                    _ => None,
                }
            } else {
                None
            }
        });
        let Some(decision) = decision else {
            return Err(draft_pr_error!(
                "policy.reference_not_durable",
                "GitHub draft pull request policy decision is not in the durable run",
            ));
        };
        if !decision.allowed
            || !decision.requires_approval
            || decision.action != Action::InvokeAdapter
            || !decision.capabilities.contains(&Capability::ExternalWrite)
            || !decision.capabilities.contains(&Capability::AdapterInvoke)
            || decision
                .capabilities
                .iter()
                .any(|capability| matches!(capability, Capability::Unknown(_)))
            || decision.workflow_id.as_ref() != Some(&input.run.snapshot.identity.workflow_id)
            || decision.run_id.as_ref() != Some(&input.run.snapshot.identity.run_id)
            || decision.actor.as_ref() != Some(&input.actor)
        {
            return Err(draft_pr_error!(
                "policy.decision_mismatch",
                "GitHub draft pull request policy decision does not authorize evaluation",
            ));
        }
    }
    Ok(())
}

fn validate_exact_capability_resolution(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<(), WorkflowOsError> {
    input.capability_resolution.validate().map_err(|_| {
        draft_pr_error!(
            "authority.resolution_invalid",
            "GitHub draft pull request capability resolution is invalid",
        )
    })?;
    let context = input.capability_resolution.context();
    if input.capability_resolution.posture() != CapabilityResolutionPosture::Authorized
        || context.capability().as_str() != EXACT_CAPABILITY
        || context.resource().kind() != CapabilityResourceKind::Repository
        || context.resource().reference() != input.target.repository_reference()
        || context.actor() != &input.actor
        || context.workflow_id() != &input.run.snapshot.identity.workflow_id
        || context.run_id() != &input.run.snapshot.identity.run_id
        || context.step_id() != &input.step_id
        || input.capability_resolution.evaluated_at() != input.attempted_at
    {
        return Err(draft_pr_error!(
            "authority.exact_scope_required",
            "GitHub draft pull request requires fresh exact repository-scoped authority",
        ));
    }
    Ok(())
}

fn compose_proposed_record(
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<SideEffectRecord, WorkflowOsError> {
    let side_effect_id = input.preflight.side_effect_id().cloned().ok_or_else(|| {
        draft_pr_error!(
            "side_effect.id_missing",
            "GitHub draft pull request requires a proposed SideEffect ID",
        )
    })?;
    let idempotency_key = input.preflight.idempotency_key().cloned().ok_or_else(|| {
        draft_pr_error!(
            "idempotency.missing",
            "GitHub draft pull request requires an idempotency key",
        )
    })?;
    let authority = SideEffectAuthority::new(
        SideEffectAuthorityDecision::ApprovedByHuman,
        input.preflight.policy_references().to_vec(),
        input.preflight.approval_references().to_vec(),
    )?;
    let target = SideEffectTargetReference::new(
        SideEffectTargetKind::ProviderOperation,
        input.target.operation_reference(),
    )?;
    let idempotency = SideEffectIdempotencyBinding::new(
        idempotency_key,
        SideEffectIdempotencyScope::Integration,
        None,
        None,
    )?;
    let mut references = input.preflight.policy_references().to_vec();
    references.extend(input.preflight.approval_references().iter().cloned());
    references.push(SideEffectReference::new(
        SideEffectReferenceKind::WorkReport,
        input.work_report_artifact.report_id().as_str(),
    )?);
    SideEffectRecord::new(SideEffectRecordDefinition {
        side_effect_id,
        lifecycle_state: SideEffectLifecycleState::Proposed,
        target,
        capability: SideEffectCapability::GitHubWrite,
        authority,
        actor: Some(input.actor.clone()),
        system_actor: None,
        workflow_id: input.run.snapshot.identity.workflow_id.clone(),
        workflow_version: input.run.snapshot.identity.workflow_version.clone(),
        schema_version: input.run.snapshot.identity.schema_version.clone(),
        spec_hash: input.run.snapshot.identity.spec_content_hash.clone(),
        run_id: input.run.snapshot.identity.run_id.clone(),
        step_id: Some(input.step_id.clone()),
        skill_id: None,
        skill_version: None,
        adapter_id: Some(input.adapter_id.clone()),
        adapter_kind: Some(AdapterKind::GitHub),
        integration_id: Some(input.integration_id.clone()),
        idempotency,
        references,
        outcome_reference: None,
        created_at: input.proposed_at,
        updated_at: None,
        correlation_id: input.correlation_id.clone(),
        summary: Some("Governed draft GitHub pull request creation proposed".to_owned()),
        reason_codes: Vec::new(),
        sensitivity: input.sensitivity,
        redaction: input.redaction.clone(),
    })
}

fn transition_attempted(
    store: &impl SideEffectRecordStore,
    side_effect_id: &SideEffectId,
    input: &GitHubDraftPullRequestMutationInput<'_>,
) -> Result<SideEffectLifecycleTransitionResult, WorkflowOsError> {
    crate::transition_side_effect_to_attempted_in_store(
        store,
        crate::SideEffectAttemptTransitionStoreInput {
            side_effect_id,
            transitioned_at: input.attempted_at,
            summary: Some("GitHub draft pull request provider mutation attempted".to_owned()),
            additional_references: Vec::new(),
            evidence_reference_count: 0,
        },
    )
}

fn complete_result(
    store: &impl SideEffectRecordStore,
    input: &GitHubDraftPullRequestMutationInput<'_>,
    proposed_record: SideEffectRecord,
    attempted: SideEffectLifecycleTransitionResult,
    observation: &GitHubDraftPullRequestObservation,
    status: GitHubDraftPullRequestMutationStatus,
    create_attempted: bool,
) -> Result<GitHubDraftPullRequestMutationResult, WorkflowOsError> {
    let evidence = build_evidence(input, observation, status)?;
    let evidence_reference = SideEffectReference::new(
        SideEffectReferenceKind::EvidenceReference,
        evidence.id.as_str(),
    )?;
    let outcome_reference = SideEffectOutcomeReference::new(
        SideEffectOutcomeReferenceKind::Outcome,
        observation.provider_reference().to_owned(),
    )?;
    let completed = crate::transition_side_effect_to_completed_in_store(
        store,
        SideEffectCompleteTransitionStoreInput {
            side_effect_id: proposed_record.side_effect_id(),
            transitioned_at: input.outcome_at,
            outcome_reference,
            summary: Some("GitHub draft pull request provider state reconciled".to_owned()),
            additional_references: vec![evidence_reference],
            evidence_reference_count: 1,
        },
    )?;
    let citations = vec![
        WorkReportCitation::new(WorkReportCitationDefinition {
            target: WorkReportCitationTarget::SideEffect {
                side_effect_id: proposed_record.side_effect_id().clone(),
            },
            summary: None,
            missing: false,
            redaction: input.redaction.clone(),
            sensitivity: report_sensitivity(input.sensitivity),
        })?,
        WorkReportCitation::new(WorkReportCitationDefinition {
            target: WorkReportCitationTarget::EvidenceReference {
                evidence_reference_id: evidence.id.clone(),
            },
            summary: None,
            missing: false,
            redaction: input.redaction.clone(),
            sensitivity: report_sensitivity(input.sensitivity),
        })?,
    ];
    Ok(GitHubDraftPullRequestMutationResult {
        disclosure: disclosure(status, true, create_attempted, create_attempted),
        proposed_record,
        attempted_transition: Some(attempted),
        outcome_transition: Some(completed),
        evidence: Some(evidence),
        report_citations: citations,
    })
}

fn build_evidence(
    input: &GitHubDraftPullRequestMutationInput<'_>,
    observation: &GitHubDraftPullRequestObservation,
    status: GitHubDraftPullRequestMutationStatus,
) -> Result<EvidenceReference, WorkflowOsError> {
    let suffix = &input.content.commitment()[..16];
    let evidence_id = EvidenceReferenceId::new(format!("evidence/github-draft-pr-{suffix}"))?;
    let mut evidence = EvidenceReference::new(EvidenceReferenceRequiredFields {
        id: evidence_id,
        kind: EvidenceKind::AdapterResponseSummary,
        title: "GitHub draft pull request provider outcome".to_owned(),
        target: EvidenceReferenceTarget::external("github", observation.provider_reference())?,
        source_component: EvidenceSourceComponent::Adapter,
        scope: EvidenceScope::External,
        created_at: input.outcome_at,
        redaction_metadata: EvidenceRedactionMetadata::reference_only(
            "provider_object",
            "provider payload omitted; stable reference only",
        )?,
        sensitivity: Some(evidence_sensitivity(input.sensitivity)),
    })?
    .with_run_identity(
        input.run.snapshot.identity.workflow_id.clone(),
        input.run.snapshot.identity.workflow_version.clone(),
        input.run.snapshot.identity.schema_version.clone(),
        input.run.snapshot.identity.spec_content_hash.clone(),
        input.run.snapshot.identity.run_id.clone(),
    )
    .with_step_id(input.step_id.clone())
    .with_adapter(input.adapter_id.clone(), AdapterKind::GitHub);
    evidence.set_summary(match status {
        GitHubDraftPullRequestMutationStatus::Created => {
            "Draft GitHub pull request created and reconciled"
        }
        GitHubDraftPullRequestMutationStatus::ExistingManaged => {
            "Existing managed draft GitHub pull request reconciled"
        }
        _ => "GitHub draft pull request reconciliation requires operator review",
    })?;
    evidence.validate()?;
    Ok(evidence)
}

fn non_attempted_result(
    status: GitHubDraftPullRequestMutationStatus,
    proposed_record: SideEffectRecord,
) -> GitHubDraftPullRequestMutationResult {
    GitHubDraftPullRequestMutationResult {
        disclosure: disclosure(status, true, false, false),
        proposed_record,
        attempted_transition: None,
        outcome_transition: None,
        evidence: None,
        report_citations: Vec::new(),
    }
}

fn result_without_terminal(
    status: GitHubDraftPullRequestMutationStatus,
    proposed_record: SideEffectRecord,
    attempted: SideEffectLifecycleTransitionResult,
    create_attempted: bool,
    post_create_observation_performed: bool,
) -> GitHubDraftPullRequestMutationResult {
    GitHubDraftPullRequestMutationResult {
        disclosure: disclosure(
            status,
            true,
            create_attempted,
            post_create_observation_performed,
        ),
        proposed_record,
        attempted_transition: Some(attempted),
        outcome_transition: None,
        evidence: None,
        report_citations: Vec::new(),
    }
}

const fn disclosure(
    status: GitHubDraftPullRequestMutationStatus,
    lookup_performed: bool,
    create_attempted: bool,
    post_create_observation_performed: bool,
) -> GitHubDraftPullRequestDisclosure {
    let retry_blocked = matches!(
        status,
        GitHubDraftPullRequestMutationStatus::ExistingManagedWithRefDrift
            | GitHubDraftPullRequestMutationStatus::Conflict
            | GitHubDraftPullRequestMutationStatus::Ambiguous
            | GitHubDraftPullRequestMutationStatus::ConcurrentRefChange
    );
    let operator_action_required = !matches!(
        status,
        GitHubDraftPullRequestMutationStatus::Created
            | GitHubDraftPullRequestMutationStatus::ExistingManaged
    );
    GitHubDraftPullRequestDisclosure {
        status,
        lookup_performed,
        create_attempted,
        post_create_observation_performed,
        retry_blocked,
        operator_action_required,
    }
}

fn validate_managed_observation(
    observation: &GitHubDraftPullRequestObservation,
) -> Result<(), WorkflowOsError> {
    if !observation.draft() || !observation.managed_marker_matches() {
        return Err(draft_pr_error!(
            "provider.managed_observation_invalid",
            "GitHub pull request observation is not an exact managed draft",
        ));
    }
    Ok(())
}

fn refs_match_target(
    observation: &GitHubDraftPullRequestRefObservation,
    target: &GitHubDraftPullRequestTarget,
) -> bool {
    observation.head_sha() == target.expected_head_sha()
        && observation.base_sha() == target.observed_base_sha()
}

fn validate_branch(label: &str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > BRANCH_MAX_BYTES
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains("..")
        || value.contains("@{")
        || value.ends_with('.')
        || value.bytes().any(|byte| {
            byte.is_ascii_control()
                || byte.is_ascii_whitespace()
                || matches!(byte, b'~' | b'^' | b':' | b'?' | b'*' | b'[' | b'\\')
        })
    {
        return Err(draft_pr_error!(
            "target.branch_invalid",
            "GitHub draft pull request branch reference is invalid",
        ));
    }
    validate_not_secret_like(&format!("GitHub draft pull request {label}"), value)
}

fn validate_commit_sha(value: &str) -> Result<(), WorkflowOsError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(draft_pr_error!(
            "target.commit_sha_invalid",
            "GitHub draft pull request commit observation must be a lowercase full SHA",
        ));
    }
    Ok(())
}

fn validate_text(label: &str, value: &str, max_bytes: usize) -> Result<(), WorkflowOsError> {
    if value.trim().is_empty() || value.len() > max_bytes || value.contains('\0') {
        return Err(draft_pr_error!(
            "text.invalid",
            "GitHub draft pull request bounded text is invalid",
        ));
    }
    validate_not_secret_like(&format!("GitHub draft pull request {label}"), value)
}

fn content_commitment(template_version: &str, title: &str, body: &str, marker: &str) -> String {
    let mut digest = Sha256::new();
    for value in [template_version, title, body, marker] {
        digest.update((value.len() as u64).to_be_bytes());
        digest.update(value.as_bytes());
    }
    format!("{:x}", digest.finalize())
}

const fn evidence_sensitivity(value: SideEffectSensitivity) -> EvidenceSensitivity {
    match value {
        SideEffectSensitivity::Public => EvidenceSensitivity::Public,
        SideEffectSensitivity::Internal => EvidenceSensitivity::Internal,
        SideEffectSensitivity::Confidential => EvidenceSensitivity::Confidential,
        SideEffectSensitivity::Regulated => EvidenceSensitivity::Regulated,
        SideEffectSensitivity::Secret => EvidenceSensitivity::Secret,
        SideEffectSensitivity::Unknown => EvidenceSensitivity::Unknown,
    }
}

const fn report_sensitivity(value: SideEffectSensitivity) -> WorkReportSensitivity {
    match value {
        SideEffectSensitivity::Public => WorkReportSensitivity::Public,
        SideEffectSensitivity::Internal => WorkReportSensitivity::Internal,
        SideEffectSensitivity::Confidential => WorkReportSensitivity::Confidential,
        SideEffectSensitivity::Regulated => WorkReportSensitivity::Regulated,
        SideEffectSensitivity::Secret => WorkReportSensitivity::Secret,
        SideEffectSensitivity::Unknown => WorkReportSensitivity::Unknown,
    }
}
