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
