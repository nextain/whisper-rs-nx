<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Lessons Learned — whisper-rs-nx

Accumulated lessons from development cycles.

**Korean mirror**: `.users/context/ko/lessons-learned.md`
**AI context (SoT)**: `.agents/context/lessons-learned.yaml`

**Temporal accuracy rule**: Every entry must be accurate to its time period. problem/root_cause = state at the time of failure. immediate_response = right after. subsequent_fixes = later, with explicit dates. Never project current-state artifacts onto past events.

## L001 — Upstream PR #275 rejected — both AI policy and code quality cited

- **Date**: 2026-03-15
- **Category**: Fork management
- **Problem**: Submitted cuda-dynamic PR to upstream. Rejected for (1) AI policy violation and (2) code quality issues — maintainer described it as "atrocious quality" and "obviously LLM generated".
- **Root cause**: Two issues — upstream's no-AI ban, and code that didn't follow upstream contributing conventions.
- **Immediate response**: Created permanent fork with AI-assisted development allowed. At fork creation, no upstream convention analysis existed — code was written without verifying upstream style.
- **What maintainer detected**: Didn't follow contributing template, code style diverged, patterns that signal AI generation (over-documentation, unnecessary abstractions, hardcoded paths, mixed approaches).
- **Subsequent fixes (2026-03-16, after critical review)**: upstream.yaml (actual code analysis), PreToolUse hook (cargo fmt/clippy blocker), upstream_style_review workflow phase, anti-patterns section, diff minimization tracking. These did NOT exist at the time of original failure.
- **Code violations remaining**: V1-V10 identified but not yet fixed (pending #63 implementation).

## L002 — Pre-generated bindings break cross-platform builds

- **Date**: 2026-03-15
- **Category**: Build system
- **Problem**: Upstream ships pre-generated Linux bindings in `sys/src/bindings.rs`. These contain Linux-specific structs that fail on Windows MSVC.
- **Root cause**: bindgen output is platform-specific. Pre-generated bindings assume Linux.
- **Fix**: Updated to bindgen 0.72 with `rust_target` config. Bindings are generated at build time on each platform.
