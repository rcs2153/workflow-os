use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalPresentationContentHash, ApprovalPresentationId, ApprovalReferenceId,
    AuthorizedExecutionWindowId, CapabilityGrantId, EventId, SpecContentHash, Timestamp,
    WorkReportSensitivity, WorkflowId, WorkflowOsError, WorkflowRunId,
};

const IDENTIFIER_MAX_BYTES: usize = 128;
const REFERENCE_MAX_BYTES: usize = 256;
const MAX_TRANSITIONS: usize = 16;
const MAX_EVIDENCE_PREDICATES: usize = 32;
const MAX_EVIDENCE_AGE_SECONDS: u64 = 86_400;
const PROJECTION_ID_PREFIX: &str = "external-action-projection/";
const SUCCESSOR_BINDING_ID_PREFIX: &str = "external-action-successor/";

macro_rules! bounded_id {
    ($name:ident, $label:literal, $prefix:expr) => {
        #[doc = concat!("Deterministic identifier for one ", $label, ".")]
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Returns the ", $label, " text.")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            fn from_commitment(commitment: &SpecContentHash) -> Self {
                Self(format!("{}{}", $prefix, commitment.as_str()))
            }

            fn validate(&self) -> Result<(), WorkflowOsError> {
                let suffix = self.0.strip_prefix($prefix).ok_or_else(|| {
                    projection_error("id.invalid", concat!($label, " is invalid"))
                })?;
                if !is_sha256_hex(suffix) {
                    return Err(projection_error(
                        "id.invalid",
                        concat!($label, " is invalid"),
                    ));
                }
                Ok(())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"[REDACTED]")
                    .finish()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = Self(String::deserialize(deserializer)?);
                value
                    .validate()
                    .map_err(|_| serde::de::Error::custom(concat!($label, " is invalid")))?;
                Ok(value)
            }
        }
    };
}

bounded_id!(
    ExternalActionAuthorizationProjectionId,
    "external action authorization projection id",
    PROJECTION_ID_PREFIX
);
bounded_id!(
    ExternalActionSuccessorBindingId,
    "external action successor binding id",
    SUCCESSOR_BINDING_ID_PREFIX
);

/// Version of the immutable external-action authorization projection model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionAuthorizationProjectionVersion {
    /// Initial model-only projection contract.
    V1,
}

/// Connector family addressed by the projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalConnectorKind {
    /// GitHub connector boundary.
    GitHub,
}

/// Closed GitHub action set for the first delegated-maintainer lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionTransitionKind {
    /// Create one draft pull request for the exact bound repository and head.
    GitHubPullRequestCreateDraft,
    /// Fast-forward one exact Git ref from the expected old SHA to the reviewed SHA.
    GitHubGitRefFastForwardExact,
    /// Mark the exact derived pull request ready after terminal-green checks.
    GitHubPullRequestMarkReadyAfterTerminalGreen,
    /// Squash-merge the exact unchanged reviewed head.
    GitHubPullRequestSquashMergeUnchangedHead,
    /// Delete the exact non-default, non-protected branch after merge.
    GitHubBranchDeleteAfterMerge,
}

/// Bounded payload class committed for one transition without carrying its payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionPayloadClass {
    /// Draft pull-request title/body and draft posture.
    GitHubDraftPullRequest,
    /// Exact non-force ref fast-forward request.
    GitHubGitRefFastForward,
    /// Pull-request ready-state transition.
    GitHubPullRequestReadyState,
    /// Squash merge title/message/method request.
    GitHubPullRequestSquashMerge,
    /// Exact branch deletion request.
    GitHubBranchDeletion,
}

/// Exact predicate classes that may gate a projected transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalActionEvidencePredicateKind {
    /// The reviewed head still equals the committed head.
    ReviewedHeadMatches,
    /// The exact base target remains current.
    BaseTargetMatches,
    /// The exact payload commitment remains current.
    PayloadCommitmentMatches,
    /// The external ref still equals the expected old head.
    ExpectedOldHeadMatches,
    /// The exact required check set is terminal green for the reviewed head.
    RequiredChecksTerminalGreen,
    /// The pull request is ready for merge.
    PullRequestReady,
    /// Required review and policy predicates remain satisfied.
    ReviewAndPolicySatisfied,
    /// The exact squash merge completed successfully.
    ExactMergeSucceeded,
    /// The branch is neither default nor protected.
    BranchDeletionAllowed,
}

/// One payload-free, source-bound, freshness-bound evidence predicate.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct ExternalActionEvidencePredicate {
    kind: ExternalActionEvidencePredicateKind,
    source_commitment: SpecContentHash,
    subject_commitment: SpecContentHash,
    observed_at: Timestamp,
    max_age_seconds: u64,
}

impl<'de> Deserialize<'de> for ExternalActionEvidencePredicate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            kind: ExternalActionEvidencePredicateKind,
            source_commitment: SpecContentHash,
            subject_commitment: SpecContentHash,
            observed_at: Timestamp,
            max_age_seconds: u64,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.kind,
            wire.source_commitment,
            wire.subject_commitment,
            wire.observed_at,
            wire.max_age_seconds,
        )
        .map_err(|_| serde::de::Error::custom("external action evidence predicate is invalid"))
    }
}

impl ExternalActionEvidencePredicate {
    /// Creates one exact evidence predicate.
    ///
    /// # Errors
    ///
    /// Returns an error when the freshness bound is zero or exceeds one day.
    pub fn new(
        kind: ExternalActionEvidencePredicateKind,
        source_commitment: SpecContentHash,
        subject_commitment: SpecContentHash,
        observed_at: Timestamp,
        max_age_seconds: u64,
    ) -> Result<Self, WorkflowOsError> {
        if max_age_seconds == 0 || max_age_seconds > MAX_EVIDENCE_AGE_SECONDS {
            return Err(projection_error(
                "evidence.freshness_invalid",
                "external action evidence freshness is invalid",
            ));
        }
        Ok(Self {
            kind,
            source_commitment,
            subject_commitment,
            observed_at,
            max_age_seconds,
        })
    }

    #[must_use]
    /// Returns the predicate kind.
    pub const fn kind(&self) -> ExternalActionEvidencePredicateKind {
        self.kind
    }

    #[must_use]
    /// Returns the complete evidence-source commitment.
    pub const fn source_commitment(&self) -> &SpecContentHash {
        &self.source_commitment
    }

