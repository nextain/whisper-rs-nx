<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Testing — whisper-rs-nx

Testing strategy for the whisper-rs-nx Rust FFI crate.

**Korean mirror**: `.users/context/ko/testing.md`
**AI context (SoT)**: `.agents/context/testing.yaml`

## Philosophy

Integration-first. Test real behavior, not mocked internals. The sys crate is tested via build verification (cmake + bindgen). The high-level crate is tested via cargo test with real whisper.cpp calls.

## Test Layers

### sys crate (whisper-rs-nx-sys)

Build verification — if cmake + bindgen succeed, bindings are correct.

```bash
cargo build -p whisper-rs-nx-sys
cargo build -p whisper-rs-nx-sys --features cuda-dynamic
```

### High-level crate (whisper-rs-nx)

Integration tests with real whisper.cpp inference:

- **Context creation**: `WhisperContext::new()` with a model file
- **Full transcription**: Load audio → run inference → verify text output
- **Parameter configuration**: `FullParams` construction with various settings
- **GPU fallback**: `use_gpu=true` on CPU-only machine → graceful fallback

### Feature gating

- Default (CPU-only) build
- cuda-dynamic feature build
- Cross-platform build (Linux + Windows)

## Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Build check |
| `cargo build -p whisper-rs-nx-sys` | Build sys crate only |
| `cargo test` | Run all tests |
| `cargo test --features cuda-dynamic` | Test with cuda-dynamic |
| `cargo clippy --all-targets --all-features -- -D warnings` | Lint |
| `cargo fmt --check` | Format check |
| `cargo audit` | Security scan (advisory) |

## Test Attitude

Tests are diagnostic tools, not scoreboards.

**Anti-patterns to avoid**:
- **Assertion loosening**: Changing assertions to make a failing test pass without investigating root cause
- **Expected value gaming**: Updating expected values to match buggy output
- **Test deletion**: Deleting or skipping a failing test instead of fixing the code

## CI Pipeline

On every push:
1. `cargo fmt --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo build`
4. `cargo build --features cuda-dynamic`
5. `cargo test`

On PR: All above + Windows MSVC build check.
