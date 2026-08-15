use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    ActorId, ApprovalReferenceId, EventId, EventSequenceNumber, ImmutableRunBundleBinding,
    SpecContentHash, StepId, Timestamp, WorkReportSensitivity, WorkflowId, WorkflowOsError,
    WorkflowRunId,
};

const IDENTIFIER_MAX_BYTES: usize = 128;
const REFERENCE_MAX_BYTES: usize = 256;
const CONDITION_MAX_COUNT: usize = 16;
const EXECUTION_WINDOW_MAX_ATTEMPTS: u32 = 64;

macro_rules! continuity_id {
    ($name:ident, $label:literal, $code:literal, $max:expr) => {
        #[doc = concat!("Validated ", $label, ".")]
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a validated ", $label, ".")]
            ///
            /// # Errors
            ///
            /// Returns a bounded validation error for invalid or sensitive-looking text.
            pub fn new(value: impl Into<String>) -> Result<Self, WorkflowOsError> {
                let value = value.into();
                validate_identifier($label, &value, $code, $max)?;
                Ok(Self(value))
            }

            #[doc = concat!("Returns the ", $label, " text.")]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
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

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)
                    .map_err(|_| serde::de::Error::custom(concat!($label, " is invalid")))?;
                Self::new(value)
                    .map_err(|_| serde::de::Error::custom(concat!($label, " is invalid")))
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

        impl FromStr for $name {
            type Err = WorkflowOsError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

macro_rules! safe_string_enum {
    ($name:ident { $($wire:literal => $variant:ident),+ $(,)? }, $error:literal) => {
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                match String::deserialize(deserializer)
                    .map_err(|_| serde::de::Error::custom($error))?
                    .as_str()
                {
                    $($wire => Ok(Self::$variant),)+
                    _ => Err(serde::de::Error::custom($error)),
                }
            }
        }
    };
}

macro_rules! impl_safe_model_deserialize {
    ($model:ty, $wire:ty, $convert:expr) => {
        impl<'de> Deserialize<'de> for $model {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let wire: $wire = deserialize_wire_safely(deserializer)?;
                ($convert)(wire).map_err(|_| {
                    serde::de::Error::custom("authorized execution continuity value is invalid")
                })
            }
        }
    };
}

continuity_id!(
    AuthorizedExecutionWindowId,
    "authorized execution window id",
    "window_id.invalid",
    IDENTIFIER_MAX_BYTES
);
continuity_id!(
    AuthorizedExecutionAttemptId,
    "authorized execution attempt id",
    "attempt_id.invalid",
    IDENTIFIER_MAX_BYTES
);
continuity_id!(
    AuthorizedExecutionWaitConditionId,
    "authorized execution wait condition id",
    "wait.condition_id.invalid",
    IDENTIFIER_MAX_BYTES
);
continuity_id!(
    AuthorizedExecutionActionReference,
    "authorized execution action reference",
    "action_reference.invalid",
    REFERENCE_MAX_BYTES
);
continuity_id!(
    AuthorizedExecutionResourceReference,
    "authorized execution resource reference",
    "resource_reference.invalid",
    REFERENCE_MAX_BYTES
);
continuity_id!(
    AuthorizedExecutionAuthoritySourceReference,
    "authorized execution authority source reference",
    "authority_source_reference.invalid",
    REFERENCE_MAX_BYTES
);

/// Version of the model-only authorized-execution continuity vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionContinuityModelVersion {
    /// Initial model-only continuity contract.
    V1,
}
safe_string_enum!(AuthorizedExecutionContinuityModelVersion { "v1" => V1 }, "authorized execution continuity model version is invalid");

/// Explicit authority posture serialized on every model-only continuity projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionAuthorityPosture {
    /// Orientation only; the value cannot authorize an operation.
    NonAuthoritative,
}
safe_string_enum!(AuthorizedExecutionAuthorityPosture { "non_authoritative" => NonAuthoritative }, "authorized execution authority posture is invalid");

/// Current readiness of an approval gate, separate from its decision or execution authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionGateReadiness {
    /// One or more current prerequisites prevents presentation.
    PendingPrerequisites,
    /// The gate may be presented for a decision, but is not yet decided.
    ReadyForDecision,
}
safe_string_enum!(AuthorizedExecutionGateReadiness {
    "pending_prerequisites" => PendingPrerequisites,
    "ready_for_decision" => ReadyForDecision
}, "authorized execution gate readiness is invalid");

/// Typed prerequisite preventing a gate from being presented for decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionGateBlocker {
    /// Required evidence has not been accepted.
    EvidenceRequired,
    /// A required check has not been accepted.
    CheckRequired,
    /// Current policy denies presentation or decision.
    PolicyDenied,
    /// Current authority cannot be resolved.
    AuthorityUnavailable,
    /// Required approval-presentation proof is absent or stale.
    ApprovalPresentationRequired,
    /// Required requester/approver separation is unavailable.
    SeparationOfDutyRequired,
    /// The assessment cursor no longer matches durable state.
    StaleCursor,
    /// Current facts do not produce one deterministic result.
    AmbiguousFacts,
}
safe_string_enum!(AuthorizedExecutionGateBlocker {
    "evidence_required" => EvidenceRequired,
    "check_required" => CheckRequired,
    "policy_denied" => PolicyDenied,
    "authority_unavailable" => AuthorityUnavailable,
    "approval_presentation_required" => ApprovalPresentationRequired,
    "separation_of_duty_required" => SeparationOfDutyRequired,
    "stale_cursor" => StaleCursor,
    "ambiguous_facts" => AmbiguousFacts
}, "authorized execution gate blocker is invalid");

