# SiftForge Session Handover Protocol

This file is the standing protocol for preserving project continuity across Codex sessions.

At the start of every new session, the agent should read these files in order:

1. `siftforge-build-blueprint-rust.md`
2. `progress-tracker.md`
3. The newest session handover file matching `sessions/session-history-*.md`, if any exist

At the end of every substantial session, the agent should create a new handover file inside `sessions/` using this filename format:

```text
sessions/session-history-YYYY-MM-DD-N.md
```

Use the current date for `YYYY-MM-DD`. Use `N` as a simple increment if more than one handover is created on the same day.

The handover file should be concise but complete enough for the next session to continue without guessing.

## Required Handover Contents

Each `session-history-*.md` file should include:

```text
# Session History - YYYY-MM-DD

## Session Goal

Short description of what the user asked for in this session.

## Files Changed

- path/to/file
- another/path

## Work Completed

- Concrete completed item
- Concrete completed item

## Current State

Describe the project state after the session ended.

## Verification

- Commands run
- Test results
- Any checks that were skipped

## Decisions Made

- Important technical or product decision
- Reason for the decision if not obvious

## Known Issues

- Bug, risk, incomplete task, or uncertainty

## Recommended Next Steps

1. Next concrete step
2. Next concrete step
3. Next concrete step
```

## Rules For Future Agents

- Do not overwrite this protocol file with session history.
- Do not delete previous `sessions/session-history-*.md` files unless the user explicitly asks.
- Keep `progress-tracker.md` updated whenever meaningful project work is completed.
- If code is changed, record the verification commands and results.
- If tests cannot be run, state why.
- Preserve user changes and do not revert unrelated work.
- Prefer the roadmap in `siftforge-build-blueprint-rust.md` when choosing the next implementation step.
