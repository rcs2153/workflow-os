use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    CorrelationId, GovernanceAssessmentBinding, GovernanceAssessmentCompleteness,
    GovernanceDisclosureRequirement, GovernanceExecutionDisposition, Timestamp, WorkflowOsError,
};

const IDENTIFIER_MAX_BYTES: usize = 128;

/// Version of the visible-disclosure delivery contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureDeliveryVersion {
    /// Initial payload-free local delivery contract.
    V1,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureDeliveryVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "governance disclosure delivery version is invalid",
            )),
        }
    }
}

/// Identifier for one visible-disclosure delivery request and receipt.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct GovernanceDisclosureDeliveryId(String);

impl GovernanceDisclosureDeliveryId {
    /// Creates a validated delivery identifier.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the identifier is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
        let value = value.into();
        validate_identifier(&value)?;
        Ok(Self(value))
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for GovernanceDisclosureDeliveryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for GovernanceDisclosureDeliveryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("GovernanceDisclosureDeliveryId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<GovernanceDisclosureDeliveryId> for String {
    fn from(value: GovernanceDisclosureDeliveryId) -> Self {
        value.0
    }
}

impl TryFrom<String> for GovernanceDisclosureDeliveryId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for GovernanceDisclosureDeliveryId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Bounded kind of configured disclosure surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureSurfaceKind {
    /// Caller-injected local surface. No CLI, UI, or notification semantics are implied.
    InjectedLocal,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureSurfaceKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "injected_local" => Ok(Self::InjectedLocal),
            _ => Err(serde::de::Error::custom(
                "governance disclosure surface kind is invalid",
            )),
        }
    }
}

/// One explicitly selected, bounded disclosure surface.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceDisclosureSurface {
    kind: GovernanceDisclosureSurfaceKind,
    reference: String,
}

impl GovernanceDisclosureSurface {
    /// Creates a validated disclosure surface.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the reference is invalid.
    pub fn new(
        kind: GovernanceDisclosureSurfaceKind,
        reference: impl Into<String>,
    ) -> Result<Self, WorkflowOsError> {
        let surface = Self {
            kind,
            reference: reference.into(),
        };
        surface.validate()?;
        Ok(surface)
    }

    /// Returns the selected surface kind.
    #[must_use]
    pub const fn kind(&self) -> GovernanceDisclosureSurfaceKind {
        self.kind
    }

    /// Returns the bounded surface reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier(&self.reference)
    }
}

impl fmt::Debug for GovernanceDisclosureSurface {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceDisclosureSurface")
            .field("kind", &self.kind)
            .field("reference", &"[REDACTED]")
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceDisclosureSurface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            kind: GovernanceDisclosureSurfaceKind,
            reference: String,
            #[serde(flatten)]
            extra: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty() {
            return Err(serde::de::Error::custom(
                "governance disclosure surface contains an unknown field",
            ));
        }
        Self::new(wire.kind, wire.reference).map_err(serde::de::Error::custom)
    }
}

/// Sensitivity of payload-free visible-disclosure metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureSensitivity {
    /// Public metadata.
    Public,
    /// Internal metadata.
    Internal,
    /// Confidential metadata.
    Confidential,
    /// Restricted metadata.
    Restricted,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureSensitivity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "public" => Ok(Self::Public),
            "internal" => Ok(Self::Internal),
            "confidential" => Ok(Self::Confidential),
            "restricted" => Ok(Self::Restricted),
            _ => Err(serde::de::Error::custom(
                "governance disclosure sensitivity is invalid",
            )),
        }
    }
}

/// Redaction posture required by the payload-free delivery contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureRedactionPosture {
    /// Store references and bounded posture only.
    ReferenceOnly,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureRedactionPosture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "reference_only" => Ok(Self::ReferenceOnly),
            _ => Err(serde::de::Error::custom(
                "governance disclosure redaction posture is invalid",
            )),
        }
    }
}

