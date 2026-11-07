# Live Sandbox Continuation Admission Plan

Status: proposed after the native fixture trial; no live dispatch authorized.
Scope: one GitHub pull-request comment family, one interrupted run, one
independent replacement principal, and one disposable target.

## Evidence And Gap

The [native trial](../concepts/CLAUDE_DESKTOP_FIXTURE_HANDOFF_TRIAL.md) observed
the positive, repeat, and state-dependent fixture cases through Claude Desktop.
It did not observe a provider effect or an independent reconstruction of a live
run. The live-readiness review (`run-1794076118340894000-2`) confirmed:

- The public delegated CLI and Desktop bridge accept fixture mode only. A
  fixture grant is non-consuming, uses a fixture-only capability, and must
  remain incapable of opening or reserving a live attempt.
- The existing approval-backed GitHub live-sandbox composition requires a
  matching terminal run. It cannot authorize the active interrupted run from
  the [preregistration](../concepts/CROSS_HARNESS_AUTHORITATIVE_CONTINUATION_MILESTONE_PREREGISTRATION_REVIEW.md).
- Core already persists the operational opening, trusted time, attempt
  reservation, and dispatch admission projection. The opening returns an
  in-memory capability once; exact replay returns an observation, not another
  capability. The fixture snapshot check and ordinary dispatch reservation
  currently own separate SQLite transactions.
- Provider lookup and event-proof helpers exist, but the public recovery
  summary renders an input supplied by the caller. That is not recovery of
  the exact interrupted run or proof that GitHub accepted one effect.

Do not call the fixture validator and then dispatch based on its verdict.
Do not relax the terminal sandbox helper or recover an opening capability
from a checkpoint, a serialized bearer, or a replay response.

## Exact Live Authority

Implement a distinct live capability under the same scoped authority model.
Its immutable intent commits to the original and replacement principals,
issuer/delegation, run and bundle, step and attempted SideEffect, approval
decision and presentation proof, workflow and policy revisions, required
evidence/check facts, exact request and target, live operation kind,
idempotency key, execution window, and bounded validity. Check issuance
against current proof before creating the grant. Unknown authority, absence
of verifiable evidence, and fixture or dry-run credentials fail closed.

Only the kernel may bind the provider action. The operator supplies the
exact synthetic or later approved live target and injects scoped provider
authentication into the transport boundary. The replacement tool gets a
bounded request locator, not a provider credential, store path, arbitrary
body, or override. No hidden provider auth or fallback transport. If an
approved head predicate must hold at the effect, a separate read followed by
an unconditional POST is not atomic head enforcement; require a provider
guarantee for that predicate or do not dispatch under that condition.

The private Core binding now separates the exact live action from the grant.
The approval label commits to the run, step, immutable bundle, original actor,
effect, target, body, and idempotency key. It contains neither the body nor
provider credentials. The grant binder requires the live capability family,
an exact distinct replacement and issuer, and a matching granted decision
and fresh presentation for that action. It binds evidence, policy,
registration, and attempt commitments so a later verification can reject
changed facts. The binder does not resolve these facts from storage or
authenticate a principal; the trusted issuance and reservation paths must
do that before using it.

## Atomic Admission

Keep the opening and attempted intent durable before handoff. Introduce an
internal, live-only reservation entry point that derives the exact active
attempt from the durable opening and validates live grant use inside the
existing immediate SQLite reservation transaction. It is not a public
general-purpose way to rehydrate a capability.

Within the transaction:

1. Compare the authoritative event cursor and current run, bundle commitment,
   attempted record, required evidence/check state and exact opening.
2. Recheck the proof, distinct principal registration revision, grant scope,
   expiry, revocation, current trusted-time epoch, and execution window.
3. Check the exact operation and request commitment and the absence of any
   conflicting reservation. Bind the live grant use to the reservation and
   append its admission projection atomically. An error leaves no new grant
   use, reservation, or event.
4. Only the first committed admission may return a non-serializable right to
   call the injected provider once. Existing or uncertain reservations expose
   recovery posture only, even if they are exact replays. Keep the fixture
   validation and existing trusted-host entry points behaviorally unchanged.

