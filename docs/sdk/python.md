# Native Python SDK (experimental)

The `workflow-os-native` package exposes `workflow_os.WorkflowOS` through a
PyO3 extension linked directly to `workflow-core`. Runtime calls do not launch
the CLI, another Python interpreter, a daemon, or a subprocess. Rust still owns
validation, policy, approvals, immutable definitions, execution, and durable
state. Python supplies ergonomics and explicitly registered trusted callbacks.

## Install from source

From `packages/sdk-python`, using CPython 3.10 or newer and a Rust toolchain
capable of building the locked workspace dependencies:

```sh
python3 -m venv .venv
. .venv/bin/activate
python -m pip install 'maturin>=1.9,<2'
maturin develop --locked
```

For this checkout, Rust 1.96 has been exercised. The workspace's declared
minimum Rust version alone is not evidence that the current lockfile builds
on that version. No wheel or package publication is part of this change. The current Core
platforms are Unix; this change does not claim Windows support.
The Python package uses PEP 440 version `0.4.0.dev1` for the Rust preview identity.

## The same fixture agent, without subprocesses

The [agent example](../../packages/sdk-python/examples/agent.py) keeps the
start, review, resume, and inspect flow of the CLI-based example. From the
repository root, with the package installed:

```sh
python packages/sdk-python/examples/agent.py \
  --project examples/github-repository-metadata-fixture start
```

Copy the resulting `run_id`, then use `review RUN_ID`, `resume RUN_ID`, and
`inspect RUN_ID` with the same project and state directory. The wrapper is now:

```python
from workflow_os import WorkflowOS

kernel = WorkflowOS(
    project_dir="examples/github-repository-metadata-fixture",
    state_dir=".workflow-agent-state",
    mode="github_repository_metadata_fixture",
)
run = kernel.run("example/github-repository-metadata-fixture")
print(run.run_id, run.status, run.presentation)
# Present the complete presentation to a human. Only after their decision:
run = kernel.approve(run.run_id, run.approval_id,
                     actor="user/reviewer", reason="Reviewed exact fixture read")
run = kernel.run("example/github-repository-metadata-fixture", run_id=run.run_id)
print(kernel.inspect(run.run_id).events)
```

This mode uses the existing exact GitHub fixture composition, immutable fixture
commitment, proof-bound approval, SQLite state, and terminal replay. It reads no
credentials and performs no network call. It does not expose arbitrary GitHub
operations. It can inspect current-schema fixture state produced by the CLI
when explicitly pointed at that same state directory. Older SQLite schemas
must be upgraded separately with the documented CLI migration commands.

## Route a Python tool through Core

Local mode connects a Python callable to the existing Rust `SkillHandler`
interface. The workflow and skill must already be declared and validated in
YAML. Registration itself does not grant permission or activate a workflow.

```python
from workflow_os import WorkflowOS

kernel = WorkflowOS("examples/vertical-slice-approval", ".python-local-state")

def recommend(values: dict[str, str]) -> dict[str, str]:
    return {"summary": "Reviewed " + values["request"]}

kernel.register("local/rec", "v0", recommend)
run = kernel.run("ex/review")  # Callback has not executed: approval is required.
# Human-facing application displays run.presentation and obtains a decision.
run = kernel.approve(run.run_id, run.approval_id,
                     actor="user/reviewer", reason="Reviewed local recommendation")
print(run.outputs)  # Callback runs inside Core's approved execution path.
```

A replacement Python process reopens the same state directory, registers the
same trusted handlers, and uses the same run ID. Do not create a new run for an
old logical request merely because the harness restarted.

**Approval semantics differ by mode:** the existing local executor's
`approve()` operation also executes/resumes the approved local workflow. The
GitHub fixture composition grants approval first and executes on the next
`run()` call. This SDK preserves those Core semantics; it does not invent a
separate generic resume operation. An existing local `run()` projects/replays
state rather than redispatching interrupted arbitrary callbacks.

## API and result contract

- `WorkflowOS(project_dir, state_dir, mode="local", actor="system/python-agent")`
  pins explicit project/state paths and an audit actor label.
- `register(skill_id, version, handler)` accepts one synchronous callable per
  `local/` skill and version. Duplicate registration is rejected. Callbacks
  receive and return `dict[str, str]`, matching Core's current skill boundary.
- `validate()` uses the canonical Rust project validator.
- `run(workflow_id, run_id=None)` starts or reopens the explicit declared work.
- `approve(run_id, approval_id, actor=..., reason=..., deny=False)` records the
  human decision through the proof-enforced Core API. A nonempty reason is required.
- `inspect(run_id)` and `status(run_id)` return the durable projection.
- `Run` exposes `run_id`, `status`, pending `approval_id`, full `presentation`,
  ordered `events`, `snapshot`, ephemeral callback `outputs`, and fixture
  `provider_posture`. Status/event serialization is Rust's preview contract.
- `WorkflowError.code` preserves a stable kernel/binding error code. Callback
  exception messages and tracebacks are not copied into durable evidence.

Callback outputs are returned to the current caller for convenience; they are
not durable report artifacts. Terminal replay returns no old callback payload.
A callback may run and then fail output validation; outputs are not proof of
successful completion. Check the kernel status and events. Persist sensitive
application results outside the audit log under an application-owned contract.

Local mode rejects project-level governance profiles instead of silently
ignoring authoritative-execution requirements it does not implement. Use an
explicitly supported Core composition for advanced governance or providers.

## Trust and concurrency boundary

This is a synchronous, trusted-host embedding API, not a sandbox or an
untrusted Python plugin runtime. Python and native code share a process and OS
permissions. Registered handlers remain `RegisteredUnattested`; a matching
skill version does not attest callback bytecode, dependencies, or credentials.
Do not grant an untrusted model arbitrary callback registration, project
mutation, state mutation, direct provider access, or the approval API.

The application must display the entire returned approval presentation and
collect a real human decision. Persisting the presentation records the SDK's
handoff to the application; it does not prove an authenticated human viewed a
UI. Actor strings are audit labels, not identity authentication.

One SDK operation per state directory is admitted at a time using a
nonblocking file lock. Concurrent/reentrant calls fail with `python.state.busy`
rather than waiting while holding the Python interpreter lock. This serializes
cooperating SDK callers only; CLI and other Core users do not take this wrapper
lock. Do not mix concurrent writers from those surfaces. Core retains its own
backend checks. In-process callbacks cannot be forcibly killed or timed out by
this wrapper and may block the caller. No asyncio integration is provided.

No generic provider writes, arbitrary-tool interception, cross-process callback
attestation, recovery from uncertain external effects, or production readiness
is claimed. Existing local callback execution is not an exactly-once external
side-effect protocol. Never retry an uncertain external write based solely on
this local SDK's result.

## Validation

With the native package installed, from the repository root:

```sh
python -m unittest discover -s packages/sdk-python/tests -v
cargo fmt --all --check
cargo clippy -p workflow-python --all-targets -- -D warnings
```

The tests exercise real Rust approval/denial, restart projections, immutable
input drift, callback failures, missing handlers, reentrancy, terminal replay,
and fixture execution with Python subprocess entry points disabled. Approvals
in tests are explicitly synthetic. The example itself requires human input.
