<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Contributing — whisper-rs-nx

How to contribute to whisper-rs-nx. For AI agents and humans using AI tools.

**Korean mirror**: `.users/context/ko/contributing.md`
**AI context (SoT)**: `.agents/context/contributing.yaml`

## Getting Started

1. Clone: `git clone --recursive https://github.com/nextain/whisper-rs-nx.git`
2. Open with any AI coding tool (Claude Code, Cursor, Windsurf, etc.)
3. Ask in your native language: "What is this project and how can I help?"

## Code Contribution Rules

**Process**: PLAN → CHECK → BUILD (TDD) → VERIFY → CLEAN → COMMIT

- **TDD**: Write test first (RED) → minimal code (GREEN) → refactor
- **VERIFY**: Actually build and run tests — clippy alone is insufficient
- **Format**: `cargo fmt` (rustfmt)
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Tests**: Integration tests first, then unit tests if needed
- **Scope**: Minimal change principle — only modify what's needed

## Contribution Types

| Type | Difficulty | Description |
|------|-----------|-------------|
| Bug report | Low | Report build failures, runtime issues, platform incompatibilities |
| Code PR | Medium-High | Pick an issue, submit a PR (code + tests + context) |
| New feature | High | Propose or implement new features |
| Documentation | Low-Medium | Improve `.users/context/` docs |
| Context | Medium | Improve `.agents/` context files (equal value to code) |
| Translation | Low | Add `.users/context/{lang}/` translations |
| Security | Medium-High | Report via GitHub Security Advisory |

## PR Guidelines

- **Title format**: `type(scope): description` (e.g., `feat(sys): add cuda-dynamic feature`)
- **Types**: feat, fix, refactor, docs, chore, test
- **Size**: Under 20 files per PR recommended
- **Completeness**: Code + tests + context updates = one PR

### Checklist

- [ ] Tests included
- [ ] `cargo test` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] Context files updated if architecture changed
- [ ] License headers on new files

## AI Attribution

- Recommended but not required
- Git trailer: `Assisted-by: {tool name}`
- AI usage is welcomed; transparency is appreciated

## Context Rules

- All `.agents/` YAML files must have SPDX header (`CC-BY-SA-4.0`)
- All `.users/` MD files must have HTML license comment
- Changes must propagate to all three mirrors (`.agents/` ↔ `.users/context/` ↔ `.users/context/ko/`)

## License

- **Source code**: Apache 2.0
- **AI context**: CC-BY-SA 4.0
