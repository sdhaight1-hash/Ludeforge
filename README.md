# Ludeforge

**Design. Simulate. Refine. Play. Publish.**

Ludeforge is a proposed local-first, system-agnostic engineering environment for original tabletop role-playing games. It treats mechanical definitions, dependencies, executable rules, simulations, tests and publication as views of a shared ruleset, rather than requiring a separate implementation for every game.

**Status:** Scoped Phase 1 Rust authoring kernel and CLI verified with six authored fixtures, 25 tests per platform and a measured release benchmark. Compilation and conformance checks are tracked in [GitHub Actions](https://github.com/sdhaight1-hash/Ludeforge/actions). No executable action runtime or desktop editor yet. [Phase 1 release baseline](benchmarks/2026-11-22-phase1-baseline.md).

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

The minimal Rust workspace includes a shared authoring model and headless CLI. Install Rust stable (manifest minimum Rust 1.85; that minimum is not yet CI-verified. CI tests stable). From repository root:

~~~sh
cargo build --locked --workspace
cargo run --locked -p ludeforge-cli -- validate examples/lantern-academy/project.json
cargo run --locked -p ludeforge-cli -- references examples/council-of-reeds/project.json
cargo run --locked -p ludeforge-cli -- normalize examples/lantern-academy/project.json lantern-normalized.json
cargo test --locked --workspace
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
~~~

Normalization exclusively creates a **new** output file; it never overwrites an existing path. Validation failures emit a JSON array of diagnostics on stderr and exit 1; usage errors exit 2. Validation checks authored structure, types, references, local bounds, pure cross-object predicates, compound uniqueness, and declared relation topology. No action execution yet. See [Phase 1 persistence decision](design/adr/0003-phase1-json-authority.md) and [schema format](schemas/README.md).

Use issues with measurable acceptance criteria, focused branches and reviewed changes. Keep documents and tests aligned with implementation. The repository's licensing decision is outstanding; public visibility alone does not grant reuse rights.

## AI systems instructions

Follow [AGENTS.md](AGENTS.md). Lola is the project coding, research and design collaborator. Record evidence, avoid speculative abstractions, and never report a test or implementation as complete without verification.
