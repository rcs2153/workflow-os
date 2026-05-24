# Autonomy Levels

Workflow OS uses explicit autonomy levels so governed AI work is safe by default.

## Level 1 Assistive

Level 1 is assistive. The workflow may draft, summarize, classify, recommend, or prepare work for humans, but it must not perform governed external side effects autonomously.

If `autonomy_level` is omitted from a v0 workflow spec, Rust parsing defaults it to Level 1.

## Level 2 Guided With Approval

Level 2 is guided execution with approval. The workflow may declare approval-gated behavior and prepare actions that require human approval before governed action.

Workflows with explicit approval requirements should declare `autonomy_level: level_2`.

## Level 3 Conditional Autonomy

Level 3 is conditional autonomy. It is declaration-only in v0. Future runtime behavior must require explicit policy enablement before Level 3 behavior can perform governed action.

Level 3 must never be inferred as a default.

## Level 4 Scaled Automation

Level 4 is scaled automation. It is declaration-only in v0. Future runtime behavior must require explicit policy enablement before Level 4 behavior can perform governed action.

Level 4 must never be inferred as a default.

## Parsing Rules

Accepted values are:

- `level_1` or `level_1_assistive`
- `level_2` or `level_2_guided_with_approval`
- `level_3` or `level_3_conditional_autonomy`
- `level_4` or `level_4_scaled_automation`

Unknown autonomy levels fail during parsing.
