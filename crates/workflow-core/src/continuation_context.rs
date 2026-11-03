use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{ActorId, ProjectId, SpecContentHash, WorkflowId, WorkflowOsError, WorkflowRunId};

const CONTEXT_ALGORITHM: &str = "workflow-os/continuation-context/v2";
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_TEXT_BYTES: usize = 1_024;
const MAX_SOURCES: usize = 256;
const MAX_CLAIMS_PER_SOURCE: usize = 64;
const MAX_COVERAGE_ENTRIES: usize = 32;
const MAX_ACCOUNTING_ENTRIES: usize = 256;
const ALL_CONTEXT_SOURCE_CLASSES: [ContinuationContextSourceClass; 18] = [
    ContinuationContextSourceClass::ValidatedDeclaration,
    ContinuationContextSourceClass::ImmutableRunBundle,
    ContinuationContextSourceClass::DurableRunState,
    ContinuationContextSourceClass::EventHistory,
    ContinuationContextSourceClass::Policy,
    ContinuationContextSourceClass::Approval,
    ContinuationContextSourceClass::Capability,
    ContinuationContextSourceClass::RequiredContext,
    ContinuationContextSourceClass::Evidence,
    ContinuationContextSourceClass::Check,
    ContinuationContextSourceClass::SideEffect,
    ContinuationContextSourceClass::WorkReport,
    ContinuationContextSourceClass::AcceptedIntent,
    ContinuationContextSourceClass::TypedHandoff,
    ContinuationContextSourceClass::GitObservation,
    ContinuationContextSourceClass::OperatingDocument,
    ContinuationContextSourceClass::ProviderObservation,
    ContinuationContextSourceClass::AgentAdvisory,
];
const ALL_UNDERSTANDING_CLAIM_KINDS: [ContinuationUnderstandingClaimKind; 23] = [
    ContinuationUnderstandingClaimKind::Objective,
    ContinuationUnderstandingClaimKind::Priority,
    ContinuationUnderstandingClaimKind::AcceptedAmendment,
    ContinuationUnderstandingClaimKind::Scope,
    ContinuationUnderstandingClaimKind::StrictNonGoal,
    ContinuationUnderstandingClaimKind::Decision,
    ContinuationUnderstandingClaimKind::RationaleReference,
    ContinuationUnderstandingClaimKind::CompletedObligation,
    ContinuationUnderstandingClaimKind::InProgressObligation,
    ContinuationUnderstandingClaimKind::DeferredObligation,
    ContinuationUnderstandingClaimKind::RemainingObligation,
    ContinuationUnderstandingClaimKind::Blocker,
    ContinuationUnderstandingClaimKind::RegisteredWait,
    ContinuationUnderstandingClaimKind::OpenQuestion,
    ContinuationUnderstandingClaimKind::Assumption,
    ContinuationUnderstandingClaimKind::Uncertainty,
    ContinuationUnderstandingClaimKind::EvidenceRequirement,
    ContinuationUnderstandingClaimKind::CheckRequirement,
    ContinuationUnderstandingClaimKind::SideEffectPosture,
    ContinuationUnderstandingClaimKind::ValidationCommitment,
    ContinuationUnderstandingClaimKind::ReportingCommitment,
    ContinuationUnderstandingClaimKind::PromisedFollowUp,
    ContinuationUnderstandingClaimKind::LawfulNextOption,
];
const ALL_CONTEXT_AUTHORITY_DOMAINS: [ContinuationContextAuthorityDomain; 18] = [
    ContinuationContextAuthorityDomain::Declaration,
    ContinuationContextAuthorityDomain::ImmutableRunInput,
    ContinuationContextAuthorityDomain::DurableRunState,
    ContinuationContextAuthorityDomain::EventHistory,
    ContinuationContextAuthorityDomain::Policy,
    ContinuationContextAuthorityDomain::Approval,
    ContinuationContextAuthorityDomain::Capability,
    ContinuationContextAuthorityDomain::RequiredContext,
    ContinuationContextAuthorityDomain::Evidence,
    ContinuationContextAuthorityDomain::Check,
    ContinuationContextAuthorityDomain::SideEffect,
    ContinuationContextAuthorityDomain::WorkReport,
    ContinuationContextAuthorityDomain::AcceptedIntent,
    ContinuationContextAuthorityDomain::TypedHandoff,
    ContinuationContextAuthorityDomain::GitObservation,
    ContinuationContextAuthorityDomain::OperatingDocument,
    ContinuationContextAuthorityDomain::ProviderObservation,
    ContinuationContextAuthorityDomain::AgentAdvisory,
];

macro_rules! bounded_identifier {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            ///
            /// # Errors
            ///
            /// Returns an error when the value is empty, too long, or contains
            /// characters outside the canonical identifier character set.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier(stringify!($name), &value)?;
                Ok(Self(value))
            }

            /// Returns the canonical identifier text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
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

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorkflowOsError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

bounded_identifier!(
    /// Stable identity of one eligible continuation-context source.
    ContinuationContextSourceId
);
bounded_identifier!(
    /// Stable identity of one accepted-intent record.
    AcceptedIntentRecordId
);

/// Bounded semantic text used by non-authorizing context projections.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ContinuationContextText(String);

impl ContinuationContextText {
    /// Creates bounded semantic text.
    ///
    /// # Errors
    ///
    /// Returns an error when text is empty, oversized, or contains control
    /// characters other than newline and tab.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(context_error("text.empty", "context text cannot be empty"));
        }
        if value.len() > MAX_TEXT_BYTES {
            return Err(context_error(
                "text.too_long",
                "context text exceeds the bounded semantic-text limit",
            ));
        }
        if value
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
        {
            return Err(context_error(
                "text.control_character",
                "context text contains a disallowed control character",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the bounded text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ContinuationContextText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ContinuationContextText")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<ContinuationContextText> for String {
    fn from(value: ContinuationContextText) -> Self {
        value.0
    }
}

impl TryFrom<String> for ContinuationContextText {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Purpose for one bounded rehydration decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextPurpose {
    Planning,
    Implementation,
    ApprovalReview,
    ProviderReconciliation,
    ChildHandoff,
    MaterialContinuation,
}

/// Closed source classes eligible for the first model-only slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextSourceClass {
    ValidatedDeclaration,
    ImmutableRunBundle,
    DurableRunState,
    EventHistory,
    Policy,
    Approval,
    Capability,
    RequiredContext,
    Evidence,
    Check,
    SideEffect,
    WorkReport,
    AcceptedIntent,
    TypedHandoff,
    GitObservation,
    OperatingDocument,
    ProviderObservation,
    AgentAdvisory,
}

/// Authority domain carried by one source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextAuthorityDomain {
    Declaration,
    ImmutableRunInput,
    DurableRunState,
    EventHistory,
    Policy,
    Approval,
    Capability,
    RequiredContext,
    Evidence,
    Check,
    SideEffect,
    WorkReport,
    AcceptedIntent,
    TypedHandoff,
    GitObservation,
    OperatingDocument,
    ProviderObservation,
    AgentAdvisory,
}

/// Current source freshness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextFreshness {
    Current,
    StaleRefreshable,
    StaleBlocking,
    Superseded,
}

/// Current source integrity posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextIntegrity {
    Verified,
    Unverified,
    Invalid,
}

/// Current access posture for a private inventory source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextAccess {
    Permitted,
    Denied,
    Unknown,
}

/// Coarse sensitivity used before any consumer-visible projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextSensitivity {
    Public,
    Internal,
    Restricted,
}

/// Expected collector participation for one purpose.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextCollectorExpectation {
    Required,
    Optional,
    Inapplicable,
}

/// Observed collector outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextCollectorOutcome {
    Observed,
    Empty,
    SkippedOptional,
    AccessDenied,
    Failed,
    Unsupported,
    Inapplicable,
    IntentionallyOmitted,
}

/// Core-owned identity of one purpose- and scope-bound coverage profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationContextCoverageProfileId {
    PreRunPlanning,
    PreRunImplementation,
    PreRunApprovalReview,
    PreRunProviderReconciliation,
    PreRunChildHandoff,
    PreRunMaterialContinuation,
    RunBoundPlanning,
    RunBoundImplementation,
    RunBoundApprovalReview,
    RunBoundProviderReconciliation,
    RunBoundChildHandoff,
    RunBoundMaterialContinuation,
}

/// One declared collector-coverage entry.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextSourceCoverageEntry {
    pub source_class: ContinuationContextSourceClass,
    pub expectation: ContinuationContextCollectorExpectation,
    pub outcome: ContinuationContextCollectorOutcome,
}

/// Result of checking purpose-bound collector coverage.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextSourceCoverage {
    purpose: ContinuationContextPurpose,
    profile_id: ContinuationContextCoverageProfileId,
    run_bound: bool,
    entries: Vec<ContinuationContextSourceCoverageEntry>,
    complete: bool,
    degraded_optional: bool,
    commitment: SpecContentHash,
}

impl ContinuationContextSourceCoverage {
    #[must_use]
    pub const fn purpose(&self) -> ContinuationContextPurpose {
        self.purpose
    }

    #[must_use]
    const fn profile_id(&self) -> ContinuationContextCoverageProfileId {
        self.profile_id
    }

    #[must_use]
    pub const fn is_run_bound(&self) -> bool {
        self.run_bound
    }

    #[must_use]
    pub fn entries(&self) -> &[ContinuationContextSourceCoverageEntry] {
        &self.entries
    }

    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    #[must_use]
    pub const fn has_degraded_optional(&self) -> bool {
        self.degraded_optional
    }

    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }
}

/// Semantic claim kinds available to the first understanding projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationUnderstandingClaimKind {
    Objective,
    Priority,
    AcceptedAmendment,
    Scope,
    StrictNonGoal,
    Decision,
    RationaleReference,
    CompletedObligation,
    InProgressObligation,
    DeferredObligation,
    RemainingObligation,
    Blocker,
    RegisteredWait,
    OpenQuestion,
    Assumption,
    Uncertainty,
    EvidenceRequirement,
    CheckRequirement,
    SideEffectPosture,
    ValidationCommitment,
    ReportingCommitment,
    PromisedFollowUp,
    LawfulNextOption,
}

/// Core-owned identity of one purpose- and scope-bound semantic profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationContextSemanticProfileId {
    PreRunPlanning,
    PreRunImplementation,
    PreRunApprovalReview,
    PreRunProviderReconciliation,
    PreRunChildHandoff,
    PreRunMaterialContinuation,
    RunBoundPlanning,
    RunBoundImplementation,
    RunBoundApprovalReview,
    RunBoundProviderReconciliation,
    RunBoundChildHandoff,
    RunBoundMaterialContinuation,
}

/// Whether one claim needs a value, complete accounting, or neither.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationClaimRequirement {
    RequiredValue,
    RequiredAccounting,
    Optional,
    Inapplicable,
}

/// Core-owned value cardinality for one semantic claim kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationClaimCardinality {
    Singleton,
    Set,
}

/// Core-owned conflict and selection rule for one claim kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationClaimResolution {
    HighestAuthority,
}

/// Core-owned selection posture for one admitted source class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationSourceNecessity {
    Mandatory,
    Supplemental,
    AdvisoryOnly,
    Inapplicable,
}

/// One Core-owned semantic claim obligation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContinuationClaimObligation {
    kind: ContinuationUnderstandingClaimKind,
    requirement: ContinuationClaimRequirement,
    cardinality: ContinuationClaimCardinality,
    resolution: ContinuationClaimResolution,
    permitted_authority_domains: Vec<ContinuationContextAuthorityDomain>,
}

/// One Core-owned source-class obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContinuationSourceObligation {
    source_class: ContinuationContextSourceClass,
    necessity: ContinuationSourceNecessity,
}

/// Closed semantic contract derived only from purpose and validated scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContinuationContextSemanticProfile {
    profile_id: ContinuationContextSemanticProfileId,
    purpose: ContinuationContextPurpose,
    run_bound: bool,
    claim_obligations: Vec<ContinuationClaimObligation>,
    source_obligations: Vec<ContinuationSourceObligation>,
    commitment: SpecContentHash,
}

/// Typed claim-level accounting outcome supplied by an explicit collector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationClaimAccountingOutcome {
    Empty,
}

/// Claim-level accounting observation. It cannot change Core requirements.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationClaimAccountingEntry {
    pub claim_kind: ContinuationUnderstandingClaimKind,
    pub source_class: ContinuationContextSourceClass,
    pub outcome: ContinuationClaimAccountingOutcome,
}

/// One bounded semantic candidate asserted by a source.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationUnderstandingCandidate {
    pub kind: ContinuationUnderstandingClaimKind,
    pub value: ContinuationContextText,
}

/// Input definition for one eligible private inventory source.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextSourceDefinition {
    pub source_id: ContinuationContextSourceId,
    pub source_class: ContinuationContextSourceClass,
    pub authority_domain: ContinuationContextAuthorityDomain,
    pub revision: SpecContentHash,
    pub freshness: ContinuationContextFreshness,
    pub integrity: ContinuationContextIntegrity,
    pub access: ContinuationContextAccess,
    pub sensitivity: ContinuationContextSensitivity,
    pub materialized_bytes: u64,
    pub claims: Vec<ContinuationUnderstandingCandidate>,
}

/// Accepted user or delegated-maintainer intent kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptedIntentKind {
    Objective,
    Amendment,
    Priority,
    Pause,
    Stop,
    Resume,
}

/// Supersession posture for one accepted-intent record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptedIntentSupersession {
    Current,
    Superseded,
}

/// Scope of one continuation-context decision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextScope {
    pub project_id: ProjectId,
    pub workflow_id: Option<WorkflowId>,
    pub run_id: Option<WorkflowRunId>,
    pub phase: Option<ContinuationContextText>,
}

/// Typed semantic effect of accepted attributable direction.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptedIntentRecord {
    pub record_id: AcceptedIntentRecordId,
    pub directive_commitment: SpecContentHash,
    pub scope: ContinuationContextScope,
    pub kind: AcceptedIntentKind,
    pub meaning: ContinuationContextText,
    pub supersession: AcceptedIntentSupersession,
}

