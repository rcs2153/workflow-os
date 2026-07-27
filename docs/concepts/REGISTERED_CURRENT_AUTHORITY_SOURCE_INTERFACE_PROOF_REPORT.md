# Registered Current-Authority Source Interface Proof Report

## 1. Executive Summary

Workflow OS now has a private Core-owned registered current-authority source
interface proof.

One in-memory aggregate source owns a complete canonical inventory of
capability grants, capability availability, and governed context references.
For one exact immutable execution binding and required-context contract, it
returns either one coherent payload-free source snapshot commitment or one
bounded source failure.

The proof does not expose a public source trait, confer readiness, dereference
targets, integrate with the executor, or execute work.

## 2. Scope Completed

- Added one crate-private registered in-memory aggregate source.
- Added a Core-owned constructor that creates the accepted source registration
  internally rather than accepting a caller-built registration.
- Bound the source to one canonical complete three-family inventory.
- Derived exact requests from the immutable execution binding and
  required-context contract.
- Selected all matching in-scope grant candidates.
- Required one exact availability observation per query.
- Required one exact governed context reference per contract target.
- Returned one complete coherent payload-free snapshot commitment.
- Returned bounded incomplete, stale, and future-dated failures.
- Added deterministic canonicalization, duplicate rejection, and safe Debug.
- Added focused proof tests.

## 3. Scope Explicitly Not Completed

This phase does not add a public source trait, public registration service,
persistent registry, production filesystem or network source, runtime
configuration, executor consumer, current-authority readiness, target
dereference, context payload access, source records in public output,
persistence, events, audit projection, receipts, artifacts, providers,
OpenShell, sandbox execution, SideEffects, writes, schemas, SDKs, CLI behavior,
UI, examples, hosted behavior, reasoning lineage, or release changes.

## 4. Interface Summary

The proof contains three crate-private boundaries:

- a registration input controlled by Core;
- an exact read input containing the immutable execution binding,
  required-context contract, and injected evaluation time; and
- a read outcome containing either `CurrentAuthoritySourceSnapshot` or
  `CurrentAuthoritySourceFailure`.

The source always registers as a local atomic aggregate supporting:

- capability grants;
- capability availability; and
- governed context references.

No source interface type is exported from `workflow-core`.

## 5. Trust Boundary

The private source constructor creates
`CurrentAuthoritySourceRegistration` internally from Core-owned posture. A
caller cannot pass a deserialized or independently constructed public
registration into this trusted interface.

Public registrations and snapshots therefore remain data. Only construction
through this private interface establishes the proof's source provenance.
This is still a local in-memory proof, not a production trust registry.

## 6. Exact Request And Selection

Each read constructs `CurrentAuthoritySourceRequest` from:

- the source's internal registration;
- the exact immutable execution binding;
- the exact required-context contract;
- all three supported fact families; and
- an injected evaluation timestamp.

The source then:

- derives the canonical exact query set;
- retains every grant candidate matching capability, resource, actor,
  workflow, run, step, and harness scope;
- selects exactly one availability observation for each query; and
- selects exactly one governed context reference for each contract target.

Zero matching grants is a valid complete result. Missing availability or
context-reference coverage is a source failure, not a negative authority fact.

## 7. Snapshot And Failure Behavior

A successful read commits:

- the exact request;
- one atomic read window;
- canonical selected records;
- bounded per-family counts;
- opaque snapshot and watermark identities;
- optional source generation;
- source and Core freshness bounds; and
- one aggregate snapshot commitment.

Stale or future-dated observations return bounded failures. Missing exact
records return an incomplete failure. The proof does not retry and does not
convert any failure into permission.

## 8. Privacy And Redaction

The source stores only validated grants, availability posture, payload-free
context references, timestamps, bounded identity, and commitments. It does not
store target contents, provider payloads, command output, source files,
credentials, environment values, raw configuration, endpoints, or unbounded
errors.

Debug output redacts source identity, registration posture, timestamps,
inventory commitments, and target identities. Failure values retain only
registration and request commitments plus bounded kind and retry posture.

## 9. Test Coverage

Focused tests cover:

- Core-owned registration producing one complete exact snapshot;
- deterministic snapshot commitments across input order;
- exact grant, availability, and context-reference coverage;
- bounded incomplete failure when an exact record is missing;
- stale and future-dated failure;
- duplicate inventory rejection;
- stable non-leaking errors; and
- redaction-safe source Debug output.

Rust privacy enforces that downstream callers cannot construct or invoke the
private source interface. Public source-model tests continue to prove that
caller-built registrations and snapshots do not expose readiness APIs.

## 10. Validation Commands And Results

- focused registered-source unit tests: passed, 6 tests.
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed before the governed phase close.

## 11. Remaining Known Limitations

- The source is private and in-memory only.
- No persistent or external source registration exists.
- No source record leaves the private proof.
- No concurrent-change simulation or retry executor exists.
- No prerequisite decision fact families exist.
- No registered source is composed with the private same-call resolver.
- No one-time-use or replay prevention exists for a future source-backed
  assessment.
- No runtime consumer can use this proof to confer readiness.
- Proportional governance cannot yet select quiet success from this source.
- OpenShell remains a separate future execution-provider concern.

## 12. Recommended Next Phase

Focused maintainer review accepts the private registered-source interface proof
in
[Registered Current-Authority Source Interface Proof Review](REGISTERED_CURRENT_AUTHORITY_SOURCE_INTERFACE_PROOF_REVIEW.md).

Compose this registered source with the existing private same-call resolver in
a separately governed phase. Do not add a public source trait, runtime
consumer, dereference, provider, OpenShell adapter, SideEffect execution, or
writes.

## 13. Governed Phase Record

- workflow: `dg/implement`
- run ID: `run-1785162980824812000-2`
- approval ID:
  `approval/run-1785162980824812000-2/implementation-approved`
- approval presentation ID: `presentation/83826ef377285562`
- approval presentation content hash:
  `83826ef3772855624e25793e96c1d9dae44bf32c32f84d98ccbcbc1eaed55f61`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted implementation handoff was presented
- phase status: `Completed`
- event summary: 39 events, 1 approval, 0 retries, 0 escalations
- approval-presentation enforcement: proof enforced with one persisted
  presentation record and an approval event marker
- out-of-kernel work: the delegated maintainer inspected architecture,
  implemented and tested the private proof, updated documentation, and ran
  validation; the kernel governed scope and approval but did not inspect code,
  edit files, execute checks, or mutate git
