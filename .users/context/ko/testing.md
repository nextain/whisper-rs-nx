<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 테스팅 — whisper-rs-nx

whisper-rs-nx Rust FFI 크레이트의 테스팅 전략.

**영문 미러**: `.users/context/testing.md`
**AI 컨텍스트 (SoT)**: `.agents/context/testing.yaml`

## 철학

통합 우선. 목(mock)된 내부가 아닌 실제 동작을 테스트합니다. sys 크레이트는 빌드 검증(cmake + bindgen)으로 테스트합니다. 고수준 크레이트는 실제 whisper.cpp 호출로 cargo test를 실행합니다.

## 테스트 레이어

### sys 크레이트 (whisper-rs-nx-sys)

빌드 검증 — cmake + bindgen이 성공하면 바인딩이 올바른 것입니다.

```bash
cargo build -p whisper-rs-nx-sys
cargo build -p whisper-rs-nx-sys --features cuda-dynamic
```

### 고수준 크레이트 (whisper-rs-nx)

실제 whisper.cpp 추론을 사용한 통합 테스트:

- **컨텍스트 생성**: 모델 파일로 `WhisperContext::new()`
- **전체 전사**: 오디오 로드 → 추론 실행 → 텍스트 출력 검증
- **파라미터 설정**: 다양한 설정으로 `FullParams` 생성
- **GPU 폴백**: CPU 전용 머신에서 `use_gpu=true` → 자동 폴백

## 명령어

| 명령어 | 설명 |
|--------|------|
| `cargo build` | 빌드 체크 |
| `cargo build -p whisper-rs-nx-sys` | sys 크레이트만 빌드 |
| `cargo test` | 전체 테스트 실행 |
| `cargo test --features cuda-dynamic` | cuda-dynamic 포함 테스트 |
| `cargo clippy --all-targets --all-features -- -D warnings` | 린트 |
| `cargo fmt --check` | 포맷 체크 |
| `cargo audit` | 보안 스캔 (자문) |

## 테스트 태도

테스트는 진단 도구이지 점수판이 아닙니다.

**피해야 할 안티패턴**:
- **어서션 완화**: 근본 원인 조사 없이 실패하는 테스트를 통과시키기 위해 어서션 변경
- **기대값 조작**: 버그가 있는 출력에 맞춰 기대값 업데이트
- **테스트 삭제**: 코드를 수정하는 대신 실패하는 테스트를 삭제하거나 건너뛰기

## CI 파이프라인

모든 push에서:
1. `cargo fmt --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo build`
4. `cargo build --features cuda-dynamic`
5. `cargo test`

PR에서: 위 전체 + Windows MSVC 빌드 체크.
