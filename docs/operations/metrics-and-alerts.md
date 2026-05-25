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

The fixture-backed CLI examples do not yet persist adapter observability records as first-class runtime observability events. The local runtime sink records workflow, policy, approval, and skill signals; adapter-specific observability remains a contract-layer record until a future runtime adapter invocation path maps it into sinks.

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