    #[must_use]
    /// Returns the exact evidence-subject commitment.
    pub const fn subject_commitment(&self) -> &SpecContentHash {
        &self.subject_commitment
    }

    #[must_use]
    /// Returns when the evidence source was observed.
    pub const fn observed_at(&self) -> Timestamp {
        self.observed_at
    }

    #[must_use]
    /// Returns the maximum permitted evidence age.
    pub const fn max_age_seconds(&self) -> u64 {
        self.max_age_seconds
    }
}

impl fmt::Debug for ExternalActionEvidencePredicate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionEvidencePredicate")
            .field("kind", &self.kind)
            .field("source_commitment", &"[REDACTED]")
            .field("subject_commitment", &"[REDACTED]")
            .field("observed_at", &self.observed_at)
            .field("max_age_seconds", &self.max_age_seconds)
            .finish()
    }
}

/// Exact GitHub repository and immutable commit binding.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GitHubExternalActionResourceBinding {
    repository: String,
    base_ref: String,
    base_commit_sha: String,
    head_ref: String,
    reviewed_head_sha: String,
}

impl<'de> Deserialize<'de> for GitHubExternalActionResourceBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            repository: String,
            base_ref: String,
            base_commit_sha: String,
            head_ref: String,
            reviewed_head_sha: String,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.repository,
            wire.base_ref,
            wire.base_commit_sha,
            wire.head_ref,
            wire.reviewed_head_sha,
        )
        .map_err(|_| serde::de::Error::custom("GitHub external action resource binding is invalid"))
    }
}

impl GitHubExternalActionResourceBinding {
    /// Creates an exact GitHub repository binding.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed repositories, refs, commit SHAs, or equal base/head refs.
    pub fn new(
        repository: impl Into<String>,
        base_ref: impl Into<String>,
        base_commit_sha: impl Into<String>,
        head_ref: impl Into<String>,
        reviewed_head_sha: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let binding = Self {
            repository: repository.into(),
            base_ref: base_ref.into(),
            base_commit_sha: base_commit_sha.into(),
            head_ref: head_ref.into(),
            reviewed_head_sha: reviewed_head_sha.into(),
        };
        binding.validate()?;
        Ok(binding)
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_repository(&self.repository)?;
        validate_git_ref("base ref", &self.base_ref)?;
        validate_git_ref("head ref", &self.head_ref)?;
        if self.base_ref == self.head_ref {
            return Err(projection_error(
                "resource.refs_equal",
                "external action base and head refs must differ",
            ));
        }
        validate_git_sha("base commit SHA", &self.base_commit_sha)?;
        validate_git_sha("reviewed head SHA", &self.reviewed_head_sha)
    }

    #[must_use]
    /// Returns the canonical owner/repository identity.
    pub fn repository(&self) -> &str {
        &self.repository
    }

    #[must_use]
    /// Returns the exact base ref.
    pub fn base_ref(&self) -> &str {
        &self.base_ref
    }

    #[must_use]
    /// Returns the exact base commit SHA.
    pub fn base_commit_sha(&self) -> &str {
        &self.base_commit_sha
    }

    #[must_use]
    /// Returns the exact head ref.
    pub fn head_ref(&self) -> &str {
        &self.head_ref
    }

    #[must_use]
    /// Returns the reviewed immutable head SHA.
    pub fn reviewed_head_sha(&self) -> &str {
        &self.reviewed_head_sha
    }
}

impl fmt::Debug for GitHubExternalActionResourceBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GitHubExternalActionResourceBinding")
            .field("repository", &"[REDACTED]")
            .field("base_ref", &"[REDACTED]")
            .field("base_commit_sha", &"[REDACTED]")
            .field("head_ref", &"[REDACTED]")
            .field("reviewed_head_sha", &"[REDACTED]")
            .finish()
    }
}

/// One immutable transition requirement inside a projection.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ExternalActionTransition {
    kind: ExternalActionTransitionKind,
    operation_commitment: SpecContentHash,
    payload_class: ExternalActionPayloadClass,
    payload_commitment: SpecContentHash,
    evidence_predicates: Vec<ExternalActionEvidencePredicate>,
}

impl ExternalActionTransition {
    /// Creates a validated, canonically ordered transition requirement.
    ///
    /// # Errors
    ///
    /// Returns an error for an incompatible payload class, duplicate evidence,
    /// missing required evidence, or an oversized predicate set.
    pub fn new(
        kind: ExternalActionTransitionKind,
        operation_commitment: SpecContentHash,
        payload_class: ExternalActionPayloadClass,
        payload_commitment: SpecContentHash,
        mut evidence_predicates: Vec<ExternalActionEvidencePredicate>,
    ) -> Result<Self, WorkflowOsError> {
        if payload_class != expected_payload_class(kind) {
            return Err(projection_error(
                "transition.payload_class_mismatch",
                "external action transition payload class does not match the action",
            ));
        }
        if evidence_predicates.len() > MAX_EVIDENCE_PREDICATES {
            return Err(projection_error(
                "transition.evidence_too_many",
                "external action transition has too many evidence predicates",
            ));
        }
        evidence_predicates.sort();
        if has_duplicate_kinds(&evidence_predicates) {
            return Err(projection_error(
                "transition.evidence_duplicate",
                "external action transition has duplicate evidence predicate kinds",
            ));
        }
        let present = evidence_predicates
            .iter()
            .map(ExternalActionEvidencePredicate::kind)
            .collect::<BTreeSet<_>>();
        if !required_evidence(kind).is_subset(&present) {
            return Err(projection_error(
                "transition.evidence_required",
                "external action transition is missing required evidence predicates",
            ));
        }
        Ok(Self {
            kind,
            operation_commitment,
            payload_class,
            payload_commitment,
            evidence_predicates,
        })
    }

    #[must_use]
    /// Returns the transition kind.
    pub const fn kind(&self) -> ExternalActionTransitionKind {
        self.kind
    }

    #[must_use]
    /// Returns the deterministic operation commitment.
    pub const fn operation_commitment(&self) -> &SpecContentHash {
        &self.operation_commitment
    }

    #[must_use]
    /// Returns the bounded payload class.
    pub const fn payload_class(&self) -> ExternalActionPayloadClass {
        self.payload_class
    }

    #[must_use]
    /// Returns the exact payload commitment.
    pub const fn payload_commitment(&self) -> &SpecContentHash {
        &self.payload_commitment
    }

