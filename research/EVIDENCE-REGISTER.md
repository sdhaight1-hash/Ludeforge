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

Evidence classifications: proposal; documentation claim; inspected source; measured isolated prototype; verified integrated behavior; bounded exhaustive result; formal proof. Never upgrade one category based on inference alone. Negative or inconclusive results remain part of the record. License checks and rights to redistribute are independent of technical feasibility.
