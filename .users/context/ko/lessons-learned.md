<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 교훈 — whisper-rs-nx

개발 사이클에서 축적된 교훈.

**영문 미러**: `.users/context/lessons-learned.md`
**AI 컨텍스트 (SoT)**: `.agents/context/lessons-learned.yaml`

## L001 — 업스트림의 AI 기여 거부가 이 포크의 동기

- **날짜**: 2026-03-15
- **카테고리**: 포크 관리
- **문제**: 업스트림 whisper-rs가 AI 지원 기여를 받지 않음. cuda-dynamic과 Windows MSVC 패치의 업스트림 반영이 차단됨.
- **근본 원인**: 업스트림 프로젝트의 반AI 기여 정책.
- **해결**: 명확한 업스트림 어트리뷰션이 포함된 영구 포크(whisper-rs-nx) 생성. 자체 릴리스 사이클 유지.

## L002 — 사전 생성된 바인딩이 크로스 플랫폼 빌드를 깨뜨림

- **날짜**: 2026-03-15
- **카테고리**: 빌드 시스템
- **문제**: 업스트림이 `sys/src/bindings.rs`에 사전 생성된 Linux 바인딩을 포함. Windows MSVC에서 실패하는 Linux 전용 구조체 포함.
- **근본 원인**: bindgen 출력이 플랫폼별로 다름. 사전 생성된 바인딩이 Linux를 가정.
- **해결**: `rust_target` 설정과 함께 bindgen 0.72로 업데이트. 각 플랫폼에서 빌드 시 바인딩을 생성.