    #[must_use]
    /// Returns the canonically ordered evidence predicates.
    pub fn evidence_predicates(&self) -> &[ExternalActionEvidencePredicate] {
        &self.evidence_predicates
    }
}

impl fmt::Debug for ExternalActionTransition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionTransition")
            .field("kind", &self.kind)
            .field("operation_commitment", &"[REDACTED]")
            .field("payload_class", &self.payload_class)
            .field("payload_commitment", &"[REDACTED]")
            .field("evidence_predicate_count", &self.evidence_predicates.len())
            .finish()
    }
}

impl<'de> Deserialize<'de> for ExternalActionTransition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            kind: ExternalActionTransitionKind,
            operation_commitment: SpecContentHash,
            payload_class: ExternalActionPayloadClass,
            payload_commitment: SpecContentHash,
            evidence_predicates: Vec<ExternalActionEvidencePredicate>,
        }
        let wire = Wire::deserialize(deserializer)?;
        Self::new(
            wire.kind,
            wire.operation_commitment,
            wire.payload_class,
            wire.payload_commitment,
            wire.evidence_predicates,
        )
        .map_err(|_| serde::de::Error::custom("external action transition is invalid"))
    }
}

/// Public construction input for an immutable external-action projection.
#[derive(Clone, Eq, PartialEq)]
pub struct ExternalActionAuthorizationProjectionDefinition {
    /// Governed workflow identity.
    pub workflow_id: WorkflowId,
    /// Exact governed run identity.
    pub run_id: WorkflowRunId,
    /// Durable approval decision reference.
    pub approval_reference_id: ApprovalReferenceId,
    /// Approval-presentation proof identity.
    pub presentation_id: ApprovalPresentationId,
    /// Exact approval-presentation content commitment.
    pub presentation_content_hash: ApprovalPresentationContentHash,
    /// Scoped capability-grant reference.
    pub capability_grant_id: CapabilityGrantId,
    /// Policy-decision event reference.
    pub policy_decision_event_id: EventId,
    /// Proportional-governance assessment commitment.
    pub governance_assessment_commitment: SpecContentHash,
    /// Authorized execution-window identity.
    pub execution_window_id: AuthorizedExecutionWindowId,
    /// Exact immutable run-bundle commitment.
    pub immutable_run_bundle_commitment: SpecContentHash,
    /// Connector family.
    pub connector_kind: ExternalConnectorKind,
    /// Canonical provider account or installation reference.
    pub connector_account_reference: String,
    /// Exact GitHub repository and commit binding.
    pub resource: GitHubExternalActionResourceBinding,
    /// Closed transition set.
    pub transitions: Vec<ExternalActionTransition>,
    /// Actor that may consume the projected authority.
    pub actor: ActorId,
    /// Actor that delegated the bounded authority.
    pub delegator: ActorId,
    /// Actor receiving the bounded delegation.
    pub delegate: ActorId,
    /// Exact delegation-scope commitment.
    pub delegation_scope_commitment: SpecContentHash,
    /// Projection issuance time.
    pub issued_at: Timestamp,
    /// Earliest permitted use time.
    pub not_before: Timestamp,
    /// Mandatory expiry time.
    pub expires_at: Timestamp,
    /// Commitment to the complete revocation source.
    pub revocation_source_commitment: SpecContentHash,
    /// Maximum disclosure sensitivity.
    pub sensitivity: WorkReportSensitivity,
}

impl fmt::Debug for ExternalActionAuthorizationProjectionDefinition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionAuthorizationProjectionDefinition")
            .field("workflow_id", &"[REDACTED]")
            .field("run_id", &"[REDACTED]")
            .field("approval_reference_id", &"[REDACTED]")
            .field("connector_kind", &self.connector_kind)
            .field("resource", &self.resource)
            .field("transition_count", &self.transitions.len())
            .field("issued_at", &self.issued_at)
            .field("not_before", &self.not_before)
            .field("expires_at", &self.expires_at)
            .field("sensitivity", &self.sensitivity)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
struct ProjectionCommitmentInput<'a> {
    domain: &'static str,
    version: ExternalActionAuthorizationProjectionVersion,
    workflow_id: &'a WorkflowId,
    run_id: &'a WorkflowRunId,
    approval_reference_id: &'a ApprovalReferenceId,
    presentation_id: &'a ApprovalPresentationId,
    presentation_content_hash: &'a ApprovalPresentationContentHash,
    capability_grant_id: &'a CapabilityGrantId,
    policy_decision_event_id: &'a EventId,
    governance_assessment_commitment: &'a SpecContentHash,
    execution_window_id: &'a AuthorizedExecutionWindowId,
    immutable_run_bundle_commitment: &'a SpecContentHash,
    connector_kind: ExternalConnectorKind,
    connector_account_reference: &'a str,
    resource: &'a GitHubExternalActionResourceBinding,
    transitions: &'a [ExternalActionTransition],
    actor: &'a ActorId,
    delegator: &'a ActorId,
    delegate: &'a ActorId,
    delegation_scope_commitment: &'a SpecContentHash,
    issued_at: Timestamp,
    not_before: Timestamp,
    expires_at: Timestamp,
    revocation_source_commitment: &'a SpecContentHash,
    sensitivity: WorkReportSensitivity,
}

/// Immutable, payload-free commitment to one closed connector transition set.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ExternalActionAuthorizationProjection {
    version: ExternalActionAuthorizationProjectionVersion,
    projection_id: ExternalActionAuthorizationProjectionId,
    projection_commitment: SpecContentHash,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    approval_reference_id: ApprovalReferenceId,
    presentation_id: ApprovalPresentationId,
    presentation_content_hash: ApprovalPresentationContentHash,
    capability_grant_id: CapabilityGrantId,
    policy_decision_event_id: EventId,
    governance_assessment_commitment: SpecContentHash,
    execution_window_id: AuthorizedExecutionWindowId,
    immutable_run_bundle_commitment: SpecContentHash,
    connector_kind: ExternalConnectorKind,
    connector_account_reference: String,
    resource: GitHubExternalActionResourceBinding,
    transitions: Vec<ExternalActionTransition>,
    actor: ActorId,
    delegator: ActorId,
    delegate: ActorId,
    delegation_scope_commitment: SpecContentHash,
    issued_at: Timestamp,
    not_before: Timestamp,
    expires_at: Timestamp,
    revocation_source_commitment: SpecContentHash,
    sensitivity: WorkReportSensitivity,
}

