# Reference rulesets

Original, minimal fixtures are the primary architecture tests. No reproduction of commercial game text.

Initial proposed fixtures:
- Class-track: optional classes/levels, d20-style resolution and action prerequisites.
- Diceless council: ranked influence, declared intent and a human adjudication step. No dice, hit points, combat, or mandatory GM.

Later fixtures: percentile skills/anatomy; point-buy modifiers; lifepath generation; clocks and stress; resource economy and grounded conflict; an original end-to-end game. Each must document its expected semantics and invariant tests. See [evaluation protocol](../research/EVALUATION.md).

The Lantern and Council JSON fixtures now declare pure authored constraints (rank limit, pledge influence limit, unique delegate/proposal tuple). Constraint verification runs through the Rust validator. Their runtime rules and aggregate support calculation remain unimplemented. Council support edges serve as descriptive relationships distinct from authoritative numeric Pledge records; no support calculation reads them.
