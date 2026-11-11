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
if getent group "$gid" >/dev/null || getent group 61023 >/dev/null; then
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
