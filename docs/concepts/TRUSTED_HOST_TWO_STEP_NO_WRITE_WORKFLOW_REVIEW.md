# Trusted-Host Two-Step No-Write Workflow Review

## 1. Executive Verdict

**Phase accepted after two blocker fixes; proceed to focused review of the
required-context immutable execution-binding model.**

The implementation proves the intended exact two-step no-write runtime slice:
Core owns topology and progression, SQLite commits each transition atomically,
restart derives the next operation from durable history, and one private v3
terminal report artifact describes the complete run. Review found two
integrity blockers in lifecycle event identity and durable-history validation.
Both were fixed and covered before acceptance.

## 2. Scope Verification

The phase stayed within the approved unpublished, local, closed-profile,
no-write boundary. It added no third profile, arbitrary command execution,
provider access, external mutation, public CLI behavior, schemas, examples,
hosted expansion, parallelism, branching, retries beyond the existing closed
semantics, nested harness execution, or release-posture change.

The host still executes only opaque operations prepared by Core. It cannot
choose a step, substitute a profile, alter successor ordering, or patch run
state.

## 3. Architecture Assessment

`TrustedHostTwoStepNoWriteProfileRegistry` admits only the accepted project
validation and docs-check profiles. The request carries explicit run and
immutable-bundle context but no caller-selected step or profile. Core validates
the exact workflow shape, derives the runnable step from durable history, and
selects the corresponding closed profile.

The implementation is appropriately narrow. It reuses the accepted immutable
bundle, local-check result, lifecycle, report, artifact, and SQLite boundaries
rather than creating parallel orchestration primitives.

## 4. Atomic Progression Assessment

A successful first operation commits the concrete local-check result, matching
invocation success, and exact `docs-check` schedule in one SQLite transaction.
It does not complete the run. A failed operation commits the result, matching
invocation failure, and terminal run failure without scheduling another step.
The second successful operation commits the final result, invocation success,
and run completion.

The process cannot observe a committed result without its matching lifecycle
transition, and it cannot schedule the second operation from process memory.

## 5. Restart And Replay Assessment

Fresh admission after the first commit rehydrates the exact immutable bundle
and event history, validates the legal prefix, and prepares only the docs-check
operation. Terminal replay executes neither check and appends no events. A
missing exact report artifact can be recovered through the existing create-only
artifact path; conflicting or unreadable artifacts remain fail-closed.

The bounded host loop performs at most two prepared operations. It is not a
generic scheduler, queue consumer, or daemon.

## 6. Report And Artifact Assessment

The distinct v3 report commitment preserves v2 one-step compatibility. The v3
report binds the exact workflow, immutable bundle, ordered steps, closed
profiles, concrete result events, matching invocation terminals, and terminal
run event. First-step failure honestly discloses that the docs check was not
attempted; second-step failure cites both attempted results.

Artifact creation continues through the accepted no-provider integration gate
and create-only SQLite store. Report or artifact failure cannot rewrite
terminal workflow truth.

## 7. Blockers Found And Fixed

### B1. Failure used the success-progression event domain

The lifecycle derived its completion-event domain only from configured success
progression. For a first-step failure, the emitted event was `RunFailed`, but
the deterministic event and idempotency domain was `step-scheduled`.

The fix makes event-domain selection outcome-aware. Only a successful
continuation uses `step-scheduled`; failures and terminal success use
`run-terminal`. Unit coverage proves both outcomes.

### B2. History validation accepted causally contradictory prefixes

The admission validator originally checked ordered step and result names plus
counts, but did not prove result-to-invocation identity, status compatibility,
or the causal order from schedule to result to invocation to next schedule.
A tampered durable history could therefore pass admission shape checks.

The fix requires every result to bind exactly one matching invocation terminal
event, requires status compatibility, and enforces:

```text
step scheduled < result recorded < invocation terminal < next step scheduled
```

A failed result cannot have a successor schedule or remain nonterminal. A
real-SQLite entry-point test corrupts result/invocation order, removes the
artifact, and proves fresh admission fails closed without changing events or
creating a replacement artifact.

## 8. Privacy And Error Assessment

The request and registry retain bounded custom `Debug` behavior. Reports and
events cite stable identifiers rather than command output. Errors and host
posture remain fixed and do not disclose paths, source/spec contents, parser or
provider payloads, process output, environment values, credentials,
authorization headers, private keys, or tokens.

## 9. Test Quality Assessment

Focused coverage proves:

- ordered two-step success and one v3 artifact;
- first-step failure without docs scheduling;
- second-step failure with both concrete results;
- restart after the first atomic commit;
- terminal replay without event mutation;
- caller-selected-step rejection before state creation;
- exact stable-reference report citations;
- missing, equal, conflicting, and unreadable artifact posture;
- lifecycle event-domain correctness for continuation and failure; and
- fail-closed causal-history corruption without artifact recreation.

The foreground admission suite contains 27 passing tests. Existing one-step
trusted-host behavior remains covered.

## 10. Validation

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test -p workflow-local-host --test foreground_admission`: passed, 27
  tests.
- `cargo test --workspace`: passed; explicitly opt-in live-provider tests
  remained ignored by design.
- `npm run check`: passed.
- `npm run check:integrations`: passed.
- `git diff --check`: passed.

## 11. Governed Review Record

- Dogfood workflow ID: `dg/review`
- Run ID: `run-1792670017396003000-2`
- Approval ID:
  `approval/run-1792670017396003000-2/review-scope-approved`
- Approval outcome: granted under delegated-maintainer authority with persisted
  presentation proof `presentation/1d5429d4f909f1a7`
- Phase status: `Completed`
- Event summary: 39 events, 1 approval, 0 retries, and 0 escalations.

The kernel governed the bounded review scope. The agent inspected code, edited
the blocker fixes and tests, ran validation, and prepared git changes outside
the kernel under that approved scope.

## 12. Remaining Limitations

- The exact workflow remains unpublished and private to the local host crate.
- Only two closed profiles are available.
- The host remains the process executor after Core prepares an opaque
  operation.
- Reports and artifacts have no released inspection or export surface.
- No provider, mutation, generic scheduler, parallel, branching, or nested
  harness capability is implied.

## 13. Non-Blocking Follow-Ups

- Add a reviewed fault-injection seam only if future crash-before-commit proof
  cannot be expressed through existing SQLite transaction tests.
- Keep any third profile or generic topology behind a separate plan and review.
- Preserve the v2 one-step report commitment indefinitely unless a separately
  planned migration exists.

## 14. Recommended Next Phase

Perform focused maintainer review of the implemented required-context immutable
execution-binding model already identified as awaiting review. This continues
the existing scoped-authority lane without broadening execution, adding a new
provider mutation, or starting another speculative model family.
