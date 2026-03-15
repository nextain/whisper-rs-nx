<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->

# Harness Engineering — whisper-rs-nx

Mechanical enforcement of project rules via hooks and progress files.

**Korean mirror**: `.users/context/ko/harness.md`
**AI context (SoT)**: `.agents/context/harness.yaml`

## Concept

"Engineer the environment so the AI never repeats a mistake." — Mitchell Hashimoto, Feb 2026

Text rules get forgotten; mechanical enforcement doesn't.

## Pillars

1. **Hooks**: Claude Code PostToolUse hooks that intercept edits and commands in real-time
2. **Progress files**: JSON session handoff files that survive context compaction and session boundaries

## Hooks

Located in `.claude/hooks/`, registered in `.claude/settings.json`.

### sync-entry-points.js

- **Trigger**: PostToolUse on Edit|Write
- **Purpose**: Auto-sync CLAUDE.md, AGENTS.md, GEMINI.md
- **Behavior**: When any entry point file is edited, copies content to the others (if they exist)

### cascade-check.js

- **Trigger**: PostToolUse on Edit|Write
- **Purpose**: Remind agent about triple-mirror updates
- **Behavior**: Advisory reminders when `.agents/` or `.users/` files are modified

## Progress Files

- **Location**: `.agents/progress/` (gitignored)
- **Format**: JSON
- **Purpose**: Session handoff — survives context compaction and session boundaries

### Schema

```json
{
  "issue": "#5",
  "title": "Short description",
  "project": "whisper-rs-nx",
  "current_phase": "build",
  "decisions": [{ "decision": "...", "rationale": "...", "date": "..." }],
  "surprises": [],
  "blockers": [],
  "updated_at": "2026-03-15T14:30Z"
}
```
