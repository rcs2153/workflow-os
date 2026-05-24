import { readdir, readFile } from "node:fs/promises";
import { join, relative } from "node:path";

const packageRoot = new URL("../packages/sdk-typescript", import.meta.url).pathname;

async function walk(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (["dist", "node_modules"].includes(entry.name)) {
        continue;
      }
      files.push(...await walk(path));
    } else if (entry.name.endsWith(".ts")) {
      files.push(path);
    }
  }

  return files;
}

for (const file of await walk(join(packageRoot, "src"))) {
  const content = await readFile(file, "utf8");
  const rel = relative(packageRoot, file);

  if (content.includes("any")) {
    throw new Error(`${rel} must not use the any type`);
  }

  if (content.includes("TODO")) {
    throw new Error(`${rel} must not contain TODO markers`);
  }

  if (!content.endsWith("\n")) {
    throw new Error(`${rel} must end with a newline`);
  }
}
