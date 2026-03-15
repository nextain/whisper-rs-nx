<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 기여 가이드 — whisper-rs-nx

whisper-rs-nx에 기여하는 방법. AI 에이전트와 AI 도구를 사용하는 사람을 위한 가이드.

**영문 미러**: `.users/context/contributing.md`
**AI 컨텍스트 (SoT)**: `.agents/context/contributing.yaml`

## 시작하기

1. 클론: `git clone --recursive https://github.com/nextain/whisper-rs-nx.git`
2. 아무 AI 코딩 도구로 열기 (Claude Code, Cursor, Windsurf 등)
3. 모국어로 질문: "이 프로젝트가 뭐고 어떻게 도울 수 있나요?"

## 코드 기여 규칙

**프로세스**: PLAN → CHECK → BUILD (TDD) → VERIFY → CLEAN → COMMIT

- **TDD**: 테스트 먼저 작성 (RED) → 최소한의 코드 (GREEN) → 리팩터링
- **VERIFY**: 실제로 빌드하고 테스트 실행 — clippy만으로는 불충분
- **포맷**: `cargo fmt` (rustfmt)
- **린트**: `cargo clippy --all-targets --all-features -- -D warnings`
- **테스트**: 통합 테스트 우선, 필요시 단위 테스트
- **범위**: 최소 변경 원칙 — 필요한 것만 수정

## 기여 유형

| 유형 | 난이도 | 설명 |
|------|--------|------|
| 버그 리포트 | 낮음 | 빌드 실패, 런타임 이슈, 플랫폼 호환성 문제 보고 |
| 코드 PR | 중상 | 이슈를 선택하고 PR 제출 (코드 + 테스트 + 컨텍스트) |
| 새 기능 | 높음 | 새 기능 제안 또는 구현 |
| 문서화 | 중하 | `.users/context/` 문서 개선 |
| 컨텍스트 | 중 | `.agents/` 컨텍스트 파일 개선 (코드와 동등한 가치) |
| 번역 | 낮음 | `.users/context/{lang}/` 번역 추가 |
| 보안 | 중상 | GitHub Security Advisory를 통해 보고 |

## PR 가이드라인

- **제목 형식**: `type(scope): description` (예: `feat(sys): add cuda-dynamic feature`)
- **유형**: feat, fix, refactor, docs, chore, test
- **크기**: PR당 20개 파일 미만 권장
- **완전성**: 코드 + 테스트 + 컨텍스트 업데이트 = 하나의 PR

### 체크리스트

- [ ] 테스트 포함
- [ ] `cargo test` 통과
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` 통과
- [ ] `cargo fmt --check` 통과
- [ ] 아키텍처 변경 시 컨텍스트 파일 업데이트
- [ ] 새 파일에 라이선스 헤더

## AI 어트리뷰션

- 권장하지만 필수는 아님
- Git 트레일러: `Assisted-by: {도구 이름}`
- AI 사용을 환영하며 투명성을 감사히 여깁니다

## 컨텍스트 규칙

- 모든 `.agents/` YAML 파일에는 SPDX 헤더 (`CC-BY-SA-4.0`) 필수
- 모든 `.users/` MD 파일에는 HTML 라이선스 코멘트 필수
- 변경사항은 세 미러 모두에 전파 (`.agents/` ↔ `.users/context/` ↔ `.users/context/ko/`)

## 라이선스

- **소스 코드**: Apache 2.0
- **AI 컨텍스트**: CC-BY-SA 4.0
