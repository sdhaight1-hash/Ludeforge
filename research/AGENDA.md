# Research agenda (expanded 2026-11-22)

Evidence to date: [external prior-art pass](2026-11-22-external-prior-art.md), [cross-repository study](2026-11-22-cross-repository-study.md), [kernel research decisions](../design/KERNEL-RESEARCH.md), [reference fixtures](../examples/REFERENCE-FIXTURES.md). Research is iterative and does not establish implemented features.

| Priority | Question | Method / reference | Outcome |
|---|---|---|---|
| P0 | What ontology and value semantics suffice? | ConWorld ontology prototypes; inspect reference/descriptor, stable ID and constraint patterns; compare fixed schema vs typed extension | Original divergent fixture competency matrix; model/identity/version ADR |
| P0 | How do rules execute and expose dependencies? | Quest Bound QBScript, LoreKit Cruncher, Ludii, Micro-Machinations, typed expression languages | IR operation inventory; typed reads/writes; bounded execution semantics; source-level review |
| P0 | What can be replayed and who decides? | Causal Thread, Solo-RP-Engine, Ink, AI-RP epoch code, event sourcing, TLA+ | Atomic proposal/decision/commit protocol and independent bounded state checks |
| P0 | What is canonical and how is it persisted? | Gaia, ConWorld, SQLite; file-vs-SQLite scenario tests | Stable serialization, concurrency policy, corruption/failure cases, migrations; one authority |
| P0 | How to verify correctness? | Proptest, finite model checking, mutation testing; independent fixture outcomes | Seeded fault corpus; state invariants; distinguish bounded checks from proof |
| P1 | How to compute stochastic results? | Icepool paper/code, dyce, AnyDice, Troll; exact vs sampling comparison | Probability differential suite; complexity caps; confidence intervals |
| P1 | How to detect harmful interactions? | Mechflow, Machinations, graphs from formulas; AI-NPC-COG crafting | Loop classification; resource flow and exploit fixtures; bounded counterexamples |
| P1 | How to make authoring accessible? | Grimory, Quest Bound, Gaia, ConWorld, visual programming HCI studies | Form vs graph vs text experiments, task metrics; diagnostics |
| P1 | How to publish canonical rules? | RuleBindery, The New World presentation model, The Observatory | Stable references, semantic diff, preview/version bound feedback |
| P2 | How to simulate meaningful strategies? | Agent policies, search, constraint optimization, AI-NPC-COG and AI-RP-Program | Policy interface, reproducible workload and human playtest comparison |
| P2 | How to link external worlds and game systems? | ConWorld, Causal Thread, world-building projects | Optional import/export boundary; no cross-repo runtime dependency by default |

Each investigation records date, question, verified source/revision/license, method, observed facts, limits, alternative, recommendation, reproducible acceptance evidence and impacted issue/ADR. Distinguish documented claims, inspected source, measured prototype and production verification. Private sources must not be reproduced in public artifacts. No dependency is approved by inclusion.

## Scheduled investigations

Use [RESEARCH-SCHEDULE.md](RESEARCH-SCHEDULE.md) to determine actionable entry/exit gates and associated GitHub tickets. This agenda is the topic inventory, not a requirement to complete all research before implementation.
