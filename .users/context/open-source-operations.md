<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Open Source Operations — whisper-rs-nx

How whisper-rs-nx operates as an AI-native open source Rust crate.

**Korean mirror**: `.users/context/ko/open-source-operations.md`
**AI context (SoT)**: `.agents/context/open-source-operations.yaml`

## Core Position

**"Design WITH AI, not defend AGAINST AI."**

Most open source projects in 2025-2026 are defending against AI contributions. whisper-rs-nx takes the opposite approach: structurally embrace AI contributions by making the project context so rich that AI-assisted contributions are high quality by default. This fork exists precisely because upstream rejects AI work.

## Five Premises

1. **Minimum environment**: AI coding tool + Git integration is the minimum
2. **Bilateral AI**: Both contributors and maintainers use AI
3. **English lingua franca**: Public sharing on Git is in English; private work logs in native language
4. **Mixed skill levels**: Beginners to experts; AI adapts guidance
5. **Communication flow**: Person → AI → Git (English) → AI → Person

## Contribution Types

| Type | Difficulty | Description |
|------|-----------|-------------|
| Code PR | Medium-High | Pick an issue, submit a PR (Rust code + tests) |
| Bug report | Low | Report build failures, runtime issues |
| New feature | High | Propose or implement new features |
| Documentation | Low-Medium | Improve `.users/context/` docs |
| Context | Medium | Improve `.agents/` context files |
| Translation | Low | Add `.users/context/{lang}/` translations |
| Security | Medium-High | Report via GitHub Security Advisory |

## Quality Strategy

Structure ensures quality, not gatekeeping.

| Level | Name | Mechanism |
|-------|------|-----------|
| L1 | Context | `.agents/` directory — AI understands project before generating code |
| L2 | Automation | CI gates (cargo build, test, clippy, fmt) |
| L3 | Human Judgment | Maintainer review for architecture and direction |

## AI Attribution

- AI usage is welcomed; transparency is appreciated
- Responsibility for AI-generated code lies with the contributor
- Git trailer: `Assisted-by: {tool name}` (recommended, not required)
