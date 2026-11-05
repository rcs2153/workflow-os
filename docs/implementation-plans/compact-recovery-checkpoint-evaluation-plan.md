# Compact Recovery Checkpoint Evaluation Plan

Status: Planned P0 product-evidence work. This plan does not authorize runtime
implementation, model-provider integration, publication, or a public efficiency
percentage claim.

Related work:

- [Authoritative Agent Continuation Context And Rehydration Plan](authoritative-agent-continuation-context-rehydration-plan.md)
- [Self-Governed Build Benchmark](../user-guide/self-governed-build-benchmark.md)
- [Roadmap](../../ROADMAP.md)

## 1. Purpose

Workflow OS should make compact, authoritative, current recovery checkpoints a
first-class product requirement. The checkpoint is intended to orient an agent
after conversation loss without forcing it to spend another full model turn
rediscovering state that the kernel already knows.

The checkpoint is not:

- an approval, capability, policy decision, or execution credential;
- a replacement for same-call current-authority resolution;
- proof that a model read or understood the projection;
- a transcript, hidden-reasoning record, or unbounded state dump; or
- a guarantee of token, billing, latency, or completion-cost savings.

The product hypothesis is narrower:

```text
A compact, provenance-backed checkpoint of current kernel state can eliminate
a larger evidence-reconstruction turn when conversational memory is stale,
while preserving the correct recovery decision and governance boundaries.
```

## 2. Controlled Evidence Already Observed

A controlled evaluation at repository revision `000a31a` reported:

- one model: GPT-6.1 Sol;
- twenty fresh sessions used to simulate context loss;
- four matched recovery decisions in the reported comparison;
- the same correct decision in all four matched cases;
- summary plus local evidence lookup: 108,770 gross input/output tokens and two
  evidence lookups;
- the same summary plus a compact current kernel checkpoint: 72,407 gross
  input/output tokens and zero evidence lookups;
- a gross reduction of 36,363 tokens, or 33.43%;
- almost all observed savings concentrated in two stale-summary cases where the
  checkpoint eliminated a second model/tool turn; and
- only 7 to 13 added tokens in two cases where the summary was already
  sufficient.

The tested revision is contained in current `main`. The arithmetic is coherent,
but the repository does not yet contain the complete raw twenty-session package,
the exact pair-selection method, or exclusion records. Cached input favored the
summary-first condition. This result concerns gross tokens, not verified billed
dollars or whole-workflow completion cost.

A follow-up evaluation reports actual Codex compaction rather than only fresh-
session simulation:

- three paired scenarios;
- all nine decisions correct in both conditions;
- summary/recovery condition: 167,470 gross tokens;
- compact-checkpoint condition: 83,857 gross tokens; and
- a gross reduction of 83,613 tokens, or 49.93%.

This materially strengthens ecological relevance because context replacement
occurred through the real host compaction path. It still does not support a
production-wide percentage claim. The repository does not contain the complete
raw prompts, cut points, model settings, outcomes, token records, or rerun
package, and three paired scenarios do not establish cross-model or cross-task
generalization.

## 3. Current Claim Boundary

Allowed internal claim:

> Small-scale controlled evidence supports the hypothesis that a compact current
> kernel checkpoint can avoid an additional evidence-reconstruction turn while
> preserving the recovery decision.

Disallowed claims until the full evaluation passes:

- Workflow OS cuts token cost by 33.43%, 49.93%, or any fixed percentage;
- Workflow OS reduces production cost by a fixed percentage;
- checkpoint injection always reduces tokens or latency;
- the result generalizes across models, task families, hosts, or compaction
  mechanisms;
- the checkpoint proves comprehension, authority, or end-to-end correctness; or
- the controlled sessions are equivalent to actual in-session compaction.

README, release notes, repository About text, social posts, benchmark summaries,
and other public positioning must honor this boundary.

## 4. Versioned Evaluation Artifact

Before any public efficiency headline, preserve one reviewable evaluation
package in a separately governed artifact location. It must include:

- evaluation schema and version;
- repository revision and dirty-tree posture;
- Workflow OS binary identity and build provenance;
- exact model, provider, model settings, tool availability, and host version;
- prompts, summaries, checkpoints, fixtures, and authoritative expected state;
- all twenty raw session outcomes, including failed, excluded, or unmatched
  sessions;
- pair-selection algorithm and every inclusion/exclusion reason;
- input, cached-input, output, and total token accounting for each turn;
- billed-cost data only when obtained from an authoritative provider record;
- tool/evidence lookup count and references;
- decision-correctness rubric and adjudication record;
- latency and end-to-end completion timing boundaries;
- SideEffect, approval, authority, evidence, and report posture;
- rerun commands and deterministic fixture setup; and
- a payload/redaction review proving no secret, credential, private prompt, or
  prohibited raw content is published.

Aggregate tables must be derivable from raw records. Exclusion cannot improve a
headline silently. Missing provider billing evidence remains `not_available`,
not an estimate presented as fact.

## 5. Experiment Conditions

Run matched tasks under three primary conditions:

1. `summary_only`: the replacement agent receives only the same bounded
   conversational summary.
2. `summary_plus_on_demand_lookup`: the agent receives the summary and may use
   the documented local evidence path when it judges lookup necessary.
3. `summary_plus_automatic_checkpoint`: the agent receives the same summary plus
   a compact checkpoint generated from the current authoritative kernel state.

