# Reproducible feasibility experiments

These scripts are original research prototypes, not Ludeforge production libraries. They use only JavaScript standard operations (fixture script uses Node filesystem reads). The functions were executed against the exact fixture objects in the JavaScript research environment; standalone Node CLI invocation was not separately executed. Re-run locally with Node:

~~~sh
node research/prototypes/kernel-check.mjs
node research/prototypes/fixture-check.mjs
~~~

Compare printed JSON to checked-in results. The bounded model explores all reachable states of its finite abstraction. It excludes multiple requests, concurrency, persistence, random replay and actual rule interpretation. It caught five injected faults, including a revision-zero checkpoint replacement bug in the first experiment. The corrected model explicitly distinguishes null from zero. Exact probability comparison is limited to sums of small independent uniform dice.

The fixture check validates two divergent authored project examples and rejects 13 injected mutations. It uses a narrow structured data validator, not a hostile-input parser or complete schema implementation. It demonstrates deterministic normalization for unordered records and fields, rename stability and semantic round trips. It does not yet prove cross-field constraints, pledge uniqueness, symmetric relations, arbitrary types, canonical byte conformance, duplicate JSON-key handling or actual execution. Pledge objects and supports edges in Council are test data for distinct constructs, not two authoritative copies of numeric support: only the Pledge objects contain strength. Remove convenience edges or derive them once the production model defines the relevant projection. No external code copied.

Results in this directory are synthetic feasibility evidence only.

## Ontology follow-up

Run `node research/prototypes/ontology-check.mjs` and `node research/prototypes/dependency-check.mjs`. Their functions ran in the JS research environment; standalone Node invocations remain unverified. Compare JSON outputs to the committed results files. The composition model runs 32 named assertions and 24 limited edit comparisons; the dynamic dependency model maintains a reverse index and compares 16 sequential edits to a separately implemented full-scan oracle. Neither is a production parser, compiler, transaction engine or performance benchmark.


## Mathematics and resources

Run `node research/prototypes/math-check.mjs` and `node research/prototypes/resource-check.mjs`. The functions ran in the JS research environment; standalone Node invocations remain unverified. Results are saved alongside the scripts. Mathematical checks use exact BigInt counts and independent enumeration in limited domains; resource model explores finite states and deliberate fault cases. Neither is a production runtime or performance benchmark.
