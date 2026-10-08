# Explicit Zero Required-Context Contract Implementation Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups; resume the trusted-host local
application admission vertical slice.**

The implementation closes the prerequisite semantic gap without treating
absence, omission, or an arbitrary empty vector as authority. Established
non-empty contracts retain their exact hash and wire shape. Explicit zero is a
separate, marked, commitment-bound posture issued only through the private
canonical docs-check profile path.

## 2. Scope Verification

The phase stayed within its approved prerequisite-only boundary. It added no
trusted-host admission wiring, local-host application binary, public empty
contract API, provider behavior, CLI or SDK surface, workflow schema, example,
hosted or distributed behavior, OpenShell integration, nested harness runtime,
or release posture change.

## 3. Contract And Compatibility Assessment

`RequiredContextContractBinding::new` remains public and continues to reject an
empty requirement set. Non-empty contracts retain the established
`required-context-contract-v1` hash path and serialize as the original four
fields. The golden regression fixture remains:

```text
f7f7aedb81d2f26171bcd352dd1523cff4ad56a5c06f61703b8263cc9e33cfdf
```

Explicit zero uses the distinct
`required-context-contract-explicit-none-v1` hash domain and requires the
`requirement_posture: explicit_none` marker. Missing, unknown, or inconsistent
markers fail closed. This satisfies the planning review's mandatory
compatibility condition.

## 4. Construction Boundary Assessment

The explicit-zero constructor is crate-private and requires a private,
unconstructable-outside-module authorization value. The canonical resolved
docs-check profile issues that value only after its profile ID and fixed command
contract match the reviewed docs-check definition.

The authorization is construction proof, not runtime authority. It does not
cross the public API and does not itself permit execution, approval, policy
bypass, dispatch, or provider access.

## 5. Consumption Assessment

Explicit-zero contracts accept exactly zero projections and produce the
existing `Satisfied` posture with zero satisfactions and zero gaps. Supplying
any projection fails with a stable bounded error. Ordinary contracts retain
their prior non-empty projection behavior.

No evidence reference, context target, satisfaction record, or authority fact
is fabricated to represent zero.

## 6. Current-Authority Assessment

The current-authority query set and source request carry an explicit marker
only for zero posture and use separate commitment domains. Their established
non-zero serialization and commitment domains remain unchanged.

The private registered-source path binds the exact execution binding, contract
identity and hash, docs-check command-contract fingerprint, and declarations
for complete empty grant, availability, and governed-context inventories. It
can resolve the exact binding as `Ready` and `Satisfied` without inventing a
fact. Existing registration, snapshot, freshness, sensitivity, inventory, and
resolution commitments continue to govern the surrounding source lifecycle.

Policy, approval, evidence, checks, operational opening, and dispatch remain
independent requirements.

## 7. Serde And Failure Assessment

Valid explicit-zero contracts, query sets, and source requests round-trip with
their marker. Empty unmarked forms, unknown markers, marker/non-empty
conflicts, hash mismatches, and posture mismatches fail closed.

Errors are stable and payload-free. Debug output exposes posture and counts but
redacts identifiers, hashes, timestamps, targets, and payload-bearing values.
No fallback converts malformed input into explicit zero.

## 8. Test Quality Assessment

The tests directly cover:

- the legacy non-empty hash and four-field serialized shape;
- public empty-constructor rejection;
- explicit-zero round-trip and marker enforcement;
- unknown and non-empty marker combinations;
- zero-projection satisfaction and supplied-projection rejection;
- query-set marker and distinct identity behavior;
- canonical docs-check issuance without executing a process;
- exact complete-empty registered-source resolution; and
- the existing current-authority, executor, SQLite, provider, hosted, and
  local-host regression surfaces through the full workspace suite.

The coverage is sufficient for this private prerequisite. No shallow test
creates a fake evidence or authority record to make the path pass.

## 9. Privacy And Security Assessment

The model stores no command output, source content, environment value,
credential, token, private policy detail, or provider payload. The explicit
zero marker is bounded vocabulary. The private issuer and registered source
do not expose caller-authored inventory or a public path to claim no authority.

The change therefore narrows ambiguity without widening execution authority.

## 10. Blockers

None.

## 11. Non-Blocking Follow-Ups

- When trusted-host admission consumes this path, assert the expected canonical
  harness contract ID and version at the admission boundary rather than relying
  only on crate privacy and execution-binding equality.
- Add a direct substituted-profile rejection test when a second profile with a
  superficially similar command shape exists; the current closed enum and exact
  docs-check command comparison already fail closed.
- Keep issuance private if future profiles need explicit zero. Each additional
  issuer requires separate review.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Governed Review Record

- workflow: `dg/review`
- run: `run-1791403919699552000-2`
- approval: `approval/run-1791403919699552000-2/review-scope-approved`
- presentation: `presentation/980a14698d196772`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused maintainer and security review only
- out-of-kernel work: source and test inspection, review authoring, validation,
  and later git or pull-request operations

## 14. Recommended Next Phase

Resume the previously blocked trusted-host local application admission vertical
slice. Wire only the accepted canonical docs-check profile through the existing
private explicit-zero contract and registered current-authority path. Do not
broaden profile eligibility, provider behavior, public configuration, or
release posture.
