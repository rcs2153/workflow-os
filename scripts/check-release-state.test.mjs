import test from "node:test";
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

import { validateReleaseStateSnapshot } from "./check-release-state.mjs";

const finalCommit = "0d5a9c2ae26f42fc4efa907942a66e49a9e41c59";
const workflowCommit = "b0bbb4994c8b6b9816e43e38eae8cd17005e908a";
const candidateCommit = "33a77cb27fb989cb4e1695788df5eee591695aa6";
const checklistPath = "docs/release/0.4.0-preview.1_RELEASE_CHECKLIST.md";
const candidateReportPath = "docs/concepts/0.4.0-preview.1_NATIVE_CANDIDATE_PRODUCTION_REPORT.md";
const transferReportPath = "docs/concepts/0.4.0-preview.1_SCOPED_CANDIDATE_TRANSFER_REPORT.md";

function fixture() {
  const finalReport = "Merged through PR #712 at 0d5a9c2ae26f42fc4efa907942a66e49a9e41c59.";
  const workflowReport = "Merged through PR #713 at b0bbb4994c8b6b9816e43e38eae8cd17005e908a.";
  const sharedAuthority = `${finalReport}\n${workflowReport}`;
  const macArchive = "a".repeat(64);
  const macBinary = "b".repeat(64);
  const linuxArchive = "c".repeat(64);
  const linuxBinary = "d".repeat(64);
  const linuxArtifact = `workflow-os-v0.4.0-preview.1-linux-${candidateCommit}`;
  const candidateReport = [
    candidateCommit,
    "41896138868",
    macArchive,
    macBinary,
    linuxArchive,
    linuxBinary,
    "41903586104",
    "13358439075",
    linuxArtifact,
  ].join("\n");
  return {
    state: {
      schema_version: "workflowos.dev/release-state/v1",
      version: "0.4.0-preview.1",
      publication_authorized: false,
      authority_paths: ["ROADMAP.md", "final.md", "workflow.md", "plan.md", checklistPath, candidateReportPath],
      candidate_source_commit: candidateCommit,
      candidate_ci_run_id: 41896138868,
      candidates: [
        {
          target: "aarch64-apple-darwin",
          source_commit: candidateCommit,
          archive_name: "workflow-os-v0.4.0-preview.1-aarch64-apple-darwin.tar.gz",
          archive_sha256: macArchive,
          binary_sha256: macBinary,
          posture: "final_candidate",
          working_tree_clean: true,
          storage_posture: "local_ignored_dist",
          publication_authorized: false,
        },
        {
          target: "x86_64-unknown-linux-musl",
          source_commit: candidateCommit,
          workflow_definition_commit: candidateCommit,
          workflow_run_id: 41903586104,
          artifact_id: 13358439075,
          storage_artifact_name: linuxArtifact,
          archive_name: "workflow-os-v0.4.0-preview.1-x86_64-unknown-linux-musl.tar.gz",
          archive_sha256: linuxArchive,
          binary_sha256: linuxBinary,
          posture: "final_candidate",
          working_tree_clean: true,
          retention_days: 7,
          expires_at: "2026-11-11T20:11:42Z",
          publication_authorized: false,
        },
      ],
      transitions: [
        {
          id: "final_packaging_implementation",
          status: "merged",
          pull_request: 712,
          merge_commit: finalCommit,
          required_paths: ["scripts/package-release.mjs"],
          required_mentions: ["final.md", "plan.md", "ROADMAP.md"],
        },
        {
          id: "linux_candidate_workflow",
          status: "merged",
          pull_request: 713,
          merge_commit: workflowCommit,
          required_paths: [".github/workflows/final-candidate-linux.yml"],
          required_mentions: ["workflow.md", "plan.md", "ROADMAP.md"],
        },
      ],
    },
    packageManifest: { version: "0.4.0-preview.1" },
    documents: new Map([
      ["ROADMAP.md", sharedAuthority],
      ["final.md", finalReport],
      ["workflow.md", workflowReport],
      ["plan.md", sharedAuthority],
      [checklistPath, "- [x] Exact-commit Linux candidate workflow is implemented, reviewed, and\n  merged before dispatch."],
      [candidateReportPath, candidateReport],
    ]),
    existingPaths: new Set([
      "scripts/package-release.mjs",
      ".github/workflows/final-candidate-linux.yml",
    ]),
    isAncestor: () => true,
  };
}

