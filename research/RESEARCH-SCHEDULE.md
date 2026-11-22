# Research along the development path

Status: active planning schedule, 2026-11-22. This document schedules research by **dependency and evidence**, not calendar date. Existing issues #1–#5 and #9 retain their original acceptance criteria. Dedicated research issues refine those parent investigations and future feature gates; they do not duplicate or replace implementation work. No issue is declared completed by creating this schedule.

## Execution policy

- **Start gate:** concrete inputs that make research actionable (e.g. typed IR or real runtime traces).
- **Decision gate:** last safe point to resolve findings before the dependent architecture/implementation is committed.
- **Verification gate:** revisit conclusions against actual integrated behavior; isolated prototypes do not close implementation tickets.
- A research issue blocks only the named dependent capability, not unrelated phases. Research may begin early if it answers an immediate architecture question, but speculative features must not delay the minimum kernel. A phase may have parallel workstreams; a research window spanning several phases is not a continuous mandatory effort.
- Each study must produce a dated note, source/revision/license evidence, alternatives, recommendation, reproducible checks, known limitations, and links to affected issues and ADRs. If evidence contradicts a decision, reopen the relevant question rather than quietly rewriting foundations.
- The project currently has no production engine. Prior experiments are preliminary. Minimum Phase 1 work must proceed once scoped canonical format and identity contracts are ready; advanced math, AI, packaging and balance need not be finalized beforehand.

## Research ticket register