/// Typed dependency responsible for a genuine execution wait.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionWaitConditionKind {
    /// Wait for one human or delegated decision.
    HumanDecision,
    /// Wait for one evidence obligation.
    EvidenceRequired,
    /// Wait for one check obligation.
    CheckRequired,
    /// Wait for one external event.
    ExternalEvent,
    /// Wait for a scoped capability to become available.
    CapabilityUnavailable,
    /// Wait for a declared deadline.
    TimeWindow,
    /// Wait for current authority facts to change.
    AuthorityRefresh,
    /// Wait for one identified conflict to be resolved.
    ConflictResolution,
}
safe_string_enum!(AuthorizedExecutionWaitConditionKind {
    "human_decision" => HumanDecision,
    "evidence_required" => EvidenceRequired,
    "check_required" => CheckRequired,
    "external_event" => ExternalEvent,
    "capability_unavailable" => CapabilityUnavailable,
    "time_window" => TimeWindow,
    "authority_refresh" => AuthorityRefresh,
    "conflict_resolution" => ConflictResolution
}, "authorized execution wait condition kind is invalid");

/// Deterministic event class that permits a wait to be reassessed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionWakeTriggerKind {
    /// Reassess after an approval decision event.
    ApprovalDecisionRecorded,
    /// Reassess after required evidence is accepted.
    EvidenceAccepted,
    /// Reassess after a required check is accepted.
    CheckAccepted,
    /// Reassess after an identified external event arrives.
    ExternalEventRecorded,
    /// Reassess after capability availability changes.
    CapabilityAvailabilityChanged,
    /// Reassess when the declared deadline is reached.
    DeadlineReached,
    /// Reassess after the authority source advances.
    AuthoritySourceChanged,
    /// Reassess after the identified conflict is resolved.
    ConflictResolved,
}
safe_string_enum!(AuthorizedExecutionWakeTriggerKind {
    "approval_decision_recorded" => ApprovalDecisionRecorded,
    "evidence_accepted" => EvidenceAccepted,
    "check_accepted" => CheckAccepted,
    "external_event_recorded" => ExternalEventRecorded,
    "capability_availability_changed" => CapabilityAvailabilityChanged,
    "deadline_reached" => DeadlineReached,
    "authority_source_changed" => AuthoritySourceChanged,
    "conflict_resolved" => ConflictResolved
}, "authorized execution wake trigger kind is invalid");

/// Current non-authoritative wake posture for one wait condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionWaitStatus {
    /// The dependency remains unsatisfied.
    Waiting,
    /// The dependency was satisfied and should not remain active.
    Satisfied,
    /// The wait condition expired.
    Expired,
    /// Newer durable state superseded the condition.
    Superseded,
}
safe_string_enum!(AuthorizedExecutionWaitStatus {
    "waiting" => Waiting,
    "satisfied" => Satisfied,
    "expired" => Expired,
    "superseded" => Superseded
}, "authorized execution wait status is invalid");

/// Public construction fields for one exact typed wait condition.
pub struct AuthorizedExecutionWaitConditionDefinition {
    /// Stable condition identity.
    pub condition_id: AuthorizedExecutionWaitConditionId,
    /// Monotonic version of this exact condition.
    pub condition_version: u32,
    /// Dependency class.
    pub kind: AuthorizedExecutionWaitConditionKind,
    /// Bound workflow identity.
    pub workflow_id: WorkflowId,
    /// Bound run identity.
    pub run_id: WorkflowRunId,
    /// Bound execution-window identity.
    pub window_id: AuthorizedExecutionWindowId,
    /// Exact action waiting to become lawful.
    pub action_reference: AuthorizedExecutionActionReference,
    /// Bound workflow step.
    pub step_id: StepId,
    /// Bound execution attempt.
    pub attempt_id: AuthorizedExecutionAttemptId,
    /// Expected durable event sequence.
    pub expected_sequence_number: EventSequenceNumber,
    /// Expected durable event identity.
    pub expected_event_id: EventId,
    /// Stable dependency reference without its payload.
    pub required_reference: AuthorizedExecutionResourceReference,
    /// Condition creation time.
    pub created_at: Timestamp,
    /// Optional condition deadline.
    pub deadline: Option<Timestamp>,
    /// Event class that permits reassessment.
    pub wake_trigger: AuthorizedExecutionWakeTriggerKind,
    /// Current non-authoritative wake posture.
    pub status: AuthorizedExecutionWaitStatus,
}

/// Bounded identity-, cursor-, action-, and wake-bound genuine wait projection.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct AuthorizedExecutionWaitCondition {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    condition_id: AuthorizedExecutionWaitConditionId,
    condition_version: u32,
    kind: AuthorizedExecutionWaitConditionKind,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    window_id: AuthorizedExecutionWindowId,
    action_reference: AuthorizedExecutionActionReference,
    step_id: StepId,
    attempt_id: AuthorizedExecutionAttemptId,
    expected_sequence_number: EventSequenceNumber,
    expected_event_id: EventId,
    required_reference: AuthorizedExecutionResourceReference,
    created_at: Timestamp,
    deadline: Option<Timestamp>,
    wake_trigger: AuthorizedExecutionWakeTriggerKind,
    status: AuthorizedExecutionWaitStatus,
}

