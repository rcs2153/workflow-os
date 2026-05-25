# Observability

Workflow OS emits observability signals from runtime events. The goal is a vendor-neutral foundation that future local tools, CLIs, exporters, and production backends can consume without coupling the core to one monitoring system.

## v0 Signals

The v0 runtime emits local observability events for:

- workflow run started, completed, failed, and canceled
- skill invocation succeeded and failed
- retry started and retry exhausted
- escalation triggered
- approval requested, granted, and denied
- policy allowed, denied, and approval-required decisions

The model also defines event kinds for:

- workflow latency
- skill invocation latency
- approval wait duration
- stuck workflow detection
- backend health check result
- runtime error count

Some latency and background detection signals are model hooks in v0. They are represented in the event model but are not a background scheduler or distributed monitoring system.

## Adapter Telemetry In Phase 2

Phase 2 read-only adapters produce contract-level adapter telemetry through `AdapterObservabilityRecord` values. These records capture adapter action, mode, status, classified error where relevant, latency, correlation ID, and non-secret attributes.

The controlled fixture-backed GitHub, Jira, and CI examples now map adapter observability records into `AdapterRuntimeObservabilityRecord` values through the local executor. This provides scoped runtime-visible adapter telemetry. The local filesystem backend persists those records by run, and `workflow-os inspect` reports the number of mapped adapter observability records.

This is local, read-only, fixture-scoped observability. It is not production metrics export, OpenTelemetry integration, or generic runtime adapter execution.

## Correlation

Observability events carry correlation IDs where the source runtime event has one. Operators should use correlation IDs to connect validation output, policy decisions, audit records, local logs, and runtime events for a single user-initiated action.

## Non-Goals

v0 does not implement OpenTelemetry, Prometheus, Datadog, SIEM export, or hosted metrics. Those integrations must be isolated behind observability sinks in future work.