/// Payload-free request to deliver one authoritative visible disclosure.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceDisclosureDeliveryRequest {
    version: GovernanceDisclosureDeliveryVersion,
    delivery_id: GovernanceDisclosureDeliveryId,
    assessment: GovernanceAssessmentBinding,
    surface: GovernanceDisclosureSurface,
    correlation_id: CorrelationId,
    requested_at: Timestamp,
    sensitivity: GovernanceDisclosureSensitivity,
    redaction: GovernanceDisclosureRedactionPosture,
}

impl GovernanceDisclosureDeliveryRequest {
    /// Creates a request for one complete source-bound `Proceed + Visible` assessment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the assessment or metadata is invalid.
    pub fn new(
        delivery_id: GovernanceDisclosureDeliveryId,
        assessment: GovernanceAssessmentBinding,
        surface: GovernanceDisclosureSurface,
        correlation_id: CorrelationId,
        requested_at: Timestamp,
        sensitivity: GovernanceDisclosureSensitivity,
    ) -> Result<Self, WorkflowOsError> {
        let request = Self {
            version: GovernanceDisclosureDeliveryVersion::V1,
            delivery_id,
            assessment,
            surface,
            correlation_id,
            requested_at,
            sensitivity,
            redaction: GovernanceDisclosureRedactionPosture::ReferenceOnly,
        };
        request.validate()?;
        Ok(request)
    }

    /// Returns the contract version.
    #[must_use]
    pub const fn version(&self) -> GovernanceDisclosureDeliveryVersion {
        self.version
    }

    /// Returns the delivery identity.
    #[must_use]
    pub const fn delivery_id(&self) -> &GovernanceDisclosureDeliveryId {
        &self.delivery_id
    }

    /// Returns the exact authoritative assessment being disclosed.
    #[must_use]
    pub const fn assessment(&self) -> &GovernanceAssessmentBinding {
        &self.assessment
    }

    /// Returns the explicitly selected delivery surface.
    #[must_use]
    pub const fn surface(&self) -> &GovernanceDisclosureSurface {
        &self.surface
    }

    /// Returns the correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    /// Returns the delivery-request timestamp.
    #[must_use]
    pub const fn requested_at(&self) -> Timestamp {
        self.requested_at
    }

    /// Returns the sensitivity classification.
    #[must_use]
    pub const fn sensitivity(&self) -> GovernanceDisclosureSensitivity {
        self.sensitivity
    }