function hostedTransferFixture() {
  const input = fixture();
  const priorCommit = input.state.candidate_source_commit;
  const transferCommit = "e".repeat(40);
  const reportFacts = [transferCommit, priorCommit, "41896139999"];

  input.state.candidate_source_commit = transferCommit;
  input.state.candidate_ci_run_id = 41896139999;
  input.state.candidate_transfer_status = "verified_bounded_actions_artifacts";
  input.state.supersedes_candidate_source_commit = priorCommit;
  input.state.candidate_evidence_report_path = transferReportPath;
  input.state.authority_paths.push(transferReportPath);
  input.state.candidates = [
    {
      target: "aarch64-apple-darwin",
      source_commit: transferCommit,
      workflow_definition_path: ".github/workflows/final-candidate-macos.yml",
      workflow_definition_commit: transferCommit,
      workflow_run_id: 41945942592,
      workflow_run_attempt: 1,
      workflow_run_conclusion: "success",
      workflow_actor: "maintainer",
      workflow_run_created_at: "2030-11-05T01:01:00Z",
      artifact_id: 13378129896,
      artifact_size_bytes: 5213240,
      artifact_created_at: "2030-11-05T01:08:56Z",
      storage_artifact_name: `workflow-os-v0.4.0-preview.1-macos-${transferCommit}`,
      storage_posture: "bounded_actions_artifact",
      archive_name: "workflow-os-v0.4.0-preview.1-aarch64-apple-darwin.tar.gz",
      archive_sha256: "1".repeat(64),
      binary_sha256: "2".repeat(64),
      artifact_digest_sha256: "3".repeat(64),
      candidate_evidence_schema_version: "workflowos.dev/final-candidate-evidence/v1",
      candidate_evidence_sha256: "4".repeat(64),
      binary_shape: "Mach-O 64-bit executable arm64",
      native_version_output: "workflow-os 0.4.0-preview.1",
      posture: "final_candidate",
      working_tree_clean: true,
      retention_days: 7,
      expires_at: "2030-11-12T01:08:56Z",
      publication_authorized: false,
    },
    {
      target: "x86_64-unknown-linux-musl",
      source_commit: transferCommit,
      workflow_definition_path: ".github/workflows/final-candidate-linux.yml",
      workflow_definition_commit: transferCommit,
      workflow_run_id: 41945946070,
      workflow_run_attempt: 1,
      workflow_run_conclusion: "success",
      workflow_actor: "maintainer",
      workflow_run_created_at: "2030-11-05T01:01:30Z",
      artifact_id: 13379065507,
      artifact_size_bytes: 5765457,
      artifact_created_at: "2030-11-05T01:08:12Z",
      storage_artifact_name: `workflow-os-v0.4.0-preview.1-linux-${transferCommit}`,
      storage_posture: "bounded_actions_artifact",
      archive_name: "workflow-os-v0.4.0-preview.1-x86_64-unknown-linux-musl.tar.gz",
      archive_sha256: "5".repeat(64),
      binary_sha256: "6".repeat(64),
      artifact_digest_sha256: "7".repeat(64),
      candidate_evidence_schema_version: "workflowos.dev/final-candidate-evidence/v1",
      candidate_evidence_sha256: "8".repeat(64),
      binary_shape: "ELF 64-bit LSB pie executable, x86-64, static-pie linked",
      native_version_output: "workflow-os 0.4.0-preview.1",
      posture: "final_candidate",
      working_tree_clean: true,
      retention_days: 7,
      expires_at: "2030-11-12T01:08:12Z",
      publication_authorized: false,
    },
  ];

  for (const candidate of input.state.candidates) {
    reportFacts.push(
      candidate.archive_sha256,
      candidate.binary_sha256,
      String(candidate.workflow_run_id),
      String(candidate.workflow_run_attempt),
      String(candidate.artifact_id),
      candidate.artifact_digest_sha256,
      candidate.candidate_evidence_sha256,
      candidate.storage_artifact_name,
    );
  }
  input.documents.set(transferReportPath, reportFacts.join("\n"));
  return input;
}

