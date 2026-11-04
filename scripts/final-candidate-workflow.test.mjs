import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import test from "node:test";

const workflowPath = resolve(import.meta.dirname, "../.github/workflows/final-candidate-linux.yml");
const workflow = readFileSync(workflowPath, "utf8");

test("candidate workflow is manual, exact-source bound, and least privilege", () => {
  assert.match(workflow, /workflow_dispatch:/);
  assert.match(workflow, /source_commit:/);
  assert.match(workflow, /\^\[0-9a-f\]\{40\}\$/);
  assert.match(workflow, /SOURCE_COMMIT}" != "\$\{WORKFLOW_DEFINITION_COMMIT\}/);
  assert.match(workflow, /WORKFLOW_DEFINITION_COMMIT: \$\{\{ github\.workflow_sha \}\}/);
  assert.match(workflow, /ref: \$\{\{ inputs\.source_commit \}\}/);
  assert.match(workflow, /persist-credentials: false/);
  assert.match(workflow, /test "\$\(git rev-parse HEAD\)" = "\$\{SOURCE_COMMIT\}"/);
  assert.match(workflow, /git status --porcelain --untracked-files=all/);
  assert.match(workflow, /version'\)" = "0\.4\.0-preview\.1"/);
  assert.match(workflow, /permissions:\n  contents: read/);
  assert.match(workflow, /timeout-minutes: 45/);
  assert.doesNotMatch(workflow, /contents: write|actions: write|packages: write|pull-requests: write/);
});

test("candidate workflow pins action revisions and one toolchain", () => {
  const uses = [...workflow.matchAll(/^\s*uses:\s*([^\s#]+)/gm)].map((match) => match[1]);
  assert.deepEqual(uses, [
    "actions/checkout@11d5960a326750d5838078e36cf38b85af677262",
    "dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8",
    "actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020",
    "actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02",
  ]);
  assert.match(workflow, /toolchain: 1\.99\.0/);
  assert.match(workflow, /node-version: 20\.19\.5/);
});

test("candidate workflow admits final posture and emits bounded evidence only", () => {
  assert.match(workflow, /--final-candidate/);
  assert.doesNotMatch(workflow, /--provisional/);
  assert.match(workflow, /workflowos\.dev\/release-package\/v1/);
  assert.match(workflow, /workflowos\.dev\/final-candidate-evidence\/v1/);
  assert.match(workflow, /publication_authorized: false/);
  assert.match(workflow, /workflow_run_id: process\.env\.WORKFLOW_RUN_ID/);
  assert.match(workflow, /storage_artifact_name: process\.env\.ARTIFACT_NAME/);
  assert.match(workflow, /rustc_version: process\.env\.RUSTC_VERSION/);
  assert.match(workflow, /retention-days: 7/);
  assert.match(workflow, /overwrite: false/);
  assert.match(workflow, /compression-level: 0/);
  assert.doesNotMatch(workflow, /gh release|npm publish|cargo publish|git tag|git push/);
});
