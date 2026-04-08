# Spec: Desktop Notification Alarm

## Problem Statement

Tumodori has no alarm mechanism. When a phase ends, the timer silently transitions to `Finished` state. Users must be watching the terminal to notice the transition, which defeats the purpose of a Pomodoro timer.

## Goals

- Notify the user when any phase (Work, Short Break, Long Break) finishes
- Use native desktop notifications for non-intrusive alerts
- Allow users to disable notifications via CLI flag

## Context

The timer loop runs in `app.rs` with 100ms ticks. The transition to `TimerState::Finished` happens inside `timer.rs::tick()`. There is currently no notification, sound, or alert of any kind.

## Requirements

### P1 (MVP)

1. [REQ-001] WHEN a phase (Work, Short Break, or Long Break) reaches zero THEN the system SHALL send a desktop notification via `notify-rust`
2. [REQ-002] The notification SHALL contain the title "tumodori" and a message indicating which phase finished (e.g., "Work session complete!", "Short break is over!", "Long break is over!")
3. [REQ-003] WHEN the user passes the `--no-notify` flag THEN the system SHALL NOT send any notifications

### P2 (Should Have)

4. [REQ-004] The notification SHALL use appropriate system urgency (normal for breaks, critical for work)

## Edge Cases

- If the notification daemon is not running, the app should not crash (handle errors gracefully)
- Multiple rapid phase skips (via `s` key) should each trigger a notification

## Architecture Decisions

- **Crate**: `notify-rust` - lightweight, cross-platform, no audio dependencies
- **Trigger point**: In `app.rs`, detect transition to `TimerState::Finished` in the main loop (track previous state to detect edge)
- **Flag**: `--no-notify` in `Config` via clap (default: notifications enabled)
- **Module**: New `notification.rs` module to encapsulate notification logic

## Constraints

- Depends on a notification daemon (libnotify/dbus on Linux, native on macOS/Windows)
- No custom sounds; relies on system notification sound

## Out of Scope

| Item | Reason |
|------|--------|
| Custom audio/sound files | Unnecessary complexity; desktop notification has native sound |
| Per-phase notification toggle | Simplicity; all phases notify |
| Notification content customization | Not needed for MVP |

## Test Strategy

- Unit: Verify correct notification message is generated for each phase
- Unit: Verify `--no-notify` flag prevents notification calls
- Integration: Manual verification of desktop notification appearance

## Requirement Traceability

| ID | Description | Priority | Status |
|----|-------------|----------|--------|
| REQ-001 | Send desktop notification when phase ends | P1 | Pending |
| REQ-002 | Notification title and phase-specific message | P1 | Pending |
| REQ-003 | `--no-notify` flag disables notifications | P1 | Pending |
| REQ-004 | Appropriate urgency level per phase type | P2 | Pending |
