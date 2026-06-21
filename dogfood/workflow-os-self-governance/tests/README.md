# Self-Governance Dogfood Tests

No executable Workflow OS `.test.yml` specs are defined for the dogfood project yet.

The current multi-step dogfood conversion is covered by repository-level CLI tests and focused core executor tests. Coverage includes planning approval pause/grant/denial, cancellation while waiting on planning approval, duplicate run-id rehydration, and report-bearing dogfood execution through existing explicit APIs.

The implementation, maintainer review, PR hygiene, runtime composition, blocker-fix, release hygiene, branch cleanup, and workflow discovery dogfood workflows are currently covered by project validation. They are governed checklist workflows only: they do not run git commands, inspect GitHub, resolve conflicts, push branches, open PRs, delete local branches, delete remote branches, generate workflow files, register workflows, tag releases, publish packages, execute arbitrary commands, or replace maintainer review.

This directory exists so the dogfood project layout is explicit. Future self-governed validation/check planning may add `.test.yml` files once test execution semantics are scoped.
