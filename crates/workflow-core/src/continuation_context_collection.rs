//! Private trusted continuation-context collector contract.
//!
//! This module is intentionally crate-private and read-only. It defines the
//! closed Phase A reference model used to prove collector completeness,
//! authority mapping, request binding, stable anchors, and deterministic
//! conversion into the accepted pure continuation-context decision input.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use serde::Serialize;

use crate::continuation_context::{
    canonical_commitment, collector_expectation_for, derive_continuation_context,
    empty_accounting_claims_for, AcceptedIntentKind, AcceptedIntentSupersession,
    ContinuationClaimAccountingEntry, ContinuationClaimAccountingOutcome,
    ContinuationContextAccess, ContinuationContextAuthorityDomain,
    ContinuationContextCollectorExpectation, ContinuationContextCollectorOutcome,
    ContinuationContextDecision, ContinuationContextDecisionInput, ContinuationContextFreshness,
    ContinuationContextIntegrity, ContinuationContextPurpose, ContinuationContextScope,
    ContinuationContextSensitivity, ContinuationContextSourceClass,
    ContinuationContextSourceCoverageEntry, ContinuationContextSourceDefinition,
    ContinuationContextSourceId, ContinuationUnderstandingCandidate,
    ContinuationUnderstandingClaimKind,
};
use crate::{ActorId, SpecContentHash, WorkflowOsError};

mod accepted_intent;
mod cross_gate_lifecycle;
mod delivery;
mod local_observers;
mod replacement_executor;
mod store_backed;

const COLLECTION_ALGORITHM: &str = "workflow-os/continuation-collection/v1";
const MAX_OBSERVATIONS: usize = ALL_COLLECTOR_KINDS.len();
const MAX_CLAIMS_PER_OBSERVATION: usize = 64;
const MAX_ACCOUNTING_PER_OBSERVATION: usize = 64;
const MAX_WORK_IDENTITIES_PER_OBSERVATION: usize = 64;

const ALL_COLLECTOR_KINDS: [ContinuationCollectorKind; 18] = [
    ContinuationCollectorKind::ValidatedDeclaration,
    ContinuationCollectorKind::ImmutableRunBundle,
    ContinuationCollectorKind::DurableRunState,
    ContinuationCollectorKind::EventHistory,
    ContinuationCollectorKind::Policy,
    ContinuationCollectorKind::Approval,
    ContinuationCollectorKind::Capability,
    ContinuationCollectorKind::RequiredContext,
    ContinuationCollectorKind::Evidence,
    ContinuationCollectorKind::Check,
    ContinuationCollectorKind::SideEffect,
    ContinuationCollectorKind::WorkReport,
    ContinuationCollectorKind::AcceptedIntent,
    ContinuationCollectorKind::TypedHandoff,
    ContinuationCollectorKind::GitObservation,
    ContinuationCollectorKind::OperatingDocument,
    ContinuationCollectorKind::ProviderObservation,
    ContinuationCollectorKind::AgentAdvisory,
];

/// Closed identity for every collector recognized by the Phase A contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationCollectorKind {
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

impl ContinuationCollectorKind {
    const fn source_class(self) -> ContinuationContextSourceClass {
        match self {
            Self::ValidatedDeclaration => ContinuationContextSourceClass::ValidatedDeclaration,
            Self::ImmutableRunBundle => ContinuationContextSourceClass::ImmutableRunBundle,
            Self::DurableRunState => ContinuationContextSourceClass::DurableRunState,
            Self::EventHistory => ContinuationContextSourceClass::EventHistory,
            Self::Policy => ContinuationContextSourceClass::Policy,
            Self::Approval => ContinuationContextSourceClass::Approval,
            Self::Capability => ContinuationContextSourceClass::Capability,
            Self::RequiredContext => ContinuationContextSourceClass::RequiredContext,
            Self::Evidence => ContinuationContextSourceClass::Evidence,
            Self::Check => ContinuationContextSourceClass::Check,
            Self::SideEffect => ContinuationContextSourceClass::SideEffect,
            Self::WorkReport => ContinuationContextSourceClass::WorkReport,
            Self::AcceptedIntent => ContinuationContextSourceClass::AcceptedIntent,
            Self::TypedHandoff => ContinuationContextSourceClass::TypedHandoff,
            Self::GitObservation => ContinuationContextSourceClass::GitObservation,
            Self::OperatingDocument => ContinuationContextSourceClass::OperatingDocument,
            Self::ProviderObservation => ContinuationContextSourceClass::ProviderObservation,
            Self::AgentAdvisory => ContinuationContextSourceClass::AgentAdvisory,
        }
    }

    const fn authority_domain(self) -> ContinuationContextAuthorityDomain {
        match self {
            Self::ValidatedDeclaration => ContinuationContextAuthorityDomain::Declaration,
            Self::ImmutableRunBundle => ContinuationContextAuthorityDomain::ImmutableRunInput,
            Self::DurableRunState => ContinuationContextAuthorityDomain::DurableRunState,
            Self::EventHistory => ContinuationContextAuthorityDomain::EventHistory,
            Self::Policy => ContinuationContextAuthorityDomain::Policy,
            Self::Approval => ContinuationContextAuthorityDomain::Approval,
            Self::Capability => ContinuationContextAuthorityDomain::Capability,
            Self::RequiredContext => ContinuationContextAuthorityDomain::RequiredContext,
            Self::Evidence => ContinuationContextAuthorityDomain::Evidence,
            Self::Check => ContinuationContextAuthorityDomain::Check,
            Self::SideEffect => ContinuationContextAuthorityDomain::SideEffect,
            Self::WorkReport => ContinuationContextAuthorityDomain::WorkReport,
            Self::AcceptedIntent => ContinuationContextAuthorityDomain::AcceptedIntent,
            Self::TypedHandoff => ContinuationContextAuthorityDomain::TypedHandoff,
            Self::GitObservation => ContinuationContextAuthorityDomain::GitObservation,
            Self::OperatingDocument => ContinuationContextAuthorityDomain::OperatingDocument,
            Self::ProviderObservation => ContinuationContextAuthorityDomain::ProviderObservation,
            Self::AgentAdvisory => ContinuationContextAuthorityDomain::AgentAdvisory,
        }
    }
}

/// Exact revision fence for one collector source.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectorRevision {
    kind: ContinuationCollectorKind,
    revision: SpecContentHash,
}

impl fmt::Debug for ContinuationCollectorRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationCollectorRevision")
            .field("kind", &self.kind)
            .field("revision", &"[REDACTED]")
            .finish()
    }
}

/// Complete private collection anchor captured before or after collection.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectionAnchor {
    immutable_bundle: Option<SpecContentHash>,
    event_cursor: Option<SpecContentHash>,
    source_revisions: Vec<ContinuationCollectorRevision>,
    commitment: SpecContentHash,
}

impl fmt::Debug for ContinuationCollectionAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationCollectionAnchor")
            .field(
                "immutable_bundle",
                &self.immutable_bundle.as_ref().map(|_| "[REDACTED]"),
            )
            .field(
                "event_cursor",
                &self.event_cursor.as_ref().map(|_| "[REDACTED]"),
            )
            .field("source_revisions", &self.source_revisions.len())
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

impl ContinuationCollectionAnchor {
    fn new(
        immutable_bundle: Option<SpecContentHash>,
        event_cursor: Option<SpecContentHash>,
        mut source_revisions: Vec<ContinuationCollectorRevision>,
    ) -> Result<Self, WorkflowOsError> {
        source_revisions.sort();
        validate_complete_kinds(source_revisions.iter().map(|item| item.kind), "anchor")?;
        let commitment = collection_commitment(
            "anchor",
            &(&immutable_bundle, &event_cursor, &source_revisions),
        )?;
        Ok(Self {
            immutable_bundle,
            event_cursor,
            source_revisions,
            commitment,
        })
    }

    fn revision_for(
        &self,
        kind: ContinuationCollectorKind,
    ) -> Result<&SpecContentHash, WorkflowOsError> {
        self.source_revisions
            .iter()
            .find(|item| item.kind == kind)
            .map(|item| &item.revision)
            .ok_or_else(|| {
                collection_error(
                    "anchor.revision_missing",
                    "validated anchor is missing a closed collector revision",
                )
            })
    }
}

/// Core-built request bound to one pre-collection anchor.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectionRequest {
    purpose: ContinuationContextPurpose,
    scope: ContinuationContextScope,
    actor: ActorId,
    maximum_sensitivity: ContinuationContextSensitivity,
    materialization_budget_bytes: u64,
    anchor: ContinuationCollectionAnchor,
    commitment: SpecContentHash,
}

impl fmt::Debug for ContinuationCollectionRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationCollectionRequest")
            .field("purpose", &self.purpose)
            .field("scope", &"[REDACTED]")
            .field("actor", &"[REDACTED]")
            .field("maximum_sensitivity", &self.maximum_sensitivity)
            .field(
                "materialization_budget_bytes",
                &self.materialization_budget_bytes,
            )
            .field("anchor", &self.anchor)
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

impl ContinuationCollectionRequest {
    fn new(
        purpose: ContinuationContextPurpose,
        scope: ContinuationContextScope,
        actor: ActorId,
        maximum_sensitivity: ContinuationContextSensitivity,
        materialization_budget_bytes: u64,
        anchor: ContinuationCollectionAnchor,
    ) -> Result<Self, WorkflowOsError> {
        let run_bound = scope.run_id.is_some();
        if run_bound != scope.workflow_id.is_some() {
            return Err(collection_error(
                "request.scope_incomplete",
                "run-bound collection requires both workflow and run identity",
            ));
        }
        if run_bound && (anchor.immutable_bundle.is_none() || anchor.event_cursor.is_none()) {
            return Err(collection_error(
                "request.run_anchor_incomplete",
                "run-bound collection requires immutable-bundle and event-cursor anchors",
            ));
        }
        if !run_bound && (anchor.immutable_bundle.is_some() || anchor.event_cursor.is_some()) {
            return Err(collection_error(
                "request.pre_run_anchor_invalid",
                "pre-run collection cannot carry run-bound anchors",
            ));
        }
        if materialization_budget_bytes == 0 {
            return Err(collection_error(
                "request.budget_empty",
                "collection materialization budget must be positive",
            ));
        }
        let commitment = collection_commitment(
            "request",
            &(
                purpose,
                &scope,
                &actor,
                maximum_sensitivity,
                materialization_budget_bytes,
                &anchor.commitment,
            ),
        )?;
        Ok(Self {
            purpose,
            scope,
            actor,
            maximum_sensitivity,
            materialization_budget_bytes,
            anchor,
            commitment,
        })
    }
}

/// Closed Phase A result posture returned by one collector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationCollectionResultPosture {
    Observed,
    Empty,
    Unavailable,
    Denied,
    Failed,
    Unsupported,
    Stale,
    OmittedOptional,
    Inapplicable,
}

/// Bounded source payload admitted only for observed or stale posture.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectedSource {
    source_id: ContinuationContextSourceId,
    freshness: ContinuationContextFreshness,
    integrity: ContinuationContextIntegrity,
    access: ContinuationContextAccess,
    sensitivity: ContinuationContextSensitivity,
    materialized_bytes: u64,
    claims: Vec<ContinuationUnderstandingCandidate>,
    work_identities: Vec<ContinuationCollectedWorkIdentity>,
}

impl fmt::Debug for ContinuationCollectedSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationCollectedSource")
            .field("source_id", &"[REDACTED]")
            .field("freshness", &self.freshness)
            .field("integrity", &self.integrity)
            .field("access", &self.access)
            .field("sensitivity", &self.sensitivity)
            .field("materialized_bytes", &self.materialized_bytes)
            .field("claims", &self.claims.len())
            .field("work_identity_count", &self.work_identities.len())
            .finish()
    }
}

/// Bounded typed empty-accounting claim owned by the collector kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectedAccounting {
    claim_kind: ContinuationUnderstandingClaimKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationCollectedWorkDisposition {
    Completed,
    LawfulNext,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectedWorkIdentity {
    identity: SpecContentHash,
    source_binding: SpecContentHash,
    disposition: ContinuationCollectedWorkDisposition,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectedWorkCandidate {
    source_id: ContinuationContextSourceId,
    identity: ContinuationCollectedWorkIdentity,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectedAcceptedIntent {
    record: crate::continuation_context::AcceptedIntentRecord,
    work_identity: Option<SpecContentHash>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationControlDisposition {
    None,
    Pause,
    Stop,
    Resume,
}

/// One private collector observation.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectionObservation {
    kind: ContinuationCollectorKind,
    request_commitment: SpecContentHash,
    revision: SpecContentHash,
    posture: ContinuationCollectionResultPosture,
    source: Option<ContinuationCollectedSource>,
    accepted_intents: Vec<ContinuationCollectedAcceptedIntent>,
    accounting: Vec<ContinuationCollectedAccounting>,
}

impl fmt::Debug for ContinuationCollectionObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationCollectionObservation")
            .field("kind", &self.kind)
            .field("request_commitment", &"[REDACTED]")
            .field("revision", &"[REDACTED]")
            .field("posture", &self.posture)
            .field("source", &self.source)
            .field("accepted_intents", &self.accepted_intents.len())
            .field("accounting", &self.accounting.len())
            .finish()
    }
}

trait ContinuationContextCollector {
    fn kind(&self) -> ContinuationCollectorKind;

    fn collect(
        &self,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError>;
}

/// Commitment chain for one complete private aggregate composition.
///
/// This proves which stable request, anchor, observation set, coverage result,
/// understanding projection, and decision were composed together. It is not a
/// capability, approval, delivery receipt, or proof that an external collector
/// implementation was side-effect free.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationAggregateCollectorAccountingPosture {
    Complete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationAggregateRequiredCoveragePosture {
    Complete,
    Incomplete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationAggregateObservationMode {
    NonConsuming,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ContinuationAggregateAuthorityPosture {
    NonAuthorizing,
}

#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationAggregateCompositionProof {
    request_commitment: SpecContentHash,
    collector_contract_commitment: SpecContentHash,
    pre_collection_anchor_commitment: SpecContentHash,
    post_collection_anchor_commitment: SpecContentHash,
    observation_set_commitment: SpecContentHash,
    coverage_commitment: SpecContentHash,
    understanding_commitment: SpecContentHash,
    decision_commitment: SpecContentHash,
    collector_kinds_accounted: usize,
    collector_accounting: ContinuationAggregateCollectorAccountingPosture,
    required_source_coverage: ContinuationAggregateRequiredCoveragePosture,
    observation_mode: ContinuationAggregateObservationMode,
    authority_posture: ContinuationAggregateAuthorityPosture,
    control_disposition: ContinuationControlDisposition,
    work_identities: Vec<ContinuationCollectedWorkIdentity>,
    commitment: SpecContentHash,
}

impl fmt::Debug for ContinuationAggregateCompositionProof {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationAggregateCompositionProof")
            .field("request_commitment", &"[REDACTED]")
            .field("collector_contract_commitment", &"[REDACTED]")
            .field("pre_collection_anchor_commitment", &"[REDACTED]")
            .field("post_collection_anchor_commitment", &"[REDACTED]")
            .field("observation_set_commitment", &"[REDACTED]")
            .field("coverage_commitment", &"[REDACTED]")
            .field("understanding_commitment", &"[REDACTED]")
            .field("decision_commitment", &"[REDACTED]")
            .field("collector_kinds_accounted", &self.collector_kinds_accounted)
            .field("collector_accounting", &self.collector_accounting)
            .field("required_source_coverage", &self.required_source_coverage)
            .field("observation_mode", &self.observation_mode)
            .field("authority_posture", &self.authority_posture)
            .field("control_disposition", &self.control_disposition)
            .field("work_identity_count", &self.work_identities.len())
            .field("commitment", &"[REDACTED]")
            .finish()
    }
}

/// One private non-authorizing aggregate decision and its exact proof chain.
#[derive(Clone, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationAggregateComposition {
    purpose: ContinuationContextPurpose,
    scope: ContinuationContextScope,
    source_actor: ActorId,
    decision: ContinuationContextDecision,
    control_disposition: ContinuationControlDisposition,
    work_identities: Vec<ContinuationCollectedWorkIdentity>,
    proof: ContinuationAggregateCompositionProof,
}

impl fmt::Debug for ContinuationAggregateComposition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContinuationAggregateComposition")
            .field("purpose", &self.purpose)
            .field("scope", &"[REDACTED]")
            .field("source_actor", &"[REDACTED]")
            .field("decision_posture", &self.decision.posture())
            .field("claim_count", &self.decision.understanding().claims().len())
            .field("gap_count", &self.decision.gaps().len())
            .field("control_disposition", &self.control_disposition)
            .field("work_identity_count", &self.work_identities.len())
            .field("proof", &self.proof)
            .finish()
    }
}

