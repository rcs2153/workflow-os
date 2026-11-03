# Continuation Context Bounded Local Observers Report

## Status

Phase C implemented as a Core-private injected observation contract with
closed, bounded validation. Focused maintainer/security review is required
before durable accepted-intent capture may begin.

## Delivered

Core now has a private Phase C source set for three continuation source classes:

- Git/worktree observation;
- allowlisted operating-document observation; and
- typed handoff.

The Git/worktree contract carries only bounded typed metadata: repository
presence, payload-free head and worktree commitments, and tracked, untracked,
and conflict counts. It does not carry paths, diffs, commit messages, branch
names, repository contents, or command output.

The operating-document contract accepts exactly three typed document identities:

- `AGENTS.md`;
- `docs/ENGINEERING_STANDARD.md`; and
- `ROADMAP.md`.

Each observation carries only a payload-free content commitment, byte count,
path-safety attestations, and bounded sensitivity. Core canonicalizes document
ordering and rejects duplicates, unsafe path or symlink posture, oversized
documents, oversized aggregate observations, and unsupported sensitivity.

The typed-handoff collector remains explicitly `Unsupported`. The repository
does not yet expose a stable, non-consuming typed-handoff persistence interface,
and this phase does not invent one or derive handoff state from prose.

## Injection Boundary

This phase defines and validates an injected local-observer contract. It does
not add a production host implementation that invokes Git or reads files. The
injected producer is responsible for obtaining observations and attesting that
an allowlisted document resolved canonically beneath the project root without a
symlink traversal. Core independently validates the closed shape, request
binding, limits, and revision stability before accepting the observation.

This separation establishes the kernel-side trust boundary without silently
enabling local process or filesystem access. A future host producer must be
implemented and reviewed separately before the model can represent real local
observations.

## Revision And Drift Boundary

Observer requests are bound to continuation purpose, scope, actor, and a
payload-free observer contract commitment. The observer contract prevents
method substitution between anchor capture and collection, but does not
authenticate or authorize a future host producer. Anchor capture obtains source-specific
revisions for Git/worktree state and the canonicalized document set. Collection
re-observes the source against the same request and rejects any changed revision
rather than combining observations from incompatible moments.

The Phase C revisions are overlaid onto the complete Phase B anchor. Every
other collector revision remains unchanged, preserving one complete collection
boundary instead of introducing a second anchor system.

## Security Properties

- The implementation is crate-private and has no runtime consumer.
- Core owns the allowlist, limits, sensitivity ceiling, and typed result
  posture.
- Observations are bound to purpose, scope, and actor.
- The observer contract is bound across anchor and collection; producer trust
  remains a separate future host-registration responsibility.
- Revision drift fails closed.
- Git and document commitments are payload-free and redacted from debug output.
- Document identities are a closed enum, not caller-provided paths.
- Empty Git or document results retain typed negative-knowledge accounting.
- Missing stable handoff persistence is `Unsupported`, not empty or inferred.
- Collection cannot consume approval, capability, policy permission, or other
  authority.
- No shell command, Git process, filesystem read, provider call, state write,
  event append, report write, or external side effect is performed.

## Focused Coverage

Six focused tests prove:

- Git and operating-document observations produce typed observed posture while
  typed handoff remains explicitly unsupported;
- Git/worktree revision drift rejects collection against the captured anchor;
- observer-contract substitution rejects collection against the captured
  anchor;
- duplicate or path-unsafe document observations fail closed;
- document ordering does not change the source revision; and
- debug surfaces do not expose commitments or allowlisted paths.

## Governed Implementation

- workflow: `dg/implement`
- run ID: `run-1793707808478249000-2`
- approval ID:
  `approval/run-1793707808478249000-2/implementation-approved`
- presentation ID: `presentation/49d7c39dbfd1bde7`
- approval outcome: granted under delegated-maintainer authority after review
  of the complete persisted handoff

## Validation

- focused bounded-local-observer tests: 6 passed
- strict `workflow-core` clippy with warnings denied: passed with the existing
  narrow dead-code allowance for private future-facing models
- full workspace tests: passed
- rustdoc with warnings denied: passed
- formatting: passed
- documentation checks: passed
- diff and whitespace checks: passed
- focused maintainer/security review: accepted after the observer-contract
  substitution blocker was corrected

## Boundary Preserved

This phase did not implement a real Git or filesystem observer, add a typed
handoff store, capture accepted intent, compose an aggregate continuation
projection, expose CLI, schema, SDK, report, hook, or child-harness behavior,
consume authority, invoke a continuation runtime consumer, perform provider
work, add nested harness behavior, add Salsa, or expand hosted behavior.

## Out-Of-Kernel Work

The delegated maintainer implemented the private contract, validation,
fixtures, focused tests, roadmap update, and this report; ran validation; and
reviewed diagnostics outside the kernel. The kernel governed scope and approval
but did not edit files, execute tests, create commits, or mutate GitHub.