No network call may run inside the transaction. Commit is the authority
linearization point; it is not evidence the request was sent or accepted.
If commit outcome is uncertain, reacquire only an observation of the durable
reservation. Never issue another dispatch right from that observation.

The first implementation adds the private in-transaction admission check to
the existing reservation path. Trusted time is sampled after acquiring the
write transaction. A test-only grant-use write proves denial and pre-commit
failure roll back both sides, and post-commit ambiguity and exact replay
return no dispatch right. Existing callers use their prior policy; this
change does not implement live issuance, bind a real live grant use, or
make recovery receipts available to an unverified caller.

The implementation run `run-1794078982477702000-2` covers the live action
and grant binder, including synthetic rejection of fixture grants, action
drift, changed evidence, changed policy, changed registration, stale
presentation, revocation, and expiry. It does not close delivery item 1.
The next change must connect distinct durable live grant issuance and
revocation to the atomic guard, consume the grant with the reservation, and
prove replay cannot regain an invocation right. No provider call or CLI
surface was added.

The private SQLite path in implementation run `run-1794080442376776000-2`
adds explicit V10-to-V11 opt-in for ready stores. The default and transfer
format remain V10; no command enables live storage. The live path verifies
the exact persisted opening, latest effective approval and presentation
before grant issuance and use. It records an immutable grant binding and
durable revocation. An attempt with a live grant cannot fall back to
ordinary reservation, including after revocation.

The grant use and reservation share an immediate transaction and the use
has a deferred foreign key to its exact attempt's reservation. A failure
before commit leaves neither record; a lost result after commit exposes no
dispatch right. A contended trial returns exactly one right. These are
synthetic Core tests with no provider call. The store still accepts evidence,
policy, and principal facts only from a trusted Core caller. The live
opening and resolver must authenticate the replacement, reconstruct those
facts and the attempted SideEffect from durable sources, and prevent
ordinary reservation before issuance. Do not expose this private path to
the harness until those boundaries are implemented.

## Post-Admission Recovery

After a reservation, an interrupted process is recovery-only. This includes
death before sending, a lost response, and provider acceptance before local
acknowledgement. Preserve an exact provider-observable reconciliation
commitment bound to the request and authorized target. Review the lookup
contract for that commitment before permitting an effect. A body-only match,
caller-supplied summary, absent result, or ambiguous match is not proof of
one accepted effect and must not authorize another POST.

After positive provider observation, bind the remote reference and observation
to the reservation and attempted record. Persist eligible SideEffect and
event proof before report disclosure. Repeated recovery can confirm an
existing proof but cannot duplicate it or reopen admission. Expose a bounded
public view separating procedure authority, invocation authority, execution
truth, and unresolved reconciliation. Do not expose payloads, authentication,
raw tool transcripts, or private database access to the reviewer.

## Delivery And Tests

1. Implement live authority issuance and atomic reservation in Core with an
   injected, non-network transport. Test exactly one dispatch right under
   two concurrent replacements; reject wrong principal, fixture grant, target
   or body drift, revoked or expired grant, stale proof or window, changed
   evidence, terminal run, and conflicting operation. Fault-inject before
   commit and after commit before return; assert no orphan writes and no
   replay dispatch right respectively.
2. Implement explicit dispatch and recovery against the same reservation.
   Inject provider acceptance followed by local acknowledgement loss, and
   prove recovery observes one exact effect without a second call to the write
   transport. Test absent, ambiguous, mismatched, unauthorized and unavailable
   observations; none produces successful proof or permits another dispatch.
   Verify stable public record reconstruction without local store access.
3. Add the narrow public CLI and Desktop tool for synthetic transport first.
   Verify redaction, rejection before transport, tool isolation, and the
   bounded result through the actual replacement harness. Collect recovery
   turns, lookups, latency, interruptions and token counts when available.
4. Only after implementation and focused security review, seek separate
   explicit approval for an exact maintainer-owned disposable target, scoped
   authentication, bounded execution window, and independent reviewer.
   Preserve interruption before dispatch and after provider acceptance, and
   reconcile without retrying the effect. Do not claim production readiness or
   broad cross-harness exactly-once behavior.
