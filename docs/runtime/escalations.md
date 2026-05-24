# Escalations

Escalation is the runtime state for work that cannot safely continue automatically.

## Local Runtime Scope

The v0 local executor escalates when a retry-enabled step exhausts its bounded retry attempts and declares an escalation policy.

Escalation is local state only. It does not send notifications, page operators, create tickets, or call external systems.

## Escalation Event

`EscalationTriggered` records:

- escalation ID
- workflow run ID
- step ID
- skill ID
- attempts made
- last error
- failure class
- suggested next action
- reason
- optional escalation contact

The run enters `Escalated`. Future operator resolution is not implemented yet.

## Failure Without Escalation

If retry exhaustion occurs without an escalation policy, the v0 local executor emits `RunFailed`. This is allowed only for workflows that explicitly use terminal failure behavior.

## Non-Goals

v0 escalation does not implement:

- external notifications
- issue creation
- paging systems
- human resolution APIs
- automatic escalation timers

Those features require future adapter and policy work.