impl ExternalActionAuthorizationProjection {
    /// Creates an immutable, canonical authorization projection.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed time bounds, resource or actor substitution,
    /// duplicate transitions, missing draft creation dependencies, or unsafe metadata.
    pub fn new(
        mut definition: ExternalActionAuthorizationProjectionDefinition,
    ) -> Result<Self, WorkflowOsError> {
        validate_projection_definition(&definition)?;
        definition
            .transitions
            .sort_by_key(ExternalActionTransition::kind);
        let projection_commitment = compute_projection_commitment(&definition)?;
        Ok(Self {
            version: ExternalActionAuthorizationProjectionVersion::V1,
            projection_id: ExternalActionAuthorizationProjectionId::from_commitment(
                &projection_commitment,
            ),
            projection_commitment,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            approval_reference_id: definition.approval_reference_id,
            presentation_id: definition.presentation_id,
            presentation_content_hash: definition.presentation_content_hash,
            capability_grant_id: definition.capability_grant_id,
            policy_decision_event_id: definition.policy_decision_event_id,
            governance_assessment_commitment: definition.governance_assessment_commitment,
            execution_window_id: definition.execution_window_id,
            immutable_run_bundle_commitment: definition.immutable_run_bundle_commitment,
            connector_kind: definition.connector_kind,
            connector_account_reference: definition.connector_account_reference,
            resource: definition.resource,
            transitions: definition.transitions,
            actor: definition.actor,
            delegator: definition.delegator,
            delegate: definition.delegate,
            delegation_scope_commitment: definition.delegation_scope_commitment,
            issued_at: definition.issued_at,
            not_before: definition.not_before,
            expires_at: definition.expires_at,
            revocation_source_commitment: definition.revocation_source_commitment,
            sensitivity: definition.sensitivity,
        })
    }

    #[must_use]
    /// Returns the projection model version.
    pub const fn version(&self) -> ExternalActionAuthorizationProjectionVersion {
        self.version
    }

    #[must_use]
    /// Returns the deterministic projection identity.
    pub const fn projection_id(&self) -> &ExternalActionAuthorizationProjectionId {
        &self.projection_id
    }

    #[must_use]
    /// Returns the complete immutable projection commitment.
    pub const fn projection_commitment(&self) -> &SpecContentHash {
        &self.projection_commitment
    }

    #[must_use]
    /// Returns the exact GitHub resource binding.
    pub const fn resource(&self) -> &GitHubExternalActionResourceBinding {
        &self.resource
    }

    #[must_use]
    /// Returns the canonically ordered closed transition set.
    pub fn transitions(&self) -> &[ExternalActionTransition] {
        &self.transitions
    }

    #[must_use]
    /// Returns one transition requirement when it is projected.
    pub fn transition(
        &self,
        kind: ExternalActionTransitionKind,
    ) -> Option<&ExternalActionTransition> {
        self.transitions
            .iter()
            .find(|transition| transition.kind == kind)
    }

    #[must_use]
    /// Returns the mandatory projection expiry time.
    pub const fn expires_at(&self) -> Timestamp {
        self.expires_at
    }

    #[must_use]
    /// Returns the connector family.
    pub const fn connector_kind(&self) -> ExternalConnectorKind {
        self.connector_kind
    }
}

impl fmt::Debug for ExternalActionAuthorizationProjection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionAuthorizationProjection")
            .field("version", &self.version)
            .field("projection_id", &"[REDACTED]")
            .field("projection_commitment", &"[REDACTED]")
            .field("connector_kind", &self.connector_kind)
            .field("resource", &self.resource)
            .field("transition_count", &self.transitions.len())
            .field("issued_at", &self.issued_at)
            .field("not_before", &self.not_before)
            .field("expires_at", &self.expires_at)
            .field("sensitivity", &self.sensitivity)
            .finish_non_exhaustive()
    }
}

impl<'de> Deserialize<'de> for ExternalActionAuthorizationProjection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            version: ExternalActionAuthorizationProjectionVersion,
            projection_id: ExternalActionAuthorizationProjectionId,
            projection_commitment: SpecContentHash,
            workflow_id: WorkflowId,
            run_id: WorkflowRunId,
            approval_reference_id: ApprovalReferenceId,
            presentation_id: ApprovalPresentationId,
            presentation_content_hash: ApprovalPresentationContentHash,
            capability_grant_id: CapabilityGrantId,
            policy_decision_event_id: EventId,
            governance_assessment_commitment: SpecContentHash,
            execution_window_id: AuthorizedExecutionWindowId,
            immutable_run_bundle_commitment: SpecContentHash,
            connector_kind: ExternalConnectorKind,
            connector_account_reference: String,
            resource: GitHubExternalActionResourceBinding,
            transitions: Vec<ExternalActionTransition>,
            actor: ActorId,
            delegator: ActorId,
            delegate: ActorId,
            delegation_scope_commitment: SpecContentHash,
            issued_at: Timestamp,
            not_before: Timestamp,
            expires_at: Timestamp,
            revocation_source_commitment: SpecContentHash,
            sensitivity: WorkReportSensitivity,
        }
        let wire = Wire::deserialize(deserializer)?;
        if wire.version != ExternalActionAuthorizationProjectionVersion::V1 {
            return Err(serde::de::Error::custom(
                "external action authorization projection version is invalid",
            ));
        }
        let rebuilt = Self::new(ExternalActionAuthorizationProjectionDefinition {
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            approval_reference_id: wire.approval_reference_id,
            presentation_id: wire.presentation_id,
            presentation_content_hash: wire.presentation_content_hash,
            capability_grant_id: wire.capability_grant_id,
            policy_decision_event_id: wire.policy_decision_event_id,
            governance_assessment_commitment: wire.governance_assessment_commitment,
            execution_window_id: wire.execution_window_id,
            immutable_run_bundle_commitment: wire.immutable_run_bundle_commitment,
            connector_kind: wire.connector_kind,
            connector_account_reference: wire.connector_account_reference,
            resource: wire.resource,
            transitions: wire.transitions,
            actor: wire.actor,
            delegator: wire.delegator,
            delegate: wire.delegate,
            delegation_scope_commitment: wire.delegation_scope_commitment,
            issued_at: wire.issued_at,
            not_before: wire.not_before,
            expires_at: wire.expires_at,
            revocation_source_commitment: wire.revocation_source_commitment,
            sensitivity: wire.sensitivity,
        })
        .map_err(|_| {
            serde::de::Error::custom("external action authorization projection is invalid")
        })?;
        if rebuilt.projection_id != wire.projection_id
            || rebuilt.projection_commitment != wire.projection_commitment
        {
            return Err(serde::de::Error::custom(
                "external action authorization projection commitment is invalid",
            ));
        }
        Ok(rebuilt)
    }
}

