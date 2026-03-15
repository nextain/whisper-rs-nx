<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Lessons Learned — whisper-rs-nx

Accumulated lessons from development cycles.

**Korean mirror**: `.users/context/ko/lessons-learned.md`
**AI context (SoT)**: `.agents/context/lessons-learned.yaml`

## L001 — Upstream rejection of AI contributions motivated this fork

- **Date**: 2026-03-15
- **Category**: Fork management
- **Problem**: Upstream whisper-rs does not accept AI-assisted contributions. This blocks cuda-dynamic and Windows MSVC patches from being upstreamed.
- **Root cause**: Anti-AI contribution policy in upstream project.
- **Fix**: Created permanent fork (whisper-rs-nx) with clear upstream attribution. Maintain our own release cycle.

## L002 — Pre-generated bindings break cross-platform builds

- **Date**: 2026-03-15
- **Category**: Build system
- **Problem**: Upstream ships pre-generated Linux bindings in `sys/src/bindings.rs`. These contain Linux-specific structs that fail on Windows MSVC.
- **Root cause**: bindgen output is platform-specific. Pre-generated bindings assume Linux.
- **Fix**: Updated to bindgen 0.72 with `rust_target` config. Bindings are generated at build time on each platform.
