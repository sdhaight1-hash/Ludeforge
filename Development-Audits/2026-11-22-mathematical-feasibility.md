# Mathematical and resource feasibility — 2026-11-22

Starting head 24e480afdab804694d8f3af43c4626d84c0333b1. Mathematical study commit 8f9ad0a48eecda6310d894ec2343d0c30ed6bc62.

Evidence: primary docs for F# unit checking, UCUM, decimal rounding, num-rational, exact dice research, finite sampling, reproducible PRNG, Micro-Machinations, Petri nets, SimPy and safety/liveness. Original math model: 20 assertions, 162 compared bins across 17 finite distributions, 14 finite uniform mapping cases. Resource model: 27 reachable states / 189 transitions, no modeled baseline conservation violation; two faulty variants detected; six competing request orderings agree; bounded trade loop witness. All results committed as synthetic research, not integrated tests.

Remaining: strict parser/persistence, typed compiler, multi-request replay, explicit rounding and bit budgets, actual stateful decks and correlated outcomes, scheduling fairness/liveness, larger resource and exploit fixtures, statistical analysis, third-party license review. Do not delay minimum Phase 1 implementation for advanced math, simulation or UI. No production Rust code, no benchmark, no CI results.