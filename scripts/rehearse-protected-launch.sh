#!/usr/bin/env bash
set -Eeuo pipefail
trap 'printf "Protected launch rehearsal failed at line %s\n" "$LINENO" >&2' ERR

if [[ ${GITHUB_ACTIONS:-} != true || ${RUNNER_ENVIRONMENT:-} != github-hosted || ${RUNNER_OS:-} != Linux ]]; then
  echo 'Protected launch rehearsal requires a disposable GitHub-hosted Linux runner.' >&2
  exit 2
fi
[[ $# == 1 && -f $1 && -x $1 ]]
binary=$(realpath -- "$1")
strace=$(command -v strace)
uid=61021
other_uid=61022
gid=61020
for actor in "$uid" "$other_uid" 61023 61025 61026; do
  if getent passwd "$actor" >/dev/null; then
    echo 'Rehearsal identities are already assigned.' >&2
    exit 2
  fi
done
if getent group "$gid" >/dev/null || getent group 61023 >/dev/null || getent group 61027 >/dev/null; then
  echo 'Rehearsal groups are already assigned.' >&2
  exit 2
fi
scratch=$(mktemp -d)
root=
cleanup() {
  if [[ -n $root ]]; then sudo -n rm -rf -- "$root"; fi
  rm -rf -- "$scratch"
}
trap cleanup EXIT
read -r root_owner root_mode < <(stat -c '%u:%g %a' /)
[[ $root_owner == 0:0 && $root_mode =~ ^[0-7]{3,4}$ ]]
(( (8#$root_mode & 07022) == 0 ))
if [[ -e /opt ]]; then stat -c 'ambient_opt_owner=%u:%g mode=%a' /opt; fi
root=$(sudo -n mktemp -d /workflow-os-launch-preflight.XXXXXXXX)
sudo -n chown 0:0 "$root"
sudo -n chmod 0755 "$root"
for actor in "$uid" "$other_uid" 61023 61025 61026; do
  sudo -n install -d -o "$actor" -g "$gid" -m 0700 "$root/trace-$actor"
done
sudo -n install -d -o 0 -g 0 -m 0700 "$root/custody"
sudo -n install -o 0 -g 0 -m 0555 "$binary" "$root/workflow-os"
sudo -n install -o 0 -g 0 -m 0555 "$binary" "$root/substitute"
exe_sha=$(sha256sum "$root/workflow-os")
exe_sha=${exe_sha%% *}
printf -v wrong_sha '%064d' 0
manifest=$root/launch.json
jq -n --arg digest "$exe_sha" '{
  version: "workflow-os/protected-launch/v1", executable_sha256: $digest, role: "core",
  identities: {core:{uid:61021,gid:61020},signer:{uid:61022,gid:61020},
    reader:{uid:61025,gid:61020},receiver:{uid:61026,gid:61020},harness:{uid:61023,gid:61023}}
}' > "$scratch/valid.json"
install_manifest() {
  sudo -n install -o 0 -g 0 -m 0444 "$1" "$manifest"
  sha=$(sha256sum "$manifest")
  sha=${sha%% *}
}
install_manifest "$scratch/valid.json"
stat -c 'protected_fixture_input: %F owner=%u:%g mode=%a links=%h' / "$root" "$manifest" "$root/workflow-os"
[[ $(stat -c '%F:%u:%g:%a' "$root") == directory:0:0:755 ]]
[[ $(stat -c '%F:%u:%g:%a:%h' "$manifest") == 'regular file:0:0:444:1' ]]
[[ $(stat -c '%F:%u:%g:%a:%h' "$root/workflow-os") == 'regular file:0:0:555:1' ]]
cd "$scratch"

run_case() {
  local code=$1 actor=$2 configuration=$3 digest=$4 program=${5:-$root/workflow-os}
  local inject=${6:-no} trace=$root/trace-$actor/syscalls
  local expected_role=${7:-core} actor_gid=${8:-$gid}
  local -a invocation=("$program" launch-preflight --manifest "$configuration" --sha256 "$digest")
  if [[ $inject == yes ]]; then
    invocation=(/bin/sh -c 'exec 9</dev/null; exec "$@"' sh "${invocation[@]}")
  fi
  local status=0
  printf 'protected_launch_case: %s\n' "$code"
  sudo -n /usr/bin/unshare --net -- /usr/bin/setpriv \
    --reuid="$actor" --regid="$actor_gid" --clear-groups --no-new-privs \
    --inh-caps=-all --ambient-caps=-all --bounding-set=-all \
    /usr/bin/env -i /usr/bin/timeout --kill-after=2s 30s "$strace" -qq -f -s 256 \
      -e trace=%network,open,openat,openat2,clone,clone3,fork,vfork -o "$trace" \
      "${invocation[@]}" </dev/null > "$scratch/out" 2> "$scratch/err" || status=$?
  if ! sudo -n test -s "$trace"; then
    printf 'Missing syscall trace for %s (exit %s)\n' "$code" "$status" >&2
    head -c 1024 "$scratch/err" >&2
    exit 1
  fi
  if ! sudo -n awk '
    {
      sub(/^[0-9]+ +/, "");
      if ($0 ~ /\/custody\//) { print "Unexpected custody access"; exit 1 }
      if ($0 !~ /^(open|openat|openat2)\(/) {
        split($0, parts, "[( ]");
        printf "Unexpected trace record: %.48s\n", parts[1];
        exit 1;
      }
    }' "$trace" >&2; then
    printf 'Syscall check failed for %s (exit %s)\n' "$code" "$status" >&2
    head -c 1024 "$scratch/err" >&2
    exit 1
  fi
  if [[ $code == success ]]; then
    if [[ $status != 0 || -s $scratch/err ]]; then
      printf 'Expected verified launch without stderr, got exit %s\n' "$status" >&2
      head -c 1024 "$scratch/err" >&2
      exit 1
    fi
    if ! jq -e --arg manifest "$digest" --arg executable "$exe_sha" --arg role "$expected_role" \
      --argjson uid "$actor" --argjson gid "$actor_gid" '
      .status == "verified_no_dispatch" and .role == $role and .uid == $uid and .gid == $gid
      and .manifest_sha256 == $manifest and .executable_sha256 == $executable
      and .unverified == ["deployment","custody","ingress","trusted_time","provider_scope"]
    ' "$scratch/out" >/dev/null; then
      echo 'Verified launch result did not match expected facts.' >&2
      exit 1
    fi
  elif [[ $status != 3 || -s $scratch/out ]] || ! grep -Fq "error[$code]:" "$scratch/err"; then
    printf 'Expected %s (exit 3), got exit %s\n' "$code" "$status" >&2
    head -c 1024 "$scratch/err" >&2
    exit 1
  fi
  [[ ! -e $scratch/.workflow-os && ! -e $root/.workflow-os ]]
  printf 'protected_launch_rehearsal: %s\n' "$code"
}

run_case success "$uid" "$manifest" "$sha"
run_case protected_launch.manifest.digest_mismatch "$uid" "$manifest" "$wrong_sha"
run_case protected_launch.identity.mismatch "$other_uid" "$manifest" "$sha"
run_case protected_launch.descriptors.unverified "$uid" "$manifest" "$sha" "$root/workflow-os" yes
run_case protected_launch.executable.mismatch "$uid" "$manifest" "$sha" "$root/substitute"
status=0
sudo -n "$root/workflow-os" launch-preflight --manifest "$manifest" --sha256 "$sha" > "$scratch/out" 2> "$scratch/err" || status=$?
[[ $status == 3 && ! -s $scratch/out ]]
grep -Fq 'error[protected_launch.identity.root]:' "$scratch/err"
sudo -n chmod 0644 "$manifest"
run_case protected_launch.file.unprotected "$uid" "$manifest" "$sha"
sudo -n chmod 0444 "$manifest"
sudo -n chown "$uid:$gid" "$manifest"
run_case protected_launch.file.unprotected "$uid" "$manifest" "$sha"
sudo -n chown 0:0 "$manifest"
sudo -n ln -s "$manifest" "$root/alias.json"
run_case protected_launch.file.unprotected "$uid" "$root/alias.json" "$sha"
sudo -n ln -s "$root" "$root/linked-directory"
run_case protected_launch.file.unprotected "$uid" "$root/linked-directory/launch.json" "$sha"
sudo -n ln "$manifest" "$root/hardlink.json"
run_case protected_launch.file.unprotected "$uid" "$manifest" "$sha"
sudo -n rm -- "$root/hardlink.json"
sudo -n chmod 0775 "$root"
run_case protected_launch.file.unprotected "$uid" "$manifest" "$sha"
sudo -n chmod 0755 "$root"
jq --arg digest "$wrong_sha" '.executable_sha256 = $digest' "$scratch/valid.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.executable.mismatch "$uid" "$manifest" "$sha"
jq --arg path "$root/custody/not-an-input" '.custody_path = $path' "$scratch/valid.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.manifest.invalid "$uid" "$manifest" "$sha"
jq '.identities.signer.uid = .identities.core.uid' "$scratch/valid.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.manifest.invalid "$uid" "$manifest" "$sha"
head -c 4097 /dev/zero > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.file.size_invalid "$uid" "$manifest" "$sha"

jq '.version = "workflow-os/protected-launch/v2" | del(.role)' \
  "$scratch/valid.json" > "$scratch/shared.json"
install_manifest "$scratch/shared.json"
shared_sha=$sha
for role in core signer reader receiver harness; do
  actor=$(jq -r --arg role "$role" '.identities[$role].uid' "$scratch/shared.json")
  actor_gid=$(jq -r --arg role "$role" '.identities[$role].gid' "$scratch/shared.json")
  run_case success "$actor" "$manifest" "$shared_sha" "$root/workflow-os" no "$role" "$actor_gid"
  printf 'protected_launch_shared_role: %s manifest_sha256=%s\n' "$role" "$shared_sha"
done
run_case protected_launch.identity.mismatch "$uid" "$manifest" "$shared_sha" "$root/workflow-os" no core 61023
run_case protected_launch.manifest.digest_mismatch "$uid" "$manifest" "$wrong_sha"
jq '.role = "core"' "$scratch/shared.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.manifest.invalid "$uid" "$manifest" "$sha"
jq '.identities.receiver.uid = .identities.core.uid' "$scratch/shared.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.manifest.invalid "$uid" "$manifest" "$sha"
jq '.version = "workflow-os/protected-launch/v1"' "$scratch/shared.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.manifest.invalid "$uid" "$manifest" "$sha"
jq '.version = "workflow-os/protected-launch/v3"' "$scratch/shared.json" > "$scratch/changed.json"
install_manifest "$scratch/changed.json"
run_case protected_launch.manifest.invalid "$uid" "$manifest" "$sha"

install_manifest "$scratch/shared.json"
[[ $sha == "$shared_sha" ]]
sudo -n install -o 0 -g 0 -m 0444 "$scratch/valid.json" "$root/v1.json"
v1_sha=$(sha256sum "$root/v1.json")
jq '.identities.core.gid = 61027' "$scratch/shared.json" > "$scratch/named.json"
sudo -n install -o 0 -g 0 -m 0444 "$scratch/named.json" "$root/named.json"
named_sha=$(sha256sum "$root/named.json")
sudo -n /usr/bin/unshare --net -- /usr/bin/python3 - "$root" "$shared_sha" "${v1_sha%% *}" "${named_sha%% *}" "$strace" <<'PY'
import errno
import json
import os
import select
import socket
import subprocess
import sys
import uuid

root, digest, v1_digest, named_digest, strace = sys.argv[1:]
worker = r"""
import errno
import json
import os
import socket
import subprocess
import sys

root, digest, v1_digest, named_digest, strace, mode, address, case = sys.argv[1:]
named = mode.startswith("named-")
shared = mode.startswith("named-shared-")
if named:
    mode = mode.removeprefix("named-")
if shared:
    mode = mode.removeprefix("shared-")
address = address if named else "\0" + address
conn = None
if mode == "access":
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as denied:
        denied.settimeout(5)
        try:
            denied.connect(address)
        except OSError as error:
            assert error.errno == errno.EACCES, error
        else:
            raise RuntimeError("unexpected endpoint connection")
    sys.exit(0)
if mode == "listen":
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as listener:
        listener.settimeout(15)
        listener.bind(address)
        listener.listen(1)
        print("ready", flush=True)
        conn, _ = listener.accept()
elif mode in ("connect", "connect-extra"):
    conn = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    conn.settimeout(15)
    conn.connect(address)
elif mode in ("datagram", "socket-output"):
    kind = socket.SOCK_DGRAM if mode == "datagram" else socket.SOCK_STREAM
    conn, other = socket.socketpair(socket.AF_UNIX, kind)
elif mode == "unconnected":
    conn = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

manifest = root + ("/named.json" if named and not shared else "/v1.json" if mode == "v1" else "/launch.json")
trace = f"{root}/trace-{os.getuid()}/peer-{case}.trace"
args = [
    strace, "-qq", "-f", "-s", "160",
    "-e", "trace=%network,read,readv,write,writev,open,openat,openat2,clone,clone3,fork,vfork",
    "-o", trace, root + "/workflow-os", "launch-peer-preflight",
    "--manifest", manifest, "--sha256", named_digest if named and not shared else v1_digest if mode == "v1" else digest,
]
if named:
    args.append("--named-endpoint")
extra = os.open("/dev/null", os.O_RDONLY) if mode == "connect-extra" else None
try:
    result = subprocess.run(
        args,
        stdin=conn if conn is not None else subprocess.DEVNULL,
        stdout=conn if mode == "socket-output" else subprocess.PIPE,
        stderr=subprocess.PIPE, text=True, env={}, timeout=30,
        pass_fds=() if extra is None else (extra,),
    )
finally:
    if extra is not None:
        os.close(extra)
if mode == "socket-output":
    conn.close()
    other.settimeout(5)
    if other.recv(1):
        raise RuntimeError("peer preflight emitted bytes to its socket")
print(json.dumps([result.returncode, result.stdout or "", result.stderr]))
"""


def argv(uid, gid, mode, address, case):
    return [
        "/usr/bin/setpriv", f"--reuid={uid}", f"--regid={gid}", "--clear-groups",
        "--no-new-privs", "--inh-caps=-all", "--ambient-caps=-all", "--bounding-set=-all",
        sys.executable, "-c", worker, root, digest, v1_digest, named_digest, strace, mode, address, case,
    ]


def check(result, label, expected):
    if isinstance(expected, str):
        code, stdout, stderr = result
        assert code == 3 and not stdout and f"error[{expected}]:" in stderr, (label, result)
    elif expected is None:
        assert result == [3, "", ""], (label, result)
    else:
        code, stdout, stderr = result
        assert code == 0 and not stderr, (label, result)
        evidence = json.loads(stdout)
        local, peer, uid, gid = expected
        named = label.startswith("endpoint-")
        assert evidence["status"] == ("endpoint_verified_no_dispatch" if named else "peer_verified_no_dispatch"), evidence
        assert evidence["launch"]["role"] == local, evidence
        assert evidence["launch"]["manifest_sha256"] == (named_digest if named else digest), evidence
        assert (evidence["peer_role"], evidence["peer_uid"], evidence["peer_gid"]) == (peer, uid, gid), evidence
    print("protected_launch_peer_case:", label, flush=True)


def run_pair(label, server_uid, server_gid, expected_client, expected_server):
    address = "workflow-os-peer-" + uuid.uuid4().hex
    with subprocess.Popen(
        argv(server_uid, server_gid, "listen", address, label + "-server"),
        stdout=subprocess.PIPE, text=True, env={},
    ) as server:
        try:
            assert select.select([server.stdout], [], [], 10)[0], "peer listener not ready"
            assert server.stdout.readline().strip() == "ready"
            client = subprocess.check_output(
                argv(61021, 61020, "connect", address, label + "-client"),
                text=True, env={}, timeout=35,
            )
            observed, _ = server.communicate(timeout=35)
            assert server.returncode == 0, observed
            check(json.loads(client), label + "-client", expected_client)
            check(json.loads(observed), label + "-server", expected_server)
        finally:
            if server.poll() is None:
                server.kill()


run_pair("matching", 61022, 61020, ("core", "signer", 61022, 61020), ("signer", "core", 61021, 61020))
run_pair("wrong-peer", 61023, 61023, "protected_launch.peer.identity_mismatch", "protected_launch.peer.role_unsupported")
run_pair("wrong-gid", 61022, 61023, "protected_launch.peer.identity_mismatch", "protected_launch.identity.mismatch")
for mode, expected in (
    ("null", "protected_launch.peer.transport_unverified"),
    ("datagram", "protected_launch.peer.transport_unverified"),
    ("unconnected", "protected_launch.peer.transport_unverified"),
    ("v1", "protected_launch.peer.role_unsupported"),
    ("socket-output", None),
):
    result = subprocess.check_output(argv(61021, 61020, mode, "", mode), text=True, env={}, timeout=35)
    check(json.loads(result), mode, expected)


def run_endpoint(label, expected_client, expected_server, *, mode=0o620, owner=0, wrong_path=False, extra=False, shared=False):
    label = "endpoint-" + label
    directory = root + "/peer"
    reviewed = directory + "/signer.sock"
    address = directory + "/other.sock" if wrong_path else reviewed
    core_gid = 61020 if shared else 61027
    prefix = "named-shared-" if shared else "named-"
    subprocess.run(["/usr/bin/install", "-d", "-o", "61022", "-g", "61020", "-m", "0700", directory], check=True)
    with subprocess.Popen(
        argv(61022, 61020, prefix + "listen", address, label + "-server"),
        stdout=subprocess.PIPE, text=True, env={},
    ) as server:
        try:
            assert select.select([server.stdout], [], [], 10)[0], "named listener not ready"
            assert server.stdout.readline().strip() == "ready"
            # Seal the directory before enabling access to the socket.
            os.chown(directory, 0, 0)
            os.chmod(directory, 0o711)
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as unused:
                if wrong_path:
                    unused.bind(reviewed)
                    os.chown(reviewed, 0, 61027)
                    os.chmod(reviewed, 0o620)
                os.chown(address, owner, core_gid)
                os.chmod(address, mode)
                if label == "endpoint-matching":
                    for uid, gid in [(61025, 61020), (61026, 61020), (61023, 61023)]:
                        subprocess.run(argv(uid, gid, "named-access", address, label), env={}, check=True, timeout=8)
                    print("protected_launch_peer_case: endpoint-unrelated-access-denied", flush=True)
                client = subprocess.check_output(
                    argv(61021, core_gid, prefix + ("connect-extra" if extra else "connect"), address, label + "-client"),
                    text=True, env={}, timeout=35,
                )
                observed, _ = server.communicate(timeout=35)
                assert server.returncode == 0, observed
                check(json.loads(client), label + "-client", expected_client)
                check(json.loads(observed), label + "-server", expected_server)
                # The path can remain after exit, but no listener remains.
                with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as stale:
                    stale.settimeout(5)
                    try:
                        stale.connect(address)
                    except OSError as error:
                        assert error.errno == errno.ECONNREFUSED, error
                    else:
                        raise RuntimeError("endpoint listener survived the probe")
        finally:
            if server.poll() is None:
                server.kill()
            subprocess.run(["/usr/bin/rm", "-f", "--", address, reviewed], check=True)
            subprocess.run(["/usr/bin/rmdir", "--", directory], check=True)
    print("protected_launch_peer_case:", label + "-retired", flush=True)


client_match = ("core", "signer", 61022, 61020)
server_match = ("signer", "core", 61021, 61027)
endpoint_denial = "protected_launch.endpoint.unverified"
run_endpoint("matching", client_match, server_match)
run_endpoint("wrong-mode", endpoint_denial, endpoint_denial, mode=0o622)
run_endpoint("wrong-owner", endpoint_denial, endpoint_denial, owner=61021)
run_endpoint("wrong-path", endpoint_denial, endpoint_denial, wrong_path=True)
run_endpoint("shared-group", endpoint_denial, endpoint_denial, shared=True)
run_endpoint("extra-fd", "protected_launch.descriptors.unverified", server_match, extra=True)
PY
[[ ! -e "$scratch/.workflow-os" && ! -e "$root/.workflow-os" ]]
[[ $(sudo -n find "$root" -name 'peer-*.trace' -type f -size +0c | wc -l) -eq 23 ]]
sudo -n find "$root" -name 'peer-*.trace' -type f -exec awk '
  {
    sub(/^[0-9]+ +/, "");
    if ($0 ~ /\/custody\// || $0 ~ /^(read|readv|write|writev)\(0,/ ||
        $0 !~ /^(read|readv|write|writev|open|openat|openat2|getsockopt|getpeername|getsockname)\(/) {
      print "Unexpected peer preflight syscall: " $0;
      bad = 1;
    }
  }
  END { exit bad }
' {} +
printf 'protected_launch_peer_rehearsal: passed\n'

sudo -n /usr/bin/unshare --net -- /usr/bin/python3 - "$root" "$shared_sha" "$strace" <<'PY'
import json
import os
from pathlib import Path
import re
import subprocess
import sys

root, digest, strace = sys.argv[1:]
receiver = 61026
gid = 61020
inputs = Path(root, "receiver-fixtures")
inputs.mkdir(mode=0o700)
for name, size, owner, mode in [
    ("first", 32, receiver, 0o400),
    ("second", 32, receiver, 0o400),
    ("bad-mode", 32, receiver, 0o440),
    ("bad-owner", 32, 0, 0o400),
    ("bad-size", 31, receiver, 0o400),
]:
    path = inputs / name
    path.write_bytes(bytes(size))
    os.chown(path, owner, gid)
    path.chmod(mode)

worker = r"""
import fcntl
import os
import sys

strace, trace, binary, manifest, digest, option, *sources = sys.argv[1:]
sources = [int(fd) for fd in sources]
copies = [fcntl.fcntl(fd, fcntl.F_DUPFD_CLOEXEC, 20) for fd in sources]
for fd in set(sources):
    os.close(fd)
for dest, fd in enumerate(copies, 3):
    os.dup2(fd, dest, inheritable=True)
    os.close(fd)
args = [strace, '-qq', '-f', '-yy', '-s', '128',
        '-e', 'trace=%network,read,readv,pread64,preadv,preadv2,write,writev,mmap,open,openat,openat2,clone,clone3,fork,vfork',
        '-o', trace, binary, 'launch-preflight', '--manifest', manifest, '--sha256', digest]
if option == 'receiver':
    args.append('--receiver-descriptors')
os.execv(strace, args)
"""


def probe(label, expected, files, uid=receiver, option="receiver"):
    trace = Path(root, f"trace-{uid}", f"receiver-{label}.trace")
    fds = [os.open(inputs / name, flags) for name, flags in files]
    try:
        result = subprocess.run(
            ["/usr/bin/setpriv", f"--reuid={uid}", f"--regid={gid}", "--clear-groups",
             "--no-new-privs", "--inh-caps=-all", "--ambient-caps=-all", "--bounding-set=-all",
             "/usr/bin/python3", "-c", worker, strace, str(trace), f"{root}/workflow-os",
             f"{root}/launch.json", digest, option, *map(str, fds)],
            env={}, pass_fds=fds, stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=35,
        )
    finally:
        for fd in fds:
            os.close(fd)
    assert trace.is_file() and trace.stat().st_size, (label, result.stderr)
    for line in trace.read_text().splitlines():
        # fdinfo is kernel metadata; only the synthetic input directory denotes material.
        assert str(inputs) not in line, (label, "input access syscall")
        syscall = re.sub(r"^\d+\s+", "", line).split("(", 1)[0]
        assert syscall in {"read", "readv", "pread64", "preadv", "preadv2", "write", "writev",
                           "mmap", "open", "openat", "openat2"}, (label, syscall)
    if expected is None:
        assert result.returncode == 0 and not result.stderr, (label, result.stderr)
        report = json.loads(result.stdout)
        assert report.keys() == {"status", "launch", "descriptor_count"}
        assert report["status"] == "receiver_descriptor_metadata_verified_no_dispatch"
        assert report["descriptor_count"] == 2
        launch = report["launch"]
        assert launch["role"] == "receiver" and launch["uid"] == uid and launch["gid"] == gid
        assert launch["manifest_sha256"] == digest and "custody" in launch["unverified"]
    else:
        assert result.returncode == 3 and not result.stdout, (label, result.returncode)
        assert f"error[{expected}]:" in result.stderr and str(inputs) not in result.stderr, (
            label, result.stderr,
        )
    print("protected_launch_receiver_case:", label, flush=True)


valid = [("first", os.O_RDONLY), ("second", os.O_RDONLY)]
invalid = "protected_launch.receiver.descriptors_unverified"
probe("matching", None, valid)
probe("missing", invalid, valid[:1])
probe("extra", "protected_launch.descriptors.unverified", valid + valid[:1])
probe("default-denies", "protected_launch.descriptors.unverified", valid, option="default")
probe("wrong-role", "protected_launch.receiver.role_unsupported", valid, uid=61025)
probe("alias", invalid, valid[:1] * 2)
probe("writable", invalid, [("first", os.O_RDWR), valid[1]])
probe("path-only", invalid, [("first", os.O_PATH), valid[1]])
for name in ["bad-mode", "bad-owner", "bad-size"]:
    probe(name, invalid, [(name, os.O_RDONLY), valid[1]])
PY
[[ ! -e "$scratch/.workflow-os" && ! -e "$root/.workflow-os" ]]
printf 'protected_launch_receiver_rehearsal: passed\n'
