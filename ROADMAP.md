# Development roadmap

Each phase is gated by evidence, not calendar dates. Earlier research can continue while later implementation is underway, but no dependent subsystem should bypass its prerequisites.

| Phase | Scope | Exit gate | Status |
|---|---|---|---|
| 0 | Research, design alternatives, fixture selection | Document reference evidence, tradeoffs, initial contracts and evaluation method | Scoped authoring and rule-entry contracts specified; upstream license/source review and owner project license outstanding |
| 1 | Mechanical kernel, canonical storage, validation | Two divergent schemas; lossless round trip; invalid inputs rejected | Core gate verified; six authored slices, 25 tests on Windows/Linux; LF-B01 release baseline measured; advanced work tracked separately |
| 2 | Rules, runtime, events, random and human inputs | Atomic actions; stable ordering; replay; suspend/resume demonstrated | Planned |
| 3 | Designer workspace | Original small game without Rust coding | Planned |
| 4 | Probability, dependency analysis and tests | Independent probability agreement and seeded fault detection | Planned |
| 5 | Simulation laboratory | Reproducible experiments and known bounded exploit cases | Planned |
| 6 | Advanced design, modules and versioning | Cross-fixture reuse and tested migration | Planned |
| 7 | Publishing and playtesting | Generated references agree with canonical values; feedback linked to version | Planned |
| 8 | Empirical evaluation | Five divergent fixtures; controlled comparisons; report uncertainty | Planned |

## Research timing and gates

[Research schedule and ticket register](research/RESEARCH-SCHEDULE.md) governs when each investigation starts, what decision it gates, and when it must be verified against implementation. It includes fifteen dedicated research workstreams (#10–#24) plus existing early research/evaluation tickets. Research does not constitute production completion.

| Development phase | Research decision gates |
|---|---|
| 0–1 | R03 canonical persistence; R01 minimum types; R04 runtime lifecycle; R07 diagnostics; R08 identity rules |
| 2 | R01 executable IR; R04 atomic replay; R07 traces; begin R02 after stable IR and R13 after stochastic semantics |
| 3 | R02 authoring round trips; R11 usability; begin R09/R10 projection/reference design |
| 4 | R07 debugger; R13 probability; R06 policy prerequisites |
| 5 | R06 agency and R14 reproducible simulation/optimization |
| 6 | R05 character/progression; R08 packaging/migration; refine R09/R10 |
| 7 | R09 sheets; R10 publication; R15 optional interoperability |
| 8 | #9 empirical evaluation; R12 optional AI evaluation and integrated revalidation |

R11 task research may begin in Phase 1; R05 competency cases in Phase 2; R14 workload definition in Phases 1–2; R12 conceptual work after validated authoring (Phase 3). Exact cutoffs and exceptions are in the schedule. Advanced research does not block the minimum Phase 1 kernel.

## Immediate critical path

Phase 0: verify prior-art sources and licenses; choose original minimal reference fixtures; settle project identity, value types, persistence semantics and rule execution boundaries. Then implement the smallest Phase 1 vertical slice with a CLI validator and tests. Do not build the desktop shell or specialized studios before establishing the shared contracts.

## Overall targets

See [THESIS.md](THESIS.md#21-proposed-acceptance-criteria). The 100,000-trial benchmark is provisional until workload, hardware, time and accuracy thresholds are specified. No phase is complete merely because documentation exists.

## Current verified gate and deferred successors

Initial Phase 0/1 scoped contracts and authoring acceptance: #2/#3/#4/#6/#7/#9/#12 closed. Remaining #1 upstream verification and #5 owner project license; Phase 2 implementation #8 with #10/#13/#16 integration work. [Phase 2 entry contract](design/PHASE2-RULE-ENTRY-CONTRACT.md) is specification, not runtime. Do not treat safe in-place writes as implemented: [#25](https://github.com/sdhaight1-hash/Ludeforge/issues/25) gates editor persistence. [#26](https://github.com/sdhaight1-hash/Ludeforge/issues/26) gates integrated Phase 8 evaluation using the completed protocol.