test("accepts exact merged release-state authority", () => {
  assert.doesNotThrow(() => validateReleaseStateSnapshot(fixture()));
});

test("accepts symmetric exact-source bounded Actions candidate transfer evidence", () => {
  assert.doesNotThrow(() => validateReleaseStateSnapshot(hostedTransferFixture()));
});

test("rejects mixed hosted and local candidate storage", () => {
  const input = hostedTransferFixture();
  input.state.candidates[0].storage_posture = "local_ignored_dist";
  assert.throws(() => validateReleaseStateSnapshot(input), /must not mix bounded Actions artifacts/);
});

test("rejects hosted transfer without explicit verified status", () => {
  const input = hostedTransferFixture();
  delete input.state.candidate_transfer_status;
  assert.throws(() => validateReleaseStateSnapshot(input), /require verified_bounded_actions_artifacts/);
});

test("rejects hosted transfer without a distinct superseded source", () => {
  const input = hostedTransferFixture();
  input.state.supersedes_candidate_source_commit = input.state.candidate_source_commit;
  assert.throws(() => validateReleaseStateSnapshot(input), /distinct exact superseded candidate source/);
});

test("rejects nonterminal hosted workflow evidence", () => {
  const input = hostedTransferFixture();
  input.state.candidates[1].workflow_run_conclusion = "failure";
  assert.throws(() => validateReleaseStateSnapshot(input), /workflow run must be terminal success/);
});

test("rejects incomplete hosted workflow identity", () => {
  const input = hostedTransferFixture();
  input.state.candidates[0].workflow_definition_path = ".github/workflows/final-candidate-linux.yml";
  delete input.state.candidates[1].workflow_actor;
  assert.throws(
    () => validateReleaseStateSnapshot(input),
    /exact target workflow definition path[\s\S]*bounded workflow actor and run creation time/,
  );
});

test("rejects incomplete hosted artifact metadata", () => {
  const input = hostedTransferFixture();
  delete input.state.candidates[0].artifact_digest_sha256;
  input.state.candidates[1].expires_at = "2030-11-06T01:08:12Z";
  delete input.state.candidates[1].candidate_evidence_schema_version;
  assert.throws(
    () => validateReleaseStateSnapshot(input),
    /artifact and candidate-evidence SHA-256[\s\S]*bounded seven-day artifact retention[\s\S]*supported candidate-evidence schema/,
  );
});

test("rejects hosted artifact identity or binary-shape drift", () => {
  const input = hostedTransferFixture();
  input.state.candidates[0].storage_artifact_name = "workflow-os-v0.4.0-preview.1-macos-wrong";
  input.state.candidates[1].binary_shape = "ELF 64-bit ARM";
  assert.throws(
    () => validateReleaseStateSnapshot(input),
    /artifact name must be[\s\S]*expected native binary shape/,
  );
});

test("rejects transition-sensitive merge-pending language", () => {
  const input = fixture();
  input.documents.set("ROADMAP.md", `${input.documents.get("ROADMAP.md")}\nmerge pending`);
  assert.throws(() => validateReleaseStateSnapshot(input), /transition-sensitive merge-pending language/);
});

