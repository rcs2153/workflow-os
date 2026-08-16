# Authorized Execution Continuity Semantic V2 Blocker Fix Review

## 1. Executive Verdict

**Needs additional blocker fixes. Do not proceed to the SQLite V2 continuity
backend.**

The focused correction fixes historical expiry replay, terminal-state
precedence, the reviewed embedded-record identity cases, and V1/V2 wire
compatibility. Independent adversarial review nevertheless found one remaining
authoritative ownership defect: replay of a committed yield registration still
accepts a window whose `active_yield` no longer names the committed generation.

The next phase must close that exact owner-to-target relationship and add an
operation-family integrity matrix before SQLite copies the semantic oracle.

## 2. Accepted Corrections

- Historical expiry rejection replay accepts a lawful monotonic global
  trusted-time successor after an unrelated valid operation.
- Non-expiry security rejections remain exact because quarantine prevents
  later lawful mutation.
- Persisted `closed`, `expired`, `revoked`, and `superseded` windows classify
  as terminal before live clock and execution-eligibility checks.
- The reviewed window, directive, attempt, yield, and wait embedded identifiers
  are compared with their map keys or committed result identities.
- Cursor, subject, authority commitment, and owning-window relationships are
  checked where the current validator names them.
- V1 preserves caller operation ordering and accepts previously tolerated
  unknown outer and entry fields.
- V2 remains strict for unknown outer and nested entry fields.
- Errors remain stable, bounded, and non-leaking.

These corrections are suitable to retain. They do not by themselves make the
reference semantics complete enough for a durable backend.

## 3. Remaining Blocker: Active Yield Ownership

`validate_success_target` validates a `YieldRegistered` result against the
window, attempt, and yield rows, including their embedded identifiers and
cursor relationships. It does not require the authoritative window's
`active_yield` to equal the committed `generation_id` when the window remains
at the committed revision.

The review reproduced this sequence:

1. consume a directive and create a started attempt;
2. register a yield successfully;
3. mutate only the authoritative window's `active_yield` from the committed
   generation to `None`; and
4. replay the exact yield-registration operation.

The replay returned success instead of
`authorized_execution_continuity_state.state.corrupt`. The committed yield
therefore remained individually well-shaped while detached from its
authoritative owning window.

This contradicts the blocker-fix report's claim that successful-operation
replay rejects authoritative ownership mismatches. A durable SQLite backend
must not copy an oracle that accepts this disconnected success graph.

Required correction:

- when the authoritative window is still at the committed yield revision,
  require `window.active_yield == Some(committed_generation_id)`;
- distinguish legitimate later successor states from corruption rather than
  requiring historical current-state equality forever; and
- add equivalent owner-to-target corruption probes for all five operation
  families, including active-yield, wait membership, directive ownership,
  attempt ownership, and consume-operation linkage.

## 4. Historical Replay Assessment

The historical expiry correction now recomputes the original security
transition and accepts current global trusted-time state only when revision and
watermark are monotonic and posture remains one of the legal successor
postures. The rejected window itself remains exact. The focused sibling-window
regression passes.

Independent review also probed replay of a successful directive consumption
after a later lawful attempt outcome. That replay remained exact, so no blocker
was found in that path. The probe was review-only and was removed after the
result was observed.

The reference model still has no external rollback-resistant epoch anchor and
does not prove arbitrary backup restoration. Those remain documented later
operational limitations rather than regressions in this fix.

## 5. Terminal Classification Assessment

The terminal-state correction is accepted. Window identity is checked first,
and persisted terminal states no longer depend on a currently available clock
or live execution eligibility. `expired` additionally requires its persisted
watermark to reach expiry. Non-terminal states retain the conservative live
clock, provenance, epoch, eligibility, and expiry checks.

This is the correct semantic distinction for the false-stall problem: a valid
terminal record is terminal, while a yielded runnable window can be classified
for continuation without fabricating an approval wait. It does not implement a
supervisor or executor redispatch.

## 6. Compatibility And Privacy Assessment

The V1 public constructor no longer sorts valid caller input. V1 deserialization
accepts unknown outer and entry fields as the prior wire behavior did. Required
fields, duplicate or missing operation families, and invalid enum values still
fail closed.

The private V2 entry wire retains `deny_unknown_fields`, so V1 tolerance does
not widen the additive V2 contract. Malformed-wire errors remain fixed and do
not echo secret-like field names or values. No prompt, source content, command
output, provider payload, credential, token, or reusable authority is added.

## 7. Test Quality Assessment

The focused regressions adequately cover:

- expiry rejection replay after sibling-window trusted-time advancement;
- all four terminal states under unavailable, expired, and quarantined live
  posture;
- embedded window, directive, and attempt identity corruption on directive
  consumption;
- V1 order and unknown-field compatibility; and
- V2 strict unknown-field behavior.

Coverage remains incomplete for the ownership blocker. The new embedded-ID
test is concentrated on directive consumption and does not provide a
table-driven corruption matrix across every authoritative row and operation
family. The adversarial active-yield probe demonstrates that this is a
behavioral gap, not merely a preference for more tests.

## 8. Validation Evidence

The review ran:

- the 23 focused continuity semantic unit tests: passed;
- the 6 public V1/V2 continuity contract tests: passed;
- a review-only successful-consume successor replay probe: passed;
- a review-only detached-active-yield replay probe: **failed closedness** by
  returning success; and
- the merged PR's seven GitHub Actions jobs: passed before merge.

The temporary probes were removed after execution. Required repository
validation also passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace` using a clean target directory under
  `/private/tmp`;
- `npm run check`;
- `npm run check:integrations`;
- `npm run check:docs`; and
- `git diff --check`.

## 9. Scope Verification

The review changed documentation only. It did not implement the blocker fix,
SQLite state, schema or transactions, a scheduler, executor supervisor,
automatic approval, provider mutation, CLI or workflow-schema behavior,
hosted execution, nested harness runtime, or release-posture changes.

## 10. Governed Review Record

- workflow: `dg/review`;
- run: `run-1786840159846277000-2`;
- approval: `approval/run-1786840159846277000-2/review-scope-approved`;
- presentation: `presentation/7e1ebe3b11a13972`;
- approval outcome: granted under standing delegated-maintainer authority after
  the complete proof-enforced handoff was evaluated;
- phase status: completed; and
- out-of-kernel work: source inspection, adversarial probes, validation,
  documentation edits, and command execution were performed by the external
  executor; the kernel recorded governance only.

## 11. Recommended Next Phase

Perform one focused blocker correction for active-yield ownership and the
equivalent exact owner-to-target integrity matrix across all successful
operation families. Then repeat focused maintainer/security review. Do not
begin SQLite V2 implementation, supervisor integration, approval automation,
or provider mutation broadening before acceptance.
