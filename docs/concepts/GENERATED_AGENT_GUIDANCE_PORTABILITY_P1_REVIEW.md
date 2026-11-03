# Generated Agent Guidance Portability P1 Review

Status: Accepted after one review-driven blocker correction.

## Review Scope

This focused maintainer review examined the generated `AGENTS.md` template,
both scaffold entry points, portability regression assertions, roadmap status,
and the bounded implementation report.

The review was governed by:

- Workflow: `dg/review`
- Run: `run-1793722513230123000-2`
- Approval: `approval/run-1793722513230123000-2/review-scope-approved`
- Approval presentation: `presentation/ec62c761cd1489dc`
- Approval outcome: granted under delegated-maintainer authority

## Finding

The initial correction removed the source-checkout-only quickstart reference,
but the shared template still instructed harness-only scaffold users to read
`.workflow-os/README.md`. `init-agent-harness` does not create that file. The
first patch therefore would have preserved a different dead local reference.

This was a blocking portability defect.

## Correction

Generated guidance is now context-aware without expanding either scaffold:

- `init-agent-harness` references only
  `.workflow-os/agent-harness-prompt.md`, which it creates.
- `init-repo-governance` references the local prompt and
  `.workflow-os/README.md`, both of which it creates.
- neither path emits the Workflow OS source-checkout-only quickstart path.

Tests assert both the generated text and the filesystem presence or absence of
the referenced setup files.

## Validation

- Focused harness-only scaffold test: passed.
- Focused repository-governance scaffold test: passed.
- Full `workflow-cli` integration suite: passed, 176 tests.
- Strict `workflow-cli` Clippy with warnings denied: passed.
- Rust formatting check: passed.
- Documentation check: passed.
- Diff hygiene check: passed.

## Boundary Review

The correction does not add files to either scaffold, introduce a remote
documentation dependency, broaden generated governance content, change managed
block preservation, or alter runtime, provider, continuation, hosted, schema,
or release behavior.

## Verdict

Accepted. Generated downstream agent guidance is now portable across both
scaffold entry points and refers only to repository-local artifacts created by
the selected command.

The next authoritative queue item remains provider-fixture-aware
`workflow-os next-action` routing.