| ID / issue | Domain | Research window | Required decision / evidence before |
|---|---|---|---|
| R01 [#10](https://github.com/sdhaight1-hash/Ludeforge/issues/10) | Typed language compiler/interpreter | P0–2: now, then runtime verification | #8 typed rule execution; minimal AST contract before dependent implementation |
| R02 [#11](https://github.com/sdhaight1-hash/Ludeforge/issues/11) | Visual programming and textual DSL | Late P2–3 | P3 rule editor, LF-09; one canonical IR, lossless supported views |
| R03 [#12](https://github.com/sdhaight1-hash/Ludeforge/issues/12) | Persistence, transactions, recovery | P0–1: now | #3 authority decision, #7 persistence; failure injection before in-place save |
| R04 [#13](https://github.com/sdhaight1-hash/Ludeforge/issues/13) | State machines, procedures, multi-request replay | P0–2: now | #4/#8 action checkpoint, replay, cancellation and conflict contract |
| R05 [#14](https://github.com/sdhaight1-hash/Ludeforge/issues/14) | Character creation, progression, lifepaths | P2 cases; P3–5 prototypes; P6 decision | P6 LF-16 creation/progression |
| R06 [#15](https://github.com/sdhaight1-hash/Ludeforge/issues/15) | Automated agent-based playtesting | P4–5 | P5 agent-driven simulation; deterministic baseline can precede study |
| R07 [#16](https://github.com/sdhaight1-hash/Ludeforge/issues/16) | Debugging and dependency traces | P1–2 instrumentation; P4 full debugger | #6–8 minimum diagnostics; P4 LF-12 debugger |
| R08 [#17](https://github.com/sdhaight1-hash/Ludeforge/issues/17) | Modules, packaging and versioning | P1 identity rules; after runnable fixtures; P6 | P6 LF-17/18 module reuse and migration |
| R09 [#18](https://github.com/sdhaight1-hash/Ludeforge/issues/18) | Character sheets | P3 projections; P6–7 integrated study | P7 LF-20 sheets |
| R10 [#19](https://github.com/sdhaight1-hash/Ludeforge/issues/19) | Rulebooks and publication | P3 reference IDs; P6 study; P7 | P7 LF-19 rulebook/SRD generation |
| R11 [#20](https://github.com/sdhaight1-hash/Ludeforge/issues/20) | Designer UI/UX and accessibility | P1–3: task study before UI framework | P3 workspace commitment and workflows |
| R12 [#21](https://github.com/sdhaight1-hash/Ludeforge/issues/21) | AI-assisted system engineering | P3 conceptual; P4–6 prototypes; P8 evaluation | Optional LF-21 only; never block core |
| R13 [#22](https://github.com/sdhaight1-hash/Ludeforge/issues/22) | Complex probability and numeric budgets | P2–4 | P4 LF-11 advanced probability tools |
| R14 [#23](https://github.com/sdhaight1-hash/Ludeforge/issues/23) | Simulation, balance, optimization, benchmarks | P1–2 workload planning; P3–5 real experiments | P5 LF-14/15 simulation and analysis |
| R15 [#24](https://github.com/sdhaight1-hash/Ludeforge/issues/24) | External ruleset/world interoperability | P6–7 | Optional adapter/export integration only |

## Phase-specific execution sequence

| Phase | Research at entry / during development | Decision or verification gate |
|---|---|---|
| 0 | Continue #1 prior-art/licensing evidence, #2 fixture design, #3 canonical model, #4 runtime semantics, #5 licensing, #9 evaluation. Prioritize R03 store/decoder, R01 minimal typed operators and R04 action contract. | Agree minimal IDs/types/format/validation contract. No need to finish full compiler or simulation research to start #6. |
| 1 | Implement #6/#7 headless slice; R03 strict parser/round trip/recovery spike; R01 type vectors; R07 diagnostic IDs; R08 stable identity rules; R11 task research; #9/R14 preregister lightweight benchmark. | Two divergent authored fixtures validate. Verify strict decoding/normalization and safe write strategy; defer destructive saves if not tested. |
| 2 | R01 typed compilation and evaluation; R04 state transitions/replay; verify R07 trace schema in real runtime; R02 begin after stable IR; R13 stochastic cases; R05 identify character procedure requirements. | #8 executes/replays scenarios and human checkpoints. No graph UI before one authoritative IR; characterize faults and unresolved semantics. |
| 3 | R02 form/graph/text proof, R11 editor workflows and accessibility; begin R09 sheet projections and R10 reference binding; R12 conceptual AI proposals only; R14 scenario/experiment authoring. | Editor authors original game using shared validated model; no second rules representation. |
| 4 | R07 debugger/dependency investigation on real traces; R13 exact/approximate probability and complexity limits; R06 baseline policy research; R14 calibrated analysis methods; R12 optional validated proposal spike. | Seeded fault detection, independent exact probabilities, safe diagnostic limits. |
| 5 | R06 policy comparison and R14 integrated simulation/optimization, resource/scheduling and feedback cases, actual parallel reproducibility, bounded exploit search. Continue R05 progression cases. | Reproducible experiments with explicit uncertainty/coverage; no claim of global balance or human enjoyment. |
| 6 | R05 creation and progression, R08 package pins/import/migrations/reuse; R09/R10 refine using real versioned characters/rules; R12 optional AI and module proposal evaluation; begin R15 after schema/version gates. | Cross-fixture module reuse and explicit tested migrations. No floating edition dependencies. |
| 7 | R09 sheet generation and editing; R10 rulebooks and publication, R15 optional adapters; prepare version-bound playtest data and evaluation under #9. | Canonical references agree across outputs; supported exports and losses explicit. |
| 8 | #9 empirical evaluation of thesis hypotheses, five divergent systems, author tasks, performance and human evidence; R12 comparative AI study if optional feature exists; revisit earlier research against integrated results. | Document actual findings, null/negative evidence and limitations. Do not retroactively label research prototype results as production tests. |

## Immediate work order

1. R03/#3: choose minimal authority and raw file/semantic validation contract. Resolve #5 licensing decision separately; don't import external sources.
2. R01/#4: define minimal pure AST and operation types using original fixtures; R04/#4: settle transaction/decision/replay lifecycle. Both can continue in parallel with the headless kernel once the Phase 1 contract is stable.
3. #6/#7: Rust workspace, parser, typed project, validator, fixture suite and CI. R07 minimum diagnostics from the start; #9 define actual workloads.
4. #8: executable rule/runtime slice with R01/R04 evidence; only then advance toward editor and analysis implementation.
5. Research R02–R15 at their stated entry gates. Do not mark all Phase 0 studies mandatory before writing any code.

## Ongoing management

At each phase transition, audit the issue register, check the dependency/entry gate, record the research decision and verify against current code. Update ROADMAP.md and this schedule together if a research gate moves. Relevant research issues should be cited when creating feature implementation tickets. Use #9 as cross-phase evidence/evaluation coordinator and #1 for upstream source/license checks; a later dedicated ticket belongs to its own window rather than being a blanket blocker.
