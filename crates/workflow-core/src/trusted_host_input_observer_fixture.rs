//! Process-free fixture proof for trusted-host input observation.
//!
//! This module accepts explicit synthetic facts and constructs the accepted
//! non-authorizing observer result. It does not inspect a host, read files or
//! environment state, run a process, select a provider, or grant authority.

use std::fmt;

use crate::{
    TrustedHostInputObservationFact, TrustedHostInputObservationProducer,
    TrustedHostInputObservationProducerAssurance, TrustedHostInputObservationProducerKind,
    TrustedHostInputObservationRequest, TrustedHostInputObservationResult,
    TrustedHostInputObservationResultDefinition, TrustedHostRootObservation, WorkflowOsError,
};

/// Explicit synthetic source for one process-free observer proof.
#[derive(Clone, Eq, PartialEq)]
pub struct TrustedHostInputObservationFixture {
    producer: TrustedHostInputObservationProducer,
    started_revision: u64,
    completed_revision: u64,
    repository_root: TrustedHostRootObservation,
    facts: Vec<TrustedHostInputObservationFact>,
}

impl TrustedHostInputObservationFixture {
    /// Creates one explicit fixture source.
    ///
    /// # Errors
    ///
    /// Rejects a producer that is not explicitly fixture-only. Complete fact,
    /// root, ordering, and request coherence are validated when the fixture is
    /// observed through [`observe_trusted_host_inputs_from_fixture`].
    pub fn new(
        producer: TrustedHostInputObservationProducer,
        started_revision: u64,
        completed_revision: u64,
        repository_root: TrustedHostRootObservation,
        facts: Vec<TrustedHostInputObservationFact>,
    ) -> Result<Self, WorkflowOsError> {
        if producer.kind() != TrustedHostInputObservationProducerKind::Fixture
            || producer.assurance() != TrustedHostInputObservationProducerAssurance::FixtureOnly
        {
            return Err(WorkflowOsError::validation(
                "trusted_host_input_observer.fixture.producer.invalid",
                "trusted-host input observation fixture producer is invalid",
            ));
        }
        Ok(Self {
            producer,
            started_revision,
            completed_revision,
            repository_root,
            facts,
        })
    }
}

impl fmt::Debug for TrustedHostInputObservationFixture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostInputObservationFixture")
            .field("producer", &self.producer)
            .field("started_revision", &self.started_revision)
            .field("completed_revision", &self.completed_revision)
            .field("repository_root", &self.repository_root)
            .field("fact_count", &self.facts.len())
            .finish()
    }
}

/// Constructs one exact-request-bound result from explicit synthetic facts.
///
/// # Errors
///
/// Fails closed when the fixture is incomplete, incoherent, stale,
/// substituted, or incompatible with the exact request. The helper uses the
/// accepted result constructor and request validator as its enforcement
/// boundary.
pub fn observe_trusted_host_inputs_from_fixture(
    request: &TrustedHostInputObservationRequest,
    fixture: &TrustedHostInputObservationFixture,
) -> Result<TrustedHostInputObservationResult, WorkflowOsError> {
    let result =
        TrustedHostInputObservationResult::new(TrustedHostInputObservationResultDefinition {
            request_fingerprint: request.fingerprint().clone(),
            repository_tree: request.repository_tree().clone(),
            ordering_authority: request.ordering_authority().clone(),
            producer: fixture.producer.clone(),
            started_revision: fixture.started_revision,
            completed_revision: fixture.completed_revision,
            repository_root: fixture.repository_root.clone(),
            facts: fixture.facts.clone(),
        })?;
    result.validate_for_request(request)?;
    Ok(result)
}
