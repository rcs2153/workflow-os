# Policies

Workflow OS policies are runtime gates. They complement semantic validation, but they are not the same thing.

Validation checks whether a project is structurally and semantically safe enough to run. Runtime policy checks whether a specific actor, action, capability set, workflow, run, step, and skill context may proceed now.

## Core Concepts

The v0 policy model includes:

- capability
- action
- policy evaluation context
- policy decision
- policy violation
- conservative policy engine
- kill switch

Policy decisions include stable reason codes and violations. They are recorded as runtime audit events before meaningful actions.

## Conservative Defaults

The default policy engine:

- allows local deterministic skill execution for validated Level 1/2 workflows
- requires explicit approval policy for sensitive actions
- denies unknown actions
- denies unknown capabilities
- denies Level 3/4 execution by default
- denies `secret.read` by default
- denies `external.write` in v0
- denies adapter invocation in v0
- fails closed on missing actor, workflow, correlation ID, or capability context

## Boundary

v0 policy does not implement enterprise RBAC, identity provider integration, real secret providers, or external adapter authorization. Those systems must integrate later without weakening the default fail-closed behavior.