impl fmt::Debug for AcceptedIntentRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptedIntentRecord")
            .field("record_id", &"[REDACTED]")
            .field("directive_commitment", &"[REDACTED]")
            .field("scope", &"[REDACTED]")
            .field("kind", &self.kind)
            .field("meaning", &"[REDACTED]")
            .field("supersession", &self.supersession)
            .finish()
    }
}

/// Requested pure decision input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextDecisionInput {
    pub purpose: ContinuationContextPurpose,
    pub scope: ContinuationContextScope,
    pub actor: ActorId,
    pub maximum_sensitivity: ContinuationContextSensitivity,
    pub materialization_budget_bytes: u64,
    pub coverage: Vec<ContinuationContextSourceCoverageEntry>,
    pub claim_accounting: Vec<ContinuationClaimAccountingEntry>,
    pub sources: Vec<ContinuationContextSourceDefinition>,
    pub accepted_intents: Vec<AcceptedIntentRecord>,
}

/// Private inventory disposition for one source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextSourceDisposition {
    Selected,
    ReferenceOnly,
    OmittedBudget,
    StaleRefreshable,
    StaleBlocking,
    Superseded,
    Unauthorized,
    IntegrityUnverified,
    IntegrityInvalid,
}

/// One private inventory entry. It is not a consumer-visible projection.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextInventoryEntry {
    source_id: ContinuationContextSourceId,
    source_class: ContinuationContextSourceClass,
    authority_domain: ContinuationContextAuthorityDomain,
    revision: SpecContentHash,
    freshness: ContinuationContextFreshness,
    integrity: ContinuationContextIntegrity,
    access: ContinuationContextAccess,
    sensitivity: ContinuationContextSensitivity,
    necessity: ContinuationSourceNecessity,
    disposition: ContinuationContextSourceDisposition,
}

impl ContinuationContextInventoryEntry {
    #[must_use]
    pub const fn source_id(&self) -> &ContinuationContextSourceId {
        &self.source_id
    }

    #[must_use]
    pub const fn disposition(&self) -> ContinuationContextSourceDisposition {
        self.disposition
    }
}

impl fmt::Debug for ContinuationContextInventoryEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationContextInventoryEntry")
            .field("source_id", &"[REDACTED]")
            .field("source_class", &self.source_class)
            .field("authority_domain", &self.authority_domain)
            .field("revision", &"[REDACTED]")
            .field("freshness", &self.freshness)
            .field("integrity", &self.integrity)
            .field("access", &self.access)
            .field("sensitivity", &self.sensitivity)
            .field("necessity", &self.necessity)
            .field("disposition", &self.disposition)
            .finish()
    }
}

/// Complete private inventory used only by Core decisions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextInventory {
    algorithm: String,
    entries: Vec<ContinuationContextInventoryEntry>,
    commitment: SpecContentHash,
}

impl ContinuationContextInventory {
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    #[must_use]
    pub fn entries(&self) -> &[ContinuationContextInventoryEntry] {
        &self.entries
    }

    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }
}

/// Explicit reason that the requested context could not be complete.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextGapKind {
    CollectorUnavailable,
    SourceStale,
    SourceUnauthorized,
    SourceIntegrity,
    SourceOverBudget,
    ClaimAccountingMissing,
    ClaimAdvisoryOnly,
    ClaimUnknown,
    ClaimConflicting,
    ClaimWithheld,
}

/// Bounded gap record with no protected source locator or payload.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextGap {
    pub kind: ContinuationContextGapKind,
    pub claim_kind: Option<ContinuationUnderstandingClaimKind>,
}

/// Selected permitted context for one exact purpose. This is not authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextBundle {
    purpose: ContinuationContextPurpose,
    scope: ContinuationContextScope,
    actor: ActorId,
    selected_source_ids: Vec<ContinuationContextSourceId>,
    materialized_bytes: u64,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl ContinuationContextBundle {
    #[must_use]
    pub fn selected_source_ids(&self) -> &[ContinuationContextSourceId] {
        &self.selected_source_ids
    }

    #[must_use]
    pub const fn materialized_bytes(&self) -> u64 {
        self.materialized_bytes
    }

    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    #[must_use]
    pub const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }
}

/// Evidence posture for one derived understanding claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationUnderstandingClaimPosture {
    Authoritative,
    Supported,
    Advisory,
    AccountedEmpty,
    Conflicting,
    Unknown,
    Withheld,
}

/// One provenance-backed understanding claim.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationUnderstandingClaim {
    kind: ContinuationUnderstandingClaimKind,
    posture: ContinuationUnderstandingClaimPosture,
    value: Option<ContinuationContextText>,
    source_ids: Vec<ContinuationContextSourceId>,
}

impl ContinuationUnderstandingClaim {
    #[must_use]
    pub const fn kind(&self) -> ContinuationUnderstandingClaimKind {
        self.kind
    }

    #[must_use]
    pub const fn posture(&self) -> ContinuationUnderstandingClaimPosture {
        self.posture
    }

    #[must_use]
    pub const fn value(&self) -> Option<&ContinuationContextText> {
        self.value.as_ref()
    }

    #[must_use]
    pub fn source_ids(&self) -> &[ContinuationContextSourceId] {
        &self.source_ids
    }
}

impl fmt::Debug for ContinuationUnderstandingClaim {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationUnderstandingClaim")
            .field("kind", &self.kind)
            .field("posture", &self.posture)
            .field("value", &self.value.as_ref().map(|_| "[REDACTED]"))
            .field("source_count", &self.source_ids.len())
            .finish()
    }
}

/// Bounded understanding derived from selected authorized context.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationUnderstandingProjection {
    claims: Vec<ContinuationUnderstandingClaim>,
    commitment: SpecContentHash,
    non_authorizing: bool,
}

impl ContinuationUnderstandingProjection {
    #[must_use]
    pub fn claims(&self) -> &[ContinuationUnderstandingClaim] {
        &self.claims
    }

    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    #[must_use]
    pub const fn is_non_authorizing(&self) -> bool {
        self.non_authorizing
    }
}

/// Whether the pure context decision is ready for a later authority check.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationContextDecisionPosture {
    ReadyForAuthorityResolution,
    Blocked,
}

/// Pure deterministic result. It never authorizes execution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationContextDecision {
    coverage: ContinuationContextSourceCoverage,
    semantic_profile: ContinuationContextSemanticProfile,
    inventory: ContinuationContextInventory,
    bundle: ContinuationContextBundle,
    understanding: ContinuationUnderstandingProjection,
    gaps: Vec<ContinuationContextGap>,
    posture: ContinuationContextDecisionPosture,
    commitment: SpecContentHash,
}

impl ContinuationContextDecision {
    #[must_use]
    pub const fn coverage(&self) -> &ContinuationContextSourceCoverage {
        &self.coverage
    }

    #[must_use]
    pub const fn inventory(&self) -> &ContinuationContextInventory {
        &self.inventory
    }

    #[must_use]
    pub const fn bundle(&self) -> &ContinuationContextBundle {
        &self.bundle
    }

    #[must_use]
    pub const fn understanding(&self) -> &ContinuationUnderstandingProjection {
        &self.understanding
    }

    #[must_use]
    pub fn gaps(&self) -> &[ContinuationContextGap] {
        &self.gaps
    }

    #[must_use]
    pub const fn posture(&self) -> ContinuationContextDecisionPosture {
        self.posture
    }

    #[must_use]
    pub const fn commitment(&self) -> &SpecContentHash {
        &self.commitment
    }

    /// Always false: this result is orientation and admission posture only.
    #[must_use]
    pub const fn authorizes_execution() -> bool {
        false
    }
}

/// Derives private inventory, coverage, bundle, and bounded understanding from
/// explicit validated inputs.
///
/// # Errors
///
/// Returns a stable validation error for duplicate identities, missing coverage,
/// oversized inputs, scope mismatch, or non-canonical source definitions.
#[allow(clippy::too_many_lines)]
pub fn derive_continuation_context(
    mut input: ContinuationContextDecisionInput,
) -> Result<ContinuationContextDecision, WorkflowOsError> {
    validate_input_limits(&input)?;
    input.coverage.sort();
    input
        .sources
        .sort_by(|left, right| left.source_id.cmp(&right.source_id));
    input
        .accepted_intents
        .sort_by(|left, right| left.record_id.cmp(&right.record_id));
    input.claim_accounting.sort();
    reject_duplicate_coverage(&input.coverage)?;
    reject_duplicate_sources(&input.sources)?;
    reject_duplicate_intents(&input.accepted_intents)?;
    reject_duplicate_claim_accounting(&input.claim_accounting)?;
    validate_scope(&input.scope)?;
    let run_bound = input.scope.run_id.is_some();
    let coverage_profile = coverage_profile_id(input.purpose, run_bound);
    let semantic_profile = derive_semantic_profile(input.purpose, run_bound)?;
    validate_source_coverage(&input)?;
    validate_coverage_expectations(coverage_profile, &input.coverage)?;
    validate_expectation_outcome_compatibility(&input.coverage)?;
    validate_coverage_outcomes(&input)?;
    validate_source_claims(&input.scope, &input.sources)?;
    validate_intent_scope(&input.scope, &input.accepted_intents)?;
    validate_claim_accounting(&input, &semantic_profile)?;

    let coverage = derive_coverage(input.purpose, coverage_profile, run_bound, &input.coverage)?;
    let mut sources = input.sources;
    sources.extend(intent_sources(&input.accepted_intents)?);
    sources.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    reject_duplicate_sources(&sources)?;
    validate_semantic_sources(&semantic_profile, &sources)?;

    let mut selection_order = sources.iter().collect::<Vec<_>>();
    selection_order.sort_by(|left, right| {
        source_necessity(&semantic_profile, left.source_class)
            .cmp(&source_necessity(&semantic_profile, right.source_class))
            .then_with(|| left.source_id.cmp(&right.source_id))
    });

    let mut remaining_budget = input.materialization_budget_bytes;
    let mut inventory_entries = Vec::with_capacity(sources.len());
    let mut selected = Vec::new();
    let mut gaps = Vec::new();
    let mut withheld_claims = BTreeSet::new();

    for source in selection_order {
        let disposition = source_disposition(source, input.maximum_sensitivity, remaining_budget);
        if disposition == ContinuationContextSourceDisposition::Selected {
            remaining_budget = remaining_budget.saturating_sub(source.materialized_bytes);
            selected.push(source.clone());
        } else if disposition == ContinuationContextSourceDisposition::Unauthorized {
            withheld_claims.extend(source.claims.iter().map(|claim| claim.kind));
        }
        if source_necessity(&semantic_profile, source.source_class)
            == ContinuationSourceNecessity::Mandatory
        {
            if let Some(kind) = source_gap(disposition) {
                gaps.push(ContinuationContextGap {
                    kind,
                    claim_kind: None,
                });
            }
        }
        inventory_entries.push(inventory_entry(
            source,
            source_necessity(&semantic_profile, source.source_class),
            disposition,
        ));
    }
    inventory_entries.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    selected.sort_by(|left, right| left.source_id.cmp(&right.source_id));

    if !coverage.is_complete() {
        gaps.push(ContinuationContextGap {
            kind: ContinuationContextGapKind::CollectorUnavailable,
            claim_kind: None,
        });
    }

    let inventory_commitment = canonical_commitment(
        "private-inventory",
        &(semantic_profile.commitment.clone(), &inventory_entries),
    )?;
    let inventory = ContinuationContextInventory {
        algorithm: CONTEXT_ALGORITHM.to_owned(),
        entries: inventory_entries,
        commitment: inventory_commitment,
    };

    let selected_source_ids = selected
        .iter()
        .map(|source| source.source_id.clone())
        .collect::<Vec<_>>();
    let materialized_bytes = selected
        .iter()
        .map(|source| source.materialized_bytes)
        .sum();
    let bundle_commitment = canonical_commitment(
        "context-bundle",
        &(
            input.purpose,
            &input.scope,
            &input.actor,
            input.maximum_sensitivity,
            semantic_profile.commitment.clone(),
            &input.claim_accounting,
            &selected_source_ids,
            materialized_bytes,
            &selected,
        ),
    )?;
    let bundle = ContinuationContextBundle {
        purpose: input.purpose,
        scope: input.scope,
        actor: input.actor,
        selected_source_ids,
        materialized_bytes,
        commitment: bundle_commitment,
        non_authorizing: true,
    };

    let claims = derive_claims(
        &selected,
        &semantic_profile,
        &input.claim_accounting,
        &input.coverage,
        &withheld_claims,
        run_bound,
        &mut gaps,
    );
    gaps.sort();
    gaps.dedup();
    let understanding_commitment = canonical_commitment(
        "understanding",
        &(
            semantic_profile.commitment.clone(),
            bundle.commitment(),
            &claims,
        ),
    )?;
    let understanding = ContinuationUnderstandingProjection {
        claims,
        commitment: understanding_commitment,
        non_authorizing: true,
    };
    let posture = if gaps.is_empty() {
        ContinuationContextDecisionPosture::ReadyForAuthorityResolution
    } else {
        ContinuationContextDecisionPosture::Blocked
    };
    let commitment = canonical_commitment(
        "decision",
        &(
            coverage.commitment(),
            semantic_profile.commitment.clone(),
            inventory.commitment(),
            bundle.commitment(),
            understanding.commitment(),
            &gaps,
            posture,
        ),
    )?;

    Ok(ContinuationContextDecision {
        coverage,
        semantic_profile,
        inventory,
        bundle,
        understanding,
        gaps,
        posture,
        commitment,
    })
}

