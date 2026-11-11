"""Experimental synchronous, in-process bindings to the canonical Rust kernel."""
from __future__ import annotations

from dataclasses import dataclass
import json
from pathlib import Path
from typing import Callable, Literal, Mapping, Any

from . import _native

__all__ = ["WorkflowOS", "Run", "WorkflowError"]
Mode = Literal["local", "github_repository_metadata_fixture"]
Handler = Callable[[dict[str, str]], dict[str, str]]


class WorkflowError(RuntimeError):
    """A stable kernel error code and bounded error message."""

    def __init__(self, code: str, message: str):
        self.code = code
        super().__init__(message)


@dataclass(frozen=True)
class Run:
    """Kernel projection. Outputs are ephemeral, not durable completion evidence."""

    run_id: str
    status: str
    approval_id: str | None
    presentation: Mapping[str, Any] | None
    events: tuple[Mapping[str, Any], ...]
    outputs: tuple[Mapping[str, Any], ...]
    provider_posture: str | None
    snapshot: Mapping[str, Any]

    @classmethod
    def _from_native(cls, data: dict[str, Any]) -> Run:
        snapshot = data["run"]["snapshot"]
        presentation = data["presentation"]
        return cls(
            run_id=snapshot["identity"]["run_id"],
            status=snapshot["status"],
            approval_id=presentation["approval_id"] if presentation else None,
            presentation=presentation,
            events=tuple(data["run"]["events"]),
            outputs=tuple(data["outputs"]),
            provider_posture=data["provider_posture"],
            snapshot=snapshot,
        )


class WorkflowOS:
    """Explicit tool registration and durable workflows, without subprocesses.

    Callbacks are trusted application code, not sandboxed. The local approval
    operation resumes callbacks immediately, following Core's existing contract.
    Fixture approval only grants; call run again to execute the fixture adapter.
    One operation per state directory may run at a time through this SDK.
    """

    def __init__(self, project_dir: str | Path, state_dir: str | Path, *,
                 mode: Mode = "local", actor: str = "system/python-agent"):
        self.project_dir = str(Path(project_dir).resolve())
        self.state_dir = str(Path(state_dir).resolve())
        self.mode = mode
        self.actor = actor
        self._handlers: dict[tuple[str, str], Handler] = {}

    def register(self, skill_id: str, version: str, handler: Handler) -> None:
        """Register a synchronous trusted callback for a declared local skill."""
        if self.mode != "local":
            raise ValueError("Fixture mode does not accept callbacks")
        if not skill_id.startswith("local/") or not callable(handler):
            raise ValueError("Expected a local/ skill ID and a callable")
        key = (skill_id, version)
        if key in self._handlers:
            raise ValueError("This skill ID and version are already registered")
        self._handlers[key] = handler

    def _request(self, operation: str, **fields: Any) -> dict[str, Any]:
        request = dict(project_dir=self.project_dir, state_dir=self.state_dir,
                       mode=self.mode, actor=self.actor, operation=operation)
        request.update(fields)
        handlers = [(skill, version, handler)
                    for (skill, version), handler in self._handlers.items()]
        try:
            return json.loads(_native.request(json.dumps(request), handlers))
        except _native.NativeError as error:
            code, message = error.args
            raise WorkflowError(code, message) from None

    def validate(self) -> None:
        """Validate the project and reject unsupported local governance profiles."""
        self._request("validate")

    def run(self, workflow_id: str, *, run_id: str | None = None) -> Run:
        """Start a declared workflow or inspect/replay the same durable request."""
        return Run._from_native(self._request("run", workflow_id=workflow_id, run_id=run_id))

    def approve(self, run_id: str, approval_id: str, *, actor: str,
                reason: str, deny: bool = False) -> Run:
        """Record an explicit human decision. Do not expose this as a model tool.

        Local mode also resumes execution; fixture mode requires another run call.
        Actor is an audit label, not authenticated identity.
        """
        return Run._from_native(self._request("approve", run_id=run_id,
            approval_id=approval_id, actor=actor, reason=reason, deny=deny))

    def inspect(self, run_id: str) -> Run:
        """Read the canonical snapshot and ordered event history."""
        return Run._from_native(self._request("inspect", run_id=run_id))

    def status(self, run_id: str) -> Run:
        """Read the durable run projection."""
        return self.inspect(run_id)