/// Payload-free binding from draft creation to the exact provider-created PR identity.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct ExternalActionSuccessorBinding {
    binding_id: ExternalActionSuccessorBindingId,
    binding_commitment: SpecContentHash,
    projection_id: ExternalActionAuthorizationProjectionId,
    create_operation_commitment: SpecContentHash,
    repository_commitment: SpecContentHash,
    provider_pull_request_number: u64,
    reviewed_head_sha: String,
    observed_at: Timestamp,
}

impl ExternalActionSuccessorBinding {
    #[must_use]
    /// Returns the deterministic successor-binding identity.
    pub const fn binding_id(&self) -> &ExternalActionSuccessorBindingId {
        &self.binding_id
    }

    #[must_use]
    /// Returns the complete successor-binding commitment.
    pub const fn binding_commitment(&self) -> &SpecContentHash {
        &self.binding_commitment
    }

    #[must_use]
    /// Returns the exact provider-created pull-request number.
    pub const fn provider_pull_request_number(&self) -> u64 {
        self.provider_pull_request_number
    }

    /// Verifies that the successor still belongs to the exact projection.
    ///
    /// # Errors
    ///
    /// Returns an error when the projection, create operation, repository, head,
    /// or deterministic binding commitment does not match.
    pub fn validate_for_projection(
        &self,
        projection: &ExternalActionAuthorizationProjection,
    ) -> Result<(), WorkflowOsError> {
        let expected = derive_external_action_successor_binding(
            projection,
            self.provider_pull_request_number,
            self.observed_at,
        )?;
        if &expected != self {
            return Err(projection_error(
                "successor.projection_mismatch",
                "external action successor binding does not match the projection",
            ));
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for ExternalActionSuccessorBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            binding_id: ExternalActionSuccessorBindingId,
            binding_commitment: SpecContentHash,
            projection_id: ExternalActionAuthorizationProjectionId,
            create_operation_commitment: SpecContentHash,
            repository_commitment: SpecContentHash,
            provider_pull_request_number: u64,
            reviewed_head_sha: String,
            observed_at: Timestamp,
        }
        let wire = Wire::deserialize(deserializer)?;
        validate_git_sha("reviewed head SHA", &wire.reviewed_head_sha).map_err(|_| {
            serde::de::Error::custom("external action successor binding is invalid")
        })?;
        if wire.provider_pull_request_number == 0 {
            return Err(serde::de::Error::custom(
                "external action successor binding is invalid",
            ));
        }
        let expected = compute_successor_binding_commitment(
            &wire.projection_id,
            &wire.create_operation_commitment,
            &wire.repository_commitment,
            wire.provider_pull_request_number,
            &wire.reviewed_head_sha,
            wire.observed_at,
        )
        .map_err(|_| serde::de::Error::custom("external action successor binding is invalid"))?;
        if expected != wire.binding_commitment
            || ExternalActionSuccessorBindingId::from_commitment(&expected) != wire.binding_id
        {
            return Err(serde::de::Error::custom(
                "external action successor binding commitment is invalid",
            ));
        }
        Ok(Self {
            binding_id: wire.binding_id,
            binding_commitment: wire.binding_commitment,
            projection_id: wire.projection_id,
            create_operation_commitment: wire.create_operation_commitment,
            repository_commitment: wire.repository_commitment,
            provider_pull_request_number: wire.provider_pull_request_number,
            reviewed_head_sha: wire.reviewed_head_sha,
            observed_at: wire.observed_at,
        })
    }
}

impl fmt::Debug for ExternalActionSuccessorBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalActionSuccessorBinding")
            .field("binding_id", &"[REDACTED]")
            .field("binding_commitment", &"[REDACTED]")
            .field("projection_id", &"[REDACTED]")
            .field("provider_pull_request_number", &"[REDACTED]")
            .field("reviewed_head_sha", &"[REDACTED]")
            .field("observed_at", &self.observed_at)
            .finish_non_exhaustive()
    }
}

/// Derives one non-authorizing successor binding from a validated draft-create transition.
///
/// # Errors
///
/// Returns an error when the PR number is zero or the projection does not authorize
/// draft creation.
pub fn derive_external_action_successor_binding(
    projection: &ExternalActionAuthorizationProjection,
    provider_pull_request_number: u64,
    observed_at: Timestamp,
) -> Result<ExternalActionSuccessorBinding, WorkflowOsError> {
    if provider_pull_request_number == 0 {
        return Err(projection_error(
            "successor.pr_number_invalid",
            "external action successor pull request number is invalid",
        ));
    }
    let create = projection
        .transition(ExternalActionTransitionKind::GitHubPullRequestCreateDraft)
        .ok_or_else(|| {
            projection_error(
                "successor.create_transition_missing",
                "external action successor binding requires draft creation authority",
            )
        })?;
    let repository_commitment = SpecContentHash::from_text(projection.resource().repository());
    let binding_commitment = compute_successor_binding_commitment(
        projection.projection_id(),
        create.operation_commitment(),
        &repository_commitment,
        provider_pull_request_number,
        projection.resource().reviewed_head_sha(),
        observed_at,
    )?;
    Ok(ExternalActionSuccessorBinding {
        binding_id: ExternalActionSuccessorBindingId::from_commitment(&binding_commitment),
        binding_commitment,
        projection_id: projection.projection_id().clone(),
        create_operation_commitment: create.operation_commitment().clone(),
        repository_commitment,
        provider_pull_request_number,
        reviewed_head_sha: projection.resource().reviewed_head_sha().to_owned(),
        observed_at,
    })
}

