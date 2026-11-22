# Comparative experiment design and reproducible reductions — 2026-11-22

## Sources

[Glasserman & Yao, Some Guidelines and Guarantees for Common Random Numbers, Management Science 38(6), 1992](https://doi.org/10.1287/mnsc.38.6.884) studies when common random numbers are effective. Shared inputs can induce positive covariance and reduce variance of paired differences, but not in every model. [Stout & Goldie, Keeping the Noise Down, 2008](https://pmc.ncbi.nlm.nih.gov/articles/PMC2761656/) demonstrates synchronization of stochastic inputs across variant simulations. Transfer only the experimental method. No clinical or queueing assumptions are part of Ludeforge.

## Original finite model

A succeeds on d20 >=12; B succeeds on d20 >=10. Exact marginals are 9/20 and 11/20. Delta is 0.10. Enumerate 20 same-roll pairs; independently enumerate all 400 cross-products; also enumerate 20 reverse-roll pairs. Population variance of B-A is 0.09 under matching rolls, 0.495 under independent outcomes, 0.99 under opposite pairing. All have identical marginal probabilities and mean difference. With equal numbers of IID pairs, the independent-to-matched variance ratio is 5.5, while negative covariance makes pairing worse. This is an exact finite model, not a measured Monte Carlo confidence interval or performance result.

A separate check splits the 400 ordered pairs by trial ID into 1, 2, 3, 7 or 16 logical partitions. Integer sufficient statistics agree in both forward and reverse reduction orders (10 cases). Workers were simulated sequentially; no actual parallel runtime tested. A fixed receipt fixture demonstrates adding an incidental draw in one variant shifts positional alignment, but stable semantic draw identities can preserve a matched attack input. This is not proof of statistical independence or a production RNG.

Disposition accounting: 3 completed, 1 rejected, 1 timeout, 1 limit. An experiment manifest must account for all attempted trials; define whether rejected actions are valid modeled outcomes, and never silently remove timeouts/limits. A separate completion rate is needed before an outcome distribution conditional on completion can be interpreted.

[Source](prototypes/comparison-check.mjs), [results](prototypes/comparison-check-results.json). Executed function, not independently invoked standalone Node. All assertions passed.

## Proposed experiment lifecycle

1. Declare question, scenario and variation, hard constraints, objective/units, initial state, policy, replication count, stopping rule and budget before running.
2. Define trial IDs and input event alignment; only share comparable stochastic input across variants. If incompatible, use independent streams. Record draw bindings and per-variant receipts.
3. Perform bounded validation and execution, capturing all dispositions and trace hashes; never mutate authored ruleset.
4. Reduce exact integer counts in canonical order; for floating measurements use a fixed reduction tree or numerically stable accumulation. Record execution environment and version.
5. Estimate paired or independent uncertainty as appropriate, report sample design and failures. If using adaptive stopping or exploring multiple goals, do not pretend fixed-sample unadjusted intervals give confirmatory evidence.
6. For search/optimization, evaluate chosen configuration again on held-out scenarios/seeds; don't use selection data as independent evidence.
7. Produce a read-only experiment report and optional reviewable change proposal. Equivalent seeds are not equivalent semantic input if the draw topology changes.

## Next steps

Implement first headless Rust kernel and independent fixture oracles. Later verify genuine parallel ordering, bounded trial interruption, explicit stateful deck draws and correlated outcomes. Define runtime benchmark separately from statistical trial planning. No added dependencies or production code.