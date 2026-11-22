# Development roadmap

Each phase is gated by evidence, not calendar dates. Earlier research can continue while later implementation is underway, but no dependent subsystem should bypass its prerequisites.

| Phase | Scope | Exit gate | Status |
|---|---|---|---|
| 0 | Research, design alternatives, fixture selection | Document reference evidence, tradeoffs, initial contracts and evaluation method | Open |
| 1 | Mechanical kernel, canonical storage, validation | Two divergent schemas; lossless round trip; invalid inputs rejected | Planned |
| 2 | Rules, runtime, events, random and human inputs | Atomic actions; stable ordering; replay; suspend/resume demonstrated | Planned |
| 3 | Designer workspace | Original small game without Rust coding | Planned |
| 4 | Probability, dependency analysis and tests | Independent probability agreement and seeded fault detection | Planned |
| 5 | Simulation laboratory | Reproducible experiments and known bounded exploit cases | Planned |
| 6 | Advanced design, modules and versioning | Cross-fixture reuse and tested migration | Planned |
| 7 | Publishing and playtesting | Generated references agree with canonical values; feedback linked to version | Planned |
| 8 | Empirical evaluation | Five divergent fixtures; controlled comparisons; report uncertainty | Planned |

## Immediate critical path

Phase 0: verify prior-art sources and licenses; choose original minimal reference fixtures; settle project identity, value types, persistence semantics and rule execution boundaries. Then implement the smallest Phase 1 vertical slice with a CLI validator and tests. Do not build the desktop shell or specialized studios before establishing the shared contracts.

## Overall targets

See [THESIS.md](THESIS.md#21-proposed-acceptance-criteria). The 100,000-trial benchmark is provisional until workload, hardware, time and accuracy thresholds are specified. No phase is complete merely because documentation exists.
