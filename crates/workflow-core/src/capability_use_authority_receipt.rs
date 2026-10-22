use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    current_authority_source::SuccessfulCapabilityUseProof, ActorId, CapabilityGrantId,
    CapabilityReference, CapabilityResourceKind, HarnessContractId, HarnessContractVersion,
    SpecContentHash, StepId, Timestamp, WorkReportSensitivity, WorkflowId, WorkflowOsError,
    WorkflowRunId,
};

const RECEIPT_ID_PREFIX: &str = "capability-use-authority-receipt/";

/// Version of the capability-use authority receipt model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityUseAuthorityReceiptVersion {
    /// Initial local unsigned, evidence-only receipt.
    V1,
}

impl<'de> Deserialize<'de> for CapabilityUseAuthorityReceiptVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "capability-use authority receipt version is invalid",
            )),
        }
    }
}

/// Exact operation recorded by a capability-use authority receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityUseAuthorityReceiptOperationKind {
    /// One GitHub repository-metadata read completed successfully.
    #[serde(rename = "github_repository_metadata_read_v1")]
    GitHubRepositoryMetadataReadV1,
}

impl<'de> Deserialize<'de> for CapabilityUseAuthorityReceiptOperationKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "github_repository_metadata_read_v1" => Ok(Self::GitHubRepositoryMetadataReadV1),
            _ => Err(serde::de::Error::custom(
                "capability-use authority receipt operation is invalid",
            )),
        }
    }
}

/// Trust posture of a deserialized capability-use receipt claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityUseAuthorityReceiptClaimVerificationPosture {
    /// The claim is structurally valid but its source provenance is not authenticated.
    UnverifiedSerializedClaim,
}

/// Deterministic identifier for one capability-use receipt.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct CapabilityUseAuthorityReceiptId(String);

impl CapabilityUseAuthorityReceiptId {
    /// Returns the stable receipt identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_commitment(commitment: &SpecContentHash) -> Self {
        Self(format!("{RECEIPT_ID_PREFIX}{}", commitment.as_str()))
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        let suffix = self.0.strip_prefix(RECEIPT_ID_PREFIX).ok_or_else(|| {
            receipt_error(
                "id.invalid",
                "capability-use authority receipt identifier is invalid",
            )
        })?;
        if suffix.len() != 64
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(receipt_error(
                "id.invalid",
                "capability-use authority receipt identifier is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for CapabilityUseAuthorityReceiptId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CapabilityUseAuthorityReceiptId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for CapabilityUseAuthorityReceiptId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(String::deserialize(deserializer)?);
        value.validate().map_err(|_| {
            serde::de::Error::custom("capability-use authority receipt identifier is invalid")
        })?;
        Ok(value)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityUseAuthorityReceiptRecord {
    version: CapabilityUseAuthorityReceiptVersion,
    receipt_id: CapabilityUseAuthorityReceiptId,
    execution_binding_hash: SpecContentHash,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    actor: ActorId,
    harness_contract_id: HarnessContractId,
    harness_contract_version: HarnessContractVersion,
    requested_sensitivity: WorkReportSensitivity,
    capability: CapabilityReference,
    resource_kind: CapabilityResourceKind,
    resource_scope_commitment: SpecContentHash,
    grant_id: CapabilityGrantId,
    source_snapshot_commitment: SpecContentHash,
    fact_set_commitment: SpecContentHash,
    assessment_commitment: SpecContentHash,
    operation_kind: CapabilityUseAuthorityReceiptOperationKind,
    operation_outcome_commitment: SpecContentHash,
    issued_at: Timestamp,
    validity: String,
    signature_posture: String,
    effect: String,
    redaction_posture: String,
    receipt_commitment: SpecContentHash,
}

#[derive(Serialize)]
struct CapabilityUseAuthorityReceiptCommitmentInput<'a> {
    domain: &'static str,
    version: CapabilityUseAuthorityReceiptVersion,
    execution_binding_hash: &'a SpecContentHash,
    workflow_id: &'a WorkflowId,
    run_id: &'a WorkflowRunId,
    step_id: &'a StepId,
    actor: &'a ActorId,
    harness_contract_id: &'a HarnessContractId,
    harness_contract_version: &'a HarnessContractVersion,
    requested_sensitivity: WorkReportSensitivity,
    capability: &'a CapabilityReference,
    resource_kind: CapabilityResourceKind,
    resource_scope_commitment: &'a SpecContentHash,
    grant_id: &'a CapabilityGrantId,
    source_snapshot_commitment: &'a SpecContentHash,
    fact_set_commitment: &'a SpecContentHash,
    assessment_commitment: &'a SpecContentHash,
    operation_kind: CapabilityUseAuthorityReceiptOperationKind,
    operation_outcome_commitment: &'a SpecContentHash,
    issued_at: Timestamp,
    validity: &'a str,
    signature_posture: &'a str,
    effect: &'a str,
    redaction_posture: &'a str,
}

impl CapabilityUseAuthorityReceiptRecord {
    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.receipt_id.validate()?;
        if self.requested_sensitivity == WorkReportSensitivity::Unknown {
            return Err(receipt_error(
                "sensitivity.unknown",
                "capability-use authority receipt sensitivity must be known",
            ));
        }
        if self.resource_kind == CapabilityResourceKind::Unknown
            || self.validity != "point_in_time_only"
            || self.signature_posture != "local_unsigned"
            || self.effect != "evidence_only_not_authorization"
            || self.redaction_posture != "reference_only"
        {
            return Err(receipt_error(
                "posture.invalid",
                "capability-use authority receipt posture is invalid",
            ));
        }
        let expected = compute_receipt_commitment(self)?;
        if self.receipt_commitment != expected
            || self.receipt_id != CapabilityUseAuthorityReceiptId::from_commitment(&expected)
        {
            return Err(receipt_error(
                "commitment.mismatch",
                "capability-use authority receipt commitment is invalid",
            ));
        }
        Ok(())
    }
}

/// Payload-free evidence that one capability-authorized operation completed.
///
/// This trusted type is serialize-only. Deserialization produces an
/// [`UnverifiedCapabilityUseAuthorityReceipt`] and never restores authority.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct CapabilityUseAuthorityReceipt {
    record: CapabilityUseAuthorityReceiptRecord,
}