impl AuthorizedExecutionWaitCondition {
    /// Creates a validated typed wait condition.
    ///
    /// # Errors
    ///
    /// Returns a stable error for invalid version, deadline, or wake semantics.
    pub fn new(
        definition: AuthorizedExecutionWaitConditionDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            model_version: AuthorizedExecutionContinuityModelVersion::V1,
            authority_posture: AuthorizedExecutionAuthorityPosture::NonAuthoritative,
            condition_id: definition.condition_id,
            condition_version: definition.condition_version,
            kind: definition.kind,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            window_id: definition.window_id,
            action_reference: definition.action_reference,
            step_id: definition.step_id,
            attempt_id: definition.attempt_id,
            expected_sequence_number: definition.expected_sequence_number,
            expected_event_id: definition.expected_event_id,
            required_reference: definition.required_reference,
            created_at: definition.created_at,
            deadline: definition.deadline,
            wake_trigger: definition.wake_trigger,
            status: definition.status,
        };
        value.validate()?;
        Ok(value)
    }

    /// Returns the stable condition identity.
    #[must_use]
    pub const fn condition_id(&self) -> &AuthorizedExecutionWaitConditionId {
        &self.condition_id
    }
    /// Returns the typed dependency class.
    #[must_use]
    pub const fn kind(&self) -> AuthorizedExecutionWaitConditionKind {
        self.kind
    }
    /// Returns the current non-authoritative condition status.
    #[must_use]
    pub const fn status(&self) -> AuthorizedExecutionWaitStatus {
        self.status
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.authority_posture != AuthorizedExecutionAuthorityPosture::NonAuthoritative {
            return Err(continuity_error(
                "wait.authority_posture_invalid",
                "wait condition must be explicitly non-authoritative",
            ));
        }
        if self.condition_version == 0 {
            return Err(continuity_error(
                "wait.condition_version_zero",
                "wait condition version must be positive",
            ));
        }
        if self
            .deadline
            .is_some_and(|deadline| deadline <= self.created_at)
        {
            return Err(continuity_error(
                "wait.deadline_invalid",
                "wait condition deadline must follow creation",
            ));
        }
        let expected_trigger = match self.kind {
            AuthorizedExecutionWaitConditionKind::HumanDecision => {
                AuthorizedExecutionWakeTriggerKind::ApprovalDecisionRecorded
            }
            AuthorizedExecutionWaitConditionKind::EvidenceRequired => {
                AuthorizedExecutionWakeTriggerKind::EvidenceAccepted
            }
            AuthorizedExecutionWaitConditionKind::CheckRequired => {
                AuthorizedExecutionWakeTriggerKind::CheckAccepted
            }
            AuthorizedExecutionWaitConditionKind::ExternalEvent => {
                AuthorizedExecutionWakeTriggerKind::ExternalEventRecorded
            }
            AuthorizedExecutionWaitConditionKind::CapabilityUnavailable => {
                AuthorizedExecutionWakeTriggerKind::CapabilityAvailabilityChanged
            }
            AuthorizedExecutionWaitConditionKind::TimeWindow => {
                AuthorizedExecutionWakeTriggerKind::DeadlineReached
            }
            AuthorizedExecutionWaitConditionKind::AuthorityRefresh => {
                AuthorizedExecutionWakeTriggerKind::AuthoritySourceChanged
            }
            AuthorizedExecutionWaitConditionKind::ConflictResolution => {
                AuthorizedExecutionWakeTriggerKind::ConflictResolved
            }
        };
        if self.wake_trigger != expected_trigger {
            return Err(continuity_error(
                "wait.wake_trigger_mismatch",
                "wait condition wake trigger does not match its kind",
            ));
        }
        if self.kind == AuthorizedExecutionWaitConditionKind::TimeWindow && self.deadline.is_none()
        {
            return Err(continuity_error(
                "wait.deadline_required",
                "time-window wait requires a deadline",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for AuthorizedExecutionWaitCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizedExecutionWaitCondition")
            .field("model_version", &self.model_version)
            .field("authority_posture", &self.authority_posture)
            .field("kind", &self.kind)
            .field("status", &self.status)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl_safe_model_deserialize!(
    AuthorizedExecutionWaitCondition,
    AuthorizedExecutionWaitConditionWire,
    |wire: AuthorizedExecutionWaitConditionWire| {
        validate_wire_header(wire.model_version, wire.authority_posture)?;
        AuthorizedExecutionWaitCondition::new(AuthorizedExecutionWaitConditionDefinition {
            condition_id: wire.condition_id,
            condition_version: wire.condition_version,
            kind: wire.kind,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            window_id: wire.window_id,
            action_reference: wire.action_reference,
            step_id: wire.step_id,
            attempt_id: wire.attempt_id,
            expected_sequence_number: wire.expected_sequence_number,
            expected_event_id: wire.expected_event_id,
            required_reference: wire.required_reference,
            created_at: wire.created_at,
            deadline: wire.deadline,
            wake_trigger: wire.wake_trigger,
            status: wire.status,
        })
    }
);

/// Public construction fields for a non-authoritative gate readiness assessment.
pub struct AuthorizedExecutionGateAssessmentDefinition {
    /// Workflow identity.
    pub workflow_id: WorkflowId,
    /// Run identity.
    pub run_id: WorkflowRunId,
    /// Gate-bearing step identity.
    pub step_id: StepId,
    /// Exact approval request being assessed.
    pub approval_reference: ApprovalReferenceId,
    /// Exact action requiring a decision.
    pub action_reference: AuthorizedExecutionActionReference,
    /// Immutable run material bound to the request.
    pub immutable_run_bundle: ImmutableRunBundleBinding,
    /// Last durable sequence used by the assessment.
    pub last_sequence_number: EventSequenceNumber,
    /// Last durable event used by the assessment.
    pub last_event_id: EventId,
    /// Assessment time.
    pub assessed_at: Timestamp,
    /// Current gate-presentability posture.
    pub readiness: AuthorizedExecutionGateReadiness,
    /// Typed unsatisfied prerequisites.
    pub blockers: Vec<AuthorizedExecutionGateBlocker>,
    /// Payload-free commitment to all assessed inputs.
    pub assessment_commitment: SpecContentHash,
}

/// Bounded gate-presentability assessment; never an approval or execution permit.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AuthorizedExecutionGateAssessment {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    approval_reference: ApprovalReferenceId,
    action_reference: AuthorizedExecutionActionReference,
    immutable_run_bundle: ImmutableRunBundleBinding,
    last_sequence_number: EventSequenceNumber,
    last_event_id: EventId,
    assessed_at: Timestamp,
    readiness: AuthorizedExecutionGateReadiness,
    blockers: Vec<AuthorizedExecutionGateBlocker>,
    assessment_commitment: SpecContentHash,
}

impl AuthorizedExecutionGateAssessment {
    /// Creates a validated non-authoritative gate readiness assessment.
    ///
    /// # Errors
    ///
    /// Returns a stable error when readiness contradicts its blockers.
    pub fn new(
        definition: AuthorizedExecutionGateAssessmentDefinition,
    ) -> Result<Self, WorkflowOsError> {
        let value = Self {
            model_version: AuthorizedExecutionContinuityModelVersion::V1,
            authority_posture: AuthorizedExecutionAuthorityPosture::NonAuthoritative,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            step_id: definition.step_id,
            approval_reference: definition.approval_reference,
            action_reference: definition.action_reference,
            immutable_run_bundle: definition.immutable_run_bundle,
            last_sequence_number: definition.last_sequence_number,
            last_event_id: definition.last_event_id,
            assessed_at: definition.assessed_at,
            readiness: definition.readiness,
            blockers: definition.blockers,
            assessment_commitment: definition.assessment_commitment,
        };
        value.validate()?;
        Ok(value)
    }

    /// Returns the serialized non-authoritative posture.
    #[must_use]
    pub const fn authority_posture(&self) -> AuthorizedExecutionAuthorityPosture {
        self.authority_posture
    }
    /// Returns current gate readiness.
    #[must_use]
    pub const fn readiness(&self) -> AuthorizedExecutionGateReadiness {
        self.readiness
    }
    /// Returns current typed prerequisite blockers.
    #[must_use]
    pub fn blockers(&self) -> &[AuthorizedExecutionGateBlocker] {
        &self.blockers
    }
    /// Returns the durable sequence assessed.
    #[must_use]
    pub const fn last_sequence_number(&self) -> EventSequenceNumber {
        self.last_sequence_number
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        validate_unique_bounded(
            &self.blockers,
            "gate_assessment.too_many_blockers",
            "gate_assessment.duplicate_blocker",
        )?;
        match (self.readiness, self.blockers.is_empty()) {
            (AuthorizedExecutionGateReadiness::ReadyForDecision, true)
            | (AuthorizedExecutionGateReadiness::PendingPrerequisites, false) => Ok(()),
            _ => Err(continuity_error(
                "gate_assessment.readiness_mismatch",
                "gate readiness does not match its prerequisite blockers",
            )),
        }
    }
}

impl fmt::Debug for AuthorizedExecutionGateAssessment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizedExecutionGateAssessment")
            .field("model_version", &self.model_version)
            .field("authority_posture", &self.authority_posture)
            .field("readiness", &self.readiness)
            .field("blocker_count", &self.blockers.len())
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl_safe_model_deserialize!(
    AuthorizedExecutionGateAssessment,
    AuthorizedExecutionGateAssessmentWire,
    |wire: AuthorizedExecutionGateAssessmentWire| {
        validate_wire_header(wire.model_version, wire.authority_posture)?;
        AuthorizedExecutionGateAssessment::new(AuthorizedExecutionGateAssessmentDefinition {
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            approval_reference: wire.approval_reference,
            action_reference: wire.action_reference,
            immutable_run_bundle: wire.immutable_run_bundle,
            last_sequence_number: wire.last_sequence_number,
            last_event_id: wire.last_event_id,
            assessed_at: wire.assessed_at,
            readiness: wire.readiness,
            blockers: wire.blockers,
            assessment_commitment: wire.assessment_commitment,
        })
    }
);

/// Current lifecycle posture of a model-only execution window.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionWindowStatus {
    /// Window is eligible only for a fresh authorization assessment.
    Open,
    /// Time bound has elapsed.
    Expired,
    /// Current authority revoked the window.
    Revoked,
    /// Governed work closed the window normally.
    Closed,
    /// Newer durable state superseded the window.
    Superseded,
}
safe_string_enum!(AuthorizedExecutionWindowStatus {
    "open" => Open, "expired" => Expired, "revoked" => Revoked, "closed" => Closed,
    "superseded" => Superseded
}, "authorized execution window status is invalid");

/// Public construction fields for one bounded scheduling envelope.
pub struct AuthorizedExecutionWindowDefinition {
    /// Stable window identity.
    pub window_id: AuthorizedExecutionWindowId,
    /// Current lifecycle posture.
    pub status: AuthorizedExecutionWindowStatus,
    /// Bound workflow identity.
    pub workflow_id: WorkflowId,
    /// Bound run identity.
    pub run_id: WorkflowRunId,
    /// Bound step identity.
    pub step_id: StepId,
    /// Immutable run material bound to the window.
    pub immutable_run_bundle: ImmutableRunBundleBinding,
    /// External executor subject.
    pub subject_actor_id: ActorId,
    /// Approval decisions included in the opening facts.
    pub approval_references: Vec<ApprovalReferenceId>,
    /// Allowed action classes for scheduling orientation.
    pub allowed_actions: Vec<AuthorizedExecutionActionReference>,
    /// Bounded resource scope.
    pub resource_scope: Vec<AuthorizedExecutionResourceReference>,
    /// Current-authority source binding.
    pub authority_source: AuthorizedExecutionAuthoritySourceReference,
    /// Durable sequence at open.
    pub opened_sequence_number: EventSequenceNumber,
    /// Durable event at open.
    pub opened_event_id: EventId,
    /// Opening time.
    pub opened_at: Timestamp,
    /// Time at which lifecycle posture was evaluated.
    pub evaluated_at: Timestamp,
    /// Mandatory upper time bound.
    pub expires_at: Timestamp,
    /// Maximum number of freshly authorized attempts.
    pub maximum_attempts: u32,
    /// Provenance event for every non-open lifecycle status.
    pub status_event_id: Option<EventId>,
    /// Maximum allowed information sensitivity.
    pub sensitivity_ceiling: WorkReportSensitivity,
    /// Commitment to proportional-governance and policy inputs.
    pub governance_commitment: SpecContentHash,
    /// Commitment to current authority inputs.
    pub authority_commitment: SpecContentHash,
}

/// Durable scheduling orientation for bounded work; possession grants no authority.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AuthorizedExecutionWindow {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    window_id: AuthorizedExecutionWindowId,
    status: AuthorizedExecutionWindowStatus,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    immutable_run_bundle: ImmutableRunBundleBinding,
    subject_actor_id: ActorId,
    approval_references: Vec<ApprovalReferenceId>,
    allowed_actions: Vec<AuthorizedExecutionActionReference>,
    resource_scope: Vec<AuthorizedExecutionResourceReference>,
    authority_source: AuthorizedExecutionAuthoritySourceReference,
    opened_sequence_number: EventSequenceNumber,
    opened_event_id: EventId,
    opened_at: Timestamp,
    evaluated_at: Timestamp,
    expires_at: Timestamp,
    maximum_attempts: u32,
    status_event_id: Option<EventId>,
    sensitivity_ceiling: WorkReportSensitivity,
    governance_commitment: SpecContentHash,
    authority_commitment: SpecContentHash,
}

impl AuthorizedExecutionWindow {
    /// Creates a validated non-authoritative execution-window projection.
    ///
    /// # Errors
    ///
    /// Returns a stable error for unbound scope, invalid time/budget, or unsupported lifecycle claims.
    pub fn new(definition: AuthorizedExecutionWindowDefinition) -> Result<Self, WorkflowOsError> {
        let value = Self {
            model_version: AuthorizedExecutionContinuityModelVersion::V1,
            authority_posture: AuthorizedExecutionAuthorityPosture::NonAuthoritative,
            window_id: definition.window_id,
            status: definition.status,
            workflow_id: definition.workflow_id,
            run_id: definition.run_id,
            step_id: definition.step_id,
            immutable_run_bundle: definition.immutable_run_bundle,
            subject_actor_id: definition.subject_actor_id,
            approval_references: definition.approval_references,
            allowed_actions: definition.allowed_actions,
            resource_scope: definition.resource_scope,
            authority_source: definition.authority_source,
            opened_sequence_number: definition.opened_sequence_number,
            opened_event_id: definition.opened_event_id,
            opened_at: definition.opened_at,
            evaluated_at: definition.evaluated_at,
            expires_at: definition.expires_at,
            maximum_attempts: definition.maximum_attempts,
            status_event_id: definition.status_event_id,
            sensitivity_ceiling: definition.sensitivity_ceiling,
            governance_commitment: definition.governance_commitment,
            authority_commitment: definition.authority_commitment,
        };
        value.validate()?;
        Ok(value)
    }

    /// Returns the serialized non-authoritative posture.
    #[must_use]
    pub const fn authority_posture(&self) -> AuthorizedExecutionAuthorityPosture {
        self.authority_posture
    }
    /// Returns the stable window identity.
    #[must_use]
    pub const fn window_id(&self) -> &AuthorizedExecutionWindowId {
        &self.window_id
    }
    /// Returns the lifecycle posture.
    #[must_use]
    pub const fn status(&self) -> AuthorizedExecutionWindowStatus {
        self.status
    }
    /// Returns the bound run identity.
    #[must_use]
    pub const fn run_id(&self) -> &WorkflowRunId {
        &self.run_id
    }
    /// Returns the bound step identity.
    #[must_use]
    pub const fn step_id(&self) -> &StepId {
        &self.step_id
    }
    /// Returns the bounded attempt count.
    #[must_use]
    pub const fn maximum_attempts(&self) -> u32 {
        self.maximum_attempts
    }

    fn validate(&self) -> Result<(), WorkflowOsError> {
        if self.authority_posture != AuthorizedExecutionAuthorityPosture::NonAuthoritative {
            return Err(continuity_error(
                "window.authority_posture_invalid",
                "execution window must be explicitly non-authoritative",
            ));
        }
        if self.maximum_attempts == 0 || self.maximum_attempts > EXECUTION_WINDOW_MAX_ATTEMPTS {
            return Err(continuity_error(
                "window.attempt_bound_invalid",
                "execution window attempt bound is invalid",
            ));
        }
        if self.evaluated_at < self.opened_at || self.expires_at <= self.opened_at {
            return Err(continuity_error(
                "window.time_bound_invalid",
                "execution window time bounds are invalid",
            ));
        }
        validate_nonempty_unique(&self.allowed_actions, "window.allowed_actions_invalid")?;
        validate_nonempty_unique(&self.resource_scope, "window.resource_scope_invalid")?;
        validate_unique_bounded(
            &self.approval_references,
            "window.too_many_approval_references",
            "window.duplicate_approval_reference",
        )?;
        if self.sensitivity_ceiling == WorkReportSensitivity::Unknown {
            return Err(continuity_error(
                "window.sensitivity_unknown",
                "execution window sensitivity must be known",
            ));
        }
        let is_open = self.status == AuthorizedExecutionWindowStatus::Open;
        if is_open != self.status_event_id.is_none() {
            return Err(continuity_error(
                "window.status_provenance_mismatch",
                "closed execution-window status requires provenance and open status forbids it",
            ));
        }
        if self.status == AuthorizedExecutionWindowStatus::Expired
            && self.evaluated_at < self.expires_at
        {
            return Err(continuity_error(
                "window.expired_before_deadline",
                "expired execution window was evaluated before its expiry",
            ));
        }
        if self.status == AuthorizedExecutionWindowStatus::Open
            && self.evaluated_at >= self.expires_at
        {
            return Err(continuity_error(
                "window.open_after_expiry",
                "open execution window has reached its expiry",
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for AuthorizedExecutionWindow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizedExecutionWindow")
            .field("model_version", &self.model_version)
            .field("authority_posture", &self.authority_posture)
            .field("status", &self.status)
            .field("maximum_attempts", &self.maximum_attempts)
            .field("allowed_action_count", &self.allowed_actions.len())
            .field("resource_count", &self.resource_scope.len())
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl_safe_model_deserialize!(
    AuthorizedExecutionWindow,
    AuthorizedExecutionWindowWire,
    |wire: AuthorizedExecutionWindowWire| {
        validate_wire_header(wire.model_version, wire.authority_posture)?;
        AuthorizedExecutionWindow::new(AuthorizedExecutionWindowDefinition {
            window_id: wire.window_id,
            status: wire.status,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            immutable_run_bundle: wire.immutable_run_bundle,
            subject_actor_id: wire.subject_actor_id,
            approval_references: wire.approval_references,
            allowed_actions: wire.allowed_actions,
            resource_scope: wire.resource_scope,
            authority_source: wire.authority_source,
            opened_sequence_number: wire.opened_sequence_number,
            opened_event_id: wire.opened_event_id,
            opened_at: wire.opened_at,
            evaluated_at: wire.evaluated_at,
            expires_at: wire.expires_at,
            maximum_attempts: wire.maximum_attempts,
            status_event_id: wire.status_event_id,
            sensitivity_ceiling: wire.sensitivity_ceiling,
            governance_commitment: wire.governance_commitment,
            authority_commitment: wire.authority_commitment,
        })
    }
);

/// Reason an external executor yielded while the run remained non-terminal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionYieldReason {
    /// Ordinary executor or conversation turn boundary.
    TurnBoundary,
    /// Executor reached its bounded context budget.
    ContextBudget,
    /// Host scheduler preempted the executor.
    HostPreemption,
    /// Executor intentionally emitted a governed checkpoint.
    VoluntaryCheckpoint,
    /// Executor stopped after a transient local failure.
    TransientExecutorFailure,
}
safe_string_enum!(AuthorizedExecutionYieldReason {
    "turn_boundary" => TurnBoundary, "context_budget" => ContextBudget,
    "host_preemption" => HostPreemption, "voluntary_checkpoint" => VoluntaryCheckpoint,
    "transient_executor_failure" => TransientExecutorFailure
}, "authorized execution yield reason is invalid");

/// Non-authoritative scheduling disposition after yield.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionResumeDisposition {
    /// Host may ask Core for a fresh exact authorization decision.
    EligibleForFreshAuthorization,
    /// Host must preserve the typed wait conditions.
    Wait,
}
safe_string_enum!(AuthorizedExecutionResumeDisposition {
    "eligible_for_fresh_authorization" => EligibleForFreshAuthorization, "wait" => Wait
}, "authorized execution resume disposition is invalid");

/// Verification posture for a serialized yield's execution-window binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionYieldBindingVerification {
    /// The owning window must be reloaded before the serialized yield is trusted.
    RequiresOwningWindowReconciliation,
}
safe_string_enum!(AuthorizedExecutionYieldBindingVerification {
    "requires_owning_window_reconciliation" => RequiresOwningWindowReconciliation
}, "authorized execution yield binding verification is invalid");

/// Bounded outcome of one attempt. Every outcome blocks retry until fresh authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizedExecutionAttemptOutcome {
    /// Exact attempt completed successfully.
    Succeeded,
    /// Attempt failed in a potentially retryable way.
    RetryableFailure,
    /// Attempt failed terminally.
    TerminalFailure,
    /// Attempt may have started and needs reconciliation.
    AmbiguousMayHaveStarted,
}
safe_string_enum!(AuthorizedExecutionAttemptOutcome {
    "succeeded" => Succeeded, "retryable_failure" => RetryableFailure,
    "terminal_failure" => TerminalFailure, "ambiguous_may_have_started" => AmbiguousMayHaveStarted
}, "authorized execution attempt outcome is invalid");

impl AuthorizedExecutionAttemptOutcome {
    /// Returns true because every retry requires a new current authorization.
    #[must_use]
    pub const fn blocks_automatic_retry(self) -> bool {
        true
    }
    /// Returns whether operator or deterministic reconciliation is required.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(self, Self::AmbiguousMayHaveStarted)
    }
}

/// Public fields for one executor-yield projection.
pub struct AuthorizedExecutionYieldDefinition {
    /// Exact attempt that yielded.
    pub attempt_id: AuthorizedExecutionAttemptId,
    /// Durable sequence observed at yield.
    pub yielded_sequence_number: EventSequenceNumber,
    /// Durable event observed at yield.
    pub yielded_event_id: EventId,
    /// Yield time.
    pub yielded_at: Timestamp,
    /// Reason the external executor stopped producing work.
    pub reason: AuthorizedExecutionYieldReason,
    /// Exact active dependency waits, if any.
    pub wait_conditions: Vec<AuthorizedExecutionWaitCondition>,
    /// Non-authoritative scheduling disposition.
    pub resume_disposition: AuthorizedExecutionResumeDisposition,
}

/// Bounded model of an executor yielding one exact window-bound attempt.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct AuthorizedExecutionYield {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    window_id: AuthorizedExecutionWindowId,
    window_governance_commitment: SpecContentHash,
    binding_verification: AuthorizedExecutionYieldBindingVerification,
    attempt_id: AuthorizedExecutionAttemptId,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    yielded_sequence_number: EventSequenceNumber,
    yielded_event_id: EventId,
    yielded_at: Timestamp,
    reason: AuthorizedExecutionYieldReason,
    wait_conditions: Vec<AuthorizedExecutionWaitCondition>,
    resume_disposition: AuthorizedExecutionResumeDisposition,
}

