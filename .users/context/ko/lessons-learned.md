<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 교훈 — whisper-rs-nx

개발 사이클에서 축적된 교훈.

**영문 미러**: `.users/context/lessons-learned.md`
**AI 컨텍스트 (SoT)**: `.agents/context/lessons-learned.yaml`

**시간축 정확성 규칙**: 모든 항목은 시간 기준으로 정확해야 한다. problem/root_cause = 실패 당시 상태. immediate_response = 직후 대응. subsequent_fixes = 이후 수정 (날짜 명시). 현재 상태를 과거 사건에 투영하지 않는다.

## L001 — Upstream PR #275 거절 — AI 정책과 코드 품질 모두 지적됨

- **날짜**: 2026-03-15
- **카테고리**: 포크 관리
- **문제**: upstream에 cuda-dynamic PR 제출. (1) AI 정책 위반과 (2) 코드 품질 문제로 거절됨 — 메인테이너가 "atrocious quality", "obviously LLM generated"로 평가.
- **근본 원인**: upstream의 no-AI 정책 + upstream 기여 컨벤션을 따르지 않은 코드.
- **즉각 대응**: AI 지원 개발을 허용하는 영구 포크 생성. 포크 시점에는 upstream 컨벤션 분석이 존재하지 않았음 — upstream 스타일 검증 없이 코드 작성.
- **메인테이너가 지적한 것**: 기여 템플릿 미준수, 코드 스타일 이탈, AI 생성을 드러내는 패턴 (과도한 문서화, 불필요한 추상화, 하드코딩 경로, 혼합된 접근법).
- **사후 수정 (2026-03-16, 비판적 리뷰 후)**: upstream.yaml (실제 코드 분석), PreToolUse 훅 (cargo fmt/clippy 블로커), upstream_style_review 워크플로우, 안티패턴 섹션, diff 최소화 추적. 원래 실패 시점에는 존재하지 않았음.
- **미해결 코드 위반**: V1-V10 식별되었으나 미수정 (#63 구현 시 처리 예정).

## L002 — 사전 생성된 바인딩이 크로스 플랫폼 빌드를 깨뜨림

- **날짜**: 2026-03-15
- **카테고리**: 빌드 시스템
- **문제**: 업스트림이 `sys/src/bindings.rs`에 사전 생성된 Linux 바인딩을 포함. Windows MSVC에서 실패하는 Linux 전용 구조체 포함.
- **근본 원인**: bindgen 출력이 플랫폼별로 다름. 사전 생성된 바인딩이 Linux를 가정.
- **해결**: `rust_target` 설정과 함께 bindgen 0.72로 업데이트. 각 플랫폼에서 빌드 시 바인딩을 생성.