impl CapabilityUseAuthorityReceipt {
    pub(crate) fn from_successful_use(
        proof: &SuccessfulCapabilityUseProof,
    ) -> Result<Self, WorkflowOsError> {
        let mut record = CapabilityUseAuthorityReceiptRecord {
            version: CapabilityUseAuthorityReceiptVersion::V1,
            receipt_id: CapabilityUseAuthorityReceiptId(String::new()),
            execution_binding_hash: proof.execution_binding_hash().clone(),
            workflow_id: proof.workflow_id().clone(),
            run_id: proof.run_id().clone(),
            step_id: proof.step_id().clone(),
            actor: proof.actor().clone(),
            harness_contract_id: proof.harness_contract_id().clone(),
            harness_contract_version: proof.harness_contract_version().clone(),
            requested_sensitivity: proof.requested_sensitivity(),
            capability: proof.capability().clone(),
            resource_kind: proof.resource_kind(),
            resource_scope_commitment: proof.resource_scope_commitment().clone(),
            grant_id: proof.grant_id().clone(),
            source_snapshot_commitment: proof.source_snapshot_commitment().clone(),
            fact_set_commitment: proof.fact_set_commitment().clone(),
            assessment_commitment: proof.assessment_commitment().clone(),
            operation_kind:
                CapabilityUseAuthorityReceiptOperationKind::GitHubRepositoryMetadataReadV1,
            operation_outcome_commitment: proof.operation_outcome_commitment().clone(),
            issued_at: proof.issued_at(),
            validity: "point_in_time_only".to_owned(),
            signature_posture: "local_unsigned".to_owned(),
            effect: "evidence_only_not_authorization".to_owned(),
            redaction_posture: "reference_only".to_owned(),
            receipt_commitment: SpecContentHash::from_text("pending"),
        };
        record.receipt_commitment = compute_receipt_commitment(&record)?;
        record.receipt_id =
            CapabilityUseAuthorityReceiptId::from_commitment(&record.receipt_commitment);
        record.validate()?;
        Ok(Self { record })
    }

    /// Validates deterministic field and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the receipt is inconsistent.
    pub fn validate(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the receipt version.
    #[must_use]
    pub const fn version(&self) -> CapabilityUseAuthorityReceiptVersion {
        self.record.version
    }

    /// Returns the deterministic receipt identifier.
    #[must_use]
    pub const fn receipt_id(&self) -> &CapabilityUseAuthorityReceiptId {
        &self.record.receipt_id
    }

    /// Returns the exact operation recorded by this receipt.
    #[must_use]
    pub const fn operation_kind(&self) -> CapabilityUseAuthorityReceiptOperationKind {
        self.record.operation_kind
    }

    /// Returns the payload-free operation outcome commitment.
    #[must_use]
    pub const fn operation_outcome_commitment(&self) -> &SpecContentHash {
        &self.record.operation_outcome_commitment
    }

    /// Returns the complete deterministic receipt commitment.
    #[must_use]
    pub const fn receipt_commitment(&self) -> &SpecContentHash {
        &self.record.receipt_commitment
    }
}

impl fmt::Debug for CapabilityUseAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_receipt("CapabilityUseAuthorityReceipt", &self.record, formatter)
    }
}

