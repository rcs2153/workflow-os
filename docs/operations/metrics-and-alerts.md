# Metrics And Alerts

Workflow OS v0 exposes observability signals through `ObservabilitySink`.

## Local Signals

`LocalObservabilitySink` records in-process events for development and tests. It can be used to assert that runtime paths emit the expected signals, but it is not a production metrics backend.

Current emitted signals include:

- workflow run started, completed, failed, and canceled
- skill invocation succeeded and failed
- retry started and retry exhausted
- escalation triggered
- approval requested, granted, and denied
- policy allowed, denied, and approval-required decisions

## Adapter Telemetry

Phase 2 read-only adapters produce **contract-level adapter telemetry** as `AdapterObservabilityRecord` values. These records include adapter status, latency, correlation ID, operation mode, and classified errors where relevant.

For the controlled fixture-backed GitHub, Jira, and CI examples, the local executor maps adapter observability records into `AdapterRuntimeObservabilityRecord` values as scoped runtime-visible adapter telemetry, and the local filesystem backend persists them by run. `workflow-os inspect` reports mapped adapter observability record counts.

This mapping is local and preview-scoped. It is not production metric export, OpenTelemetry integration, or generic live adapter execution.

## Alert Candidates

Production operators should eventually alert on:

- repeated workflow failures
- retry exhaustion
- escalation count
- long approval wait duration
- stuck workflow detection
- backend health check failures
- policy denial spikes
- runtime error count

## v0 Limitations

v0 does not include a background scheduler for stuck workflow detection, active timeout scanning, external metric export, or vendor-specific observability. Those features must be implemented behind the sink interfaces without changing the core runtime contract.
