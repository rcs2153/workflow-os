"""Behavior tests against the compiled Rust extension, with synthetic approvals."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from workflow_os import WorkflowOS, WorkflowError
from workflow_os import _native

ROOT = Path(__file__).resolve().parents[3]
FIXTURE = "example/github-repository-metadata-fixture"


class KernelTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.project, self.state = root / "project", root / "state"
        shutil.copytree(ROOT / "examples/github-repository-metadata-fixture", self.project)
        self.mode = "github_repository_metadata_fixture"
        self.client = self.new_client()

    def new_client(self):
        return WorkflowOS(self.project, self.state, mode=self.mode)

    def decide(self, run, deny=False, client=None):
        return (client or self.client).approve(run.run_id, run.approval_id,
            presentation_id=run.presentation["presentation_id"],
            actor="user/synthetic-test-reviewer", reason="Fixture-only integration test", deny=deny)

    def use_local(self, callback=None):
        shutil.rmtree(self.project)
        shutil.copytree(ROOT / "examples/vertical-slice-approval", self.project)
        self.mode = "local"
        self.client = self.new_client()
        if callback:
            self.client.register("local/rec", "v0", callback)

    def test_native_fixture_approval_restart_and_replay_without_subprocess(self):
        # Fail any Python process launch; the compiled module invokes Core directly.
        with patch("subprocess.Popen", side_effect=AssertionError("subprocess forbidden")), \
             patch("os.system", side_effect=AssertionError("shell forbidden")), \
             patch.dict(os.environ, {"PATH": ""}):
            self.client.validate()
            run = self.client.run(FIXTURE)
            self.assertEqual(run.status, "waiting_for_approval")
            self.assertIsNotNone(run.approval_id)
            self.assertTrue(run.presentation["approved_scope"])
            waiting = self.new_client().run(FIXTURE, run_id=run.run_id)
            self.assertEqual(waiting.events, run.events)
            approved = self.decide(waiting, client=self.new_client())
            self.assertNotEqual(approved.status, "completed")
            done = self.new_client().run(FIXTURE, run_id=run.run_id)
            self.assertEqual(done.status, "completed")
            self.assertIn("SkillInvocationSucceeded", json.dumps(done.events))
            self.assertEqual(done.provider_posture, "Completed")
            replay = self.new_client().run(FIXTURE, run_id=run.run_id)
            self.assertEqual(replay.provider_posture, "ExistingTerminal")
            self.assertEqual(done.events, replay.events)
            self.assertEqual(self.client.inspect(run.run_id).events, done.events)

    def test_fixture_denial_never_enters_provider(self):
        run = self.client.run(FIXTURE)
        denied = self.decide(run, deny=True)
        replay = self.client.run(FIXTURE, run_id=run.run_id)
        self.assertEqual(denied.events, replay.events)
        self.assertNotIn("SkillInvocationSucceeded", json.dumps(replay.events))

    def test_fixture_drift_rejected_before_approval(self):
        run = self.client.run(FIXTURE)
        file = self.project / "fixtures/github/repository.json"
        data = json.loads(file.read_text())
        data["default_branch"] = "changed"
        file.write_text(json.dumps(data))
        with self.assertRaises(WorkflowError):
            self.decide(run)
        self.assertEqual(self.client.inspect(run.run_id).events, run.events)

    def test_fixture_drift_rejected_after_approval(self):
        run = self.client.run(FIXTURE)
        self.decide(run)
        file = self.project / "fixtures/github/repository.json"
        file.write_text(file.read_text().replace('"main"', '"changed"'))
        with self.assertRaises(WorkflowError):
            self.client.run(FIXTURE, run_id=run.run_id)

    def test_callback_only_runs_after_human_decision_and_not_on_replay(self):
        calls = []
        def tool(values):
            calls.append(values)
            return {"summary": "Reviewed " + values["request"]}
        self.use_local(tool)
        run = self.client.run("ex/review")
        self.assertEqual(calls, [])
        resumed_client = self.new_client()
        resumed_client.register("local/rec", "v0", tool)
        done = self.decide(run, client=resumed_client)
        self.assertEqual(len(calls), 1)
        self.assertEqual(done.status, "completed")
        self.assertIn("SkillInvocationSucceeded", json.dumps(done.events))
        self.assertEqual(done.outputs[0]["values"]["summary"], "Reviewed internal-policy-exception-request")
        replay = resumed_client.run("ex/review", run_id=run.run_id)
        self.assertEqual(len(calls), 1)
        self.assertEqual(done.events, replay.events)
        self.assertEqual(replay.outputs, ())

    def test_local_denial_does_not_call_python(self):
        calls = []
        self.use_local(lambda values: calls.append(values) or {"summary": "ok"})
        run = self.client.run("ex/review")
        self.decide(run, deny=True)
        self.assertEqual(calls, [])

    def test_callback_exception_is_sanitized_in_durable_state(self):
        def fail(values):
            raise RuntimeError("secret-test-token-do-not-store")
        self.use_local(fail)
        run = self.client.run("ex/review")
        done = self.decide(run)
        evidence = json.dumps(done.events)
        self.assertIn("python.callback.failed", evidence)
        self.assertNotIn("secret-test-token-do-not-store", evidence)

    def test_invalid_callback_output_fails_closed(self):
        self.use_local(lambda _: {"summary": 123})
        run = self.client.run("ex/review")
        done = self.decide(run)
        self.assertIn("python.callback.failed", json.dumps(done.events))

    def test_missing_handler_cannot_complete(self):
        self.use_local()
        run = self.client.run("ex/review")
        done = self.decide(run)
        self.assertNotIn("SkillInvocationSucceeded", json.dumps(done.events))
        self.assertEqual(done.outputs, ())

    def test_local_workflow_drift_blocks_callback(self):
        calls = []
        self.use_local(lambda values: calls.append(values) or {"summary": "ok"})
        run = self.client.run("ex/review")
        path = self.project / "workflows/request-review.workflow.yml"
        path.write_text(path.read_text().replace("internal-policy-exception-request", "changed-input"))
        with self.assertRaises(WorkflowError):
            self.decide(run)
        self.assertEqual(calls, [])

    def test_reentrant_callback_rejected_without_deadlock(self):
        errors = []
        def callback(values):
            try:
                self.client.run("ex/review")
            except WorkflowError as error:
                errors.append(error.code)
            return {"summary": "ok"}
        self.use_local(callback)
        self.decide(self.client.run("ex/review"))
        self.assertEqual(errors, ["python.state.busy"])

    def test_unsupported_governance_is_not_silently_ignored(self):
        self.mode = "local"
        with self.assertRaises(WorkflowError) as caught:
            self.new_client().run(FIXTURE)
        self.assertEqual(caught.exception.code, "python.governance.unsupported")

    def test_registration_is_explicit_and_unique(self):
        self.use_local(lambda _: {"summary": "ok"})
        with self.assertRaises(ValueError):
            self.client.register("local/rec", "v0", lambda _: {})
        with self.assertRaises(ValueError):
            self.client.register("adapter/github", "v0", lambda _: {})

    def test_policy_denial_prevents_callback(self):
        calls = []
        self.use_local(lambda values: calls.append(values) or {"summary": "ok"})
        policy = self.project / "policies/local.policy.yml"
        policy.write_text(policy.read_text().replace("effect: allow_local", "effect: deny"))
        try:
            result = self.client.run("ex/review")
            self.assertNotEqual(result.status, "completed")
        except WorkflowError:
            pass
        self.assertEqual(calls, [])

    def test_wrong_approval_id_cannot_authorize_another_run(self):
        first = self.client.run(FIXTURE)
        second = self.client.run(FIXTURE)
        with self.assertRaises(WorkflowError):
            self.client.approve(second.run_id, first.approval_id,
                presentation_id=first.presentation["presentation_id"],
                actor="user/synthetic-test-reviewer", reason="Mismatched approval test")
        self.assertEqual(self.client.inspect(second.run_id).events, second.events)

    def test_local_approval_requires_the_presented_record_before_callback(self):
        calls = []
        def callback(values):
            calls.append(values)
            return {"summary": "ok"}
        self.use_local(callback)
        first = self.client.run("ex/review")
        second = self.client.run("ex/review")
        self.assertIn(first.presentation["step_id"], first.presentation["approved_scope"])
        self.assertTrue(any("local/rec" in surface
                            for surface in first.presentation["expected_touched_surfaces"]))
        fields = dict(run_id=first.run_id, approval_id=first.approval_id,
                      actor="user/synthetic-test-reviewer", reason="Reviewed local step")
        with self.assertRaisesRegex(WorkflowError, "missing required presentation_id"):
            self.client._request("approve", **fields)
        with self.assertRaises(WorkflowError):
            self.client.approve(**fields, presentation_id=second.presentation["presentation_id"])
        self.assertEqual(self.client.inspect(first.run_id).events, first.events)
        self.assertEqual(calls, [])
        self.assertEqual(self.decide(first).status, "completed")
        self.assertEqual(len(calls), 1)

    def test_validation_and_missing_inspection_do_not_create_state(self):
        for mode in ("github_repository_metadata_fixture", "local"):
            if mode == "local":
                self.use_local()
            self.client.validate()
            with self.assertRaises(WorkflowError):
                self.client.inspect("unknown-run")
            self.assertFalse(self.state.exists())

    def test_inspection_does_not_recreate_writer_lock_or_change_state(self):
        run = self.client.run(FIXTURE)
        (self.state / ".python-sdk.lock").unlink()
        database = self.state / "github-repository-metadata.sqlite3"
        before = database.read_bytes()
        self.assertEqual(self.client.inspect(run.run_id).events, run.events)
        self.assertEqual(database.read_bytes(), before)
        self.assertFalse((self.state / ".python-sdk.lock").exists())

    def test_separate_python_process_reopens_durable_state(self):
        run = self.client.run(FIXTURE)
        # This test launches a replacement harness; SDK calls themselves never spawn.
        code = """import json, sys
