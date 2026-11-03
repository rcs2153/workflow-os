use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{ActorId, ProjectId, SpecContentHash, WorkflowId, WorkflowOsError, WorkflowRunId};

const CONTEXT_ALGORITHM: &str = "workflow-os/continuation-context/v1";
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_TEXT_BYTES: usize = 1_024;
const MAX_SOURCES: usize = 256;
const MAX_CLAIMS_PER_SOURCE: usize = 64;
const MAX_COVERAGE_ENTRIES: usize = 32;
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
    pub required_for_purpose: bool,
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
    pub required_claims: BTreeSet<ContinuationUnderstandingClaimKind>,
    pub coverage: Vec<ContinuationContextSourceCoverageEntry>,
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
    required_for_purpose: bool,
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
            .field("required_for_purpose", &self.required_for_purpose)
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
    reject_duplicate_coverage(&input.coverage)?;
    reject_duplicate_sources(&input.sources)?;
    reject_duplicate_intents(&input.accepted_intents)?;
    validate_source_coverage(&input)?;
    validate_coverage_outcomes(&input)?;
    validate_source_claims(&input.scope, &input.sources)?;
    validate_scope(&input.scope)?;
    validate_intent_scope(&input.scope, &input.accepted_intents)?;

    let coverage = derive_coverage(input.purpose, &input.coverage)?;
    let mut sources = input.sources;
    sources.extend(intent_sources(&input.accepted_intents)?);
    sources.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    reject_duplicate_sources(&sources)?;

    let mut selection_order = sources.iter().collect::<Vec<_>>();
    selection_order.sort_by(|left, right| {
        right
            .required_for_purpose
            .cmp(&left.required_for_purpose)
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
        if source.required_for_purpose {
            if let Some(kind) = source_gap(disposition) {
                gaps.push(ContinuationContextGap {
                    kind,
                    claim_kind: None,
                });
            }
        }
        inventory_entries.push(inventory_entry(source, disposition));
    }
    inventory_entries.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    selected.sort_by(|left, right| left.source_id.cmp(&right.source_id));

    if !coverage.is_complete() {
        gaps.push(ContinuationContextGap {
            kind: ContinuationContextGapKind::CollectorUnavailable,
            claim_kind: None,
        });
    }

    let inventory_commitment = canonical_commitment("private-inventory", &inventory_entries)?;
    let inventory = ContinuationContextInventory {
        algorithm: CONTEXT_ALGORITHM.to_owned(),
        entries: inventory_entries,
        commitment: inventory_commitment,
    };

    let selected_source_ids = selected
        .iter()
        .map(|source| source.source_id.clone())
        .collect::<Vec<_>>();
    let run_bound = input.scope.run_id.is_some();
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
        &input.required_claims,
        &withheld_claims,
        run_bound,
        &mut gaps,
    );
    gaps.sort();
    gaps.dedup();
    let understanding_commitment =
        canonical_commitment("understanding", &(bundle.commitment(), &claims))?;
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
            inventory.commitment(),
            bundle.commitment(),
            understanding.commitment(),
            &gaps,
            posture,
        ),
    )?;

    Ok(ContinuationContextDecision {
        coverage,
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
        entries: entries.to_vec(),
        complete,
        degraded_optional,
        commitment: canonical_commitment("source-coverage", &(purpose, entries))?,
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
                required_for_purpose: true,
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
        required_for_purpose: source.required_for_purpose,
        disposition,
    }
}

