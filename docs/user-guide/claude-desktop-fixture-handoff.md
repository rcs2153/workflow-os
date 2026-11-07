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
