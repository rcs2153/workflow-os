import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const script = fileURLToPath(new URL("./check-rustsec-freshness.mjs", import.meta.url));
const day = 24 * 60 * 60 * 1000;

function reviewCheckout(t, offset) {
  const directory = mkdtempSync(join(tmpdir(), "workflow-os-rustsec-"));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  if (offset !== undefined) {
    const date = new Date(Date.now() + offset).toISOString();
    execFileSync("git", ["init", "-q", directory]);
    execFileSync("git", [
      "-C", directory, "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
      "commit", "-q", "--allow-empty", "--no-gpg-sign", "-m", "Synthetic advisory feed",
    ], { env: { ...process.env, GIT_AUTHOR_DATE: date, GIT_COMMITTER_DATE: date } });
  }
  return spawnSync(process.execPath, [script, directory], { encoding: "utf8" });
}

test("fresh advisory commit is accepted and its identity is reported", (t) => {
  const result = reviewCheckout(t, -day);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /revision [0-9a-f]{40}/);
});

test("old advisory commit blocks the job even with a valid Git checkout", (t) => {
  const result = reviewCheckout(t, -8 * day);
  assert.equal(result.status, 1);
  assert.match(result.stderr, /last seven days/);
});

test("future advisory commit and missing checkout fail closed", (t) => {
  assert.equal(reviewCheckout(t, day).status, 1);
  assert.equal(reviewCheckout(t).status, 1);
});
