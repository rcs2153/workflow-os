# CLI Exit Codes

`workflow-os` uses stable v0 exit code categories.

| Code | Meaning |
| --- | --- |
| `0` | Command completed successfully. |
| `1` | Parse or validation failure. |
| `2` | CLI usage error or unsupported command shape. |
| `3` | Runtime, policy, security, invalid state, or internal error. |

Diagnostics are printed for validation failures where available. Experimental preview JSON output is available for selected commands with `--json`, but it is not a versioned stable machine-output contract in `0.1.0-preview.1`.

Examples of usage errors include missing required positional arguments and `workflow-os approve --deny` without an explicit `--reason`.
