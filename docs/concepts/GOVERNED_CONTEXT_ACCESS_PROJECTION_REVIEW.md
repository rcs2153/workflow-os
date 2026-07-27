# Governed Context Access Projection Review

## 1. Executive Verdict

**Phase accepted with non-blocking follow-ups.**

The implementation is a bounded, deterministic, payload-free projection of
authorized context references for one exact actor, workflow, run, step, and
optional harness context. It preserves the distinction between knowing a
stable reference and being authorized to dereference its target.

The review found two blockers: standalone entries could retain an unavailable
target, and default enum deserialization could echo invalid caller-supplied
values before model validation. Both are corrected and covered by focused
regression tests.

## 2. Scope Verification

The phase stayed within the approved model-and-pure-helper scope.

It did not add target dereference, source or repository reads, memory access,
tool loading, command execution, connector activation, provider calls,
OpenShell integration, runtime consumption, persistence, events, authority
receipts, schemas, SDKs, CLI behavior, SideEffect execution, writes, hosted
administration, enterprise identity, or release changes.

## 3. Model Assessment

The model is domain-neutral and appropriately narrow:

- `GovernedContextReferenceTarget` has a fixed typed first-slice taxonomy and
  no generic string escape hatch;
- `GovernedContextReference` retains stable identity, sensitivity,
  availability, and redaction posture without target contents;
- `GovernedContextProjectionCandidate` retains the exact capability resolution
  evaluated for one requested access level;
- `GovernedContextProjectionEntry` exposes a stable reference and optional
  fixed bounded metadata only;
- `GovernedContextProjectionGap` exposes target kind and bounded reason without
  the rejected target ID; and
- `GovernedContextProjection` retains candidates, entries, and gaps so its
  serialized derivation can be recomputed.

## 4. Target Taxonomy Assessment

The first slice supports existing Core identities for:

- EvidenceReference;
- workflow events;
- audit events;
- validation diagnostics;
- approval decisions;
- policy decisions;
- SideEffects;
- typed handoffs; and
- WorkReports.

Every target constructs through its existing validated ID type. The canonical
resource is derived by Core as `<target-kind>/<stable-id>`. Callers cannot
invent an untyped target category or choose an unrelated resource mapping.

## 5. Authority Assessment

`reference_only` maps exactly to `context.reference.view`.
`bounded_metadata` maps exactly to `context.metadata.view`.

Both require `CapabilityResourceKind::ContextReference` and the exact
Core-derived resource reference. Candidate validation also requires matching
actor, workflow, run, step, optional harness, evaluation time, and
sensitivity.

Only an `authorized` resolution with an available target and sensitivity
within the projection ceiling can produce an entry. Availability does not
grant authority, and independent policy, approval, evidence, and check
requirements remain separate obligations.

The projection is not a lease or time-of-use grant. A later dereference must
re-resolve current authority and availability.

## 6. Candidate, Entry, And Gap Integrity

Candidates are sorted deterministically and duplicates fail closed. Projection
validation recomputes exact entries and gaps from the retained candidates and
rejects omission, substitution, reordering, or inconsistent derived posture.

The review found that a standalone serialized entry could carry an unavailable
reference while retaining an otherwise authorized source resolution. Entry
validation now requires the retained reference to be `available`.

Candidate completeness is internal completeness relative to the candidates
supplied to the helper. This model does not prove that the caller supplied
every context target that exists in a repository, store, or external system.
Required-context contract consumption and immutable-source binding remain
separate future boundaries.

## 7. Privacy, Debug, And Serde Assessment

- Debug output redacts target IDs, actor, workflow, run, step, harness,
  capability, resource, grant, and redaction text.
- The model stores no raw source, event, report, evidence, provider, command,
  parser, environment, credential, or transcript payload.
- Bounded metadata is fixed to target kind, declared sensitivity, and
  availability observation time.
- Gaps omit rejected target IDs.
- Secret-like IDs and redaction metadata fail with stable non-leaking errors.

The review found that default Serde unknown-variant errors in local and reused
nested enums could echo a forged caller value. Custom static deserializers now
cover governed-context enums, capability availability/resolution enums,
capability resource kind, WorkReport sensitivity, and redaction disposition.
Invalid wire values fail without echoing the rejected value.

## 8. Test Quality Assessment

Focused tests cover:

- authorized reference-only and bounded-metadata projection;
- every fixed target variant;
- exact capability and canonical resource mapping;
- unavailable, unknown, missing-authority, independent-prerequisite, and
  sensitivity gaps;
- deterministic ordering and duplicate rejection;
- wrong context and access authority;
- serde round trip;
- omitted and reordered candidate/entry rejection;
- standalone unavailable-entry rejection;
- local and nested enum error non-leakage;
- secret-like ID and redaction rejection;
- Debug non-leakage; and
- absence of forbidden raw-payload fields.

Existing capability-authority and WorkReport regression suites pass after the
shared safe-deserialization changes.

## 9. Documentation Assessment

The roadmap, implementation plan, and report state that:

- governed context projection is implemented;
- reference visibility is not payload authority;
- the helper is pure and non-executing;
- candidate completeness is not global target discovery;
- runtime freshness, immutable binding, receipts, and dereference remain
  future work; and
- providers, OpenShell, SideEffects, writes, schemas, SDKs, and CLI behavior
  remain unimplemented.

The documentation does not overclaim current runtime capability.

## 10. Blockers

None after the focused corrections.

## 11. Non-Blocking Follow-Ups

- Define how required-context contracts identify mandatory references without
  turning declaration into authority.
- Bind future runtime projections to immutable run inputs or a reviewed
  authority receipt.
- Re-resolve authority, target availability, sensitivity, and policy at
  dereference time.
- Record audited dereference separately from projection.
- Ensure any future sandbox receives only projected material rather than
  ambient workspace access.

## 12. Validation

- `cargo fmt --all --check`: passed.
- `cargo test -p workflow-core --test governed_context_access --quiet`: passed,
  11 tests.
- `cargo test -p workflow-core --test capability_authority --test work_report --quiet`:
  passed, 48 capability-authority tests and 223 WorkReport tests.
- `cargo clippy -p workflow-core --test governed_context_access -- -D warnings`:
  passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace --quiet`: passed.
- `npm run check:docs`: passed.
- `git diff --check`: passed.

## 13. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785129133413843000-2`
- approval ID:
  `approval/run-1785129133413843000-2/review-scope-approved`
- presentation ID: `presentation/be875148227d7a0f`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted approval handoff
- event summary: 39 events, 1 approval, 0 retries, 0 escalations, with
  proof-enforced presentation
- blockers found: 2
- blockers corrected: 2

An earlier review run referenced during resumed work was no longer present in
the helper's temporary state event store and could not be closed. No kernel
state was edited or reconstructed by hand. This fresh proof-enforced review run
governs the accepted phase close recorded here.

## 14. Recommended Next Phase

Proceed to **required-context contract consumption planning**.

Define how an immutable, validated required-context declaration is compared
with a fresh projection without making declaration equivalent to authority.
Keep the first phase planning-only. Do not add target dereference, runtime
payload access, providers, OpenShell, sandbox lifecycle, SideEffect execution,
writes, schemas, SDKs, CLI behavior, hosted administration, or enterprise
identity.
