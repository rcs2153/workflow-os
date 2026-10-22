use std::fmt;

use sha2::{Digest, Sha256};

use crate::{
    AuthorizedExecutionAttemptId, EventId, FailureClass, FailureRecord, IdempotencyKey,
    SkillAttemptId, SkillId, SkillInvocation, SkillInvocationAttempt, SkillInvocationId,
    SkillOutput, SkillVersion, SpecContentHash, StepId, WorkflowOsError, WorkflowRunEventKind,
};

#[cfg(feature = "trusted-host-application-spi")]
use crate::SkillDefinition;

#[derive(Clone)]
pub(crate) struct TrustedHostWorkflowLifecycleContext {
    invocation_id: SkillInvocationId,
    invocation_idempotency_key: IdempotencyKey,
    step_id: StepId,
    skill_id: SkillId,
    skill_version: SkillVersion,
    required_outputs: Vec<String>,
}

impl TrustedHostWorkflowLifecycleContext {
    #[cfg(feature = "trusted-host-application-spi")]
    pub(crate) fn from_skill(
        invocation_id: SkillInvocationId,
        invocation_idempotency_key: IdempotencyKey,
        step_id: StepId,
        skill: &SkillDefinition,
    ) -> Self {
        Self {
            invocation_id,
            invocation_idempotency_key,
            step_id,
            skill_id: skill.id.clone(),
            skill_version: skill.version.clone(),
            required_outputs: skill.output_contract.required.clone(),
        }
    }

    #[cfg(test)]
    #[allow(clippy::expect_used)]
    pub(crate) fn test_context() -> Self {
        Self {
            invocation_id: SkillInvocationId::new("invocation/trusted-host-test")
                .expect("test invocation ID"),
            invocation_idempotency_key: IdempotencyKey::new(
                "invocation/trusted-host-test/idempotency",
            )
            .expect("test idempotency key"),
            step_id: StepId::new("step-sqlite-continuity").expect("test step ID"),
            skill_id: SkillId::new("local/trusted-host-test").expect("test skill ID"),
            skill_version: SkillVersion::new("v0").expect("test skill version"),
            required_outputs: Vec::new(),
        }
    }

