# Evidence register

| ID | Record | Evidence level | Status |
|---|---|---|---|
| E-001 | [Foundational thesis](../THESIS.md) | Hypotheses and design proposal | Established baseline |
| E-002 | [Sibling repository inspection](2026-11-22-cross-repository-study.md) | Tree, selected source/doc inspection | First pass; no tests executed |
| E-003 | [External survey](2026-11-22-external-prior-art.md) | Primary docs, repository descriptions, publisher abstracts | First pass; no binary or benchmark verification |
| E-004 | [Kernel decisions](../design/KERNEL-RESEARCH.md) | Inference from E-002/003 | Proposed; ADR review pending |
| E-005 | [Reference fixture specifications](../examples/REFERENCE-FIXTURES.md) | Original independently expected cases | Draft; executable validation pending |
| E-006 | [Evaluation](EVALUATION.md) | Draft experimental method | Needs preregistered workload and metrics |
| E-007 | [Kernel source study](2026-11-22-kernel-source-study.md) | Inspected external code and standards | Four source systems; license metadata checked, license text review pending |
| E-008 | [Kernel contract v0.1](../design/KERNEL-CONTRACT-v0.1.md) | Proposed specification | Candidate for Phase 1/2; open decisions remain |
| E-009 | [Bounded protocol and probability results](prototypes/kernel-check-results.json) | Executed finite abstraction and exact enumeration | 22 reachable states, 154 transitions, 5 injected faults detected; 182 bins and 200 thresholds compared |
| E-010 | [Fixture validation results](prototypes/fixture-check-results.json) | Executed isolated prototype | 2 valid fixtures, 13 injected mutations rejected; no production parser or persistence |

| E-011 | [Ontology composition](2026-11-22-ontology-composition-study.md) | Executed finite model and source research | 32 assertions, 24 bounded edit comparisons with hand-indexed full scan |
| E-012 | [Ontology contract delta](../design/ONTOLOGY-CONTRACT-v0.2.md) | Proposed design | Constraints, composition, imports, migration |
| E-013 | [Dynamic dependency study](2026-11-22-dynamic-dependency-study.md) | Executed isolated model with separate oracle | Initial and 16 sequential edit comparisons passed |

| E-014 | [Mathematical engine study](2026-11-22-mathematical-engine-study.md) | Primary docs and executed bounded model | 20 assertions, 162 exact bins across 17 distributions, 14 toy range mappings |
| E-015 | [Mathematical contract v0.3](../design/MATHEMATICS-CONTRACT-v0.3.md) | Proposed specification | Exact numeric semantics, units, modifiers, stochastic receipts |
| E-016 | [Resource and scheduling study](2026-11-22-resource-scheduling-study.md) | Executed isolated finite model | 27 states / 189 transitions, two injected faults detected, six scheduler permutations and bounded economic witness |

| E-017 | [Balance and optimization study](2026-11-22-balance-optimization-study.md) | Source study and executed finite model | 15 assertions, 65 legal builds of 256, 32 nondominated, exact factorial and best-response examples |
| E-018 | [Analysis contract v0.4](../design/ANALYSIS-CONTRACT-v0.4.md) | Proposed design | Experiments, optimization, sensitivity, policy, feedback |
| E-019 | [Comparative experiments](2026-11-22-comparative-experiments-study.md) | Executed isolated finite model | Three coupling constructions, ten partition-order checks, explicit draw alignment and disposition accounting |

Evidence classifications: proposal; documentation claim; inspected source; measured isolated prototype; verified integrated behavior; bounded exhaustive result; formal proof. Never upgrade one category based on inference alone. Negative or inconclusive results remain part of the record. License checks and rights to redistribute are independent of technical feasibility.
