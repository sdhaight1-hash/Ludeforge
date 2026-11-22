# Cross-repository architecture study — 2026-11-22

Status: first source inspection, not an exhaustive code audit or a reuse authorization. Read-only review of the owner's accessible public repositories. Private material is intentionally excluded from this public research record. No sibling repository was modified; no external source code, game text, or datasets were copied. Repository trees were inspected for 18 sibling public projects; selected design records and source files were read in 14. Source anchors below are blob SHAs observed on main (links use stable Git commit references where available; descriptive claims are limited to the reviewed excerpts). No sibling test suite was run.

## Priority 1: Causal Thread (original solo RPG)

[Repository](https://github.com/sdhaight1-hash/Casual-Thread-Solo-RPG-Engine); [interpretation.py](https://github.com/sdhaight1-hash/Casual-Thread-Solo-RPG-Engine/blob/main/interpretation.py), blob 19d97a01c3e332deafed124efd6c68c7e94e8804; [causal_thread.py](https://github.com/sdhaight1-hash/Casual-Thread-Solo-RPG-Engine/blob/main/causal_thread.py), blob d8ea4a7e9e91a7e3511c36a264d2a3a75d4c822d.

Observed code: an authority router selects one resolution authority; previously established outcomes can bypass randomness. Interpretation drafts capture base revision and journal hash, proposed operations, origin, supporting facts and visibility. Review flags stale state. The module explicitly leaves semantic interpretation to humans and delegates writes to journaled transactions. README describes no license grant and optional evidence-scoped assistance, not autonomous GM behavior.

Transfer: distinguish resolution authority, choice, proposal and committed outcome. Treat suspended procedures as revision-bound, single-use decisions; persist explicit randomness and a context receipt. Separate author/runtime claims and visibility. Provide original fixtures for duplicate resolution, stale decision, rejection, and replay. Do not port the hard-coded native/TBE routing map into the universal core.

## Priority 2: ConWorld Tools

[Architecture evolution](https://github.com/sdhaight1-hash/ConWorld-Tools/blob/main/Design-Docs/Architectural-Evolutions/universal-worldbuilding-ontology-and-governed-semantic-extensibility.md), blob 207eb2cd0ffc81114d618008cff0695b427290b5; [bounded graph code](https://github.com/sdhaight1-hash/ConWorld-Tools/blob/main/crates/atlas-store/src/graph.rs), blob 3bb5e8e80169a9a186a60190ae7949b287c84e25; [prototype results](https://github.com/sdhaight1-hash/ConWorld-Tools/blob/main/Design-Docs/Research-Studies/ontology-value-prototype/results.txt), blob 9386438c141576e24e8d94f2c64d3eadc8063def.

Observed: accepted design separates stable identities from governed classification, versioned descriptors and semantic profiles. It explicitly rejects unbounded EAV and arbitrary scripts as defaults. The prototype report records 20 equality assertions and 9 invalid cases, a novel category and pinned historical unit interpretation; these are reported synthetic results, not Ludeforge evidence. Rust graph code has budgeted depth-limited traversal and topological cycle checking; traversal returns truncation when another eligible node would exceed budget.

Transfer: identity independent of labels; references to stable schema/property IDs; explicit unknown/missing distinction; typed constraints; bounded query results with truncation. Separate semantic dependency graph, runtime feedback graph and display graph. Adopt the concept of descriptors, but not ConWorld's historical kinds or full bitemporal model. Require determinism for diagnostics and ordering, not arbitrary iteration through hash maps. Separate graph presentation coordinates from semantics.

## Priority 3: WebGL The Broken Empires and Solo RP Engine

[Mechanics functions](https://github.com/sdhaight1-hash/Webgl-The-Broken-Empires/blob/main/src/mechanics.ts), blob 5d3504fbf20da792fbc0e6b0fa9d617eb38565d8; [sheet model](https://github.com/sdhaight1-hash/Webgl-The-Broken-Empires/blob/main/src/characterSheetModel.ts), blob 703aa9feba0b02d4920e7986fabb5a20ded8c548; [worksheet tests](https://github.com/sdhaight1-hash/Webgl-The-Broken-Empires/blob/main/src/soloWorksheetModel.test.ts), blob 9372a9d2dce534e649837021342d0a3f66d4d71c.

Observed: numerical rules have explicit input bounds, supplied roll values, outcome reasons and unsupported boundary cases. Sheet functions derive values from canonical character fields; allocation validation rejects excess expenditure. Tests check editing a selected record preserves neighboring records and rejects missing IDs or oversized inputs. Transfer: rule result explains inputs, comparisons, tie-breaks and limitations; character sheets are projections, not new rules. Use independent fictional percentile fixtures, not source tables or published mechanics text.

[Solo-RP-Engine mechanics limits](https://github.com/sdhaight1-hash/Solo-RP-Engine/blob/main/docs/PUM-MECHANICS.md), blob d2c6c83d0cf76698cfde16e4f6bfc4b97eabd801; [tests](https://github.com/sdhaight1-hash/Solo-RP-Engine/blob/main/test/engine.test.js), blob f541ee08838396cc65ef13fd4ebb6aaaed053a22.

Observed: proposal and confirmation separated; confirmed plot progress derived from confirmed records; tests cover duplicate confirmation, capacity, invalid inputs, references, migration, dice endpoints, round trips and rejected unsupported formats. Transfer: narrative procedures can be first-class without combat; derive progress from committed records when appropriate; distinguish explicit human interpretation from random prompts. Test action identity and retries.

## Priority 4: AI-NPC-COG and AI-RP-Program

[Crafting design](https://github.com/sdhaight1-hash/AI-NPC-COG/blob/main/docs/crafting-architecture.md), blob 7d0ef950e3416078b61ddcf3c512d02ed04da466; [crafting implementation excerpt](https://github.com/sdhaight1-hash/AI-NPC-COG/blob/main/public/crafting.js), blob 39239ea193e04d91ab1c309dc917aafbf283aaef; [executive research](https://github.com/sdhaight1-hash/AI-NPC-COG/blob/main/docs/final-research-cognitive-executive-contract-2026-11-19.md), blob 97167d7ad2196093cc9cc89ebee0a3a9560dd1b9.

Observed: staged recipe definitions specify inputs, tools, operations, timed steps and output. The design describes persistent unfinished work and deterministic authority over production. Executive research distinguishes legal action execution from policy, goal commitment from per-tick evaluation, and relevant invalidation from global churn. The executive contract is proposed, not an implementation claim.
Transfer: add an original interrupted manufacturing fixture. Represent bounded procedures with explicit progress, resource reservation/consumption policy and permitted cancellation. Agent policies select legal actions, never directly grant results. Keep knowledge/permissions distinct from world state. Do not integrate a live cognition engine in Phase 1.

[AI-RP architecture](https://github.com/sdhaight1-hash/AI-RP-Program/blob/main/docs/architecture-character-fidelity-world-harness.md), blob f399f606e9a745a1916bb0fdf5aa63c8d1ac114e; [epoch.mjs](https://github.com/sdhaight1-hash/AI-RP-Program/blob/main/world/epoch.mjs), blob da6e54faa40a615011c42759379a47f3a8e26a9b.

Observed: design distinguishes player authority, NPC proposals and world authority. SQLite triggers increment a persisted mutation version for watched tables inside the write transaction. Transfer: stale-context rejection with explicit revision; optional policy adapters; human-owned choices cannot be silently substituted by AI. Start with coarse project/run revisions, relax to dependency-specific versions only if measured contention justifies it.

## Priority 5: Gaia, World-building-Project, The Observatory, The New World

[Gaia README](https://github.com/sdhaight1-hash/Gaia/blob/main/README.md), blob 81994b6b544814a813ee8ff5b7993d5ea4087b63; [graph editing source](https://github.com/sdhaight1-hash/Gaia/blob/main/src/graph_edit.rs), blob 79e34d4300c7e1fde4f728b0629628ce2e43c825. README describes bounded structural validation before preview evaluation, cancellable worker and temporary-file replacement, noting limits on power-loss durability. Graph edit source separates selection and group position operations and tests only selected nodes change. Transfer: validate graph before execution, keep layout independent, perform slow analysis off UI thread with revision-tagged results; never equate atomic replacement with guaranteed crash durability.

[World-building-Project architecture](https://github.com/sdhaight1-hash/World-building-Project/blob/main/docs/ARCHITECTURE.md), blob 7db680bbe8d4369cbb3e28a5e8521bae58ed7290. Design separates immutable inputs, editable overlays, derived fields and multiple projections; tracks lineage/parameters/seeds and targeted invalidation. Transfer: canonical inputs, derived cache, explicit invalidations; no terrain/render dependencies.

[The Observatory architecture](https://github.com/sdhaight1-hash/The-Observatory-/blob/main/docs/ARCHITECTURE.md), blob 52db14813536ae13a966fbb45da1ec4ee9d88b41. Proposed shared identity, validation, persistence, provenance and multiple projections. Transfer: evidence-linked mechanics, source/assumption distinctions, references and change history; AI suggestions are proposals. No OSINT collection subsystem belongs in Ludeforge.

[The New World design](https://github.com/sdhaight1-hash/The-New-World/blob/main/design/README.md), blob 74a6ca08661d5033c2b6189df0c40229ec5c3a19. Describes presentation-only typed block layouts, stable entity references, validation and preview/publish separation. Transfer: a reusable document/sheet layout language with safe rendering and accessibility; no independent value edits in layouts.

## Secondary references

| Public project | Observed evidence or screening result | Applicability |
|---|---|---|
| [Santa-Aurelia-Vally](https://github.com/sdhaight1-hash/Santa-Aurelia-Vally/blob/main/docs/12-genealogy-architecture.md) | Genealogy design: stable identities, directional vs symmetric relations, rejected ancestry cycles, explicitly unvalidated uncertain dates | Relation-specific invariants; do not forbid all relationship cycles |
| [Hexel-Game-Engine](https://github.com/sdhaight1-hash/Hexel-Game-Engine/blob/main/README.md) | Prototype and conservation-oriented long-term design distinguished | Optional resource ledgers; avoid graphics dependency |
| [Hexel-survival-game](https://github.com/sdhaight1-hash/Hexel-survival-game/blob/main/README.md) | Bounded test fixture, headless seeded generator and stage-specific metrics described | Benchmark boundaries and honest limitations |
| [Voxel-Game-Engine](https://github.com/sdhaight1-hash/Voxel-Game-Engine/blob/main/docs/procedural-world-design-decisions.md) | Authoring assets separated from future runtime; superseded decisions retained | Explicit lifecycle and versioned design decisions |
| Voxel-Survival, World-building, American-Frontair-, Reverse-Engineer-HavenCraft | Tree screened only; no detailed source review this pass | Further review if a concrete requirement emerges |

## Synthesis and gates

1. Causal Thread is the strongest direct conceptual reference for adjudication and replay; ConWorld is the strongest ontology and typed-relationship reference.
2. Separate mechanical evaluation from display, agent behavior and narrative interpretation. No imports of sibling internals.
3. Define the model around stable IDs, typed values, explicit authority and structured diagnostics. Use test fixtures to constrain generality.
4. Add graph read/write extraction, resource conservation checks, and long-running procedural cases as requirements, not immediate core feature inflation.
5. Preserve a reviewed source and license inventory; evaluate any actual reuse in an explicit ADR. Verify functionality by executing Ludeforge's own tests, not transferring sibling-repo test counts.

See [external comparison](2026-11-22-external-prior-art.md), [kernel research decisions](../design/KERNEL-RESEARCH.md), and [reference fixtures](../examples/REFERENCE-FIXTURES.md).