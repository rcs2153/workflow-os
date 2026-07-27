use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    CapabilityAvailabilityRecord, CapabilityGrant, CapabilityReference, CapabilityResourceScope,
    GovernedContextAccessLevel, GovernedContextReferenceTarget, RequiredContextContractBinding,
    RequiredContextExecutionBinding, RequiredContextObligation, RequiredContextRequirement,
    RequiredContextRequirementId, SpecContentHash, Timestamp, WorkReportSensitivity,
    WorkflowOsError,
};

/// Versioned current-authority fact-set model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentAuthorityFactSetVersion {
    /// Initial payload-free commitment model.
    V1,
}

impl<'de> Deserialize<'de> for CurrentAuthorityFactSetVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "authority fact-set version is invalid",
            )),
        }
    }
}

/// Bounded source vocabulary for fact-set commitments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityFactSourceKind {
    /// Caller-owned in-memory inventory snapshot used by the model-only slice.
    InMemoryInventorySnapshot,
}

impl<'de> Deserialize<'de> for AuthorityFactSourceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "in_memory_inventory_snapshot" => Ok(Self::InMemoryInventorySnapshot),
            _ => Err(serde::de::Error::custom(
                "authority fact source kind is invalid",
            )),
        }
    }
}

/// Completeness claimed by the committed source snapshot.
///
/// This is commitment vocabulary only. It is not trusted authority unless a
/// future Core-owned source independently proves the claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityFactCompletenessPosture {
    /// The source claims complete coverage for the exact query set.
    CompleteForExactQuery,
    /// The source was unavailable.
    Unavailable,
    /// Completeness is unknown.
    Unknown,
}

impl<'de> Deserialize<'de> for AuthorityFactCompletenessPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "complete_for_exact_query" => Ok(Self::CompleteForExactQuery),
            "unavailable" => Ok(Self::Unavailable),
            "unknown" => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(
                "authority fact completeness posture is invalid",
            )),
        }
    }
}

/// One exact query derived from a required-context requirement.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CurrentAuthorityQuery {
    requirement_id: RequiredContextRequirementId,
    target: GovernedContextReferenceTarget,
    access_level: GovernedContextAccessLevel,
    obligation: RequiredContextObligation,
    maximum_sensitivity: WorkReportSensitivity,
    capability: CapabilityReference,
    resource: CapabilityResourceScope,
}

impl CurrentAuthorityQuery {
    fn from_requirement(requirement: &RequiredContextRequirement) -> Result<Self, WorkflowOsError> {
        Ok(Self {
            requirement_id: requirement.requirement_id().clone(),
            target: requirement.target().clone(),
            access_level: requirement.access_level(),
            obligation: requirement.obligation(),
            maximum_sensitivity: requirement.maximum_sensitivity(),
            capability: requirement.access_level().required_capability()?,
            resource: requirement.target().capability_resource()?,
        })
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.capability != self.access_level.required_capability()?
            || self.resource != self.target.capability_resource()?
        {
            return Err(fact_error(
                "query.derived_mismatch",
                "authority query does not match its typed target and access level",
            ));
        }
        if self.maximum_sensitivity == WorkReportSensitivity::Unknown {
            return Err(fact_error(
                "query.sensitivity_unknown",
                "authority query sensitivity must be known",
            ));
        }
        Ok(())
    }

    /// Returns the source requirement ID.
    #[must_use]
    pub const fn requirement_id(&self) -> &RequiredContextRequirementId {
        &self.requirement_id
    }

    /// Returns the derived capability.
    #[must_use]
    pub const fn capability(&self) -> &CapabilityReference {
        &self.capability
    }

    /// Returns the derived resource.
    #[must_use]
    pub const fn resource(&self) -> &CapabilityResourceScope {
        &self.resource
    }
}

