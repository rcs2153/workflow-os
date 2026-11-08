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

Implementation run `run-1794082035335242000-2` reserves an action-derived
live opening and window identity in the existing V11 store. Ordinary opening
rejects that identity. The private live path checks the exact persisted
approval and presentation under the opening transaction, uses a bounded
single-attempt window, and reserves the window identity for live grant
admission before any grant is issued. Ordinary dispatch rejects this window
after restart, including before grant issuance. Grant issuance and use
require the matching live opening projection. No schema version, public
entry point, or provider call was added. Authentication and durable
reconstruction of evidence, policy, and the attempted record remain open.
The V11 readiness checksum now commits to the opening protocol as well as
the unchanged physical schema. Earlier V11 dispatch writers reject this
checksum in their transaction; the new writer also rejects earlier
experimental V11 stores. There is no implicit upgrade or change to V10.

Implementation run `run-1794083121561276000-2` adds distinct private
live principal registrations. Issuance binds the current registration
from SQLite rather than a caller-provided commitment. Reservation resolves
the principal and harness from the persisted grant, verifies possession
against the live verifier, and checks registration and grant status under
the same write lock before replay or consumption. Revocation survives
restart and the same principal ID cannot be re-registered. The fixture
registry cannot issue live grants. Raw credentials are not stored.
The opt-in V11 manifest and readiness checksum cover the new tables and
registration binding; earlier experimental V11 stores remain unsupported.
This is not a public authentication path. The durable attempted record,
policy, and evidence resolver is still required.

Implementation run `run-1794084480503610000-2` resolves the exact attempted
SideEffect, linked latest adapter policy decision, and passing check events
under the SQLite issuance and consumption transactions. The record must
cite the exact opening and approval; the check IDs must match the grant
requirements and resolve to unsuperseded results for the run's bundle with
matching successful invocation events. Their commitments are derived from
durable records, not accepted from the caller. Missing, changed, denied,
failed, or unsupported referenced facts fail closed.

The allowed write decision in the test is synthetic. The default V0 policy
still denies external writes. Before any live route is connected, Core must
verify immutable declarations and required check assurance and produce a
separately scoped sandbox policy decision. This change does not enable
provider execution or public commands.

Implementation run `run-1794085918903339000-2` reads the exact validated
bundle under both SQLite admission transactions. It matches the run
identity, bundle root, workflow content, and approval's resolved execution
context, then resolves the canonical declaration sets through the selected
step. Missing, legacy, or changed bundles are denied. A required declaration
is denied until the store can prove its attestation; a passing result and
invocation event alone cannot satisfy required assurance. The bundle's
sensitivity also contributes to the grant ceiling. No checks run in the
transaction and no route, schema, or policy default changed.

Implementation run `run-1794087216226519000-2` adds storage for an explicitly
verifier-backed projection in the existing local create-only assurance store.
It does not upgrade the current selected validation route, whose injected
runner can supply process results and whose durable posture stays unattested.
Reopening the store validates the proof's integrity and exact projection
binding. Ordinary deserialization still rejects accepted-proof assertions.
Existing unattested records retain their format; legacy accepted references
without verifier proof fail closed. The local record trusts the storage owner
and is not external attestation or permission to reuse a prior check.
SQLite live admission still denies required checks until it can resolve the
same proof and evaluate its declared constraints at time of use in its own
transaction.

Implementation run `run-1794089581194127000-2` adds opt-in SQLite proof storage
and resolves each required declaration against the exact referenced result
event, successful invocation, and verifier record inside issuance and
reservation. It commits that proof to the grant, checks the immutable command
and requirement, and rechecks the bounded maximum age using trusted time.
`NoReuse` remains ineligible for a persisted observation. Ordinary state
keeps its current schema; existing opt-in live stores can upgrade in place
without synthesizing proofs. The private writer has synthetic coverage but is
not yet wired to a trusted production producer. This does not make the
existing local runner authoritative, authorize a sandbox launch, or enable
provider writes.

Implementation run `run-1794091942389997000-2` adds a private native proof
producer for one exact required check with bounded freshness. Only Core's
native process-runner constructors can enter this path. It compares the
resolved process request with the reviewed execution fingerprint before launch
and includes the verifier proof only after a passing result. The injected
runner path remains unattested. A real disposable subprocess and substitution
tests verify this boundary. The fingerprint identifies the resolved request
and pinned executable content; it is not a process sandbox or proof that the
host, interpreter, or workspace was immutable. The producer is not yet
connected to the supervisor's transactional result projection or to any
public or live dispatch route.

Implementation run `run-1794092896536764000-2` connects the native producer
to the existing supervised local-check attempt in opt-in V12 stores. A
required bounded check must use the native runner. The producer reuses the
execution fingerprint bound to the attempt and derives its result ID from
that attempt. SQLite writes the accepted proof with the exact result event
and outcome in one transaction. The outcome request binds the proof and
exact operation replay verifies the persisted row. A failure after proof
insertion rolls back the proof and result together, but not the earlier
dispatch reservation. Ordinary stores and checks without required proof
keep their existing behavior. No sandbox dispatch, external action, or
public schema changes are enabled here.

Implementation run `run-1794094353607782000-2` replaces the generic adapter
allow check with an exact private sandbox policy decision. The trusted
operator supplies one provisioned target, revision, and validity bound.
The policy event commits to the comment action, approval proof, replacement,
requirements, and opening window without carrying the comment text or
credentials. Issuance and reservation both require the latest correlated
decision to match, after the approval event. A generic allow, later denial,
or changed provisioning revision cannot admit. V12 now requires an explicit
protocol checksum upgrade before operational writes; no physical schema or
ordinary policy changes. Tests provision synthetic targets only. A trusted
production resolver and provider route remain unconnected.

Implementation run `run-1794095556237154000-2` adds a private opt-in V13
target registry. The trusted operator can register one exact PR target with
an operator, revision and expiry, and revoke it without permitting
re-registration of the same target. Opening, grant issuance, and reservation
resolve the current registration under their SQLite transaction, not from a
caller-provided provisioning object. Core records the scoped policy event
only after matching durable approval and immutable bundle identity. Exact
recording replay returns the same event; a later conflicting decision
cannot be overwritten. Upgrade preserves history and does not infer target
registrations from earlier policy events. Tests use disposable local stores
and synthetic targets; no real target or provider route is enabled.

