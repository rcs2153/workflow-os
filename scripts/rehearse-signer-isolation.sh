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
reader_uid=61025
receiver_uid=61026
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
for uid in "$core_uid" "$signer_uid" "$harness_uid" 61024 "$reader_uid" "$receiver_uid"; do
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
chown=/usr/bin/chown
if [[ $RUNNER_OS == macOS ]]; then
  tmp=/private/tmp
  chown=/usr/sbin/chown
fi
for utility in "$chown" /bin/chmod /bin/cp /bin/rm /usr/bin/install /usr/bin/tar; do
  if [[ ! -x $utility ]]; then
    echo "Required system utility unavailable: $utility" >&2
    exit 2
  fi
done
root=$(mktemp -d "$tmp/workflow-os-signer.XXXXXXXX")
staging=
cleanup() {
  if [[ -n $staging ]]; then /bin/rm -rf -- "$staging"; fi
  sudo -n /bin/rm -rf -- "$root"
}
trap cleanup EXIT
sudo -n "$chown" 0:0 "$root"
sudo -n /bin/chmod 0711 "$root"
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
sudo -n /bin/cp -R -- "$staging" "$root/core-fixture/project"
sudo -n /bin/chmod -R a=rX "$root/core-fixture/project"

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
reader_prefix=provider_comment_transport::installation_signer_channel::rehearsal::reader
reader_signer_test=$reader_prefix::rehearsal_reader_signer_process
reader_core_test=$reader_prefix::rehearsal_reader_core_process
reader_writer_test=$reader_prefix::rehearsal_reader_writer_probe
reader_harness_test=$reader_prefix::rehearsal_reader_harness_probe
notification_prefix=provider_comment_transport::installation_notification::rehearsal
notification_test=$notification_prefix::rehearsal_notification_receiver
notification_core_test=$notification_prefix::rehearsal_notification_core
notification_writer_test=$notification_prefix::rehearsal_notification_writer_probe
notification_harness_test=$notification_prefix::rehearsal_notification_harness_probe
listed=$("$1" --list)
for test in "$signer_test" "$core_test" "$replacement_test" "$harness_test" \
  "$reader_signer_test" "$reader_core_test" "$reader_writer_test" "$reader_harness_test" \
  "$notification_test" "$notification_core_test" "$notification_writer_test" "$notification_harness_test"; do
  if ! grep -Fqx "$test: test" <<<"$listed"; then
    echo "Missing exact rehearsal test: $test" >&2
    exit 2
  fi
done

echo 'Notification receiver isolation rehearsal: distinct receipt producer'
trial=notification
case_root="$root/$trial"
sudo -n /usr/bin/install -d -o 0 -g 0 -m 0711 "$case_root"
sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0700 "$case_root/core-private"
sudo -n /usr/bin/install -d -o "$signer_uid" -g "$service_gid" -m 0700 "$case_root/signer-private"
sudo -n /usr/bin/install -d -o "$reader_uid" -g "$service_gid" -m 0700 "$case_root/reader-private"
sudo -n /usr/bin/install -d -o "$receiver_uid" -g "$service_gid" -m 0700 "$case_root/receiver-private"
sudo -n /usr/bin/install -d -o "$receiver_uid" -g "$service_gid" -m 0750 "$case_root/receiver-socket"
sudo -n /usr/bin/install -o "$core_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/core-private/authority"
sudo -n /usr/bin/install -o "$signer_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/signer-private/custody"
sudo -n /usr/bin/install -o "$reader_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/reader-private/custody"
run_worker "$receiver_uid" "$service_gid" "$notification_test" &
notification_pid=$!
notification_probe_status=0
run_worker "$signer_uid" "$service_gid" "$notification_writer_test" &&
  run_worker "$harness_uid" "$harness_uid" "$notification_harness_test" &&
  run_worker "$core_uid" "$service_gid" "$notification_core_test" || notification_probe_status=$?
notification_status=0
wait "$notification_pid" || notification_status=$?
if ((notification_probe_status != 0 || notification_status != 0)); then
  echo "Unexpected notification rehearsal exit: probes/Core=$notification_probe_status receiver=$notification_status" >&2
  exit 1
fi

echo 'Reader signer isolation rehearsal: distinct custody'
trial=reader
case_root="$root/$trial"
sudo -n /usr/bin/install -d -o 0 -g 0 -m 0711 "$case_root"
sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0700 "$case_root/core-private"
sudo -n /usr/bin/install -d -o "$signer_uid" -g "$service_gid" -m 0700 "$case_root/signer-private"
sudo -n /usr/bin/install -d -o "$reader_uid" -g "$service_gid" -m 0700 "$case_root/reader-private"
sudo -n /usr/bin/install -d -o "$reader_uid" -g "$service_gid" -m 0750 "$case_root/reader-socket"
sudo -n /usr/bin/install -o "$core_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/core-private/authority"
sudo -n /usr/bin/install -o "$signer_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/signer-private/custody"
sudo -n /usr/bin/install -o "$reader_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/reader-private/custody"
run_worker "$reader_uid" "$service_gid" "$reader_signer_test" &
reader_pid=$!
reader_probe_status=0
run_worker "$signer_uid" "$service_gid" "$reader_writer_test" &&
  run_worker "$harness_uid" "$harness_uid" "$reader_harness_test" &&
  run_worker "$core_uid" "$service_gid" "$reader_core_test" || reader_probe_status=$?
reader_status=0
wait "$reader_pid" || reader_status=$?
if ((reader_probe_status != 0 || reader_status != 0)); then
  echo "Unexpected reader rehearsal exit: probes/Core=$reader_probe_status signer=$reader_status" >&2
  exit 1
fi
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