impl fmt::Debug for CurrentAuthorityQuery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthorityQuery")
            .field("requirement_id", &"[REDACTED]")
            .field("target_kind", &self.target.kind())
            .field("access_level", &self.access_level)
            .field("obligation", &self.obligation)
            .field("maximum_sensitivity", &self.maximum_sensitivity)
            .field("capability", &"[REDACTED]")
            .field("resource", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthorityQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            requirement_id: RequiredContextRequirementId,
            target: GovernedContextReferenceTarget,
            access_level: GovernedContextAccessLevel,
            obligation: RequiredContextObligation,
            maximum_sensitivity: WorkReportSensitivity,
            capability: CapabilityReference,
            resource: CapabilityResourceScope,
        }
        let wire = Wire::deserialize(deserializer)?;
        let query = Self {
            requirement_id: wire.requirement_id,
            target: wire.target,
            access_level: wire.access_level,
            obligation: wire.obligation,
            maximum_sensitivity: wire.maximum_sensitivity,
            capability: wire.capability,
            resource: wire.resource,
        };
        query
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid current authority query"))?;
        Ok(query)
    }
}

/// Canonical complete query set derived from one exact contract.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CurrentAuthorityQuerySet {
    contract_content_hash: SpecContentHash,
    queries: Vec<CurrentAuthorityQuery>,
    query_set_hash: SpecContentHash,
}

impl CurrentAuthorityQuerySet {
    /// Derives the complete canonical query set.
    ///
    /// # Errors
    ///
    /// Returns a stable error for invalid or duplicate derived queries.
    pub fn from_contract(
        contract: &RequiredContextContractBinding,
    ) -> Result<Self, WorkflowOsError> {
        let mut queries = contract
            .requirements()
            .iter()
            .map(CurrentAuthorityQuery::from_requirement)
            .collect::<Result<Vec<_>, _>>()?;
        queries.sort_by(|left, right| {
            left.requirement_id
                .as_str()
                .cmp(right.requirement_id.as_str())
        });
        validate_queries(&queries)?;
        let query_set_hash = hash_serializable("query-set", &queries)?;
        Ok(Self {
            contract_content_hash: contract.content_hash().clone(),
            queries,
            query_set_hash,
        })
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_queries(&self.queries)?;
        if self.query_set_hash != hash_serializable("query-set", &self.queries)? {
            return Err(fact_error(
                "query_set.hash_mismatch",
                "authority query-set hash is invalid",
            ));
        }
        Ok(())
    }

    /// Returns the complete canonical queries.
    #[must_use]
    pub fn queries(&self) -> &[CurrentAuthorityQuery] {
        &self.queries
    }

    /// Returns the deterministic query-set hash.
    #[must_use]
    pub const fn query_set_hash(&self) -> &SpecContentHash {
        &self.query_set_hash
    }
}

impl fmt::Debug for CurrentAuthorityQuerySet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthorityQuerySet")
            .field("contract_content_hash", &"[REDACTED]")
            .field("query_count", &self.queries.len())
            .field("query_set_hash", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthorityQuerySet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            contract_content_hash: SpecContentHash,
            queries: Vec<CurrentAuthorityQuery>,
            query_set_hash: SpecContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let value = Self {
            contract_content_hash: wire.contract_content_hash,
            queries: wire.queries,
            query_set_hash: wire.query_set_hash,
        };
        value
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid current authority query set"))?;
        Ok(value)
    }
}

/// Payload-free source snapshot commitment.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorityFactSourceBinding {
    kind: AuthorityFactSourceKind,
    snapshot_hash: SpecContentHash,
    observed_at: Timestamp,
    completeness: AuthorityFactCompletenessPosture,
    query_set_hash: SpecContentHash,
    grant_count: u64,
    availability_count: u64,
    records_hash: SpecContentHash,
}

impl fmt::Debug for AuthorityFactSourceBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorityFactSourceBinding")
            .field("kind", &self.kind)
            .field("snapshot_hash", &"[REDACTED]")
            .field("observed_at", &"[REDACTED]")
            .field("completeness", &self.completeness)
            .field("query_set_hash", &"[REDACTED]")
            .field("grant_count", &self.grant_count)
            .field("availability_count", &self.availability_count)
            .field("records_hash", &"[REDACTED]")
            .finish()
    }
}

