import assert from "node:assert/strict";
import test from "node:test";

import {
  integrationCommandMaxBufferBytes,
  run,
} from "./check-integrations.mjs";

test("integration helper accepts a successful bounded child process", () => {
  const result = run("bounded success", process.execPath, [
    "-e",
    "process.stdout.write('ok')",
  ]);

  assert.equal(result.status, 0);
  assert.equal(result.stdout, "ok");
  assert.equal(integrationCommandMaxBufferBytes, 16 * 1024 * 1024);
});

test("integration helper reports command failure with status and output", () => {
  assert.throws(
    () =>
      run("bounded failure", process.execPath, [
        "-e",
        "process.stderr.write('expected failure'); process.exit(7)",
      ]),
    (error) => {
      assert.match(error.message, /bounded failure failed with status 7/);
      assert.match(error.message, /expected failure/);
      return true;
    },
  );
});

test("integration helper reports output exhaustion instead of null status", () => {
  assert.throws(
    () =>
      run(
        "bounded overflow",
        process.execPath,
        ["-e", "process.stdout.write('x'.repeat(4096))"],
        { maxBufferBytes: 1024 },
      ),
    (error) => {
      assert.match(
        error.message,
        /bounded overflow exceeded the bounded 1024-byte output limit/,
      );
      assert.doesNotMatch(error.message, /failed with status null/);
      return true;
    },
  );
});
