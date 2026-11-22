# Phase 0–1 constraints and fixture expansion audit — 2026-11-22

Baseline 880a75f892b1e78f9b6de2aee271eba55897e1f8 (locked workflow complete green on both platforms). New implementation ea1d299, tests f4d8605, formatted 451aa47, six original authored slices 30622c43, protocol/benchmark ff90cf9.

## Integrated implementation

Added general typed predicates with statically checked scope/read/ref-read, integer and Boolean ops, short circuit, checked safe arithmetic, explicit missing values, bounded AST depth/nodes and global constraint workload. Added compound unique keys, declared directed max in/out degree and cycle detection, symmetric max degree, constraint identities and static reference edges. All validate before normalizing/indexing. No game-specific categories or new dependencies. Six authored examples include point allocation, material conservation, progression, injury, role and diceless pledge. See [ADR 0004](../design/adr/0004-phase1-constraints.md) and [fixture descriptions](../examples/REFERENCE-FIXTURES.md).

## Verified integration

- [Formatted constraints run](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45067876578) fully green on Windows/Linux, 19 tests per platform (17 core conformance + 2 CLI).
- [Six fixture run](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45067916962) fully green on Windows/Linux; 25 tests per platform (17 core conformance + 6 fixture matrix + 2 CLI), six fixture CLI validations and locked lint/format. Expanded suite includes global constraint budget failure. No arbitrary performance/security/formal correctness proof.
- [Benchmark source revision](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45067986057): both jobs fully green; benchmark example compiled under Clippy.
- [Benchmark workflow](https://github.com/sdhaight1-hash/Ludeforge/actions/runs/45068090726) fully green on Windows/Linux; Linux release benchmark executed successfully. [Measured LF-B01 baseline](../benchmarks/2026-11-22-phase1-baseline.md), 100/1000/10000 generated recruits; no gameplay performance claim.
- Previous intermediate formatting-only failures corrected using exact rustfmt output. No actual runtime rule execution claimed.

## Phase 0 evidence

- [Two-request replay model](../research/2026-11-22-multi-request-replay-study.md): 173 reachable abstract states / 2249 transitions, four injected counterexamples. Not production replay or persistence.
- [Storage probe](../research/2026-11-22-persistence-tradeoff-study.md): 18 equivalent edits, three rollback and three truncated-new-file tests. retain JSON, defer destructive writes and migrations until scoped future implementation.
- [Benchmark protocol](../benchmarks/PHASE1-PROTOCOL.md): preregistered authored sizes 100,1000,10000 recruits, plus separate future 100k encounter target. First authored-kernel release timings now recorded; no measured gameplay throughput yet.

## Next gate

Original Phase 1 core validation scope (#7) now met by six authored slices and independent negative cases. Phase 0 research on executable action IR, multi-request receipts, licenses and future UI remains active; no derived fields, generalized aggregation, compiled action rules, migration or durable runtime. Do not make new feature tickets implicit blockers for bootstrap; explicitly scope additional work. 

## Scoped ticket disposition

#2, #3, #7 and #12 accepted/closed with linked conformance and research evidence; initial #4 rule-entry contract and #9 protocol documented. #1 external license/source checks and #5 project licensing remain open; Phase 2 runtime, #10/#13 implementation research and #16 deeper diagnostics are outstanding. The verified authoring core is not a complete TTRPG engine.
