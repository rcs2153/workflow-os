use std::fmt;

use serde::{Deserialize, Serialize};

/// Local governance strictness profiles.
///
/// These profiles describe how Workflow OS discloses governance posture. They
/// do not grant authority, enable hosted admin controls, or change executor
/// approval behavior by themselves.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceStrictnessProfile {
    /// Fast local mode: record, disclose, and report without approval pauses by default.
    ObserveAndReport,
    /// Agent may provide bounded evidence/details for selected gates.
    AgentAssistedGated,
    /// Human approvals are expected at configured gates.
    HumanApprovalGated,
    /// Future organization-managed stewardship posture.
    StrictEnterprise,
}

impl GovernanceStrictnessProfile {
    /// Stable label used in CLI and JSON output.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ObserveAndReport => "observe_and_report",
            Self::AgentAssistedGated => "agent_assisted_gated",
            Self::HumanApprovalGated => "human_approval_gated",
            Self::StrictEnterprise => "strict_enterprise",
        }
    }

    /// Human-facing summary of the profile's intended prompt/block posture.
    #[must_use]
    pub const fn prompt_behavior(self) -> &'static str {
        match self {
            Self::ObserveAndReport => "record_and_disclose_without_default_approval_pauses",
            Self::AgentAssistedGated => "block_when_required_agent_evidence_is_missing",
            Self::HumanApprovalGated => "pause_at_configured_human_approval_gates",
            Self::StrictEnterprise => "future_steward_defined_gate_enforcement",
        }
    }

    /// Whether this profile represents current local-preview behavior.
    #[must_use]
    pub const fn is_currently_enforced(self) -> bool {
        match self {
            Self::ObserveAndReport => false,
            Self::AgentAssistedGated | Self::HumanApprovalGated | Self::StrictEnterprise => false,
        }
    }
}

impl fmt::Display for GovernanceStrictnessProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

/// Bounded disclosure for the active governance profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernanceProfileDisclosure {
    profile: GovernanceStrictnessProfile,
    posture: GovernanceProfilePosture,
}

impl GovernanceProfileDisclosure {
    /// Returns the current default local-preview profile disclosure.
    #[must_use]
    pub const fn current_local_default() -> Self {
        Self {
            profile: GovernanceStrictnessProfile::ObserveAndReport,
            posture: GovernanceProfilePosture::DisclosedNotEnforced,
        }
    }

    /// Returns the disclosed profile.
    #[must_use]
    pub const fn profile(self) -> GovernanceStrictnessProfile {
        self.profile
    }

    /// Returns the disclosed posture.
    #[must_use]
    pub const fn posture(self) -> GovernanceProfilePosture {
        self.posture
    }

    /// Stable profile label.
    #[must_use]
    pub const fn profile_label(self) -> &'static str {
        self.profile.label()
    }

    /// Stable posture label.
    #[must_use]
    pub const fn posture_label(self) -> &'static str {
        self.posture.label()
    }
}

/// Current enforcement posture for a governance profile disclosure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceProfilePosture {
    /// The profile is disclosed for operator clarity but does not enforce
    /// profile-specific gate behavior.
    DisclosedNotEnforced,
    /// Future posture for steward/admin-controlled profile enforcement.
    FutureStewardManaged,
}

impl GovernanceProfilePosture {
    /// Stable label used in CLI and JSON output.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::DisclosedNotEnforced => "disclosed_not_enforced",
            Self::FutureStewardManaged => "future_steward_managed",
        }
    }
}

impl fmt::Display for GovernanceProfilePosture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GovernanceProfileDisclosure, GovernanceProfilePosture, GovernanceStrictnessProfile,
    };

    #[test]
    fn local_default_discloses_observe_and_report_without_enforcement() {
        let disclosure = GovernanceProfileDisclosure::current_local_default();

        assert_eq!(
            disclosure.profile(),
            GovernanceStrictnessProfile::ObserveAndReport
        );
        assert_eq!(
            disclosure.posture(),
            GovernanceProfilePosture::DisclosedNotEnforced
        );
        assert_eq!(disclosure.profile_label(), "observe_and_report");
        assert_eq!(disclosure.posture_label(), "disclosed_not_enforced");
        assert!(!disclosure.profile().is_currently_enforced());
    }

    #[test]
    fn planned_profile_labels_are_stable() {
        assert_eq!(
            GovernanceStrictnessProfile::AgentAssistedGated.label(),
            "agent_assisted_gated"
        );
        assert_eq!(
            GovernanceStrictnessProfile::HumanApprovalGated.label(),
            "human_approval_gated"
        );
        assert_eq!(
            GovernanceStrictnessProfile::StrictEnterprise.label(),
            "strict_enterprise"
        );
    }
}