/// Test-only reference registry. Production registration remains unavailable.
struct InMemoryContinuationCollectorRegistry {
    collectors: BTreeMap<ContinuationCollectorKind, Box<dyn ContinuationContextCollector>>,
}

impl InMemoryContinuationCollectorRegistry {
    fn new(
        collectors: Vec<Box<dyn ContinuationContextCollector>>,
    ) -> Result<Self, WorkflowOsError> {
        if collectors.len() != MAX_OBSERVATIONS {
            return Err(collection_error(
                "registry.incomplete",
                "collector registry must contain every closed collector kind exactly once",
            ));
        }
        let mut by_kind = BTreeMap::new();
        for collector in collectors {
            if by_kind.insert(collector.kind(), collector).is_some() {
                return Err(collection_error(
                    "registry.duplicate",
                    "collector registry contains a duplicate collector kind",
                ));
            }
        }
        validate_complete_kinds(by_kind.keys().copied(), "registry")?;
        Ok(Self {
            collectors: by_kind,
        })
    }

    fn collect(
        &self,
        request: ContinuationCollectionRequest,
        after_anchor: &ContinuationCollectionAnchor,
    ) -> Result<ContinuationContextDecisionInput, WorkflowOsError> {
        let observations = self.collect_observations(&request)?;
        compose_collection(request, after_anchor, observations).map(|(input, _)| input)
    }

    fn compose(
        &self,
        request: ContinuationCollectionRequest,
        after_anchor: &ContinuationCollectionAnchor,
    ) -> Result<ContinuationAggregateComposition, WorkflowOsError> {
        let observations = self.collect_observations(&request)?;
        compose_aggregate(request, after_anchor, observations)
    }

    fn collect_observations(
        &self,
        request: &ContinuationCollectionRequest,
    ) -> Result<Vec<ContinuationCollectionObservation>, WorkflowOsError> {
        let mut observations = Vec::with_capacity(self.collectors.len());
        for (expected_kind, collector) in &self.collectors {
            let observation = collector.collect(request)?;
            if observation.kind != *expected_kind {
                return Err(collection_error(
                    "collector.kind_substitution",
                    "collector returned an observation for a different collector kind",
                ));
            }
            observations.push(observation);
        }
        Ok(observations)
    }
}

/// Private conformance composition over the accepted Phase B-D source sets.
///
/// This remains an internal proof boundary. It has no runtime registration,
/// delivery, authority, consumption, or provider-operation surface.
struct IntegratedContinuationSourceSet<'a, S, B, O, I> {
    store_backed: store_backed::StoreBackedContinuationSourceSet<'a, S, B>,
    local: local_observers::BoundedLocalContinuationSourceSet<'a, O>,
    accepted_intent: accepted_intent::AcceptedIntentContinuationSourceSet<'a, I>,
}

impl<'a, S, B, O, I> IntegratedContinuationSourceSet<'a, S, B, O, I>
where
    S: store_backed::ContinuationStateReadStore,
    B: crate::ImmutableRunBundleStore,
    O: local_observers::ContinuationLocalObserver,
    I: accepted_intent::AcceptedIntentRecordStore,
{
    const fn new(state: &'a S, bundles: &'a B, observer: &'a O, intents: &'a I) -> Self {
        Self {
            store_backed: store_backed::StoreBackedContinuationSourceSet::new(state, bundles),
            local: local_observers::BoundedLocalContinuationSourceSet::new(observer),
            accepted_intent: accepted_intent::AcceptedIntentContinuationSourceSet::new(intents),
        }
    }

    fn capture_anchor(
        &self,
        purpose: ContinuationContextPurpose,
        scope: &ContinuationContextScope,
        actor: &ActorId,
    ) -> Result<ContinuationCollectionAnchor, WorkflowOsError> {
        let store_anchor = self.store_backed.capture_anchor(purpose, scope)?;
        let local_anchor =
            self.local
                .capture_anchor(&store_anchor, purpose, scope.clone(), actor.clone())?;
        self.accepted_intent
            .capture_anchor_over(&local_anchor, scope)
    }

    fn compose(
        &self,
        purpose: ContinuationContextPurpose,
        scope: ContinuationContextScope,
        actor: ActorId,
        maximum_sensitivity: ContinuationContextSensitivity,
        materialization_budget_bytes: u64,
    ) -> Result<ContinuationAggregateComposition, WorkflowOsError> {
        let anchor = self.capture_anchor(purpose, &scope, &actor)?;
        let request = ContinuationCollectionRequest::new(
            purpose,
            scope,
            actor,
            maximum_sensitivity,
            materialization_budget_bytes,
            anchor,
        )?;
        let mut observations = Vec::with_capacity(ALL_COLLECTOR_KINDS.len());
        for kind in store_backed::STORE_BACKED_KINDS {
            observations.push(self.store_backed.collector(kind)?.collect(&request)?);
        }
        for kind in local_observers::LOCAL_OBSERVER_KINDS {
            observations.push(self.local.collector(kind)?.collect(&request)?);
        }
        observations.push(self.accepted_intent.collector().collect(&request)?);
        for kind in [
            ContinuationCollectorKind::ProviderObservation,
            ContinuationCollectorKind::AgentAdvisory,
        ] {
            observations.push(ExplicitClosedPostureCollector::new(kind)?.collect(&request)?);
        }
        let after_anchor = self.capture_anchor(request.purpose, &request.scope, &request.actor)?;
        compose_aggregate(request, &after_anchor, observations)
    }
}

struct ExplicitClosedPostureCollector {
    kind: ContinuationCollectorKind,
}

impl ExplicitClosedPostureCollector {
    fn new(kind: ContinuationCollectorKind) -> Result<Self, WorkflowOsError> {
        if !matches!(
            kind,
            ContinuationCollectorKind::ProviderObservation
                | ContinuationCollectorKind::AgentAdvisory
        ) {
            return Err(collection_error(
                "closed_posture.kind_invalid",
                "explicit closed posture is limited to provider observation and agent advisory",
            ));
        }
        Ok(Self { kind })
    }
}

impl ContinuationContextCollector for ExplicitClosedPostureCollector {
    fn kind(&self) -> ContinuationCollectorKind {
        self.kind
    }

    fn collect(
        &self,
        request: &ContinuationCollectionRequest,
    ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
        let revision = store_backed::outside_batch_revision(self.kind)?;
        if &revision != request.anchor.revision_for(self.kind)? {
            return Err(collection_error(
                "closed_posture.source_drift",
                "explicit closed-posture revision changed after anchor capture",
            ));
        }
        let posture = if collector_expectation_for(
            request.purpose,
            request.scope.run_id.is_some(),
            self.kind.source_class(),
        ) == ContinuationContextCollectorExpectation::Inapplicable
        {
            ContinuationCollectionResultPosture::Inapplicable
        } else {
            ContinuationCollectionResultPosture::Unsupported
        };
        Ok(ContinuationCollectionObservation {
            kind: self.kind,
            request_commitment: request.commitment.clone(),
            revision,
            posture,
            source: None,
            accepted_intents: Vec::new(),
            accounting: Vec::new(),
        })
    }
}

fn compose_aggregate(
    request: ContinuationCollectionRequest,
    after_anchor: &ContinuationCollectionAnchor,
    mut observations: Vec<ContinuationCollectionObservation>,
) -> Result<ContinuationAggregateComposition, WorkflowOsError> {
    observations.sort_by_key(|item| item.kind);
    validate_complete_kinds(observations.iter().map(|item| item.kind), "observation")?;
    let observation_set_commitment = collection_commitment("observation-set", &observations)?;
    let purpose = request.purpose;
    let scope = request.scope.clone();
    let source_actor = request.actor.clone();
    let request_commitment = request.commitment.clone();
    let collector_contract_commitment = closed_collector_contract_commitment()?;
    let pre_collection_anchor_commitment = request.anchor.commitment.clone();
    let post_collection_anchor_commitment = after_anchor.commitment.clone();
    let (input, work_candidates) = compose_collection(request, after_anchor, observations)?;
    let control_disposition = current_control_disposition(&input.accepted_intents)?;
    let decision = derive_continuation_context(input)?;
    let selected_source_ids = decision
        .bundle()
        .selected_source_ids()
        .iter()
        .collect::<BTreeSet<_>>();
    let mut work_identities = work_candidates
        .into_iter()
        .filter(|candidate| selected_source_ids.contains(&candidate.source_id))
        .map(|candidate| candidate.identity)
        .collect::<Vec<_>>();
    canonicalize_work_identities(&mut work_identities)?;

    if ContinuationContextDecision::authorizes_execution()
        || !decision.bundle().is_non_authorizing()
        || !decision.understanding().is_non_authorizing()
    {
        return Err(collection_error(
            "aggregate.authority_invalid",
            "aggregate continuation composition must remain non-authorizing",
        ));
    }

    let collector_kinds_accounted = decision.coverage().entries().len();
    if collector_kinds_accounted != ALL_COLLECTOR_KINDS.len() {
        return Err(collection_error(
            "aggregate.coverage_incomplete",
            "aggregate continuation composition must account for every collector class",
        ));
    }
    let collector_accounting = ContinuationAggregateCollectorAccountingPosture::Complete;

    let coverage_commitment = decision.coverage().commitment().clone();
    let understanding_commitment = decision.understanding().commitment().clone();
    let decision_commitment = decision.commitment().clone();
    let required_source_coverage = if decision.coverage().is_complete() {
        ContinuationAggregateRequiredCoveragePosture::Complete
    } else {
        ContinuationAggregateRequiredCoveragePosture::Incomplete
    };
    let observation_mode = ContinuationAggregateObservationMode::NonConsuming;
    let authority_posture = ContinuationAggregateAuthorityPosture::NonAuthorizing;
    let commitment = collection_commitment(
        "aggregate-composition-proof",
        &(
            &request_commitment,
            &collector_contract_commitment,
            &pre_collection_anchor_commitment,
            &post_collection_anchor_commitment,
            &observation_set_commitment,
            &coverage_commitment,
            &understanding_commitment,
            &decision_commitment,
            collector_kinds_accounted,
            collector_accounting,
            required_source_coverage,
            observation_mode,
            authority_posture,
            control_disposition,
            &work_identities,
        ),
    )?;
    let proof = ContinuationAggregateCompositionProof {
        request_commitment,
        collector_contract_commitment,
        pre_collection_anchor_commitment,
        post_collection_anchor_commitment,
        observation_set_commitment,
        coverage_commitment,
        understanding_commitment,
        decision_commitment,
        collector_kinds_accounted,
        collector_accounting,
        required_source_coverage,
        observation_mode,
        authority_posture,
        control_disposition,
        work_identities: work_identities.clone(),
        commitment,
    };
    Ok(ContinuationAggregateComposition {
        purpose,
        scope,
        source_actor,
        decision,
        control_disposition,
        work_identities,
        proof,
    })
}

fn canonicalize_work_identities(
    work_identities: &mut [ContinuationCollectedWorkIdentity],
) -> Result<(), WorkflowOsError> {
    work_identities.sort();
    if work_identities.windows(2).any(|items| items[0] == items[1]) {
        return Err(collection_error(
            "observation.work_identity_duplicate",
            "selected authoritative sources contain a duplicate work identity and disposition",
        ));
    }
    Ok(())
}

fn closed_collector_contract_commitment() -> Result<SpecContentHash, WorkflowOsError> {
    let contract = ALL_COLLECTOR_KINDS
        .iter()
        .map(|kind| (*kind, kind.source_class(), kind.authority_domain()))
        .collect::<Vec<_>>();
    collection_commitment("closed-collector-contract", &contract)
}

#[allow(clippy::too_many_lines)]
fn compose_collection(
    request: ContinuationCollectionRequest,
    after_anchor: &ContinuationCollectionAnchor,
    mut observations: Vec<ContinuationCollectionObservation>,
) -> Result<
    (
        ContinuationContextDecisionInput,
        Vec<ContinuationCollectedWorkCandidate>,
    ),
    WorkflowOsError,
