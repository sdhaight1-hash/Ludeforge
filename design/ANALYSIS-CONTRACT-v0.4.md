# Mechanical analysis and optimization contract v0.4 — proposed

Extends kernel v0.1, ontology v0.2, mathematics v0.3. [Research evidence](../research/2026-11-22-balance-optimization-study.md). This governs later phases 4–5; no requirement to implement a solver or specialized balance subsystem in Phase 1.

## Experiments

An experiment manifest pins project semantic revision, compiler/runtime version, scenario initial state, eligible action/policy versions, visibility/authority, parameter domains, sampling distribution, objectives (direction and units), stopping rule, seed scheme, sample size, resource limits and output metrics. Separate hard invariants from soft design targets. Record each run disposition including failure/rejection/timeout; don't silently exclude unfavorable trials. Keep exact and statistical results separate. Experiments are read-only against canonical project; proposed changes become reviewed revisions, never silent auto-balancing.

## Analysis result contract

Every finding includes kind, source revision, method, assumptions, scope/bounds, evidence, witness or counterexample if available, reproducible inputs and relevant rule/entity references. Verdicts: proven under explicit assumptions; exhaustive bounded; heuristic witness; statistical estimate with uncertainty; unsupported/unknown/limit. Solver feasible != optimal; no witness under budget != impossible. Separate actual defect from designer-approved intentional behavior.

## Build and design optimization

Search finite designer-authored choice spaces with hard constraints and derived metrics. Validate generated candidates via canonical execution; no duplicate mechanical implementation in optimizer. Small exact reference enumeration precedes pruning and solver integration. Multi-objective default: expose nondominated alternatives without arbitrary composite score; report ties and distinct builds with equivalent metrics. Weighted ranking only with explicit designer weights and normalization. Warn that front depends on considered scenario set and objective proxy. Search proposals retain legal witness and evaluation evidence.

## Sensitivity

Start with exact scenario grids and one-factor perturbations, then factorial interaction checks, then screened/global sampling if needed. Record parameter domain, probability measure, correlation/feasibility restrictions and output variation. Do not use independent Sobol variance assumptions on correlated input without explicit method. Report first versus total effects and uncertainty. Distinguish critical cliffs and global distributions. Reject invalid candidate configurations instead of simulating them as if legal.

## Strategic behavior

Policy interface receives only its authorized observation and legal action candidates; engine adjudicates attempts. Objectives and policy compute budgets explicit. Include baseline random, heuristic and optimizer policies; optional external AI must not mutate state. Compare matchups across policies and scenarios; include role asymmetry and seat swaps where meaningful, and ties as explicit outcomes. Best-response/equilibrium metrics only when their game assumptions are satisfied; otherwise report bounded profitable deviation or observed policy result. No policy is a substitute for actual player preferences. Designer target graph may specify unequal desired outcomes.

## Feedback

Analyze semantic dependency, interaction and temporal feedback separately. Identify possible reinforcing or balancing loops, then validate with controlled trajectories or bounded witness. No global stability conclusion from cycle detection or local derivative. Safety and liveness independent. Explicit bounds for steps, states, branching, memory and time; retain counterexample traces and report truncation. No automatic deletion/fix of intended instability.

## Reproducible execution

Use stable trial IDs and versioned seed derivation independent of worker ordering. Pair variants by matched scenario/random inputs where semantically appropriate; do not assume different rules consume identical random streams. Save input receipts, candidate revision, observations and failure reason. Report paired uncertainty or independent intervals appropriate to sample design; seed alone is insufficient. Validate serial/parallel equivalence before adding performance claims.

## Gates

1. Exact legal build enumeration and independent expected cases; pruned/solver proposals checked against engine.
2. Known dominated/nondominated examples, conflicting objectives, infeasible and unknown search cases.
3. Known interaction with zero marginal effect but nonzero total effect; discrete cliffs.
4. Known best-response counterexample despite equal average payoff; hidden-information isolation.
5. Bounded feedback, oscillation, conservation and profitable-loop fixtures, no false guarantee of liveness.
6. Reproducible experiment manifests, exact trace and distinct result status; actual user playtest measures separately.

Avoid solver/ML/agent complexity until the engine is executable and baseline evidence exists.