At reservation, the replacement supplies a grant ID, attempted-effect
locator, exact proposed comment bytes, and principal proof. Core resolves the
grant scope, approval decision and presentation, immutable run identity,
attempted effect, and registered target in the reservation transaction.
Proposed bytes must reproduce the original action commitment; a missing
effect or changed body leaves the grant and reservation untouched. Sensitivity
comes from the persisted presentation, bundle, and effect, not the caller.
This admission path does not perform a dispatch.

## Post-Admission Recovery

After a reservation, an interrupted process is recovery-only. This includes
death before sending, a lost response, and provider acceptance before local
acknowledgement. Preserve an exact provider-observable reconciliation
commitment bound to the request and authorized target. Review the lookup
contract for that commitment before permitting an effect. A body-only match,
caller-supplied summary, absent result, or ambiguous match is not proof of
one accepted effect and must not authorize another POST.

The private SQLite path now exercises this boundary through injected,
non-network callbacks. Only fresh reservation can invoke send. Exact replay
and ambiguous commit return recovery-only. Recovery validates the stored
reservation projection and grant use before recomputing a marker bound to
the action and reservation. A positive synthetic observation requires an
exact target, marker, action commitment and provider reference. The callback
is trusted in this test; a real adapter must independently verify the author
and exact accepted payload, not infer authorship from a copyable marker.
Synthetic recovery now binds the exact attempted SideEffect into the
reservation commitment. A verified observation can atomically complete that
record and append one proof event referencing its admission. Replay verifies
the same completed record and event; drift, conflicting observations, or
older reservations without the attempted-record commitment cannot create
proof. A separate synthetic-recovery Desktop bridge mode exposes only the
bounded read-only inspection command through operator-allowlisted cases.
It does not expose a dispatch tool or a production lookup adapter. See the
[Desktop fixture guide](../user-guide/claude-desktop-fixture-handoff.md) for
snapshot export and trial setup.

Local reviewers can inspect an existing synthetic reservation:

```sh
workflow-os --state-dir <state-dir> provider github-pr-comment inspect-synthetic-recovery \
  --run-id <id> --operation-id <id> --side-effect-id <id>
```

The command opens the existing continuation SQLite database read-only. Its
bounded JSON distinguishes verified historical admission from unresolved
reconciliation or verified completion. It never grants dispatch authority,
calls a provider, or returns comment bytes, markers, or provider references.

After positive provider observation, bind the remote reference and observation
to the reservation and attempted record. Persist eligible SideEffect and
event proof before report disclosure. Repeated recovery can confirm an
existing proof but cannot duplicate it or reopen admission. Expose a bounded
public view separating procedure authority, invocation authority, execution
truth, and unresolved reconciliation. Do not expose payloads, authentication,
raw tool transcripts, or private database access to the reviewer.

## Focused Maintainer Review: Live Lookup Not Admitted