> {
    if &request.anchor != after_anchor {
        return Err(collection_error(
            "anchor.drifted",
            "continuation collection anchors changed during collection",
        ));
    }
    if observations.len() != MAX_OBSERVATIONS {
        return Err(collection_error(
            "observation.incomplete",
            "collector observations must account for every closed collector kind",
        ));
    }
    observations.sort_by_key(|item| item.kind);
    validate_complete_kinds(observations.iter().map(|item| item.kind), "observation")?;

    let run_bound = request.scope.run_id.is_some();
    let mut coverage = Vec::with_capacity(observations.len());
    let mut sources = Vec::new();
    let mut claim_accounting = Vec::new();
    let mut accepted_intents = Vec::new();
    let mut work_candidates = Vec::new();

    for mut observation in observations {
        if observation.request_commitment != request.commitment {
            return Err(collection_error(
                "observation.request_mismatch",
                "collector observation is bound to a different collection request",
            ));
        }
        if &observation.revision != request.anchor.revision_for(observation.kind)? {
            return Err(collection_error(
                "observation.revision_mismatch",
                "collector observation revision does not match the stable source fence",
            ));
        }
        validate_observation_limits(&observation)?;

        let expectation =
            collector_expectation_for(request.purpose, run_bound, observation.kind.source_class());
        validate_result_shape(&observation, expectation, request.purpose, run_bound)?;
        let outcome = collector_outcome(observation.posture);

        if let Some(mut source) = observation.source.take() {
            if source.claims.len() > MAX_CLAIMS_PER_OBSERVATION {
                return Err(collection_error(
                    "observation.claim_limit",
                    "collector observation exceeds the semantic-claim limit",
                ));
            }
            source.claims.sort();
            for identity in &source.work_identities {
                work_candidates.push(ContinuationCollectedWorkCandidate {
                    source_id: source.source_id.clone(),
                    identity: identity.clone(),
                });
            }
            sources.push(ContinuationContextSourceDefinition {
                source_id: source.source_id,
                source_class: observation.kind.source_class(),
                authority_domain: observation.kind.authority_domain(),
                revision: observation.revision.clone(),
                freshness: source.freshness,
                integrity: source.integrity,
                access: source.access,
                sensitivity: source.sensitivity,
                materialized_bytes: source.materialized_bytes,
                claims: source.claims,
            });
        }

        for accepted in observation.accepted_intents {
            if let Some(identity) = accepted.work_identity {
                work_candidates.push(ContinuationCollectedWorkCandidate {
                    source_id: accepted_intent_source_id(&accepted.record)?,
                    identity: ContinuationCollectedWorkIdentity {
                        identity,
                        source_binding: canonical_commitment(
                            "accepted-intent-record",
                            &accepted.record,
                        )?,
                        disposition: ContinuationCollectedWorkDisposition::LawfulNext,
                    },
                });
            }
            accepted_intents.push(accepted.record);
        }

        observation.accounting.sort();
        claim_accounting.extend(observation.accounting.into_iter().map(|item| {
            ContinuationClaimAccountingEntry {
                claim_kind: item.claim_kind,
                source_class: observation.kind.source_class(),
                outcome: ContinuationClaimAccountingOutcome::Empty,
            }
        }));
        coverage.push(ContinuationContextSourceCoverageEntry {
            source_class: observation.kind.source_class(),
            expectation,
            outcome,
        });
    }

    sources.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    claim_accounting.sort();
    coverage.sort();
    canonicalize_work_candidates(&mut work_candidates)?;
    Ok((
        ContinuationContextDecisionInput {
            purpose: request.purpose,
            scope: request.scope,
            actor: request.actor,
            maximum_sensitivity: request.maximum_sensitivity,
            materialization_budget_bytes: request.materialization_budget_bytes,
            coverage,
            claim_accounting,
            sources,
            accepted_intents,
        },
        work_candidates,
    ))
}

fn validate_observation_limits(
    observation: &ContinuationCollectionObservation,
) -> Result<(), WorkflowOsError> {
    if observation.accounting.len() > MAX_ACCOUNTING_PER_OBSERVATION {
        return Err(collection_error(
            "observation.accounting_limit",
            "collector observation exceeds the accounting-entry limit",
        ));
    }
    let source_identities = observation
        .source
        .as_ref()
        .map_or(0, |source| source.work_identities.len());
    let intent_identities = observation
        .accepted_intents
        .iter()
        .filter(|intent| intent.work_identity.is_some())
        .count();
    if source_identities.saturating_add(intent_identities) > MAX_WORK_IDENTITIES_PER_OBSERVATION {
        return Err(collection_error(
            "observation.work_identity_limit",
            "collector observation exceeds the work-identity limit",
        ));
    }
    Ok(())
}

fn canonicalize_work_candidates(
    work_candidates: &mut [ContinuationCollectedWorkCandidate],
) -> Result<(), WorkflowOsError> {
    work_candidates.sort();
    if work_candidates.windows(2).any(|items| items[0] == items[1]) {
        return Err(collection_error(
            "observation.work_identity_duplicate",
            "collector observations contain a duplicate work identity and disposition",
        ));
    }
    Ok(())
}

fn accepted_intent_source_id(
    intent: &crate::continuation_context::AcceptedIntentRecord,
) -> Result<ContinuationContextSourceId, WorkflowOsError> {
    ContinuationContextSourceId::new(format!("accepted-intent/{}", intent.record_id.as_str()))
}

fn current_control_disposition(
    intents: &[crate::continuation_context::AcceptedIntentRecord],
) -> Result<ContinuationControlDisposition, WorkflowOsError> {
    let controls = intents
        .iter()
        .filter(|intent| intent.supersession == AcceptedIntentSupersession::Current)
        .filter_map(|intent| match intent.kind {
            AcceptedIntentKind::Pause => Some(ContinuationControlDisposition::Pause),
            AcceptedIntentKind::Stop => Some(ContinuationControlDisposition::Stop),
            AcceptedIntentKind::Resume => Some(ContinuationControlDisposition::Resume),
            AcceptedIntentKind::Objective
            | AcceptedIntentKind::Amendment
            | AcceptedIntentKind::Priority => None,
        })
        .collect::<Vec<_>>();
    match controls.as_slice() {
        [] => Ok(ContinuationControlDisposition::None),
        [control] => Ok(*control),
        _ => Err(collection_error(
            "accepted_intent.control_ambiguous",
            "multiple current control intents cannot be composed",
        )),
    }
}

#[allow(clippy::too_many_lines)]
fn validate_result_shape(
    observation: &ContinuationCollectionObservation,
    expectation: ContinuationContextCollectorExpectation,
    purpose: ContinuationContextPurpose,
    run_bound: bool,
) -> Result<(), WorkflowOsError> {
    let accepted_intent_observation = observation.kind == ContinuationCollectorKind::AcceptedIntent;
    if observation.source.as_ref().is_some_and(|source| {
        source.work_identities.iter().any(|item| {
            observation.kind != ContinuationCollectorKind::WorkReport
                || item.disposition != ContinuationCollectedWorkDisposition::Completed
        })
    }) || observation.accepted_intents.iter().any(|intent| {
        intent.work_identity.is_some()
            && (observation.kind != ContinuationCollectorKind::AcceptedIntent
                || intent.record.kind != AcceptedIntentKind::Resume
                || intent.record.supersession != AcceptedIntentSupersession::Current)
    }) {
        return Err(collection_error(
            "observation.work_identity_kind_mismatch",
            "work identity disposition does not match its authoritative collector kind",
        ));
    }
    validate_work_identity_bindings(observation)?;
    if !accepted_intent_observation && !observation.accepted_intents.is_empty() {
        return Err(collection_error(
            "observation.accepted_intent_kind_mismatch",
            "only the accepted-intent collector can return accepted-intent records",
        ));
    }
    if accepted_intent_observation {
        let records_required = observation.posture == ContinuationCollectionResultPosture::Observed;
        if records_required == observation.accepted_intents.is_empty()
            || observation.source.is_some()
        {
            return Err(collection_error(
                "observation.accepted_intent_shape",
                "accepted-intent posture must use bounded records without a generic source payload",
            ));
        }
    }
    match (expectation, observation.posture) {
        (ContinuationContextCollectorExpectation::Inapplicable, posture)
            if posture != ContinuationCollectionResultPosture::Inapplicable =>
        {
            return Err(collection_error(
                "observation.expectation_mismatch",
                "inapplicable collector kind must return inapplicable posture",
            ));
        }
        (
            ContinuationContextCollectorExpectation::Required,
            ContinuationCollectionResultPosture::OmittedOptional,
        )
        | (
            ContinuationContextCollectorExpectation::Optional,
            ContinuationCollectionResultPosture::Inapplicable,
        ) => {
            return Err(collection_error(
                "observation.expectation_mismatch",
                "collector result posture contradicts the Core-owned expectation",
            ));
        }
        _ => {}
    }

    let source_required = !accepted_intent_observation
        && matches!(
            observation.posture,
            ContinuationCollectionResultPosture::Observed
                | ContinuationCollectionResultPosture::Stale
        );
    if source_required != observation.source.is_some() {
        return Err(collection_error(
            "observation.source_shape",
            "collector result posture and source payload are inconsistent",
        ));
    }
    if !matches!(
        observation.posture,
        ContinuationCollectionResultPosture::Observed
            | ContinuationCollectionResultPosture::Stale
            | ContinuationCollectionResultPosture::Empty
    ) && !observation.accounting.is_empty()
    {
        return Err(collection_error(
            "observation.accounting_shape",
            "collector failure or omission posture cannot carry empty accounting",
        ));
    }
    if observation.posture == ContinuationCollectionResultPosture::Empty
        && observation.source.is_some()
    {
        return Err(collection_error(
            "observation.empty_with_source",
            "empty collector posture cannot carry a source payload",
        ));
    }
    if observation.posture == ContinuationCollectionResultPosture::Empty {
        let expected =
            empty_accounting_claims_for(purpose, run_bound, observation.kind.source_class())?;
        let actual = observation
            .accounting
            .iter()
            .map(|item| item.claim_kind)
            .collect::<BTreeSet<_>>();
        if actual.len() != observation.accounting.len()
            || actual != expected.into_iter().collect::<BTreeSet<_>>()
        {
            return Err(collection_error(
                "observation.empty_accounting_incomplete",
                "empty collector posture requires complete Core-owned typed accounting",
            ));
        }
    }
    if let Some(source) = &observation.source {
        let freshness_valid = match observation.posture {
            ContinuationCollectionResultPosture::Observed => {
                source.freshness == ContinuationContextFreshness::Current
            }
            ContinuationCollectionResultPosture::Stale => matches!(
                source.freshness,
                ContinuationContextFreshness::StaleRefreshable
                    | ContinuationContextFreshness::StaleBlocking
                    | ContinuationContextFreshness::Superseded
            ),
            _ => false,
        };
        if !freshness_valid {
            return Err(collection_error(
                "observation.freshness_mismatch",
                "collector source freshness contradicts result posture",
            ));
        }
    }
    Ok(())
}

fn validate_work_identity_bindings(
    observation: &ContinuationCollectionObservation,
) -> Result<(), WorkflowOsError> {
    if let Some(source) = &observation.source {
        for item in &source.work_identities {
            let binding_matches = source
                .claims
                .iter()
                .filter(|claim| {
                    claim.kind == ContinuationUnderstandingClaimKind::CompletedObligation
                })
                .any(|claim| {
                    collection_commitment("collected-work-claim", claim)
                        .is_ok_and(|binding| binding == item.source_binding)
                });
            if !binding_matches {
                return Err(collection_error(
                    "observation.work_identity_binding_mismatch",
                    "completed work identity is not carried by and bound to the exact authoritative source claim",
                ));
            }
        }
    }
    for intent in &observation.accepted_intents {
        if intent.work_identity.is_some()
            && canonical_commitment("accepted-intent-record", &intent.record).is_err()
        {
            return Err(collection_error(
                "observation.work_identity_binding_mismatch",
                "lawful-next work identity cannot be bound to its exact authoritative intent record",
            ));
        }
    }
    Ok(())
}

const fn collector_outcome(
    posture: ContinuationCollectionResultPosture,
) -> ContinuationContextCollectorOutcome {
    match posture {
        ContinuationCollectionResultPosture::Observed => {
            ContinuationContextCollectorOutcome::Observed
        }
        ContinuationCollectionResultPosture::Empty => ContinuationContextCollectorOutcome::Empty,
        ContinuationCollectionResultPosture::Unavailable => {
            ContinuationContextCollectorOutcome::Unavailable
        }
        ContinuationCollectionResultPosture::Denied => {
            ContinuationContextCollectorOutcome::AccessDenied
        }
        ContinuationCollectionResultPosture::Failed => ContinuationContextCollectorOutcome::Failed,
        ContinuationCollectionResultPosture::Unsupported => {
            ContinuationContextCollectorOutcome::Unsupported
        }
        ContinuationCollectionResultPosture::Stale => ContinuationContextCollectorOutcome::Stale,
        ContinuationCollectionResultPosture::OmittedOptional => {
            ContinuationContextCollectorOutcome::SkippedOptional
        }
        ContinuationCollectionResultPosture::Inapplicable => {
            ContinuationContextCollectorOutcome::Inapplicable
        }
    }
}

fn validate_complete_kinds(
    kinds: impl IntoIterator<Item = ContinuationCollectorKind>,
    subject: &str,
) -> Result<(), WorkflowOsError> {
    let kinds = kinds.into_iter().collect::<Vec<_>>();
    let unique = kinds.iter().copied().collect::<BTreeSet<_>>();
    if kinds.len() != ALL_COLLECTOR_KINDS.len() || unique.len() != ALL_COLLECTOR_KINDS.len() {
        return Err(collection_error(
            "kinds.incomplete",
            format!("{subject} must account for every closed collector kind exactly once"),
        ));
    }
    if ALL_COLLECTOR_KINDS
        .iter()
        .any(|kind| !unique.contains(kind))
    {
        return Err(collection_error(
            "kinds.missing",
            format!("{subject} is missing a closed collector kind"),
        ));
    }
    Ok(())
}

fn collection_commitment<T: Serialize + ?Sized>(
    domain: &str,
    value: &T,
) -> Result<SpecContentHash, WorkflowOsError> {
    canonical_commitment(COLLECTION_ALGORITHM, &(domain, value))
}

