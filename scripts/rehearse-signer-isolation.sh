#!/usr/bin/env bash
set -euo pipefail

if [[ ${GITHUB_ACTIONS:-} != true || ${RUNNER_ENVIRONMENT:-} != github-hosted ||
      (${RUNNER_OS:-} != Linux && ${RUNNER_OS:-} != macOS) ]]; then
  echo "Signer rehearsal requires a disposable GitHub-hosted Linux or macOS runner." >&2
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
id_is_assigned() {
  if [[ $RUNNER_OS == Linux ]]; then
    getent "$1" "$3" >/dev/null
  else
    local entry
    entry=$(/usr/bin/dscacheutil -q "$2" -a "$4" "$3") || exit 2
    [[ -n $entry ]]
  fi
}
for uid in "$core_uid" "$signer_uid" "$harness_uid" 61024; do
  if id_is_assigned passwd user "$uid" uid; then
    echo "Rehearsal UID is already assigned." >&2
    exit 2
  fi
done
for gid in "$service_gid" "$harness_uid"; do
  if id_is_assigned group group "$gid" gid; then
    echo "Rehearsal GID is already assigned." >&2
    exit 2
  fi
done

umask 077
tmp=/tmp
if [[ $RUNNER_OS == macOS ]]; then tmp=/private/tmp; fi
root=$(mktemp -d "$tmp/workflow-os-signer.XXXXXXXX")
staging=
cleanup() {
  if [[ -n $staging ]]; then /usr/bin/rm -rf -- "$staging"; fi
  sudo -n /usr/bin/rm -rf -- "$root"
}
trap cleanup EXIT
sudo -n /usr/bin/chown 0:0 "$root"
sudo -n /usr/bin/chmod 0711 "$root"
sudo -n /usr/bin/install -o 0 -g 0 -m 0555 -- "$1" "$root/worker"
if [[ $RUNNER_OS == macOS ]]; then
  sudo -n /usr/bin/install -o 0 -g 0 -m 0444 -- scripts/signer-rehearsal-macos.sb "$root/network.sb"
fi

staging=$(mktemp -d "$tmp/workflow-os-signer-spec.XXXXXXXX")
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
  local -a fixture_env=(
    "WORKFLOW_OS_SIGNER_REHEARSAL=$case_root" "WORKFLOW_OS_SIGNER_TRIAL=$trial"
    "TMPDIR=$case_root/core-private"
  )
  local -a args=(--ignored --exact "$test" --test-threads=1)
  # Only system launchers run elevated. Both variants replace inherited groups
  # and environment before starting the test executable.
  if [[ $RUNNER_OS == Linux ]]; then
    sudo -n /usr/bin/unshare --net -- /usr/bin/setpriv \
      --reuid="$uid" --regid="$gid" --clear-groups --no-new-privs \
      --inh-caps=-all --ambient-caps=-all --bounding-set=-all \
      /usr/bin/env -i "${fixture_env[@]}" \
      /usr/bin/timeout --kill-after=2s 35s "$root/worker" "${args[@]}"
  else
    # Root remains the system root; chroot is used here only to drop identity.
    sudo -n /usr/sbin/chroot -u "$uid" -g "$gid" -G "$gid" / \
      /usr/bin/env -i "${fixture_env[@]}" \
      /usr/bin/sandbox-exec -D "REHEARSAL_ROOT=$case_root" -f "$root/network.sb" \
      "$root/worker" "${args[@]}"
  fi
}

signer_test=provider_comment_transport::installation_signer_channel::rehearsal::rehearsal_signer_process
core_test=sqlite_state::operational_opening_store::tests::rehearsal_core_process
replacement_test=sqlite_state::operational_opening_store::tests::rehearsal_replacement_core_process
harness_test=provider_comment_transport::installation_signer_channel::rehearsal::rehearsal_untrusted_process
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
