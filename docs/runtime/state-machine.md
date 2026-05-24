# Workflow Run State Machine

This document defines the v0 workflow run state machine. Future runtime implementation must enforce these transitions and reject invalid transitions.

## States

### Created

The run record exists and is bound to workflow ID, workflow version, schema version, and spec content hash. Execution has not started.

### Validated

The run's workflow definition and required project context have passed deterministic validation.

### Running

The runtime is actively evaluating steps, checking policy, invoking skills, or advancing the run.

### WaitingForApproval

The run is paused until a required approval is granted, denied, or expires.

### WaitingForExternalEvent

The run is paused until a required external event is received, deduplicated, and accepted by policy.

### Retrying

The run is waiting for or performing a retry after a retryable failure.

### Escalated

The run requires human review or intervention because policy, ambiguity, retry exhaustion, timeout, unsafe state, or another governed condition requires escalation.

### Completed

The run finished successfully. This is a terminal state.

### Failed

The run ended unsuccessfully. This is a terminal state.

### Canceled

The run was explicitly canceled. This is a terminal state.

## Terminal States

Terminal states are:

- Completed.
- Failed.
- Canceled.

Terminal states must not transition to any other state in v0. Re-run or resume-after-terminal behavior requires a new run.

## Transition Events

| From | To | Event Name | Description |
| --- | --- | --- | --- |
| None | Created | `RunCreated` | A run record is created for an immutable workflow spec identity. |
| Created | Validated | `RunValidated` | Deterministic validation succeeds. |
| Created | Failed | `RunValidationFailed` | Deterministic validation fails. |
| Validated | Running | `RunStarted` | Execution starts. |
| Any non-terminal state | Same state | `PolicyDecisionRecorded` | Policy decision is recorded for audit before a meaningful action. |
| Running | WaitingForApproval | `ApprovalRequested` | Policy or workflow definition requires approval before continuing. |
| WaitingForApproval | WaitingForApproval | `ApprovalGranted` | Required approval is granted and recorded. `RunResumed` must follow before execution continues. |
| WaitingForApproval | WaitingForApproval | `ApprovalDenied` | Approval is denied and recorded. v0 local execution then emits `RunFailed`. |
| WaitingForApproval | Running | `RunResumed` | A granted approval resumes execution. |
| WaitingForApproval | Escalated | `ApprovalExpired` | Approval window expires and policy requires escalation. |
| WaitingForApproval | Failed | `ApprovalExpiredTerminal` | Approval window expires and policy requires terminal failure. |
| Running | WaitingForExternalEvent | `ExternalEventRequired` | Execution pauses for an external event. |
| WaitingForExternalEvent | Running | `ExternalEventReceived` | Required external event is accepted after deduplication and policy checks. |
| Running | WaitingForExternalEvent | `RunPaused` | v0 runtime state model pauses a run for an external wait. Future executor work may introduce a more specific scheduling event before this transition. |
| WaitingForExternalEvent | Escalated | `ExternalEventTimedOut` | External event wait times out and policy requires escalation. |
| WaitingForExternalEvent | Failed | `ExternalEventTimedOutTerminal` | External event wait times out and policy requires terminal failure. |
| Running | Retrying | `RetryScheduled` | A retryable failure occurs and retry budget remains. |
| Retrying | Running | `RetryStarted` | A retry attempt starts. |
| Running | Running | `RetryExhausted` | Retry budget is exhausted and the runtime must immediately escalate or fail terminally. |
| Retrying | Retrying | `RetryExhausted` | Retry budget is exhausted and the runtime must immediately escalate or fail terminally. |
| Running | Escalated | `EscalationTriggered` | Policy, ambiguity, unsafe action, retry exhaustion, timeout, or operator condition requires escalation. |
| Retrying | Escalated | `EscalationTriggered` | Retry-context escalation is triggered. |
| Escalated | Running | `EscalationResolved` | Human or policy resolution allows execution to continue. |
| Escalated | Running | `RunResumed` | v0 runtime state model resumes an escalated run after external resolution. |
| Escalated | Failed | `EscalationFailedTerminal` | Escalation resolves to terminal failure. |
| Running | Completed | `RunCompleted` | Execution completes successfully. |
| Running | Failed | `RunFailed` | Execution fails terminally. |
| Created | Canceled | `RunCanceled` | Run is canceled before validation completes. |
| Validated | Canceled | `RunCanceled` | Run is canceled before execution starts. |
| Running | Canceled | `RunCanceled` | Run is canceled during execution. |
| WaitingForApproval | Canceled | `RunCanceled` | Run is canceled while waiting for approval. |
| WaitingForExternalEvent | Canceled | `RunCanceled` | Run is canceled while waiting for an external event. |
| Retrying | Canceled | `RunCanceled` | Run is canceled while retrying. |
| Escalated | Canceled | `RunCanceled` | Run is canceled while escalated. |

## Invalid Transitions

The runtime must reject invalid transitions, including:

- Any transition from Completed, Failed, or Canceled.
- Created directly to Running.
- Created directly to Completed.
- Created directly to Escalated.
- Validated directly to Completed.
- WaitingForApproval directly to Completed.
- WaitingForApproval directly to Retrying.
- WaitingForExternalEvent directly to Completed.
- WaitingForExternalEvent directly to Retrying.
- Retrying directly to Completed.
- Retrying directly to WaitingForApproval.
- Escalated directly to Completed.
- Any transition that skips a required policy check.
- Any transition that would apply an event to the wrong run ID, workflow ID, workflow version, schema version, or spec content hash.
- Any transition from an unknown state.

Unknown requested transitions must fail closed and emit an auditable rejection or failure event when possible.

## Transition Metadata

Each transition event must capture:

- Event ID.
- Event name.
- Timestamp.
- Previous state.
- New state.
- Workflow ID.
- Workflow version.
- Schema version.
- Spec content hash.
- Run ID.
- Actor or system actor.
- Correlation ID where relevant.
- Causation ID where relevant.
- Policy context where relevant.
- Idempotency key where relevant.
- Trigger event ID where relevant.
- Attempt number where relevant.
- Retry budget where relevant.
- Approval request ID where relevant.
- External event key where relevant.
- Error category and summary where relevant.
- Input and output references where relevant.

Sensitive payloads must be redacted or referenced rather than stored inline by default.

## State Projection

The current run state may be stored as a projection for efficient reads. The projection must be derivable from, or reconcilable against, the append-only event log.

If projection and event history disagree, event history is authoritative unless an explicit recovery procedure proves otherwise.
