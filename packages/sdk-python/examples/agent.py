#!/usr/bin/env python3
"""The fixture agent, now using Rust Core in process instead of subprocesses."""
import argparse
from dataclasses import asdict
import json
from pathlib import Path

from workflow_os import WorkflowOS, WorkflowError

WORKFLOW = "example/github-repository-metadata-fixture"


def governed_read(kernel: WorkflowOS, run_id: str | None = None):
    """Expose this function to the model; keep approve in the human interface."""
    return kernel.run(WORKFLOW, run_id=run_id)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--project", type=Path, required=True)
    parser.add_argument("--state", type=Path, default=Path(".workflow-agent-state"))
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("start")
    for name in ("resume", "inspect", "review"):
        command = commands.add_parser(name)
        command.add_argument("run_id")
    args = parser.parse_args()
    kernel = WorkflowOS(args.project, args.state,
                        mode="github_repository_metadata_fixture")
    if args.command == "start":
        kernel.validate()
        result = governed_read(kernel)
    elif args.command == "resume":
        result = governed_read(kernel, args.run_id)
    elif args.command == "inspect":
        result = kernel.inspect(args.run_id)
    else:
        result = kernel.status(args.run_id)
        if result.approval_id is None:
            raise ValueError("This run is not waiting for approval")
        # Show every scope/context field returned by Rust before collecting a decision.
        print(json.dumps(result.presentation, indent=2))
        answer = input("Human decision [approve/deny/cancel]: ").strip().lower()
        if answer not in ("approve", "deny"):
            return
        reason = input("Reason: ").strip()
        result = kernel.approve(result.run_id, result.approval_id,
            presentation_id=result.presentation["presentation_id"],
            actor="user/local-demo-reviewer", reason=reason, deny=answer == "deny")
    print(json.dumps(asdict(result), indent=2))


if __name__ == "__main__":
    try:
        main()
    except (WorkflowError, ValueError) as error:
        raise SystemExit(f"Stopped: {error}") from None
