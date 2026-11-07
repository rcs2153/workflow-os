# Claude Desktop Fixture Handoff

This evaluator integration validates a prepared, interrupted GitHub PR-comment
fixture from Claude Desktop on the same Mac. It does not post a comment,
authorize later dispatch, claim OS-level isolation, or require GitHub access.
The [fixture authority contract](../implementation-plans/local-fixture-replacement-authority-contract.md)
and [milestone plan](../implementation-plans/cross-harness-authoritative-continuation-milestone-plan.md)
govern the experiment.

## Operator Setup

1. Prepare a disposable project and state directory. Start the original run
   as Codex, and capture the exact delegated CLI input envelope (`request` and
   `fixture`) at the approved, pre-dispatch interruption boundary. Do not use
   production state, payloads, or credentials.
2. Provision the distinct Claude connector principal through the kernel's
   public `provider github-pr-comment fixture-authority` operator commands.
   Register a fresh 32-byte cryptographically random value over stdin and
   issue a short-lived grant for the exact attempted fixture. Keep the raw
   value in a mode-0600 regular file in a private operator directory outside
   the shared workspace. Do not put it in app configuration, tool arguments,
   prompts, shell history, or evidence.
3. In that same directory create a mode-0600 manifest. Pin the absolute paths
   to the exact reviewed kernel executable, disposable project, and state.
   Keep the build and directory under operator control throughout the trial.
   Add only named, intentionally prepared test cases. Example shape (replace
   the empty objects with the documented delegated CLI input for the case):

   ```json
   {
     "kernel": "/absolute/path/to/reviewed/workflow-os",
     "project": "/absolute/path/to/disposable/project",
     "state": "/absolute/path/to/disposable/state",
     "cases": [
       {
         "id": "exact",
         "grant": "grant/example",
         "principal_file": "principal",
         "input": {"request": {}, "fixture": {}}
       }
     ]
   }
   ```

4. In a separately governed trial, configure the local stdio server in Claude
   Desktop with an absolute Node 20+ executable and exactly two arguments:
   the absolute path to `scripts/claude-fixture-mcp.mjs` and the manifest's
   absolute path. Preserve unrelated user settings. Start a fresh chat without
   workspace, shell, filesystem, or unrelated tool access. If those surfaces
   cannot be excluded, stop and record the app isolation blocker.
5. Verify that Claude has exactly one Workflow OS tool:
   `verify_workflow_os_fixture`. Give Claude only a synthetic case ID and ask
   it to invoke the tool. The only accepted argument is an operator-allowlisted
   `case_id`; it cannot select paths, actors, credentials, or CLI flags.
   The bridge forwards the pinned input and proof via stdin to the public
   delegated CLI and returns a restricted set of result fields. It has no
   direct database, provider, operator, or shell interface.

## Corpus And Evidence

For a repeatable synthetic starting fixture, the opt-in Unix evaluator export
uses the existing Core integration test and a locally built CLI:

```sh
cargo build -p workflow-cli --bin workflow-os
WORKFLOW_OS_CLAUDE_FIXTURE_KERNEL="$PWD/target/debug/workflow-os" \
  cargo test -p workflow-core --test local_executor \
  fixture_continuation_validates_prepared_window_and_rejects_drift -- --exact --nocapture
```

It creates a fresh mode-0700 directory under the system temporary directory
and prints the manifest path after verifying the fixture through the real
delegated CLI. The baseline also prints the exact grant expiry. The files containing the manifest and
principal inputs are mode 0600. The exact grant lasts 210 seconds from issue;
prepare the app session before exporting and regenerate a fresh fixture if it
expires. An expired run is not made current by reusing its manifest.

This exporter uses Core execution, a replayed event prefix, and synthetic
check evidence to prepare the pre-dispatch state. It is not proof that a live
run can be suspended at that boundary. The manifest covers `exact` (run it
twice), `action-drift`, `target-drift`, `wrong-principal`, `missing-grant`,
`revoked-grant`, `expired-grant`, `denied-approval`, and `terminal-replay`.
The denied run has a durable proof-backed denial and no attempted effect; the
CLI rejects it at the missing-effect boundary. The completed run has a
synthetic attempted record but is rejected as no longer active. Neither run
has its own continuation grant; the calls present the original fixture
principal and try to reuse the positive case's grant. The exporter additionally
probes actor override rejection at the CLI; the bridge does not accept that field.

For a separate one-case manifest, prefix the export command with
`WORKFLOW_OS_CLAUDE_FIXTURE_VARIANT=changed-evidence` or
`WORKFLOW_OS_CLAUDE_FIXTURE_VARIANT=stale-window`. Both first verify the
unchanged request through the public delegated CLI. The first appends a
superseding failed check using the Core event store and verifies the original
grant no longer validates. The second waits approximately five minutes for
the real execution window to expire, verifies the window-specific error via
Core's same-actor validator, then verifies the delegated denial. The stale
grant also expires; the delegated result does not distinguish the two clocks.
Neither variant edits the database directly or changes the trusted clock.

The dispatch-reservation conflict remains a separate case. The fixture
opening discards its dispatch capability; the evaluator must use a test-only
Core path to reserve that exact attempt without dispatching. Do not insert a
reservation row or grant fixture tools dispatch authority to fabricate this
case. A missing case is not a pass.

Prepare cases for exact allow and repeat, denial, target and action drift,
missing/expired/revoked grants, wrong principal, stale window, changed
evidence, terminal replay, and blocked duplicate dispatch. In an isolated
operator view, compare public kernel inspection before and after validation;
record exact CLI version/build, app version, case IDs, returned posture and
counts, and zero provider effects. The repeated allow case is a repeatable
validation, not permission to repeat an external action. Collect unresolved
cases and any ambiguous execution evidence without inventing an outcome.

Revoke the test grants and principal through the operator interface and
remove the trial server configuration when finished. Never publish the
manifest, principal file, raw tool traffic, or private state. This test
authenticates the provisioned local connector, not an Anthropic account or
the application binary. The same OS user can access its own files; tool
restriction is not a defense against that user or an arbitrary same-user
process.

`node --test scripts/claude-fixture-mcp.test.mjs` verifies the stdio and
redaction boundary using a stand-in CLI. It is not a Claude Desktop result
or a substitute for the actual corpus.
