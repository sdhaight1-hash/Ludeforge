# Resource conservation, scheduling and exploit search — 2026-11-22

## Research foundation

- [Micro-Machinations (Klint and van Rozen, 2013)](https://research.hva.nl/en/publications/micro-machinations-a-dsl-for-game-economies/) and [author's project](https://vrozen.github.io/Micro-Machinations/): formal resource-flow semantics and simulation/analysis on a domain-specific representation. Transfer a resource-flow projection of applicable rules rather than require all RPG mechanics to be pools.
- [Petri net modeling and enabling/firing rules](https://journals.sagepub.com/doi/10.1016/j.jala.2005.04.005), [invariant analysis](https://journals.physiology.org/doi/full/10.1152/ajpcell.00487.2022): an incidence matrix captures changes per transition; a weight vector orthogonal to all columns establishes a linear place invariant for the modeled transitions. Token count is not inherently conserved. Transfer explicit material weights and source/sink classifications. A passed invariant is only about modeled state and transitions.
- [SimPy time and scheduling](https://simpy.readthedocs.io/en/latest/topical_guides/time_and_scheduling.html): sequential deterministic event processing, with same-time events ordered by scheduling sequence. Transfer explicit tie/order semantics, not universal game time.
- [Lamport PlusCal tutorial, safety/liveness](https://lamport.azurewebsites.net/tla/tutorial/session9.html): no finite safety violation does not prove eventual progress. Separate resource conservation/exactly-once checks from fairness, deadlines, starvation and deadlock. A system that rejects everything can be safe but unusable.

## Proposed mechanical model

A transaction declares inputs, effects, output and invariant scopes. A resource ledger distinguishes available stock, reserved stock, in-transit or unfinished work, output, disposal, and sources/sinks. Reservations belong to persistent procedure instances; copying a procedure must not duplicate reservations. Define whether costs are spent on start, per stage or final completion. Cancellation, failed attempts and expiry have explicit policies. Use conservation of a designer-defined weighted dimension only when the designer declares that invariant. A magic system may legitimately create material, but its source policy must be visible.

An optional scheduler uses designer-defined time representation and phases. Explicit ordered steps in the core remain the default. If a later module allows simultaneous submissions, evaluate proposal eligibility using one snapshot, then apply declared arbitration for shared writes/resources. Sequential semantics and simultaneous intent collection are distinct. Never depend on array insertion order or thread completion order. Stable ID tie ordering is reproducible but may create systematic favoritism; allow designer-selected priority, rotating order or recorded random tie, with analysis of fairness. Reject invalid/conflicting effects atomically. No implicit global turns or time requirement.

Events should specify time/phase if used, causal origin, action/procedure ID and disposition. Distinguish immediate, delayed and next-step triggers. Bound events per step and zero-delay chains; diagnose non-progressing loops. Allow cancellation and recovery at explicit checkpoints, no silently disappearing workpieces. Read/write conflicts can be flagged statically but require execution guards or conflict policy, not unconditional conclusions about all reachable states.

## Executed bounded experiment

[Source](prototypes/resource-check.mjs), [results](prototypes/resource-check-results.json). Original JavaScript function executed in research runtime; standalone Node entry point not independently executed.

Tidemill-inspired fixture: 6 units stock, two possible jobs, each requiring 4 units at start. Tick advances work, finishing converts 4 reserved units into 3 product + 1 waste; cancel returns reservation. Initial mass 6 is preserved over all reachable states of the bounded model. Clock bounded to four ticks, no future inputs. Baseline 27 reachable states / 189 transitions with no checked conservation violation. Overproduction mutant and over-refund mutant produce counterexamples. Does not prove all jobs eventually finish. Model fixed to two jobs and permits start even at clock limit, so safety is distinct from liveness.

Priority allocation example with stock 6 and requests A=4, B=4, C=2: deterministic ordering accepts A and C, rejects B, stock 0. The outcome agreed across 6 input permutations. This is prioritized partial allocation, not all-or-nothing batch semantics, and does not prove fairness.

Bounded economic reachability example: convert one unit of A into one B, then sell one B into A. Balanced conversion returns initial state; gain variant sells for two A and produces witness buy->sell, state [2,0] from [1,0]. Since that toy transition has no other guards or capacity, it can repeat. Search depth eight; arbitrary game graphs require declared objectives, feasibility/eligibility, bounds, cost and time horizons. A profitable loop may be intended; classify against designer intent. An algebraic positive net vector alone need not be reachable or repeatable.

## Next gates

- Test multi-request transaction conflict, stale resume, cancellation, exact trace and restoration across persisted state; integrate with earlier protocol.
- Define independent scheduling fixture (clockless game must remain valid), temporal unit/ordering semantics, interruption and missing choice behavior.
- Derive static resource projections and compare against trace evidence; handle conversions, capacities, correlated resources and explicit sinks/sources.
- Compare reachability, bounded optimization, policy simulation and human playtests without overstating evidence. No general solver dependency before requirements and license review.

No Rust runtime or production benchmark introduced.