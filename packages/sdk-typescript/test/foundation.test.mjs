import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("SDK source states the TypeScript boundary", async () => {
  const source = await readFile(new URL("../src/index.ts", import.meta.url), "utf8");

  assert.match(source, /canonical Rust core model/);
  assert.doesNotMatch(source, /GitHub|Jira|CI adapter/);
});