impl AuthorizedExecutionYield {
    /// Creates a yield derived from one validated open execution window.
    ///
    /// # Errors
    ///
    /// Returns a stable error for closed/stale windows or inconsistent genuine waits.
    pub fn new(
        window: &AuthorizedExecutionWindow,
        definition: AuthorizedExecutionYieldDefinition,
    ) -> Result<Self, WorkflowOsError> {
        if window.status != AuthorizedExecutionWindowStatus::Open {
            return Err(continuity_error(
                "yield.window_not_open",
                "executor yield requires an open execution window",
            ));
        }
        let value = Self {
            model_version: AuthorizedExecutionContinuityModelVersion::V1,
            authority_posture: AuthorizedExecutionAuthorityPosture::NonAuthoritative,
            window_id: window.window_id.clone(),
            window_governance_commitment: window.governance_commitment.clone(),
            binding_verification:
                AuthorizedExecutionYieldBindingVerification::RequiresOwningWindowReconciliation,
            attempt_id: definition.attempt_id,
            workflow_id: window.workflow_id.clone(),
            run_id: window.run_id.clone(),
            step_id: window.step_id.clone(),
            yielded_sequence_number: definition.yielded_sequence_number,
            yielded_event_id: definition.yielded_event_id,
            yielded_at: definition.yielded_at,
            reason: definition.reason,
            wait_conditions: definition.wait_conditions,
            resume_disposition: definition.resume_disposition,
        };
        value.validate(Some(window))?;
        Ok(value)
    }