/// Structurally valid but unauthenticated serialized capability-use claim.
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct UnverifiedCapabilityUseAuthorityReceipt {
    record: CapabilityUseAuthorityReceiptRecord,
}

impl UnverifiedCapabilityUseAuthorityReceipt {
    /// Validates only deterministic field and commitment consistency.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the serialized claim is inconsistent.
    pub fn validate_claim(&self) -> Result<(), WorkflowOsError> {
        self.record.validate()
    }

    /// Returns the fixed unverified trust posture.
    #[must_use]
    pub const fn verification_posture(
        &self,
    ) -> CapabilityUseAuthorityReceiptClaimVerificationPosture {
        CapabilityUseAuthorityReceiptClaimVerificationPosture::UnverifiedSerializedClaim
    }

    /// Returns the claimed receipt identity.
    #[must_use]
    pub const fn receipt_id(&self) -> &CapabilityUseAuthorityReceiptId {
        &self.record.receipt_id
    }
}

impl fmt::Debug for UnverifiedCapabilityUseAuthorityReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_receipt(
            "UnverifiedCapabilityUseAuthorityReceipt",
            &self.record,
            formatter,
        )
    }
}

impl<'de> Deserialize<'de> for UnverifiedCapabilityUseAuthorityReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = CapabilityUseAuthorityReceiptRecord::deserialize(deserializer)?;
        record.validate().map_err(|_| {
            serde::de::Error::custom("invalid unverified capability-use authority receipt claim")
        })?;
        Ok(Self { record })
    }
}

fn debug_receipt(
    name: &str,
    record: &CapabilityUseAuthorityReceiptRecord,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("version", &record.version)
        .field("receipt_id", &"[REDACTED]")
        .field("execution_identity", &"[REDACTED]")
        .field("requested_sensitivity", &record.requested_sensitivity)
        .field("capability", &"[REDACTED]")
        .field("resource_kind", &record.resource_kind)
        .field("resource_scope_commitment", &"[REDACTED]")
        .field("grant_id", &"[REDACTED]")
        .field("source_commitments", &"[REDACTED]")
        .field("operation_kind", &record.operation_kind)
        .field("operation_outcome_commitment", &"[REDACTED]")
        .field("issued_at", &"[REDACTED]")
        .field("effect", &record.effect)
        .field("receipt_commitment", &"[REDACTED]")
        .finish_non_exhaustive()
}

fn compute_receipt_commitment(
    record: &CapabilityUseAuthorityReceiptRecord,
) -> Result<SpecContentHash, WorkflowOsError> {
    let bytes = serde_json::to_vec(&CapabilityUseAuthorityReceiptCommitmentInput {
        domain: "workflow-os/capability-use-authority-receipt/v1",
        version: record.version,
        execution_binding_hash: &record.execution_binding_hash,
        workflow_id: &record.workflow_id,
        run_id: &record.run_id,
        step_id: &record.step_id,
        actor: &record.actor,
        harness_contract_id: &record.harness_contract_id,
        harness_contract_version: &record.harness_contract_version,
        requested_sensitivity: record.requested_sensitivity,
        capability: &record.capability,
        resource_kind: record.resource_kind,
        resource_scope_commitment: &record.resource_scope_commitment,
        grant_id: &record.grant_id,
        source_snapshot_commitment: &record.source_snapshot_commitment,
        fact_set_commitment: &record.fact_set_commitment,
        assessment_commitment: &record.assessment_commitment,
        operation_kind: record.operation_kind,
        operation_outcome_commitment: &record.operation_outcome_commitment,
        issued_at: record.issued_at,
        validity: &record.validity,
        signature_posture: &record.signature_posture,
        effect: &record.effect,
        redaction_posture: &record.redaction_posture,
    })
    .map_err(|_| {
        receipt_error(
            "commitment.serialization_failed",
            "capability-use authority receipt commitment could not be computed",
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(b"workflow-os/capability-use-authority-receipt/v1");
    hasher.update(u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(bytes);
    Ok(SpecContentHash::from_bytes(hasher.finalize()))
}

fn receipt_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("capability_use_authority_receipt.{suffix}"),
        message,
    )
}
