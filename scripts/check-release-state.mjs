import { access, readFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const releaseStatePath = "docs/release/0.4.0-preview.1_RELEASE_STATE.json";
const checklistPath = "docs/release/0.4.0-preview.1_RELEASE_CHECKLIST.md";
const candidateReportPath = "docs/concepts/0.4.0-preview.1_NATIVE_CANDIDATE_PRODUCTION_REPORT.md";
const transferReportPath = "docs/concepts/0.4.0-preview.1_SCOPED_CANDIDATE_TRANSFER_REPORT.md";
const candidateTargets = new Set(["aarch64-apple-darwin", "x86_64-unknown-linux-musl"]);
const hostedArtifactLabels = new Map([
  ["aarch64-apple-darwin", "macos"],
  ["x86_64-unknown-linux-musl", "linux"],
]);
const hostedWorkflowPaths = new Map([
  ["aarch64-apple-darwin", ".github/workflows/final-candidate-macos.yml"],
  ["x86_64-unknown-linux-musl", ".github/workflows/final-candidate-linux.yml"],
]);
const hostedBinaryShapes = new Map([
  ["aarch64-apple-darwin", /Mach-O 64-bit executable arm64/],
  ["x86_64-unknown-linux-musl", /ELF 64-bit LSB .*x86-64.*static(?:-pie)? linked/],
]);

function isSha256(value) {
  return /^[0-9a-f]{64}$/.test(value ?? "");
}

function isExactCommit(value) {
  return /^[0-9a-f]{40}$/.test(value ?? "");
}

function isPositiveInteger(value) {
  return Number.isSafeInteger(value) && value > 0;
}

function isBoundedIdentifier(value) {
  return typeof value === "string" && value.length > 0 && value.length <= 128;
}

function fail(errors) {
  throw new Error(`Release state validation failed:\n- ${errors.join("\n- ")}`);
}

export function validateReleaseStateSnapshot({
  state,
  packageManifest,
  documents,
  existingPaths,
  isAncestor,
}) {
  const errors = [];

  if (state.schema_version !== "workflowos.dev/release-state/v1") {
    errors.push(`unsupported release-state schema ${state.schema_version ?? "missing"}`);
  }
  if (state.version !== packageManifest.version) {
    errors.push(`release-state version ${state.version} does not match package version ${packageManifest.version}`);
  }
  if (state.publication_authorized !== false) {
    errors.push("active release-state record must keep publication_authorized false before a separate decision");
  }

  if (!isExactCommit(state.candidate_source_commit)) {
    errors.push("candidate_source_commit must be an exact lowercase commit");
  } else if (!isAncestor(state.candidate_source_commit)) {
    errors.push("candidate_source_commit is not an ancestor of HEAD");
  }
  if (!isPositiveInteger(state.candidate_ci_run_id)) {
    errors.push("candidate_ci_run_id must be a positive integer");
  }

  const candidates = state.candidates ?? [];
  const targets = new Set(candidates.map((candidate) => candidate.target));
  if (candidates.length !== candidateTargets.size || targets.size !== candidateTargets.size ||
      [...candidateTargets].some((target) => !targets.has(target))) {
    errors.push("candidate evidence must contain exactly one record for each selected native target");
  }

  const hostedCandidates = candidates.filter(
    (candidate) => candidate.storage_posture === "bounded_actions_artifact",
  );
  const hostedTransfer = hostedCandidates.length === candidateTargets.size;
  if (hostedCandidates.length > 0 && !hostedTransfer) {
    errors.push("candidate transfer must not mix bounded Actions artifacts with another storage posture");
  }
  if (hostedTransfer && state.candidate_transfer_status !== "verified_bounded_actions_artifacts") {
    errors.push("symmetric hosted candidates require verified_bounded_actions_artifacts transfer status");
  }
  if (!hostedTransfer && state.candidate_transfer_status !== undefined) {
    errors.push("candidate_transfer_status is only valid for the symmetric hosted transfer posture");
  }

  if (hostedTransfer) {
    if (!isExactCommit(state.supersedes_candidate_source_commit) ||
        state.supersedes_candidate_source_commit === state.candidate_source_commit) {
      errors.push("hosted transfer must identify one distinct exact superseded candidate source commit");
    }
    if (state.candidate_evidence_report_path !== transferReportPath) {
      errors.push(`hosted transfer evidence report must be ${transferReportPath}`);
    }
    if (!(state.authority_paths ?? []).includes(transferReportPath)) {
      errors.push("hosted transfer evidence report must be an authority path");
    }
  }

  for (const candidate of candidates) {
    const prefix = `candidate ${candidate.target ?? "missing-target"}`;
    if (!candidateTargets.has(candidate.target)) {
      errors.push(`${prefix} is not a selected native target`);
    }
    if (candidate.source_commit !== state.candidate_source_commit) {
      errors.push(`${prefix} source commit does not match candidate_source_commit`);
    }
    if (candidate.posture !== "final_candidate") {
      errors.push(`${prefix} must use final_candidate posture`);
    }
    if (candidate.working_tree_clean !== true) {
      errors.push(`${prefix} must record a clean working tree`);
    }
    if (candidate.publication_authorized !== false) {
      errors.push(`${prefix} must keep publication_authorized false`);
    }
    if (!isSha256(candidate.archive_sha256) || !isSha256(candidate.binary_sha256)) {
      errors.push(`${prefix} must record lowercase SHA-256 archive and binary digests`);
    }
    const expectedArchive = `workflow-os-v${state.version}-${candidate.target}.tar.gz`;
    if (candidate.archive_name !== expectedArchive) {
      errors.push(`${prefix} archive name must be ${expectedArchive}`);
    }

    if (hostedTransfer) {
      if (candidate.workflow_definition_path !== hostedWorkflowPaths.get(candidate.target)) {
        errors.push(`${prefix} must record the exact target workflow definition path`);
      }
      if (candidate.workflow_definition_commit !== state.candidate_source_commit) {
        errors.push(`${prefix} workflow definition commit must equal candidate_source_commit`);
      }
      if (!isPositiveInteger(candidate.workflow_run_id) ||
          !isPositiveInteger(candidate.workflow_run_attempt) ||
          !isPositiveInteger(candidate.artifact_id) ||
          !isPositiveInteger(candidate.artifact_size_bytes)) {
        errors.push(`${prefix} must record positive workflow run and artifact identifiers`);
      }
      if (candidate.workflow_run_conclusion !== "success") {
        errors.push(`${prefix} workflow run must be terminal success`);
      }
      if (!isBoundedIdentifier(candidate.workflow_actor) ||
          Number.isNaN(Date.parse(candidate.workflow_run_created_at ?? ""))) {
        errors.push(`${prefix} must record the bounded workflow actor and run creation time`);
      }
      const expectedArtifact = `workflow-os-v${state.version}-${hostedArtifactLabels.get(candidate.target)}-${state.candidate_source_commit}`;
      if (candidate.storage_artifact_name !== expectedArtifact) {
        errors.push(`${prefix} artifact name must be ${expectedArtifact}`);
      }
      const createdAt = Date.parse(candidate.artifact_created_at ?? "");
      const expiresAt = Date.parse(candidate.expires_at ?? "");
      const sevenDaysMs = 7 * 24 * 60 * 60 * 1000;
      const retentionToleranceMs = 5 * 60 * 1000;
      if (candidate.retention_days !== 7 || Number.isNaN(createdAt) || Number.isNaN(expiresAt) ||
          Math.abs((expiresAt - createdAt) - sevenDaysMs) > retentionToleranceMs) {
        errors.push(`${prefix} must record the bounded seven-day artifact retention and expiry`);
      }
      if (candidate.candidate_evidence_schema_version !== "workflowos.dev/final-candidate-evidence/v1") {
        errors.push(`${prefix} must record the supported candidate-evidence schema`);
      }
      if (!isSha256(candidate.artifact_digest_sha256) || !isSha256(candidate.candidate_evidence_sha256)) {
        errors.push(`${prefix} must record artifact and candidate-evidence SHA-256 digests`);
      }
      if (candidate.native_version_output !== `workflow-os ${state.version}`) {
        errors.push(`${prefix} must record exact native version output`);
      }
      if (!hostedBinaryShapes.get(candidate.target)?.test(candidate.binary_shape ?? "")) {
        errors.push(`${prefix} must record the expected native binary shape`);
      }
    } else {
      if (candidate.target === "aarch64-apple-darwin" && candidate.storage_posture !== "local_ignored_dist") {
        errors.push(`${prefix} must disclose local_ignored_dist evidence storage`);
      }
      if (candidate.target === "x86_64-unknown-linux-musl") {
        if (candidate.workflow_definition_commit !== state.candidate_source_commit) {
          errors.push(`${prefix} workflow definition commit must equal candidate_source_commit`);
        }
        if (!isPositiveInteger(candidate.workflow_run_id) || !isPositiveInteger(candidate.artifact_id)) {
          errors.push(`${prefix} must record positive workflow run and artifact identifiers`);
        }
        const expectedArtifact = `workflow-os-v${state.version}-linux-${state.candidate_source_commit}`;
        if (candidate.storage_artifact_name !== expectedArtifact) {
          errors.push(`${prefix} artifact name must be ${expectedArtifact}`);
        }
        if (candidate.retention_days !== 7 || Number.isNaN(Date.parse(candidate.expires_at ?? ""))) {
          errors.push(`${prefix} must record the bounded seven-day artifact retention and expiry`);
        }
      }
    }
  }

  for (const path of state.authority_paths ?? []) {
    const content = documents.get(path);
    if (content === undefined) {
      errors.push(`missing authority document ${path}`);
      continue;
    }
    if (/merge pending/i.test(content)) {
      errors.push(`${path} contains transition-sensitive merge-pending language`);
    }
  }

  for (const transition of state.transitions ?? []) {
    if (transition.status !== "merged") {
      errors.push(`${transition.id} must be recorded as merged`);
    }
    if (!Number.isSafeInteger(transition.pull_request) || transition.pull_request <= 0) {
      errors.push(`${transition.id} has an invalid pull request number`);
    }
    if (!isExactCommit(transition.merge_commit)) {
      errors.push(`${transition.id} has an invalid merge commit`);
    } else if (!isAncestor(transition.merge_commit)) {
      errors.push(`${transition.id} merge commit is not an ancestor of HEAD`);
    }

    for (const path of transition.required_paths ?? []) {
      if (!existingPaths.has(path)) {
        errors.push(`${transition.id} is missing required implementation path ${path}`);
      }
    }

    for (const path of transition.required_mentions ?? []) {
      const content = documents.get(path);
      if (content === undefined) {
        errors.push(`${transition.id} is missing required authority document ${path}`);
        continue;
      }
      if (!content.includes(`PR #${transition.pull_request}`)) {
        errors.push(`${path} does not cite PR #${transition.pull_request}`);
      }
      if (!content.includes(transition.merge_commit)) {
        errors.push(`${path} does not cite merge commit ${transition.merge_commit}`);
      }
    }
  }

  const checklist = documents.get(checklistPath) ?? "";
  if (!/- \[x\] Exact-commit Linux candidate workflow is implemented, reviewed, and\s+merged before dispatch\./s.test(checklist)) {
    errors.push("active release checklist does not close the merged Linux candidate workflow gate");
  }

  const evidenceReportPath = hostedTransfer ? transferReportPath : candidateReportPath;
  const candidateReport = documents.get(evidenceReportPath) ?? "";
  for (const requiredFact of [
    state.candidate_source_commit,
    String(state.candidate_ci_run_id),
    hostedTransfer && state.supersedes_candidate_source_commit,
    ...candidates.flatMap((candidate) => [
      candidate.archive_sha256,
      candidate.binary_sha256,
      candidate.workflow_run_id && String(candidate.workflow_run_id),
      candidate.workflow_run_attempt && String(candidate.workflow_run_attempt),
      candidate.artifact_id && String(candidate.artifact_id),
      candidate.artifact_digest_sha256,
      candidate.candidate_evidence_sha256,
      candidate.storage_artifact_name,
    ]),
  ]) {
    if (requiredFact && !candidateReport.includes(requiredFact)) {
      errors.push(`${evidenceReportPath} does not cite candidate fact ${requiredFact}`);
    }
  }

  if (errors.length > 0) {
    fail(errors);
  }
}

async function run() {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
  const state = JSON.parse(await readFile(resolve(root, releaseStatePath), "utf8"));
  const packageManifest = JSON.parse(await readFile(resolve(root, "package.json"), "utf8"));
  const documents = new Map();
  const existingPaths = new Set();

  for (const path of new Set([
    ...(state.authority_paths ?? []),
    ...(state.transitions ?? []).flatMap((transition) => transition.required_mentions ?? []),
  ])) {
    documents.set(path, await readFile(resolve(root, path), "utf8"));
  }

  for (const path of (state.transitions ?? []).flatMap((transition) => transition.required_paths ?? [])) {
    await access(resolve(root, path));
    existingPaths.add(path);
  }

  validateReleaseStateSnapshot({
    state,
    packageManifest,
    documents,
    existingPaths,
    isAncestor(commit) {
      const result = spawnSync("git", ["merge-base", "--is-ancestor", commit, "HEAD"], {
        cwd: root,
        stdio: "ignore",
      });
      return result.status === 0;
    },
  });

  console.log(`Release state is consistent for ${state.version}.`);
}

if (resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  await run();
}
