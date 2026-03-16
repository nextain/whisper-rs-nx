<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Lessons Learned — whisper-rs-nx

Accumulated lessons from development cycles.

**Korean mirror**: `.users/context/ko/lessons-learned.md`
**AI context (SoT)**: `.agents/context/lessons-learned.yaml`

## L001 — Upstream PR #275 rejected — both AI policy and code quality cited

- **Date**: 2026-03-15
- **Category**: Fork management
- **Problem**: Submitted cuda-dynamic PR to upstream. Rejected for (1) AI policy violation and (2) code quality issues — maintainer described it as "atrocious quality" and "obviously LLM generated".
- **Root cause**: Two issues — upstream's no-AI ban, and code that didn't follow upstream contributing conventions.
- **Fix**: Created permanent fork with upstream.yaml (exact upstream conventions), PreToolUse hook (cargo fmt/clippy enforcement), and upstream_style_review workflow phase. Fork allows AI-assisted development while enforcing upstream code quality standards.
- **What maintainer detected**: Didn't follow contributing template, code style diverged, patterns that signal AI generation (over-documentation, unnecessary abstractions).
- **What fork does differently**: upstream.yaml from actual code analysis, PreToolUse blocker, semantic review phase, diff minimization tracking.

## L002 — Pre-generated bindings break cross-platform builds

- **Date**: 2026-03-15
- **Category**: Build system
- **Problem**: Upstream ships pre-generated Linux bindings in `sys/src/bindings.rs`. These contain Linux-specific structs that fail on Windows MSVC.
- **Root cause**: bindgen output is platform-specific. Pre-generated bindings assume Linux.
- **Fix**: Updated to bindgen 0.72 with `rust_target` config. Bindings are generated at build time on each platform.
