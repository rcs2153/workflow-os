import { readdir, readFile, stat } from "node:fs/promises";
import { join, relative } from "node:path";

const requiredFiles = [
  "README.md",
  "LICENSE",
  "CODE_OF_CONDUCT.md",
  "CONTRIBUTING.md",
  "SECURITY.md",
  "CHANGELOG.md",
  "MAINTAINERS.md",
  "ROADMAP.md",
  "docs/ENGINEERING_STANDARD.md",
  "docs/PROJECT_CHARTER.md",
  "docs/release/SEMVER.md",
  "docs/release/RELEASE_PROCESS.md",
];

async function walk(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      if ([".git", ".tools", "node_modules", "target", "dist"].includes(entry.name)) {
        continue;
      }
      files.push(...await walk(path));
    } else {
      files.push(path);
    }
  }

  return files;
}

const root = new URL("..", import.meta.url).pathname;
const allFiles = await walk(root);
const relativeFiles = new Set(allFiles.map((file) => relative(root, file)));

const missing = requiredFiles.filter((file) => !relativeFiles.has(file));
if (missing.length > 0) {
  throw new Error(`Missing required documentation files: ${missing.join(", ")}`);
}

for (const file of allFiles) {
  if (!file.endsWith(".md")) {
    continue;
  }

  const content = await readFile(file, "utf8");
  const lines = content.split("\n");
  lines.forEach((line, index) => {
    if (/[ \t]$/.test(line)) {
      throw new Error(`${relative(root, file)}:${index + 1} has trailing whitespace`);
    }
  });

  const fileStat = await stat(file);
  if (fileStat.size === 0) {
    throw new Error(`${relative(root, file)} is empty`);
  }
}
