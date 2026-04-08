# tumodori

A terminal-based Pomodoro timer built with [Ratatui](https://ratatui.rs/) and Rust.

## Features

- Configurable work session duration (default: 25 minutes)
- Configurable short break duration (default: 5 minutes)
- Configurable long break duration (default: 15 minutes)
- Automatic long break after a configurable number of sessions (default: 4)
- Desktop notifications with alarm sound when a phase ends
- Big digit timer display, centered in the terminal
- Progress bar with color-coded phases
- Color-coded timer states (idle, running, paused, finished)
- Session counter

## Installation

```bash
cargo install --path .
```

## Usage

```bash
tumodori
```

## Keybindings

| Key | Action |
|-----|--------|
| `Space` | Start / Pause / Resume / Next phase |
| `r` | Reset current timer |
| `s` | Skip to next phase |
| `q` / `Esc` | Quit |

## Configuration

All settings are configurable via CLI flags:

| Flag | Short | Default | Range | Description |
|------|-------|---------|-------|-------------|
| `--work` | `-w` | 25 | 1-1440 | Work session duration in minutes |
| `--short-break` | `-s` | 5 | 1-1440 | Short break duration in minutes |
| `--long-break` | `-l` | 15 | 1-1440 | Long break duration in minutes |
| `--sessions` | `-n` | 4 | 1-100 | Work sessions before a long break |
| `--no-notify` | | false | | Disable desktop notifications |

### Examples

```bash
tumodori                          # defaults (25/5/15, 4 sessions)
tumodori -w 50 -s 10 -l 20 -n 6  # custom durations
tumodori --no-notify              # disable notifications
```

### Audio notifications

When a phase ends, tumodori plays an alarm sound using PipeWire (`pw-play`) or PulseAudio (`paplay`) as fallback. A terminal bell is also emitted. To disable all notifications, use `--no-notify`.

On Linux, the alarm sound file is expected at:
`/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga`

Install it with `sound-theme-freedesktop` (available on most distributions).

## Building from source

```bash
git clone https://github.com/nandomoreirame/tumodori.git
cd tumodori
cargo build --release
```

The binary will be available at `target/release/tumodori`.

## License

[MIT](LICENSE)