/// Explicit model-only fact-set construction inputs.
pub struct CurrentAuthorityFactSetInput<'a> {
    /// Exact immutable execution commitment.
    pub execution_binding: &'a RequiredContextExecutionBinding,
    /// Exact required-context contract.
    pub contract: &'a RequiredContextContractBinding,
    /// Source kind.
    pub source_kind: AuthorityFactSourceKind,
    /// Caller-owned source snapshot commitment.
    pub source_snapshot_hash: SpecContentHash,
    /// Source observation time.
    pub source_observed_at: Timestamp,
    /// Claimed source completeness.
    pub completeness: AuthorityFactCompletenessPosture,
    /// Time represented by the fact set.
    pub evaluated_at: Timestamp,
    /// Candidate grants for the exact query set.
    pub grants: Vec<CapabilityGrant>,
    /// Availability observations for the exact query set.
    pub availability_records: Vec<CapabilityAvailabilityRecord>,
}

/// Deterministic payload-free commitment to supplied current authority facts.
///
/// This model exposes no authorization or readiness decision. Source
/// completeness remains a claim until a future Core-owned source proves it.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct CurrentAuthorityFactSet {
    version: CurrentAuthorityFactSetVersion,
    execution_binding_hash: SpecContentHash,
    query_set: CurrentAuthorityQuerySet,
    source: AuthorityFactSourceBinding,
    evaluated_at: Timestamp,
    grants: Vec<CapabilityGrant>,
    availability_records: Vec<CapabilityAvailabilityRecord>,
    fact_set_hash: SpecContentHash,
}

impl CurrentAuthorityFactSet {
    /// Creates a model-only commitment to supplied current authority facts.
    ///
    /// # Errors
    ///
    /// Returns stable non-leaking errors for mismatched, duplicate, incomplete,
    /// or temporally inconsistent input.
    pub fn new(input: CurrentAuthorityFactSetInput<'_>) -> Result<Self, WorkflowOsError> {
        if input.execution_binding.contract_content_hash() != input.contract.content_hash() {
            return Err(fact_error(
                "contract.mismatch",
                "authority fact set contract does not match execution binding",
            ));
        }
        if input.evaluated_at < input.execution_binding.bound_at()
            || input.source_observed_at > input.evaluated_at
        {
            return Err(fact_error(
                "time.invalid",
                "authority fact-set times are inconsistent",
            ));
        }
        let query_set = CurrentAuthorityQuerySet::from_contract(input.contract)?;
        let mut grants = input.grants;
        grants.sort_by(|left, right| left.grant_id().as_str().cmp(right.grant_id().as_str()));
        if grants
            .windows(2)
            .any(|pair| pair[0].grant_id() == pair[1].grant_id())
        {
            return Err(fact_error(
                "grant.duplicate",
                "authority fact set contains duplicate grants",
            ));
        }
        let mut availability_records = input.availability_records;
        availability_records.sort_by(availability_order);
        if availability_records
            .windows(2)
            .any(|pair| same_availability_key(&pair[0], &pair[1]))
        {
            return Err(fact_error(
                "availability.duplicate",
                "authority fact set contains duplicate availability records",
            ));
        }
        validate_record_scope(&query_set, &grants, &availability_records)?;
        if input.completeness == AuthorityFactCompletenessPosture::CompleteForExactQuery
            && availability_records.len() != query_set.queries.len()
        {
            return Err(fact_error(
                "availability.incomplete",
                "complete authority fact set requires exact availability coverage",
            ));
        }
        let records_hash = hash_serializable("records", &(&grants, &availability_records))?;
        let source = AuthorityFactSourceBinding {
            kind: input.source_kind,
            snapshot_hash: input.source_snapshot_hash,
            observed_at: input.source_observed_at,
            completeness: input.completeness,
            query_set_hash: query_set.query_set_hash.clone(),
            grant_count: u64::try_from(grants.len()).unwrap_or(u64::MAX),
            availability_count: u64::try_from(availability_records.len()).unwrap_or(u64::MAX),
            records_hash,
        };
        let mut value = Self {
            version: CurrentAuthorityFactSetVersion::V1,
            execution_binding_hash: input.execution_binding.binding_hash().clone(),
            query_set,
            source,
            evaluated_at: input.evaluated_at,
            grants,
            availability_records,
            fact_set_hash: SpecContentHash::from_text("pending"),
        };
        value.fact_set_hash = value.compute_hash()?;
        value.validate()?;
        Ok(value)
    }

