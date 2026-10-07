# Trusted-Host Local Application Session Preparation Plan Review

## 1. Executive Verdict

**Plan accepted with binding clarifications; proceed to the one Core
session-preparation implementation slice only.**

The plan preserves the correct authority boundary. Preparation is a
read-only coherence check and one-shot capture operation, while consumed
`run` remains the only authority-consuming edge. This prevents preparation
from leaving admitted durable work behind and prevents a prepared value from
becoming a durable authorization claim.

The first implementation must keep the prepared-pair wrapper private, model
fresh and existing-window posture as explicit private variants, and project
the cancellation handle's broad failure before any pair is issued.

## 2. Scope Verification

The plan remained within architecture and implementation planning. It does
not authorize a local-host package, executable, production caller, discovery,
scheduling, signal handling, provider behavior, writes, schemas, SDK
behavior, hosted parity, or release changes.

## 3. Preparation Boundary Assessment

The crate-private helper is the smallest credible boundary. It may accept
private Core values and return private preparation errors because no
application value exists on failure. Once issuance succeeds, the public
session and handle must expose only the fixed application failure vocabulary.

The prepared-pair wrapper should remain crate-private in the first
implementation. Only its already-reviewed opaque session and cancellation
handle parts need to cross the feature-gated SPI. Making the wrapper public
would add an unnecessary compatibility and construction surface before a
real embedding package exists.

## 4. Read-Only And Time-Of-Use Assessment

The plan correctly rejects authority consumption, window opening, resume
consumption, capability allocation, event append, and executor entry during
preparation. Those actions belong to consumed `run`, where current state is
rehydrated and atomically revalidated.

Preparation may validate structural coherence and take a current read-only
snapshot of expected bindings. It must not describe that result as durable
authorization or imply that trusted time, current authority, or continuation
posture will remain valid. A prepared session is allowed to fail later when
`run` observes drift.

## 5. Fresh And Existing Posture Assessment

Fresh opening and existing-window continuation must be represented as
explicit private enum variants. One input with optional fresh and existing
fields would permit invalid combinations and move ambiguity into runtime
branches. Each variant should carry only the fields valid for that posture,
while shared exact bindings remain outside the enum.

The helper must reject absent, substituted, or multiple matching windows. It
must not perform general project or queue discovery.

## 6. Binding And Authority Assessment

The required binding set is sufficient: backend object identity, workflow,
run, step, actor, immutable run bundle, window posture, invocation commitment,
skill input, executor binding, opening persistence identity, and current
authority context where fresh opening is intended.

Locators, capabilities, continuation dispositions, trusted-time authority,
and identity-provider construction remain private to Core. Application code
must not rebuild the session from copied identifiers or inject wake or
redispatch identity providers.

## 7. Cancellation And Failure Assessment

Bounded cancellation failure is a mandatory part of the implementation
slice, not a follow-up. The current public handle still returns
`WorkflowOsError`; issuing it from production preparation before projection
would recreate the already-fixed application error leak on a second method.

Run and cancellation failures may share
`TrustedHostLocalApplicationFailure` if the implementation can preserve a
clear fixed code for every path. A separate smaller enum is also acceptable,
but it must remain fixed, payload-free, non-serializable, and non-leaking.

## 8. Identity Ownership Assessment

Core ownership of production redispatch and wake identity generation is
correct. The implementation may refactor private lifetimes or closure
capture, but must not export provider traits, random material, or identity
constructors. Deterministic identity injection belongs only in private test
paths.

## 9. Prepared Pair And Privacy Assessment

The pair must be one-shot, non-cloneable, non-serde, Debug-redacted, and
bound to one backend instance and exact invocation context. Dropping it must
produce no durable state, event, workflow claim, or executor action.

The input and pair must not retain or format raw provider payloads, command
output, credentials, source text, or broad Core errors. Feature visibility is
packaging only and must not become authority.

## 10. Test Plan Assessment

The planned tests cover the required security boundary. The implementation
must include direct before/after state assertions proving that prepare and
drop do not change rows, events, authority, directives, attempts, or executor
entry. It must also prove:

- fresh and existing variants prepare exactly once;
- stale state after preparation fails at `run`;
- backend and every identity/binding substitution fail closed;
- ambiguous or missing continuity posture fails closed;
- cancellation before `run` remains zero-entry;
- run and cancellation errors omit secret-like internal values;
- external callers cannot construct, clone, serialize, or inject identity
  providers; and
- default workspace, CLI, hosted, filesystem, PostgreSQL, provider, and SDK
  behavior remain unchanged.

## 11. Blockers

None for the one Core implementation slice, provided that bounded
cancellation failure is implemented before issuance and the fresh/existing
input is modeled without ambiguous optional fields.

Operational application adoption remains blocked. No production source or
embedding process currently owns all required inputs and lifecycle duties.

## 12. Non-Blocking Follow-Ups

- Add an external-consumer feature fixture before a separate package depends
  on the SPI.
- Keep the prepared-pair wrapper private until an embedding package proves a
  public need.
- Select a real current-authority source only in a later caller-planning
  phase.
- Preserve explicit documentation that successful preparation does not
  guarantee successful execution.

## 13. Recommended Next Phase

Implement one Core-only session-preparation slice:

1. project cancellation failure to bounded application vocabulary;
2. add explicit private fresh/existing preparation variants;
3. add a private read-only preparation helper and opaque private pair;
4. capture a one-shot runner with Core-owned identity generation and
   authoritative `run` revalidation; and
5. add direct state-invariance, stale-state, substitution, cancellation,
   privacy, and feature-isolation tests.

Do not add a host package, executable, operational caller, discovery,
provider behavior, writes, schemas, SDK behavior, hosted parity, or release
changes.

## 14. Validation

- `npm run check:docs`: passed;
- `git diff --check`: passed.

## 15. Governed Review Record

- workflow: `dg/review`
- run: `run-1791381579158457000-2`
- approval:
  `approval/run-1791381579158457000-2/review-scope-approved`
- presentation: `presentation/abd7e0d4919e616c`
- presentation hash:
  `abd7e0d4919e616cee441f9a070380e0cdde082c7dddef72457ebc35039b1247`
- approval outcome: granted by delegated maintainer through persisted
  presentation proof
- approved boundary: focused preparation-plan maintainer/security review only
- phase status: completed
- event summary: 39 events, one approval, zero retries, and zero escalations;
  presentation proof enforced with one persisted record and matching event
  marker
- validation summary: docs and diff hygiene passed
- out-of-kernel work: source and plan inspection, review authoring,
  validation, and later git or pull-request work
