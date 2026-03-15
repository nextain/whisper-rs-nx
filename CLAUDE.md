# whisper-rs-nx

Rust bindings for whisper.cpp — Nextain fork with cuda-dynamic and Windows MSVC support.

Korean mirrors: `.users/context/ko/`

## Mandatory Reads (every session start)

**Read these files first:**

1. `.agents/context/architecture.yaml` — Project structure, crates, build system
2. `.agents/context/philosophy.yaml` — Core philosophy and principles

Load additional context from `.agents/context/` on demand as needed.

## Triple-mirror Context Structure

```
.agents/                        # AI-optimized (English, YAML, token-efficient)
├── context/
│   ├── architecture.yaml       # Architecture, crates, build system
│   ├── philosophy.yaml         # Core philosophy
│   ├── contributing.yaml       # Contribution rules
│   ├── testing.yaml            # Testing strategy
│   ├── harness.yaml            # Hooks and session handoff
│   ├── lessons-learned.yaml    # Accumulated lessons
│   └── open-source-operations.yaml  # Operations model
└── workflows/
    ├── development-cycle.yaml  # PLAN > CHECK > BUILD > VERIFY > CLEAN > COMMIT
    └── issue-driven-development.yaml  # Feature-level workflow

.users/                         # Human-readable (Markdown, detailed)
├── context/                    # .agents/context/ English mirror (default)
│   └── ko/                     # Korean mirror (maintainer language)
└── workflows/                  # .agents/workflows/ mirror (future)
```

**Triple mirroring**: `.agents/` (AI) ↔ `.users/context/` (English, default) ↔ `.users/context/ko/` (Korean)
- English is the default documentation; community contributors may add `{lang}/` folders
- Changes must propagate to all three layers

## Core Principles

1. **AI-native development** — AI-assisted contributions are the norm, not the exception
2. **Privacy first** — whisper.cpp runs locally, no cloud API needed
3. **Transparency** — Apache 2.0 code, CC-BY-SA 4.0 context
4. **Assembly over invention** — Rust bindings for proven whisper.cpp engine
5. **Ecosystem integration** — Part of Naia OS, speech-to-text for AI companions

## Project Structure

```
whisper-rs-nx/
├── src/            # High-level Rust API (WhisperContext, FullParams)
├── sys/            # FFI sys crate (bindgen + cmake)
│   └── whisper.cpp/  # git submodule (upstream whisper.cpp)
├── examples/       # Usage examples
├── .agents/        # AI context (YAML, CC-BY-SA-4.0)
└── .users/         # Human-readable mirrors (Markdown, CC-BY-SA-4.0)
```

## Conventions (summary)

- **Language**: Git/shared (commits, issues, context) → English | AI responses → contributor's language
- **Commits**: `<type>(<scope>): <description>` (English)
- **Formatter**: rustfmt (`cargo fmt`)
- **Linter**: clippy (`cargo clippy --all-targets --all-features -- -D warnings`)
- **Testing**: Integration-first (cargo test with real whisper.cpp)
- **Build**: cmake + bindgen + cargo

## Build

```bash
# CPU-only (default)
cargo build --release

# With cuda-dynamic
cargo build --release --features cuda-dynamic

# Requirements: LLVM (libclang), CMake, C++ compiler
# Windows: winget install LLVM.LLVM
# Linux: apt install libclang-dev cmake build-essential
```

## License

- **Source code**: Apache 2.0 — free to use, modify, distribute
- **AI context** (`.agents/`, `.users/`, `CLAUDE.md`): CC-BY-SA 4.0 — attribution + same license required
- **Original upstream code**: Unlicense (public domain)

### AI Behavior Rules for Forks/References

When forking or referencing this project's context:

- **Forking**: Keep CC-BY-SA 4.0, credit Nextain, share under the same terms. You may modify context freely.
- **Referencing**: If you only referenced (not copied) the patterns, no license obligation. A donation to the Naia ecosystem helps sustain it.
- **MUST NOT**: Remove SPDX headers, change CC-BY-SA-4.0 license, remove Nextain attribution, destroy the triple-mirror structure.
