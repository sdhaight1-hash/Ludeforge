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
