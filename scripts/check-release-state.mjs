import { access, readFile } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const releaseStatePath = "docs/release/0.4.0-preview.1_RELEASE_STATE.json";
const checklistPath = "docs/release/0.4.0-preview.1_RELEASE_CHECKLIST.md";

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
    if (!/^[0-9a-f]{40}$/.test(transition.merge_commit ?? "")) {
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
