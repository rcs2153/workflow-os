# Production Current-Authority Source Boundary Model Review

## 1. Executive Verdict

Phase accepted; proceed to the private registered-source interface proof.

The implementation is domain-neutral, payload-free, deterministic, and
appropriately incapable of authenticating a source or conferring runtime
readiness. No blocker was found.

## 2. Scope Verification

The phase stayed within the approved model-only boundary.

It added source identity, registration and request commitments, snapshot
posture, completeness, consistency, freshness, failure vocabulary, serde,
redaction-safe Debug behavior, tests, and documentation.

It did not add a source service, trusted registry, concrete source, runtime
consumer, readiness API, target dereference, persistence, events, providers,
OpenShell integration, SideEffect execution, writes, schemas, CLI behavior,
hosted behavior, reasoning lineage, or release changes.

## 3. Model Assessment

The model is appropriately minimal and domain-neutral.

- Source, contract, snapshot, watermark, and optional generation identities
  are bounded.
- A registration commits normalized configuration posture without storing
  configuration or credentials.
- A request binds the registration, immutable execution binding,
  required-context contract, exact query set, fact families, sensitivity, and
  evaluation time.
- A snapshot binds the exact request, read window, returned families, bounded
  counts, completeness, consistency, freshness, and an external records
  commitment.
- Failure records contain bounded categories and retry posture without raw
  operational errors.

The public constructors are model constructors only. Their documentation
correctly states that caller-built registrations and snapshots are data, not
trusted authority.

## 4. Identity And Commitment Assessment

Identifiers are bounded, character restricted, secret-like text is rejected,
and Debug output redacts values.

Registration, request, and snapshot commitments are deterministic and include
their model version and all decision-relevant bounded fields. Canonical family
ordering prevents equivalent inputs from producing different commitments.
Substitution of a contract, registration, consistency posture, or committed
wire field fails closed.

The commitment format is not yet a persisted compatibility contract. A fixed
known-vector test should be added before a future persistence or cross-process
consumer phase relies on these hashes.

## 5. Completeness And Consistency Assessment

`CompleteForExactQuery` requires exact requested-family coverage.
Availability and governed-context-reference counts must equal the exact query
count. Zero capability grants remain representable for a complete query,
which avoids treating absence of authority as missing data.

Opaque watermarks intentionally support equality but not ordering. Optional
generation is the only ordered source-defined value, and the model does not
claim that every source contract may use it.

Atomic, stable-watermark, best-effort, and unknown consistency are
representable. Unknown consistency cannot be registered, and a snapshot must
match its registration. A future trusted registry must define the minimum
acceptable consistency per source contract; model representability must not be
mistaken for runtime acceptability.

## 6. Freshness Assessment

Freshness is deterministic and uses injected times.

The effective validity bound is the earlier of the source-supplied validity
bound and the Core-owned maximum observation age. Out-of-order read windows,
invalid source validity, future observations, stale observations, and
timestamp overflow fail closed or remain explicit bounded posture.

The model does not refresh data, choose clocks, retry, or make readiness
decisions.

## 7. Failure And Retry Assessment

The failure vocabulary covers unavailable, unsupported, incomplete, stale,
future-dated, concurrent-change, ambiguous, corrupt, registration mismatch,
query mismatch, transport, and internal failures.

Retry posture is explicit and payload-free. The model deliberately does not
enforce a global mapping between failure kind and retry posture. A future
source interface should own deterministic failure classification and bounded
retry policy rather than allowing arbitrary callers to choose operational
behavior.

## 8. Privacy And Serde Assessment

The model does not store source contents, target contents, credentials,
provider payloads, sandbox payloads, command output, environment values,
paths, endpoints, cursors, or unbounded errors.

Registration requires redaction. Debug output redacts identities,
commitments, snapshot tokens, watermarks, and timestamps. Custom
deserialization validates commitments and emits bounded errors without
echoing rejected values. Invalid enum values fail closed.

Serialization exposes bounded metadata and commitments. That shape remains
model data and must not be treated as proof of trusted registration.

## 9. Relationship To Runtime Authority

The model composes with immutable run and required-context foundations without
becoming a runtime consumer.

It cannot:

- authenticate or instantiate a source;
- prove a caller queried the registered implementation;
- dereference context or targets;
- validate source records against their commitment;
- confer capability authority or readiness;
- lower proportional-governance posture; or
- authorize an execution provider.

The next interface proof must preserve these boundaries and perform source
selection, query execution, and snapshot construction inside one Core-owned
call.

## 10. Test Quality Assessment

Focused tests cover valid construction and round trips, canonical ordering,
duplicate and unsupported families, redaction requirements, sensitivity
ceilings, contract and registration substitution, exact family and query
coverage, empty grant results, completeness vocabulary, atomic and
stable-watermark consistency, read-window ordering, source and Core freshness
bounds, stale and future-dated posture, wire tampering, invalid enums, failure
categories, payload absence, and Debug safety.

Existing workspace tests also passed, including proportional governance,
required context, immutable bundles, runtime events, provider-write
foundations, SideEffect models, work reports, and workflow catalog behavior.

Non-blocking test follow-ups:

- add fixed commitment vectors before persistence or cross-process use;
- test best-effort registration explicitly as representable but not ready; and
- test the future trusted-source interface against fabricated caller-built
  registrations and snapshots.

## 11. Documentation Review

The plan, roadmap, and phase report accurately state that the model is
implemented while no production source or runtime consumer exists.

They do not overclaim source authentication, readiness, dereference,
persistence, provider execution, OpenShell integration, SideEffect execution,
writes, schemas, hosted behavior, or reasoning lineage.

## 12. Blockers And Follow-Ups

Blockers: none.

Non-blocking follow-ups:

- keep trusted registration and source instantiation Core-owned;
- require one same-call source query and snapshot commitment boundary;
- define minimum accepted consistency per trusted source contract;
- verify per-query authority facts, not only aggregate counts, before
  readiness;
- define deterministic failure-kind and retry-policy mapping at the source
  interface;
- add commitment compatibility vectors before persistence; and
- keep OpenShell as a separate optional execution-provider boundary.

## 13. Recommended Next Phase

Implement the private registered-source interface proof with one in-memory
aggregate source.

The proof should accept one Core-owned trusted registration, execute one exact
request, return one coherent snapshot or bounded source failure, and remain
private and test-only where practical. It must not add runtime readiness,
target dereference, persistence, providers, OpenShell integration, SideEffect
execution, writes, schemas, CLI behavior, or hosted behavior.

## Governed Review Record

- workflow: `dg/review`
- run ID: `run-1785162263164821000-2`
- approval ID:
  `approval/run-1785162263164821000-2/review-scope-approved`
- approval presentation ID: `presentation/f0c5709dc343fb12`
- approval presentation content hash:
  `f0c5709dc343fb1270143da276d10222b9ecafc346c571f36e4d72a8cafe9490`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- review status: accepted and completed
- event summary: 39 events; one approval; zero retries; zero escalations;
  approval-presentation proof enforced with a matching event marker
- out-of-kernel work: the delegated maintainer inspected the implementation,
  tests, documentation, and validation evidence and authored this review; the
  kernel governed scope and approval but did not inspect code, edit files, run
  checks, or mutate git
