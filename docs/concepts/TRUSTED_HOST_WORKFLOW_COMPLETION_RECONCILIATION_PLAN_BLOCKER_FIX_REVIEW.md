# Trusted-Host Workflow Completion Reconciliation Planning Blocker Fix Review

## 1. Executive Verdict

**Planning blockers fixed; proceed to the bounded private Core and SQLite
implementation.**

The corrected plan removes the unrecoverable post-outcome crash window and
defines one coherent Core-owned invocation lifecycle. Opening owns invocation
request, dispatch owns invocation start, and attempt-outcome persistence owns
continuity outcome plus invocation and workflow terminal events. The
foreground application remains non-authoritative.

Implementation must normalize required-output validation before selecting the
terminal transaction outcome: invalid output follows the fixed terminal-failure
path and commits no success output reference.

## 2. Scope Verification

The blocker fix changed documentation only. It added no Rust runtime behavior,
state mutation, migration, scheduler, profile, provider, OpenShell, nested
harness, public API, write, example, or release behavior.

## 3. Lost-Result Blocker Assessment

The blocker is fixed in the plan.

The corrected design no longer commits `Succeeded` and then relies on a later
transaction that needs a potentially lost `SkillOutput`. The exact in-memory
executor result is consumed inside the attempt-outcome transaction. Continuity
state, invocation terminal state, and workflow terminal state commit together
or not at all.

Restart after committed success reads the complete durable event sequence; it
does not need the prior process memory. A crash before commit claims no
terminal result. Ambiguous commit reopens SQLite and validates the complete
exact sequence before reporting success.

## 4. Invocation-Lifecycle Blocker Assessment

The blocker is fixed in the plan.

- Core owns the invocation ID.
- Core derives the skill-attempt ID from the exact continuity attempt through
  a domain-separated commitment.
- Opening atomically appends invocation request and operational opening.
- Dispatch atomically appends dispatch admission and invocation start before
  executor entry.
- Outcome persistence atomically appends continuity outcome, invocation
  terminal, and run terminal events.
- Existing-run preparation must recover exactly one durable invocation request.
- Exact replay validates each complete associated event sequence.

The application cannot select or modify lifecycle identities or events.

## 5. Atomicity And Replay Assessment

Assigning lifecycle facts to the transaction that establishes each fact is the
correct boundary. It prevents orphan terminal invocation events, avoids a new
reusable completion capability, and reuses existing opening, dispatch, and
outcome operation records for exact replay.

The plan correctly avoids promising that no internal migration will be needed.
Implementation must first prove whether existing request JSON, event rows, and
idempotency keys bind all lifecycle facts. Any internal migration remains
separately visible in implementation scope and review; no public schema is
authorized.

## 6. Output Validation Condition

The implementation must validate required outputs against the immutable skill
definition before applying any terminal write set.

- valid success output selects `Succeeded`, carries only the bounded output
  reference, and commits the success sequence;
- invalid output selects the fixed `TerminalFailure` mapping, discards any
  success output reference, and commits the failure sequence; and
- validation failure must not leave continuity successful while the workflow
  fails or remains running.

This is a mandatory implementation condition, not a new planning blocker,
because the corrected transaction already owns both state machines and the
existing executor has established output-validation semantics.

## 7. Unsupported Outcome Assessment

The plan correctly leaves retryable, yielded, ambiguous, stale, mismatched,
and incomplete outcomes non-terminal at workflow level. It does not collapse
recovery posture into failure or rerun already-admitted work.

## 8. Privacy Assessment

The privacy posture remains sound:

- only bounded output references and fixed failure metadata may persist;
- raw output, commands, paths, environment values, payloads, credentials, and
  secrets are prohibited;
- lifecycle context remains crate-private and Debug-redacted; and
- the application still returns only fixed posture strings.

## 9. Test Plan Assessment

The revised plan now covers all blocker-sensitive proofs:

- the three atomic lifecycle transaction boundaries;
- Core-owned identity and ordering;
- crash injection and restart without in-memory output;
- exact and conflicting replay;
- partial/orphan lifecycle corruption;
- output-validation consistency;
- unsupported outcome non-terminal posture; and
- application output and error non-leakage.

Focused implementation tests must verify durable continuity records and the
workflow event stream together, not only returned application text.

## 10. Remaining Blockers

None for beginning the bounded implementation.

## 11. Non-Blocking Follow-Ups

- Determine during implementation whether existing SQLite records suffice or
  an explicit internal schema migration is necessary.
- Keep audit and observability sink delivery outside durable transaction truth
  and disclose its exact posture.
- Preserve the final executable-validation-to-process-entry race disclosure.
- Keep retry, ambiguity recovery, additional profiles, scheduling, providers,
  OpenShell, and nested harnesses separately scoped.

## 12. Recommended Next Phase

Implement the private trusted-host workflow lifecycle composition for the
existing one-step docs-check profile only. The phase may extend the existing
opening, dispatch, and attempt-outcome SQLite transactions and add focused
tests and reports. It must not add a public API or broaden runtime surfaces.

## 13. Governed Re-Review Record

- Workflow: `dg/review`
- Run: `run-1791507201771986000-2`
- Approval: `approval/run-1791507201771986000-2/review-scope-approved`
- Presentation proof: `presentation/30601cb95ae0234f`
- Approval outcome: granted under delegated maintainer authority
- Final status: `Completed`
- Event summary: 39 events, one approval, zero retries, zero escalations
- Presentation enforcement: proof enforced with one durable presentation record
- Validation: `npm run check:docs` passed; `git diff --check` passed

Repository inspection, review reasoning, documentation edits, and validation
were performed by the delegated maintainer outside the kernel. The kernel
governed scope, approval, sequencing, and durable review evidence.