Optional diagnostic conditions may include checkpoint-only and full permitted
bundle, but they must not replace the three primary conditions or be pooled into
their reported result.

The checkpoint condition must use an exact canonical projection generated from
the same underlying authoritative state available to the lookup condition. It
must not contain the expected answer, hidden adjudication, or extra task guidance
not derivable from that state.

## 6. Pre-Registered Recovery Cut Points

At minimum, induce context replacement at:

- before an approval request is presented;
- after approval is granted but before the authorized operation;
- after a SideEffect is durably recorded but before report closure; and
- after terminal commit while a stale summary still describes pre-terminal
  work.

Include both already-current summaries and intentionally stale summaries. The
staleness must arise from governed state transitions, not misleading prose
injected solely to make the checkpoint condition look better.

## 7. Task And Model Coverage

The broader evaluation must include multiple realistic task families:

- repository implementation and validation;
- approval-gated maintenance;
- evidence or report reconciliation;
- provider read/reconciliation without mutation;
- bounded SideEffect recovery; and
- terminal replay or duplicate-prevention posture.

Repeat across several model families and at least two host/context-replacement
mechanisms. One condition must use actual in-session compaction or host-level
context replacement in a long-running coding task rather than only fresh-session
simulation.

## 8. Measures

Primary correctness and safety measures:

- recovery-decision correctness;
- exact current objective and accepted-intent selection;
- completed-work recognition;
- duplicate SideEffects;
- false authority or approval claims;
- stale or superseded intent use;
- missed blockers or validation obligations;
- lawful next-action equivalence; and
- end-to-end task completion.

Efficiency and experience measures:

- model turns to recovery;
- evidence/tool lookups;
- gross input, cached-input, output, and total tokens;
- authoritative billed cost when available;
- time to correct orientation;
- end-to-end latency; and
- user or delegated-maintainer interruption count.

Report distributions and per-case results, not only averages. A token reduction
does not pass if it increases incorrect continuation, duplicate mutation, stale
intent use, false authority, or incomplete work.

## 9. Matching, Selection, And Exclusion

Pre-register task instances, randomization, pairing rules, expected decisions,
and exclusion criteria before running the broader experiment. Preserve every
attempt. Acceptable exclusions are limited to documented infrastructure failure,
provider outage, or corrupted fixture setup that prevents all conditions from
receiving the declared inputs.

Do not exclude a session because it used more tokens, required another lookup,
made an incorrect decision, or produced an unfavorable result. Unmatched cases
remain reported separately with their reason. The twenty-session controlled
evaluation must be reconciled under these rules before it is used publicly.

## 10. Checkpoint Product Contract

The eventual compact checkpoint must be:

- generated from the accepted continuation inventory, bundle, and understanding
  projection rather than ad hoc transcript summarization;
- purpose-, actor-, run-, phase-, cursor-, and freshness-bound where applicable;
- provenance-backed and explicit about unknown, conflicting, omitted, or
  withheld posture;
- sensitivity-filtered through governed context access;
- bounded in size and deterministic for equivalent authorized inputs;
- non-authorizing and incapable of satisfying approvals, policy, capability,
  evidence, checks, or same-call current-authority resolution;
- invalidated by relevant authoritative-state change; and
- safe to regenerate after process restart with no cache dependence.

Checkpoint delivery and acknowledgment may prove exact projection delivery. They
must not claim model comprehension. A material action still requires fresh
same-call Core resolution at the concrete consumer boundary.

## 11. Sequencing

1. Preserve and independently review the complete controlled-evaluation raw
   package.
2. Define a versioned eval schema, fixture manifest, correctness rubric, and
   aggregate derivation.
3. Reconcile why twenty sessions yielded four matched reported pairs.
4. Implement no runtime checkpoint feature until the canonical continuation
   projection and exposure contract are accepted.
5. Add the three-condition long-running coding-task experiment with actual
   compaction/context replacement.
6. Run the pre-registered cut points across multiple tasks and models.
7. Review correctness and safety before evaluating efficiency.
8. Permit bounded public positioning only after the versioned package is
   reproducible and independently reviewed.

This evidence lane does not reorder the current WorkReport integrity queue and
does not authorize Salsa, provider mutation, hosted execution, nested harnesses,
new authority, or publication.

## 12. Acceptance Criteria

- The complete controlled-evaluation package is versioned and reproducible.
- All twenty sessions and all selection/exclusion decisions are auditable.
- The broader experiment includes real context replacement during long-running
  coding work and all four pre-registered cut points.
- All three primary conditions receive equivalent task inputs and access to the
  same underlying authoritative state.
- Correctness, duplicate SideEffects, false authority, stale intent, blockers,
  and completion are evaluated before efficiency.
- Gross tokens, billed cost, latency, lookups, turns, and completion cost remain
  separately reported.
- Results are reported per model and task family before any pooled aggregate.
- The checkpoint remains non-authorizing, provenance-backed, bounded, and
  invalidated by relevant state change.
- Public claims state the tested scope and do not generalize a controlled
  percentage beyond the evidence.
- Failure to reproduce or preserve correctness blocks the efficiency claim but
  does not weaken the kernel's governance requirements.

## 13. Final Recommendation

Treat the controlled result as meaningful product evidence and a prompt to make
compact recovery checkpoints explicit in the product contract. Do not turn it
into a marketing percentage yet. First preserve the raw evidence, prove the
selection logic, run a real compaction experiment, and show that efficiency
never trades away correctness, authority, or SideEffect safety.
