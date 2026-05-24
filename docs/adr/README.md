# Architecture Decision Records

Architecture decision records document decisions that affect Workflow OS product boundaries, public contracts, runtime invariants, security posture, or long-term maintainability.

## When An ADR Is Required

Create or update an ADR for changes that:

- Change the Workflow OS Core product boundary.
- Add or change public schemas.
- Add or change runtime state semantics.
- Add or change policy, audit, or observability invariants.
- Introduce unsafe Rust.
- Add a production integration or adapter.
- Break backward compatibility.
- Add substantial dependencies or toolchain requirements.

## ADR Format

Each ADR should include:

- Title.
- Status.
- Context.
- Decision.
- Consequences.
- Alternatives considered, when useful.

Accepted ADRs are binding until superseded by a later ADR.
