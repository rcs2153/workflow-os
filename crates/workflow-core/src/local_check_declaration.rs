use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    LocalCheckAttestationAssurance, LocalCheckAttestationFreshnessPolicy, LocalCheckCommandId,
    LocalCheckNetworkPolicy, LocalCheckRequirementId, LocalCheckResultStatus,
    LocalCheckSideEffectClass, WorkflowOsError,
};

/// Whether a workflow-authored local-check obligation is required or optional.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckRequirementLevel {
    /// The future authoritative gate must satisfy this obligation.
    Required,
    /// The check may be absent, but an executed failure must never become success.
    Optional,
}

/// Validated workflow-authored local-check requirement vocabulary.
///
/// This type does not resolve command contracts, execute checks, construct
/// immutable-bundle records, or enforce an executor gate.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct LocalCheckRequirementDeclaration {
    id: LocalCheckRequirementId,
    command_id: LocalCheckCommandId,
    requirement_level: LocalCheckRequirementLevel,
    minimum_assurance: LocalCheckAttestationAssurance,
    accepted_statuses: Vec<LocalCheckResultStatus>,
    freshness: LocalCheckAttestationFreshnessPolicy,
    exact_immutable_run_binding_required: bool,
    truncation_allowed: bool,
    network_maximum: LocalCheckNetworkPolicy,
    side_effect_maximum: LocalCheckSideEffectClass,
}

/// Input fields for a validated workflow-authored local-check requirement.
pub struct LocalCheckRequirementDeclarationDefinition {
    /// Stable requirement identifier unique within its workflow step.
    pub id: LocalCheckRequirementId,
    /// Exact allowlisted command-contract identifier to resolve later.
    pub command_id: LocalCheckCommandId,
    /// Required or optional obligation posture.
    pub requirement_level: LocalCheckRequirementLevel,
    /// Minimum assurance required from a future verifier.
    pub minimum_assurance: LocalCheckAttestationAssurance,
    /// Accepted result statuses.
    pub accepted_statuses: Vec<LocalCheckResultStatus>,
    /// Observation freshness requirement.
    pub freshness: LocalCheckAttestationFreshnessPolicy,
    /// Whether the result must bind to the exact immutable run bundle.
    pub exact_immutable_run_binding_required: bool,
    /// Whether bounded truncated output may satisfy the future gate.
    pub truncation_allowed: bool,
    /// Maximum network posture allowed by this declaration.
    pub network_maximum: LocalCheckNetworkPolicy,
    /// Maximum `SideEffect` posture allowed by this declaration.
    pub side_effect_maximum: LocalCheckSideEffectClass,
}

impl LocalCheckRequirementDeclaration {
    /// Creates a validated schema-facing local-check declaration.
    ///
    /// # Errors
    ///
    /// Returns a stable non-leaking validation error when the declaration
    /// weakens independent assurance, accepts an unsafe outcome, relaxes exact
    /// run binding, or permits unsupported network or `SideEffect` posture.
    pub fn new(
        definition: LocalCheckRequirementDeclarationDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if definition.minimum_assurance
            != LocalCheckAttestationAssurance::KernelObservedLocalProcess
        {
            return Err(declaration_error(
                "assurance_unsupported",
                "local check declaration requires kernel-observed assurance",
            ));
        }
        if definition.accepted_statuses != [LocalCheckResultStatus::Passed] {
            return Err(declaration_error(
                "accepted_statuses_unsupported",
                "local check declaration accepts only a passed result in v0",
            ));
        }
        if !definition.exact_immutable_run_binding_required {
            return Err(declaration_error(
                "bundle_binding_required",
                "local check declaration requires exact immutable-run binding",
            ));
        }
        if definition.network_maximum != LocalCheckNetworkPolicy::Disabled {
            return Err(declaration_error(
                "network_unsupported",
                "local check declaration cannot permit network access in v0",
            ));
        }
        if definition.side_effect_maximum == LocalCheckSideEffectClass::Unclassified {
            return Err(declaration_error(
                "side_effect_unclassified",
                "local check declaration needs a classified SideEffect maximum",
            ));
        }

        Ok(Self {
            id: definition.id,
            command_id: definition.command_id,
            requirement_level: definition.requirement_level,
            minimum_assurance: definition.minimum_assurance,
            accepted_statuses: definition.accepted_statuses,
            freshness: definition.freshness,
            exact_immutable_run_binding_required: true,
            truncation_allowed: definition.truncation_allowed,
            network_maximum: definition.network_maximum,
            side_effect_maximum: definition.side_effect_maximum,
        })
    }