    /// Returns the fixed payload-free redaction posture.
    #[must_use]
    pub const fn redaction(&self) -> GovernanceDisclosureRedactionPosture {
        self.redaction
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_identifier(self.delivery_id.as_str())?;
        self.surface.validate()?;
        validate_identifier(self.correlation_id.as_str())?;
        if self.assessment.completeness() != GovernanceAssessmentCompleteness::Complete {
            return Err(disclosure_error(
                "request.assessment_incomplete",
                "governance disclosure requires a complete assessment",
            ));
        }
        if self.assessment.execution() != GovernanceExecutionDisposition::Proceed
            || self.assessment.disclosure() != GovernanceDisclosureRequirement::Visible
        {
            return Err(disclosure_error(
                "request.route_invalid",
                "governance disclosure request route is invalid",
            ));
        }
        if self.assessment.source_binding().is_none() {
            return Err(disclosure_error(
                "request.source_binding_required",
                "governance disclosure requires an authoritative source binding",
            ));
        }
        if self.redaction != GovernanceDisclosureRedactionPosture::ReferenceOnly {
            return Err(disclosure_error(
                "request.redaction_invalid",
                "governance disclosure redaction posture is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for GovernanceDisclosureDeliveryRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceDisclosureDeliveryRequest")
            .field("version", &self.version)
            .field("delivery_id", &"[REDACTED]")
            .field("assessment", &self.assessment)
            .field("surface", &self.surface)
            .field("correlation_id", &"[REDACTED]")
            .field("requested_at", &self.requested_at)
            .field("sensitivity", &self.sensitivity)
            .field("redaction", &self.redaction)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceDisclosureDeliveryRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            version: GovernanceDisclosureDeliveryVersion,
            delivery_id: GovernanceDisclosureDeliveryId,
            assessment: GovernanceAssessmentBinding,
            surface: GovernanceDisclosureSurface,
            correlation_id: CorrelationId,
            requested_at: Timestamp,
            sensitivity: GovernanceDisclosureSensitivity,
            redaction: GovernanceDisclosureRedactionPosture,
            #[serde(flatten)]
            extra: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty() {
            return Err(serde::de::Error::custom(
                "governance disclosure delivery request contains an unknown field",
            ));
        }
        let request = Self {
            version: wire.version,
            delivery_id: wire.delivery_id,
            assessment: wire.assessment,
            surface: wire.surface,
            correlation_id: wire.correlation_id,
            requested_at: wire.requested_at,
            sensitivity: wire.sensitivity,
            redaction: wire.redaction,
        };
        if request.version != GovernanceDisclosureDeliveryVersion::V1 {
            return Err(serde::de::Error::custom(
                "governance disclosure delivery request version is invalid",
            ));
        }
        request.validate().map_err(serde::de::Error::custom)?;
        Ok(request)
    }
}

/// Narrow claim made by a valid delivery receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureDeliveryStatus {
    /// The configured surface accepted the bounded disclosure.
    SurfaceAccepted,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureDeliveryStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "surface_accepted" => Ok(Self::SurfaceAccepted),
            _ => Err(serde::de::Error::custom(
                "governance disclosure delivery status is invalid",
            )),
        }
    }
}

/// Explicitly absent human-observation claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureHumanObservation {
    /// The receipt does not claim that a human observed the disclosure.
    NotClaimed,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureHumanObservation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_claimed" => Ok(Self::NotClaimed),
            _ => Err(serde::de::Error::custom(
                "governance disclosure human-observation posture is invalid",
            )),
        }
    }
}

/// Explicitly absent acknowledgement claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDisclosureAcknowledgement {
    /// The receipt does not claim that any operator acknowledged the disclosure.
    NotClaimed,
}

impl<'de> Deserialize<'de> for GovernanceDisclosureAcknowledgement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_claimed" => Ok(Self::NotClaimed),
            _ => Err(serde::de::Error::custom(
                "governance disclosure acknowledgement posture is invalid",
            )),
        }
    }
}

/// Payload-free record that one configured surface accepted a disclosure request.
///
/// This receipt does not prove independent delivery, human observation,
/// understanding, or acknowledgement. Future executor integration must create
/// it only from the result of an explicitly injected delivery surface.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceDisclosureDeliveryReceipt {
    version: GovernanceDisclosureDeliveryVersion,
    request: GovernanceDisclosureDeliveryRequest,
    status: GovernanceDisclosureDeliveryStatus,
    accepted_at: Timestamp,
    human_observation: GovernanceDisclosureHumanObservation,
    acknowledgement: GovernanceDisclosureAcknowledgement,
}

