# Continuation Context Replacement-Executor Proof Hardening Review

## Decision

Accepted at the private conformance boundary. The replacement-executor proof
now preserves typed control disposition, derives stable work identity only from
authoritative private source records selected by Core, rejects completed-work
replay by identity, and binds a payload-free fresh-authority request to the
exact accepted aggregate, decision, recipient, and operation after proof
validation.

This is an owner-conducted focused maintainer/security review, supplemented by
a second repository-local agent review. It is not independent external
security assurance.

## Governed Review

- workflow: `dg/review`
- run: `run-1793720349400619000-2`
- approval: `approval/run-1793720349400619000-2/review-scope-approved`
- presentation: `presentation/2366b70cfe6e593c`
- approval outcome: granted by the delegated maintainer for the bounded
  hardening re-review

## Prior Blockers Closed

### Control disposition remains typed

Current pause, stop, and resume intent remain distinct through aggregate
composition, exact delivery, and replacement orientation. Pause and stop never
invoke fresh authority. A ready orientation without one current resume fails
closed, and a pause or stop cannot carry lawful-next work identity.

### Fresh authority is exact and last

The private authority resolver receives a payload-free request bound to the
scope, aggregate proof, decision, replacement recipient, and exact lawful-next
operation identity. Complete proof construction and validation precede the
resolver call. The resolver returns no authority payload, and the replacement
proof remains explicitly non-authorizing.

### Completed work uses authoritative stable identity

Completed identity is carried inside the private authoritative WorkReport
source. Lawful-next identity is carried by the private accepted-intent record
wrapper. No independent top-level identity assertion remains. Completed
identity must bind the exact completed-obligation claim; lawful-next identity
is bound to the exact current resume record.

Core collects identity candidates with their exact source identity and projects
them only when that source is selected by the continuation-context decision.
Stale, unauthorized, invalid-integrity, and over-budget WorkReport sources
cannot project completed-work identity. Rephrasing a lawful-next description
does not bypass an already-completed stable identity.

### Recipient and aggregate substitution remain closed

Original and replacement recipients must be distinct. The accepted aggregate,
typed gaps, control disposition, selected work identities, delivery, receipt,
and understanding commitments must match before fresh authority is consulted.

## Validation

- focused continuation collection tests: passed, 53 tests;
- strict `workflow-core` Clippy with all targets and features: passed;
- formatting check: passed;
- full workspace tests with all features: passed;
- warnings-denied workspace rustdoc: passed;
- documentation checks: passed; and
- diff hygiene check: passed.

## Residual Boundary

The accepted work-identity producers are private conformance fixtures. Existing
production-shaped collectors deliberately emit no identity. This review does
not accept a runtime consumer, host compaction signal, model comprehension,
process replacement, persistent identity producer, automatic WorkReport,
public API or schema, provider operation, hosted behavior, nested harness, agent
memory system, or Salsa.

## Next Authorized Work

The authoritative post-compaction understanding P0 lane is complete at its
private replacement-executor conformance boundary. The next queued work is the
bounded P1 onboarding correction for generated downstream agent guidance,
followed by provider-fixture-aware `next-action` routing. Neither phase may
broaden provider mutations or claim operational compaction-safe continuation.
