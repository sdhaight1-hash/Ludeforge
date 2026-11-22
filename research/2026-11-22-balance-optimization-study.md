# Balance, optimization and strategic analysis — 2026-11-22

Baseline 58779de00cbddc9f14034c722327603a8d0d069f. Research objective: design tools that show the consequences of mechanics under declared scenarios, not an authoritative universal balance score. Primary sources checked and original bounded JS experiment executed. No external code adopted.

## Research foundation

- [Hernandez et al., Metagame Autobalancing for Competitive Multiplayer Games, CoG 2020, DOI 10.1109/CoG47356.2020.9231762](https://arxiv.org/abs/2006.04419): designer-authored target graph of relative strategic outcomes; simulation-based search minimizes distance to target. Transfer: allow intended asymmetric matchup targets rather than require equal win rates. The competitive experiments do not establish TTRPG fun or generality.
- [OR-Tools CP-SAT](https://developers.google.com/optimization/cp/cp_solver): integer constraints, solution enumeration and distinct solver statuses. Candidate for finite build eligibility and bounded optimization, not mandatory infrastructure. A feasible solution is not proven optimal; UNKNOWN does not show infeasibility. Exact rational scaling must avoid overflow and accidental rounding.
- [Blank and Deb, pymoo](https://www.pymoo.org/), [NSGA-II](https://www.pymoo.org/algorithms/moo/nsga2.html): non-dominated sorting and diversity-aware multi-objective search. Use exhaustive Pareto filtering for small spaces, heuristics only when scale demands. Keep equivalent builds and mutually incomparable objectives visible. Approximate fronts are not exact fronts.
- [SALib](https://github.com/SALib/SALib), [basic workflow](https://salib.readthedocs.io/en/latest/user_guide/basics.html): sample a model input domain, evaluate model, analyze sensitivity using e.g. Morris/Sobol. Compare screening and variance attribution; respect input distribution, dependence and sampling design. Local perturbation alone misses nonlinear interactions.
- [OpenSpiel exploitability implementation](https://github.com/google-deepmind/open_spiel/blob/master/open_spiel/python/algorithms/exploitability.py): best-response gain and NashConv; exploitability helper limited to two-player constant-sum games. Restrict analysis to compatible scenarios and authorized information; don't treat any TTRPG session as zero-sum or assume agents know private facts.

## 1. Build analysis

Generate candidate decisions against actual canonical eligibility rules. Distinguish budget, mutually exclusive groups, prerequisites, conditional grants and derived properties. Use minimal exhaustive enumeration; prune only with sound constraints; validate each accepted candidate against engine. Later compile explicitly supported fragments to integer constraints, or use heuristic search under budget. Never silently drop constraints during translation. Report solution status, objective definition, bounds, exact witness, model version and search coverage.

The original eight-option fixture tests budget, equipment-group exclusivity and a prerequisite. It enumerates 256 subsets, finds 65 legal builds, and computes 32 nondominated builds when minimizing cost and maximizing attack and guard simultaneously. A separately structured pruned search agrees with exhaustive legal IDs. Each excluded build has a dominating front witness. Attack and guard are toy scores, not win rates. Exact front only for this candidate domain.

## 2. Local versus global sensitivity

Use designer input ranges and distributions, not arbitrary parameter changes. Report discrete cliffs, interactions, objective response and uncertainty. A factorial grid finds interactions on small discrete domains; later Morris screening and Sobol indices may be appropriate for independent inputs (correlated distributions need specialized methods). Preserve feasible parameter combinations and do not interpret correlation as causation without design assumptions.

Original response y=10+2a+3ab with independent equiprobable a,b,c in {-1,1}. Exact variance 13. A explains 4/13 as a first-order effect, B has zero first-order effect but 9/13 total effect, C has none. A total effect 1. This demonstrates how a balance knob can look irrelevant in marginal averages but matter strongly through interactions. Total-order indices can sum above one; first/total gap signals interactions, not a universal causal ranking. No Monte Carlo or general sensitivity estimator was implemented.

## 3. Strategic analysis

For explicit finite competitive scenarios, construct policy matchup matrices, with scenario, initial state, objectives, information and random assumptions pinned. Pure strategy dominance, best response and regret are distinct from general balance. Use designer target matrix and per-matchup uncertainty instead of one average. Report strategy coverage: a policy can seem unbeatable merely because relevant counters are absent. Distinguish game-level equilibrium from a restricted policy-set result.

The original rock-paper-scissors zero-sum matrix verifies zero unilateral gain for uniform mixed play; pure mirror play appears tied yet admits unilateral gain of 1 for each participant, and biased mixed play is also exploitable. No game-engine or hidden-information policy implementation. For cooperative or narrative scenarios use goals, contribution distributions, costs, risk and decision diversity, not forced zero-sum utilities.

## 4. Feedback and stability

A positive or negative edge is insufficient to diagnose global behavior. Evaluate a specified model at a specified state, perturb inputs within domains, track transient response, threshold crossings, recurrence and recovery. Local Jacobian/eigenvalue analysis only where differentiable and valid; discrete nonlinear or discontinuous rules need step traces and bounded reachability. Feedback loops can be intentional. In scalar recurrence x[n+1]=k*x[n], |k|<1 damps, |k|>1 amplifies and k=-1 oscillates without damping. Test finite integer toy sequences with initial 8 and six steps; damping reaches 0, reinforcing 512, oscillation alternates ±8. This is illustrative not a real game stability analysis.

## 5. Experiment and evidence

[Original script](prototypes/balance-check.mjs) and [recorded results](prototypes/balance-check-results.json): 15 named assertions passed. Executed function in JS research runtime, standalone Node command not separately verified. Front results, conditional means, explicit best-response gains and perturbation traces recorded. No performance benchmark, external solver, production compiler, Rust runtime or actual playtest.

Recommended next: typed expression compiler, precise experiment contracts and multi-request replay; build deterministic simulator before agent sophistication. Advanced balance is not a prerequisite for minimum Phase 1. See [analysis contract](../design/ANALYSIS-CONTRACT-v0.4.md).
