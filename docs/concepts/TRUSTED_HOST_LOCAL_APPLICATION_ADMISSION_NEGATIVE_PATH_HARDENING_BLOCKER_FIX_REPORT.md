# Trusted-Host Admission Negative-Path Hardening Blocker Fix Report

## 1. Executive Summary

The resolved-execution commitment blocker is fixed. Trusted-host admission now
binds exact encoded local path bytes and the content identity of the resolved
executable, rather than lossy path text alone. Resolved handlers revalidate the
executable content identity before every process request, and durable replay
rejects same-path executable replacement.

## 2. Blocker Fixed

The initial hardening rejected a different executable path but could not detect
different executable contents installed at the same path. It also converted
paths through `to_string_lossy`, allowing distinct non-UTF-8 byte sequences to
collapse to the same commitment material.

Both conditions undermined the claim that the executor commitment represented
the exact resolved local process boundary.

## 3. Implementation Approach

- Each explicit local-check handler computes a streaming SHA-256 content
  identity when it is resolved.
- Each handler re-reads and compares that identity immediately before process
  request construction.
- The private resolved-execution fingerprint uses fixed-width byte framing.
- Executable and working-directory paths contribute
  `OsStr::as_encoded_bytes`, not lossy Unicode text.
- The executable content hash joins the executable path, fixed arguments,
  exact working directory, sanitized environment, and timeout in the private
  fingerprint.
- Trusted-host executor commitment v2 combines that resolved fingerprint with
  the logical command-contract fingerprint and skill identity.

The content identity and path bytes remain private implementation material.
Only the resulting `SpecContentHash` crosses the admission boundary.

## 4. Failure And Privacy Posture

Unreadable executable identity returns the stable
`local_check.profile.handler.executable_identity_unavailable` code. Changed
content returns
`local_check.profile.handler.executable_identity_changed`. Neither error
contains a path, executable bytes, digest, environment value, command output,
or provider payload.

Handler Debug output marks executable paths and content hashes as redacted.
No new serialized field or public model surface is introduced.

## 5. Regression Coverage

- The foreground binary creates an admitted operation, replaces the executable
  contents at the same path, and proves replay fails closed.
- A direct handler test replaces executable contents after resolution and
  proves failure occurs before the injected process runner is called.
- A Unix-only unit proof shows distinct non-UTF-8 path byte sequences produce
  distinct commitment digests.
- Existing actor, correlation, approval-wait, topology, SQLite, exact replay,
  and output-bounding tests remain active.

## 6. Scope Explicitly Not Added

The fix adds no broader profile, public CLI, discovery, scheduler, provider,
write, automatic approval, workflow completion, hosted parity, nested harness,
OpenShell, workflow schema, example, or release behavior.

It does not claim cryptographic platform attestation or eliminate the narrow
trusted-host race between final identity validation and operating-system
process entry. A stronger descriptor-based or sandbox-attested launch boundary
requires separate design.

## 7. Validation

Focused tests passed before full validation. Final phase validation passed:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace -- --test-threads=1`;
- `npm run check:docs`;
- `npm run check:integrations`; and
- `git diff --check`.

The ordinary parallel workspace run exposed two pre-existing timing-sensitive
scheduler tests in separate attempts. Each exact test passed immediately on
isolated rerun. The serialized workspace run then passed in full, including all
six trusted-host foreground-admission proofs. Running `check:integrations`
concurrently with a workspace test was also rejected as evidence because its
Cargo build intentionally changed the executable whose immutability the new
check was proving; the integration check and serialized workspace suite each
passed independently.

## 8. Governed Phase Record

- Workflow: `dg/blocker`
- Run: `run-1791484022840303000-2`
- Approval: `approval/run-1791484022840303000-2/fix-approved`
- Presentation proof: `presentation/7051b87adf3aecf1`
- Approval outcome: granted under delegated maintainer authority
- Phase status: completed
- Event summary: 39 events, including one approval request, one approval
  grant, eight policy decisions, six scheduled steps, six successful skill
  invocations, and one run completion; zero retries and zero escalations

No `.workflow-os` runtime state is part of the blocker fix.

Repository inspection, implementation, testing, documentation updates, and
git/PR work were performed by the delegated maintainer outside the kernel. The
kernel governed scope, approval, sequencing, and durable phase evidence.

## 9. Recommended Next Phase

Perform a focused blocker-fix review of byte-exact path framing, executable
content identity, pre-request revalidation, same-path replacement rejection,
error non-leakage, and the remaining trusted-host launch race disclosure.

Do not broaden trusted-host adoption before that review is accepted.
