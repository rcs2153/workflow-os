# Required Context Current Authority Fact-Set Plan Review

## 1. Executive Verdict

**Plan accepted with non-blocking implementation guardrails.**

The plan defines the missing completeness and provenance boundary that must
exist before any authoritative same-call required-context time-of-use result.
It composes existing authority sources without flattening them into booleans
or authorizing target access.

## 2. Scope Verification

The plan remains planning-only. It does not authorize:

- a fact-set implementation in this phase;
- an authoritative time-of-use decision;
- context or source dereference;
- executor integration or persistence;
- events, audit projection, receipts, artifacts, schemas, SDKs, CLI, UI, or
  examples;
- providers, OpenShell, sandbox execution, SideEffects, or writes;
- hosted administration, reasoning lineage, or release changes.

## 3. Problem Assessment

The plan identifies the correct blocker: existing deterministic capability
resolution can evaluate supplied candidates but cannot prove that the caller
supplied every relevant current candidate.

Treating an arbitrary slice as complete would make omission an authority
escalation path. The plan correctly makes completeness a property of an exact
query against a bounded source snapshot rather than a caller-controlled
boolean.

## 4. Source Boundary Assessment

The source-of-truth table preserves the ownership of:

- immutable run identity;
- required-context contract requirements;
- capability grant lifecycle and scope;
- availability observations;
- policy decisions;
- approval requests and decisions;
- evidence references;
- verified local checks;
- sensitivity;
- SideEffect posture; and
- proportional-governance presentation.

The fact set is a payload-free composition boundary, not a replacement store
or policy engine.

## 5. Query And Completeness Assessment

Deriving the query set from every typed required and optional contract
requirement is correct. Callers cannot omit inconvenient requirements.

The proposed source binding covers the exact query-set hash, source snapshot,
observation time, record counts, records hash, and completeness posture. The
absence of a caller-controlled `complete: bool` is a critical design choice.

Implementation guardrail: arbitrary public construction or deserialization
must not mint trusted `CompleteForExactQuery` provenance. A deserialized model
may prove internal commitment consistency, but a future consumer must require
a Core-owned completeness-capable source comparison in the same authority
path.

## 6. Grant And Availability Assessment

The plan correctly requires all potentially decision-relevant grants,
including revoked and expired candidates, to remain present. Filtering them
before resolution would conceal relevant denial posture.

Availability remains independent from authority. The plan blocks missing,
duplicate, stale, future-dated, unknown, disconnected, or unsupported
observations for a future ready path.

Freshness policy remains an implementation choice, but its required existence
is clear.

## 7. Prerequisite Assessment

Policy, approval, evidence, and checks remain independent accepted facts rather
than booleans.

The plan correctly identifies that the current generic `PolicyDecision` may
not carry sufficient exact step and policy-definition identity for direct use.
Likewise, an approval decision ID without its exact request subject and expiry
is insufficient.

These are not plan blockers because the first phase is model-only and the plan
explicitly allows a new payload-free accepted-fact wrapper where existing
records are insufficient. Implementation must not silently reinterpret a
generic record as exact accepted authority.

## 8. Sensitivity And SideEffect Assessment

Sensitivity composition is monotonic: the most restrictive applicable bound
wins and unknown blocks.

The explicit read-only SideEffect posture prevents context visibility from
becoming write authority. The plan does not add SideEffect execution or imply
that a readable target may be mutated.

## 9. Product Alignment

The plan supports proportional governance and quiet success correctly.
Presentation may become less interruptive for low-risk work, but the
underlying authority and completeness proof does not weaken.

The optional sandbox-provider boundary also remains correct: Workflow OS owns
governed intent and authority; a provider may later own containment. Neither
layer substitutes for the other.

## 10. Privacy And Error Assessment

The proposed fact set stores bounded typed IDs, hashes, postures, timestamps,
counts, and source references only.

Raw source, target, provider, policy-input, approval-prose, evidence, check,
command, parser, environment, credential, log, path, and sandbox payloads
remain forbidden. Stable non-leaking errors and redacted Debug are required.

## 11. Test Plan Assessment

The future test plan covers:

- exact query derivation;
- omitted-query rejection;
- source-completeness provenance;
- grant and availability duplicates;
- lifecycle, expiry, revocation, and freshness;
- exact prerequisite facts;
- sensitivity composition;
- SideEffect non-authority;
- deterministic hashing and known vectors;
- serde tampering;
- privacy; and
- adjacent regression suites.

This is adequate for the first model phase.

## 12. Planning Blockers

None.

## 13. Non-Blocking Implementation Guardrails

- Expose no `authorize`, `permit`, `ready`, or `dereference` method from the
  model-only phase.
- Do not let public constructors or serde alone confer trusted
  `CompleteForExactQuery` provenance.
- Prefer a crate-private or source-owned trusted construction path.
- Keep accepted policy and approval wrappers explicit if current records lack
  exact scope.
- Add a fixed hash vector and framing regression in the first implementation.
- Keep the first model phase small enough to review; defer any source/store
  implementation.

## 14. Recommended Next Phase

Proceed to **required-context current authority fact-set core model only**.

Implement exact query derivation, payload-free source-completeness binding,
current grant and availability fact retention, bounded prerequisite fact
references, deterministic hashing, serde, privacy, and focused tests.

Do not implement authoritative readiness, dereference, executor integration,
persistence, events, schemas, CLI behavior, providers, OpenShell, sandbox
execution, SideEffects, writes, hosted behavior, reasoning lineage, or release
changes.

## 15. Dogfood Governance

- workflow: `dg/review`
- run ID: `run-1785141551362557000-2`
- approval ID:
  `approval/run-1785141551362557000-2/review-scope-approved`
- presentation ID: `presentation/a468414973f80231`
- approval outcome: granted under delegated-maintainer authority after the
  complete persisted review handoff was presented
- phase status: completed
- event summary: 39 events, one approval, zero retries, zero escalations;
  approval-presentation proof was persisted and enforced
- out-of-kernel work: plan inspection, review writing, roadmap updates, and
  validation were performed by the delegated maintainer; the kernel governed
  scope and approval but did not edit files, invoke tools, or mutate git