    /// Returns the bound window identity.
    #[must_use]
    pub const fn window_id(&self) -> &AuthorizedExecutionWindowId {
        &self.window_id
    }
    /// Returns the exact yielded attempt identity.
    #[must_use]
    pub const fn attempt_id(&self) -> &AuthorizedExecutionAttemptId {
        &self.attempt_id
    }
    /// Returns active typed waits.
    #[must_use]
    pub fn wait_conditions(&self) -> &[AuthorizedExecutionWaitCondition] {
        &self.wait_conditions
    }
    /// Returns the yield reason.
    #[must_use]
    pub const fn reason(&self) -> AuthorizedExecutionYieldReason {
        self.reason
    }
    /// Returns the non-authoritative scheduling disposition.
    #[must_use]
    pub const fn resume_disposition(&self) -> AuthorizedExecutionResumeDisposition {
        self.resume_disposition
    }
    /// Returns the serialized non-authoritative posture.
    #[must_use]
    pub const fn authority_posture(&self) -> AuthorizedExecutionAuthorityPosture {
        self.authority_posture
    }

    fn validate(&self, window: Option<&AuthorizedExecutionWindow>) -> Result<(), WorkflowOsError> {
        if self.binding_verification
            != AuthorizedExecutionYieldBindingVerification::RequiresOwningWindowReconciliation
        {
            return Err(continuity_error(
                "yield.binding_verification_invalid",
                "serialized executor yield requires owning-window reconciliation",
            ));
        }
        validate_unique_bounded(
            &self.wait_conditions,
            "wait.too_many_conditions",
            "wait.duplicate_condition",
        )?;
        for condition in &self.wait_conditions {
            condition.validate()?;
            if condition.status != AuthorizedExecutionWaitStatus::Waiting
                || condition.window_id != self.window_id
                || condition.workflow_id != self.workflow_id
                || condition.run_id != self.run_id
                || condition.step_id != self.step_id
                || condition.attempt_id != self.attempt_id
                || condition.expected_sequence_number != self.yielded_sequence_number
                || condition.expected_event_id != self.yielded_event_id
                || condition.created_at > self.yielded_at
            {
                return Err(continuity_error(
                    "yield.wait_binding_mismatch",
                    "executor-yield wait is not actively bound to the yielded attempt",
                ));
            }
        }
        match (self.wait_conditions.is_empty(), self.resume_disposition) {
            (true, AuthorizedExecutionResumeDisposition::EligibleForFreshAuthorization)
            | (false, AuthorizedExecutionResumeDisposition::Wait) => {}
            _ => {
                return Err(continuity_error(
                    "yield.resume_disposition_mismatch",
                    "executor-yield waits do not match its resume disposition",
                ))
            }
        }
        if let Some(window) = window {
            if self.yielded_at < window.evaluated_at
                || self.yielded_at >= window.expires_at
                || self.yielded_sequence_number < window.opened_sequence_number
                || (self.yielded_sequence_number == window.opened_sequence_number
                    && self.yielded_event_id != window.opened_event_id)
                || self.window_governance_commitment != window.governance_commitment
                || self.wait_conditions.iter().any(|condition| {
                    condition.created_at < window.evaluated_at
                        || !window.allowed_actions.contains(&condition.action_reference)
                })
            {
                return Err(continuity_error(
                    "yield.window_binding_mismatch",
                    "executor yield does not match the execution window",
                ));
            }
        }
        Ok(())
    }
}