from workflow_os import WorkflowOS
kernel = WorkflowOS(sys.argv[1], sys.argv[2], mode='github_repository_metadata_fixture')
run = kernel.status(sys.argv[3])
print(json.dumps({'status': run.status, 'approval_id': run.approval_id}))
"""
        process = subprocess.run([sys.executable, "-c", code, str(self.project),
            str(self.state), run.run_id], check=True, text=True, capture_output=True)
        reopened = json.loads(process.stdout)
        self.assertEqual(reopened, {"status": run.status, "approval_id": run.approval_id})

    @unittest.skipUnless(os.environ.get("WORKFLOW_OS_TEST_CLI"), "set WORKFLOW_OS_TEST_CLI for CLI/native parity")
    def test_cli_and_native_fixture_state_interoperate(self):
        cli = os.environ["WORKFLOW_OS_TEST_CLI"]
        def invoke(*args):
            return subprocess.run([cli, "--project-dir", str(self.project),
                "--state-dir", str(self.state), *args], check=True,
                capture_output=True, text=True).stdout
        started = invoke("run", FIXTURE)
        run_id = next(line.split(": ", 1)[1] for line in started.splitlines() if line.startswith("run_id: "))
        waiting = self.client.status(run_id)
        self.decide(waiting)
        completed = invoke("run", FIXTURE, "--run-id", run_id)
        self.assertIn("status: Completed", completed)
        native = self.client.run(FIXTURE, run_id=run_id)
        self.assertEqual(native.provider_posture, "ExistingTerminal")
        # The reverse direction: Python's presentation is understood by CLI approval.
        waiting = self.client.run(FIXTURE)
        invoke("approve", waiting.run_id, waiting.approval_id,
            "--actor", "user/synthetic-test-reviewer", "--reason", "Native CLI parity test")
        self.assertEqual(self.client.run(FIXTURE, run_id=waiting.run_id).status, "completed")

    def test_native_request_rejects_unknown_fields(self):
        with self.assertRaises(_native.NativeError):
            _native.request('{"unexpected": true}', [])


if __name__ == "__main__":
    unittest.main(verbosity=2)
