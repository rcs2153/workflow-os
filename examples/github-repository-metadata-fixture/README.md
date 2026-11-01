# GitHub Repository Metadata Fixture

This example is the first public fixture-backed provider execution path. It
selects only `github.repository.metadata`, requires an authored approval, and
uses the repository fixture at `fixtures/github/repository.json`.

It does not read a token, open a network connection, call GitHub.com, or enable
provider writes.

```sh
workflow-os validate
workflow-os run example/github-repository-metadata-fixture
workflow-os approve <run-id> <approval-id> --actor user/reviewer --reason fixture-reviewed
workflow-os run example/github-repository-metadata-fixture --run-id <run-id>
workflow-os status <run-id>
workflow-os inspect <run-id>
```

The approval presentation binds the exact immutable workflow and fixture
content. Changing either before approval or execution fails closed.