The synthetic dispatch, atomic proof, read-only CLI, and MCP bridge are
implemented. [PR #818](https://github.com/rcs2153/workflow-os/pull/818) made
the real CLI bridge round trip and unchanged snapshot hashes mandatory in
clean-checkout CI. That is headless synthetic evidence. Native Desktop
recovery inspection is pending separate operator consent; the earlier
native delegated fixture validation is not evidence for this recovery mode.
This is a maintainer review, not independent assurance or live approval.

### Why The Existing Lookup Cannot Establish Completion

- [The legacy lookup classifier](../../crates/workflow-core/src/provider_write.rs)
  matches the exact target and either the provider reference or the managed
  marker. It has no raw body commitment or verified author in the observation.
  A copied marker or a known ID on an edited comment must not qualify.
- Its absent-result next action can suggest reevaluating retry. That is not
  allowed for this lane: after a committed or uncertain reservation, all
  lookup outcomes are recovery-only, including absence and deletion.
- The new private recovery code checks target, reservation-derived marker,
  provider reference, and a verified action commitment. Today the trusted
  synthetic callback supplies that commitment. This verifies the kernel
  binding, not the origin or accuracy of a real GitHub response.

### Acceptance For A Provider Observer

1. Use a distinct read-only observation boundary. Bind the trusted adapter
   and expected provider principal to the exact reservation before dispatch.
   The observer must derive the result from provider data, never from a
   replacement's asserted digest. A credential, login, author association,
   or marker alone does not establish the expected stable provider identity.
2. Verify the exact canonical repository and PR, stable comment ID, stable
   author identity, exactly one correctly placed reservation marker, and the
   exact raw payload commitment. Bind the marked payload format before
   dispatch; do not hash rendered Markdown or tolerate undocumented
   normalization. Establish that the observed content corresponds to the
   acceptance boundary. If edits or provider metadata leave that uncertain,
   do not record completion.
3. Treat pagination and changes during lookup as evidence limits. Do not
   classify the first matching page as unique; enforce a bounded enumeration
   or an independently verified comment reference. Truncation, an incomplete
   scan, conflicting results, or unsupported response metadata stays
   unresolved. A verified observation is not a claim of global exactly-once
   provider delivery.
4. Exercise copied markers, wrong author with identical bytes, wrong target,
   changed body with the same ID, edited or deleted comment, duplicate
   matches across pages, and unavailable, unauthorized or incomplete
   responses. Each must produce no completion proof and no additional POST.
   Exact positive replay must preserve one proof. Keep raw body, author and
   provider response out of reviewer output.

GitHub's [issue comment REST contract](https://docs.github.com/en/rest/issues/comments#list-issue-comments)
documents the PR conversation endpoint and raw body, comment ID, user ID,
issue URL and timestamps. Its [pagination contract](https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api)
documents how subsequent pages are advertised. Those fields are inputs to
an observer, not a guarantee about historical accepted bytes or a consistent
snapshot. A production observer and its failure tests still need
implementation and review. Do not select a live target or invoke a real
write under this review.

The private offline verifier now checks synthetic current observations
against stable repository, PR, author and comment identity; exact raw body
and marker placement; creation bounds; and a bounded complete enumeration.
It rejects edits, drift and ambiguous observations. Its strongest result is
`ExactCurrentMatch`, not accepted-payload proof. Equal creation and update
timestamps alone cannot rule out changes at the provider's clock resolution.
No production transport feeds this verifier, no completion path consumes its result,
and it cannot authorize a resend. A trusted adapter and independent evidence
of accepted bytes remain prerequisites for live completion.

The private target registration now requires stable provider repository and
expected author IDs plus an operator-selected adapter commitment. The
scoped policy commits to all three; reopening, grant issuance and
reservation revalidate that policy rather than accepting recovery-supplied
identity. Missing or substituted binding fields fail closed for new
admission. Older synthetic reservations are not upgraded into new authority.
This pins expectations only; it does not verify a running adapter, its
credential, or any accepted provider payload.

New private reservations persist the provider binding and exact attempted
record commitment atomically with admission. The versioned reservation
commitment covers that witness; recovery verifies it and the existing
attempt or completion proof before lookup. Registry revocation prevents
new admission but does not erase historical recovery evidence. Removing
the witness cannot downgrade a new reservation or proof to the older
synthetic format. Older snapshots retain their original read-only
inspection posture without gaining provider identity or send authority.

The private SQLite observation path now connects that witness to the offline
verifier. It validates the exact action and adapter commitment before invoking
a bounded synthetic lookup, and requires the observed creation time to fall
between reservation and the exclusive execution-window deadline. Target
revocation does not erase those historical expectations. A witness without a
recorded deadline remains readable for synthetic inspection but cannot admit
this observation path. A current match cannot complete the run or authorize a
resend; the adapter commitment is not an attestation of a running adapter.

### Focused Maintainer Review Of Reservation-Bound Observation

Reviewed the merged observation at `cac2ebd4` under `dg/review` run
`run-1794110383756795000-2`, with proof-backed approval
`approval/run-1794110383756795000-2/review-scope-approved`. This is a maintainer
review, not independent assurance.

- **Blocker before a real provider lookup:** the lost-acknowledgement path has
  no known comment ID. Candidate selection starts with marker matches, so a
  complete enumeration containing one exact marked row and a second row with
  the same provider comment ID but no marker can return `ExactCurrentMatch`.
  The second row might represent an edit seen on a later page. Count identity
  conflicts across the entire bounded enumeration, not just across marker
  matches. A repeated candidate ID must stay unresolved even when only one
  row has a marker, in either order. Identical repeated rows are not proof of
  a stable enumeration either. Distinct IDs with the same marker must remain
  ambiguous; unrelated unique rows must not obscure an exact candidate.
- **Provider time evidence still needs a contract:** reservation time is
  recorded at local precision, but a provider creation timestamp can be
  coarser or come from a different clock. An observation overlapping either
  bound cannot prove creation within the window. Do not round the reservation
  down or add a tolerance that silently admits pre-reservation or expired
  effects. Specify and test the conservative mapping before using live data.

The SQLite path checks the historical binding before lookup, closes the
read transaction before invoking the callback, and returns only current
posture. No code path converts that posture to a completion record. The
legacy synthetic reconciliation writer is separate and must not be fed live
observations or adapter-asserted digests. Close the duplicate-identity blocker
with synthetic regression coverage before wiring a transport. Even then,
bounded enumeration is not a consistent provider snapshot and cannot prove
the bytes originally accepted. No target selection, live dispatch, or retry
of an uncertain write is authorized by this review.

Both existing offline verifier tests and the SQLite lost-acknowledgement
test passed. The conflicting-row case above was found by tracing candidate
selection; those tests do not exercise it. Repository reads, tests, and
documentation edits were performed outside the phase runner. No WorkReport
artifact or provider evidence was generated.

### Duplicate Candidate Identity Correction

The offline verifier now checks candidate IDs against the full bounded
enumeration before reporting a match or ambiguity. A repeated candidate ID
is unresolved even if only one row carries the marker, and even if the rows
are identical. Distinct unique IDs carrying the marker remain ambiguous;
unrelated unique rows do not hide a match. The offline regression reproduced
the false match before the correction. The SQLite lost-acknowledgement test
now exercises both row orders and checks that recovery posture and events
remain unchanged. Provider time mapping, transport completeness, and proof
of the originally accepted payload are still separate requirements.

### Conservative Provider Timestamp Mapping

The private verifier requires an explicit pair of inclusive signed
corrections from the reported provider timestamp to the trusted local
timeline. The bounds must cover the combined uncertainty from timestamp
encoding and clock differences at creation, not just at lookup. All possible
creation times must be at or after the recorded reservation, strictly before
the recorded deadline, and no later than the observation. Unknown or reversed
bounds stay unresolved, as does unrepresentable timestamp arithmetic. Zero
correction is used only by exact-time synthetic fixtures. Nothing in this
path estimates clock skew, trusts provider display precision, or attests a
live adapter's mapping. Before admitting live data, establish a defensible
source for those bounds; otherwise leave the outcome unresolved. Matching
creation and update timestamps still do not prove the original payload.

### Bounded Synthetic Page Chain

The private verifier now requires normalized page links before classifying
an observation. A scan starts at the initial request, follows each advertised
cursor exactly once, ends at a terminal page, and stays within eight pages
and 128 rows total. A broken link, missing terminal page, repeated cursor,
or exhausted budget leaves the result unresolved even if an earlier page
matches. Identity conflicts are checked across the whole chain.

GitHub's [pagination documentation](https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api)
describes navigation, not snapshot consistency. The synthetic cursor IDs
do not parse or authorize following external URLs. Live admission still
needs a trusted adapter that validates links and provider identity. Neither
the [comment endpoint](https://docs.github.com/en/rest/issues/comments#list-issue-comments)
nor the [HTTP Date contract](https://www.rfc-editor.org/rfc/rfc9110.html#section-6.6.1)
establishes a bound between the clock that recorded comment creation and the
local reservation clock. Do not derive that bound from a later response
header or an observed timestamp's display precision.

### Exact Marked Dispatch Body

Before creating a reservation, the private live-sandbox path now rejects
empty bodies, reserved marker injection, and bodies that exceed the verifier's
raw byte budget once encoded. After a fresh reservation, Core sends the
injected callback one exact raw body: original bytes followed by the canonical
reservation-derived marker suffix. The verifier shares that suffix contract;
neither path normalizes newlines or Markdown. The existing reservation and
marker commitments are unchanged. The SQLite lost-acknowledgement fixture
feeds the exact bytes received by the sender into the observer after reopening
the database. This binds the attempted bytes in the synthetic path, not what
a real external service originally accepted.

### Exact Synthetic Creation Response

The private dispatch no longer accepts a boolean acknowledgement. Before
consuming a fresh grant, reservation checks the adapter commitment against
the active registered target. Dispatch then reopens the committed witness
read-only and checks the exact reservation and invocation before handing
bytes to the injected sender. A synthetic response verifies only when it
matches the [creation response contract](https://docs.github.com/en/rest/issues/comments#create-an-issue-comment):
status `201`, a positive comment ID, exact raw body, and the expected target,
stable repository and author. Missing or substituted responses remain
recovery-only. The injected sender and its claimed adapter commitment are not
authenticated live evidence.

The synthetic sender now passes bounded JSON to the same verifier used
before proof persistence. The decoder requires one positive comment ID and
author ID, the exact raw body, and exact issue and comment URLs at the
expected `api.github.com` target. It rejects duplicate required fields,
malformed payloads, and responses larger than 512 KiB before decoding.
The repository ID is supplied by the adapter's separate identity resolution;
it is not inferred from the comment body or URL. A real adapter must still
establish that identity, enforce the byte limit while reading, and disable
redirects and write retries. The parser does not follow response URLs.

### Private HTTP Exchange

The private transport pins production requests to `https://api.github.com`.
It compares the reserved stable repository ID and canonical name with
[repository metadata](https://docs.github.com/en/rest/repos/repos#get-a-repository)
and the expected author ID with the
[authenticated user](https://docs.github.com/en/rest/users/users#get-the-authenticated-user)
before exposing a single-use creation request. Redirects, connection reuse,
and environment proxy discovery are disabled. Both identity and creation
responses are capped while reading. A dropped connection, redirect,
unexpected status, or oversized response is not a successful acknowledgement
and is never retried by this transport. Loopback tests use fake auth only.

This exchange is not wired to the kernel's dispatch path or any public
command. Identity preflight is not atomic with the POST and cannot protect
against a name moving between requests. Live admission still requires a
provider-side authorization boundary scoped to the disposable repository,
operator approval, and review of the exact transport-to-reservation binding.

An exact synthetic response now persists a completed SideEffect and a
distinct creation-response proof event in the same SQLite transaction.
Reopening the database verifies the proof against the original reservation
and completed effect without calling the sender. Legacy lookup proof IDs
and commitments remain readable. A later synthetic lookup may confirm the
same outcome but cannot replace it, create a second proof, or authorize a
resend. Missing responses and interrupted persistence still need recovery;
this does not establish acceptance when an HTTP response never arrived.

### Focused Maintainer Review Of The Private Exchange

Reviewed merged commit `18b35f5b`. Accept the isolated transport as a tested
building block; do not connect it to live dispatch yet. This is an owner review,
not independent security assurance. Three requirements block live composition:

1. **Prove the target is the approved pull request before POST.** GitHub's
   [issue-comment endpoint](https://docs.github.com/en/rest/issues/comments#create-an-issue-comment)
   accepts both issues and pull requests. Matching the repository and author
   does not prove that the number identifies a pull request. Require bounded
   [pull request metadata](https://docs.github.com/en/rest/pulls/pulls#get-a-pull-request)
   matching the approved target and stable repository identity. Test that an
   issue-only number, a missing PR, or mismatched metadata produces no POST.
2. **Bind the send to the durable reservation and current execution window.**
   The prepared client is single-use but accepts caller-supplied body text; it
   does not prove the adapter commitment or carry the dispatch capability.
   Compose it so only the freshly committed reservation supplies the exact
   adapter binding, target, and marked body. Do preflight before the final
   authority check, and check the trusted deadline before dispatch rather than
   treating a prior reservation as permission to send after a slow preflight
   or process pause. The local check is not atomic with remote acceptance;
   retain repository-scoped provider authorization and the bounded target.
3. **Keep uncertainty recoverable, never retryable as a write.** An invalid or
   missing acknowledgement, expiry after reservation, or interruption after
   POST must not release another send right. Test the composed path with
   delayed preflight, exact retry, and acceptance followed by response loss.
   Only the exact response or independently bound observation can establish
   an effect; never infer that no comment exists from a transport failure.

The private preflight now addresses item 1: it requires a bounded successful
pull-request response with the exact number and matching base repository ID and
canonical name before returning a prepared client. A missing pull request
(including an issue-only number), incomplete or mismatched metadata, and an
oversized response all stop before POST. Items 2 and 3 still block live
composition; none of these preflight reads is atomic with remote acceptance.

The injected SQLite dispatch path now commits the earlier of the grant and
window expiry to the reservation witness. Immediately before giving the marked
body to the sender, it checks a fresh trusted-time observation against the
committed admission, current time provenance, and the strict deadline. An
unusable observation or elapsed deadline leaves the durable attempt in
recovery-only posture. An exact retry cannot regain the send right. This is a
local check, not an atomic deadline at GitHub. The HTTP preflight and adapter
binding are not connected to a production sender, and no live proof is issued.

The test-only loopback composition checks the prepared target and provider
registration inside admission, before consuming the grant, and compares the
prepared binding again to the committed witness before send. It exercises the
actual HTTP exchange with a disposable SQLite store: a changed adapter binding
cannot POST, the matching request sends one reservation-marked body, and a
dropped acknowledgement followed by restart and exact retry sends nothing
again. The fixture reconciles the received body using a synthetic observation;
this is not evidence of live GitHub behavior or a production recovery route.

### Live Admission Review After Loopback Composition

**Verdict: accept the loopback evidence at `07c46c16`; do not admit a live
trial.** This is a maintainer review, not independent assurance. The fixture
demonstrates that binding mismatch does not consume the grant, that the
successful local exchange sends the reservation-marked bytes once, and that
response loss and restart do not mint another send right. The following gaps
must close before requesting live approval:

1. **Resolve the adapter and provider scope independently.** The prepared
   exchange currently copies the expected adapter commitment. The fixture
   proves comparison with the registry, not independent identification of the
   selected implementation or provider authorization. Derive a versioned,
   payload-free binding from the trusted adapter and its reviewed configuration;
   compare that binding with the registry and reservation. Verify provider-side
   access is restricted to the disposable repository. A caller-provided scope
   summary, a matching user ID, or a successful repository GET is not proof
   of that restriction.
2. **Define the last local authority check before POST.** The send path rereads
   the reservation witness and checks time but does not re-resolve grant,
   principal, or target revocation after reservation. A production caller must
   reject changes visible at a final consistent local read and preserve the
   consumed reservation for recovery. Test revocation between reservation and
   send and a prepared exchange that expires before dispatch. Do not hold a
   SQLite transaction across HTTP or claim atomicity with provider acceptance.
3. **Use a real read-only recovery source and distinct evidence provenance.**
   The current observation verifier receives injected pages, and the loopback
   recovery writer receives a synthetic observation of the captured body.
   Implement a bounded provider reader that proves the complete page chain
   and exact identity without inventing timestamps or turning an incomplete
   read into absence. Persist live creation or recovery evidence through a
   distinct verified path; do not route HTTP results into the synthetic
   proof writer. Verify response loss, restart, read-only recovery, and no
   resend as one scenario. Keep a separately approved exact target and an
   independent reviewer as trial prerequisites.

Start with the bounded read-only provider observer and its adverse-page tests.
It can be verified on loopback without enabling any real write. Do not promote
the test-only dispatch wrapper or broaden the roadmap to another mutation family.

### Bounded Read-Only HTTP Observation

The private observer verifies the repository, authenticated actor, and pull
request before listing comments against the stable repository ID and exact
number. It validates [GitHub pagination links](https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api)
and constructs each request itself. Only sequential same-origin links for
the exact named or stable-ID target are accepted. It bounds the scan to eight
pages of sixteen comments and 512 KiB per page, validates row identity and
ascending IDs, and rejects malformed, unauthorized, oversized, or incomplete
responses without inferring absence. Complete observations use the existing
current-comment verifier and require separately supplied trusted time bounds.
Page traversal is not a snapshot, a live acceptance proof, or permission
to retry. Loopback exercises the HTTP boundary; no production caller, writer,
or provider request is enabled.

### Final Local Revocation Check

After reservation, the injected dispatch path samples trusted time and reads
the historical admission and current grant use, principal registration, and
exact target and provider binding in one SQLite snapshot. A grant, principal,
or target revocation visible at that boundary prevents adapter entry and
leaves the consumed reservation intact for recovery. Tests commit each
revocation after reservation and verify that restarting cannot send, add
another admission event, or turn uncertainty into completion. The transaction
ends before the sender is called; a later local change cannot be made atomic
with provider acceptance. Live wiring and evidence provenance remain gated.

### Separate Provider Observation Provenance

The private reservation reader now compares the registered adapter binding
with a versioned identity derived by the fixed GitHub HTTP reader. It uses
the same durable action, reservation marker, repository and author IDs, and
creation interval as the offline verifier, without holding SQLite open across
HTTP. Only an exact match from the fixed production origin can create the
private, non-deserializable current-observation witness. The witness binds the
reservation, target context, comment ID and observation time without storing
the body or authentication material. Injected pages still return a posture;
loopback HTTP can also return a matching posture but cannot mint the witness.
This does not attest provider-side scope, prove creation, persist completion,
or authorize a send. A production caller and durable live evidence path
remain separate review requirements.

### Durable Current Observation

An explicit V13-to-V14 SQLite upgrade adds a separate observation table.
The private writer accepts only the fixed reader's verifier witness and
revalidates the exact durable context in its write transaction. One immutable
record is retained per reservation: exact replay returns it and any changed
record is rejected. Reads recheck the reservation and a domain-separated
payload commitment; storage remains trusted, not externally attested. The
record contains no raw comment or authentication material and cannot be
decoded back into a verifier witness. It records a historical current match,
not provider creation or ongoing presence. It does not complete the SideEffect,
append a completion event, or permit a resend. No live caller is enabled.

### Maintainer Admission Recheck At `d5bb32f8`

**Accept the merged local safeguards; do not admit a live trial.** The final
SQLite check rejects visible revocation before the injected sender. The
bounded reader, private witness, and opt-in V14 store preserve the distinction
between a current match and a completed external effect. Their tests establish
local behavior, not live transport behavior or independent assurance.

1. **The write adapter still has no independently derived identity.** The
   prepared creation path carries a copy of the expected binding, while the
   HTTP reader derives its own versioned commitment. The registry and witness
   currently carry one adapter commitment. Before composing production
   dispatch, bind the read and creation configurations explicitly and prove
   that a substituted implementation, target, or fixture origin cannot reach
   POST. The production path must consume an exact reservation, not promote
   the test-only callback that routes responses into synthetic proof.
2. **Observation time inputs still need a trusted source.** The private reader
   accepts supplied correction bounds; a caller's zero-uncertainty value is
   not evidence of provider clock behavior. Bind reviewed bounds and their
   provenance to the exact operator run. Missing or contradictory bounds must
   leave the result unresolved. Similarly, the local revocation read is not
   atomic with HTTP; carry the approved deadline to the transport handoff and
   do not claim provider-side deadline or cancellation enforcement.
3. **A real trial still needs a separately approved operating boundary.**
   Verify repository-limited provider access for the exact disposable target,
   not just account identity or a declared scope summary. Review the bounded
   operator entry point, the distinction between creation acknowledgement and
   historical observation, and the no-resend behavior after interruption.
   Require a distinct reviewer and explicit trial approval before any live
   provider request. A persisted current observation alone must never advance
   the run as if a creation acknowledgement had been verified.

Next, derive and compare the creation transport identity against the
registration and reservation, retaining the fixed reader's separate
provenance. Prove substitutions on loopback before adding an operator entry
point. Do not enable live dispatch as part of that change.

### Bind The Creation Adapter To The Reservation

The private creation transport derives a versioned adapter commitment from
the fixed GitHub origin and creation configuration, together with the fixed
reader's commitment. Preparation compares that identity against the reserved
binding before HTTP and rebuilds the prepared binding from verified target
and author identities. Existing reader-only reservations remain valid for
observation and its durable evidence, but do not authorize creation. The
test-only loopback transport derives a distinct commitment and cannot produce
a production provider witness.

This does not add an operator command, grant GitHub access, or enable live
dispatch. Trusted observation bounds, the final transport deadline, and
independent trial approval remain separate requirements.

### Maintainer Review Of Observation Time At `d11a1644`

The creation adapter binding is merged and locally accepted. Do not admit
a live trial on that evidence. The remaining time problem has two distinct
boundaries:

- **Local execution window.** `read_reserved_comment_context` reconstructs
  its start and deadline from the verified reservation; the reservation
  commits to the trusted local observation. The observation clock sampled
  after pagination establishes when the reader finished. Neither value
  calibrates the provider's creation clock.
- **Provider creation interval.** `observe_reserved_live_comment` accepts
  caller-supplied corrections and passes them to `observe_bound_comment`.
  `verify_current_comment_identity` checks interval arithmetic, not where
  those corrections came from. The V14 stored projection records reservation,
  context, comment identity, and observation time, but not a source or
  commitment for the corrections. A plausible interval can therefore pass
  the arithmetic without proving its provenance.

There is no reviewed evidence in this lane that bounds the creation clock
for the exact GitHub run. Do not substitute local UTC, displayed timestamp
precision, a lookup response header, or calibration measured only after the
fact. The next code change must remove caller-supplied bounds from the
production witness path and keep that path unresolved until a defensible
source exists. Synthetic loopback tests can still exercise interval math but
must not mint a production witness. Prove the unresolved path does not insert
an observation row, complete an effect, or authorize resend after reopen.

Any future positive time proof must cover timestamp encoding and clock
uncertainty at creation, bind its reviewed source and validity interval to the
exact reservation and adapter, and carry that commitment into the durable
observation and its replay checks. A missing, stale, mismatched, or
contradictory proof must remain unresolved. This does not prohibit separately
reviewing the final local deadline at transport handoff: that deadline limits
when the executor may start a request, not when GitHub will apply it. A lost
acknowledgement remains unresolved and must not lead to another POST.

### Enforce Missing Observation Time Provenance

The private live observation boundary now validates the reservation and
returns unresolved without starting HTTP. It accepts neither time corrections
nor a caller-provided transport. There is no production constructor for the
persistable current-match witness. Loopback verification still tests bounded
pagination and timestamp arithmetic but only returns a non-persistable
posture. The V14 format is unchanged; tests on both supported adapter
bindings verify that an unresolved attempt leaves no row or new events,
requires reconciliation after reopen, and cannot send again.

### Recheck The Deadline At HTTP Handoff

The prepared sender receives the final Core-validated local time and the
exclusive deadline from the committed reservation. It samples the same
local UTC source again after building the request and before handing it to
HTTP. A sample before the validated time or at or after the deadline rejects
the POST. Loopback tests cover the boundary and confirm the consumed
reservation remains recovery-only after reopening. No production dispatch
entry point is enabled. This is a local handoff check, not an atomic network
deadline, provider clock proof, or guarantee of remote cancellation.

### Repository-Scoped Access Review At `a96a241b`

**Do not admit a live trial.** The fixed transport's repository, PR, and
`GET /user` preflight checks establish target and user identity, not the
set of repositories reachable by the supplied access token. Use a dedicated
GitHub App installation on a disposable repository for the first trial.
Do not accept an arbitrary bearer, an installation-wide default, a human
account's access, or evidence assembled by a harness.

GitHub documents narrowing installation issuance by numeric
`repository_ids` and `permissions` and returning granted permissions and
expiry. Enterprise-level installations do not support the same narrowing.
For the proposed path, require one selected repository, exactly the
`pull_requests: write` and `metadata: read` permission set, no account
permissions, and a fresh expiration. Pull request write is sufficient for
issue-comment creation but is not comment-only permission. Workflow OS
must still enforce the exact PR and create action; the provider does not.
See [installation issuance](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-an-installation-access-token-for-a-github-app)
and [comment endpoint permissions](https://docs.github.com/en/rest/issues/comments?apiVersion=2022-11-28#create-an-issue-comment).

Before admitting an operator path:

1. A trusted issuer must bind the reviewed App, installation, numeric
   repository ID, and explicit permissions to the actual issuance
   exchange. Require affirmative issuance evidence, then enumerate
   [installation repositories](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#list-repositories-accessible-to-the-app-installation)
   using that exact in-memory bearer. Require one unambiguous complete
   result with the expected ID and name; extra rows, advertised further
   pages, missing facts, and substitutions fail closed. This proves the
   installation repository scope, not inability to read public resources.
2. Resolve the App and installation independently through the
   [authenticated App API](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#get-the-authenticated-app)
   and require a reviewed mapping to the numeric bot author used in
   comment evidence. Do not equate the App ID with the author ID or retain
   a user-authenticated fallback. The documented
   [authenticated user endpoint](https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-the-authenticated-user)
   does not list installation access tokens as supported. Confirm the
   author mapping and response contract before enabling that transport.
3. Keep the issuer's signing key outside Workflow OS and both harnesses.
   The verifier and prepared sender must use the same opaque in-memory
   bearer instance, with no raw-token entry point or silent rotation.
   Bind a payload-free scope receipt to the immutable run, provider
   identities, target and adapter; retain the earlier of provider expiry
   and local deadline. Never persist the bearer, signing material, raw
   issuance payload, or a reusable credential fingerprint. A replacement
   bearer must not inherit an existing send right.
4. First implement and test the verifier on fixtures without provisioning,
   dispatch, or a public command. Cover broad or omitted grants, wrong
   installation, author or repository, mismatched bearer continuity,
   expiry, incomplete enumeration, and redaction. Fixture verification
   must not admit a live reservation. A distinct live review is required
   for the issuer trust boundary, exact disposable target, real issuance
   and author evidence, and the creation and recovery paths.

### Synthetic Installation Scope Verification

The test-only verifier checks the reviewed App, installation, and bot IDs
against separately supplied fixture observations. It rejects incomplete
repository enumeration, any grant other than the exact repository and
permission set, and observations attached to a different in-memory bearer
even when the bytes match. Its non-serializable receipt is bound to the
run, immutable inputs, target, and fixture adapter, and expires at the
earlier of provider expiry and the local deadline. Diagnostics contain
neither the bearer nor raw provider inputs.

This verifies decision behavior against synthetic facts, not the facts'
provenance. The fixture module is excluded from production and cannot
admit live reservations. Issuer integration, authenticated enumeration,
bot-author mapping, and provider-backed creation proof remain gated.

### Bind Issuance To Repository Enumeration

The private fixed-origin exchange uses a signed App assertion supplied by
the future trusted issuer. It checks the authenticated App and selected,
unsuspended user or organization installation before requesting one
numeric repository and the exact permission set. It requires affirmative
scope and expiry in the issuance response, then verifies a complete
singleton repository list using the same bearer it just received. Missing,
broader, redirected, incomplete, mismatched, or expired responses fail
closed. The opaque in-memory result cannot be serialized or passed to the
comment sender. Loopback tests exercise the HTTP boundary with synthetic
assertions and bearer values; no GitHub App has been provisioned or used.

This does not implement the trusted issuer, verify the bot author, bind the
result to an immutable run, or connect it to live reservation and response
proof. Those remain admission gates.

### Corroborate The Bot And Bind The Scope To The Run

The installation exchange now checks the authenticated App's reviewed ID
and slug, then looks up the separately reviewed numeric author through
[GitHub's account-by-ID endpoint](https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-a-user-using-their-id)
using the same bearer that enumerated the repository. For this sandbox it
requires both a Bot account and the App's expected `[bot]` login. The login
convention is a corroboration check, not an independent source for the
numeric App-to-bot mapping. [GitHub documents bot attribution for installation tokens](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/differences-between-github-apps-and-oauth-apps#token-based-identification).
The future comment response must still name the exact reviewed author.

The private receipt commits to the reviewed run ID, immutable-run hash,
target, App, installation, author, repository, local deadline, and
separately versioned scope adapter. It contains no bearer, raw provider
payload, or reusable secret fingerprint. Reuse requires exact binding and
time within the verified interval; loopback evidence uses a distinct
adapter commitment. This does not establish that the caller obtained the
run inputs from durable Core state. A trusted issuer and Core-owned
admission must supply those inputs and enforce the remaining gates before
any live send. No production dispatch or persistent receipt is enabled.

### Bind The Verified Scope Inside Core Admission

The private SQLite reservation lane for the installation comment adapter
requires its verified scope receipt. It matches the run and immutable
bundle already verified against the durable window, the registered exact
target and provider identities, the adapter commitment, and the Core
grant/window deadline in the reservation transaction. Any mismatch or
expiry rolls back the grant use and reservation together. Production
admission rejects loopback provenance, and the unscoped reservation path
cannot admit either installation adapter identity.

The reservation witness commits to the payload-free scope receipt and
shortens its deadline to the earlier verified expiry. This does not
persist the bearer, full receipt, or raw provider payload. Existing
unscoped witness encodings are unchanged. Reopen and exact replay cannot
issue a new send right, and the synthetic proof writer cannot complete
an installation-scoped reservation. The trusted issuer, provider-backed
completion path, and operator entry point are not connected.

### Exact Installation Creation Acknowledgement

The private transport verifies a bounded `201` response from the same
installation bearer against the exact marked body, PR, provider repository,
and bot author. It binds the resulting opaque proof to the reservation and
scope commitments. A missing or mismatched response yields no proof; lookup
does not substitute for the original response.

Core reconstructs that context from the durable reservation and requested
action before accepting the proof. It persists completion in one transaction
with a distinct proof event identity and commitment domain. Exact replay
returns the same event; conflicting outcomes and cross-reservation proof
are rejected. Loopback uses a separate proof identity, and the existing
synthetic writer and inspection do not promote it to provider evidence.
The transport and Core completion writer are not connected to an operator
or live dispatch route. Connecting them still requires a one-shot reserved
send capability, final authority and PR preflight, and trial admission.

### Compose The One-Shot Installation Dispatch

The private Core path consumes a fresh scoped reservation before any
comment preflight. An exact replay never reaches the provider. The same
installation bearer verifies the pull request number and base repository;
an ordinary issue, mismatched response, or unavailable preflight cannot
reach POST. Core then rechecks the committed reservation, grant,
principal, target, and trusted time in one final read. Only that path
mints the non-cloneable send permit, and the transport consumes it and
checks the deadline again before handing the request to HTTP.

The private sender persists only the exact acknowledgement proof. A
failed preflight, observed revocation, elapsed deadline, or lost response
leaves the committed attempt recovery-only; reopening cannot mint
another permit. Loopback and disposable SQLite exercise this composition
without a real provider or live operator route. Revocation and time
checks are local and not atomic with remote acceptance. The reviewed
issuer, operator admission, and independent trial remain outstanding.

### Operator Admission Review At `da26a691`

**Accept the loopback composition; do not admit an operator or live
trial.** The merged path proves local one-shot behavior and exact response
persistence under the tested conditions. It has no public caller, trusted
issuer, or independent assurance. Three gaps remain:

1. **Recheck current execution authority before POST.**
   `checked_comment_send` verifies the historical admission projection and
   current trusted time, grant, principal, and registered target.
   `validate_existing_projection` verifies historical events and snapshot
   consistency; it does not require that the current window and attempt
   still match the admitted revisions and execution states. The attempted
   effect commitment is verified when writing completion, after POST.
   Require the exact current executing window, started attempt, expected
   post-reservation cursor and revisions, and unchanged attempted effect
   in the final read transaction. Deny a legitimate closure or supersession
   if exposed by the supported lifecycle; separately inject state or effect
   drift in a disposable test. Prove no POST and no resend after reopen.
   These are missing preconditions, not evidence that a public caller
   can currently exploit the private route.
2. **Make historical disposition inspectable before acquiring provider
   authority.** The private dispatch currently returns no event for both
   replay and unresolved sends. The synthetic inspector intentionally
   rejects an installation completion, and provider time remains unproven.
   Add a read-only, credential-free scoped inspector for exact run,
   operation, and effect that distinguishes verified creation from an
   unresolved reservation, and rejects corruption, synthetic proof, and
   mismatched identities. Reopening and inspection must issue no provider
   request or new permit, even after local grant revocation or expiry. The
   future operator must check durable disposition before requesting an
   App assertion or minting a bearer; no operator retry may substitute a
   new identity for an unresolved send.
3. **Bind the operator and issuer to one admitted trial.** Accept only
   reviewed App, installation, bot, repository, and PR identities bound
   to the exact immutable run. Use a separately reviewed issuer with
   signing material outside the harness and Kernel, and keep assertions
   and installation bearers out of CLI arguments, workflow YAML, durable
   state, and diagnostics. Require exact prior evidence, an independent
   reviewer, and separately scoped approval for the live run. Confirm
   actual issuance, repository scope, and provider attribution before
   treating any fixture result as representative. A lost acknowledgement
   stays unresolved unless an independently defensible recovery source
   is admitted; neither a timestamp guess nor a fresh POST is recovery.

Next, harden the final local execution-state check and reproduce drift
between preflight and send. Then add credential-free inspection before
composing an operator entry point. Keep both slices local and preserve the
one-shot boundary. None of this permits another provider mutation family.

### Recheck Current Execution State Before POST

The final read transaction now requires the admitted window and attempt
to remain executing and started, at the exact committed revisions and
cursor, with unchanged authority, governance, immutable bundle, and
attempted effect. A newer run event also invalidates the send cursor.
The historical reservation remains consumed; it does not become a new
send right if current execution state changes. Disposable post-preflight
drift tests exercise the denial and no-resend boundary without requiring
a live operator or changing the persisted history contract. This is a
local precondition, not atomic with provider acceptance.

### Inspect Scoped Installation Disposition Offline

Core can read the exact run, reservation operation, and SideEffect in one
read-only SQLite transaction. It verifies the durable admission, projected
history, installation scope commitment, and matching attempted effect or
installation-creation proof. It returns only verified creation or reconciliation
required. Missing or inconsistent records and synthetic or fixture proof fail
closed; reconciliation required does not mean the provider rejected the POST.

Historical inspection does not depend on a currently usable grant, request
provider authority, write state, or yield a send permit. The contract tests seed
disposable production-shaped records without transport and separately reject
actual fixture records. These tests are not evidence of a live GitHub creation.
The trusted issuer, operator route, and independently reviewed live trial remain
separate admission steps.

### External Signer Admission Review At `85e2533f`

**Accept offline inspection; do not admit a signer, operator, or live trial.**
The inspector verifies an existing reservation. Missing history is an error,
not proof that a fresh exchange is safe. The current installation dispatcher
requires an already verified scope before reserving a send. Its serialized
reservation and final send check can prevent a duplicate POST, but they cannot
prevent duplicate App-assertion requests or installation exchanges that
precede the reservation. Do not move the existing send permit before scope
verification or catch an inspection error and call the signer.

The first executable step is one **Core-owned pre-issuance claim**, private to
the existing live comment lane. In one immediate transaction, derive the
canonical operation from the admitted attempt and immutable action; validate
the exact current principal, authority, approval evidence, window, cursor,
grant, target, attempted SideEffect and reviewed App/installation/bot/repo/PR
binding. Check the reservation, event, grant-use and claim records for that
attempt, not just the requested operation row. A mismatched key, missing
counterpart or contradictory record fails closed. Only an exact fresh attempt
may commit a new claim. Bind it to the existing grant and an exclusive deadline;
it is an ordering fence, not a parallel grant or permission to POST. Return one
process-local exchange right only after commit. Do not hold a database
transaction open across signer or network calls.

On replay, an existing reservation takes precedence: verify and return its
historical disposition without signer or GitHub calls. A claim without a
reservation after process loss is an explicit issuance-uncertain wait. A
replacement must not reacquire by changing IDs, and a timeout is not proof
that nothing was issued. Until an independently reviewed reconciliation path
is admitted, keep the wait inspectable and block automatic reissuance. Recheck
the same claim and current authority before using an assertion or attaching a
verified scope to the existing atomic send reservation; final pre-POST checks
remain mandatory. No claim, assertion or scope receipt by itself enables a send.

Use a separately administered signer with a protected channel and a caller
identity the harness cannot impersonate. The signer verifies the exact admitted
claim and signs only for the configured App; no raw signing endpoint, shared
harness identity, or caller-selected executable. The short-lived App assertion
is more powerful than the eventual installation bearer: it is not scoped by
the local claim at GitHub. Keep the signing key outside the kernel and both
harnesses, and deliver the assertion only to the fixed-origin exchange in the
trusted process. Exclude assertions, bearers and raw responses from durable
records, arguments, environment, and diagnostics. A deployment that cannot
isolate the exchange process from the harness must not run the trial.

Test the pre-issuance claim with a counted injected exchange boundary before
adding a signer channel: fresh exact admission permits one exchange across
competing callers; wrong binding, stale approval, expired or revoked grant,
or corrupt or partial records permit zero. Fault before commit, after commit
before return, and after assertion issuance before reservation. Exact replay
must distinguish absence, unresolved claim, unresolved reservation and
verified creation without issuing a second exchange right. Exercise a later
revocation and a mismatched scope at reservation time. This maintainer review
does not replace independent review of the signer deployment, provider facts,
or live trial.

### Fence The First Installation Exchange

The private SQLite lane now requires explicit V15 activation before claiming
an exchange. Core derives the operation from the exact attempt and immutable
invocation, and the reviewed App and installation must match the target
revision already bound into the scoped policy decision. The claim and its
grant binding commit together without consuming the grant or creating a send
reservation. Fresh authority is rechecked before invoking the exchange, and
the process-local result must carry that same claim into the send reservation.
The transaction does not extend across the exchange or make local revocation
atomic with a remote request.

After restart, a durable claim without a send reservation means issuance is
uncertain, even when the claim outlives the grant. An existing reservation is
classified by the offline inspector. A missing counterpart, changed request,
or inconsistent evidence is not permission to start over. No public route,
signer channel, real assertion, or live provider call is enabled by this slice.

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