test("rejects an unchecked merged-workflow gate", () => {
  const input = fixture();
  input.documents.set(checklistPath, "- [ ] Exact-commit Linux candidate workflow is implemented, reviewed, and\n  merged before dispatch.");
  assert.throws(() => validateReleaseStateSnapshot(input), /does not close the merged Linux candidate workflow gate/);
});

test("rejects a merge commit outside current history", () => {
  const input = fixture();
  input.isAncestor = (commit) => commit !== workflowCommit;
  assert.throws(() => validateReleaseStateSnapshot(input), /linux_candidate_workflow merge commit is not an ancestor of HEAD/);
});

test("rejects publication authority without a separate decision", () => {
  const input = fixture();
  input.state.publication_authorized = true;
  assert.throws(() => validateReleaseStateSnapshot(input), /must keep publication_authorized false/);
});

test("rejects active release version drift", () => {
  const input = fixture();
  input.packageManifest.version = "0.4.0-preview.2";
  assert.throws(() => validateReleaseStateSnapshot(input), /does not match package version/);
});

test("rejects missing or duplicate native candidate targets", () => {
  const input = fixture();
  input.state.candidates[1] = { ...input.state.candidates[0] };
  assert.throws(() => validateReleaseStateSnapshot(input), /exactly one record for each selected native target/);
});

test("rejects candidate source drift", () => {
  const input = fixture();
  input.state.candidates[0].source_commit = finalCommit;
  assert.throws(() => validateReleaseStateSnapshot(input), /source commit does not match candidate_source_commit/);
});

test("rejects invalid candidate checksums", () => {
  const input = fixture();
  input.state.candidates[0].archive_sha256 = "not-a-checksum";
  assert.throws(() => validateReleaseStateSnapshot(input), /must record lowercase SHA-256/);
});

test("rejects candidate publication authority", () => {
  const input = fixture();
  input.state.candidates[1].publication_authorized = true;
  assert.throws(() => validateReleaseStateSnapshot(input), /must keep publication_authorized false/);
});

test("rejects Linux workflow or retention drift", () => {
  const input = fixture();
  input.state.candidates[1].workflow_definition_commit = workflowCommit;
  input.state.candidates[1].retention_days = 30;
  assert.throws(
    () => validateReleaseStateSnapshot(input),
    /workflow definition commit must equal candidate_source_commit[\s\S]*bounded seven-day artifact retention/,
  );
});

test("rejects Linux artifact identity drift", () => {
  const input = fixture();
  input.state.candidates[1].storage_artifact_name = "workflow-os-v0.4.0-preview.1-linux-wrong";
  assert.throws(() => validateReleaseStateSnapshot(input), /artifact name must be/);
});

test("rejects a candidate source outside current history", () => {
  const input = fixture();
  input.isAncestor = (commit) => commit !== candidateCommit;
  assert.throws(() => validateReleaseStateSnapshot(input), /candidate_source_commit is not an ancestor of HEAD/);
});

test("rejects candidate evidence omitted from the production report", () => {
  const input = fixture();
  input.documents.set(candidateReportPath, candidateCommit);
  assert.throws(() => validateReleaseStateSnapshot(input), /does not cite candidate fact/);
});

test("CI gives the release-state documentation check complete Git history", async () => {
  const workflow = await readFile(".github/workflows/ci.yml", "utf8");
  const typescriptJob = workflow.match(/\n  typescript:\n(?<body>[\s\S]*?)(?=\n  [a-z][a-z0-9_-]*:\n|$)/)?.groups?.body;

  assert.ok(typescriptJob, "TypeScript And Docs job must exist");
  assert.match(
    typescriptJob,
    /- name: Checkout\n\s+uses: actions\/checkout@v4\n\s+with:\n\s+fetch-depth: 0/,
    "the ancestry-sensitive documentation job must fetch the complete commit graph",
  );
  assert.match(typescriptJob, /- name: Documentation checks\n\s+run: npm run check:docs/);
});
