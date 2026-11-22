# Phase 0 multi-request execution and replay study — 2026-11-22

Follow-up to [initial bounded protocol](prototypes/kernel-check.mjs), [resource scheduling](2026-11-22-resource-scheduling-study.md) and prior source study. Reuses principles from Causal Thread and AI-RP-Program recorded in [cross-repository review](2026-11-22-cross-repository-study.md), [Fowler event sourcing](https://martinfowler.com/eaaDev/EventSourcing.html) and [PlusCal safety/liveness](https://lamport.azurewebsites.net/tla/tutorial/session9.html). No code copied. Alternatives: global revision compare-and-swap (simplest), field-specific optimistic validation (more concurrency, harder invalidation and replay), pessimistic reservation/lock (can strand pending decisions and obstruct human play). Choose global revision for first Phase 2 in-memory implementation.

## Proposed lifecycle (scope #4/#13/#8)

| State | Input | Decision |
|---|---|---|
| Absent | authorized validated request with unused ID | Store immutable request/payload and expected mechanical revision; evaluate guards into candidate state; may pause |
| Pending | exactly equivalent request | Return same pending receipt; never overwrite checkpoint |
| Pending | different payload under same ID | Reject without mutation |
| Pending | invalid or unauthorized decision | Reject without consuming checkpoint |
| Pending | authorized cancel | Record cancellation administration, discard candidate; no mechanical revision change |
| Pending | correctly formed decision against changed revision | Terminal stale receipt, no effects |
| Pending | valid decision at matching revision | Continue typed execution using recorded inputs; validate whole candidate; atomic commit of state, terminal event/receipt and mechanical revision |
| Terminal | equivalent retry/resume | Return immutable terminal receipt; do not evaluate or spend again |
| Terminal | changed payload or conflicting terminal decision | Reject; do not replace receipt |

Requests and decisions carry run ID, request ID, rule/model version, initiating authority, expected revision and typed payload. Authenticate authority at caller boundary; rule engine accepts trusted scoped identity only, not arbitrary user-supplied strings as proof. Store normalized request content before accepting identity reuse; compare exact representation or specified collision-resistant receipt hash. Phase 2 use in-memory canonical data equality, no requirement to build a hash layer yet. Pending checkpoints are separate administrative state from committed mechanics. Cancellation and stale receipts must remain visible to audit; no accidental resource transfer. A valid failed attempted action may commit explicit effects; invalid/rejected action cannot. One mechanical revision increment per atomic committed transaction, regardless of event count. Define event sequence and ordinal independently of clock. Global revision invalidates competing pending candidates by design; do not silently rebase human decisions.

Replay modes: (1) apply committed ordered mechanical effects/events to original snapshot and verify revision/receipt journal; (2) re-execute original typed rule with recorded stochastic inputs and human choices, same rule/runtime version and initial state, then compare effects and trace. Do not use live RNG/provider during replay. Administrative decisions recorded separately; terminal receipts materialized together with in-memory transaction. Persistent crash replay is NOT provided by the initial in-memory contract; durable store and recovery tests required before claiming it.

## Executed bounded model

[Source](prototypes/multi-request-check.mjs) and [results](prototypes/multi-request-check-results.json). Executed same function in JS research environment; no standalone Node invocation. Two request IDs, fixed payload, initial stock 2, at most one external edit. Explore all reachable states of finite construction: 173 states and 2249 transitions, no checked violation. Independent event application recomputes stock/revision; validates terminal receipts and unchanged pending base. Four deliberately faulty variants detected: stale approval, duplicate commit, unauthorized cancellation, changed payload. Shortest witness recorded for each. Bounds and assumptions explicit; not proof for arbitrary concurrency, arbitrary actions, actual authority authentication, persistence or production replay. Request generation in this model omits guards, real argument validation and draw handling, and one-event commits only. The production interpreter needs independent conformance tests.

## Open decisions

Specify exact rule statement IR and typed arguments, staged state representation, event schema, resource/effect failure and execution budget; verify rejection/commit and explicit input receipts in actual Phase 2 runtime. Extend fault corpus for mismatched rule version, corrupt event sequence, failed effect validation, missing draw receipt and duplicate terminal decision with altered payload. No production code for actions in Phase 1.
