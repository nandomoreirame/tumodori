# tumodori

A terminal-based Pomodoro timer built with [Ratatui](https://ratatui.rs/) and Rust.

## Features

- Configurable work session duration (default: 25 minutes)
- Configurable short break duration (default: 5 minutes)
- Configurable long break duration (default: 15 minutes)
- Automatic long break after a configurable number of sessions (default: 4)
- Progress bar with color-coded phases
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

Default values are defined in `src/config.rs`:

| Setting | Default | Description |
|---------|---------|-------------|
| `work_minutes` | 25 | Work session duration in minutes |
| `short_break_minutes` | 5 | Short break duration in minutes |
| `long_break_minutes` | 15 | Long break duration in minutes |
| `sessions_before_long_break` | 4 | Work sessions before a long break |

## Building from source

```bash
git clone https://github.com/nandomoreirame/tumodori.git
cd tumodori
cargo build --release
```

The binary will be available at `target/release/tumodori`.

## License

[MIT](LICENSE)