fn collection_error(code: &str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(format!("continuation_collection.{code}"), message)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use std::{
        cell::Cell,
        fs,
        path::{Path, PathBuf},
        sync::{
            atomic::{AtomicU64, Ordering},
            Mutex,
        },
    };

    use super::*;
    use crate::continuation_context::{
        derive_continuation_context, AcceptedIntentKind, AcceptedIntentRecord,
        AcceptedIntentRecordId, AcceptedIntentSupersession, ContinuationContextDecisionPosture,
        ContinuationContextText,
    };
    use crate::{
        LocalImmutableRunBundleStore, LocalStateBackend, ProjectId, WorkflowId, WorkflowRunId,
    };

    static NEXT_INTEGRATED_ROOT: AtomicU64 = AtomicU64::new(1);

    struct IntegratedTestRoot(PathBuf);

    impl IntegratedTestRoot {
        fn new() -> Self {
            let id = NEXT_INTEGRATED_ROOT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "workflow-os-continuation-integrated-{}-{id}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("integrated root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for IntegratedTestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    struct IntegratedObserver {
        contract: SpecContentHash,
        git_reads: Cell<u32>,
        drift_after_collection: bool,
    }

    impl local_observers::ContinuationLocalObserver for IntegratedObserver {
        fn contract_commitment(&self) -> SpecContentHash {
            self.contract.clone()
        }

        fn observe_git_worktree(
            &self,
            request: &local_observers::LocalObservationRequest,
        ) -> Result<local_observers::GitWorktreeObservation, WorkflowOsError> {
            let read = self.git_reads.get() + 1;
            self.git_reads.set(read);
            let revision = if self.drift_after_collection && read >= 3 {
                "integrated-worktree-drifted"
            } else {
                "integrated-worktree-stable"
            };
            Ok(local_observers::GitWorktreeObservation {
                request_commitment: request.commitment.clone(),
                repository_present: true,
                head_commitment: Some(hash("integrated-head")),
                worktree_commitment: hash(revision),
                tracked_change_count: 0,
                untracked_change_count: 0,
                conflict_count: 0,
            })
        }

        fn observe_operating_documents(
            &self,
            request: &local_observers::LocalObservationRequest,
        ) -> Result<local_observers::OperatingDocumentObservationSet, WorkflowOsError> {
            Ok(local_observers::OperatingDocumentObservationSet {
                request_commitment: request.commitment.clone(),
                documents: vec![
                    integrated_document(
                        local_observers::OperatingDocumentKind::AgentInstructions,
                        "integrated-agents",
                    ),
                    integrated_document(
                        local_observers::OperatingDocumentKind::EngineeringStandard,
                        "integrated-standard",
                    ),
                    integrated_document(
                        local_observers::OperatingDocumentKind::Roadmap,
                        "integrated-roadmap",
                    ),
                ],
            })
        }
    }

    fn integrated_document(
        kind: local_observers::OperatingDocumentKind,
        commitment: &str,
    ) -> local_observers::OperatingDocumentObservation {
        local_observers::OperatingDocumentObservation {
            kind,
            content_commitment: hash(commitment),
            byte_len: 1_024,
            canonical_beneath_project_root: true,
            symlink_free: true,
            sensitivity: ContinuationContextSensitivity::Internal,
        }
    }

    struct IntegratedIntentStore {
        snapshot: Mutex<accepted_intent::AcceptedIntentStoreSnapshot>,
    }

    impl IntegratedIntentStore {
        fn new(scope: ContinuationContextScope) -> Self {
            let record = AcceptedIntentRecord {
                record_id: AcceptedIntentRecordId::new("intent/integrated-objective")
                    .expect("intent id"),
                actor: ActorId::new("user/delegated-maintainer").expect("actor"),
                directive_commitment: hash("integrated-directive"),
                scope: scope.clone(),
                kind: AcceptedIntentKind::Objective,
                meaning: ContinuationContextText::new("continue the governed roadmap phase")
                    .expect("meaning"),
                supersedes: Vec::new(),
                supersession: AcceptedIntentSupersession::Current,
            };
            Self {
                snapshot: Mutex::new(
                    accepted_intent::AcceptedIntentStoreSnapshot::new(scope, vec![record])
                        .expect("intent snapshot"),
                ),
            }
        }
    }

    impl accepted_intent::AcceptedIntentRecordStore for IntegratedIntentStore {
        fn read_snapshot(
            &self,
            scope: &ContinuationContextScope,
        ) -> Result<accepted_intent::AcceptedIntentStoreSnapshot, WorkflowOsError> {
            let snapshot = self.snapshot.lock().expect("intent lock").clone();
            if &snapshot.scope != scope {
                return Err(collection_error(
                    "integrated.intent_scope_mismatch",
                    "integrated accepted-intent store was opened for another scope",
                ));
            }
            Ok(snapshot)
        }

        fn compare_and_replace(
            &self,
            expected_revision: &SpecContentHash,
            replacement: &accepted_intent::AcceptedIntentStoreSnapshot,
        ) -> Result<bool, WorkflowOsError> {
            let mut snapshot = self.snapshot.lock().expect("intent lock");
            if &snapshot.revision != expected_revision {
                return Ok(false);
            }
            *snapshot = replacement.clone();
            Ok(true)
        }
    }

    #[derive(Clone, Copy)]
    enum IntegratedReceiptPosture {
        Exact,
        WrongDelivery,
        WrongProjection,
        WrongRecipient,
        WrongReceiptCommitment,
    }

    struct IntegratedRecipient {
        actor: ActorId,
        posture: IntegratedReceiptPosture,
    }

    impl delivery::ContinuationProjectionRecipient for IntegratedRecipient {
        fn recipient(&self) -> &ActorId {
            &self.actor
        }

        fn acknowledge(
            &self,
            projection: &delivery::ContinuationProjectionDelivery,
        ) -> Result<delivery::ContinuationProjectionDeliveryReceipt, WorkflowOsError> {
            let mut receipt = delivery::ContinuationProjectionDeliveryReceipt::acknowledge_exact(
                projection,
                self.actor.clone(),
            )?;
            match self.posture {
                IntegratedReceiptPosture::Exact => {}
                IntegratedReceiptPosture::WrongDelivery => receipt
                    .substitute_delivery_commitment_for_test(hash("wrong-delivery-commitment")),
                IntegratedReceiptPosture::WrongProjection => receipt
                    .substitute_projection_commitment_for_test(hash("wrong-projection-commitment")),
                IntegratedReceiptPosture::WrongRecipient => receipt.substitute_recipient_for_test(
                    ActorId::new("agent/replacement-substitute").expect("substitute actor"),
                ),
                IntegratedReceiptPosture::WrongReceiptCommitment => {
                    receipt
                        .substitute_receipt_commitment_for_test(hash("wrong-receipt-commitment"));
                }
            }
            Ok(receipt)
        }
    }

    struct ReplacementAuthorityResolver {
        calls: Cell<u32>,
    }

    impl replacement_executor::ReplacementExecutorFreshAuthorityResolver
        for ReplacementAuthorityResolver
    {
        fn resolve_fresh(
            &self,
            _request: &replacement_executor::ReplacementFreshAuthorityRequest,
        ) -> Result<(), WorkflowOsError> {
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }
    }

    struct ExactReplacementAuthorityResolver {
        calls: Cell<u32>,
        expected_operation: SpecContentHash,
        expected_cross_gate_lifecycle: Option<SpecContentHash>,
    }

    struct CrossGateLifecycleFixtureSource {
        observation: cross_gate_lifecycle::CrossGateLifecycleObservation,
        current_revision: SpecContentHash,
    }

    impl cross_gate_lifecycle::CrossGateLifecycleSource for CrossGateLifecycleFixtureSource {
        fn observe_current(
            &self,
        ) -> Result<cross_gate_lifecycle::CrossGateLifecycleObservation, WorkflowOsError> {
            Ok(self.observation.clone())
        }

        fn current_revision(&self) -> Result<SpecContentHash, WorkflowOsError> {
            Ok(self.current_revision.clone())
        }
    }

    fn cross_gate_binding(
        run: &str,
        context: &str,
        dispatch: &str,
        report: &str,
        resource: &str,
        revision: &str,
    ) -> cross_gate_lifecycle::CrossGateLifecycleBinding {
        let run_id = WorkflowRunId::new(run).expect("run");
        let context = hash(context);
        let revision = hash(revision);
        let source = CrossGateLifecycleFixtureSource {
            observation: cross_gate_lifecycle::CrossGateLifecycleObservation::new(
                revision.clone(),
                cross_gate_lifecycle::AcceptedDispatchObservation::new(
                    run_id.clone(),
                    context.clone(),
                    vec![hash(dispatch)],
                )
                .expect("dispatch observation"),
                cross_gate_lifecycle::VerifiedReportObservation::new(
                    run_id.clone(),
                    context.clone(),
                    hash(report),
                ),
                cross_gate_lifecycle::CurrentResourceAuthorityObservation::new(
                    run_id,
                    context,
                    hash(resource),
                ),
            ),
            current_revision: revision,
        };
        cross_gate_lifecycle::CrossGateLifecycleBinding::compose(&source)
            .expect("cross-gate binding")
    }

    impl replacement_executor::ReplacementExecutorFreshAuthorityResolver
        for ExactReplacementAuthorityResolver
    {
        fn resolve_fresh(
            &self,
            request: &replacement_executor::ReplacementFreshAuthorityRequest,
        ) -> Result<(), WorkflowOsError> {
            if request.operation_identity() != &self.expected_operation {
                return Err(collection_error(
                    "replacement_executor.authority.operation_mismatch",
                    "fresh authority request is bound to another operation identity",
                ));
            }
            if request.cross_gate_lifecycle_commitment()
                != self.expected_cross_gate_lifecycle.as_ref()
            {
                return Err(collection_error(
                    "replacement_executor.authority.lifecycle_binding_mismatch",
                    "fresh authority request is bound to another cross-gate lifecycle",
                ));
            }
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }
    }

    fn integrated_scope() -> ContinuationContextScope {
        ContinuationContextScope {
            project_id: ProjectId::new("project/integrated-continuation").expect("project"),
            workflow_id: None,
            run_id: None,
            phase: Some(ContinuationContextText::new("implementation").expect("phase")),
        }
    }

    #[derive(Clone)]
    struct FixtureCollector {
        kind: ContinuationCollectorKind,
        posture: ContinuationCollectionResultPosture,
        source: Option<ContinuationCollectedSource>,
        accepted_intents: Vec<AcceptedIntentRecord>,
        work_identities: Vec<ContinuationCollectedWorkIdentity>,
        accounting: Vec<ContinuationCollectedAccounting>,
        returned_kind: Option<ContinuationCollectorKind>,
        request_commitment: Option<SpecContentHash>,
        revision: Option<SpecContentHash>,
    }

    impl ContinuationContextCollector for FixtureCollector {
        fn kind(&self) -> ContinuationCollectorKind {
            self.kind
        }

        fn collect(
            &self,
            request: &ContinuationCollectionRequest,
        ) -> Result<ContinuationCollectionObservation, WorkflowOsError> {
            let mut source = self.source.clone();
            if let Some(source) = &mut source {
                source.work_identities = self
                    .work_identities
                    .iter()
                    .filter(|identity| {
                        identity.disposition == ContinuationCollectedWorkDisposition::Completed
                    })
                    .cloned()
                    .collect();
            }
            let lawful_next = self
                .work_identities
                .iter()
                .find(|identity| {
                    identity.disposition == ContinuationCollectedWorkDisposition::LawfulNext
                })
                .map(|identity| identity.identity.clone());
            Ok(ContinuationCollectionObservation {
                kind: self.returned_kind.unwrap_or(self.kind),
                request_commitment: self
                    .request_commitment
                    .clone()
                    .unwrap_or_else(|| request.commitment.clone()),
                revision: self.revision.clone().unwrap_or_else(|| {
                    request
                        .anchor
                        .revision_for(self.kind)
                        .expect("revision")
                        .clone()
                }),
                posture: self.posture,
                source,
                accepted_intents: self
                    .accepted_intents
                    .iter()
                    .cloned()
                    .map(|record| ContinuationCollectedAcceptedIntent {
                        work_identity: (record.kind == AcceptedIntentKind::Resume
                            && record.supersession == AcceptedIntentSupersession::Current)
                            .then(|| lawful_next.clone())
                            .flatten(),
                        record,
                    })
                    .collect(),
                accounting: self.accounting.clone(),
            })
        }
    }

    fn hash(value: &str) -> SpecContentHash {
        SpecContentHash::from_bytes(value.as_bytes())
    }

    fn anchor(run_bound: bool) -> ContinuationCollectionAnchor {
        ContinuationCollectionAnchor::new(
            run_bound.then(|| hash("bundle")),
            run_bound.then(|| hash("cursor")),
            ALL_COLLECTOR_KINDS
                .iter()
                .map(|kind| ContinuationCollectorRevision {
                    kind: *kind,
                    revision: hash(&format!("revision/{kind:?}")),
                })
                .collect(),
        )
        .expect("anchor")
    }

    fn request(run_bound: bool) -> ContinuationCollectionRequest {
        ContinuationCollectionRequest::new(
            ContinuationContextPurpose::Planning,
            ContinuationContextScope {
                project_id: ProjectId::new("project/continuation").expect("project"),
                workflow_id: run_bound
                    .then(|| WorkflowId::new("workflow/continuation").expect("workflow")),
                run_id: run_bound.then(|| WorkflowRunId::new("run/continuation").expect("run")),
                phase: None,
            },
            ActorId::new("user/tester").expect("actor"),
            ContinuationContextSensitivity::Restricted,
            32_768,
            anchor(run_bound),
        )
        .expect("request")
    }

    fn default_collector(
        kind: ContinuationCollectorKind,
        request: &ContinuationCollectionRequest,
    ) -> FixtureCollector {
        let expectation = collector_expectation_for(
            request.purpose,
            request.scope.run_id.is_some(),
            kind.source_class(),
        );
        let posture = if kind == ContinuationCollectorKind::AcceptedIntent {
            ContinuationCollectionResultPosture::Unsupported
        } else {
            match expectation {
                ContinuationContextCollectorExpectation::Required => {
                    ContinuationCollectionResultPosture::Unsupported
                }
                ContinuationContextCollectorExpectation::Optional => {
                    ContinuationCollectionResultPosture::OmittedOptional
                }
                ContinuationContextCollectorExpectation::Inapplicable => {
                    ContinuationCollectionResultPosture::Inapplicable
                }
            }
        };
        FixtureCollector {
            kind,
            posture,
            source: None,
            accepted_intents: Vec::new(),
            work_identities: Vec::new(),
            accounting: Vec::new(),
            returned_kind: None,
            request_commitment: None,
            revision: None,
        }
    }

    #[test]
    fn accepted_intent_records_survive_collection_without_generic_source_substitution() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        let target = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::AcceptedIntent)
            .expect("accepted-intent collector");
        target.posture = ContinuationCollectionResultPosture::Observed;
        target.accepted_intents = vec![AcceptedIntentRecord {
            record_id: AcceptedIntentRecordId::new("intent/objective").expect("intent"),
            actor: ActorId::new("user/maintainer").expect("actor"),
            directive_commitment: hash("directive"),
            scope: request.scope.clone(),
            kind: AcceptedIntentKind::Objective,
            meaning: ContinuationContextText::new("implement the bounded phase").expect("meaning"),
            supersedes: Vec::new(),
            supersession: AcceptedIntentSupersession::Current,
        }];
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");

        let input = registry
            .collect(request.clone(), &request.anchor)
            .expect("input");
        assert_eq!(input.accepted_intents.len(), 1);
        assert_eq!(input.sources, []);
        assert_eq!(input.accepted_intents[0].actor.as_str(), "user/maintainer");
    }

    fn registry_with_order(
        request: &ContinuationCollectionRequest,
        reverse: bool,
    ) -> InMemoryContinuationCollectorRegistry {
        let mut kinds = ALL_COLLECTOR_KINDS.to_vec();
        if reverse {
            kinds.reverse();
        }
        InMemoryContinuationCollectorRegistry::new(
            kinds
                .into_iter()
                .map(|kind| {
                    Box::new(default_collector(kind, request))
                        as Box<dyn ContinuationContextCollector>
                })
                .collect(),
        )
        .expect("registry")
    }

    fn registry(request: &ContinuationCollectionRequest) -> InMemoryContinuationCollectorRegistry {
        registry_with_order(request, false)
    }

    fn replacement_request() -> ContinuationCollectionRequest {
        let mut request = request(false);
        request.purpose = ContinuationContextPurpose::MaterialContinuation;
        request.commitment = collection_commitment(
            "request",
            &(
                request.purpose,
                &request.scope,
                &request.actor,
                request.maximum_sensitivity,
                request.materialization_budget_bytes,
                &request.anchor.commitment,
            ),
        )
        .expect("replacement request commitment");
        request
    }

    fn collected_source(
        kind: ContinuationCollectorKind,
        claims: &[(ContinuationUnderstandingClaimKind, &str)],
    ) -> ContinuationCollectedSource {
        ContinuationCollectedSource {
            source_id: ContinuationContextSourceId::new(format!("source/{kind:?}").to_lowercase())
                .expect("source id"),
            freshness: ContinuationContextFreshness::Current,
            integrity: ContinuationContextIntegrity::Verified,
            access: ContinuationContextAccess::Permitted,
            sensitivity: ContinuationContextSensitivity::Internal,
            materialized_bytes: 128,
            claims: claims
                .iter()
                .map(|(kind, value)| ContinuationUnderstandingCandidate {
                    kind: *kind,
                    value: ContinuationContextText::new(*value).expect("claim value"),
                })
                .collect(),
            work_identities: Vec::new(),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn replacement_registry(
        request: &ContinuationCollectionRequest,
        control_kind: AcceptedIntentKind,
        repeat_completed_action: bool,
    ) -> InMemoryContinuationCollectorRegistry {
        replacement_registry_with_work_report_collector(
            request,
            control_kind,
            repeat_completed_action,
            |_| {},
        )
    }

    #[allow(clippy::too_many_lines)]
    fn replacement_registry_with_work_report_collector(
        request: &ContinuationCollectionRequest,
        control_kind: AcceptedIntentKind,
        repeat_completed_action: bool,
        mutate_collector: impl FnOnce(&mut FixtureCollector),
    ) -> InMemoryContinuationCollectorRegistry {
        use ContinuationUnderstandingClaimKind as Claim;

        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, request))
            .collect::<Vec<_>>();
        for collector in &mut collectors {
            let claims: &[(ContinuationUnderstandingClaimKind, &str)] = match collector.kind {
                ContinuationCollectorKind::Policy => &[
                    (Claim::EvidenceRequirement, "attach focused test evidence"),
                    (Claim::SideEffectPosture, "no external side effects"),
                ],
                ContinuationCollectorKind::GitObservation => &[
                    (
                        Claim::InProgressObligation,
                        "replacement proof is in progress",
                    ),
                    (
                        Claim::RemainingObligation,
                        "complete the focused proof and report",
                    ),
                ],
                ContinuationCollectorKind::OperatingDocument => &[
                    (Claim::Scope, "private replacement-executor proof"),
                    (Claim::StrictNonGoal, "no runtime consumer"),
                    (Claim::Decision, "keep orientation separate from authority"),
                    (Claim::RationaleReference, "accepted continuation plan"),
                    (Claim::OpenQuestion, "host integration remains deferred"),
                    (Claim::Assumption, "authoritative inputs are stable"),
                    (Claim::Uncertainty, "external host behavior is unproven"),
                    (
                        Claim::ReportingCommitment,
                        "publish a bounded implementation report",
                    ),
                    (Claim::PromisedFollowUp, "perform focused security review"),
                ],
                ContinuationCollectorKind::Check => &[
                    (Claim::CheckRequirement, "run focused Core tests"),
                    (
                        Claim::ValidationCommitment,
                        "run strict repository validation",
                    ),
                ],
                ContinuationCollectorKind::WorkReport => &[
                    (
                        Claim::CompletedObligation,
                        "projection delivery was completed",
                    ),
                    (
                        Claim::DeferredObligation,
                        "trusted host integration is deferred",
                    ),
                ],
                _ => &[],
            };
            let expectation = collector_expectation_for(
                request.purpose,
                request.scope.run_id.is_some(),
                collector.kind.source_class(),
            );
            if (!claims.is_empty()
                || expectation == ContinuationContextCollectorExpectation::Required)
                && collector.kind != ContinuationCollectorKind::AcceptedIntent
            {
                collector.posture = ContinuationCollectionResultPosture::Observed;
                collector.source = Some(collected_source(collector.kind, claims));
                collector.accounting = empty_accounting_claims_for(
                    request.purpose,
                    request.scope.run_id.is_some(),
                    collector.kind.source_class(),
                )
                .expect("claim accounting")
                .into_iter()
                .map(|claim_kind| ContinuationCollectedAccounting { claim_kind })
                .collect();
            }
        }

        let completed = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::WorkReport)
            .expect("work report collector");
        let completed_claim = completed
            .source
            .as_ref()
            .expect("work report source")
            .claims
            .iter()
            .find(|claim| claim.kind == Claim::CompletedObligation)
            .expect("completed claim");
        completed.work_identities = vec![ContinuationCollectedWorkIdentity {
            identity: hash("work/projection-delivery"),
            source_binding: collection_commitment("collected-work-claim", completed_claim)
                .expect("completed claim binding"),
            disposition: ContinuationCollectedWorkDisposition::Completed,
        }];
        mutate_collector(completed);

        let accepted = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::AcceptedIntent)
            .expect("accepted-intent collector");
        accepted.posture = ContinuationCollectionResultPosture::Observed;
        let old_objective = AcceptedIntentRecord {
            record_id: AcceptedIntentRecordId::new("intent/old-objective").expect("old intent"),
            actor: ActorId::new("user/delegated-maintainer").expect("actor"),
            directive_commitment: hash("old-objective-directive"),
            scope: request.scope.clone(),
            kind: AcceptedIntentKind::Objective,
            meaning: ContinuationContextText::new("repeat projection delivery")
                .expect("old objective"),
            supersedes: Vec::new(),
            supersession: AcceptedIntentSupersession::Superseded,
        };
        accepted.accepted_intents = vec![
            old_objective,
            AcceptedIntentRecord {
                record_id: AcceptedIntentRecordId::new("intent/current-objective")
                    .expect("current intent"),
                actor: ActorId::new("user/delegated-maintainer").expect("actor"),
                directive_commitment: hash("current-objective-directive"),
                scope: request.scope.clone(),
                kind: AcceptedIntentKind::Objective,
                meaning: ContinuationContextText::new(
                    "prove context-free replacement understanding",
                )
                .expect("current objective"),
                supersedes: vec![
                    AcceptedIntentRecordId::new("intent/old-objective").expect("old intent")
                ],
                supersession: AcceptedIntentSupersession::Current,
            },
            AcceptedIntentRecord {
                record_id: AcceptedIntentRecordId::new("intent/accepted-amendment")
                    .expect("amendment"),
                actor: ActorId::new("user/delegated-maintainer").expect("actor"),
                directive_commitment: hash("accepted-amendment-directive"),
                scope: request.scope.clone(),
                kind: AcceptedIntentKind::Amendment,
                meaning: ContinuationContextText::new("disclose typed gaps").expect("amendment"),
                supersedes: Vec::new(),
                supersession: AcceptedIntentSupersession::Current,
            },
            AcceptedIntentRecord {
                record_id: AcceptedIntentRecordId::new("intent/current-priority")
                    .expect("priority"),
                actor: ActorId::new("user/delegated-maintainer").expect("actor"),
                directive_commitment: hash("priority-directive"),
                scope: request.scope.clone(),
                kind: AcceptedIntentKind::Priority,
                meaning: ContinuationContextText::new("p0").expect("priority"),
                supersedes: Vec::new(),
                supersession: AcceptedIntentSupersession::Current,
            },
            AcceptedIntentRecord {
                record_id: AcceptedIntentRecordId::new("intent/superseded-resume")
                    .expect("superseded resume"),
                actor: ActorId::new("user/delegated-maintainer").expect("actor"),
                directive_commitment: hash("superseded-resume-directive"),
                scope: request.scope.clone(),
                kind: AcceptedIntentKind::Resume,
                meaning: ContinuationContextText::new("resume stale completed work")
                    .expect("superseded resume"),
                supersedes: Vec::new(),
                supersession: AcceptedIntentSupersession::Superseded,
            },
            AcceptedIntentRecord {
                record_id: AcceptedIntentRecordId::new("intent/current-next-action")
                    .expect("next action"),
                actor: ActorId::new("user/delegated-maintainer").expect("actor"),
                directive_commitment: hash("next-action-directive"),
                scope: request.scope.clone(),
                kind: control_kind,
                meaning: ContinuationContextText::new(match control_kind {
                    AcceptedIntentKind::Pause => "pause the replacement-executor proof",
                    AcceptedIntentKind::Stop => "stop the replacement-executor proof",
                    AcceptedIntentKind::Resume if repeat_completed_action => {
                        "redo projection delivery using a fresh description"
                    }
                    AcceptedIntentKind::Resume => "implement the replacement-executor proof",
                    AcceptedIntentKind::Objective
                    | AcceptedIntentKind::Amendment
                    | AcceptedIntentKind::Priority => unreachable!("control intent required"),
                })
                .expect("next action"),
                supersedes: vec![AcceptedIntentRecordId::new("intent/superseded-resume")
                    .expect("superseded resume")],
                supersession: AcceptedIntentSupersession::Current,
            },
        ];
        if control_kind == AcceptedIntentKind::Resume {
            let current_resume = accepted
                .accepted_intents
                .iter()
                .find(|intent| {
                    intent.kind == AcceptedIntentKind::Resume
                        && intent.supersession == AcceptedIntentSupersession::Current
                })
                .expect("current resume");
            accepted.work_identities = vec![ContinuationCollectedWorkIdentity {
                identity: hash(if repeat_completed_action {
                    "work/projection-delivery"
                } else {
                    "work/replacement-proof"
                }),
                source_binding: canonical_commitment("accepted-intent-record", current_resume)
                    .expect("current resume binding"),
                disposition: ContinuationCollectedWorkDisposition::LawfulNext,
            }];
        }

        InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("replacement registry")
    }

    fn registry_with_posture(
        request: &ContinuationCollectionRequest,
        kind: ContinuationCollectorKind,
        posture: ContinuationCollectionResultPosture,
    ) -> InMemoryContinuationCollectorRegistry {
        let collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|candidate| {
                let mut collector = default_collector(*candidate, request);
                if *candidate == kind {
                    collector.posture = posture;
                }
                Box::new(collector) as Box<dyn ContinuationContextCollector>
            })
            .collect();
        InMemoryContinuationCollectorRegistry::new(collectors).expect("registry")
    }

    #[test]
    fn closed_collector_mapping_is_complete_and_bijective() {
        let source_classes = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| kind.source_class())
            .collect::<BTreeSet<_>>();
        let authority_domains = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| kind.authority_domain())
            .collect::<BTreeSet<_>>();

        assert_eq!(source_classes.len(), ALL_COLLECTOR_KINDS.len());
        assert_eq!(authority_domains.len(), ALL_COLLECTOR_KINDS.len());
    }

    #[test]
    fn registry_rejects_missing_and_duplicate_kinds() {
        let request = request(false);
        let mut missing = ALL_COLLECTOR_KINDS
            .iter()
            .take(17)
            .map(|kind| {
                Box::new(default_collector(*kind, &request))
                    as Box<dyn ContinuationContextCollector>
            })
            .collect::<Vec<_>>();
        let error = InMemoryContinuationCollectorRegistry::new(missing)
            .err()
            .expect("missing registry must fail");
        assert_eq!(error.code(), "continuation_collection.registry.incomplete");

        missing = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| {
                Box::new(default_collector(*kind, &request))
                    as Box<dyn ContinuationContextCollector>
            })
            .collect();
        missing.pop();
        missing.push(Box::new(default_collector(
            ContinuationCollectorKind::ValidatedDeclaration,
            &request,
        )));
        let error = InMemoryContinuationCollectorRegistry::new(missing)
            .err()
            .expect("duplicate registry must fail");
        assert_eq!(error.code(), "continuation_collection.registry.duplicate");
    }

    #[test]
    fn composition_is_deterministic_and_non_authorizing() {
        let request = request(false);
        let input = registry(&request)
            .collect(request.clone(), &request.anchor)
            .expect("input");
        let second = registry_with_order(&request, true)
            .collect(request.clone(), &request.anchor)
            .expect("second input");

        assert_eq!(input, second);
        assert_eq!(input.coverage.len(), ALL_COLLECTOR_KINDS.len());
        let decision = derive_continuation_context(input).expect("blocked decision");
        assert_eq!(
            decision.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
        assert!(!crate::continuation_context::ContinuationContextDecision::authorizes_execution());
    }

    #[test]
    fn aggregate_composition_binds_complete_coverage_and_projection() {
        let request = request(false);
        let aggregate = registry(&request)
            .compose(request.clone(), &request.anchor)
            .expect("aggregate composition");

        assert_eq!(
            aggregate.proof.collector_kinds_accounted,
            ALL_COLLECTOR_KINDS.len()
        );
        assert_eq!(
            aggregate.proof.collector_accounting,
            ContinuationAggregateCollectorAccountingPosture::Complete
        );
        assert_eq!(
            aggregate.proof.required_source_coverage,
            if aggregate.decision.coverage().is_complete() {
                ContinuationAggregateRequiredCoveragePosture::Complete
            } else {
                ContinuationAggregateRequiredCoveragePosture::Incomplete
            }
        );
        assert_eq!(
            aggregate.proof.coverage_commitment,
            *aggregate.decision.coverage().commitment()
        );
        assert_eq!(
            aggregate.proof.understanding_commitment,
            *aggregate.decision.understanding().commitment()
        );
        assert_eq!(
            aggregate.proof.decision_commitment,
            *aggregate.decision.commitment()
        );
        assert_eq!(
            aggregate.proof.collector_contract_commitment,
            closed_collector_contract_commitment().expect("collector contract")
        );
        assert_eq!(
            aggregate.proof.observation_mode,
            ContinuationAggregateObservationMode::NonConsuming
        );
        assert_eq!(
            aggregate.proof.authority_posture,
            ContinuationAggregateAuthorityPosture::NonAuthorizing
        );
        assert!(!ContinuationContextDecision::authorizes_execution());
    }

    #[test]
    fn aggregate_composition_is_order_independent_and_byte_stable() {
        let request = request(false);
        let forward = registry_with_order(&request, false)
            .compose(request.clone(), &request.anchor)
            .expect("forward aggregate");
        let reverse = registry_with_order(&request, true)
            .compose(request.clone(), &request.anchor)
            .expect("reverse aggregate");

        assert_eq!(forward, reverse);
        assert_eq!(forward.proof.commitment, reverse.proof.commitment);
    }

    #[test]
    fn aggregate_commitment_changes_when_a_collector_posture_changes() {
        let request = request(false);
        let unsupported = registry(&request)
            .compose(request.clone(), &request.anchor)
            .expect("unsupported aggregate");
        let unavailable = registry_with_posture(
            &request,
            ContinuationCollectorKind::ValidatedDeclaration,
            ContinuationCollectionResultPosture::Unavailable,
        )
        .compose(request.clone(), &request.anchor)
        .expect("unavailable aggregate");

        assert_ne!(
            unsupported.proof.observation_set_commitment,
            unavailable.proof.observation_set_commitment
        );
        assert_ne!(
            unsupported.proof.coverage_commitment,
            unavailable.proof.coverage_commitment
        );
        assert_ne!(unsupported.proof.commitment, unavailable.proof.commitment);
    }

    #[test]
    fn aggregate_composition_projects_current_accepted_intent_with_provenance() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        let accepted_intent = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::AcceptedIntent)
            .expect("accepted-intent collector");
        accepted_intent.posture = ContinuationCollectionResultPosture::Observed;
        accepted_intent.accepted_intents = vec![AcceptedIntentRecord {
            record_id: AcceptedIntentRecordId::new("intent/objective").expect("intent"),
            actor: ActorId::new("user/maintainer").expect("actor"),
            directive_commitment: hash("directive"),
            scope: request.scope.clone(),
            kind: AcceptedIntentKind::Objective,
            meaning: ContinuationContextText::new("implement aggregate composition")
                .expect("meaning"),
            supersedes: Vec::new(),
            supersession: AcceptedIntentSupersession::Current,
        }];
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");

        let aggregate = registry
            .compose(request.clone(), &request.anchor)
            .expect("aggregate composition");
        let objective = aggregate
            .decision
            .understanding()
            .claims()
            .iter()
            .find(|claim| claim.kind() == ContinuationUnderstandingClaimKind::Objective)
            .expect("objective claim");
        assert_eq!(
            objective.posture(),
            crate::continuation_context::ContinuationUnderstandingClaimPosture::Authoritative
        );
        assert_eq!(
            objective.value().map(ContinuationContextText::as_str),
            Some("implement aggregate composition")
        );
        assert_eq!(objective.source_ids().len(), 1);
    }

    #[test]
    fn aggregate_composition_rejects_post_collection_anchor_drift() {
        let request = request(true);
        let mut after = request.anchor.clone();
        after.source_revisions[0].revision = hash("aggregate-drift");
        after.commitment = collection_commitment(
            "anchor",
            &(
                &after.immutable_bundle,
                &after.event_cursor,
                &after.source_revisions,
            ),
        )
        .expect("commitment");

        let error = registry(&request)
            .compose(request, &after)
            .expect_err("drift must reject aggregate composition");
        assert_eq!(error.code(), "continuation_collection.anchor.drifted");
    }

    #[test]
    fn aggregate_debug_redacts_commitments_and_semantic_values() {
        let request = request(false);
        let request_commitment = request.commitment.as_str().to_owned();
        let aggregate = registry(&request)
            .compose(request.clone(), &request.anchor)
            .expect("aggregate composition");
        let decision_commitment = aggregate.decision.commitment().as_str().to_owned();
        let debug = format!("{aggregate:?}");

        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(&request_commitment));
        assert!(!debug.contains(&decision_commitment));
    }

    #[test]
    fn integrated_source_sets_compose_one_stable_non_authorizing_aggregate() {
        let root = IntegratedTestRoot::new();
        let state = LocalStateBackend::new(root.path().join("state")).expect("state");
        let bundles = LocalImmutableRunBundleStore::new(root.path().join("bundles"));
        let observer = IntegratedObserver {
            contract: hash("integrated-observer-contract"),
            git_reads: Cell::new(0),
            drift_after_collection: false,
        };
        let scope = integrated_scope();
        let intents = IntegratedIntentStore::new(scope.clone());
        let sources = IntegratedContinuationSourceSet::new(&state, &bundles, &observer, &intents);

        let aggregate = sources
            .compose(
                ContinuationContextPurpose::MaterialContinuation,
                scope,
                ActorId::new("user/delegated-maintainer").expect("actor"),
                ContinuationContextSensitivity::Restricted,
                32_768,
            )
            .expect("integrated composition");

        assert_eq!(
            aggregate.proof.collector_kinds_accounted,
            ALL_COLLECTOR_KINDS.len()
        );
        assert_eq!(
            aggregate.proof.collector_accounting,
            ContinuationAggregateCollectorAccountingPosture::Complete
        );
        assert_eq!(
            aggregate.proof.authority_posture,
            ContinuationAggregateAuthorityPosture::NonAuthorizing
        );
        assert!(aggregate
            .decision
            .understanding()
            .claims()
            .iter()
            .any(|claim| {
                claim.kind() == ContinuationUnderstandingClaimKind::Objective
                    && claim.value().map(ContinuationContextText::as_str)
                        == Some("continue the governed roadmap phase")
            }));
        assert!(!ContinuationContextDecision::authorizes_execution());
        assert_eq!(observer.git_reads.get(), 3);
    }

    #[test]
    fn integrated_source_sets_reject_cross_source_post_collection_drift() {
        let root = IntegratedTestRoot::new();
        let state = LocalStateBackend::new(root.path().join("state")).expect("state");
        let bundles = LocalImmutableRunBundleStore::new(root.path().join("bundles"));
        let observer = IntegratedObserver {
            contract: hash("integrated-observer-contract"),
            git_reads: Cell::new(0),
            drift_after_collection: true,
        };
        let scope = integrated_scope();
        let intents = IntegratedIntentStore::new(scope.clone());
        let sources = IntegratedContinuationSourceSet::new(&state, &bundles, &observer, &intents);

        let error = sources
            .compose(
                ContinuationContextPurpose::MaterialContinuation,
                scope,
                ActorId::new("user/delegated-maintainer").expect("actor"),
                ContinuationContextSensitivity::Restricted,
                32_768,
            )
            .expect_err("cross-source drift must fail the integrated aggregate");

        assert_eq!(error.code(), "continuation_collection.anchor.drifted");
        assert_eq!(observer.git_reads.get(), 3);
    }

    #[test]
    fn integrated_projection_delivery_returns_exact_non_authorizing_acknowledgment() {
        let root = IntegratedTestRoot::new();
        let state = LocalStateBackend::new(root.path().join("state")).expect("state");
        let bundles = LocalImmutableRunBundleStore::new(root.path().join("bundles"));
        let observer = IntegratedObserver {
            contract: hash("integrated-observer-contract"),
            git_reads: Cell::new(0),
            drift_after_collection: false,
        };
        let scope = integrated_scope();
        let intents = IntegratedIntentStore::new(scope.clone());
        let sources = IntegratedContinuationSourceSet::new(&state, &bundles, &observer, &intents);
        let purpose = ContinuationContextPurpose::MaterialContinuation;
        let source_actor = ActorId::new("user/delegated-maintainer").expect("source actor");
        let aggregate = sources
            .compose(
                purpose,
                scope.clone(),
                source_actor.clone(),
                ContinuationContextSensitivity::Restricted,
                32_768,
            )
            .expect("integrated composition");
        let recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-executor").expect("recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };

        let (delivery, receipt) =
            delivery::deliver_exact_projection(&aggregate, &recipient).expect("exact delivery");

        assert_eq!(delivery.recipient(), &recipient.actor);
        assert_eq!(
            delivery.understanding().commitment(),
            aggregate.decision.understanding().commitment()
        );
        assert!(delivery.is_non_authorizing());
        assert!(receipt.is_non_authorizing());
        assert_ne!(receipt.commitment(), delivery.commitment());
        assert!(!ContinuationContextDecision::authorizes_execution());
        let debug = format!("{delivery:?} {receipt:?}");
        assert!(!debug.contains("continue the governed roadmap phase"));
        assert!(!debug.contains(delivery.commitment().as_str()));
        assert!(!debug.contains(receipt.commitment().as_str()));
    }

    #[test]
    fn integrated_projection_delivery_rejects_substituted_acknowledgment_bindings() {
        for posture in [
            IntegratedReceiptPosture::WrongDelivery,
            IntegratedReceiptPosture::WrongProjection,
            IntegratedReceiptPosture::WrongRecipient,
            IntegratedReceiptPosture::WrongReceiptCommitment,
        ] {
            let root = IntegratedTestRoot::new();
            let state = LocalStateBackend::new(root.path().join("state")).expect("state");
            let bundles = LocalImmutableRunBundleStore::new(root.path().join("bundles"));
            let observer = IntegratedObserver {
                contract: hash("integrated-observer-contract"),
                git_reads: Cell::new(0),
                drift_after_collection: false,
            };
            let scope = integrated_scope();
            let intents = IntegratedIntentStore::new(scope.clone());
            let sources =
                IntegratedContinuationSourceSet::new(&state, &bundles, &observer, &intents);
            let purpose = ContinuationContextPurpose::MaterialContinuation;
            let source_actor = ActorId::new("user/delegated-maintainer").expect("source actor");
            let aggregate = sources
                .compose(
                    purpose,
                    scope.clone(),
                    source_actor.clone(),
                    ContinuationContextSensitivity::Restricted,
                    32_768,
                )
                .expect("integrated composition");
            let recipient = IntegratedRecipient {
                actor: ActorId::new("agent/replacement-executor").expect("recipient"),
                posture,
            };

            let error = delivery::deliver_exact_projection(&aggregate, &recipient)
                .expect_err("substituted receipt must fail closed");

            assert!(matches!(
                error.code(),
                "continuation_collection.delivery.receipt.binding_mismatch"
                    | "continuation_collection.delivery.receipt.commitment_mismatch"
            ));
        }
    }

    #[test]
    fn exact_acknowledgment_constructor_rejects_another_recipient() {
        let root = IntegratedTestRoot::new();
        let state = LocalStateBackend::new(root.path().join("state")).expect("state");
        let bundles = LocalImmutableRunBundleStore::new(root.path().join("bundles"));
        let observer = IntegratedObserver {
            contract: hash("integrated-observer-contract"),
            git_reads: Cell::new(0),
            drift_after_collection: false,
        };
        let scope = integrated_scope();
        let intents = IntegratedIntentStore::new(scope.clone());
        let sources = IntegratedContinuationSourceSet::new(&state, &bundles, &observer, &intents);
        let purpose = ContinuationContextPurpose::MaterialContinuation;
        let source_actor = ActorId::new("user/delegated-maintainer").expect("source actor");
        let aggregate = sources
            .compose(
                purpose,
                scope.clone(),
                source_actor.clone(),
                ContinuationContextSensitivity::Restricted,
                32_768,
            )
            .expect("integrated composition");
        let recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-executor").expect("recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let delivery = delivery::ContinuationProjectionDelivery::new(&aggregate, recipient.actor)
            .expect("delivery");

        let error = delivery::ContinuationProjectionDeliveryReceipt::acknowledge_exact(
            &delivery,
            ActorId::new("agent/other-executor").expect("other recipient"),
        )
        .expect_err("recipient substitution must fail");

        assert_eq!(
            error.code(),
            "continuation_collection.delivery.receipt.recipient_mismatch"
        );
    }

    #[test]
    fn context_free_replacement_cannot_resolve_authority_without_cross_gate_binding() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, false)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate =
            replacement_registry(&request, AcceptedIntentKind::Resume, false)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
        let original_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/original-executor").expect("original recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let replacement_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/context-free-replacement").expect("replacement recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };

        let error = replacement_executor::prove_replacement_executor_equivalence(
            &original_aggregate,
            &replacement_aggregate,
            &original_recipient,
            &replacement_recipient,
            &authority,
        )
        .err()
        .expect("fresh authority without cross-gate binding must fail");

        assert_eq!(
            error.code(),
            "continuation_collection.replacement_executor.lifecycle_binding.missing"
        );
        assert_eq!(authority.calls.get(), 0);
    }

    #[test]
    fn cross_gate_replacement_binds_dispatch_report_and_resource_authority() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, false)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate =
            replacement_registry(&request, AcceptedIntentKind::Resume, false)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
        let original_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/original-cross-gate").expect("original recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let replacement_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-cross-gate").expect("replacement recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let lifecycle = cross_gate_binding(
            "run/cross-gate",
            "context/cross-gate",
            "dispatch/cross-gate",
            "report/cross-gate",
            "resource/cross-gate",
            "revision/cross-gate",
        );
        let authority = ExactReplacementAuthorityResolver {
            calls: Cell::new(0),
            expected_operation: hash("work/replacement-proof"),
            expected_cross_gate_lifecycle: Some(lifecycle.commitment().clone()),
        };

        let (_, replacement, proof) =
            replacement_executor::prove_cross_gate_replacement_executor_equivalence(
                &original_aggregate,
                &replacement_aggregate,
                &original_recipient,
                &replacement_recipient,
                &lifecycle,
                &lifecycle,
                &authority,
            )
            .expect("cross-gate replacement proof");

        assert!(replacement.is_non_authorizing());
        assert!(proof.is_non_authorizing());
        assert_eq!(
            proof.authority_posture(),
            replacement_executor::ReplacementFreshAuthorityPosture::ResolvedSeparately
        );
        assert_eq!(authority.calls.get(), 1);
    }

    #[test]
    fn cross_gate_replacement_rejects_each_substituted_gate_binding() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, false)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate =
            replacement_registry(&request, AcceptedIntentKind::Resume, false)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
        let original = cross_gate_binding(
            "run/cross-gate",
            "context/cross-gate",
            "dispatch/cross-gate",
            "report/cross-gate",
            "resource/cross-gate",
            "revision/cross-gate",
        );
        let substitutions = [
            cross_gate_binding(
                "run/cross-gate",
                "context/cross-gate",
                "dispatch/substituted",
                "report/cross-gate",
                "resource/cross-gate",
                "revision/cross-gate",
            ),
            cross_gate_binding(
                "run/cross-gate",
                "context/cross-gate",
                "dispatch/cross-gate",
                "report/substituted",
                "resource/cross-gate",
                "revision/cross-gate",
            ),
            cross_gate_binding(
                "run/cross-gate",
                "context/cross-gate",
                "dispatch/cross-gate",
                "report/cross-gate",
                "resource/substituted",
                "revision/cross-gate",
            ),
        ];

        for (index, replacement) in substitutions.iter().enumerate() {
            let authority = ReplacementAuthorityResolver {
                calls: Cell::new(0),
            };
            let error = replacement_executor::prove_cross_gate_replacement_executor_equivalence(
                &original_aggregate,
                &replacement_aggregate,
                &IntegratedRecipient {
                    actor: ActorId::new(format!("agent/original-gate-{index}"))
                        .expect("original recipient"),
                    posture: IntegratedReceiptPosture::Exact,
                },
                &IntegratedRecipient {
                    actor: ActorId::new(format!("agent/replacement-gate-{index}"))
                        .expect("replacement recipient"),
                    posture: IntegratedReceiptPosture::Exact,
                },
                &original,
                replacement,
                &authority,
            )
            .err()
            .expect("substituted gate binding must fail");

            assert_eq!(
                error.code(),
                "continuation_collection.replacement_executor.lifecycle_binding.mismatch"
            );
            assert_eq!(authority.calls.get(), 0);
        }
    }

    #[test]
    fn cross_gate_binding_rejects_run_context_dispatch_and_revision_failures() {
        let run = WorkflowRunId::new("run/cross-gate").expect("run");
        let context = hash("context/cross-gate");
        let revision = hash("revision/cross-gate");
        let observation = |report_run: WorkflowRunId,
                           report_context: SpecContentHash,
                           dispatches: Vec<SpecContentHash>|
         -> Result<
            cross_gate_lifecycle::CrossGateLifecycleObservation,
            WorkflowOsError,
        > {
            Ok(cross_gate_lifecycle::CrossGateLifecycleObservation::new(
                revision.clone(),
                cross_gate_lifecycle::AcceptedDispatchObservation::new(
                    run.clone(),
                    context.clone(),
                    dispatches,
                )?,
                cross_gate_lifecycle::VerifiedReportObservation::new(
                    report_run,
                    report_context,
                    hash("report/cross-gate"),
                ),
                cross_gate_lifecycle::CurrentResourceAuthorityObservation::new(
                    run.clone(),
                    context.clone(),
                    hash("resource/cross-gate"),
                ),
            ))
        };

        let missing_dispatch = cross_gate_lifecycle::AcceptedDispatchObservation::new(
            run.clone(),
            context.clone(),
            Vec::new(),
        )
        .err()
        .expect("missing dispatch must fail");
        assert_eq!(
            missing_dispatch.code(),
            "continuation_collection.cross_gate_lifecycle.dispatch.missing"
        );

        for (source, expected_code) in [
            (
                CrossGateLifecycleFixtureSource {
                    observation: observation(
                        WorkflowRunId::new("run/substituted").expect("substituted run"),
                        context.clone(),
                        vec![hash("dispatch/cross-gate")],
                    )
                    .expect("observation"),
                    current_revision: revision.clone(),
                },
                "continuation_collection.cross_gate_lifecycle.run.mismatch",
            ),
            (
                CrossGateLifecycleFixtureSource {
                    observation: observation(
                        run.clone(),
                        hash("context/substituted"),
                        vec![hash("dispatch/cross-gate")],
                    )
                    .expect("observation"),
                    current_revision: revision.clone(),
                },
                "continuation_collection.cross_gate_lifecycle.immutable_context.mismatch",
            ),
            (
                CrossGateLifecycleFixtureSource {
                    observation: observation(
                        run.clone(),
                        context.clone(),
                        vec![hash("dispatch/cross-gate")],
                    )
                    .expect("observation"),
                    current_revision: hash("revision/drifted"),
                },
                "continuation_collection.cross_gate_lifecycle.source.drifted",
            ),
        ] {
            let error = cross_gate_lifecycle::CrossGateLifecycleBinding::compose(&source)
                .err()
                .expect("invalid cross-gate observation must fail");
            assert_eq!(error.code(), expected_code);
        }
    }

    #[test]
    fn cross_gate_binding_is_order_stable_and_rejects_duplicate_dispatch() {
        let run = WorkflowRunId::new("run/cross-gate").expect("run");
        let context = hash("context/cross-gate");
        let revision = hash("revision/cross-gate");
        let compose = |dispatches| {
            let source = CrossGateLifecycleFixtureSource {
                observation: cross_gate_lifecycle::CrossGateLifecycleObservation::new(
                    revision.clone(),
                    cross_gate_lifecycle::AcceptedDispatchObservation::new(
                        run.clone(),
                        context.clone(),
                        dispatches,
                    )
                    .expect("dispatch observation"),
                    cross_gate_lifecycle::VerifiedReportObservation::new(
                        run.clone(),
                        context.clone(),
                        hash("report/cross-gate"),
                    ),
                    cross_gate_lifecycle::CurrentResourceAuthorityObservation::new(
                        run.clone(),
                        context.clone(),
                        hash("resource/cross-gate"),
                    ),
                ),
                current_revision: revision.clone(),
            };
            cross_gate_lifecycle::CrossGateLifecycleBinding::compose(&source)
                .expect("cross-gate binding")
        };
        let left = compose(vec![hash("dispatch/b"), hash("dispatch/a")]);
        let right = compose(vec![hash("dispatch/a"), hash("dispatch/b")]);
        assert!(left == right);

        let duplicate = cross_gate_lifecycle::AcceptedDispatchObservation::new(
            run,
            context,
            vec![hash("dispatch/a"), hash("dispatch/a")],
        )
        .err()
        .expect("duplicate dispatch must fail");
        assert_eq!(
            duplicate.code(),
            "continuation_collection.cross_gate_lifecycle.dispatch.duplicate"
        );
    }

    #[test]
    fn cross_gate_pause_stop_and_completed_work_never_resolve_authority() {
        let lifecycle = cross_gate_binding(
            "run/cross-gate",
            "context/cross-gate",
            "dispatch/cross-gate",
            "report/cross-gate",
            "resource/cross-gate",
            "revision/cross-gate",
        );
        for control in [AcceptedIntentKind::Pause, AcceptedIntentKind::Stop] {
            let request = replacement_request();
            let original = replacement_registry(&request, control, false)
                .compose(request.clone(), &request.anchor)
                .expect("original aggregate");
            let replacement = replacement_registry(&request, control, false)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
            let authority = ReplacementAuthorityResolver {
                calls: Cell::new(0),
            };
            let (_, _, proof) =
                replacement_executor::prove_cross_gate_replacement_executor_equivalence(
                    &original,
                    &replacement,
                    &IntegratedRecipient {
                        actor: ActorId::new(
                            format!("agent/original-cross-{control:?}").to_lowercase(),
                        )
                        .expect("original recipient"),
                        posture: IntegratedReceiptPosture::Exact,
                    },
                    &IntegratedRecipient {
                        actor: ActorId::new(
                            format!("agent/replacement-cross-{control:?}").to_lowercase(),
                        )
                        .expect("replacement recipient"),
                        posture: IntegratedReceiptPosture::Exact,
                    },
                    &lifecycle,
                    &lifecycle,
                    &authority,
                )
                .expect("cross-gate control proof");
            assert!(matches!(
                proof.authority_posture(),
                replacement_executor::ReplacementFreshAuthorityPosture::NotRequestedPaused
                    | replacement_executor::ReplacementFreshAuthorityPosture::NotRequestedStopped
            ));
            assert_eq!(authority.calls.get(), 0);
        }

        let request = replacement_request();
        let original = replacement_registry(&request, AcceptedIntentKind::Resume, true)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement = replacement_registry(&request, AcceptedIntentKind::Resume, true)
            .compose(request.clone(), &request.anchor)
            .expect("replacement aggregate");
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };
        let error = replacement_executor::prove_cross_gate_replacement_executor_equivalence(
            &original,
            &replacement,
            &IntegratedRecipient {
                actor: ActorId::new("agent/original-cross-repeat").expect("original recipient"),
                posture: IntegratedReceiptPosture::Exact,
            },
            &IntegratedRecipient {
                actor: ActorId::new("agent/replacement-cross-repeat")
                    .expect("replacement recipient"),
                posture: IntegratedReceiptPosture::Exact,
            },
            &lifecycle,
            &lifecycle,
            &authority,
        )
        .err()
        .expect("completed work cannot be resumed");
        assert_eq!(
            error.code(),
            "continuation_collection.replacement_executor.next_action.repeats_completed"
        );
        assert_eq!(authority.calls.get(), 0);
    }

    #[test]
    fn replacement_discloses_blocking_gaps_without_resolving_authority() {
        let request = replacement_request();
        let original_aggregate = registry(&request)
            .compose(request.clone(), &request.anchor)
            .expect("original blocked aggregate");
        let replacement_aggregate = registry(&request)
            .compose(request.clone(), &request.anchor)
            .expect("replacement blocked aggregate");
        let original_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/original-blocked").expect("original recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let replacement_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-blocked").expect("replacement recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };

        let (original, replacement, proof) =
            replacement_executor::prove_replacement_executor_equivalence(
                &original_aggregate,
                &replacement_aggregate,
                &original_recipient,
                &replacement_recipient,
                &authority,
            )
            .expect("blocked replacement proof");

        assert_eq!(original.gaps(), replacement.gaps());
        assert_ne!(replacement.gaps(), []);
        assert_eq!(
            replacement.posture(),
            ContinuationContextDecisionPosture::Blocked
        );
        assert_eq!(authority.calls.get(), 0);
        assert_eq!(
            proof.authority_posture(),
            replacement_executor::ReplacementFreshAuthorityPosture::NotRequestedBlocked
        );
    }

    #[test]
    fn replacement_pause_and_stop_never_resolve_authority_or_revive_superseded_resume() {
        for (control, expected_posture) in [
            (
                AcceptedIntentKind::Pause,
                replacement_executor::ReplacementFreshAuthorityPosture::NotRequestedPaused,
            ),
            (
                AcceptedIntentKind::Stop,
                replacement_executor::ReplacementFreshAuthorityPosture::NotRequestedStopped,
            ),
        ] {
            let request = replacement_request();
            let original_aggregate = replacement_registry(&request, control, false)
                .compose(request.clone(), &request.anchor)
                .expect("original aggregate");
            let replacement_aggregate = replacement_registry(&request, control, false)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
            let original_recipient = IntegratedRecipient {
                actor: ActorId::new(format!("agent/original-{control:?}").to_lowercase())
                    .expect("original recipient"),
                posture: IntegratedReceiptPosture::Exact,
            };
            let replacement_recipient = IntegratedRecipient {
                actor: ActorId::new(format!("agent/replacement-{control:?}").to_lowercase())
                    .expect("replacement recipient"),
                posture: IntegratedReceiptPosture::Exact,
            };
            let authority = ReplacementAuthorityResolver {
                calls: Cell::new(0),
            };

            let (_, replacement, proof) =
                replacement_executor::prove_replacement_executor_equivalence(
                    &original_aggregate,
                    &replacement_aggregate,
                    &original_recipient,
                    &replacement_recipient,
                    &authority,
                )
                .expect("control disposition proof");

            assert_eq!(proof.authority_posture(), expected_posture);
            assert_eq!(authority.calls.get(), 0);
            assert!(!replacement.claims().iter().any(|claim| {
                claim.value().map(ContinuationContextText::as_str)
                    == Some("resume stale completed work")
            }));
        }
    }

    #[test]
    fn replacement_rejects_a_lawful_next_option_that_repeats_completed_work() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, true)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate =
            replacement_registry(&request, AcceptedIntentKind::Resume, true)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
        let original_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/original-repeat").expect("original recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let replacement_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-repeat").expect("replacement recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };

        let error = replacement_executor::prove_replacement_executor_equivalence(
            &original_aggregate,
            &replacement_aggregate,
            &original_recipient,
            &replacement_recipient,
            &authority,
        )
        .err()
        .expect("completed work cannot become the next action");

        assert_eq!(
            error.code(),
            "continuation_collection.replacement_executor.next_action.repeats_completed"
        );
        assert_eq!(authority.calls.get(), 0);
    }

    #[test]
    fn completed_work_identity_is_projected_only_from_a_selected_authoritative_source() {
        for scenario in 0..4 {
            let request = replacement_request();
            let aggregate = replacement_registry_with_work_report_collector(
                &request,
                AcceptedIntentKind::Resume,
                false,
                move |collector| match scenario {
                    0 => {
                        collector.posture = ContinuationCollectionResultPosture::Stale;
                        collector
                            .source
                            .as_mut()
                            .expect("work report source")
                            .freshness = ContinuationContextFreshness::StaleBlocking;
                    }
                    1 => {
                        collector
                            .source
                            .as_mut()
                            .expect("work report source")
                            .access = ContinuationContextAccess::Denied;
                    }
                    2 => {
                        collector
                            .source
                            .as_mut()
                            .expect("work report source")
                            .integrity = ContinuationContextIntegrity::Invalid;
                    }
                    3 => {
                        collector
                            .source
                            .as_mut()
                            .expect("work report source")
                            .materialized_bytes = request.materialization_budget_bytes + 1;
                    }
                    _ => unreachable!("closed source-admission scenario"),
                },
            )
            .compose(request.clone(), &request.anchor)
            .expect("source disposition must remain inspectable");

            assert!(!aggregate.work_identities.iter().any(|identity| {
                identity.disposition == ContinuationCollectedWorkDisposition::Completed
            }));
            assert!(aggregate
                .decision
                .bundle()
                .selected_source_ids()
                .iter()
                .all(|source_id| source_id.as_str() != "source/workreport"));
        }
    }

    #[test]
    fn replacement_rejects_another_aggregate_before_authority_resolution() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, false)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate = registry(&request)
            .compose(request.clone(), &request.anchor)
            .expect("different aggregate");
        let original_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/original-mismatch").expect("original recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let replacement_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-mismatch").expect("replacement recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };

        let error = replacement_executor::prove_replacement_executor_equivalence(
            &original_aggregate,
            &replacement_aggregate,
            &original_recipient,
            &replacement_recipient,
            &authority,
        )
        .err()
        .expect("aggregate substitution must fail");

        assert_eq!(
            error.code(),
            "continuation_collection.replacement_executor.aggregate.mismatch"
        );
        assert_eq!(authority.calls.get(), 0);
    }

    #[test]
    fn replacement_rejects_operation_identity_substitution_before_authority_resolution() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, false)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate =
            replacement_registry(&request, AcceptedIntentKind::Resume, true)
                .compose(request.clone(), &request.anchor)
                .expect("substituted operation aggregate");
        let original_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/original-operation").expect("original recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let replacement_recipient = IntegratedRecipient {
            actor: ActorId::new("agent/replacement-operation").expect("replacement recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };

        let error = replacement_executor::prove_replacement_executor_equivalence(
            &original_aggregate,
            &replacement_aggregate,
            &original_recipient,
            &replacement_recipient,
            &authority,
        )
        .err()
        .expect("operation identity substitution must fail");

        assert_eq!(
            error.code(),
            "continuation_collection.replacement_executor.aggregate.mismatch"
        );
        assert_eq!(authority.calls.get(), 0);
    }

    #[test]
    fn replacement_rejects_recipient_identity_reuse_before_authority_resolution() {
        let request = replacement_request();
        let original_aggregate = replacement_registry(&request, AcceptedIntentKind::Resume, false)
            .compose(request.clone(), &request.anchor)
            .expect("original aggregate");
        let replacement_aggregate =
            replacement_registry(&request, AcceptedIntentKind::Resume, false)
                .compose(request.clone(), &request.anchor)
                .expect("replacement aggregate");
        let recipient = IntegratedRecipient {
            actor: ActorId::new("agent/reused-executor").expect("recipient"),
            posture: IntegratedReceiptPosture::Exact,
        };
        let authority = ReplacementAuthorityResolver {
            calls: Cell::new(0),
        };

        let error = replacement_executor::prove_replacement_executor_equivalence(
            &original_aggregate,
            &replacement_aggregate,
            &recipient,
            &recipient,
            &authority,
        )
        .err()
        .expect("replacement identity must be distinct");

        assert_eq!(
            error.code(),
            "continuation_collection.replacement_executor.recipient.not_distinct"
        );
        assert_eq!(authority.calls.get(), 0);
    }

    #[test]
    fn collector_kind_substitution_fails_closed() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        collectors[0].returned_kind = Some(ContinuationCollectorKind::Policy);
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");
        let error = registry
            .collect(request.clone(), &request.anchor)
            .expect_err("kind substitution must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.collector.kind_substitution"
        );
    }

    #[test]
    fn changed_source_fence_invalidates_the_complete_batch() {
        let request = request(true);
        let mut after = request.anchor.clone();
        after.source_revisions[0].revision = hash("changed");
        after.commitment = collection_commitment(
            "anchor",
            &(
                &after.immutable_bundle,
                &after.event_cursor,
                &after.source_revisions,
            ),
        )
        .expect("commitment");
        let error = registry(&request)
            .collect(request, &after)
            .expect_err("anchor drift must fail");
        assert_eq!(error.code(), "continuation_collection.anchor.drifted");
    }

    #[test]
    fn observed_source_gets_core_owned_class_and_authority() {
        let request = request(false);
        let kind = ContinuationCollectorKind::ValidatedDeclaration;
        let source = ContinuationCollectedSource {
            source_id: ContinuationContextSourceId::new("source/declaration").expect("source"),
            freshness: ContinuationContextFreshness::Current,
            integrity: ContinuationContextIntegrity::Verified,
            access: ContinuationContextAccess::Permitted,
            sensitivity: ContinuationContextSensitivity::Internal,
            materialized_bytes: 128,
            claims: vec![ContinuationUnderstandingCandidate {
                kind: ContinuationUnderstandingClaimKind::Objective,
                value: ContinuationContextText::new("bounded objective").expect("text"),
            }],
            work_identities: Vec::new(),
        };
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|item| default_collector(*item, &request))
            .collect::<Vec<_>>();
        let target = collectors
            .iter_mut()
            .find(|collector| collector.kind == kind)
            .expect("collector");
        target.posture = ContinuationCollectionResultPosture::Observed;
        target.source = Some(source);
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");

        let input = registry
            .collect(request.clone(), &request.anchor)
            .expect("input");
        let source = input.sources.first().expect("source");
        assert_eq!(source.source_class, kind.source_class());
        assert_eq!(source.authority_domain, kind.authority_domain());
    }

    #[test]
    fn unavailable_and_stale_remain_distinct() {
        assert_ne!(
            collector_outcome(ContinuationCollectionResultPosture::Unavailable),
            collector_outcome(ContinuationCollectionResultPosture::Failed)
        );
        assert_ne!(
            collector_outcome(ContinuationCollectionResultPosture::Stale),
            collector_outcome(ContinuationCollectionResultPosture::Failed)
        );
    }

    #[test]
    fn stale_and_unavailable_postures_survive_input_conversion() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        let git = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::GitObservation)
            .expect("git collector");
        git.posture = ContinuationCollectionResultPosture::Stale;
        git.source = Some(ContinuationCollectedSource {
            source_id: ContinuationContextSourceId::new("source/git").expect("source"),
            freshness: ContinuationContextFreshness::StaleBlocking,
            integrity: ContinuationContextIntegrity::Verified,
            access: ContinuationContextAccess::Permitted,
            sensitivity: ContinuationContextSensitivity::Internal,
            materialized_bytes: 32,
            claims: Vec::new(),
            work_identities: Vec::new(),
        });
        let provider = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::ProviderObservation)
            .expect("provider collector");
        provider.posture = ContinuationCollectionResultPosture::Unavailable;
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");

        let input = registry
            .collect(request.clone(), &request.anchor)
            .expect("input");
        assert_eq!(
            input
                .coverage
                .iter()
                .find(|entry| entry.source_class == ContinuationContextSourceClass::GitObservation)
                .expect("git coverage")
                .outcome,
            ContinuationContextCollectorOutcome::Stale
        );
        assert_eq!(
            input
                .coverage
                .iter()
                .find(|entry| {
                    entry.source_class == ContinuationContextSourceClass::ProviderObservation
                })
                .expect("provider coverage")
                .outcome,
            ContinuationContextCollectorOutcome::Unavailable
        );
    }

    #[test]
    fn source_revision_substitution_fails_closed() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        collectors[0].revision = Some(hash("substituted-revision"));
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");
        let error = registry
            .collect(request.clone(), &request.anchor)
            .expect_err("revision substitution must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.revision_mismatch"
        );
    }

    #[test]
    fn inapplicable_collector_cannot_claim_support() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        let immutable = collectors
            .iter_mut()
            .find(|collector| collector.kind == ContinuationCollectorKind::ImmutableRunBundle)
            .expect("immutable collector");
        immutable.posture = ContinuationCollectionResultPosture::Unsupported;
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");
        let error = registry
            .collect(request.clone(), &request.anchor)
            .expect_err("inapplicable substitution must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.expectation_mismatch"
        );
    }

    #[test]
    fn debug_output_redacts_bindings_and_source_payloads() {
        let request = request(false);
        let secret = "private-continuation-payload";
        let observation = ContinuationCollectionObservation {
            kind: ContinuationCollectorKind::ValidatedDeclaration,
            request_commitment: hash(secret),
            revision: hash(secret),
            posture: ContinuationCollectionResultPosture::Observed,
            source: Some(ContinuationCollectedSource {
                source_id: ContinuationContextSourceId::new("source/private").expect("source"),
                freshness: ContinuationContextFreshness::Current,
                integrity: ContinuationContextIntegrity::Verified,
                access: ContinuationContextAccess::Permitted,
                sensitivity: ContinuationContextSensitivity::Restricted,
                materialized_bytes: 64,
                claims: vec![ContinuationUnderstandingCandidate {
                    kind: ContinuationUnderstandingClaimKind::Objective,
                    value: ContinuationContextText::new(secret).expect("text"),
                }],
                work_identities: Vec::new(),
            }),
            accepted_intents: Vec::new(),
            accounting: Vec::new(),
        };

        assert!(!format!("{request:?}").contains("project/continuation"));
        assert!(!format!("{observation:?}").contains(secret));
        assert!(!format!("{observation:?}").contains("source/private"));
    }

    #[test]
    fn accepted_intent_observation_rejects_generic_source_payload() {
        let request = request(false);
        let kind = ContinuationCollectorKind::AcceptedIntent;
        let source = ContinuationCollectedSource {
            source_id: ContinuationContextSourceId::new("source/intent").expect("source"),
            freshness: ContinuationContextFreshness::Current,
            integrity: ContinuationContextIntegrity::Verified,
            access: ContinuationContextAccess::Permitted,
            sensitivity: ContinuationContextSensitivity::Internal,
            materialized_bytes: 1,
            claims: Vec::new(),
            work_identities: Vec::new(),
        };
        let mut observation = ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: request.anchor.revision_for(kind).expect("revision").clone(),
            posture: ContinuationCollectionResultPosture::Observed,
            source: Some(source),
            accepted_intents: Vec::new(),
            accounting: Vec::new(),
        };
        let error = validate_result_shape(
            &observation,
            collector_expectation_for(request.purpose, false, kind.source_class()),
            request.purpose,
            false,
        )
        .expect_err("accepted intent must not use a generic source payload");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.accepted_intent_shape"
        );
        observation.posture = ContinuationCollectionResultPosture::Unsupported;
        observation.source = None;
        validate_result_shape(
            &observation,
            collector_expectation_for(request.purpose, false, kind.source_class()),
            request.purpose,
            false,
        )
        .expect("unsupported posture");
    }

    #[test]
    fn work_identity_must_bind_the_exact_authoritative_claim() {
        let request = replacement_request();
        let kind = ContinuationCollectorKind::WorkReport;
        let mut source = collected_source(
            kind,
            &[(
                ContinuationUnderstandingClaimKind::CompletedObligation,
                "completed replacement proof",
            )],
        );
        source.work_identities = vec![ContinuationCollectedWorkIdentity {
            identity: hash("work/replacement-proof"),
            source_binding: hash("unrelated-claim-binding"),
            disposition: ContinuationCollectedWorkDisposition::Completed,
        }];
        let observation = ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: request.anchor.revision_for(kind).expect("revision").clone(),
            posture: ContinuationCollectionResultPosture::Observed,
            source: Some(source),
            accepted_intents: Vec::new(),
            accounting: Vec::new(),
        };

        let error = validate_result_shape(
            &observation,
            collector_expectation_for(
                request.purpose,
                request.scope.run_id.is_some(),
                kind.source_class(),
            ),
            request.purpose,
            request.scope.run_id.is_some(),
        )
        .expect_err("unrelated claim binding must fail");

        assert_eq!(
            error.code(),
            "continuation_collection.observation.work_identity_binding_mismatch"
        );
    }

    #[test]
    fn accounting_cannot_accompany_failure_posture() {
        let request = request(false);
        let kind = ContinuationCollectorKind::Policy;
        let observation = ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: request.anchor.revision_for(kind).expect("revision").clone(),
            posture: ContinuationCollectionResultPosture::Failed,
            source: None,
            accepted_intents: Vec::new(),
            accounting: vec![ContinuationCollectedAccounting {
                claim_kind: ContinuationUnderstandingClaimKind::Decision,
            }],
        };
        let error = validate_result_shape(
            &observation,
            collector_expectation_for(request.purpose, false, kind.source_class()),
            request.purpose,
            false,
        )
        .expect_err("failure accounting must be rejected");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.accounting_shape"
        );
    }

    #[test]
    fn request_binding_substitution_fails_closed() {
        let request = request(false);
        let mut collectors = ALL_COLLECTOR_KINDS
            .iter()
            .map(|kind| default_collector(*kind, &request))
            .collect::<Vec<_>>();
        collectors[0].request_commitment = Some(hash("different-request"));
        let registry = InMemoryContinuationCollectorRegistry::new(
            collectors
                .into_iter()
                .map(|collector| Box::new(collector) as Box<dyn ContinuationContextCollector>)
                .collect(),
        )
        .expect("registry");
        let error = registry
            .collect(request.clone(), &request.anchor)
            .expect_err("request substitution must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.request_mismatch"
        );
    }

    #[test]
    fn empty_posture_requires_complete_typed_accounting() {
        let request = request(false);
        let kind = ALL_COLLECTOR_KINDS
            .iter()
            .copied()
            .find(|kind| {
                !empty_accounting_claims_for(request.purpose, false, kind.source_class())
                    .expect("accounting contract")
                    .is_empty()
            })
            .expect("at least one source owns negative-knowledge accounting");
        let expected = empty_accounting_claims_for(request.purpose, false, kind.source_class())
            .expect("accounting contract");
        let mut observation = ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: request.anchor.revision_for(kind).expect("revision").clone(),
            posture: ContinuationCollectionResultPosture::Empty,
            source: None,
            accepted_intents: Vec::new(),
            accounting: Vec::new(),
        };

        let error = validate_result_shape(
            &observation,
            collector_expectation_for(request.purpose, false, kind.source_class()),
            request.purpose,
            false,
        )
        .expect_err("incomplete empty accounting must fail");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.empty_accounting_incomplete"
        );

        observation.accounting = expected
            .into_iter()
            .map(|claim_kind| ContinuationCollectedAccounting { claim_kind })
            .collect();
        validate_result_shape(
            &observation,
            collector_expectation_for(request.purpose, false, kind.source_class()),
            request.purpose,
            false,
        )
        .expect("complete typed accounting");
    }
}
