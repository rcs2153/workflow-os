# Escalation Runbook

This runbook describes the local v0 escalation shape. It is not an external notification procedure.

## When A Run Escalates

A local run escalates when:

- a retry-enabled step fails
- bounded retry attempts are exhausted
- the step declares an escalation policy

## Operator Review

Inspect the `EscalationTriggered` event and confirm:

- run ID
- workflow ID and version
- spec content hash
- step ID
- skill ID
- attempts
- last error
- failure class
- suggested next action
- escalation contact, if present

## Required Safety Posture

Do not manually continue external side effects from escalation context. v0 has no external adapters and no operator resume API for escalated runs.

Any future escalation resolution must append explicit runtime events and preserve audit history.
