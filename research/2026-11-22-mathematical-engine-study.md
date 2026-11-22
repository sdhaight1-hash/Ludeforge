# Mathematical engine research — 2026-11-22

Baseline 24e480afdab804694d8f3af43c4626d84c0333b1. Method: targeted primary documentation and original finite JavaScript experiments; no third-party code adopted.

## Sources and implications

| Primary source | Relevant mechanism | Proposed transfer |
|---|---|---|
| [F# units of measure](https://learn.microsoft.com/dotnet/fsharp/language-reference/units-of-measure) | Algebraic dimensions and compile-time checking; conversion constants are explicit | Typed dimensions for user-created units, canonical exponent maps, no implicit incompatible conversions |
| [UCUM](https://ucum.org/ucum) | Standardized physical unit codes; distinguishes special non-ratio units | Optional physical-unit catalog, not mandatory SI or blanket linear conversions |
| [General Decimal Arithmetic](https://speleotrove.com/decimal/) and [dec rounding](https://docs.rs/dec/latest/dec/enum.Rounding.html) | Explicit precision and rounding modes | Expose floor, ceil, toward zero, nearest ties-even and nearest ties-away with negative tests |
| [num-rational](https://docs.rs/num-rational/latest/num_rational/) | Exact machine-sized and arbitrary-precision ratios; checked operations | Candidate dependency after license review; use checked bounded arithmetic and explicit division errors |
| [Liu, Icepool (2022)](https://ojs.aaai.org/index.php/AIIDE/article/view/21971) | Dynamic programming with exact integer weights over multiset outcomes | Face-count algorithms for keep-highest and future dice-pool operations; independent enumeration oracle |
| [Hypergeometric distribution](https://stats.libretexts.org/Bookshelves/Introductory_Statistics_%28Lane%29/05%3A_Probability/5.11%3A_Hypergeometric_Distribution) | Counting outcomes without replacement | Cards, token bags and weighted finite populations must distinguish replacement and persistent depletion |
| [Rust rand portability](https://rust-random.github.io/book/portability.html), [rand_chacha](https://docs.rs/rand_chacha/latest/rand_chacha/) | Named portable generator candidates versus default algorithms that can change | Version generator AND sampling algorithm; record explicit draw receipts; replay from recorded inputs |

## Numerical options

Binary floating-point: performant, useful for estimates and plotting; unsuitable as the sole authoritative representation when exact thresholds, ties, decimal quantities and replay matter. Fixed-point: predictable storage and arithmetic for known scales; cannot represent thirds exactly; intermediate rounding affects results. Rational: supports exact fractional operations and unit conversions, but numerator/denominator growth and computational cost must be bounded. Decision candidate: Phase 1 safe integers unchanged, then optional bounded normalized rational type for actual fractional rules. Do not introduce arbitrary precision to all project properties. Exact outcome weights may need separate larger BigInt capacity than runtime quantities. Never silently fall back to floating point; return limit diagnostics, offer explicitly labeled sampling as alternative.

Represent rational as canonical numerator/positive denominator pair with integer strings when exceeding safe JSON range. Define parsing, gcd normalization, zero as 0/1; cap input digits, bit size, operation count and intermediate allocation. Measure before choosing fixed-width checked vs bounded arbitrary precision. Rounding only at explicit nodes; model signed examples and overflow policies. Ratios with distinct gameplay semantics can share physical dimensions (currency and points should remain separate nominal quantities). A percentage bonus is not an automatic currency conversion. Affine absolute temperatures and deltas, variable exchange rates and calendar units require separate rules, not linear scale catalog.

## Modifier semantics

A designer-defined ordered pipeline: collect applicable entries; evaluate amount and grouping key from same declared snapshot; aggregate positive/negative contributions using specified strategy; choose stable tie policy; apply ordered stages (add, multiply, override, cap, explicit rounding); record applied and excluded contributors with reasons. Storage order must not determine results. A cap before multiplication is not the same as a cap afterward. Do not assume game-specific bonus categories; duplicate source handling and conflicting overrides must be explicit. Static analyzers should identify undeclared ordering and potential shared writes.

## Outcome distribution model

Represent finite weighted outcomes as integer counts; total positive weight is denominator. Keep counts exact until rendering. For independent weighted dice, convolution combines sums; for identical fair keep-highest dice, enumerate face counts descending, choose c of remaining labeled dice via binomial coefficient, track remaining dice, retained dice and sum. Compare with enumerating ordered sequences and sorting. Complexity depends on support and state count, not only number of dice. Deck hypergeometric counts are suitable for binary unordered categories only; order-dependent card rules need stateful draw semantics and actual card identities. A capped explosion is exact for that capped rule; as approximation to uncapped rule report omitted continuation mass and never renormalize without indicating what changed. Rerolls need explicit replacement/once/until semantics; not covered by this experiment.

For a single fair s-sided exploding die with at most d additional rolls, probability of requiring an extra roll beyond cap is s^-(d+1). An unbounded rule needs analytic solution or a reported residual bound. Expected-value error is not equal to residual probability. Opposed rolls may share outcomes and hence cannot always be treated as independent.

## Executed results

[Original source](prototypes/math-check.mjs) and [captured results](prototypes/math-check-results.json). Function executed in JS research runtime; standalone Node launcher not separately verified.
- 20 named arithmetic, rounding, dimensions and modifier assertions.
- 7 keep-highest configurations; 6 deck count configurations; 4 capped explosion configurations matched independent enumeration, 162 bins in total. Full mass checked for each distribution.
- 14 finite raw-word rejection mappings showed identical accepted count per outcome. This validates a toy mapping, not randomness quality or production sampling.

No measured runtime performance, full formal proof, Rust execution, arbitrary precision budget, general distributions, weighted heterogeneous keep-highest or correlated shared rolls verified. BigInt in the experiment is unbounded; production must enforce limits.

## Next research

Typed compiler (type and dimension inference, explicit conversion nodes, effects separate from expressions), probability complexity budget and precision policy, stateful decks, correlated choices, bounded rerolls and exact/approximate analysis interface. Then explore scheduling, conservation, feedback and bounded exploit discovery. See [mathematical contract](../design/MATHEMATICS-CONTRACT-v0.3.md).
