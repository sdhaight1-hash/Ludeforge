# Mathematical semantics v0.3 — proposed delta

Extends [kernel v0.1](KERNEL-CONTRACT-v0.1.md) and [ontology v0.2](ONTOLOGY-CONTRACT-v0.2.md). [Evidence](../research/2026-11-22-mathematical-engine-study.md). Does not expand Phase 1 minimum or prescribe a dependency.

## Types and evaluation

- Phase 1: signed safe integer JSON domain from v0.1, Boolean, text, enum, ref; checked overflow; no implicit rounding/coercion.
- Future exact rational: reduced signed numerator, positive denominator, unique zero; deterministic string encoding, checked digits/intermediate bit budget; explicit conversion to integer with named mode. Add only when required. For integer division, choose explicitly exact rational or named integer rounding; do not overload ambiguous division.
- A quantity carries a designer-defined dimension vector (stable dimension IDs, bounded integral exponents) with optional nominal kind and an explicitly linear scale relative to canonical unit. Addition/comparison require compatible dimensions AND nominal kind; multiplication/division combine exponents with reviewed nominal-kind rules. No implied conversion across different resource/currency types. Absolute affine quantities and dynamic exchange rates deferred.
- Numeric overflow, division by zero, invalid units, missing inputs and exhausted budget produce typed diagnostics; never NaN, infinity or implicit saturation.
- No random or external inputs in a pure expression. Draw and human choice are ordered statements whose results are immutable typed locals.

## Modifier pipeline

Policy declares source selection, grouping, positive/negative aggregation, deterministic ties, ordered stages, cap and named rounding. Reject unknown stages, duplicate/conflicting override definitions unless explicit policy. Result contains base, applied/skipped IDs, phase results and reasons. Exact accumulation within each commutative stage; do not treat early rounding as equivalent to late rounding. Derive static dependency and actual trace from same canonical rule definition. First implementation may express fixed sequence in primitive IR; generalized modifier operator only after fixture proof.

## Stochastic semantics

A distribution descriptor declares outcome domain, weights, replacement policy, dependencies and bounds. Uniform integer is first supported stochastic input. Future finite outcomes and pools use exact nonnegative integer weights and nonzero denominator, finite support. Validate weight and sum bounds. Analyzer can produce exact finite result, exact analytic result where proven, bounded residual result, sampled estimate with uncertainty, or unsupported/limit diagnostic. Never label a truncated or normalized partial distribution exact for the original uncapped mechanic. Separate stateful deck transitions (draw/discard/reshuffle) from independent draws; capture card identity, observed result and visibility in events. Correlation explicit via shared binding or joint distribution.

Use stable versioned PRNG and range-reduction method; seed alone not replay contract. Derive independent trial streams from experiment seed and trial ID, not thread scheduling. Record algorithm versions, draw instruction ID, occurrence, distribution revision, actual result and consumed state. Replays use receipts, never re-roll. Uniform rejection uses limit=floor(domain/range)*range and discards raw values outside limit; specify full-word overflow-safe implementation, cap attempts and handle error. Not a cryptographic or adversarial fairness guarantee.

## Candidate validation gates

- Positive and negative rounding tie vectors, zero divisor and intermediate overflow, canonical rational round trip.
- Conversion equivalence; incompatible dimension and nominal kind rejected; units pinned by definition revision.
- Modifier policy order independent of storage ordering, explicit stage reorder changes behavior, deterministic ties.
- Exact finite counts agree with independently computed small cases including min/max, k=0, all dice, empty deck, invalid dice/pool definitions and weights.
- Stateful without-replacement draw cannot repeat a removed card; replay restores deck state; correlated shared outcome retains dependence.
- Budget limits return explicit status, sampling result includes policy, seed, trial count, uncertainty and limitations.
- No required dice, units, modifiers or numeric resource conventions in engine's authoring model.