fn compute_successor_binding_commitment(
    projection_id: &ExternalActionAuthorizationProjectionId,
    create_operation_commitment: &SpecContentHash,
    repository_commitment: &SpecContentHash,
    provider_pull_request_number: u64,
    reviewed_head_sha: &str,
    observed_at: Timestamp,
) -> Result<SpecContentHash, WorkflowOsError> {
    #[derive(Serialize)]
    struct BindingInput<'a> {
        domain: &'static str,
        projection_id: &'a ExternalActionAuthorizationProjectionId,
        create_operation_commitment: &'a SpecContentHash,
        repository_commitment: &'a SpecContentHash,
        provider_pull_request_number: u64,
        reviewed_head_sha: &'a str,
        observed_at: Timestamp,
    }
    let bytes = serde_json::to_vec(&BindingInput {
        domain: "workflow-os/external-action-successor-binding/v1",
        projection_id,
        create_operation_commitment,
        repository_commitment,
        provider_pull_request_number,
        reviewed_head_sha,
        observed_at,
    })
    .map_err(|_| {
        projection_error(
            "successor.commitment_failed",
            "external action successor binding commitment failed",
        )
    })?;
    Ok(SpecContentHash::from_bytes(bytes))
}

fn validate_projection_definition(
    definition: &ExternalActionAuthorizationProjectionDefinition,
) -> Result<(), WorkflowOsError> {
    if definition.not_before < definition.issued_at
        || definition.expires_at <= definition.not_before
    {
        return Err(projection_error(
            "time_bounds.invalid",
            "external action authorization projection time bounds are invalid",
        ));
    }
    if definition.sensitivity == WorkReportSensitivity::Unknown {
        return Err(projection_error(
            "sensitivity.unknown",
            "external action authorization projection sensitivity must be known",
        ));
    }
    validate_reference(
        "connector account reference",
        &definition.connector_account_reference,
    )?;
    definition.resource.validate()?;
    validate_actor("actor", &definition.actor)?;
    validate_actor("delegator", &definition.delegator)?;
    validate_actor("delegate", &definition.delegate)?;
    if definition.actor != definition.delegate {
        return Err(projection_error(
            "delegation.actor_delegate_mismatch",
            "external action actor must match the delegated actor",
        ));
    }
    if definition.transitions.is_empty() || definition.transitions.len() > MAX_TRANSITIONS {
        return Err(projection_error(
            "transition.count_invalid",
            "external action authorization projection transition count is invalid",
        ));
    }
    let kinds = definition
        .transitions
        .iter()
        .map(ExternalActionTransition::kind)
        .collect::<BTreeSet<_>>();
    if kinds.len() != definition.transitions.len() {
        return Err(projection_error(
            "transition.duplicate",
            "external action authorization projection has duplicate transitions",
        ));
    }
    let later_pr_transition = kinds
        .contains(&ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen)
        || kinds.contains(&ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead)
        || kinds.contains(&ExternalActionTransitionKind::GitHubBranchDeleteAfterMerge);
    if later_pr_transition
        && !kinds.contains(&ExternalActionTransitionKind::GitHubPullRequestCreateDraft)
    {
        return Err(projection_error(
            "transition.create_dependency_missing",
            "external action pull request lifecycle requires draft creation authority",
        ));
    }
    if kinds.contains(&ExternalActionTransitionKind::GitHubBranchDeleteAfterMerge)
        && !kinds.contains(&ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead)
    {
        return Err(projection_error(
            "transition.merge_dependency_missing",
            "external action branch deletion requires squash merge authority",
        ));
    }
    Ok(())
}

fn compute_projection_commitment(
    definition: &ExternalActionAuthorizationProjectionDefinition,
) -> Result<SpecContentHash, WorkflowOsError> {
    let input = ProjectionCommitmentInput {
        domain: "workflow-os/external-action-authorization-projection/v1",
        version: ExternalActionAuthorizationProjectionVersion::V1,
        workflow_id: &definition.workflow_id,
        run_id: &definition.run_id,
        approval_reference_id: &definition.approval_reference_id,
        presentation_id: &definition.presentation_id,
        presentation_content_hash: &definition.presentation_content_hash,
        capability_grant_id: &definition.capability_grant_id,
        policy_decision_event_id: &definition.policy_decision_event_id,
        governance_assessment_commitment: &definition.governance_assessment_commitment,
        execution_window_id: &definition.execution_window_id,
        immutable_run_bundle_commitment: &definition.immutable_run_bundle_commitment,
        connector_kind: definition.connector_kind,
        connector_account_reference: &definition.connector_account_reference,
        resource: &definition.resource,
        transitions: &definition.transitions,
        actor: &definition.actor,
        delegator: &definition.delegator,
        delegate: &definition.delegate,
        delegation_scope_commitment: &definition.delegation_scope_commitment,
        issued_at: definition.issued_at,
        not_before: definition.not_before,
        expires_at: definition.expires_at,
        revocation_source_commitment: &definition.revocation_source_commitment,
        sensitivity: definition.sensitivity,
    };
    let bytes = serde_json::to_vec(&input).map_err(|_| {
        projection_error(
            "commitment.failed",
            "external action authorization projection commitment failed",
        )
    })?;
    Ok(SpecContentHash::from_bytes(bytes))
}

fn expected_payload_class(kind: ExternalActionTransitionKind) -> ExternalActionPayloadClass {
    match kind {
        ExternalActionTransitionKind::GitHubPullRequestCreateDraft => {
            ExternalActionPayloadClass::GitHubDraftPullRequest
        }
        ExternalActionTransitionKind::GitHubGitRefFastForwardExact => {
            ExternalActionPayloadClass::GitHubGitRefFastForward
        }
        ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen => {
            ExternalActionPayloadClass::GitHubPullRequestReadyState
        }
        ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead => {
            ExternalActionPayloadClass::GitHubPullRequestSquashMerge
        }
        ExternalActionTransitionKind::GitHubBranchDeleteAfterMerge => {
            ExternalActionPayloadClass::GitHubBranchDeletion
        }
    }
}