    /// Validates the aggregate commitment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error for inconsistent wire state.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.query_set.validate()?;
        if self
            .grants
            .windows(2)
            .any(|pair| pair[0].grant_id().as_str() >= pair[1].grant_id().as_str())
            || self
                .availability_records
                .windows(2)
                .any(|pair| availability_order(&pair[0], &pair[1]) != std::cmp::Ordering::Less)
        {
            return Err(fact_error(
                "record.order_invalid",
                "authority fact records must be unique and canonically ordered",
            ));
        }
        if self.source.query_set_hash != self.query_set.query_set_hash {
            return Err(fact_error(
                "source.query_mismatch",
                "authority source does not match the query set",
            ));
        }
        if self.source.grant_count != u64::try_from(self.grants.len()).unwrap_or(u64::MAX)
            || self.source.availability_count
                != u64::try_from(self.availability_records.len()).unwrap_or(u64::MAX)
            || self.source.records_hash
                != hash_serializable("records", &(&self.grants, &self.availability_records))?
            || self.source.observed_at > self.evaluated_at
        {
            return Err(fact_error(
                "source.inconsistent",
                "authority source commitment is inconsistent",
            ));
        }
        validate_record_scope(&self.query_set, &self.grants, &self.availability_records)?;
        if self.source.completeness == AuthorityFactCompletenessPosture::CompleteForExactQuery
            && self.availability_records.len() != self.query_set.queries.len()
        {
            return Err(fact_error(
                "availability.incomplete",
                "complete authority fact set requires exact availability coverage",
            ));
        }
        if self.fact_set_hash != self.compute_hash()? {
            return Err(fact_error(
                "hash.mismatch",
                "authority fact-set hash is invalid",
            ));
        }
        Ok(())
    }

    fn compute_hash(&self) -> Result<SpecContentHash, WorkflowOsError> {
        hash_serializable(
            "fact-set",
            &(
                self.version,
                &self.execution_binding_hash,
                &self.query_set,
                &self.source,
                self.evaluated_at,
                &self.grants,
                &self.availability_records,
            ),
        )
    }

    /// Returns the canonical query set.
    #[must_use]
    pub const fn query_set(&self) -> &CurrentAuthorityQuerySet {
        &self.query_set
    }

    /// Returns the source commitment.
    #[must_use]
    pub const fn source(&self) -> &AuthorityFactSourceBinding {
        &self.source
    }

    /// Returns supplied candidate grants.
    #[must_use]
    pub fn grants(&self) -> &[CapabilityGrant] {
        &self.grants
    }

    /// Returns supplied availability observations.
    #[must_use]
    pub fn availability_records(&self) -> &[CapabilityAvailabilityRecord] {
        &self.availability_records
    }

    /// Returns the aggregate commitment hash.
    #[must_use]
    pub const fn fact_set_hash(&self) -> &SpecContentHash {
        &self.fact_set_hash
    }
}

impl fmt::Debug for CurrentAuthorityFactSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentAuthorityFactSet")
            .field("version", &self.version)
            .field("execution_binding_hash", &"[REDACTED]")
            .field("query_set", &self.query_set)
            .field("source", &self.source)
            .field("evaluated_at", &"[REDACTED]")
            .field("grant_count", &self.grants.len())
            .field("availability_count", &self.availability_records.len())
            .field("fact_set_hash", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CurrentAuthorityFactSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            version: CurrentAuthorityFactSetVersion,
            execution_binding_hash: SpecContentHash,
            query_set: CurrentAuthorityQuerySet,
            source: AuthorityFactSourceBinding,
            evaluated_at: Timestamp,
            grants: Vec<CapabilityGrant>,
            availability_records: Vec<CapabilityAvailabilityRecord>,
            fact_set_hash: SpecContentHash,
        }
        let wire = Wire::deserialize(deserializer)?;
        let value = Self {
            version: wire.version,
            execution_binding_hash: wire.execution_binding_hash,
            query_set: wire.query_set,
            source: wire.source,
            evaluated_at: wire.evaluated_at,
            grants: wire.grants,
            availability_records: wire.availability_records,
            fact_set_hash: wire.fact_set_hash,
        };
        value
            .validate()
            .map_err(|_| serde::de::Error::custom("invalid current authority fact set"))?;
        Ok(value)
    }
}

