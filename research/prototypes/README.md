# Reproducible feasibility experiments

These scripts are original research prototypes, not Ludeforge production libraries. They use only JavaScript standard operations (fixture script uses Node filesystem reads). The functions were executed against the exact fixture objects in the JavaScript research environment; standalone Node CLI invocation was not separately executed. Re-run locally with Node:

~~~sh
node research/prototypes/kernel-check.mjs
node research/prototypes/fixture-check.mjs
~~~

Compare printed JSON to checked-in results. The bounded model explores all reachable states of its finite abstraction. It excludes multiple requests, concurrency, persistence, random replay and actual rule interpretation. It caught five injected faults, including a revision-zero checkpoint replacement bug in the first experiment. The corrected model explicitly distinguishes null from zero. Exact probability comparison is limited to sums of small independent uniform dice.

The fixture check validates two divergent authored project examples and rejects 13 injected mutations. It uses a narrow structured data validator, not a hostile-input parser or complete schema implementation. It demonstrates deterministic normalization for unordered records and fields, rename stability and semantic round trips. It does not yet prove cross-field constraints, pledge uniqueness, symmetric relations, arbitrary types, canonical byte conformance, duplicate JSON-key handling or actual execution. Pledge objects and supports edges in Council are test data for distinct constructs, not two authoritative copies of numeric support: only the Pledge objects contain strength. Remove convenience edges or derive them once the production model defines the relevant projection. No external code copied.

Results in this directory are synthetic feasibility evidence only.