fn validate_input_limits(input: &ContinuationContextDecisionInput) -> Result<(), WorkflowOsError> {
    if input.materialization_budget_bytes == 0 {
        return Err(context_error(
            "budget.empty",
            "materialization budget must be greater than zero",
        ));
    }
    if input
        .sources
        .len()
        .saturating_add(input.accepted_intents.len())
        > MAX_SOURCES
    {
        return Err(context_error(
            "source.too_many",
            "context source count exceeds the model limit",
        ));
    }
    if input.coverage.len() > MAX_COVERAGE_ENTRIES {
        return Err(context_error(
            "coverage.too_many",
            "collector coverage count exceeds the model limit",
        ));
    }
    if input.claim_accounting.len() > MAX_ACCOUNTING_ENTRIES {
        return Err(context_error(
            "accounting.too_many",
            "claim accounting count exceeds the model limit",
        ));
    }
    Ok(())
}

fn reject_duplicate_coverage(
    coverage: &[ContinuationContextSourceCoverageEntry],
) -> Result<(), WorkflowOsError> {
    if coverage
        .windows(2)
        .any(|entries| entries[0].source_class == entries[1].source_class)
    {
        return Err(context_error(
            "coverage.duplicate",
            "collector coverage contains a duplicate source class",
        ));
    }
    Ok(())
}

fn reject_duplicate_sources(
    sources: &[ContinuationContextSourceDefinition],
) -> Result<(), WorkflowOsError> {
    if sources
        .windows(2)
        .any(|entries| entries[0].source_id == entries[1].source_id)
    {
        return Err(context_error(
            "source.duplicate",
            "context sources contain a duplicate identity",
        ));
    }
    Ok(())
}

fn reject_duplicate_intents(intents: &[AcceptedIntentRecord]) -> Result<(), WorkflowOsError> {
    if intents
        .windows(2)
        .any(|entries| entries[0].record_id == entries[1].record_id)
    {
        return Err(context_error(
            "intent.duplicate",
            "accepted intent contains a duplicate identity",
        ));
    }
    Ok(())
}

fn reject_duplicate_claim_accounting(
    accounting: &[ContinuationClaimAccountingEntry],
) -> Result<(), WorkflowOsError> {
    if accounting.windows(2).any(|entries| {
        entries[0].claim_kind == entries[1].claim_kind
            && entries[0].source_class == entries[1].source_class
    }) {
        return Err(context_error(
            "accounting.duplicate",
            "claim accounting contains a duplicate claim and source class",
        ));
    }
    Ok(())
}

fn validate_source_coverage(
    input: &ContinuationContextDecisionInput,
) -> Result<(), WorkflowOsError> {
    let declared = input
        .coverage
        .iter()
        .map(|entry| entry.source_class)
        .collect::<BTreeSet<_>>();
    if ALL_CONTEXT_SOURCE_CLASSES
        .iter()
        .any(|source_class| !declared.contains(source_class))
    {
        return Err(context_error(
            "coverage.class_unaccounted",
            "collector coverage does not account for every closed source class",
        ));
    }
    if input
        .sources
        .iter()
        .any(|source| !declared.contains(&source.source_class))
    {
        return Err(context_error(
            "coverage.source_unaccounted",
            "a context source has no collector coverage entry",
        ));
    }
    if !input.accepted_intents.is_empty()
        && !declared.contains(&ContinuationContextSourceClass::AcceptedIntent)
    {
        return Err(context_error(
            "coverage.intent_unaccounted",
            "accepted intent has no collector coverage entry",
        ));
    }
    Ok(())
}

fn validate_coverage_expectations(
    profile_id: ContinuationContextCoverageProfileId,
    coverage: &[ContinuationContextSourceCoverageEntry],
) -> Result<(), WorkflowOsError> {
    for entry in coverage {
        if entry.expectation != coverage_profile_expectation(profile_id, entry.source_class) {
            return Err(context_error(
                "coverage.expectation_mismatch",
                "collector expectation does not match the Core-owned coverage profile",
            ));
        }
    }
    Ok(())
}

fn validate_expectation_outcome_compatibility(
    coverage: &[ContinuationContextSourceCoverageEntry],
) -> Result<(), WorkflowOsError> {
    for entry in coverage {
        let incompatible = match entry.expectation {
            ContinuationContextCollectorExpectation::Required => matches!(
                entry.outcome,
                ContinuationContextCollectorOutcome::SkippedOptional
                    | ContinuationContextCollectorOutcome::Inapplicable
            ),
            ContinuationContextCollectorExpectation::Optional => {
                entry.outcome == ContinuationContextCollectorOutcome::Inapplicable
            }
            ContinuationContextCollectorExpectation::Inapplicable => {
                entry.outcome != ContinuationContextCollectorOutcome::Inapplicable
            }
        };
        if incompatible {
            return Err(context_error(
                "coverage.outcome_expectation_mismatch",
                "collector outcome is incompatible with the Core-owned expectation",
            ));
        }
    }
    Ok(())
}

