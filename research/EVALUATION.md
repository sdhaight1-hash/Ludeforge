# Evaluation protocol (draft)

Align experiments with RQ1–RQ8 and H1–H7 in the thesis. Use original fixtures, document ruleset version, environment, input, expected results, actual results and uncertainty.

- Generality: count successfully represented divergent fixture systems; record unsupported constructs and kernel changes.
- Authoring: compare equivalent design/change tasks in structured vs document/spreadsheet environments. Measure completion, errors and revision effort; separate programmer and nonprogrammer experience.
- Validation: inject known broken references, invalid types, cycles and conflicts; measure detection and false positives against independently labeled cases.
- Probability: compare small exact results with independently enumerated outcomes; evaluate approximation error and intervals separately.
- Simulation: preregister workloads, agent policies, seeds, sample size and metrics; compare with controlled fixtures and human observation without equating simulations to real player behavior.
- Publishing: change mechanical values and compare generated references; measure manual prose discrepancies separately.
- Replay: compare event traces across repeated runs under defined version/environment.
- Performance: record CPU, memory, workload, release/debug mode and timing methodology; report distributions, not only best runs.

Treat proposed benchmarks as targets until measured. Human studies require appropriate consent, privacy and approval as applicable.


## Experimental analysis protocol refinement

- Preregister scenario, ruleset revision, model/policy, objective direction and units, bounds, metric, sample size, stopping rule, seed and stream scheme; reserve independent scenario/seed sets for checking proposed optimizations.
- Use exact enumeration for finite reference cases. Compare pruned searches to exhaustive coverage; separately test legality constraints against hand-authored expected cases. Report feasible/optimal/unknown rather than forcing universal balance scores.
- Report Pareto objectives and nondominated witnesses; test interaction-sensitive parameter cases, restricted strategy-set best responses and known stable/unstable feedback traces.
- Pair trials by semantically comparable input identities; if draw topology or meaning changes, use separate streams or justified coupling. Report paired or independent intervals as appropriate, accounting for adaptive search and multiple comparisons. Do not equate finite enumeration variance with sampling confidence.
- Account for every trial outcome including rejection, timeout, limit and runtime error. Record independent validation results, not just favorable completed trials.
- Performance gate remains a separately specified lightweight encounter workload; the current logical partition experiment is not a performance benchmark.

See [analysis contract](../design/ANALYSIS-CONTRACT-v0.4.md) and [comparative experiment study](2026-11-22-comparative-experiments-study.md).

## Phase 0–1 gate refinement

| RQ | Method | Observable evidence |
|---|---|---|
| RQ1 generality | Six original authored fixture slices, later executable procedures | Accepted schemas, required kernel changes, unsupported constructs |
| RQ2 authoring | Equivalent nonprogrammer/programmer editing tasks with form/graph/text | Task completion, errors, correction time, semantic equivalence |
| RQ3 dependency/validation | Independent seeded fault corpus in projects and runtime | Detected and missed faults, false positives, correct causal IDs |
| RQ4 simulation | Predeclare objectives/policies, fixed input receipts and human scenarios | Trace correctness, comparative uncertainty, observed vs predicted outcomes |
| RQ5 publication | Versioned change task and generated value reference comparison | Unresolved references, stale generated values, prose discrepancy |
| RQ6 creativity/usability | Matched open-ended designer tasks, qualitative feedback with consent | Task success, friction, self-reported constraint/expressivity, not automatic creativity score |
| RQ7 narrative/procedural | Lantern and diceless Council plus Wayfarer procedure cases | Exact replay, human authority boundaries, unsupported decisions |
| RQ8 optional AI | Held-out design task comparison with and without proposal assistant | Validity, correction work, cost, latency and observed error rates |

Initial seeded corpus is implemented by Rust conformance and fixture-matrix tests: raw malformed keys/types, dangling refs, arithmetic overflow, false predicates, duplicate tuple, topology and resource budget faults. Probability oracles remain independently hand-enumerated cases; no production probability analyzer yet. [Benchmark/replay specification](../benchmarks/PHASE1-PROTOCOL.md) defines workloads before measurement. Human evaluation must compare matched scenarios and report sample size, recruitment, consent, uncertainty and limitations; no participants or human results claimed. Update RQ mapping against thesis when study instruments are finalized.

### Future controlled human comparison

Preregister matched original design and revision tasks (e.g. alter Lantern rank cap, find affected references, author Council guard). Compare document/spreadsheet baseline to Ludeforge, counterbalance order and use equivalent tasks to reduce practice effects. Define eligibility and collect relevant prior experience with consent; separate outcome scoring from participant identity, avoid retaining private campaign content. Independent rubric: expected mechanical edits, seeded defects repaired/introduced, reference correctness, completion time and abandonment. Separate usability/effort and designer-reported experience from correctness. Report per-task denominators, missing data, paired differences and uncertainty intervals, recruitment bias, sample size and limitations; exploratory small samples do not establish universal improvement. For playtest comparison, fix original ruleset revision, scenario, starting resources, permitted actions and outcome measures; record optional human adjudications and explicitly compare policy coverage and observed outcomes, not infer fun from probability. Obtain approval/consent as applicable before research involving people. Follow-up #9 tracks Phase 8 evaluation; method is predeclared, not executed.
