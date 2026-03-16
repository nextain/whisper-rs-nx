# whisper-rs-nx

Rust bindings to [whisper.cpp](https://github.com/ggerganov/whisper.cpp/) — **Nextain fork** with cross-platform and GPU improvements.

Forked from [whisper-rs](https://codeberg.org/tazz4843/whisper-rs) (Unlicense).

## Improvements over upstream

| Feature | Upstream | whisper-rs-nx |
|---------|----------|---------------|
| **cuda-dynamic** | Not available | Runtime CUDA loading via dlopen/LoadLibrary — single binary works with or without GPU |
| **Windows MSVC** | Fails (Linux struct bindings) | Builds cleanly with bindgen 0.72 + LLVM |
| **Windows DLL** | Not supported | cuda-dynamic finds and loads `ggml-cuda.dll` on Windows |
| **CPU fallback** | Requires compile-time CUDA decision | GPU available = CUDA; no GPU = CPU. Automatic at runtime |

## Why this fork exists

This project is part of [Naia OS](https://github.com/nextain/naia-os). Our development workflow includes AI-assisted coding with context engineering (`.agents/`, `.users/`, `CLAUDE.md`).

An initial contribution (cuda-dynamic) was submitted to upstream whisper-rs as PR #275 and rejected for two reasons:

1. **AI policy**: upstream does not accept AI-assisted contributions
2. **Code quality**: the submission did not follow upstream contributing conventions

We created this fork to address both. The fork uses AI tools with transparent attribution, and enforces upstream code conventions via `.agents/context/upstream.yaml` and commit-time hooks to ensure code quality matches upstream standards. We respect upstream's policy and do not submit AI-assisted code to their repository.

## Usage

```toml
[dependencies]
whisper-rs = { package = "whisper-rs-nx", git = "https://github.com/nextain/whisper-rs-nx.git" }
```

The `package` alias lets you use `whisper_rs` in your Rust code without any changes:

```rust
use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
```

### Features

```toml
# CPU-only (default, works everywhere)
whisper-rs = { package = "whisper-rs-nx", git = "https://github.com/nextain/whisper-rs-nx.git" }

# Runtime CUDA loading (single binary, GPU used if available)
whisper-rs = { package = "whisper-rs-nx", git = "https://github.com/nextain/whisper-rs-nx.git", features = ["cuda-dynamic"] }

# Static CUDA linking (requires CUDA toolkit at build time)
whisper-rs = { package = "whisper-rs-nx", git = "https://github.com/nextain/whisper-rs-nx.git", features = ["cuda"] }
```

### Build requirements

| Platform | Requirements |
|----------|-------------|
| Linux | `libclang-dev`, `cmake`, C++ compiler |
| Windows | LLVM (`winget install LLVM.LLVM`), CMake, VS Build Tools 2022 |

## License

- **Source code**: Apache 2.0
- **AI context** (`.agents/`, `.users/`, `CLAUDE.md`): CC-BY-SA 4.0
- **Original upstream code**: Unlicense (public domain)

See [LICENSE](LICENSE) for details.

## Contributing

AI-assisted contributions are welcome. We use AI context engineering as a core development practice. See `CLAUDE.md` for project context.
