# context-rs

A Rust CLI that traces a codebase's module dependency graph and builds a token-optimized XML context payload for LLM prompts.

## What it does

`context-rs` collects focused context for large language models when working on a Rust project. Instead of manually copying files and guessing which modules matter, the tool parses the source, follows `mod` and `use` statements recursively, and packages only the files needed to understand the selected entry point.

## How it works

1. **Seed detection** - Uses Git status to identify the files you are currently working on.
2. **Recursive AST walking** - Parses Rust source to find module and import declarations, then follows them recursively until the full dependency graph is mapped.
3. **HTML dashboard** - Generates a local report showing the code structure, dependency graph, and estimated token cost before you paste anything into an LLM.
4. **Clipboard payload** - Builds an XML packet with system instructions and copies it to the clipboard.

## Tech stack

- Rust, Cargo
- Abstract syntax tree (AST) parsing for Rust modules and imports
- HTML report generation
- Git integration for seed-file detection

## Results / Metrics

No performance benchmarks yet.

## How to run

1. Clone the repository:

```bash
git clone https://github.com/saaga23/context-rs.git
cd context-rs
```

2. Build and run:

```bash
cargo build --release
```

3. Generate the dependency graph and copy the XML payload:

```bash
cargo run -- --smart
```

Or view only the structure map:

```bash
cargo run -- --map
```

## Project note

Built for the Rust Africa Hackathon 2026 in the AI and Developer Tools category.

## License

MIT License (stated in the README badge; no `LICENSE` file is currently present in the repository).

---

## Suggested GitHub metadata

**Suggested descriptions (<=160 chars):**

1. Rust CLI that traces module dependencies and builds XML context payloads for LLMs.
2. Compiler-aware context packager for Rust codebases, built for LLM-assisted development.

**Suggested topic tags:**

`rust`, `developer-tools`, `llm-context`, `ast-parsing`, `codebase-analysis`
