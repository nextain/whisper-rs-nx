<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# 하네스 엔지니어링 — whisper-rs-nx

훅과 진행 파일을 통한 프로젝트 규칙의 기계적 강제.

**영문 미러**: `.users/context/harness.md`
**AI 컨텍스트 (SoT)**: `.agents/context/harness.yaml`

## 개념

"환경을 엔지니어링하여 AI가 같은 실수를 반복하지 않게 한다." — Mitchell Hashimoto, 2026년 2월

텍스트 규칙은 잊히지만, 기계적 강제는 그렇지 않습니다.

## 핵심 요소

1. **훅**: 실시간으로 편집과 명령을 가로채는 Claude Code PostToolUse 훅
2. **진행 파일**: 컨텍스트 압축과 세션 경계를 넘어 유지되는 JSON 세션 핸드오프 파일

## 훅

`.claude/hooks/`에 위치, `.claude/settings.json`에 등록.

### sync-entry-points.js

- **트리거**: Edit|Write에 대한 PostToolUse
- **목적**: CLAUDE.md, AGENTS.md, GEMINI.md 자동 동기화
- **동작**: 엔트리 포인트 파일이 편집되면 다른 파일로 내용을 복사 (존재하는 경우)

### cascade-check.js

- **트리거**: Edit|Write에 대한 PostToolUse
- **목적**: 트리플 미러 업데이트 알림
- **동작**: `.agents/` 또는 `.users/` 파일이 수정될 때 알림

## 진행 파일

- **위치**: `.agents/progress/` (gitignored)
- **형식**: JSON
- **목적**: 세션 핸드오프 — 컨텍스트 압축과 세션 경계를 넘어 유지

### 스키마

```json
{
  "issue": "#5",
  "title": "간단한 설명",
  "project": "whisper-rs-nx",
  "current_phase": "build",
  "decisions": [{ "decision": "...", "rationale": "...", "date": "..." }],
  "surprises": [],
  "blockers": [],
  "updated_at": "2026-03-15T14:30Z"
}
```
