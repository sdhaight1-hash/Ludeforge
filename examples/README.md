# Reference rulesets

Original, minimal fixtures are the primary architecture tests. No reproduction of commercial game text.

Initial proposed fixtures:
- Class-track: optional classes/levels, d20-style resolution and action prerequisites.
- Diceless council: ranked influence, declared intent and a human adjudication step. No dice, hit points, combat, or mandatory GM.

Later fixtures: percentile skills/anatomy; point-buy modifiers; lifepath generation; clocks and stress; resource economy and grounded conflict; an original end-to-end game. Each must document its expected semantics and invariant tests. See [evaluation protocol](../research/EVALUATION.md).

The Lantern and Council JSON fixtures now declare pure authored constraints (rank limit, pledge influence limit, unique delegate/proposal tuple). Constraint verification runs through the Rust validator. Their runtime rules and aggregate support calculation remain unimplemented. Council support edges serve as descriptive relationships distinct from authoritative numeric Pledge records; no support calculation reads them.

## Expanded Phase 1 fixture matrix

Machine-readable authored slices in [Glass Market](glass-market/project.json), [Tidemill](tidemill-workshop/project.json), [Wayfarer](wayfarer-chronicle/project.json), and [Mosswater](mosswater-crossing/project.json) add point budget, conservation snapshot, narrative progression/acyclic stages, and anatomy reference invariants. Together with Lantern and Council these are six distinct authored data shapes, not six implemented playable RPG engines. See [reference specifications](REFERENCE-FIXTURES.md) for future runtime competency cases.