fn validate_queries(queries: &[CurrentAuthorityQuery]) -> Result<(), WorkflowOsError> {
    if queries.is_empty() {
        return Err(fact_error(
            "query_set.empty",
            "authority query set cannot be empty",
        ));
    }
    for query in queries {
        query.validate()?;
    }
    if queries
        .windows(2)
        .any(|pair| pair[0].requirement_id.as_str() >= pair[1].requirement_id.as_str())
    {
        return Err(fact_error(
            "query_set.order_invalid",
            "authority queries must be unique and canonically ordered",
        ));
    }
    for (index, query) in queries.iter().enumerate() {
        if queries[index + 1..]
            .iter()
            .any(|other| query.capability == other.capability && query.resource == other.resource)
        {
            return Err(fact_error(
                "query_set.duplicate_target",
                "authority query set contains duplicate derived targets",
            ));
        }
    }
    Ok(())
}

fn validate_record_scope(
    query_set: &CurrentAuthorityQuerySet,
    grants: &[CapabilityGrant],
    availability_records: &[CapabilityAvailabilityRecord],
) -> Result<(), WorkflowOsError> {
    let matches_query = |capability: &CapabilityReference, resource: &CapabilityResourceScope| {
        query_set
            .queries
            .iter()
            .any(|query| query.capability == *capability && query.resource == *resource)
    };
    if grants
        .iter()
        .any(|grant| !matches_query(grant.capability(), grant.resource()))
        || availability_records
            .iter()
            .any(|record| !matches_query(record.capability(), record.resource()))
    {
        return Err(fact_error(
            "record.out_of_scope",
            "authority fact set contains a record outside its exact query set",
        ));
    }
    Ok(())
}

fn availability_order(
    left: &CapabilityAvailabilityRecord,
    right: &CapabilityAvailabilityRecord,
) -> std::cmp::Ordering {
    left.capability()
        .as_str()
        .cmp(right.capability().as_str())
        .then_with(|| left.resource().kind().cmp(&right.resource().kind()))
        .then_with(|| {
            left.resource()
                .reference()
                .cmp(right.resource().reference())
        })
}

fn same_availability_key(
    left: &CapabilityAvailabilityRecord,
    right: &CapabilityAvailabilityRecord,
) -> bool {
    left.capability() == right.capability() && left.resource() == right.resource()
}

fn hash_serializable(
    domain: &str,
    value: &impl Serialize,
) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        fact_error(
            "hash.serialization_failed",
            "authority fact-set hashing failed",
        )
    })?;
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, "domain", domain.as_bytes());
    hash_field(&mut hasher, "value", &bytes);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn hash_field(hasher: &mut Sha256, label: &str, value: &[u8]) {
    hasher.update(u64::try_from(label.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(label.as_bytes());
    hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(value);
}

fn fact_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("current_authority.fact_set.{suffix}"), message)
}

#[cfg(test)]
mod in_memory_source;

#[cfg(test)]
mod same_call_resolver;

#[cfg(test)]
mod tests {
    use super::hash_serializable;
    use crate::WorkflowOsError;

    #[test]
    fn hash_framing_separates_ambiguous_domain_and_value_pairs() -> Result<(), WorkflowOsError> {
        let left = hash_serializable("a", &"bc")?;
        let right = hash_serializable("ab", &"c")?;

        assert_ne!(left, right);
        Ok(())
    }
}
