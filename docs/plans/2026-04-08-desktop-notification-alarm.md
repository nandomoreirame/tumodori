# Plan: Desktop Notification Alarm

> **Spec:** docs/specs/2026-04-08-desktop-notification-alarm.md

**Goal:** Notify users via desktop notifications when any Pomodoro phase ends.
**Architecture:** New `notification.rs` module encapsulates notify-rust logic. `app.rs` detects state transitions and triggers notifications. `config.rs` gains `--no-notify` flag.
**Tech Stack:** notify-rust, clap
**Total Tasks:** 4
**Estimated Complexity:** small

---

### Task 1: Add `--no-notify` flag to Config

**Requirement:** REQ-003
**Files:**
- Modify: `src/config.rs`

**Step 1: Write the failing test**
```rust
// src/config.rs - add test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_notifications_enabled() {
        let config = Config::parse_from::<[&str; 0], &str>([]);
        assert!(!config.no_notify);
    }

    #[test]
    fn no_notify_flag_disables_notifications() {
        let config = Config::parse_from(["tumodori", "--no-notify"]);
        assert!(config.no_notify);
    }
}
```

**Step 2: Run test to verify it fails**
Run: `cargo test --lib config`
Expected: FAIL - `no field no_notify on type Config`

**Step 3: Write minimal implementation**
Add the `no_notify` field to `Config`:
```rust
/// Disable desktop notifications.
#[arg(long = "no-notify", default_value_t = false)]
pub no_notify: bool,
```

**Step 4: Run test to verify it passes**
Run: `cargo test --lib config`
Expected: PASS

**Step 5: Commit**
Run: `/git commit`

---

### Task 2: Create notification module with phase-specific messages

**Requirement:** REQ-001, REQ-002, REQ-004
**Files:**
- Create: `src/notification.rs`
- Modify: `src/main.rs` (add mod declaration)

**Step 1: Write the failing test**
```rust
// src/notification.rs - test module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::Phase;

    #[test]
    fn work_phase_message() {
        let (title, body, urgency) = notification_content(Phase::Work);
        assert_eq!(title, "tumodori");
        assert_eq!(body, "Work session complete!");
        assert!(matches!(urgency, Urgency::Critical));
    }

    #[test]
    fn short_break_message() {
        let (title, body, urgency) = notification_content(Phase::ShortBreak);
        assert_eq!(title, "tumodori");
        assert_eq!(body, "Short break is over!");
        assert!(matches!(urgency, Urgency::Normal));
    }

    #[test]
    fn long_break_message() {
        let (title, body, urgency) = notification_content(Phase::LongBreak);
        assert_eq!(title, "tumodori");
        assert_eq!(body, "Long break is over!");
        assert!(matches!(urgency, Urgency::Normal));
    }
}
```

**Step 2: Run test to verify it fails**
Run: `cargo test --lib notification`
Expected: FAIL - module not found

**Step 3: Write minimal implementation**
```rust
// src/notification.rs
use notify_rust::{Notification, Urgency};
use crate::timer::Phase;

pub fn notification_content(phase: Phase) -> (&'static str, &'static str, Urgency) {
    let title = "tumodori";
    match phase {
        Phase::Work => (title, "Work session complete!", Urgency::Critical),
        Phase::ShortBreak => (title, "Short break is over!", Urgency::Normal),
        Phase::LongBreak => (title, "Long break is over!", Urgency::Normal),
    }
}

pub fn send(phase: Phase) {
    let (title, body, urgency) = notification_content(phase);
    let _ = Notification::new()
        .summary(title)
        .body(body)
        .urgency(urgency)
        .show();
}
```

**Step 4: Run test to verify it passes**
Run: `cargo test --lib notification`
Expected: PASS

**Step 5: Commit**
Run: `/git commit`

---

### Task 3: Make timer.tick() return whether it just finished

**Requirement:** REQ-001 (transition detection)
**Files:**
- Modify: `src/timer.rs`

**Step 1: Write the failing test**
```rust
// Add to existing timer.rs or create tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn test_config(work_secs: u64) -> Config {
        Config {
            work_minutes: work_secs,  // treat as seconds for fast tests
            short_break_minutes: 1,
            long_break_minutes: 1,
            sessions_before_long_break: 4,
            no_notify: false,
        }
    }

    #[test]
    fn tick_returns_false_when_running() {
        let mut timer = Timer::new(test_config(25));
        timer.start();
        assert!(!timer.tick());
    }

    #[test]
    fn tick_returns_false_when_idle() {
        let mut timer = Timer::new(test_config(25));
        assert!(!timer.tick());
    }
}
```

**Step 2: Run test to verify it fails**
Run: `cargo test --lib timer`
Expected: FAIL - `tick()` returns `()`, not `bool`

**Step 3: Write minimal implementation**
Change `tick()` signature to return `bool` - returns `true` only on the tick that transitions to `Finished`.

**Step 4: Run test to verify it passes**
Run: `cargo test --lib timer`
Expected: PASS

**Step 5: Commit**
Run: `/git commit`

---

### Task 4: Wire notification into the app loop

**Requirement:** REQ-001, REQ-003
**Files:**
- Modify: `src/app.rs`

**Step 1: Integration point**
In `app.rs::run()`, after `self.timer.tick()`, check the returned `bool`. If `true` and `!config.no_notify`, call `notification::send(self.timer.phase)`.

**Step 2: Implementation**
```rust
// In app.rs run() loop, replace:
//     self.timer.tick();
// With:
let just_finished = self.timer.tick();
if just_finished && !self.notify_disabled {
    notification::send(self.timer.phase);
}
```

Store `notify_disabled: bool` in `App` struct, set from `config.no_notify` in `App::new()`.

**Step 3: Run full test suite**
Run: `cargo test`
Expected: PASS

**Step 4: Manual verification**
Run: `cargo run -- -w 1` (1-minute timer, wait for notification)
Run: `cargo run -- -w 1 --no-notify` (verify no notification)

**Step 5: Commit**
Run: `/git commit`
