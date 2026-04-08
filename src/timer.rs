use std::time::{Duration, Instant};

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Work => "Work",
            Phase::ShortBreak => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Finished,
}

pub struct Timer {
    pub phase: Phase,
    pub state: TimerState,
    pub completed_sessions: u32,
    config: Config,
    duration: Duration,
    elapsed: Duration,
    last_tick: Option<Instant>,
}

impl Timer {
    pub fn new(config: Config) -> Self {
        let duration = Duration::from_secs(config.work_minutes * 60);
        Self {
            phase: Phase::Work,
            state: TimerState::Idle,
            completed_sessions: 0,
            config,
            duration,
            elapsed: Duration::ZERO,
            last_tick: None,
        }
    }

    pub fn start(&mut self) {
        self.state = TimerState::Running;
        self.last_tick = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.accumulate_elapsed();
            self.state = TimerState::Paused;
            self.last_tick = None;
        }
    }

    pub fn resume(&mut self) {
        if self.state == TimerState::Paused {
            self.state = TimerState::Running;
            self.last_tick = Some(Instant::now());
        }
    }

    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Idle => self.start(),
            TimerState::Running => self.pause(),
            TimerState::Paused => self.resume(),
            TimerState::Finished => self.advance_phase(),
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.last_tick = None;
        self.state = TimerState::Idle;
    }

    /// Advance the timer. Returns `true` on the tick that transitions to Finished.
    pub fn tick(&mut self) -> bool {
        if self.state != TimerState::Running {
            return false;
        }
        self.accumulate_elapsed();
        self.last_tick = Some(Instant::now());

        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.state = TimerState::Finished;
            self.last_tick = None;
            if self.phase == Phase::Work {
                self.completed_sessions += 1;
            }
            return true;
        }
        false
    }

    pub fn remaining(&self) -> Duration {
        let total_elapsed = self.total_elapsed();
        self.duration.saturating_sub(total_elapsed)
    }

    pub fn progress(&self) -> f64 {
        let total_elapsed = self.total_elapsed().as_secs_f64();
        let total = self.duration.as_secs_f64();
        if total == 0.0 {
            return 1.0;
        }
        (total_elapsed / total).min(1.0)
    }

    pub fn advance_phase(&mut self) {
        let next_phase = match self.phase {
            Phase::Work => {
                if self.completed_sessions.is_multiple_of(self.config.sessions_before_long_break)
                    && self.completed_sessions > 0
                {
                    Phase::LongBreak
                } else {
                    Phase::ShortBreak
                }
            }
            Phase::ShortBreak | Phase::LongBreak => Phase::Work,
        };

        self.phase = next_phase;
        self.duration = self.phase_duration(next_phase);
        self.elapsed = Duration::ZERO;
        self.last_tick = None;
        self.state = TimerState::Idle;
    }

    fn phase_duration(&self, phase: Phase) -> Duration {
        let minutes = match phase {
            Phase::Work => self.config.work_minutes,
            Phase::ShortBreak => self.config.short_break_minutes,
            Phase::LongBreak => self.config.long_break_minutes,
        };
        Duration::from_secs(minutes * 60)
    }

    fn accumulate_elapsed(&mut self) {
        if let Some(last) = self.last_tick {
            self.elapsed += last.elapsed();
        }
    }

    fn total_elapsed(&self) -> Duration {
        let mut total = self.elapsed;
        if self.state == TimerState::Running
            && let Some(last) = self.last_tick
        {
            total += last.elapsed();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quick_config() -> Config {
        Config {
            work_minutes: 0,
            short_break_minutes: 0,
            long_break_minutes: 0,
            sessions_before_long_break: 4,
            no_notify: false,
        }
    }

    #[test]
    fn tick_returns_false_when_idle() {
        let mut timer = Timer::new(quick_config());
        assert!(!timer.tick());
    }

    #[test]
    fn tick_returns_true_when_finished() {
        let mut timer = Timer::new(quick_config());
        timer.start();
        assert!(timer.tick());
        assert_eq!(timer.state, TimerState::Finished);
    }

    #[test]
    fn tick_returns_false_after_already_finished() {
        let mut timer = Timer::new(quick_config());
        timer.start();
        assert!(timer.tick());
        assert!(!timer.tick());
    }

    #[test]
    fn toggle_starts_idle_timer() {
        let mut timer = Timer::new(quick_config());
        timer.toggle();
        assert_eq!(timer.state, TimerState::Running);
    }

    #[test]
    fn toggle_pauses_running_timer() {
        let mut timer = Timer::new(quick_config());
        timer.start();
        timer.toggle();
        assert_eq!(timer.state, TimerState::Paused);
    }

    #[test]
    fn toggle_resumes_paused_timer() {
        let mut timer = Timer::new(quick_config());
        timer.start();
        timer.pause();
        timer.toggle();
        assert_eq!(timer.state, TimerState::Running);
    }

    #[test]
    fn reset_returns_to_idle() {
        let mut timer = Timer::new(quick_config());
        timer.start();
        timer.reset();
        assert_eq!(timer.state, TimerState::Idle);
    }

    #[test]
    fn advance_phase_work_to_short_break() {
        let mut timer = Timer::new(quick_config());
        timer.advance_phase();
        assert_eq!(timer.phase, Phase::ShortBreak);
        assert_eq!(timer.state, TimerState::Idle);
    }

    #[test]
    fn advance_phase_short_break_to_work() {
        let mut timer = Timer::new(quick_config());
        timer.phase = Phase::ShortBreak;
        timer.advance_phase();
        assert_eq!(timer.phase, Phase::Work);
    }

    #[test]
    fn long_break_after_n_sessions() {
        let mut timer = Timer::new(quick_config());
        timer.completed_sessions = 4;
        timer.advance_phase();
        assert_eq!(timer.phase, Phase::LongBreak);
    }

    #[test]
    fn work_session_increments_completed() {
        let mut timer = Timer::new(quick_config());
        timer.start();
        timer.tick(); // finishes immediately (0 duration)
        assert_eq!(timer.completed_sessions, 1);
    }

    #[test]
    fn break_session_does_not_increment_completed() {
        let mut timer = Timer::new(quick_config());
        timer.phase = Phase::ShortBreak;
        timer.start();
        timer.tick();
        assert_eq!(timer.completed_sessions, 0);
    }

    #[test]
    fn progress_is_one_when_duration_is_zero() {
        let timer = Timer::new(quick_config());
        assert_eq!(timer.progress(), 1.0);
    }

    #[test]
    fn phase_labels() {
        assert_eq!(Phase::Work.label(), "Work");
        assert_eq!(Phase::ShortBreak.label(), "Short Break");
        assert_eq!(Phase::LongBreak.label(), "Long Break");
    }
}