    pub(crate) fn commitment(&self) -> SpecContentHash {
        let mut hasher = Sha256::new();
        hasher.update(b"workflow-os/trusted-host-workflow-lifecycle/context/v1\0");
        hasher.update(self.invocation_id.as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(self.invocation_idempotency_key.as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(self.step_id.as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(self.skill_id.as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(self.skill_version.as_str().as_bytes());
        for required in &self.required_outputs {
            hasher.update(b"\0");
            hasher.update(required.as_bytes());
        }
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(crate) fn outcome_request_commitment(
        &self,
        base: &SpecContentHash,
        outcome: &TrustedHostWorkflowTerminalOutcome,
    ) -> SpecContentHash {
        let mut hasher = Sha256::new();
        hasher.update(b"workflow-os/trusted-host-workflow-lifecycle/outcome-request/v1\0");
        hasher.update(base.as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(self.commitment().as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(outcome.commitment().as_str().as_bytes());
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(crate) fn outcome_projection_request_commitment(
        &self,
        base: &SpecContentHash,
        outcome: &TrustedHostWorkflowTerminalOutcome,
        local_check_event: Option<&crate::LocalCheckResultRecordedEvent>,
    ) -> SpecContentHash {
        let base = self.outcome_request_commitment(base, outcome);
        let Some(local_check_event) = local_check_event else {
            return base;
        };
        let mut hasher = Sha256::new();
        hasher.update(b"workflow-os/trusted-host-workflow-lifecycle/result-projection/v1\0");
        hasher.update(base.as_str().as_bytes());
        hasher.update(b"\0");
        hasher.update(local_check_event.commitment().as_str().as_bytes());
        SpecContentHash::from_bytes(hasher.finalize())
    }

    pub(crate) fn request_event(&self) -> WorkflowRunEventKind {
        WorkflowRunEventKind::SkillInvocationRequested(SkillInvocation {
            invocation_id: self.invocation_id.clone(),
            step_id: self.step_id.clone(),
            skill_id: self.skill_id.clone(),
            skill_version: self.skill_version.clone(),
            idempotency_key: Some(self.invocation_idempotency_key.clone()),
            attempts: Vec::new(),
        })
    }

    pub(crate) fn started_event(
        &self,
        attempt_id: &AuthorizedExecutionAttemptId,
    ) -> Result<WorkflowRunEventKind, WorkflowOsError> {
        Ok(WorkflowRunEventKind::SkillInvocationStarted(
            SkillInvocationAttempt {
                invocation_id: self.invocation_id.clone(),
                attempt_id: derive_skill_attempt_id(attempt_id)?,
                step_id: self.step_id.clone(),
                skill_id: self.skill_id.clone(),
                skill_version: self.skill_version.clone(),
                attempt_number: 1,
            },
        ))
    }

    pub(crate) fn normalize_success(
        &self,
        output: &SkillOutput,
    ) -> TrustedHostWorkflowTerminalOutcome {
        if self
            .required_outputs
            .iter()
            .any(|required| !output.values.contains_key(required))
        {
            return TrustedHostWorkflowTerminalOutcome::Failed(output_validation_failure());
        }
        TrustedHostWorkflowTerminalOutcome::Succeeded {
            output_ref: output.output_ref.clone(),
        }
    }

    pub(crate) fn terminal_events(
        &self,
        outcome: &TrustedHostWorkflowTerminalOutcome,
    ) -> [WorkflowRunEventKind; 2] {
        match outcome {
            TrustedHostWorkflowTerminalOutcome::Succeeded { output_ref } => [
                WorkflowRunEventKind::SkillInvocationSucceeded {
                    invocation_id: self.invocation_id.clone(),
                    step_id: self.step_id.clone(),
                    skill_id: self.skill_id.clone(),
                    skill_version: self.skill_version.clone(),
                    output_ref: output_ref.clone(),
                },
                WorkflowRunEventKind::RunCompleted,
            ],
            TrustedHostWorkflowTerminalOutcome::Failed(failure) => [
                WorkflowRunEventKind::SkillInvocationFailed {
                    invocation_id: self.invocation_id.clone(),
                    step_id: self.step_id.clone(),
                    skill_id: self.skill_id.clone(),
                    skill_version: self.skill_version.clone(),
                    failure: failure.clone(),
                },
                WorkflowRunEventKind::RunFailed(failure.clone()),
            ],
        }
    }

    pub(crate) const fn invocation_id(&self) -> &SkillInvocationId {
        &self.invocation_id
    }

    pub(crate) const fn step_id(&self) -> &StepId {
        &self.step_id
    }

    pub(crate) fn event_id(
        &self,
        domain: &str,
        operation_id: &str,
    ) -> Result<EventId, WorkflowOsError> {
        let mut hasher = Sha256::new();
        hasher.update(b"workflow-os/trusted-host-workflow-lifecycle/v1\0");
        hasher.update(domain.as_bytes());
        hasher.update(b"\0");
        hasher.update(operation_id.as_bytes());
        hasher.update(b"\0");
        hasher.update(self.invocation_id.as_str().as_bytes());
        let digest = SpecContentHash::from_bytes(hasher.finalize());
        EventId::new(format!(
            "event/trusted-host-{}-{}",
            domain,
            &digest.as_str()[..24]
        ))
    }

    pub(crate) fn idempotency_key(&self, domain: &str) -> Result<IdempotencyKey, WorkflowOsError> {
        IdempotencyKey::new(format!(
            "trusted-host/{domain}/{}",
            self.invocation_id.as_str()
        ))
    }
}

impl fmt::Debug for TrustedHostWorkflowLifecycleContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedHostWorkflowLifecycleContext")
            .field("binding", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

pub(crate) enum TrustedHostWorkflowTerminalOutcome {
    Succeeded { output_ref: Option<String> },
    Failed(FailureRecord),
}

impl TrustedHostWorkflowTerminalOutcome {
    pub(crate) fn terminal_failure() -> Self {
        Self::Failed(FailureRecord {
            code: "trusted_host.execution.failed".to_owned(),
            message: "trusted-host execution failed".to_owned(),
            failure_class: FailureClass::Permanent,
        })
    }

    pub(crate) const fn continuity_outcome(&self) -> crate::AuthorizedExecutionAttemptOutcome {
        match self {
            Self::Succeeded { .. } => crate::AuthorizedExecutionAttemptOutcome::Succeeded,
            Self::Failed(_) => crate::AuthorizedExecutionAttemptOutcome::TerminalFailure,
        }
    }

    fn commitment(&self) -> SpecContentHash {
        let mut hasher = Sha256::new();
        hasher.update(b"workflow-os/trusted-host-workflow-lifecycle/terminal-outcome/v1\0");
        match self {
            Self::Succeeded { output_ref } => {
                hasher.update(b"succeeded\0");
                hasher.update(output_ref.as_deref().unwrap_or("none").as_bytes());
            }
            Self::Failed(failure) => {
                hasher.update(b"failed\0");
                hasher.update(failure.code.as_bytes());
                hasher.update(b"\0");
                hasher.update(failure.message.as_bytes());
                hasher.update(b"\0");
                hasher.update(
                    match failure.failure_class {
                        FailureClass::Transient => "transient",
                        FailureClass::Permanent => "permanent",
                        FailureClass::PolicyDenied => "policy_denied",
                        FailureClass::Timeout => "timeout",
                        FailureClass::Canceled => "canceled",
                        FailureClass::Unknown => "unknown",
                    }
                    .as_bytes(),
                );
            }
        }
        SpecContentHash::from_bytes(hasher.finalize())
    }
}

impl fmt::Debug for TrustedHostWorkflowTerminalOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded { output_ref } => formatter
                .debug_struct("TrustedHostWorkflowTerminalOutcome::Succeeded")
                .field("output_ref", &output_ref.as_ref().map(|_| "[REDACTED]"))
                .finish(),
            Self::Failed(_) => formatter
                .debug_struct("TrustedHostWorkflowTerminalOutcome::Failed")
                .field("failure", &"[REDACTED]")
                .finish(),
        }
    }
}

fn derive_skill_attempt_id(
    attempt_id: &AuthorizedExecutionAttemptId,
) -> Result<SkillAttemptId, WorkflowOsError> {
    let mut hasher = Sha256::new();
    hasher.update(b"workflow-os/trusted-host-skill-attempt/v1\0");
    hasher.update(attempt_id.as_str().as_bytes());
    let digest = SpecContentHash::from_bytes(hasher.finalize());
    SkillAttemptId::new(format!("attempt/trusted-host-{}", &digest.as_str()[..24]))
}

fn output_validation_failure() -> FailureRecord {
    FailureRecord {
        code: "executor.output.required_missing".to_owned(),
        message: "trusted-host skill output is missing a required field".to_owned(),
        failure_class: FailureClass::Permanent,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    fn context() -> TrustedHostWorkflowLifecycleContext {
        TrustedHostWorkflowLifecycleContext {
            invocation_id: SkillInvocationId::new("invocation/output-validation")
                .expect("invocation id"),
            invocation_idempotency_key: IdempotencyKey::new(
                "invocation/output-validation/idempotency",
            )
            .expect("idempotency key"),
            step_id: StepId::new("docs-check").expect("step id"),
            skill_id: SkillId::new("local/check-docs").expect("skill id"),
            skill_version: SkillVersion::new("v0").expect("skill version"),
            required_outputs: vec!["summary".to_owned(), "local_check_status".to_owned()],
        }
    }

    #[test]
    fn required_output_validation_accepts_complete_bounded_output() {
        let output = SkillOutput::new(
            BTreeMap::from([
                ("summary".to_owned(), "completed".to_owned()),
                ("local_check_status".to_owned(), "passed".to_owned()),
            ]),
            Some("local-check/result".to_owned()),
        );

        assert!(matches!(
            context().normalize_success(&output),
            TrustedHostWorkflowTerminalOutcome::Succeeded {
                output_ref: Some(reference)
            } if reference == "local-check/result"
        ));
    }

    #[test]
    fn required_output_validation_fails_closed_without_copying_output() {
        let secret_marker = "authorization-private-key-marker";
        let output = SkillOutput::new(
            BTreeMap::from([("summary".to_owned(), secret_marker.to_owned())]),
            Some(secret_marker.to_owned()),
        );

        let outcome = context().normalize_success(&output);
        assert!(matches!(
            outcome,
            TrustedHostWorkflowTerminalOutcome::Failed(_)
        ));
        let TrustedHostWorkflowTerminalOutcome::Failed(failure) = &outcome else {
            return;
        };
        assert_eq!(failure.code, "executor.output.required_missing");
        for rendered in [format!("{outcome:?}"), format!("{failure:?}")] {
            assert!(!rendered.contains(secret_marker));
        }
        let [invocation_event, run_event] = context().terminal_events(&outcome);
        assert!(matches!(
            invocation_event,
            WorkflowRunEventKind::SkillInvocationFailed { .. }
        ));
        assert!(matches!(run_event, WorkflowRunEventKind::RunFailed(_)));
    }

    #[test]
    fn terminal_handler_failure_maps_to_bounded_workflow_failure_events() {
        let secret_marker = "authorization-private-key-marker";
        let outcome = TrustedHostWorkflowTerminalOutcome::terminal_failure();
        let [invocation_event, run_event] = context().terminal_events(&outcome);

        assert!(matches!(
            invocation_event,
            WorkflowRunEventKind::SkillInvocationFailed { .. }
        ));
        let WorkflowRunEventKind::SkillInvocationFailed { failure, .. } = invocation_event else {
            return;
        };
        assert_eq!(failure.code, "trusted_host.execution.failed");
        assert_eq!(failure.failure_class, FailureClass::Permanent);
        assert!(matches!(run_event, WorkflowRunEventKind::RunFailed(_)));
        for rendered in [
            format!("{outcome:?}"),
            format!("{failure:?}"),
            format!("{run_event:?}"),
        ] {
            assert!(!rendered.contains(secret_marker));
        }
    }
}
