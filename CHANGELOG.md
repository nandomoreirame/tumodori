# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-08

### Added

- Big digit timer display centered in the terminal.
- Configurable work session duration (`--work`, default: 25 min).
- Configurable short break duration (`--short-break`, default: 5 min).
- Configurable long break duration (`--long-break`, default: 15 min).
- Configurable sessions before long break (`--sessions`, default: 4).
- Desktop notifications via `notify-rust` when a phase ends.
- Alarm sound playback via PipeWire (`pw-play`) or PulseAudio (`paplay`) with terminal bell as fallback.
- `--no-notify` flag to disable all notifications.
- Color-coded phases (red for work, green for short break, blue for long break).
- Color-coded timer states (gray idle, yellow paused, white finished).
- Progress bar at the bottom of the terminal.
- Session counter.
- Input validation for all CLI arguments with range checks.
- Keybindings: Space (start/pause/resume), r (reset), s (skip), q/Esc (quit).
- CI pipeline with cargo check, test, clippy, and fmt.
- Binary release workflow for Linux and macOS (x86_64 and aarch64).
- `#![forbid(unsafe_code)]` enforced across the codebase.

[Unreleased]: https://github.com/nandomoreirame/tumodori/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/nandomoreirame/tumodori/releases/tag/v0.1.0
