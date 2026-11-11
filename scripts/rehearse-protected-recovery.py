"""Run the real Linux CLI against disposable, exported synthetic state only."""

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def protected_json(path, value):
    path.write_text(json.dumps(value), encoding="utf-8")
    path.chmod(0o444)
    return digest(path)


def rehearse(root, binary, cases):
    uid, gid = 61021, 61020
    root.chmod(0o755)
    cli = root / "workflow-os"
    shutil.copyfile(binary, cli)
    cli.chmod(0o555)
    launch = root / "launch.json"
    launch_sha = protected_json(launch, {
        "version": "workflow-os/protected-launch/v2",
        "executable_sha256": digest(cli),
        "identities": {
            name: {"uid": identity, "gid": gid}
            for name, identity in [("core", uid), ("signer", 61022), ("reader", 61025),
                                   ("receiver", 61026), ("harness", 61023)]
        },
    })
    traces = root / "trace"
    traces.mkdir(mode=0o700)
    os.chown(traces, uid, gid)
    signer_traces = root / "signer-trace"
    signer_traces.mkdir(mode=0o700)
    os.chown(signer_traces, 61022, gid)
    database = root / "recovery.sqlite3"
    review = root / "recovery.json"

    def probe(label, review_sha, *, expected=None, source=None, actor=uid):
        trace = (traces if actor == uid else signer_traces) / label
        before = digest(database) if database.is_file() else None
        files_before = sorted(root.iterdir())
        result = subprocess.run([
            "/usr/bin/unshare", "--net", "--", "/usr/bin/setpriv", f"--reuid={actor}",
            f"--regid={gid}", "--clear-groups", "--no-new-privs", "--inh-caps=-all",
            "--ambient-caps=-all", "--bounding-set=-all", "/usr/bin/strace", "-qq", "-f",
            "-e", "trace=%network,clone,clone3,fork,vfork", "-o", str(trace),
            str(cli), "launch-recovery-preflight", "--manifest", str(launch),
            "--sha256", launch_sha, "--recovery-sha256", review_sha,
        ], cwd=root, env={}, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=35)
        assert trace.read_text(encoding="utf-8") == "", (label, "unexpected syscall")
        assert sorted(root.iterdir()) == files_before, (label, "filesystem changed")
        assert (digest(database) if database.is_file() else None) == before, (label, "state changed")
        if expected is not None:
            assert result.returncode == 3 and not result.stdout, (label, result.returncode)
            assert f"error[protected_launch.{expected}]:" in result.stderr, (label, result.stderr)
            assert str(root) not in result.stderr, (label, "path disclosed")
        else:
            assert result.returncode == 0 and not result.stderr, (label, result.stderr)
            output = json.loads(result.stdout)
            assert output == {
                "status": "synthetic_recovery_verified_no_dispatch",
                "launch": {
                    "status": "verified_no_dispatch", "role": "core", "uid": uid, "gid": gid,
                    "manifest_sha256": launch_sha, "executable_sha256": digest(cli),
                    "unverified": ["deployment", "custody", "ingress", "trusted_time", "provider_scope"],
                },
                "recovery_manifest_sha256": review_sha, "snapshot_sha256": before,
                "execution_posture": "completion_verified" if source else "reconciliation_required",
                "completion_proof_source": source, "dispatch_authorized": False, "provider_called": False,
            }, (label, output)
        print("protected_synthetic_recovery_case:", label, flush=True)

    for label, source in [("unresolved", None), ("completed", "lookup"),
                          ("response-completed", "creation_response")]:
        case = cases[label]
        if database.exists():
            database.unlink()
        shutil.copyfile(Path(case["state"]) / "github-pr-comment-continuation.sqlite3", database)
        database.chmod(0o444)
        binding = {
            "version": "workflow-os/protected-synthetic-recovery/v1",
            "launch_sha256": launch_sha, "snapshot_sha256": digest(database),
            **{name: case[name] for name in ["run_id", "operation_id", "side_effect_id"]},
        }
        review_sha = protected_json(review, binding)
        probe(label, review_sha, source=source)

    probe("wrong-role", review_sha, expected="recovery.role_unsupported", actor=61022)
    probe("unreviewed-manifest", "0" * 64, expected="recovery.manifest_unverified")
    wrong_launch = protected_json(review, {**binding, "launch_sha256": "0" * 64})
    probe("launch-drift", wrong_launch, expected="recovery.manifest_unverified")
    wrong_snapshot = protected_json(review, {**binding, "snapshot_sha256": "0" * 64})
    probe("snapshot-drift", wrong_snapshot, expected="recovery.snapshot_unverified")
    wrong_run = protected_json(review, {**binding, "run_id": "run-unrelated"})
    probe("wrong-run", wrong_run, expected="recovery.proof_unverified")
    review_sha = protected_json(review, binding)
    for suffix in ["-wal", "-shm", "-journal"]:
        sidecar = Path(str(database) + suffix)
        sidecar.touch()
        probe(f"unexpected{suffix}", review_sha, expected="recovery.snapshot_unverified")
        sidecar.unlink()
    database.unlink()
    probe("missing-snapshot", review_sha, expected="file.unprotected")


def main():
    binary, *manifests = map(Path, sys.argv[1:])
    cases = {}
    for manifest in manifests:
        data = json.loads(manifest.read_text(encoding="utf-8"))
        assert data["mode"] == "synthetic-recovery"
        for case in data["cases"]:
            assert Path(case["state"]).parent == manifest.parent
            assert case["id"] not in cases
            cases[case["id"]] = case
    assert set(cases) == {"unresolved", "completed", "response-completed"}
    with tempfile.TemporaryDirectory(prefix="workflow-os-protected-recovery.", dir="/") as root:
        rehearse(Path(root), binary, cases)
    print("protected_synthetic_recovery_rehearsal: passed")


if __name__ == "__main__":
    main()