fn derive_claims(
    selected: &[ContinuationContextSourceDefinition],
    required: &BTreeSet<ContinuationUnderstandingClaimKind>,
    withheld: &BTreeSet<ContinuationUnderstandingClaimKind>,
    run_bound: bool,
    gaps: &mut Vec<ContinuationContextGap>,
) -> Vec<ContinuationUnderstandingClaim> {
    let mut candidates = BTreeMap::<
        ContinuationUnderstandingClaimKind,
        Vec<(u8, ContinuationContextText, ContinuationContextSourceId)>,
    >::new();
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

    let kinds = required
        .iter()
        .copied()
        .chain(candidates.keys().copied())
        .collect::<BTreeSet<_>>();
    let mut claims = Vec::with_capacity(kinds.len());
    for kind in kinds {
        let mut values = candidates.remove(&kind).unwrap_or_default();
        values.sort();
        if values.is_empty() {
            let posture = if withheld.contains(&kind) {
                ContinuationUnderstandingClaimPosture::Withheld
            } else {
                ContinuationUnderstandingClaimPosture::Unknown
            };
            if required.contains(&kind) {
                gaps.push(ContinuationContextGap {
                    kind: if posture == ContinuationUnderstandingClaimPosture::Withheld {
                        ContinuationContextGapKind::ClaimWithheld
                    } else {
                        ContinuationContextGapKind::ClaimUnknown
                    },
                    claim_kind: Some(kind),
                });
            }
            claims.push(ContinuationUnderstandingClaim {
                kind,
                posture,
                value: None,
                source_ids: Vec::new(),
            });
            continue;
        }
        let best_rank = values[0].0;
        let best = values
            .into_iter()
            .take_while(|candidate| candidate.0 == best_rank)
            .collect::<Vec<_>>();
        let distinct_values = best
            .iter()
            .map(|(_, value, _)| value.clone())
            .collect::<BTreeSet<_>>();
        if distinct_values.len() > 1 {
            if required.contains(&kind) {
                gaps.push(ContinuationContextGap {
                    kind: ContinuationContextGapKind::ClaimConflicting,
                    claim_kind: Some(kind),
                });
            }
            claims.push(ContinuationUnderstandingClaim {
                kind,
                posture: ContinuationUnderstandingClaimPosture::Conflicting,
                value: None,
                source_ids: best.into_iter().map(|(_, _, source)| source).collect(),
            });
            continue;
        }
        let value = distinct_values.into_iter().next();
        let posture = if best_rank <= 10 {
            ContinuationUnderstandingClaimPosture::Authoritative
        } else if best_rank < 100 {
            ContinuationUnderstandingClaimPosture::Supported
        } else {
            ContinuationUnderstandingClaimPosture::Advisory
        };
        claims.push(ContinuationUnderstandingClaim {
            kind,
            posture,
            value,
            source_ids: best.into_iter().map(|(_, _, source)| source).collect(),
        });
    }
    claims
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
        let observed = observed.into_iter().collect::<BTreeSet<_>>();
        ALL_CONTEXT_SOURCE_CLASSES
            .iter()
            .map(|source_class| {
                if observed.contains(source_class) {
                    observed_coverage(*source_class)
                } else {
                    ContinuationContextSourceCoverageEntry {
                        source_class: *source_class,
                        expectation: ContinuationContextCollectorExpectation::Inapplicable,
                        outcome: ContinuationContextCollectorOutcome::Inapplicable,
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
            required_for_purpose: true,
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
        ContinuationContextDecisionInput {
            purpose: ContinuationContextPurpose::Implementation,
            scope: scope(),
            actor: ActorId::new("user/maintainer").expect("actor"),
            maximum_sensitivity: ContinuationContextSensitivity::Internal,
            materialization_budget_bytes: 4_096,
            required_claims: BTreeSet::from([ContinuationUnderstandingClaimKind::Objective]),
            coverage: complete_coverage(observed),
            sources,
            accepted_intents: Vec::new(),
        }
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
        let mut first = input(vec![immutable.clone(), intent.clone()]);
        first
            .required_claims
            .insert(ContinuationUnderstandingClaimKind::Scope);
        let mut second = input(vec![intent, immutable]);
        second
            .required_claims
            .insert(ContinuationUnderstandingClaimKind::Scope);

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
        let mut input = input(vec![document, immutable]);
        input.required_claims = BTreeSet::from([ContinuationUnderstandingClaimKind::Scope]);

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
    fn accepted_intent_objective_precedes_immutable_candidate_before_a_run_exists() {
        let immutable = source(
            "source/immutable-objective",
            ContinuationContextSourceClass::ImmutableRunBundle,
            ContinuationContextAuthorityDomain::ImmutableRunInput,
            ContinuationUnderstandingClaimKind::Objective,
            "candidate run objective",
        );
        let intent = source(
            "source/new-intent",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "planning objective",
        );
        let mut input = input(vec![immutable, intent]);
        input.purpose = ContinuationContextPurpose::Planning;
        input.scope.run_id = None;

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
            ContinuationContextSourceClass::OperatingDocument,
            ContinuationContextAuthorityDomain::OperatingDocument,
            ContinuationUnderstandingClaimKind::PromisedFollowUp,
            "optional follow-up",
        );
        optional.required_for_purpose = false;
        optional.materialized_bytes = 10_000;
        let mut input = input(vec![optional]);
        input.required_claims.clear();
        input.materialization_budget_bytes = 100;

        let decision = derive_continuation_context(input).expect("decision");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
        assert_eq!(decision.bundle().selected_source_ids(), []);
        assert_eq!(
            decision.inventory().entries()[0].disposition(),
            ContinuationContextSourceDisposition::OmittedBudget
        );
    }

    #[test]
    fn optional_source_cannot_consume_budget_before_required_source() {
        let mut optional = source(
            "source/a-optional",
            ContinuationContextSourceClass::OperatingDocument,
            ContinuationContextAuthorityDomain::OperatingDocument,
            ContinuationUnderstandingClaimKind::PromisedFollowUp,
            "optional follow-up",
        );
        optional.required_for_purpose = false;
        optional.materialized_bytes = 80;
        let mut required = source(
            "source/z-required",
            ContinuationContextSourceClass::AcceptedIntent,
            ContinuationContextAuthorityDomain::AcceptedIntent,
            ContinuationUnderstandingClaimKind::Objective,
            "required objective",
        );
        required.materialized_bytes = 80;
        let mut input = input(vec![optional, required]);
        input.materialization_budget_bytes = 100;

        let decision = derive_continuation_context(input).expect("decision");

        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::ReadyForAuthorityResolution
        );
        assert_eq!(
            decision.bundle().selected_source_ids(),
            &[source_id("source/z-required")]
        );
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