impl fmt::Debug for AuthorizedExecutionYield {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizedExecutionYield")
            .field("model_version", &self.model_version)
            .field("authority_posture", &self.authority_posture)
            .field("reason", &self.reason)
            .field("wait_condition_count", &self.wait_conditions.len())
            .field("resume_disposition", &self.resume_disposition)
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl_safe_model_deserialize!(
    AuthorizedExecutionYield,
    AuthorizedExecutionYieldWire,
    |wire: AuthorizedExecutionYieldWire| {
        validate_wire_header(wire.model_version, wire.authority_posture)?;
        let value = AuthorizedExecutionYield {
            model_version: wire.model_version,
            authority_posture: wire.authority_posture,
            window_id: wire.window_id,
            window_governance_commitment: wire.window_governance_commitment,
            binding_verification: wire.binding_verification,
            attempt_id: wire.attempt_id,
            workflow_id: wire.workflow_id,
            run_id: wire.run_id,
            step_id: wire.step_id,
            yielded_sequence_number: wire.yielded_sequence_number,
            yielded_event_id: wire.yielded_event_id,
            yielded_at: wire.yielded_at,
            reason: wire.reason,
            wait_conditions: wire.wait_conditions,
            resume_disposition: wire.resume_disposition,
        };
        value.validate(None).map(|()| value)
    }
);

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizedExecutionWaitConditionWire {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    condition_id: AuthorizedExecutionWaitConditionId,
    condition_version: u32,
    kind: AuthorizedExecutionWaitConditionKind,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    window_id: AuthorizedExecutionWindowId,
    action_reference: AuthorizedExecutionActionReference,
    step_id: StepId,
    attempt_id: AuthorizedExecutionAttemptId,
    expected_sequence_number: EventSequenceNumber,
    expected_event_id: EventId,
    required_reference: AuthorizedExecutionResourceReference,
    created_at: Timestamp,
    deadline: Option<Timestamp>,
    wake_trigger: AuthorizedExecutionWakeTriggerKind,
    status: AuthorizedExecutionWaitStatus,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizedExecutionGateAssessmentWire {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    approval_reference: ApprovalReferenceId,
    action_reference: AuthorizedExecutionActionReference,
    immutable_run_bundle: ImmutableRunBundleBinding,
    last_sequence_number: EventSequenceNumber,
    last_event_id: EventId,
    assessed_at: Timestamp,
    readiness: AuthorizedExecutionGateReadiness,
    blockers: Vec<AuthorizedExecutionGateBlocker>,
    assessment_commitment: SpecContentHash,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizedExecutionWindowWire {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    window_id: AuthorizedExecutionWindowId,
    status: AuthorizedExecutionWindowStatus,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    immutable_run_bundle: ImmutableRunBundleBinding,
    subject_actor_id: ActorId,
    approval_references: Vec<ApprovalReferenceId>,
    allowed_actions: Vec<AuthorizedExecutionActionReference>,
    resource_scope: Vec<AuthorizedExecutionResourceReference>,
    authority_source: AuthorizedExecutionAuthoritySourceReference,
    opened_sequence_number: EventSequenceNumber,
    opened_event_id: EventId,
    opened_at: Timestamp,
    evaluated_at: Timestamp,
    expires_at: Timestamp,
    maximum_attempts: u32,
    status_event_id: Option<EventId>,
    sensitivity_ceiling: WorkReportSensitivity,
    governance_commitment: SpecContentHash,
    authority_commitment: SpecContentHash,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizedExecutionYieldWire {
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
    window_id: AuthorizedExecutionWindowId,
    window_governance_commitment: SpecContentHash,
    binding_verification: AuthorizedExecutionYieldBindingVerification,
    attempt_id: AuthorizedExecutionAttemptId,
    workflow_id: WorkflowId,
    run_id: WorkflowRunId,
    step_id: StepId,
    yielded_sequence_number: EventSequenceNumber,
    yielded_event_id: EventId,
    yielded_at: Timestamp,
    reason: AuthorizedExecutionYieldReason,
    wait_conditions: Vec<AuthorizedExecutionWaitCondition>,
    resume_disposition: AuthorizedExecutionResumeDisposition,
}

fn deserialize_wire_safely<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer).map_err(|_| {
        serde::de::Error::custom("authorized execution continuity value is invalid")
    })?;
    serde_json::from_value(value)
        .map_err(|_| serde::de::Error::custom("authorized execution continuity value is invalid"))
}

fn validate_wire_header(
    model_version: AuthorizedExecutionContinuityModelVersion,
    authority_posture: AuthorizedExecutionAuthorityPosture,
) -> Result<(), WorkflowOsError> {
    if model_version != AuthorizedExecutionContinuityModelVersion::V1
        || authority_posture != AuthorizedExecutionAuthorityPosture::NonAuthoritative
    {
        return Err(continuity_error(
            "wire_header.invalid",
            "authorized execution continuity wire header is invalid",
        ));
    }
    Ok(())
}

fn validate_unique_bounded<T: Ord>(
    values: &[T],
    too_many: &'static str,
    duplicate: &'static str,
) -> Result<(), WorkflowOsError> {
    if values.len() > CONDITION_MAX_COUNT {
        return Err(continuity_error(
            too_many,
            "authorized execution collection exceeds its bound",
        ));
    }
    let mut unique = BTreeSet::new();
    for value in values {
        if !unique.insert(value) {
            return Err(continuity_error(
                duplicate,
                "authorized execution collection contains a duplicate",
            ));
        }
    }
    Ok(())
}

fn validate_nonempty_unique<T: Ord>(
    values: &[T],
    code: &'static str,
) -> Result<(), WorkflowOsError> {
    if values.is_empty() {
        return Err(continuity_error(
            code,
            "authorized execution scope must not be empty",
        ));
    }
    validate_unique_bounded(values, code, code)
}

fn validate_identifier(
    label: &'static str,
    value: &str,
    code: &'static str,
    maximum: usize,
) -> Result<(), WorkflowOsError> {
    if value.is_empty()
        || value.len() > maximum
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
    {
        return Err(continuity_error(code, format!("{label} is invalid")));
    }
    validate_not_secret_like(label, value)
}

fn validate_not_secret_like(label: &'static str, value: &str) -> Result<(), WorkflowOsError> {
    let lowercase = value.to_ascii_lowercase();
    if [
        "authorization",
        "bearer",
        "private_key",
        "private-key",
        "api_token",
        "api-token",
        "secret",
        "token",
    ]
    .iter()
    .any(|needle| lowercase.contains(needle))
    {
        return Err(continuity_error(
            "secret_like_value",
            format!("{label} contains sensitive-looking text"),
        ));
    }
    Ok(())
}

fn continuity_error(code: &'static str, message: impl Into<String>) -> WorkflowOsError {
    WorkflowOsError::validation(format!("authorized_execution_continuity.{code}"), message)
}
