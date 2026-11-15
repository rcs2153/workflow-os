import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

const maxAgeSeconds = 7 * 24 * 60 * 60;

try {
  const [database, ...extra] = process.argv.slice(2);
  if (!database || extra.length || !existsSync(join(database, ".git"))) {
    throw new Error("expected a dedicated advisory database checkout");
  }
  const head = execFileSync("git", ["-C", database, "show", "-s", "--format=%H %ct", "HEAD"], {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "ignore"],
  }).trim();
  const match = /^([0-9a-f]{40}) ([0-9]+)$/.exec(head);
  if (!match) {
    throw new Error("invalid advisory commit metadata");
  }
  const committedAt = Number(match[2]);
  const age = Math.floor(Date.now() / 1000) - committedAt;
  if (!Number.isSafeInteger(committedAt) || committedAt <= 0 || age < 0 || age > maxAgeSeconds) {
    throw new Error("advisory commit must be dated within the last seven days");
  }
  console.log(`RustSec advisory revision ${match[1]}: ${age}s old (maximum ${maxAgeSeconds}s)`);
} catch (error) {
  console.error(`RustSec freshness check failed: ${error.message}`);
  process.exitCode = 1;
}