fn required_evidence(
    kind: ExternalActionTransitionKind,
) -> BTreeSet<ExternalActionEvidencePredicateKind> {
    use ExternalActionEvidencePredicateKind as Evidence;
    match kind {
        ExternalActionTransitionKind::GitHubPullRequestCreateDraft => [
            Evidence::ReviewedHeadMatches,
            Evidence::BaseTargetMatches,
            Evidence::PayloadCommitmentMatches,
        ]
        .into_iter()
        .collect(),
        ExternalActionTransitionKind::GitHubGitRefFastForwardExact => [
            Evidence::ExpectedOldHeadMatches,
            Evidence::ReviewedHeadMatches,
        ]
        .into_iter()
        .collect(),
        ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen => [
            Evidence::ReviewedHeadMatches,
            Evidence::RequiredChecksTerminalGreen,
        ]
        .into_iter()
        .collect(),
        ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead => [
            Evidence::ReviewedHeadMatches,
            Evidence::RequiredChecksTerminalGreen,
            Evidence::PullRequestReady,
            Evidence::ReviewAndPolicySatisfied,
        ]
        .into_iter()
        .collect(),
        ExternalActionTransitionKind::GitHubBranchDeleteAfterMerge => [
            Evidence::ExactMergeSucceeded,
            Evidence::BranchDeletionAllowed,
        ]
        .into_iter()
        .collect(),
    }
}

fn has_duplicate_kinds(predicates: &[ExternalActionEvidencePredicate]) -> bool {
    predicates
        .windows(2)
        .any(|pair| pair[0].kind == pair[1].kind)
}

fn validate_repository(value: &str) -> Result<(), WorkflowOsError> {
    validate_reference("repository", value)?;
    let mut parts = value.split('/');
    let owner = parts.next().unwrap_or_default();
    let repository = parts.next().unwrap_or_default();
    if owner.is_empty() || repository.is_empty() || parts.next().is_some() {
        return Err(projection_error(
            "resource.repository_invalid",
            "external action repository is invalid",
        ));
    }
    Ok(())
}

fn validate_git_ref(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    validate_reference(label, value)?;
    if value.starts_with('/')
        || value.ends_with('/')
        || value.ends_with('.')
        || value.contains("..")
        || value.contains("@{")
        || value.contains("//")
    {
        return Err(projection_error(
            "resource.ref_invalid",
            "external action Git ref is invalid",
        ));
    }
    Ok(())
}

fn validate_git_sha(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(projection_error(
            "resource.commit_sha_invalid",
            format!("external action {label} is invalid"),
        ));
    }
    Ok(())
}

fn validate_actor(label: &'static str, actor: &ActorId) -> Result<(), WorkflowOsError> {
    validate_not_secret_like(label, actor.as_str())
}

fn validate_reference(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() || value.len() > REFERENCE_MAX_BYTES || value.chars().any(char::is_control)
    {
        return Err(projection_error(
            "reference.invalid",
            format!("external action {label} is invalid"),
        ));
    }
    validate_not_secret_like(label, value)
}

