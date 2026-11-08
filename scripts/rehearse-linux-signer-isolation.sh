#!/usr/bin/env bash
set -euo pipefail

if [[ ${GITHUB_ACTIONS:-} != true || ${RUNNER_OS:-} != Linux || ${RUNNER_ENVIRONMENT:-} != github-hosted ]]; then
  echo "Signer rehearsal requires a disposable GitHub-hosted Linux runner." >&2
  exit 2
fi
if [[ $# != 1 || ! -f $1 || ! -x $1 ]]; then
  echo "Expected exactly one compiled Core test executable." >&2
  exit 2
fi

core_uid=61021
signer_uid=61022
harness_uid=61023
service_gid=61020
for uid in "$core_uid" "$signer_uid" "$harness_uid" 61024; do
  if getent passwd "$uid" >/dev/null; then
    echo "Rehearsal UID is already assigned." >&2
    exit 2
  fi
done
for gid in "$service_gid" "$harness_uid"; do
  if getent group "$gid" >/dev/null; then
    echo "Rehearsal GID is already assigned." >&2
    exit 2
  fi
done

umask 077
root=$(mktemp -d /tmp/workflow-os-signer.XXXXXXXX)
staging=
cleanup() {
  if [[ -n $staging ]]; then /usr/bin/rm -rf -- "$staging"; fi
  sudo -n /usr/bin/rm -rf -- "$root"
}
trap cleanup EXIT
sudo -n /usr/bin/chown 0:0 "$root"
sudo -n /usr/bin/chmod 0711 "$root"
sudo -n /usr/bin/install -o 0 -g 0 -m 0555 -- "$1" "$root/worker"

staging=$(mktemp -d /tmp/workflow-os-signer-spec.XXXXXXXX)
git archive --format=tar HEAD:dogfood/workflow-os-self-governance |
  /usr/bin/tar -xf - -C "$staging"
if [[ ! -f $staging/workflow-os.yml || -n $(find "$staging" ! -type d ! -type f -print -quit) ]]; then
  echo "Expected a regular, tracked synthetic project fixture." >&2
  exit 2
fi
sudo -n /usr/bin/install -d -o 0 -g 0 -m 0555 "$root/core-fixture"
sudo -n /usr/bin/cp -R -- "$staging" "$root/core-fixture/project"
sudo -n /usr/bin/chmod -R a=rX "$root/core-fixture/project"

run_worker() {
  local uid=$1 gid=$2 test=$3
  # Only system launchers run elevated. The test executable starts with no
  # elevated identity, supplementary groups, capabilities, or host network.
  sudo -n /usr/bin/unshare --net -- /usr/bin/setpriv \
    --reuid="$uid" --regid="$gid" --clear-groups --no-new-privs \
    --inh-caps=-all --ambient-caps=-all --bounding-set=-all \
    /usr/bin/env -i WORKFLOW_OS_SIGNER_REHEARSAL="$case_root" \
    WORKFLOW_OS_SIGNER_TRIAL="$trial" TMPDIR="$case_root/core-private" \
    /usr/bin/timeout --kill-after=2s 35s "$root/worker" --ignored --exact "$test" --test-threads=1
}

signer_test=provider_comment_transport::installation_signer_channel::linux_rehearsal::linux_rehearsal_signer_process
core_test=sqlite_state::operational_opening_store::tests::linux_rehearsal_core_process
replacement_test=sqlite_state::operational_opening_store::tests::linux_rehearsal_replacement_core_process
harness_test=provider_comment_transport::installation_signer_channel::linux_rehearsal::linux_rehearsal_untrusted_process
listed=$("$1" --list)
for test in "$signer_test" "$core_test" "$replacement_test" "$harness_test"; do
  if ! grep -Fqx "$test: test" <<<"$listed"; then
    echo "Missing exact rehearsal test: $test" >&2
    exit 2
  fi
done
for trial in success before-commit after-commit after-callback; do
  echo "Signer isolation rehearsal: $trial"
  case_root="$root/$trial"
  sudo -n /usr/bin/install -d -o 0 -g 0 -m 0711 "$case_root"
  sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0750 "$case_root/core-socket"
  sudo -n /usr/bin/install -d -o "$signer_uid" -g "$service_gid" -m 0750 "$case_root/signer-socket"
  sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0700 "$case_root/core-private"
  sudo -n /usr/bin/install -d -o "$signer_uid" -g "$service_gid" -m 0700 "$case_root/signer-private"
  sudo -n /usr/bin/install -o "$core_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/core-private/authority"
  sudo -n /usr/bin/install -o "$signer_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/signer-private/custody"
  run_worker "$signer_uid" "$service_gid" "$signer_test" &
  signer_pid=$!
  core_status=0
  run_worker "$core_uid" "$service_gid" "$core_test" || core_status=$?
  signer_status=0
  wait "$signer_pid" || signer_status=$?
  expected_core=0
  expected_signer=0
  case "$trial" in
    before-commit) expected_core=78 ;;
    after-commit) expected_core=79 ;;
    after-callback) expected_signer=77 ;;
  esac
  if ((core_status != expected_core || signer_status != expected_signer)); then
    echo "Unexpected process exit: Core=$core_status (expected $expected_core) signer=$signer_status (expected $expected_signer)" >&2
    exit 1
  fi
  if ((expected_core != 0)); then
    run_worker "$core_uid" "$service_gid" "$replacement_test"
  fi
  run_worker "$harness_uid" "$harness_uid" "$harness_test"
done