impl GovernanceDisclosureDeliveryReceipt {
    /// Records the narrow claim that the configured surface accepted the request.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when timestamps or request posture are invalid.
    pub fn surface_accepted(
        request: GovernanceDisclosureDeliveryRequest,
        accepted_at: Timestamp,
    ) -> Result<Self, WorkflowOsError> {
        let receipt = Self {
            version: GovernanceDisclosureDeliveryVersion::V1,
            request,
            status: GovernanceDisclosureDeliveryStatus::SurfaceAccepted,
            accepted_at,
            human_observation: GovernanceDisclosureHumanObservation::NotClaimed,
            acknowledgement: GovernanceDisclosureAcknowledgement::NotClaimed,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    /// Returns the receipt version.
    #[must_use]
    pub const fn version(&self) -> GovernanceDisclosureDeliveryVersion {
        self.version
    }

    /// Returns the exact request accepted by the configured surface.
    #[must_use]
    pub const fn request(&self) -> &GovernanceDisclosureDeliveryRequest {
        &self.request
    }

    /// Returns the narrow delivery status.
    #[must_use]
    pub const fn status(&self) -> GovernanceDisclosureDeliveryStatus {
        self.status
    }

    /// Returns when the configured surface reported acceptance.
    #[must_use]
    pub const fn accepted_at(&self) -> Timestamp {
        self.accepted_at
    }

    /// Returns the explicit human-observation non-claim.
    #[must_use]
    pub const fn human_observation(&self) -> GovernanceDisclosureHumanObservation {
        self.human_observation
    }

    /// Returns the explicit acknowledgement non-claim.
    #[must_use]
    pub const fn acknowledgement(&self) -> GovernanceDisclosureAcknowledgement {
        self.acknowledgement
    }

    /// Validates that this receipt belongs to the exact expected request.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error when the request does not match.
    pub fn validate_for_request(
        &self,
        expected: &GovernanceDisclosureDeliveryRequest,
    ) -> Result<(), WorkflowOsError> {
        self.validate()?;
        if &self.request != expected {
            return Err(disclosure_error(
                "receipt.request_mismatch",
                "governance disclosure receipt request does not match",
            ));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        self.request.validate()?;
        if self.version != GovernanceDisclosureDeliveryVersion::V1
            || self.status != GovernanceDisclosureDeliveryStatus::SurfaceAccepted
            || self.human_observation != GovernanceDisclosureHumanObservation::NotClaimed
            || self.acknowledgement != GovernanceDisclosureAcknowledgement::NotClaimed
        {
            return Err(disclosure_error(
                "receipt.claim_invalid",
                "governance disclosure receipt claim is invalid",
            ));
        }
        if self.accepted_at < self.request.requested_at {
            return Err(disclosure_error(
                "receipt.timestamp_invalid",
                "governance disclosure receipt timestamp is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for GovernanceDisclosureDeliveryReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceDisclosureDeliveryReceipt")
            .field("version", &self.version)
            .field("request", &self.request)
            .field("status", &self.status)
            .field("accepted_at", &self.accepted_at)
            .field("human_observation", &self.human_observation)
            .field("acknowledgement", &self.acknowledgement)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceDisclosureDeliveryReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            version: GovernanceDisclosureDeliveryVersion,
            request: GovernanceDisclosureDeliveryRequest,
            status: GovernanceDisclosureDeliveryStatus,
            accepted_at: Timestamp,
            human_observation: GovernanceDisclosureHumanObservation,
            acknowledgement: GovernanceDisclosureAcknowledgement,
            #[serde(flatten)]
            extra: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty() {
            return Err(serde::de::Error::custom(
                "governance disclosure delivery receipt contains an unknown field",
            ));
        }
        let receipt = Self {
            version: wire.version,
            request: wire.request,
            status: wire.status,
            accepted_at: wire.accepted_at,
            human_observation: wire.human_observation,
            acknowledgement: wire.acknowledgement,
        };
        receipt.validate().map_err(serde::de::Error::custom)?;
        Ok(receipt)
    }
}

fn validate_identifier(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > IDENTIFIER_MAX_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(disclosure_error(
            "identifier.invalid",
            "governance disclosure identifier is invalid",
        ));
    }
    let lowercase = value.to_ascii_lowercase();
    if [
        "authorization",
        "bearer",
        "credential",
        "password",
        "private_key",
        "private-key",
        "api_token",
        "api-token",
        "secret",
        "token",
    ]
    .iter()
    .any(|marker| lowercase.contains(marker))
    {
        return Err(disclosure_error(
            "identifier.secret_like",
            "governance disclosure identifier contains sensitive-looking text",
        ));
    }
    Ok(())
}

fn disclosure_error(suffix: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("governance.disclosure_delivery.{suffix}"), message)
}