fn validate_not_secret_like(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let normalized = value.to_ascii_lowercase();
    if value.len() > IDENTIFIER_MAX_BYTES
        || [
            "bearer ",
            "password",
            "private_key",
            "private-key",
            "credential",
            "api_key",
            "api-key",
            "secret",
            "token",
            "sk-",
        ]
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        return Err(projection_error(
            "reference.secret_like",
            format!("external action {label} is invalid"),
        ));
    }
    Ok(())
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn projection_error(suffix: &str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(format!("external_action_authorization.{suffix}"), message)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use serde_json::Value;

    use super::*;

    fn time(value: &str) -> Timestamp {
        Timestamp::parse_rfc3339(value).expect("valid time")
    }

    fn hash(value: &str) -> SpecContentHash {
        SpecContentHash::from_text(value)
    }

    fn predicate(kind: ExternalActionEvidencePredicateKind) -> ExternalActionEvidencePredicate {
        ExternalActionEvidencePredicate::new(
            kind,
            hash(&format!("source-{kind:?}")),
            hash(&format!("subject-{kind:?}")),
            time("2026-11-01T20:00:00Z"),
            3_600,
        )
        .expect("predicate")
    }

    fn transition(kind: ExternalActionTransitionKind) -> ExternalActionTransition {
        ExternalActionTransition::new(
            kind,
            hash(&format!("operation-{kind:?}")),
            expected_payload_class(kind),
            hash(&format!("payload-{kind:?}")),
            required_evidence(kind)
                .into_iter()
                .rev()
                .map(predicate)
                .collect(),
        )
        .expect("transition")
    }

    fn definition() -> ExternalActionAuthorizationProjectionDefinition {
        ExternalActionAuthorizationProjectionDefinition {
            workflow_id: WorkflowId::new("dg/pr").expect("workflow"),
            run_id: WorkflowRunId::new("run/external-action").expect("run"),
            approval_reference_id: ApprovalReferenceId::new("approval/external-action")
                .expect("approval"),
            presentation_id: ApprovalPresentationId::new("presentation/external-action")
                .expect("presentation"),
            presentation_content_hash: ApprovalPresentationContentHash::new(
                hash("presentation").as_str(),
            )
            .expect("presentation hash"),
            capability_grant_id: CapabilityGrantId::new("grant/external-action").expect("grant"),
            policy_decision_event_id: EventId::new("event/external-action").expect("event"),
            governance_assessment_commitment: hash("governance"),
            execution_window_id: AuthorizedExecutionWindowId::new("window/external-action")
                .expect("window"),
            immutable_run_bundle_commitment: hash("bundle"),
            connector_kind: ExternalConnectorKind::GitHub,
            connector_account_reference: "github-installation/workflow-os".to_owned(),
            resource: GitHubExternalActionResourceBinding::new(
                "rcs2153/workflow-os",
                "main",
                "1111111111111111111111111111111111111111",
                "codex/external-action-model",
                "2222222222222222222222222222222222222222",
            )
            .expect("resource"),
            transitions: vec![
                transition(ExternalActionTransitionKind::GitHubPullRequestSquashMergeUnchangedHead),
                transition(ExternalActionTransitionKind::GitHubPullRequestCreateDraft),
                transition(
                    ExternalActionTransitionKind::GitHubPullRequestMarkReadyAfterTerminalGreen,
                ),
            ],
            actor: ActorId::new("actor/delegated-maintainer").expect("actor"),
            delegator: ActorId::new("actor/repository-owner").expect("delegator"),
            delegate: ActorId::new("actor/delegated-maintainer").expect("delegate"),
            delegation_scope_commitment: hash("delegation"),
            issued_at: time("2026-11-01T20:00:00Z"),
            not_before: time("2026-11-01T20:00:00Z"),
            expires_at: time("2026-11-01T22:00:00Z"),
            revocation_source_commitment: hash("revocation-source"),
            sensitivity: WorkReportSensitivity::Internal,
        }
    }

    #[test]
    fn projection_canonicalizes_transition_and_evidence_order() {
        let first = ExternalActionAuthorizationProjection::new(definition()).expect("projection");
        let mut reordered = definition();
        reordered.transitions.reverse();
        let second = ExternalActionAuthorizationProjection::new(reordered).expect("projection");
        assert_eq!(
            first.projection_commitment(),
            second.projection_commitment()
        );
        assert_eq!(first, second);
    }

    #[test]
    fn projection_rejects_duplicate_and_dependency_broadening() {
        let mut duplicate = definition();
        duplicate.transitions.push(transition(
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
        ));
        assert_eq!(
            ExternalActionAuthorizationProjection::new(duplicate)
                .expect_err("duplicate")
                .code(),
            "external_action_authorization.transition.duplicate"
        );

        let mut missing_create = definition();
        missing_create.transitions.retain(|transition| {
            transition.kind() != ExternalActionTransitionKind::GitHubPullRequestCreateDraft
        });
        assert_eq!(
            ExternalActionAuthorizationProjection::new(missing_create)
                .expect_err("missing create")
                .code(),
            "external_action_authorization.transition.create_dependency_missing"
        );
    }

    #[test]
    fn transition_rejects_wrong_payload_and_missing_or_duplicate_evidence() {
        let wrong_payload = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            hash("operation"),
            ExternalActionPayloadClass::GitHubBranchDeletion,
            hash("payload"),
            vec![],
        )
        .expect_err("wrong payload");
        assert_eq!(
            wrong_payload.code(),
            "external_action_authorization.transition.payload_class_mismatch"
        );

        let missing = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubPullRequestCreateDraft,
            hash("operation"),
            ExternalActionPayloadClass::GitHubDraftPullRequest,
            hash("payload"),
            vec![predicate(
                ExternalActionEvidencePredicateKind::ReviewedHeadMatches,
            )],
        )
        .expect_err("missing evidence");
        assert_eq!(
            missing.code(),
            "external_action_authorization.transition.evidence_required"
        );

        let repeated = predicate(ExternalActionEvidencePredicateKind::ReviewedHeadMatches);
        let duplicate = ExternalActionTransition::new(
            ExternalActionTransitionKind::GitHubGitRefFastForwardExact,
            hash("operation"),
            ExternalActionPayloadClass::GitHubGitRefFastForward,
            hash("payload"),
            vec![
                repeated.clone(),
                repeated,
                predicate(ExternalActionEvidencePredicateKind::ExpectedOldHeadMatches),
            ],
        )
        .expect_err("duplicate evidence");
        assert_eq!(
            duplicate.code(),
            "external_action_authorization.transition.evidence_duplicate"
        );
    }

    #[test]
    fn projection_commitment_detects_serialized_scope_substitution() {
        let projection =
            ExternalActionAuthorizationProjection::new(definition()).expect("projection");
        let mut value = serde_json::to_value(&projection).expect("serialize");
        value["resource"]["reviewed_head_sha"] =
            Value::String("3333333333333333333333333333333333333333".to_owned());
        let error = serde_json::from_value::<ExternalActionAuthorizationProjection>(value)
            .expect_err("changed head must fail");
        assert!(error
            .to_string()
            .contains("projection commitment is invalid"));
    }

    #[test]
    fn successor_binding_is_exact_and_non_substitutable() {
        let projection =
            ExternalActionAuthorizationProjection::new(definition()).expect("projection");
        let binding = derive_external_action_successor_binding(
            &projection,
            676,
            time("2026-11-01T21:00:00Z"),
        )
        .expect("binding");
        binding
            .validate_for_projection(&projection)
            .expect("same projection");

        let mut changed = definition();
        changed.resource = GitHubExternalActionResourceBinding::new(
            "rcs2153/workflow-os",
            "main",
            "1111111111111111111111111111111111111111",
            "codex/external-action-model",
            "3333333333333333333333333333333333333333",
        )
        .expect("changed resource");
        let changed =
            ExternalActionAuthorizationProjection::new(changed).expect("changed projection");
        assert_eq!(
            binding
                .validate_for_projection(&changed)
                .expect_err("changed projection")
                .code(),
            "external_action_authorization.successor.projection_mismatch"
        );
    }

    #[test]
    fn public_nested_models_revalidate_deserialized_input() {
        let predicate = predicate(ExternalActionEvidencePredicateKind::ReviewedHeadMatches);
        let mut predicate_value = serde_json::to_value(predicate).expect("predicate value");
        predicate_value["max_age_seconds"] = Value::from(0);
        assert!(
            serde_json::from_value::<ExternalActionEvidencePredicate>(predicate_value)
                .expect_err("zero freshness")
                .to_string()
                .contains("evidence predicate is invalid")
        );

        let resource = definition().resource;
        let mut resource_value = serde_json::to_value(resource).expect("resource value");
        resource_value["head_ref"] = Value::String("main".to_owned());
        assert!(
            serde_json::from_value::<GitHubExternalActionResourceBinding>(resource_value)
                .expect_err("equal refs")
                .to_string()
                .contains("resource binding is invalid")
        );
    }

    #[test]
    fn successor_serialization_rejects_changed_provider_identity() {
        let projection =
            ExternalActionAuthorizationProjection::new(definition()).expect("projection");
        let binding = derive_external_action_successor_binding(
            &projection,
            676,
            time("2026-11-01T21:00:00Z"),
        )
        .expect("binding");
        let mut value = serde_json::to_value(binding).expect("binding value");
        value["provider_pull_request_number"] = Value::from(677);
        assert!(
            serde_json::from_value::<ExternalActionSuccessorBinding>(value)
                .expect_err("changed PR")
                .to_string()
                .contains("binding commitment is invalid")
        );
    }

    #[test]
    fn debug_and_errors_do_not_expose_bound_resource_values() {
        let projection =
            ExternalActionAuthorizationProjection::new(definition()).expect("projection");
        let debug = format!("{projection:?}");
        assert!(!debug.contains("rcs2153"));
        assert!(!debug.contains("2222222222"));

        let error = GitHubExternalActionResourceBinding::new(
            "owner/repository",
            "main",
            "not-a-sha",
            "feature",
            "2222222222222222222222222222222222222222",
        )
        .expect_err("invalid SHA");
        assert!(!error.message().contains("not-a-sha"));
    }
}
