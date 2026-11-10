#!/usr/bin/env bash
set -euo pipefail

deny() {
  printf '%s\n' "$1" >&2
  exit 2
}

intended_host=false
if [[ ${1:-} == --intended-host ]]; then
  intended_host=true
  export PATH=/usr/sbin:/usr/bin:/sbin:/bin
  [[ $# == 3 && $2 =~ ^[a-f0-9]{40}$ && $3 =~ ^[a-f0-9]{64}$ ]] ||
    deny "Expected --intended-host COMMIT SHA256SUMS_DIGEST."
  [[ -z ${GITHUB_ACTIONS:-}${RUNNER_ENVIRONMENT:-}${RUNNER_OS:-} && $EUID != 0 &&
    $(uname -s) == Linux && $(uname -m) == aarch64 &&
    $(systemd-detect-virt --vm) == apple && $(hostname) == lima-wos-pf ]] ||
    deny "Expected the designated Linux ARM64 guest and an unprivileged operator."
  RUNNER_OS=Linux
  bundle="/opt/workflow-os-preflight/${2:0:8}"
  protected() {
    [[ ! -L $1 && $(stat -c '%u:%g:%a' -- "$1") == "0:0:$2" ]]
  }
  marker=/etc/workflow-os-preflight-host
  [[ -f $marker ]] && protected "$marker" 444 ||
    deny "Missing protected intended-host designation."
  mapfile -t designation < "$marker"
  [[ ${#designation[@]} == 3 && ${designation[0]} == workflow-os/intended-preflight-host/v1 &&
    ${designation[1]} == lima-wos-pf && ${designation[2]} =~ ^[a-f0-9]{64}$ ]] ||
    deny "Invalid intended-host designation."
  verify_firewall() {
    local actual
    systemctl is-enabled --quiet nftables && systemctl is-active --quiet nftables ||
      deny "Intended-host outbound filter is not enabled and active."
    actual=$(sudo -n /usr/sbin/nft -s list ruleset | /usr/bin/sha256sum)
    [[ ${actual%% *} == "${designation[2]}" ]] ||
      deny "Intended-host firewall does not match the reviewed ruleset."
  }
  verify_firewall
  for dir in /opt /opt/workflow-os-preflight "$bundle"; do
    [[ -d $dir ]] && protected "$dir" 755 || deny "Unprotected bundle directory."
  done
  [[ ${BASH_SOURCE[0]} == "$bundle/launcher" ]] || deny "Launch the protected bundle copy."
  for name in launcher project.tar manifest.json SHA256SUMS; do
    [[ -f $bundle/$name ]] && protected "$bundle/$name" 444 || deny "Unprotected bundle input."
  done
  [[ -f $bundle/worker ]] && protected "$bundle/worker" 555 || deny "Unprotected worker."
  [[ $(wc -c < "$bundle/SHA256SUMS") -le 512 ]] || deny "Invalid checksum manifest size."
  actual=$(sha256sum "$bundle/SHA256SUMS")
  [[ ${actual%% *} == "$3" ]] || deny "Checksum manifest does not match the reviewed digest."
  expected_files=$'launcher\nmanifest.json\nproject.tar\nworker'
  listed_files=$(sed -nE 's/^[0-9a-f]{64}  (launcher|manifest\.json|project\.tar|worker)$/\1/p' "$bundle/SHA256SUMS" | sort)
  [[ $listed_files == "$expected_files" && $(wc -l < "$bundle/SHA256SUMS") -eq 4 ]] ||
    deny "Unexpected bundle checksum entries."
  (cd "$bundle" && sha256sum --check --strict SHA256SUMS)
  jq -e --arg commit "$2" '
    .kind == "workflow-os/offline-preflight-fixture/v1" and .source_commit == $commit and
    .target == "aarch64-unknown-linux-gnu" and (.source_tree | test("^[0-9a-f]{40}$"))
  ' "$bundle/manifest.json" >/dev/null || deny "Bundle does not match the reviewed source."
  worker_input="$bundle/worker"
else
  if [[ ${GITHUB_ACTIONS:-} != true || ${RUNNER_ENVIRONMENT:-} != github-hosted ||
        (${RUNNER_OS:-} != Linux && ${RUNNER_OS:-} != macOS) ]]; then
    deny "Signer rehearsal requires a disposable GitHub-hosted Linux or macOS runner."
  fi
  [[ $# == 1 && -f $1 && -x $1 ]] || deny "Expected exactly one compiled Core test executable."
  worker_input=$1
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
sudo -n /usr/bin/install -o 0 -g 0 -m 0555 -- "$worker_input" "$root/worker"
sudo -n /usr/bin/install -o 0 -g 0 -m 0444 -- "${BASH_SOURCE[0]}" "$root/launcher"
if [[ $RUNNER_OS == macOS ]]; then
  sudo -n /usr/bin/install -o 0 -g 0 -m 0444 -- scripts/signer-rehearsal-macos.sb "$root/network.sb"
fi

staging=$(mktemp -d "$tmp/workflow-os-signer-spec.XXXXXXXX")
if $intended_host; then
  /usr/bin/tar -xf "$bundle/project.tar" -C "$staging" --no-same-owner --no-same-permissions
else
  git archive --format=tar HEAD:dogfood/workflow-os-self-governance |
    /usr/bin/tar -xf - -C "$staging"
fi
if [[ ! -f $staging/workflow-os.yml || -n $(find "$staging" ! -type d ! -type f -print -quit) ]]; then
  echo "Expected a regular, tracked synthetic project fixture." >&2
  exit 2
fi
sudo -n /usr/bin/install -d -o 0 -g 0 -m 0555 "$root/core-fixture"
sudo -n /bin/cp -R -- "$staging" "$root/core-fixture/project"
sudo -n /bin/chmod -R a=rX "$root/core-fixture/project"

hash_file() {
  local digest rest
  if [[ $RUNNER_OS == Linux ]]; then
    read -r digest rest < <(/usr/bin/sha256sum "$1")
  else
    read -r digest rest < <(/usr/bin/shasum -a 256 "$1")
  fi
  [[ $digest =~ ^[a-f0-9]{64}$ ]] || return 2
  printf '%s\n' "$digest"
}
worker_digest=$(hash_file "$root/worker")
launcher_digest=$(hash_file "$root/launcher")
network_digest=linux-netns
if [[ $RUNNER_OS == macOS ]]; then
  network_digest=$(hash_file "$root/network.sb")
fi
stage_launch() {
  local manifest="$staging/operator-launch"
  printf '%s\n' 'workflow-os/hosted-operator-rehearsal/v1' "$worker_digest" "$trial" \
    "core=$core_uid signer=$signer_uid reader=$reader_uid receiver=$receiver_uid harness=$harness_uid group=$service_gid" \
    "$launcher_digest" "$network_digest" > "$manifest"
  if [[ $trial == operator ]]; then printf '%s\n' "$reader_deadline" >> "$manifest"; fi
  sudo -n /usr/bin/install -o 0 -g 0 -m 0444 -- "$manifest" "$case_root/operator-launch"
}

run_worker() {
  local uid=$1 gid=$2 test=$3 descriptor_probe=${4:-}
  local -a fixture_env=(
    "WORKFLOW_OS_SIGNER_REHEARSAL=$case_root" "WORKFLOW_OS_SIGNER_TRIAL=$trial"
    "TMPDIR=$case_root/core-private"
  )
  if [[ $trial == operator ]]; then
    fixture_env+=("WORKFLOW_OS_INSTALLATION_RECOVERY_TRIAL=$case_root/core-private"
      "WORKFLOW_OS_INSTALLATION_RECOVERY_READER_DEADLINE=$reader_deadline")
  fi
  local -a args=(--ignored --exact "$test" --test-threads=1)
  local -a worker=("$root/worker" "${args[@]}")
  if [[ $descriptor_probe == inject ]]; then
    worker=(/bin/sh -c 'exec 9</dev/null; exec "$@"' sh "${worker[@]}")
  fi
  # Only system launchers run elevated. Both variants replace inherited groups
  # and environment before starting the test executable.
  if [[ $RUNNER_OS == Linux ]]; then
    sudo -n /usr/bin/unshare --net -- /usr/bin/setpriv \
      --reuid="$uid" --regid="$gid" --clear-groups --no-new-privs \
      --inh-caps=-all --ambient-caps=-all --bounding-set=-all \
      /usr/bin/env -i "${fixture_env[@]}" \
      /usr/bin/timeout --kill-after=2s 35s "${worker[@]}"
  else
    # Root remains the system root; chroot is used here only to drop identity.
    sudo -n /usr/sbin/chroot -u "$uid" -g "$gid" -G "$gid" / \
      /usr/bin/env -i "${fixture_env[@]}" \
      /usr/bin/sandbox-exec -D "REHEARSAL_ROOT=$case_root" -f "$root/network.sb" \
      "${worker[@]}"
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
operator_prefix=sqlite_state::operational_opening_store::tests::recovery_process
listed=$("$worker_input" --list)
for test in "$signer_test" "$core_test" "$replacement_test" "$harness_test" \
  "$reader_signer_test" "$reader_core_test" "$reader_writer_test" "$reader_harness_test" \
  "$notification_test" "$notification_core_test" "$notification_writer_test" "$notification_harness_test" \
  "$operator_prefix::hosted_operator_producer" "$operator_prefix::hosted_operator_replacement" \
  "$operator_prefix::hosted_operator_receiver" "$operator_prefix::hosted_operator_reader"; do
  if ! grep -Fqx "$test: test" <<<"$listed"; then
    echo "Missing exact rehearsal test: $test" >&2
    exit 2
  fi
done

echo 'Exact operator rehearsal: one write, lost acknowledgement, isolated replacement'
trial=operator
case_root="$root/$trial"
if [[ $RUNNER_OS == Linux ]]; then
  reader_deadline=$(/bin/date -u -d '+15 minutes' '+%Y-%m-%dT%H:%M:%SZ')
else
  reader_deadline=$(/bin/date -u -v+15M '+%Y-%m-%dT%H:%M:%SZ')
fi
sudo -n /usr/bin/install -d -o 0 -g 0 -m 0711 "$case_root"
stage_launch
for role in core signer reader receiver; do
  case "$role" in
    core) uid=$core_uid ;;
    signer) uid=$signer_uid ;;
    reader) uid=$reader_uid ;;
    receiver) uid=$receiver_uid ;;
  esac
  sudo -n /usr/bin/install -d -o "$uid" -g "$service_gid" -m 0700 "$case_root/$role-private"
  sudo -n /usr/bin/install -d -o "$uid" -g "$service_gid" -m 0750 "$case_root/$role-socket"
done
sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0750 "$case_root/provider-result"
sudo -n /usr/bin/install -o "$core_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/core-private/authority"
sudo -n /usr/bin/install -o "$signer_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/signer-private/custody"
run_worker "$signer_uid" "$service_gid" "$signer_test" &
signer_pid=$!
producer_status=0
run_worker "$core_uid" "$service_gid" "$operator_prefix::hosted_operator_producer" || producer_status=$?
signer_status=0
wait "$signer_pid" || signer_status=$?
if ((producer_status != 77 || signer_status != 0)); then
  echo "Expected accepted write followed by Core acknowledgement loss: Core=$producer_status signer=$signer_status" >&2
  exit 1
fi
run_worker "$receiver_uid" "$service_gid" "$operator_prefix::hosted_operator_receiver" &
receiver_pid=$!
run_worker "$reader_uid" "$service_gid" "$operator_prefix::hosted_operator_reader" &
reader_pid=$!
replacement_status=0
run_worker "$core_uid" "$service_gid" "$operator_prefix::hosted_operator_replacement" || replacement_status=$?
receiver_status=0
reader_status=0
wait "$receiver_pid" || receiver_status=$?
wait "$reader_pid" || reader_status=$?
if ((replacement_status != 0 || receiver_status != 0 || reader_status != 0)); then
  echo "Exact operator recovery failed: Core=$replacement_status receiver=$receiver_status reader=$reader_status" >&2
  exit 1
fi

echo 'Notification receiver isolation rehearsal: distinct receipt producer'
trial=notification
case_root="$root/$trial"
sudo -n /usr/bin/install -d -o 0 -g 0 -m 0711 "$case_root"
stage_launch
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
stage_launch
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
  stage_launch
  sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0750 "$case_root/core-socket"
  sudo -n /usr/bin/install -d -o "$signer_uid" -g "$service_gid" -m 0750 "$case_root/signer-socket"
  sudo -n /usr/bin/install -d -o "$core_uid" -g "$service_gid" -m 0700 "$case_root/core-private"
  sudo -n /usr/bin/install -d -o "$signer_uid" -g "$service_gid" -m 0700 "$case_root/signer-private"
  sudo -n /usr/bin/install -o "$core_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/core-private/authority"
  sudo -n /usr/bin/install -o "$signer_uid" -g "$service_gid" -m 0600 /dev/null "$case_root/signer-private/custody"
  if [[ $trial == success ]]; then
    inherited_status=0
    run_worker "$core_uid" "$service_gid" "$core_test" inject || inherited_status=$?
    if ((inherited_status != 76)) || [[ -e $case_root/core-socket/decision ]]; then
      echo "Inherited descriptor probe did not deny before signer access: $inherited_status" >&2
      exit 1
    fi
  fi
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

if $intended_host; then
  verify_firewall
  echo 'Intended-host synthetic fixture completed without provider access.'
fi