    /// Returns the authored requirement identifier.
    #[must_use]
    pub const fn id(&self) -> &LocalCheckRequirementId {
        &self.id
    }

    /// Returns the unresolved allowlisted command-contract identifier.
    #[must_use]
    pub const fn command_id(&self) -> &LocalCheckCommandId {
        &self.command_id
    }

    /// Returns the requirement level.
    #[must_use]
    pub const fn requirement_level(&self) -> LocalCheckRequirementLevel {
        self.requirement_level
    }

    /// Returns the minimum required assurance.
    #[must_use]
    pub const fn minimum_assurance(&self) -> LocalCheckAttestationAssurance {
        self.minimum_assurance
    }

    /// Returns accepted statuses.
    #[must_use]
    pub fn accepted_statuses(&self) -> &[LocalCheckResultStatus] {
        &self.accepted_statuses
    }

    /// Returns the freshness policy.
    #[must_use]
    pub const fn freshness(&self) -> LocalCheckAttestationFreshnessPolicy {
        self.freshness
    }

    /// Returns whether exact immutable-run binding is mandatory.
    #[must_use]
    pub const fn exact_immutable_run_binding_required(&self) -> bool {
        self.exact_immutable_run_binding_required
    }

    /// Returns whether bounded truncation is permitted.
    #[must_use]
    pub const fn truncation_allowed(&self) -> bool {
        self.truncation_allowed
    }

    /// Returns the maximum network posture.
    #[must_use]
    pub const fn network_maximum(&self) -> LocalCheckNetworkPolicy {
        self.network_maximum
    }

    /// Returns the maximum `SideEffect` posture.
    #[must_use]
    pub const fn side_effect_maximum(&self) -> LocalCheckSideEffectClass {
        self.side_effect_maximum
    }

    pub(crate) fn obligation_key(&self) -> &str {
        self.command_id.as_str()
    }
}

impl fmt::Debug for LocalCheckRequirementDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckRequirementDeclaration")
            .field("id", &"[REDACTED]")
            .field("command_id", &"[REDACTED]")
            .field("requirement_level", &self.requirement_level)
            .field("minimum_assurance", &self.minimum_assurance)
            .field("accepted_statuses", &self.accepted_statuses)
            .field("freshness", &self.freshness)
            .field(
                "exact_immutable_run_binding_required",
                &self.exact_immutable_run_binding_required,
            )
            .field("truncation_allowed", &self.truncation_allowed)
            .field("network_maximum", &self.network_maximum)
            .field("side_effect_maximum", &self.side_effect_maximum)
            .finish()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalCheckRequirementDeclarationWire {
    id: LocalCheckRequirementId,
    command_id: LocalCheckCommandId,
    requirement_level: LocalCheckRequirementLevel,
    minimum_assurance: LocalCheckAttestationAssurance,
    accepted_statuses: Vec<LocalCheckResultStatus>,
    freshness: LocalCheckAttestationFreshnessPolicy,
    exact_immutable_run_binding_required: bool,
    truncation_allowed: bool,
    network_maximum: LocalCheckNetworkPolicy,
    side_effect_maximum: LocalCheckSideEffectClass,
}

impl<'de> Deserialize<'de> for LocalCheckRequirementDeclaration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = LocalCheckRequirementDeclarationWire::deserialize(deserializer)?;
        Self::new(LocalCheckRequirementDeclarationDefinition {
            id: wire.id,
            command_id: wire.command_id,
            requirement_level: wire.requirement_level,
            minimum_assurance: wire.minimum_assurance,
            accepted_statuses: wire.accepted_statuses,
            freshness: wire.freshness,
            exact_immutable_run_binding_required: wire.exact_immutable_run_binding_required,
            truncation_allowed: wire.truncation_allowed,
            network_maximum: wire.network_maximum,
            side_effect_maximum: wire.side_effect_maximum,
        })
        .map_err(|_| serde::de::Error::custom("invalid local check requirement declaration"))
    }
}

fn declaration_error(suffix: &str, message: &'static str) -> WorkflowOsError {
    WorkflowOsError::validation(format!("local_check.declaration.{suffix}"), message)
}
