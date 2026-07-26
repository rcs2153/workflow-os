use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    GovernanceAssessmentBinding, GovernanceAssessmentCompleteness, GovernanceDisclosureRequirement,
    GovernanceExecutionDisposition, WorkflowId, WorkflowOsError, WorkflowRunId,
};

const APPROVAL_BINDING_ID_MAX_BYTES: usize = 128;

/// Version of the aggregate proportional-governance approval binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceApprovalBindingVersion {
    /// Initial authoritative aggregate-assessment approval binding.
    V1,
}

impl<'de> Deserialize<'de> for GovernanceApprovalBindingVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "v1" => Ok(Self::V1),
            _ => Err(serde::de::Error::custom(
                "governance approval binding version is invalid",
            )),
        }
    }
}

/// Identifier for one aggregate proportional-governance approval binding.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct GovernanceApprovalBindingId(String);

impl GovernanceApprovalBindingId {
    /// Creates a validated aggregate approval-binding identifier.
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

impl fmt::Display for GovernanceApprovalBindingId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl fmt::Debug for GovernanceApprovalBindingId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("GovernanceApprovalBindingId")
            .field(&"[REDACTED]")
            .finish()
    }
}

impl From<GovernanceApprovalBindingId> for String {
    fn from(value: GovernanceApprovalBindingId) -> Self {
        value.0
    }
}

impl TryFrom<String> for GovernanceApprovalBindingId {
    type Error = WorkflowOsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for GovernanceApprovalBindingId {
    type Err = WorkflowOsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Payload-free approval subject for one source-bound aggregate assessment.
///
/// This binding does not request, grant, deny, persist, or resume an approval.
/// It gives the existing approval lifecycle a truthful aggregate subject that
/// does not pretend the pre-execution governance gate belongs to one workflow
/// step or skill. The model is not standalone proof that an authoritative
/// check ran; an executor must construct it from the same-call authoritative
/// assessment and match the durable binding before using it.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct GovernanceApprovalBinding {
    binding_version: GovernanceApprovalBindingVersion,
    approval_binding_id: GovernanceApprovalBindingId,
    assessment: GovernanceAssessmentBinding,
}

impl GovernanceApprovalBinding {
    /// Binds an approval identifier to an exact source-bound aggregate assessment.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking error unless the assessment is complete,
    /// source-bound, visible, and requires approval. Structural validation does
    /// not grant runtime authority.
    pub fn new(
        approval_binding_id: GovernanceApprovalBindingId,
        assessment: GovernanceAssessmentBinding,
    ) -> Result<Self, WorkflowOsError> {
        let binding = Self {
            binding_version: GovernanceApprovalBindingVersion::V1,
            approval_binding_id,
            assessment,
        };
        binding.validate()?;
        Ok(binding)
    }

    /// Returns the binding model version.
    #[must_use]
    pub const fn binding_version(&self) -> GovernanceApprovalBindingVersion {
        self.binding_version
    }

    /// Returns the aggregate approval-binding identifier.
    #[must_use]
    pub const fn approval_binding_id(&self) -> &GovernanceApprovalBindingId {
        &self.approval_binding_id
    }

    /// Returns the exact authoritative aggregate assessment.
    #[must_use]
    pub const fn assessment(&self) -> &GovernanceAssessmentBinding {
        &self.assessment
    }

    /// Returns the workflow identity committed by the assessment.
    #[must_use]
    pub const fn workflow_id(&self) -> &WorkflowId {
        self.assessment.workflow_id()
    }

    /// Returns the run identity committed by the assessment.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        self.assessment.run_id()
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.binding_version != GovernanceApprovalBindingVersion::V1 {
            return Err(approval_binding_error(
                "version.invalid",
                "governance approval binding version is invalid",
            ));
        }
        validate_identifier(self.approval_binding_id.as_str())?;
        if self.assessment.completeness() != GovernanceAssessmentCompleteness::Complete {
            return Err(approval_binding_error(
                "assessment.incomplete",
                "governance approval binding assessment must be complete",
            ));
        }
        if self.assessment.source_binding().is_none() {
            return Err(approval_binding_error(
                "assessment.source_binding_required",
                "governance approval binding requires an authoritative assessment source",
            ));
        }
        if self.assessment.execution() != GovernanceExecutionDisposition::RequireApproval
            || self.assessment.disclosure() != GovernanceDisclosureRequirement::Visible
        {
            return Err(approval_binding_error(
                "assessment.route_invalid",
                "governance approval binding assessment route is invalid",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for GovernanceApprovalBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernanceApprovalBinding")
            .field("binding_version", &self.binding_version)
            .field("approval_binding_id", &"<redacted>")
            .field("assessment", &self.assessment)
            .finish()
    }
}

impl<'de> Deserialize<'de> for GovernanceApprovalBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            binding_version: GovernanceApprovalBindingVersion,
            approval_binding_id: GovernanceApprovalBindingId,
            assessment: GovernanceAssessmentBinding,
            #[serde(flatten)]
            extra: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.extra.is_empty() {
            return Err(serde::de::Error::custom(
                "governance approval binding contains an unknown field",
            ));
        }
        let binding = Self {
            binding_version: wire.binding_version,
            approval_binding_id: wire.approval_binding_id,
            assessment: wire.assessment,
        };
        binding.validate().map_err(serde::de::Error::custom)?;
        Ok(binding)
    }
}

fn validate_identifier(value: &str) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > APPROVAL_BINDING_ID_MAX_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(approval_binding_error(
            "identifier.invalid",
            "governance approval binding identifier is invalid",
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
        return Err(approval_binding_error(
            "identifier.secret_like",
            "governance approval binding identifier contains sensitive-looking text",
        ));
    }
    Ok(())
}

fn approval_binding_error(suffix: &'static str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(
        format!("governance.proportional_approval_binding.{suffix}"),
        message,
    )
}
