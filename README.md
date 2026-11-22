# Ludeforge

**Design. Simulate. Refine. Play. Publish.**

Ludeforge is a proposed local-first, system-agnostic engineering environment for original tabletop role-playing games. It treats mechanical definitions, dependencies, executable rules, simulations, tests and publication as views of a shared ruleset, rather than requiring a separate implementation for every game.

**Status:** research and repository foundation. No functioning TTRPG engine or desktop editor is claimed yet.

## Project direction

Designers define their own concepts, properties, relationships and rules. Dice, hit points, classes, turns and game masters are optional conventions, not engine requirements. Formalized mechanics should be reproducible and explainable, while informal rules and human adjudication remain valid.

## Project documentation

- [Foundational thesis](THESIS.md)
- [Architecture and system boundaries](design/ARCHITECTURE.md)
- [Feature catalog](FEATURES.md)
- [Phases and acceptance gates](ROADMAP.md)
- [Research program](research/README.md)
- [Architecture decision records](design/adr/README.md)
- [Development process](CONTRIBUTING.md)
- [AI development instructions](AGENTS.md)
- [Development audits](Development-Audits/README.md)
- [Development screenshots](Development-Screenshots/Dev-Screenshots.md)
- [Issue tracker](https://github.com/sdhaight1-hash/Ludeforge/issues)

## Repository layout

| Path | Purpose |
|---|---|
| crates/ | Rust mechanical kernel and later justified libraries |
| apps/ | CLI and desktop application |
| design/ | Design specifications and decision records |
| research/ | Literature, software surveys and experiments |
| examples/ | Original divergent reference rulesets |
| schemas/ | Versioned interchange schemas |
| modules/ | Optional reusable game mechanics |
| tests/ | Cross-component fixtures and conformance cases |
| benchmarks/ | Reproducible performance workloads |
| Development-Audits/ | Evidence-based development assessments |
| Development-Screenshots/ | Dated screenshots and videos |

## Development

Phase 0 research and Phase 1 kernel specification precede implementation. Build commands will be added once a runnable workspace exists. Rust is the proposed core language; UI and storage choices are subject to documented decisions.

Use issues with measurable acceptance criteria, focused branches and reviewed changes. Keep documents and tests aligned with implementation. The repository's licensing decision is outstanding; public visibility alone does not grant reuse rights.

## AI systems instructions

Follow [AGENTS.md](AGENTS.md). Lola is the project coding, research and design collaborator. Record evidence, avoid speculative abstractions, and never report a test or implementation as complete without verification.