fn validate_coverage_outcomes(
    input: &ContinuationContextDecisionInput,
) -> Result<(), WorkflowOsError> {
    let mut observed = BTreeMap::<ContinuationContextSourceClass, usize>::new();
    for source in &input.sources {
        *observed.entry(source.source_class).or_default() += 1;
    }
    if !input.accepted_intents.is_empty() {
        observed.insert(
            ContinuationContextSourceClass::AcceptedIntent,
            input.accepted_intents.len(),
        );
    }

    for entry in &input.coverage {
        let observed_count = observed
            .get(&entry.source_class)
            .copied()
            .unwrap_or_default();
        match entry.outcome {
            ContinuationContextCollectorOutcome::Observed if observed_count == 0 => {
                return Err(context_error(
                    "coverage.observed_without_source",
                    "collector coverage reports observed context without a supplied source",
                ));
            }
            ContinuationContextCollectorOutcome::Observed => {}
            ContinuationContextCollectorOutcome::Empty if observed_count == 0 => {}
            _ if observed_count > 0 => {
                return Err(context_error(
                    "coverage.outcome_contradicts_source",
                    "collector coverage outcome contradicts a supplied context source",
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_source_claims(
    scope: &ContinuationContextScope,
    sources: &[ContinuationContextSourceDefinition],
) -> Result<(), WorkflowOsError> {
    for source in sources {
        if source.authority_domain != authority_domain_for_source_class(source.source_class) {
            return Err(context_error(
                "source.authority_domain_mismatch",
                "context source authority domain does not match its source class",
            ));
        }
        if source.materialized_bytes == 0 {
            return Err(context_error(
                "source.size_empty",
                "context source size must be greater than zero",
            ));
        }
        if source.claims.len() > MAX_CLAIMS_PER_SOURCE {
            return Err(context_error(
                "source.claims_too_many",
                "context source claim count exceeds the model limit",
            ));
        }
        let mut claims = source.claims.clone();
        claims.sort();
        if claims.windows(2).any(|items| items[0] == items[1]) {
            return Err(context_error(
                "source.claim_duplicate",
                "context source contains a duplicate understanding claim",
            ));
        }
        if source.claims.iter().any(|claim| {
            authority_rank(claim.kind, source.authority_domain, scope.run_id.is_some()).is_none()
        }) {
            return Err(context_error(
                "source.claim_authority_invalid",
                "context source authority domain cannot support one declared claim",
            ));
        }
    }
    Ok(())
}

const fn authority_domain_for_source_class(
    source_class: ContinuationContextSourceClass,
) -> ContinuationContextAuthorityDomain {
    match source_class {
        ContinuationContextSourceClass::ValidatedDeclaration => {
            ContinuationContextAuthorityDomain::Declaration
        }
        ContinuationContextSourceClass::ImmutableRunBundle => {
            ContinuationContextAuthorityDomain::ImmutableRunInput
        }
        ContinuationContextSourceClass::DurableRunState => {
            ContinuationContextAuthorityDomain::DurableRunState
        }
        ContinuationContextSourceClass::EventHistory => {
            ContinuationContextAuthorityDomain::EventHistory
        }
        ContinuationContextSourceClass::Policy => ContinuationContextAuthorityDomain::Policy,
        ContinuationContextSourceClass::Approval => ContinuationContextAuthorityDomain::Approval,
        ContinuationContextSourceClass::Capability => {
            ContinuationContextAuthorityDomain::Capability
        }
        ContinuationContextSourceClass::RequiredContext => {
            ContinuationContextAuthorityDomain::RequiredContext
        }
        ContinuationContextSourceClass::Evidence => ContinuationContextAuthorityDomain::Evidence,
        ContinuationContextSourceClass::Check => ContinuationContextAuthorityDomain::Check,
        ContinuationContextSourceClass::SideEffect => {
            ContinuationContextAuthorityDomain::SideEffect
        }
        ContinuationContextSourceClass::WorkReport => {
            ContinuationContextAuthorityDomain::WorkReport
        }
        ContinuationContextSourceClass::AcceptedIntent => {
            ContinuationContextAuthorityDomain::AcceptedIntent
        }
        ContinuationContextSourceClass::TypedHandoff => {
            ContinuationContextAuthorityDomain::TypedHandoff
        }
        ContinuationContextSourceClass::GitObservation => {
            ContinuationContextAuthorityDomain::GitObservation
        }
        ContinuationContextSourceClass::OperatingDocument => {
            ContinuationContextAuthorityDomain::OperatingDocument
        }
        ContinuationContextSourceClass::ProviderObservation => {
            ContinuationContextAuthorityDomain::ProviderObservation
        }
        ContinuationContextSourceClass::AgentAdvisory => {
            ContinuationContextAuthorityDomain::AgentAdvisory
        }
    }
}

fn validate_intent_scope(
    scope: &ContinuationContextScope,
    intents: &[AcceptedIntentRecord],
) -> Result<(), WorkflowOsError> {
    if intents.iter().any(|intent| &intent.scope != scope) {
        return Err(context_error(
            "intent.scope_mismatch",
            "accepted intent scope does not match the decision scope",
        ));
    }
    Ok(())
}

fn validate_scope(scope: &ContinuationContextScope) -> Result<(), WorkflowOsError> {
    if scope.run_id.is_some() && scope.workflow_id.is_none() {
        return Err(context_error(
            "scope.run_without_workflow",
            "run-bound continuation context requires a workflow identity",
        ));
    }
    Ok(())
}

fn derive_coverage(
    purpose: ContinuationContextPurpose,
    profile_id: ContinuationContextCoverageProfileId,
    run_bound: bool,
    entries: &[ContinuationContextSourceCoverageEntry],
) -> Result<ContinuationContextSourceCoverage, WorkflowOsError> {
    let complete = entries.iter().all(|entry| match entry.expectation {
        ContinuationContextCollectorExpectation::Required => matches!(
            entry.outcome,
            ContinuationContextCollectorOutcome::Observed
                | ContinuationContextCollectorOutcome::Empty
        ),
        ContinuationContextCollectorExpectation::Optional => true,
        ContinuationContextCollectorExpectation::Inapplicable => {
            entry.outcome == ContinuationContextCollectorOutcome::Inapplicable
        }
    });
    let degraded_optional = entries.iter().any(|entry| {
        entry.expectation == ContinuationContextCollectorExpectation::Optional
            && !matches!(
                entry.outcome,
                ContinuationContextCollectorOutcome::Observed
                    | ContinuationContextCollectorOutcome::Empty
                    | ContinuationContextCollectorOutcome::SkippedOptional
                    | ContinuationContextCollectorOutcome::IntentionallyOmitted
                    | ContinuationContextCollectorOutcome::Inapplicable
            )
    });
    Ok(ContinuationContextSourceCoverage {
        purpose,
        profile_id,
        run_bound,
        entries: entries.to_vec(),
        complete,
        degraded_optional,
        commitment: canonical_commitment(
            "source-coverage",
            &(purpose, profile_id, run_bound, entries),
        )?,
    })
}

const fn coverage_profile_id(
    purpose: ContinuationContextPurpose,
    run_bound: bool,
) -> ContinuationContextCoverageProfileId {
    match (purpose, run_bound) {
        (ContinuationContextPurpose::Planning, false) => {
            ContinuationContextCoverageProfileId::PreRunPlanning
        }
        (ContinuationContextPurpose::Implementation, false) => {
            ContinuationContextCoverageProfileId::PreRunImplementation
        }
        (ContinuationContextPurpose::ApprovalReview, false) => {
            ContinuationContextCoverageProfileId::PreRunApprovalReview
        }
        (ContinuationContextPurpose::ProviderReconciliation, false) => {
            ContinuationContextCoverageProfileId::PreRunProviderReconciliation
        }
        (ContinuationContextPurpose::ChildHandoff, false) => {
            ContinuationContextCoverageProfileId::PreRunChildHandoff
        }
        (ContinuationContextPurpose::MaterialContinuation, false) => {
            ContinuationContextCoverageProfileId::PreRunMaterialContinuation
        }
        (ContinuationContextPurpose::Planning, true) => {
            ContinuationContextCoverageProfileId::RunBoundPlanning
        }
        (ContinuationContextPurpose::Implementation, true) => {
            ContinuationContextCoverageProfileId::RunBoundImplementation
        }
        (ContinuationContextPurpose::ApprovalReview, true) => {
            ContinuationContextCoverageProfileId::RunBoundApprovalReview
        }
        (ContinuationContextPurpose::ProviderReconciliation, true) => {
            ContinuationContextCoverageProfileId::RunBoundProviderReconciliation
        }
        (ContinuationContextPurpose::ChildHandoff, true) => {
            ContinuationContextCoverageProfileId::RunBoundChildHandoff
        }
        (ContinuationContextPurpose::MaterialContinuation, true) => {
            ContinuationContextCoverageProfileId::RunBoundMaterialContinuation
        }
    }
}

const fn coverage_profile_expectation(
    profile_id: ContinuationContextCoverageProfileId,
    source_class: ContinuationContextSourceClass,
) -> ContinuationContextCollectorExpectation {
    if is_run_bound_profile(profile_id) {
        return match source_class {
            ContinuationContextSourceClass::AgentAdvisory => {
                ContinuationContextCollectorExpectation::Optional
            }
            ContinuationContextSourceClass::ProviderObservation
                if !matches!(
                    profile_id,
                    ContinuationContextCoverageProfileId::RunBoundProviderReconciliation
                ) =>
            {
                ContinuationContextCollectorExpectation::Optional
            }
            _ => ContinuationContextCollectorExpectation::Required,
        };
    }

    match source_class {
        ContinuationContextSourceClass::ValidatedDeclaration
        | ContinuationContextSourceClass::Policy
        | ContinuationContextSourceClass::RequiredContext
        | ContinuationContextSourceClass::AcceptedIntent
        | ContinuationContextSourceClass::GitObservation
        | ContinuationContextSourceClass::OperatingDocument => {
            ContinuationContextCollectorExpectation::Required
        }
        ContinuationContextSourceClass::Evidence
        | ContinuationContextSourceClass::Check
        | ContinuationContextSourceClass::WorkReport
        | ContinuationContextSourceClass::TypedHandoff
        | ContinuationContextSourceClass::ProviderObservation
        | ContinuationContextSourceClass::AgentAdvisory => {
            ContinuationContextCollectorExpectation::Optional
        }
        ContinuationContextSourceClass::ImmutableRunBundle
        | ContinuationContextSourceClass::DurableRunState
        | ContinuationContextSourceClass::EventHistory
        | ContinuationContextSourceClass::Approval
        | ContinuationContextSourceClass::Capability
        | ContinuationContextSourceClass::SideEffect => {
            ContinuationContextCollectorExpectation::Inapplicable
        }
    }
}

const fn is_run_bound_profile(profile_id: ContinuationContextCoverageProfileId) -> bool {
    matches!(
        profile_id,
        ContinuationContextCoverageProfileId::RunBoundPlanning
            | ContinuationContextCoverageProfileId::RunBoundImplementation
            | ContinuationContextCoverageProfileId::RunBoundApprovalReview
            | ContinuationContextCoverageProfileId::RunBoundProviderReconciliation
            | ContinuationContextCoverageProfileId::RunBoundChildHandoff
            | ContinuationContextCoverageProfileId::RunBoundMaterialContinuation
    )
}

const fn semantic_profile_id(
    purpose: ContinuationContextPurpose,
    run_bound: bool,
) -> ContinuationContextSemanticProfileId {
    match (purpose, run_bound) {
        (ContinuationContextPurpose::Planning, false) => {
            ContinuationContextSemanticProfileId::PreRunPlanning
        }
        (ContinuationContextPurpose::Implementation, false) => {
            ContinuationContextSemanticProfileId::PreRunImplementation
        }
        (ContinuationContextPurpose::ApprovalReview, false) => {
            ContinuationContextSemanticProfileId::PreRunApprovalReview
        }
        (ContinuationContextPurpose::ProviderReconciliation, false) => {
            ContinuationContextSemanticProfileId::PreRunProviderReconciliation
        }
        (ContinuationContextPurpose::ChildHandoff, false) => {
            ContinuationContextSemanticProfileId::PreRunChildHandoff
        }
        (ContinuationContextPurpose::MaterialContinuation, false) => {
            ContinuationContextSemanticProfileId::PreRunMaterialContinuation
        }
        (ContinuationContextPurpose::Planning, true) => {
            ContinuationContextSemanticProfileId::RunBoundPlanning
        }
        (ContinuationContextPurpose::Implementation, true) => {
            ContinuationContextSemanticProfileId::RunBoundImplementation
        }
        (ContinuationContextPurpose::ApprovalReview, true) => {
            ContinuationContextSemanticProfileId::RunBoundApprovalReview
        }
        (ContinuationContextPurpose::ProviderReconciliation, true) => {
            ContinuationContextSemanticProfileId::RunBoundProviderReconciliation
        }
        (ContinuationContextPurpose::ChildHandoff, true) => {
            ContinuationContextSemanticProfileId::RunBoundChildHandoff
        }
        (ContinuationContextPurpose::MaterialContinuation, true) => {
            ContinuationContextSemanticProfileId::RunBoundMaterialContinuation
        }
    }
}

const fn claim_requirement(
    profile_id: ContinuationContextSemanticProfileId,
    kind: ContinuationUnderstandingClaimKind,
) -> ContinuationClaimRequirement {
    use ContinuationClaimRequirement::{RequiredAccounting, RequiredValue};
    use ContinuationContextPurpose as Purpose;
    use ContinuationUnderstandingClaimKind as Claim;

    let purpose = semantic_profile_purpose(profile_id);
    let run_bound = semantic_profile_is_run_bound(profile_id);
    match kind {
        Claim::Objective | Claim::Scope | Claim::LawfulNextOption => RequiredValue,
        Claim::RemainingObligation
            if (run_bound && !matches!(purpose, Purpose::ApprovalReview))
                || (!run_bound
                    && matches!(
                        purpose,
                        Purpose::Implementation
                            | Purpose::ChildHandoff
                            | Purpose::MaterialContinuation
                    )) =>
        {
            RequiredValue
        }
        Claim::EvidenceRequirement
            if matches!(
                purpose,
                Purpose::ApprovalReview | Purpose::ProviderReconciliation
            ) || (run_bound
                && matches!(
                    purpose,
                    Purpose::Implementation | Purpose::ChildHandoff | Purpose::MaterialContinuation
                )) =>
        {
            RequiredValue
        }
        Claim::CheckRequirement
            if matches!(
                purpose,
                Purpose::Implementation | Purpose::ApprovalReview | Purpose::MaterialContinuation
            ) || (run_bound && matches!(purpose, Purpose::ChildHandoff)) =>
        {
            RequiredValue
        }
        Claim::SideEffectPosture
            if matches!(
                purpose,
                Purpose::Implementation
                    | Purpose::ApprovalReview
                    | Purpose::ProviderReconciliation
                    | Purpose::MaterialContinuation
            ) || (run_bound && matches!(purpose, Purpose::ChildHandoff)) =>
        {
            RequiredValue
        }
        Claim::ValidationCommitment | Claim::ReportingCommitment
            if matches!(
                purpose,
                Purpose::Implementation
                    | Purpose::ApprovalReview
                    | Purpose::ChildHandoff
                    | Purpose::MaterialContinuation
            ) =>
        {
            RequiredValue
        }
        _ => RequiredAccounting,
    }
}

const fn claim_cardinality(
    kind: ContinuationUnderstandingClaimKind,
) -> ContinuationClaimCardinality {
    match kind {
        ContinuationUnderstandingClaimKind::Objective
        | ContinuationUnderstandingClaimKind::Priority
        | ContinuationUnderstandingClaimKind::Scope => ContinuationClaimCardinality::Singleton,
        _ => ContinuationClaimCardinality::Set,
    }
}

const fn semantic_profile_purpose(
    profile_id: ContinuationContextSemanticProfileId,
) -> ContinuationContextPurpose {
    match profile_id {
        ContinuationContextSemanticProfileId::PreRunPlanning
        | ContinuationContextSemanticProfileId::RunBoundPlanning => {
            ContinuationContextPurpose::Planning
        }
        ContinuationContextSemanticProfileId::PreRunImplementation
        | ContinuationContextSemanticProfileId::RunBoundImplementation => {
            ContinuationContextPurpose::Implementation
        }
        ContinuationContextSemanticProfileId::PreRunApprovalReview
        | ContinuationContextSemanticProfileId::RunBoundApprovalReview => {
            ContinuationContextPurpose::ApprovalReview
        }
        ContinuationContextSemanticProfileId::PreRunProviderReconciliation
        | ContinuationContextSemanticProfileId::RunBoundProviderReconciliation => {
            ContinuationContextPurpose::ProviderReconciliation
        }
        ContinuationContextSemanticProfileId::PreRunChildHandoff
        | ContinuationContextSemanticProfileId::RunBoundChildHandoff => {
            ContinuationContextPurpose::ChildHandoff
        }
        ContinuationContextSemanticProfileId::PreRunMaterialContinuation
        | ContinuationContextSemanticProfileId::RunBoundMaterialContinuation => {
            ContinuationContextPurpose::MaterialContinuation
        }
    }
}

const fn semantic_profile_is_run_bound(profile_id: ContinuationContextSemanticProfileId) -> bool {
    matches!(
        profile_id,
        ContinuationContextSemanticProfileId::RunBoundPlanning
            | ContinuationContextSemanticProfileId::RunBoundImplementation
            | ContinuationContextSemanticProfileId::RunBoundApprovalReview
            | ContinuationContextSemanticProfileId::RunBoundProviderReconciliation
            | ContinuationContextSemanticProfileId::RunBoundChildHandoff
            | ContinuationContextSemanticProfileId::RunBoundMaterialContinuation
    )
}

const fn semantic_source_necessity(
    profile_id: ContinuationContextSemanticProfileId,
    source_class: ContinuationContextSourceClass,
) -> ContinuationSourceNecessity {
    if matches!(source_class, ContinuationContextSourceClass::AgentAdvisory) {
        return ContinuationSourceNecessity::AdvisoryOnly;
    }
    let expectation = coverage_profile_expectation(
        coverage_profile_id(
            semantic_profile_purpose(profile_id),
            semantic_profile_is_run_bound(profile_id),
        ),
        source_class,
    );
    match expectation {
        ContinuationContextCollectorExpectation::Required => ContinuationSourceNecessity::Mandatory,
        ContinuationContextCollectorExpectation::Optional => {
            ContinuationSourceNecessity::Supplemental
        }
        ContinuationContextCollectorExpectation::Inapplicable => {
            ContinuationSourceNecessity::Inapplicable
        }
    }
}

fn derive_semantic_profile(
    purpose: ContinuationContextPurpose,
    run_bound: bool,
) -> Result<ContinuationContextSemanticProfile, WorkflowOsError> {
    let profile_id = semantic_profile_id(purpose, run_bound);
    let claim_obligations = ALL_UNDERSTANDING_CLAIM_KINDS
        .iter()
        .map(|kind| ContinuationClaimObligation {
            kind: *kind,
            requirement: claim_requirement(profile_id, *kind),
            cardinality: claim_cardinality(*kind),
            resolution: ContinuationClaimResolution::HighestAuthority,
            permitted_authority_domains: ALL_CONTEXT_AUTHORITY_DOMAINS
                .iter()
                .copied()
                .filter(|domain| authority_rank(*kind, *domain, run_bound).is_some())
                .collect(),
        })
        .collect::<Vec<_>>();
    let source_obligations = ALL_CONTEXT_SOURCE_CLASSES
        .iter()
        .map(|source_class| ContinuationSourceObligation {
            source_class: *source_class,
            necessity: semantic_source_necessity(profile_id, *source_class),
        })
        .collect::<Vec<_>>();
    let commitment = canonical_commitment(
        "semantic-profile",
        &(
            profile_id,
            purpose,
            run_bound,
            &claim_obligations,
            &source_obligations,
        ),
    )?;
    Ok(ContinuationContextSemanticProfile {
        profile_id,
        purpose,
        run_bound,
        claim_obligations,
        source_obligations,
        commitment,
    })
}

fn source_necessity(
    profile: &ContinuationContextSemanticProfile,
    source_class: ContinuationContextSourceClass,
) -> ContinuationSourceNecessity {
    profile
        .source_obligations
        .iter()
        .find(|obligation| obligation.source_class == source_class)
        .map_or(ContinuationSourceNecessity::Inapplicable, |obligation| {
            obligation.necessity
        })
}

fn claim_obligation(
    profile: &ContinuationContextSemanticProfile,
    kind: ContinuationUnderstandingClaimKind,
) -> ContinuationClaimObligation {
    profile
        .claim_obligations
        .iter()
        .find(|obligation| obligation.kind == kind)
        .cloned()
        .unwrap_or(ContinuationClaimObligation {
            kind,
            requirement: ContinuationClaimRequirement::Inapplicable,
            cardinality: ContinuationClaimCardinality::Set,
            resolution: ContinuationClaimResolution::HighestAuthority,
            permitted_authority_domains: Vec::new(),
        })
}

fn validate_semantic_sources(
    profile: &ContinuationContextSemanticProfile,
    sources: &[ContinuationContextSourceDefinition],
) -> Result<(), WorkflowOsError> {
    if sources.iter().any(|source| {
        source_necessity(profile, source.source_class) == ContinuationSourceNecessity::Inapplicable
    }) {
        return Err(context_error(
            "source.semantic_inapplicable",
            "context source class is inapplicable to the Core-owned semantic profile",
        ));
    }
    Ok(())
}

fn best_accounting_source_classes(
    profile: &ContinuationContextSemanticProfile,
    kind: ContinuationUnderstandingClaimKind,
) -> Vec<ContinuationContextSourceClass> {
    let mut ranked = profile
        .source_obligations
        .iter()
        .filter(|obligation| obligation.necessity == ContinuationSourceNecessity::Mandatory)
        .filter_map(|obligation| {
            authority_rank(
                kind,
                authority_domain_for_source_class(obligation.source_class),
                profile.run_bound,
            )
            .map(|rank| (rank, obligation.source_class))
        })
        .collect::<Vec<_>>();
    ranked.sort();
    let Some(best_rank) = ranked.first().map(|(rank, _)| *rank) else {
        return Vec::new();
    };
    ranked
        .into_iter()
        .take_while(|(rank, _)| *rank == best_rank)
        .map(|(_, source_class)| source_class)
        .collect()
}

fn validate_claim_accounting(
    input: &ContinuationContextDecisionInput,
    profile: &ContinuationContextSemanticProfile,
) -> Result<(), WorkflowOsError> {
    for entry in &input.claim_accounting {
        if claim_obligation(profile, entry.claim_kind).requirement
            == ContinuationClaimRequirement::Inapplicable
        {
            return Err(context_error(
                "accounting.claim_inapplicable",
                "claim accounting targets an inapplicable claim kind",
            ));
        }
        if !best_accounting_source_classes(profile, entry.claim_kind).contains(&entry.source_class)
        {
            return Err(context_error(
                "accounting.source_not_authoritative",
                "claim accounting source is not a Core-selected accounting source",
            ));
        }
        if !input
            .coverage
            .iter()
            .any(|coverage| coverage.source_class == entry.source_class)
        {
            return Err(context_error(
                "accounting.coverage_missing",
                "claim accounting source has no collector coverage entry",
            ));
        }
    }
    Ok(())
}

fn claim_accounting_complete(
    profile: &ContinuationContextSemanticProfile,
    accounting: &[ContinuationClaimAccountingEntry],
    coverage: &[ContinuationContextSourceCoverageEntry],
    kind: ContinuationUnderstandingClaimKind,
) -> bool {
    let expected = best_accounting_source_classes(profile, kind);
    !expected.is_empty()
        && expected.iter().all(|source_class| {
            accounting.iter().any(|entry| {
                entry.claim_kind == kind
                    && entry.source_class == *source_class
                    && entry.outcome == ContinuationClaimAccountingOutcome::Empty
                    && coverage.iter().any(|coverage| {
                        coverage.source_class == *source_class
                            && matches!(
                                coverage.outcome,
                                ContinuationContextCollectorOutcome::Observed
                                    | ContinuationContextCollectorOutcome::Empty
                            )
                    })
            })
        })
}

fn intent_sources(
    intents: &[AcceptedIntentRecord],
) -> Result<Vec<ContinuationContextSourceDefinition>, WorkflowOsError> {
    intents
        .iter()
        .filter(|intent| intent.supersession == AcceptedIntentSupersession::Current)
        .map(|intent| {
            let kind = match intent.kind {
                AcceptedIntentKind::Objective => ContinuationUnderstandingClaimKind::Objective,
                AcceptedIntentKind::Amendment => {
                    ContinuationUnderstandingClaimKind::AcceptedAmendment
                }
                AcceptedIntentKind::Priority => ContinuationUnderstandingClaimKind::Priority,
                AcceptedIntentKind::Pause
                | AcceptedIntentKind::Stop
                | AcceptedIntentKind::Resume => {
                    ContinuationUnderstandingClaimKind::LawfulNextOption
                }
            };
            Ok(ContinuationContextSourceDefinition {
                source_id: ContinuationContextSourceId::new(format!(
                    "accepted-intent/{}",
                    intent.record_id.as_str()
                ))?,
                source_class: ContinuationContextSourceClass::AcceptedIntent,
                authority_domain: ContinuationContextAuthorityDomain::AcceptedIntent,
                revision: intent.directive_commitment.clone(),
                freshness: ContinuationContextFreshness::Current,
                integrity: ContinuationContextIntegrity::Verified,
                access: ContinuationContextAccess::Permitted,
                sensitivity: ContinuationContextSensitivity::Internal,
                materialized_bytes: u64::try_from(intent.meaning.as_str().len())
                    .unwrap_or(u64::MAX),
                claims: vec![ContinuationUnderstandingCandidate {
                    kind,
                    value: intent.meaning.clone(),
                }],
            })
        })
        .collect()
}

fn source_disposition(
    source: &ContinuationContextSourceDefinition,
    maximum_sensitivity: ContinuationContextSensitivity,
    remaining_budget: u64,
) -> ContinuationContextSourceDisposition {
    if source.access != ContinuationContextAccess::Permitted
        || source.sensitivity > maximum_sensitivity
    {
        return ContinuationContextSourceDisposition::Unauthorized;
    }
    match source.integrity {
        ContinuationContextIntegrity::Invalid => {
            return ContinuationContextSourceDisposition::IntegrityInvalid;
        }
        ContinuationContextIntegrity::Unverified => {
            return ContinuationContextSourceDisposition::IntegrityUnverified;
        }
        ContinuationContextIntegrity::Verified => {}
    }
    match source.freshness {
        ContinuationContextFreshness::StaleRefreshable => {
            return ContinuationContextSourceDisposition::StaleRefreshable;
        }
        ContinuationContextFreshness::StaleBlocking => {
            return ContinuationContextSourceDisposition::StaleBlocking;
        }
        ContinuationContextFreshness::Superseded => {
            return ContinuationContextSourceDisposition::Superseded;
        }
        ContinuationContextFreshness::Current => {}
    }
    if source.materialized_bytes <= remaining_budget {
        ContinuationContextSourceDisposition::Selected
    } else {
        ContinuationContextSourceDisposition::OmittedBudget
    }
}

fn source_gap(
    disposition: ContinuationContextSourceDisposition,
) -> Option<ContinuationContextGapKind> {
    match disposition {
        ContinuationContextSourceDisposition::OmittedBudget => {
            Some(ContinuationContextGapKind::SourceOverBudget)
        }
        ContinuationContextSourceDisposition::StaleRefreshable
        | ContinuationContextSourceDisposition::StaleBlocking
        | ContinuationContextSourceDisposition::Superseded => {
            Some(ContinuationContextGapKind::SourceStale)
        }
        ContinuationContextSourceDisposition::Unauthorized => {
            Some(ContinuationContextGapKind::SourceUnauthorized)
        }
        ContinuationContextSourceDisposition::IntegrityUnverified
        | ContinuationContextSourceDisposition::IntegrityInvalid => {
            Some(ContinuationContextGapKind::SourceIntegrity)
        }
        ContinuationContextSourceDisposition::Selected
        | ContinuationContextSourceDisposition::ReferenceOnly => None,
    }
}

fn inventory_entry(
    source: &ContinuationContextSourceDefinition,
    necessity: ContinuationSourceNecessity,
    disposition: ContinuationContextSourceDisposition,
) -> ContinuationContextInventoryEntry {
    ContinuationContextInventoryEntry {
        source_id: source.source_id.clone(),
        source_class: source.source_class,
        authority_domain: source.authority_domain,
        revision: source.revision.clone(),
        freshness: source.freshness,
        integrity: source.integrity,
        access: source.access,
        sensitivity: source.sensitivity,
        necessity,
        disposition,
    }
}

fn derive_claims(
    selected: &[ContinuationContextSourceDefinition],
    profile: &ContinuationContextSemanticProfile,
    accounting: &[ContinuationClaimAccountingEntry],
    coverage: &[ContinuationContextSourceCoverageEntry],
    withheld: &BTreeSet<ContinuationUnderstandingClaimKind>,
    run_bound: bool,
    gaps: &mut Vec<ContinuationContextGap>,
) -> Vec<ContinuationUnderstandingClaim> {
    let mut candidates = BTreeMap::<ContinuationUnderstandingClaimKind, Vec<RankedClaim>>::new();
    for source in selected {
        for claim in &source.claims {
            if let Some(rank) = authority_rank(claim.kind, source.authority_domain, run_bound) {
                candidates.entry(claim.kind).or_default().push((
                    rank,
                    claim.value.clone(),
                    source.source_id.clone(),
                ));
            }
        }
    }

    let mut claims = Vec::new();
    for obligation in &profile.claim_obligations {
        if obligation.requirement == ContinuationClaimRequirement::Inapplicable {
            continue;
        }
        let kind = obligation.kind;
        let mut values = candidates.remove(&kind).unwrap_or_default();
        values.sort();
        if values.is_empty() {
            claims.push(derive_missing_claim(
                obligation,
                profile,
                accounting,
                coverage,
                withheld.contains(&kind),
                gaps,
            ));
            continue;
        }
        if obligation.requirement == ContinuationClaimRequirement::RequiredAccounting
            && values[0].0 >= 100
            && claim_accounting_complete(profile, accounting, coverage, kind)
        {
            claims.push(ContinuationUnderstandingClaim {
                kind,
                posture: ContinuationUnderstandingClaimPosture::AccountedEmpty,
                value: None,
                source_ids: Vec::new(),
            });
            continue;
        }
        claims.extend(derive_present_claims(obligation, values, gaps));
    }
    claims
}

type RankedClaim = (u8, ContinuationContextText, ContinuationContextSourceId);

fn derive_missing_claim(
    obligation: &ContinuationClaimObligation,
    profile: &ContinuationContextSemanticProfile,
    accounting: &[ContinuationClaimAccountingEntry],
    coverage: &[ContinuationContextSourceCoverageEntry],
    withheld: bool,
    gaps: &mut Vec<ContinuationContextGap>,
) -> ContinuationUnderstandingClaim {
    let accounted_empty = claim_accounting_complete(profile, accounting, coverage, obligation.kind);
    let posture = if withheld {
        ContinuationUnderstandingClaimPosture::Withheld
    } else if accounted_empty {
        ContinuationUnderstandingClaimPosture::AccountedEmpty
    } else {
        ContinuationUnderstandingClaimPosture::Unknown
    };
    let gap_kind = match obligation.requirement {
        ContinuationClaimRequirement::RequiredValue => Some(if withheld {
            ContinuationContextGapKind::ClaimWithheld
        } else {
            ContinuationContextGapKind::ClaimUnknown
        }),
        ContinuationClaimRequirement::RequiredAccounting if !accounted_empty => Some(if withheld {
            ContinuationContextGapKind::ClaimWithheld
        } else {
            ContinuationContextGapKind::ClaimAccountingMissing
        }),
        ContinuationClaimRequirement::RequiredAccounting
        | ContinuationClaimRequirement::Optional
        | ContinuationClaimRequirement::Inapplicable => None,
    };
    if let Some(kind) = gap_kind {
        gaps.push(ContinuationContextGap {
            kind,
            claim_kind: Some(obligation.kind),
        });
    }
    ContinuationUnderstandingClaim {
        kind: obligation.kind,
        posture,
        value: None,
        source_ids: Vec::new(),
    }
}

fn derive_present_claims(
    obligation: &ContinuationClaimObligation,
    values: Vec<RankedClaim>,
    gaps: &mut Vec<ContinuationContextGap>,
) -> Vec<ContinuationUnderstandingClaim> {
    let best_rank = values[0].0;
    let best = match obligation.resolution {
        ContinuationClaimResolution::HighestAuthority => values
            .into_iter()
            .take_while(|candidate| candidate.0 == best_rank)
            .collect::<Vec<_>>(),
    };
    let distinct_values = best
        .iter()
        .map(|(_, value, _)| value.clone())
        .collect::<BTreeSet<_>>();
    if obligation.cardinality == ContinuationClaimCardinality::Singleton
        && distinct_values.len() > 1
    {
        if is_required_claim(obligation.requirement) {
            gaps.push(ContinuationContextGap {
                kind: ContinuationContextGapKind::ClaimConflicting,
                claim_kind: Some(obligation.kind),
            });
        }
        return vec![ContinuationUnderstandingClaim {
            kind: obligation.kind,
            posture: ContinuationUnderstandingClaimPosture::Conflicting,
            value: None,
            source_ids: best.into_iter().map(|(_, _, source)| source).collect(),
        }];
    }
    let posture = claim_posture(best_rank);
    if posture == ContinuationUnderstandingClaimPosture::Advisory
        && is_required_claim(obligation.requirement)
    {
        gaps.push(ContinuationContextGap {
            kind: ContinuationContextGapKind::ClaimAdvisoryOnly,
            claim_kind: Some(obligation.kind),
        });
    }
    materialize_claim_values(obligation, posture, distinct_values, &best)
}

const fn is_required_claim(requirement: ContinuationClaimRequirement) -> bool {
    matches!(
        requirement,
        ContinuationClaimRequirement::RequiredValue
            | ContinuationClaimRequirement::RequiredAccounting
    )
}

const fn claim_posture(best_rank: u8) -> ContinuationUnderstandingClaimPosture {
    if best_rank <= 10 {
        ContinuationUnderstandingClaimPosture::Authoritative
    } else if best_rank < 100 {
        ContinuationUnderstandingClaimPosture::Supported
    } else {
        ContinuationUnderstandingClaimPosture::Advisory
    }
}

fn materialize_claim_values(
    obligation: &ContinuationClaimObligation,
    posture: ContinuationUnderstandingClaimPosture,
    distinct_values: BTreeSet<ContinuationContextText>,
    best: &[RankedClaim],
) -> Vec<ContinuationUnderstandingClaim> {
    match obligation.cardinality {
        ContinuationClaimCardinality::Singleton => vec![ContinuationUnderstandingClaim {
            kind: obligation.kind,
            posture,
            value: distinct_values.into_iter().next(),
            source_ids: best.iter().map(|(_, _, source)| source.clone()).collect(),
        }],
        ContinuationClaimCardinality::Set => distinct_values
            .into_iter()
            .map(|value| {
                let mut source_ids = best
                    .iter()
                    .filter(|(_, candidate, _)| candidate == &value)
                    .map(|(_, _, source)| source.clone())
                    .collect::<Vec<_>>();
                source_ids.sort();
                source_ids.dedup();
                ContinuationUnderstandingClaim {
                    kind: obligation.kind,
                    posture,
                    value: Some(value),
                    source_ids,
                }
            })
            .collect(),
    }
}

fn authority_rank(
    kind: ContinuationUnderstandingClaimKind,
    domain: ContinuationContextAuthorityDomain,
    run_bound: bool,
) -> Option<u8> {
    use ContinuationContextAuthorityDomain as Domain;
    use ContinuationUnderstandingClaimKind as Claim;

    let rank = match kind {
        Claim::Objective | Claim::AcceptedAmendment => {
            return objective_authority_rank(domain, run_bound);
        }
        Claim::Priority => match domain {
            Domain::AcceptedIntent => 0,
            Domain::ImmutableRunInput => 10,
            Domain::TypedHandoff => 20,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::Scope | Claim::StrictNonGoal => match domain {
            Domain::ImmutableRunInput => 0,
            Domain::Approval => 5,
            Domain::AcceptedIntent => 10,
            Domain::TypedHandoff => 20,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::Decision | Claim::RationaleReference => match domain {
            Domain::Approval => 0,
            Domain::ImmutableRunInput => 5,
            Domain::AcceptedIntent => 10,
            Domain::WorkReport => 20,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::CompletedObligation
        | Claim::InProgressObligation
        | Claim::DeferredObligation
        | Claim::RemainingObligation
        | Claim::Blocker
        | Claim::RegisteredWait => match domain {
            Domain::DurableRunState | Domain::EventHistory => 0,
            Domain::ImmutableRunInput => 10,
            Domain::WorkReport => 20,
            Domain::GitObservation | Domain::ProviderObservation => 40,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::EvidenceRequirement => match domain {
            Domain::Evidence | Domain::RequiredContext | Domain::ImmutableRunInput => 0,
            Domain::Policy => 10,
            Domain::WorkReport => 20,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::CheckRequirement | Claim::ValidationCommitment => match domain {
            Domain::Check | Domain::RequiredContext | Domain::ImmutableRunInput => 0,
            Domain::Policy => 10,
            Domain::WorkReport => 20,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::SideEffectPosture => match domain {
            Domain::SideEffect | Domain::ImmutableRunInput => 0,
            Domain::Policy | Domain::Approval => 10,
            Domain::WorkReport => 20,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::ReportingCommitment => match domain {
            Domain::ImmutableRunInput | Domain::RequiredContext => 0,
            Domain::WorkReport => 10,
            Domain::OperatingDocument => 80,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::LawfulNextOption => match domain {
            Domain::Capability | Domain::Approval | Domain::Policy => 0,
            Domain::DurableRunState | Domain::EventHistory => 5,
            Domain::ImmutableRunInput | Domain::TypedHandoff => 10,
            Domain::AcceptedIntent => 20,
            Domain::AgentAdvisory => 100,
            _ => return None,
        },
        Claim::OpenQuestion | Claim::Assumption | Claim::Uncertainty | Claim::PromisedFollowUp => {
            match domain {
                Domain::AcceptedIntent | Domain::TypedHandoff => 10,
                Domain::WorkReport => 20,
                Domain::OperatingDocument => 80,
                Domain::AgentAdvisory => 100,
                _ => return None,
            }
        }
    };
    Some(rank)
}

const fn objective_authority_rank(
    domain: ContinuationContextAuthorityDomain,
    run_bound: bool,
) -> Option<u8> {
    use ContinuationContextAuthorityDomain as Domain;

    match domain {
        Domain::ImmutableRunInput if run_bound => Some(0),
        Domain::AcceptedIntent if run_bound => Some(10),
        Domain::AcceptedIntent => Some(0),
        Domain::ImmutableRunInput => Some(10),
        Domain::TypedHandoff => Some(20),
        Domain::OperatingDocument => Some(80),
        Domain::AgentAdvisory => Some(100),
        _ => None,
    }
}

fn canonical_commitment<T: Serialize + ?Sized>(
    domain: &str,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    let payload = serde_json::to_vec(&(CONTEXT_ALGORITHM, domain, value)).map_err(|_| {
        context_error(
            "commitment.serialization_failed",
            "context commitment serialization failed",
        )
    })?;
    Ok(SpecContentHash::from_bytes(payload))
}

fn validate_identifier(type_name: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty() {
        return Err(context_error(
            "identifier.empty",
            format!("{type_name} cannot be empty"),
        ));
    }
    if value.len() > MAX_IDENTIFIER_BYTES {
        return Err(context_error(
            "identifier.too_long",
            format!("{type_name} exceeds the identifier limit"),
        ));
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(context_error(
            "identifier.invalid_character",
            format!("{type_name} contains an invalid character"),
        ));
    }
    Ok(())
}

fn context_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(format!("continuation_context.{code}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assert_is_empty, clippy::expect_used)]
    use super::*;

    fn text(value: &str) -> ContinuationContextText {
        ContinuationContextText::new(value).expect("valid text")
    }

    fn source_id(value: &str) -> ContinuationContextSourceId {
        ContinuationContextSourceId::new(value).expect("valid source id")
    }

    fn scope() -> ContinuationContextScope {
        ContinuationContextScope {
            project_id: ProjectId::new("project/context").expect("project id"),
            workflow_id: Some(WorkflowId::new("workflow/context").expect("workflow id")),
            run_id: Some(WorkflowRunId::new("run-context").expect("run id")),
            phase: Some(text("implementation")),
        }
    }

    fn observed_coverage(
        source_class: ContinuationContextSourceClass,
    ) -> ContinuationContextSourceCoverageEntry {
        ContinuationContextSourceCoverageEntry {
            source_class,
            expectation: ContinuationContextCollectorExpectation::Required,
            outcome: ContinuationContextCollectorOutcome::Observed,
        }
    }

    fn complete_coverage(
        observed: impl IntoIterator<Item = ContinuationContextSourceClass>,
    ) -> Vec<ContinuationContextSourceCoverageEntry> {
        coverage_for(ContinuationContextPurpose::Implementation, true, observed)
    }

    fn coverage_for(
        purpose: ContinuationContextPurpose,
        run_bound: bool,
        observed: impl IntoIterator<Item = ContinuationContextSourceClass>,
    ) -> Vec<ContinuationContextSourceCoverageEntry> {
        let observed = observed.into_iter().collect::<BTreeSet<_>>();
        let profile_id = coverage_profile_id(purpose, run_bound);
        ALL_CONTEXT_SOURCE_CLASSES
            .iter()
            .map(|source_class| {
                let expectation = coverage_profile_expectation(profile_id, *source_class);
                if observed.contains(source_class) {
                    ContinuationContextSourceCoverageEntry {
                        source_class: *source_class,
                        expectation,
                        outcome: ContinuationContextCollectorOutcome::Observed,
                    }
                } else {
                    ContinuationContextSourceCoverageEntry {
                        source_class: *source_class,
                        expectation,
                        outcome: match expectation {
                            ContinuationContextCollectorExpectation::Required => {
                                ContinuationContextCollectorOutcome::Empty
                            }
                            ContinuationContextCollectorExpectation::Optional => {
                                ContinuationContextCollectorOutcome::SkippedOptional
                            }
                            ContinuationContextCollectorExpectation::Inapplicable => {
                                ContinuationContextCollectorOutcome::Inapplicable
                            }
                        },
                    }
                }
            })
            .collect()
    }

    fn source(
        id: &str,
        source_class: ContinuationContextSourceClass,
        domain: ContinuationContextAuthorityDomain,
        claim_kind: ContinuationUnderstandingClaimKind,
        value: &str,
    ) -> ContinuationContextSourceDefinition {
        ContinuationContextSourceDefinition {
            source_id: source_id(id),
            source_class,
            authority_domain: domain,
            revision: SpecContentHash::from_text(id),
            freshness: ContinuationContextFreshness::Current,
            integrity: ContinuationContextIntegrity::Verified,
            access: ContinuationContextAccess::Permitted,
            sensitivity: ContinuationContextSensitivity::Internal,
            materialized_bytes: 64,
            claims: vec![ContinuationUnderstandingCandidate {
                kind: claim_kind,
                value: text(value),
            }],
        }
    }

    fn input(
        sources: Vec<ContinuationContextSourceDefinition>,
    ) -> ContinuationContextDecisionInput {
        let observed = sources
            .iter()
            .map(|source| source.source_class)
            .collect::<BTreeSet<_>>();
        let purpose = ContinuationContextPurpose::Implementation;
        let run_bound = true;
        let coverage = coverage_for(purpose, run_bound, observed);
        ContinuationContextDecisionInput {
            purpose,
            scope: scope(),
            actor: ActorId::new("user/maintainer").expect("actor"),
            maximum_sensitivity: ContinuationContextSensitivity::Internal,
            materialization_budget_bytes: 4_096,
            claim_accounting: accounting_for(purpose, run_bound),
            coverage,
            sources,
            accepted_intents: Vec::new(),
        }
    }

    fn accounting_for(
        purpose: ContinuationContextPurpose,
        run_bound: bool,
    ) -> Vec<ContinuationClaimAccountingEntry> {
        let profile = derive_semantic_profile(purpose, run_bound).expect("semantic profile");
        profile
            .claim_obligations
            .iter()
            .filter(|obligation| {
                obligation.requirement == ContinuationClaimRequirement::RequiredAccounting
            })
            .flat_map(|obligation| {
                best_accounting_source_classes(&profile, obligation.kind)
                    .into_iter()
                    .map(|source_class| ContinuationClaimAccountingEntry {
                        claim_kind: obligation.kind,
                        source_class,
                        outcome: ContinuationClaimAccountingOutcome::Empty,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn ready_input(
        mut sources: Vec<ContinuationContextSourceDefinition>,
    ) -> ContinuationContextDecisionInput {
        let profile = derive_semantic_profile(ContinuationContextPurpose::Implementation, true)
            .expect("semantic profile");
        let missing = profile
            .claim_obligations
            .iter()
            .filter(|obligation| {
                obligation.requirement == ContinuationClaimRequirement::RequiredValue
                    && !sources.iter().any(|source| {
                        source
                            .claims
                            .iter()
                            .any(|claim| claim.kind == obligation.kind)
                    })
            })
            .cloned()
            .collect::<Vec<_>>();
        for obligation in missing {
            let (source_class, domain) = match obligation.kind {
                ContinuationUnderstandingClaimKind::Objective
                | ContinuationUnderstandingClaimKind::Scope
                | ContinuationUnderstandingClaimKind::ReportingCommitment => (
                    ContinuationContextSourceClass::ImmutableRunBundle,
                    ContinuationContextAuthorityDomain::ImmutableRunInput,
                ),
                ContinuationUnderstandingClaimKind::RemainingObligation => (
                    ContinuationContextSourceClass::DurableRunState,
                    ContinuationContextAuthorityDomain::DurableRunState,
                ),
                ContinuationUnderstandingClaimKind::EvidenceRequirement => (
                    ContinuationContextSourceClass::Evidence,
                    ContinuationContextAuthorityDomain::Evidence,
                ),
                ContinuationUnderstandingClaimKind::CheckRequirement
                | ContinuationUnderstandingClaimKind::ValidationCommitment => (
                    ContinuationContextSourceClass::Check,
                    ContinuationContextAuthorityDomain::Check,
                ),
                ContinuationUnderstandingClaimKind::SideEffectPosture => (
                    ContinuationContextSourceClass::SideEffect,
                    ContinuationContextAuthorityDomain::SideEffect,
                ),
                ContinuationUnderstandingClaimKind::LawfulNextOption => (
                    ContinuationContextSourceClass::Capability,
                    ContinuationContextAuthorityDomain::Capability,
                ),
                _ => continue,
            };
            sources.push(source(
                &format!("source/baseline/{:?}", obligation.kind).to_ascii_lowercase(),
                source_class,
                domain,
                obligation.kind,
                "baseline semantic value",
            ));
        }
        input(sources)
    }

    fn set_profile(
        input: &mut ContinuationContextDecisionInput,
        purpose: ContinuationContextPurpose,
        run_bound: bool,
    ) {
        input.purpose = purpose;
        if run_bound {
            input.scope.run_id = Some(WorkflowRunId::new("run-context").expect("run id"));
        } else {
            input.scope.run_id = None;
        }
        let observed = input
            .sources
            .iter()
            .map(|source| source.source_class)
            .collect::<BTreeSet<_>>();
        input.coverage = coverage_for(purpose, run_bound, observed);
        input.claim_accounting = accounting_for(purpose, run_bound);
    }

    #[test]
    fn derives_order_independent_commitments() {
        let immutable = source(
            "source/immutable",
            ContinuationContextSourceClass::ImmutableRunBundle,
            ContinuationContextAuthorityDomain::ImmutableRunInput,
            ContinuationUnderstandingClaimKind::Scope,
            "model only",
        );
        let intent = source(
            "source/intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "restore bounded understanding",
        );
        let first = ready_input(vec![immutable.clone(), intent.clone()]);
        let mut second = ready_input(vec![intent, immutable]);
        second.coverage.reverse();

        let first = derive_continuation_context(first).expect("first decision");
        let second = derive_continuation_context(second).expect("second decision");

        assert_eq!(first.commitment(), second.commitment());
        assert_eq!(first, second);
        assert_eq!(
            first.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
        assert!(!ContinuationContextDecision::authorizes_execution());
        assert!(first.bundle().is_non_authorizing());
        assert!(first.understanding().is_non_authorizing());
    }

    #[test]
    fn coverage_commitment_is_bound_to_the_declared_purpose() {
        let source = source(
            "source/intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective",
        );
        let implementation = derive_continuation_context(input(vec![source.clone()]))
            .expect("implementation decision");
        let mut review_input = input(vec![source]);
        review_input.purpose = ContinuationContextPurpose::ApprovalReview;
        let review = derive_continuation_context(review_input).expect("review decision");

        assert_eq!(
            implementation.coverage().purpose(),
            ContinuationContextPurpose::Implementation
        );
        assert_eq!(
            review.coverage().purpose(),
            ContinuationContextPurpose::ApprovalReview
        );
        assert_ne!(
            implementation.coverage().commitment(),
            review.coverage().commitment()
        );
    }

    #[test]
    fn bundle_and_understanding_commitments_change_with_selected_source_content() {
        let first = source(
            "source/intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "first objective",
        );
        let mut second = first.clone();
        second.claims[0].value = text("changed objective");

        let first = derive_continuation_context(input(vec![first])).expect("first decision");
        let second = derive_continuation_context(input(vec![second])).expect("second decision");

        assert_ne!(first.bundle().commitment(), second.bundle().commitment());
        assert_ne!(
            first.understanding().commitment(),
            second.understanding().commitment()
        );
    }

    #[test]
    fn run_scope_without_workflow_identity_fails_closed() {
        let mut input = input(Vec::new());
        input.scope.workflow_id = None;

        let error = derive_continuation_context(input).expect_err("must reject partial scope");

        assert_eq!(
            error.code(),
            "continuation_context.scope.run_without_workflow"
        );
    }

    #[test]
    fn immutable_run_scope_precedes_operating_document_scope() {
        let immutable = source(
            "source/immutable",
            ContinuationContextSourceClass::ImmutableRunBundle,
            ContinuationContextAuthorityDomain::ImmutableRunInput,
            ContinuationUnderstandingClaimKind::Scope,
            "approved model slice",
        );
        let document = source(
            "source/docs",
            ContinuationContextSourceClass::OperatingDocument,
            ContinuationContextAuthorityDomain::OperatingDocument,
            ContinuationUnderstandingClaimKind::Scope,
            "broader future work",
        );
        let input = input(vec![document, immutable]);

        let decision = derive_continuation_context(input).expect("decision");
        let claim = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Scope)
            .expect("scope claim");

        assert_eq!(
            claim.posture(),
            ContinuationUnderstandingClaimPosture::Authoritative
        );
        assert_eq!(
            claim.value().expect("value").as_str(),
            "approved model slice"
        );
        assert_eq!(claim.source_ids(), &[source_id("source/immutable")]);
    }

    #[test]
    fn immutable_run_objective_precedes_new_intent_for_an_active_run() {
        let immutable = source(
            "source/immutable-objective",
            ContinuationContextSourceClass::ImmutableRunBundle,
            ContinuationContextAuthorityDomain::ImmutableRunInput,
            ContinuationUnderstandingClaimKind::Objective,
            "bound run objective",
        );
        let intent = source(
            "source/new-intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "later objective",
        );

        let decision = derive_continuation_context(input(vec![intent, immutable]))
            .expect("active run decision");
        let objective = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            objective.value().expect("objective value").as_str(),
            "bound run objective"
        );
        assert_eq!(
            objective.source_ids(),
            &[source_id("source/immutable-objective")]
        );
    }

    #[test]
    fn accepted_intent_objective_precedes_operating_document_before_a_run_exists() {
        let document = source(
            "source/planning-document",
            ContinuationContextSourceClass::OperatingDocument,
            ContinuationContextAuthorityDomain::OperatingDocument,
            ContinuationUnderstandingClaimKind::Objective,
            "documented objective",
        );
        let intent = source(
            "source/new-intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "planning objective",
        );
        let mut input = input(vec![document, intent]);
        set_profile(&mut input, ContinuationContextPurpose::Planning, false);

        let decision = derive_continuation_context(input).expect("planning decision");
        let objective = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            objective.value().expect("objective value").as_str(),
            "planning objective"
        );
        assert_eq!(objective.source_ids(), &[source_id("source/new-intent")]);
    }

    #[test]
    fn conflicting_equal_authority_claims_block() {
        let first = source(
            "source/intent-a",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective a",
        );
        let second = source(
            "source/intent-b",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective b",
        );

        let decision = derive_continuation_context(input(vec![first, second])).expect("decision");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
        assert!(decision.gaps().contains(&ContinuationContextGap {
            kind: ContinuationContextGapKind::ClaimConflicting,
            claim_kind: Some(ContinuationUnderstandingClaimKind::Objective),
        }));
    }

    #[test]
    fn missing_required_collector_blocks_even_when_registered_sources_are_valid() {
        let source = source(
            "source/intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective",
        );
        let mut input = input(vec![source]);
        let event_history = input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::EventHistory)
            .expect("event history coverage");
        event_history.expectation = ContinuationContextCollectorExpectation::Required;
        event_history.outcome = ContinuationContextCollectorOutcome::Unsupported;

        let decision = derive_continuation_context(input).expect("decision");

        assert!(!decision.coverage().is_complete());
        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
        assert!(decision.gaps().contains(&ContinuationContextGap {
            kind: ContinuationContextGapKind::CollectorUnavailable,
            claim_kind: None,
        }));
    }

    #[test]
    fn observed_collector_without_a_source_fails_closed() {
        let mut input = input(Vec::new());
        let event_history = input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::EventHistory)
            .expect("event history coverage");
        *event_history = observed_coverage(ContinuationContextSourceClass::EventHistory);

        let error = derive_continuation_context(input).expect_err("must reject contradiction");

        assert_eq!(
            error.code(),
            "continuation_context.coverage.observed_without_source"
        );
    }

    #[test]
    fn non_observed_collector_with_a_source_fails_closed() {
        let source = source(
            "source/intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective",
        );
        let mut input = input(vec![source]);
        input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::AcceptedIntent)
            .expect("accepted intent coverage")
            .outcome = ContinuationContextCollectorOutcome::Empty;

        let error = derive_continuation_context(input).expect_err("must reject contradiction");

        assert_eq!(
            error.code(),
            "continuation_context.coverage.outcome_contradicts_source"
        );
    }

    #[test]
    fn source_class_cannot_substitute_a_stronger_authority_domain() {
        let mut advisory = source(
            "source/advisory",
            ContinuationContextSourceClass::AgentAdvisory,
            ContinuationContextAuthorityDomain::AgentAdvisory,
            ContinuationUnderstandingClaimKind::Decision,
            "advisory decision",
        );
        advisory.authority_domain = ContinuationContextAuthorityDomain::Approval;

        let error = derive_continuation_context(input(vec![advisory]))
            .expect_err("must reject authority substitution");

        assert_eq!(
            error.code(),
            "continuation_context.source.authority_domain_mismatch"
        );
    }

    #[test]
    fn omitted_closed_source_class_cannot_claim_complete_coverage() {
        let source = source(
            "source/intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective",
        );
        let mut input = input(vec![source]);
        input.coverage.retain(|entry| {
            entry.source_class != ContinuationContextSourceClass::ProviderObservation
        });

        let error = derive_continuation_context(input).expect_err("must reject omission");

        assert_eq!(
            error.code(),
            "continuation_context.coverage.class_unaccounted"
        );
    }

    #[test]
    fn every_purpose_and_scope_posture_has_a_distinct_complete_profile() {
        let purposes = [
            ContinuationContextPurpose::Planning,
            ContinuationContextPurpose::Implementation,
            ContinuationContextPurpose::ApprovalReview,
            ContinuationContextPurpose::ProviderReconciliation,
            ContinuationContextPurpose::ChildHandoff,
            ContinuationContextPurpose::MaterialContinuation,
        ];
        let mut profile_ids = BTreeSet::new();

        for purpose in purposes {
            for run_bound in [false, true] {
                let profile_id = coverage_profile_id(purpose, run_bound);
                assert_eq!(is_run_bound_profile(profile_id), run_bound);
                assert!(profile_ids.insert(profile_id));
                let matrix = ALL_CONTEXT_SOURCE_CLASSES
                    .iter()
                    .map(|source_class| coverage_profile_expectation(profile_id, *source_class))
                    .collect::<Vec<_>>();
                assert_eq!(matrix.len(), ALL_CONTEXT_SOURCE_CLASSES.len());
                assert!(matrix.iter().all(|expectation| matches!(
                    expectation,
                    ContinuationContextCollectorExpectation::Required
                        | ContinuationContextCollectorExpectation::Optional
                        | ContinuationContextCollectorExpectation::Inapplicable
                )));
            }
        }

        assert_eq!(profile_ids.len(), 12);
    }

    #[test]
    fn every_semantic_profile_is_complete_distinct_and_authority_bound() {
        let purposes = [
            ContinuationContextPurpose::Planning,
            ContinuationContextPurpose::Implementation,
            ContinuationContextPurpose::ApprovalReview,
            ContinuationContextPurpose::ProviderReconciliation,
            ContinuationContextPurpose::ChildHandoff,
            ContinuationContextPurpose::MaterialContinuation,
        ];
        let mut profile_ids = BTreeSet::new();
        let mut commitments = BTreeSet::new();

        for purpose in purposes {
            for run_bound in [false, true] {
                let profile = derive_semantic_profile(purpose, run_bound).expect("profile");
                assert_eq!(profile.purpose, purpose);
                assert_eq!(profile.run_bound, run_bound);
                assert!(profile_ids.insert(profile.profile_id));
                assert!(commitments.insert(profile.commitment.clone()));
                assert_eq!(
                    profile.claim_obligations.len(),
                    ALL_UNDERSTANDING_CLAIM_KINDS.len()
                );
                assert_eq!(
                    profile.source_obligations.len(),
                    ALL_CONTEXT_SOURCE_CLASSES.len()
                );
                assert!(profile.claim_obligations.iter().all(|obligation| {
                    obligation.resolution == ContinuationClaimResolution::HighestAuthority
                        && !obligation.permitted_authority_domains.is_empty()
                }));
            }
        }

        assert_eq!(profile_ids.len(), 12);
        assert_eq!(commitments.len(), 12);
    }

    #[test]
    fn semantic_requirement_matrix_matches_reviewed_purpose_and_scope_differences() {
        use ContinuationClaimRequirement::{RequiredAccounting, RequiredValue};
        use ContinuationUnderstandingClaimKind as Claim;

        let pre_planning = semantic_profile_id(ContinuationContextPurpose::Planning, false);
        let pre_implementation =
            semantic_profile_id(ContinuationContextPurpose::Implementation, false);
        let pre_review = semantic_profile_id(ContinuationContextPurpose::ApprovalReview, false);
        let pre_child = semantic_profile_id(ContinuationContextPurpose::ChildHandoff, false);
        let run_planning = semantic_profile_id(ContinuationContextPurpose::Planning, true);
        let run_child = semantic_profile_id(ContinuationContextPurpose::ChildHandoff, true);

        assert_eq!(
            claim_requirement(pre_planning, Claim::RemainingObligation),
            RequiredAccounting
        );
        assert_eq!(
            claim_requirement(pre_implementation, Claim::RemainingObligation),
            RequiredValue
        );
        assert_eq!(
            claim_requirement(run_planning, Claim::RemainingObligation),
            RequiredValue
        );
        assert_eq!(
            claim_requirement(pre_planning, Claim::EvidenceRequirement),
            RequiredAccounting
        );
        assert_eq!(
            claim_requirement(pre_review, Claim::EvidenceRequirement),
            RequiredValue
        );
        assert_eq!(
            claim_requirement(pre_child, Claim::CheckRequirement),
            RequiredAccounting
        );
        assert_eq!(
            claim_requirement(run_child, Claim::CheckRequirement),
            RequiredValue
        );
        assert_eq!(
            claim_requirement(pre_planning, Claim::LawfulNextOption),
            RequiredValue
        );
    }

    #[test]
    fn missing_core_required_value_blocks_without_caller_override() {
        let decision = derive_continuation_context(input(Vec::new())).expect("decision");
        let objective = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
        assert_eq!(
            objective.posture(),
            ContinuationUnderstandingClaimPosture::Unknown
        );
        assert!(decision.gaps().contains(&ContinuationContextGap {
            kind: ContinuationContextGapKind::ClaimUnknown,
            claim_kind: Some(ContinuationUnderstandingClaimKind::Objective),
        }));
    }

    #[test]
    fn required_accounting_accepts_typed_empty_and_missing_accounting_blocks() {
        let complete = derive_continuation_context(input(Vec::new())).expect("complete decision");
        let priority = complete
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Priority)
            .expect("priority claim");
        assert_eq!(
            priority.posture(),
            ContinuationUnderstandingClaimPosture::AccountedEmpty
        );
        assert!(!complete.gaps().iter().any(|gap| {
            gap.kind == ContinuationContextGapKind::ClaimAccountingMissing
                && gap.claim_kind == Some(ContinuationUnderstandingClaimKind::Priority)
        }));

        let mut missing = input(Vec::new());
        missing
            .claim_accounting
            .retain(|entry| entry.claim_kind != ContinuationUnderstandingClaimKind::Priority);
        let missing = derive_continuation_context(missing).expect("missing accounting decision");
        assert!(missing.gaps().contains(&ContinuationContextGap {
            kind: ContinuationContextGapKind::ClaimAccountingMissing,
            claim_kind: Some(ContinuationUnderstandingClaimKind::Priority),
        }));
    }

    #[test]
    fn advisory_only_value_cannot_satisfy_a_required_claim() {
        let advisory = source(
            "source/advisory-objective",
            ContinuationContextSourceClass::AgentAdvisory,
            ContinuationContextAuthorityDomain::AgentAdvisory,
            ContinuationUnderstandingClaimKind::Objective,
            "plausible but non-authoritative objective",
        );

        let decision = derive_continuation_context(input(vec![advisory])).expect("decision");
        let objective = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            objective.posture(),
            ContinuationUnderstandingClaimPosture::Advisory
        );
        assert!(decision.gaps().contains(&ContinuationContextGap {
            kind: ContinuationContextGapKind::ClaimAdvisoryOnly,
            claim_kind: Some(ContinuationUnderstandingClaimKind::Objective),
        }));
    }

    #[test]
    fn authoritative_empty_accounting_precedes_advisory_only_prose() {
        let advisory = source(
            "source/advisory-priority",
            ContinuationContextSourceClass::AgentAdvisory,
            ContinuationContextAuthorityDomain::AgentAdvisory,
            ContinuationUnderstandingClaimKind::Priority,
            "model-guessed priority",
        );

        let decision = derive_continuation_context(ready_input(vec![advisory]))
            .expect("accounted-empty decision");
        let priority = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Priority)
            .expect("priority claim");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
        assert_eq!(
            priority.posture(),
            ContinuationUnderstandingClaimPosture::AccountedEmpty
        );
        assert!(priority.value().is_none());
        assert!(priority.source_ids().is_empty());
        assert!(!decision.gaps().iter().any(|gap| {
            gap.kind == ContinuationContextGapKind::ClaimAdvisoryOnly
                && gap.claim_kind == Some(ContinuationUnderstandingClaimKind::Priority)
        }));
    }

    #[test]
    fn equal_rank_set_values_are_retained_with_deterministic_provenance() {
        let first = source(
            "source/intent-a",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::PromisedFollowUp,
            "publish the implementation report",
        );
        let second = source(
            "source/intent-b",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::PromisedFollowUp,
            "run the replacement-executor proof later",
        );

        let decision = derive_continuation_context(ready_input(vec![second, first]))
            .expect("set-valued decision");
        let follow_ups = decision
            .understanding()
            .claims()
            .iter()
            .filter(|claim| claim.kind() == ContinuationUnderstandingClaimKind::PromisedFollowUp)
            .collect::<Vec<_>>();

        assert_eq!(follow_ups.len(), 2);
        assert_eq!(
            follow_ups
                .iter()
                .map(|claim| claim.value().expect("value").as_str())
                .collect::<Vec<_>>(),
            vec![
                "publish the implementation report",
                "run the replacement-executor proof later",
            ]
        );
        assert_eq!(follow_ups[0].source_ids(), &[source_id("source/intent-a")]);
        assert_eq!(follow_ups[1].source_ids(), &[source_id("source/intent-b")]);
    }

    #[test]
    fn accounting_from_a_non_selected_authority_fails_closed() {
        let mut input = input(Vec::new());
        let priority = input
            .claim_accounting
            .iter_mut()
            .find(|entry| entry.claim_kind == ContinuationUnderstandingClaimKind::Priority)
            .expect("priority accounting");
        priority.source_class = ContinuationContextSourceClass::AgentAdvisory;

        let error =
            derive_continuation_context(input).expect_err("must reject advisory accounting");
        assert_eq!(
            error.code(),
            "continuation_context.accounting.source_not_authoritative"
        );
    }

    #[test]
    fn inapplicable_pre_run_source_is_rejected_before_projection() {
        let approval = source(
            "source/approval",
            ContinuationContextSourceClass::Approval,
            ContinuationContextAuthorityDomain::Approval,
            ContinuationUnderstandingClaimKind::Decision,
            "approval decision",
        );
        let mut input = input(vec![approval]);
        set_profile(&mut input, ContinuationContextPurpose::Planning, false);

        let error =
            derive_continuation_context(input).expect_err("must reject inapplicable source");
        assert_eq!(
            error.code(),
            "continuation_context.coverage.outcome_expectation_mismatch"
        );
    }

    #[test]
    fn profile_matrices_match_the_reviewed_pre_run_and_run_bound_baselines() {
        let pre_run = coverage_profile_id(ContinuationContextPurpose::Planning, false);
        for source_class in [
            ContinuationContextSourceClass::ValidatedDeclaration,
            ContinuationContextSourceClass::Policy,
            ContinuationContextSourceClass::RequiredContext,
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextSourceClass::GitObservation,
            ContinuationContextSourceClass::OperatingDocument,
        ] {
            assert_eq!(
                coverage_profile_expectation(pre_run, source_class),
                ContinuationContextCollectorExpectation::Required
            );
        }
        for source_class in [
            ContinuationContextSourceClass::ImmutableRunBundle,
            ContinuationContextSourceClass::DurableRunState,
            ContinuationContextSourceClass::EventHistory,
            ContinuationContextSourceClass::Approval,
            ContinuationContextSourceClass::Capability,
            ContinuationContextSourceClass::SideEffect,
        ] {
            assert_eq!(
                coverage_profile_expectation(pre_run, source_class),
                ContinuationContextCollectorExpectation::Inapplicable
            );
        }

        let run_bound = coverage_profile_id(ContinuationContextPurpose::Implementation, true);
        for source_class in ALL_CONTEXT_SOURCE_CLASSES {
            let expected = if matches!(
                source_class,
                ContinuationContextSourceClass::ProviderObservation
                    | ContinuationContextSourceClass::AgentAdvisory
            ) {
                ContinuationContextCollectorExpectation::Optional
            } else {
                ContinuationContextCollectorExpectation::Required
            };
            assert_eq!(
                coverage_profile_expectation(run_bound, source_class),
                expected
            );
        }
    }

    #[test]
    fn caller_cannot_downgrade_required_expectation() {
        let mut input = input(Vec::new());
        input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::ImmutableRunBundle)
            .expect("immutable run coverage")
            .expectation = ContinuationContextCollectorExpectation::Optional;

        let error = derive_continuation_context(input).expect_err("must reject downgrade");

        assert_eq!(
            error.code(),
            "continuation_context.coverage.expectation_mismatch"
        );
    }

    #[test]
    fn caller_cannot_activate_inapplicable_pre_run_class() {
        let mut input = input(Vec::new());
        input.purpose = ContinuationContextPurpose::Planning;
        input.scope.run_id = None;
        input.coverage = coverage_for(ContinuationContextPurpose::Planning, false, []);
        input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::Approval)
            .expect("approval coverage")
            .expectation = ContinuationContextCollectorExpectation::Required;

        let error = derive_continuation_context(input).expect_err("must reject substitution");

        assert_eq!(
            error.code(),
            "continuation_context.coverage.expectation_mismatch"
        );
    }

    #[test]
    fn caller_cannot_relabel_optional_or_required_collection_through_outcomes() {
        let mut optional = input(Vec::new());
        optional
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::AgentAdvisory)
            .expect("advisory coverage")
            .outcome = ContinuationContextCollectorOutcome::Inapplicable;
        let optional_error =
            derive_continuation_context(optional).expect_err("optional cannot be inapplicable");
        assert_eq!(
            optional_error.code(),
            "continuation_context.coverage.outcome_expectation_mismatch"
        );

        let mut required = input(Vec::new());
        required
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::EventHistory)
            .expect("event history coverage")
            .outcome = ContinuationContextCollectorOutcome::SkippedOptional;
        let required_error =
            derive_continuation_context(required).expect_err("required cannot be skipped optional");
        assert_eq!(
            required_error.code(),
            "continuation_context.coverage.outcome_expectation_mismatch"
        );
    }

    #[test]
    fn inapplicable_pre_run_class_requires_exact_inapplicable_outcome() {
        let mut input = input(Vec::new());
        input.purpose = ContinuationContextPurpose::Planning;
        input.scope.run_id = None;
        input.coverage = coverage_for(ContinuationContextPurpose::Planning, false, []);
        input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::Approval)
            .expect("approval coverage")
            .outcome = ContinuationContextCollectorOutcome::Empty;

        let error = derive_continuation_context(input).expect_err("must reject outcome relabel");

        assert_eq!(
            error.code(),
            "continuation_context.coverage.outcome_expectation_mismatch"
        );
    }

    #[test]
    fn pre_run_and_run_bound_profiles_have_distinct_commitments() {
        let run_bound = input(Vec::new());
        let run_bound = derive_continuation_context(run_bound).expect("run-bound decision");

        let mut pre_run = input(Vec::new());
        set_profile(&mut pre_run, ContinuationContextPurpose::Planning, false);
        let pre_run = derive_continuation_context(pre_run).expect("pre-run decision");

        assert!(run_bound.coverage().is_run_bound());
        assert!(!pre_run.coverage().is_run_bound());
        assert_ne!(
            run_bound.coverage().profile_id(),
            pre_run.coverage().profile_id()
        );
        assert_ne!(
            run_bound.coverage().commitment(),
            pre_run.coverage().commitment()
        );
    }

    #[test]
    fn provider_observation_is_required_only_for_run_bound_reconciliation() {
        for purpose in [
            ContinuationContextPurpose::Planning,
            ContinuationContextPurpose::Implementation,
            ContinuationContextPurpose::ApprovalReview,
            ContinuationContextPurpose::ChildHandoff,
            ContinuationContextPurpose::MaterialContinuation,
        ] {
            assert_eq!(
                coverage_profile_expectation(
                    coverage_profile_id(purpose, true),
                    ContinuationContextSourceClass::ProviderObservation,
                ),
                ContinuationContextCollectorExpectation::Optional
            );
        }
        assert_eq!(
            coverage_profile_expectation(
                coverage_profile_id(ContinuationContextPurpose::ProviderReconciliation, true),
                ContinuationContextSourceClass::ProviderObservation,
            ),
            ContinuationContextCollectorExpectation::Required
        );
        assert_eq!(
            coverage_profile_expectation(
                coverage_profile_id(ContinuationContextPurpose::ProviderReconciliation, false),
                ContinuationContextSourceClass::ProviderObservation,
            ),
            ContinuationContextCollectorExpectation::Optional
        );
    }

    #[test]
    fn required_empty_is_complete_but_required_failure_blocks() {
        let complete = input(Vec::new());
        let complete = derive_continuation_context(complete).expect("complete empty coverage");
        assert!(complete.coverage().is_complete());

        let mut failed = input(Vec::new());
        failed
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::EventHistory)
            .expect("event history coverage")
            .outcome = ContinuationContextCollectorOutcome::Failed;
        let failed = derive_continuation_context(failed).expect("failed coverage decision");

        assert!(!failed.coverage().is_complete());
        assert_eq!(
            failed.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
    }

    #[test]
    fn optional_failure_is_degraded_without_breaking_completeness() {
        let mut input = ready_input(Vec::new());
        input
            .coverage
            .iter_mut()
            .find(|entry| entry.source_class == ContinuationContextSourceClass::AgentAdvisory)
            .expect("advisory coverage")
            .outcome = ContinuationContextCollectorOutcome::Failed;

        let decision = derive_continuation_context(input).expect("degraded decision");

        assert!(decision.coverage().is_complete());
        assert!(decision.coverage().has_degraded_optional());
        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
    }

    #[test]
    fn denied_source_is_not_exposed_and_required_claim_is_withheld() {
        let mut denied = source(
            "source/protected",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "protected objective",
        );
        denied.access = ContinuationContextAccess::Denied;

        let decision = derive_continuation_context(input(vec![denied])).expect("decision");
        let claim = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            claim.posture(),
            ContinuationUnderstandingClaimPosture::Withheld
        );
        assert!(claim.value().is_none());
        assert!(claim.source_ids().is_empty());
        assert!(decision.bundle().selected_source_ids().is_empty());
    }

    #[test]
    fn accepted_intent_is_bounded_non_authorizing_input() {
        let intent = AcceptedIntentRecord {
            record_id: AcceptedIntentRecordId::new("intent/objective").expect("intent id"),
            directive_commitment: SpecContentHash::from_text("directive"),
            scope: scope(),
            kind: AcceptedIntentKind::Objective,
            meaning: text("implement the pure model slice"),
            supersession: AcceptedIntentSupersession::Current,
        };
        let mut input = input(Vec::new());
        input.coverage = complete_coverage([ContinuationContextSourceClass::AcceptedIntent]);
        input.accepted_intents = vec![intent];

        let decision = derive_continuation_context(input).expect("decision");
        let claim = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            claim.posture(),
            ContinuationUnderstandingClaimPosture::Authoritative
        );
        assert_eq!(
            claim.value().expect("value").as_str(),
            "implement the pure model slice"
        );
        assert!(!ContinuationContextDecision::authorizes_execution());
    }

    #[test]
    fn required_source_budget_exhaustion_blocks_instead_of_silent_omission() {
        let mut required = source(
            "source/large",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "objective",
        );
        required.materialized_bytes = 10_000;
        let mut input = input(vec![required]);
        input.materialization_budget_bytes = 100;

        let decision = derive_continuation_context(input).expect("decision");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
        assert!(decision.gaps().contains(&ContinuationContextGap {
            kind: ContinuationContextGapKind::SourceOverBudget,
            claim_kind: None,
        }));
    }

    #[test]
    fn optional_source_over_budget_is_disclosed_without_blocking() {
        let mut optional = source(
            "source/optional",
            ContinuationContextSourceClass::AgentAdvisory,
            ContinuationContextAuthorityDomain::AgentAdvisory,
            ContinuationUnderstandingClaimKind::PromisedFollowUp,
            "optional follow-up",
        );
        optional.materialized_bytes = 10_000;
        let mut input = ready_input(vec![optional]);
        input.materialization_budget_bytes = 600;

        let decision = derive_continuation_context(input).expect("decision");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
        assert!(!decision
            .bundle()
            .selected_source_ids()
            .contains(&source_id("source/optional")));
        assert_eq!(
            decision
                .inventory()
                .entries()
                .iter()
                .find(|entry| entry.source_id() == &source_id("source/optional"))
                .expect("optional inventory entry")
                .disposition(),
            ContinuationContextSourceDisposition::OmittedBudget
        );
    }

    #[test]
    fn optional_source_cannot_consume_budget_before_required_source() {
        let mut optional = source(
            "source/a-optional",
            ContinuationContextSourceClass::AgentAdvisory,
            ContinuationContextAuthorityDomain::AgentAdvisory,
            ContinuationUnderstandingClaimKind::PromisedFollowUp,
            "optional follow-up",
        );
        optional.materialized_bytes = 80;
        let mut required = source(
            "source/z-required",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "required objective",
        );
        required.materialized_bytes = 80;
        let mut input = ready_input(vec![optional, required]);
        input.materialization_budget_bytes = 600;

        let decision = derive_continuation_context(input).expect("decision");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
        assert!(decision
            .bundle()
            .selected_source_ids()
            .contains(&source_id("source/z-required")));
        assert!(!decision
            .bundle()
            .selected_source_ids()
            .contains(&source_id("source/a-optional")));
    }

    #[test]
    fn sensitivity_above_actor_ceiling_is_withheld() {
        let mut restricted = source(
            "source/restricted",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "restricted objective",
        );
        restricted.sensitivity = ContinuationContextSensitivity::Restricted;
        let mut input = input(vec![restricted]);
        input.maximum_sensitivity = ContinuationContextSensitivity::Internal;

        let decision = derive_continuation_context(input).expect("decision");
        let claim = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            claim.posture(),
            ContinuationUnderstandingClaimPosture::Withheld
        );
        assert_eq!(claim.source_ids(), []);
        assert_eq!(decision.bundle().selected_source_ids(), []);
    }

    #[test]
    fn superseded_intent_cannot_restore_old_objective() {
        let intent = AcceptedIntentRecord {
            record_id: AcceptedIntentRecordId::new("intent/old-objective").expect("intent id"),
            directive_commitment: SpecContentHash::from_text("old-directive"),
            scope: scope(),
            kind: AcceptedIntentKind::Objective,
            meaning: text("superseded objective"),
            supersession: AcceptedIntentSupersession::Superseded,
        };
        let mut input = input(Vec::new());
        input.coverage = complete_coverage([ContinuationContextSourceClass::AcceptedIntent]);
        input.accepted_intents = vec![intent];

        let decision = derive_continuation_context(input).expect("decision");
        let claim = decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");

        assert_eq!(
            claim.posture(),
            ContinuationUnderstandingClaimPosture::Unknown
        );
        assert!(claim.value().is_none());
        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
    }

    #[test]
    fn debug_output_redacts_semantic_values_and_source_ids() {
        let claim = ContinuationUnderstandingClaim {
            kind: ContinuationUnderstandingClaimKind::Objective,
            posture: ContinuationUnderstandingClaimPosture::Authoritative,
            value: Some(text("sensitive meaning")),
            source_ids: vec![source_id("source/private")],
        };
        let debug = format!("{claim:?}");

        assert!(!debug.contains("sensitive meaning"));
        assert!(!debug.contains("source/private"));
    }
}
