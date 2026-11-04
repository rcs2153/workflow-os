import test from "node:test";
import assert from "node:assert/strict";

import { validateReleaseStateSnapshot } from "./check-release-state.mjs";

const finalCommit = "0d5a9c2ae26f42fc4efa907942a66e49a9e41c59";
const workflowCommit = "b0bbb4994c8b6b9816e43e38eae8cd17005e908a";
const checklistPath = "docs/release/0.4.0-preview.1_RELEASE_CHECKLIST.md";

function fixture() {
  const finalReport = "Merged through PR #712 at 0d5a9c2ae26f42fc4efa907942a66e49a9e41c59.";
  const workflowReport = "Merged through PR #713 at b0bbb4994c8b6b9816e43e38eae8cd17005e908a.";
  const sharedAuthority = `${finalReport}\n${workflowReport}`;
  return {
    state: {
      schema_version: "workflowos.dev/release-state/v1",
      version: "0.4.0-preview.1",
      publication_authorized: false,
      authority_paths: ["ROADMAP.md", "final.md", "workflow.md", "plan.md", checklistPath],
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
    ]),
    existingPaths: new Set([
      "scripts/package-release.mjs",
      ".github/workflows/final-candidate-linux.yml",
    ]),
    isAncestor: () => true,
  };
}

test("accepts exact merged release-state authority", () => {
  assert.doesNotThrow(() => validateReleaseStateSnapshot(fixture()));
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
