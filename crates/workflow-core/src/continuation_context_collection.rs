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
    canonical_commitment, collector_expectation_for, empty_accounting_claims_for,
    ContinuationClaimAccountingEntry, ContinuationClaimAccountingOutcome,
    ContinuationContextAccess, ContinuationContextAuthorityDomain,
    ContinuationContextCollectorExpectation, ContinuationContextCollectorOutcome,
    ContinuationContextDecisionInput, ContinuationContextFreshness, ContinuationContextIntegrity,
    ContinuationContextPurpose, ContinuationContextScope, ContinuationContextSensitivity,
    ContinuationContextSourceClass, ContinuationContextSourceCoverageEntry,
    ContinuationContextSourceDefinition, ContinuationContextSourceId,
    ContinuationUnderstandingCandidate, ContinuationUnderstandingClaimKind,
};
use crate::{ActorId, SpecContentHash, WorkflowOsError};

const COLLECTION_ALGORITHM: &str = "workflow-os/continuation-collection/v1";
const MAX_OBSERVATIONS: usize = ALL_COLLECTOR_KINDS.len();
const MAX_CLAIMS_PER_OBSERVATION: usize = 64;
const MAX_ACCOUNTING_PER_OBSERVATION: usize = 64;

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
            .finish()
    }
}

/// Bounded typed empty-accounting claim owned by the collector kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
struct ContinuationCollectedAccounting {
    claim_kind: ContinuationUnderstandingClaimKind,
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
        let mut observations = Vec::with_capacity(self.collectors.len());
        for (expected_kind, collector) in &self.collectors {
            let observation = collector.collect(&request)?;
            if observation.kind != *expected_kind {
                return Err(collection_error(
                    "collector.kind_substitution",
                    "collector returned an observation for a different collector kind",
                ));
            }
            observations.push(observation);
        }
        compose_collection(request, after_anchor, observations)
    }
}

fn compose_collection(
    request: ContinuationCollectionRequest,
    after_anchor: &ContinuationCollectionAnchor,
    mut observations: Vec<ContinuationCollectionObservation>,
) -> Result<ContinuationContextDecisionInput, WorkflowOsError> {
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
        if observation.accounting.len() > MAX_ACCOUNTING_PER_OBSERVATION {
            return Err(collection_error(
                "observation.accounting_limit",
                "collector observation exceeds the accounting-entry limit",
            ));
        }

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
    Ok(ContinuationContextDecisionInput {
        purpose: request.purpose,
        scope: request.scope,
        actor: request.actor,
        maximum_sensitivity: request.maximum_sensitivity,
        materialization_budget_bytes: request.materialization_budget_bytes,
        coverage,
        claim_accounting,
        sources,
        accepted_intents: Vec::new(),
    })
}

fn validate_result_shape(
    observation: &ContinuationCollectionObservation,
    expectation: ContinuationContextCollectorExpectation,
    purpose: ContinuationContextPurpose,
    run_bound: bool,
) -> Result<(), WorkflowOsError> {
    if observation.kind == ContinuationCollectorKind::AcceptedIntent
        && observation.posture != ContinuationCollectionResultPosture::Unsupported
    {
        return Err(collection_error(
            "observation.accepted_intent_unavailable",
            "accepted-intent collection remains unsupported until its capture boundary exists",
        ));
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

    let source_required = matches!(
        observation.posture,
        ContinuationCollectionResultPosture::Observed | ContinuationCollectionResultPosture::Stale
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

    use super::*;
    use crate::continuation_context::{
        derive_continuation_context, ContinuationContextDecisionPosture, ContinuationContextText,
    };
    use crate::{ProjectId, WorkflowId, WorkflowRunId};

    #[derive(Clone)]
    struct FixtureCollector {
        kind: ContinuationCollectorKind,
        posture: ContinuationCollectionResultPosture,
        source: Option<ContinuationCollectedSource>,
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
                source: self.source.clone(),
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
            accounting: Vec::new(),
            returned_kind: None,
            request_commitment: None,
            revision: None,
        }
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
            }),
            accounting: Vec::new(),
        };

        assert!(!format!("{request:?}").contains("project/continuation"));
        assert!(!format!("{observation:?}").contains(secret));
        assert!(!format!("{observation:?}").contains("source/private"));
    }

    #[test]
    fn accepted_intent_cannot_be_observed_before_capture_boundary() {
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
        };
        let mut observation = ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: request.anchor.revision_for(kind).expect("revision").clone(),
            posture: ContinuationCollectionResultPosture::Observed,
            source: Some(source),
            accounting: Vec::new(),
        };
        let error = validate_result_shape(
            &observation,
            collector_expectation_for(request.purpose, false, kind.source_class()),
            request.purpose,
            false,
        )
        .expect_err("accepted intent must remain unsupported");
        assert_eq!(
            error.code(),
            "continuation_collection.observation.accepted_intent_unavailable"
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
    fn accounting_cannot_accompany_failure_posture() {
        let request = request(false);
        let kind = ContinuationCollectorKind::Policy;
        let observation = ContinuationCollectionObservation {
            kind,
            request_commitment: request.commitment.clone(),
            revision: request.anchor.revision_for(kind).expect("revision").clone(),
            posture: ContinuationCollectionResultPosture::Failed,
            source: None,
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
