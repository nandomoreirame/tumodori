use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::config::Config;
use crate::notification;
use crate::timer::Timer;
use crate::ui;

const TICK_RATE: Duration = Duration::from_millis(100);

pub struct App {
    pub timer: Timer,
    pub should_quit: bool,
    notify_enabled: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let notify_enabled = !config.no_notify;
        Self {
            timer: Timer::new(config),
            should_quit: false,
            notify_enabled,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| ui::draw(frame, &self.timer))?;

            if event::poll(TICK_RATE)?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.handle_key(key.code);
            }

            let just_finished = self.timer.tick();
            if just_finished && self.notify_enabled {
                notification::send(self.timer.phase);
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }

    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char(' ') => self.timer.toggle(),
            KeyCode::Char('r') => self.timer.reset(),
            KeyCode::Char('s') => self.timer.advance_phase(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timer::{Phase, TimerState};

    fn test_app() -> App {
        App::new(Config {
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            sessions_before_long_break: 4,
            no_notify: true,
        })
    }

    #[test]
    fn quit_on_q() {
        let mut app = test_app();
        app.handle_key(KeyCode::Char('q'));
        assert!(app.should_quit);
    }

    #[test]
    fn quit_on_esc() {
        let mut app = test_app();
        app.handle_key(KeyCode::Esc);
        assert!(app.should_quit);
    }

    #[test]
    fn space_starts_timer() {
        let mut app = test_app();
        assert_eq!(app.timer.state, TimerState::Idle);
        app.handle_key(KeyCode::Char(' '));
        assert_eq!(app.timer.state, TimerState::Running);
    }

    #[test]
    fn space_pauses_running_timer() {
        let mut app = test_app();
        app.handle_key(KeyCode::Char(' ')); // start
        assert_eq!(app.timer.state, TimerState::Running);
        app.handle_key(KeyCode::Char(' ')); // pause
        assert_eq!(app.timer.state, TimerState::Paused);
    }

    #[test]
    fn r_resets_timer() {
        let mut app = test_app();
        app.handle_key(KeyCode::Char(' ')); // start
        app.handle_key(KeyCode::Char('r')); // reset
        assert_eq!(app.timer.state, TimerState::Idle);
    }

    #[test]
    fn s_advances_phase() {
        let mut app = test_app();
        assert_eq!(app.timer.phase, Phase::Work);
        app.handle_key(KeyCode::Char('s'));
        assert_eq!(app.timer.phase, Phase::ShortBreak);
    }

    #[test]
    fn unknown_key_does_nothing() {
        let mut app = test_app();
        app.handle_key(KeyCode::Char('x'));
        assert!(!app.should_quit);
        assert_eq!(app.timer.state, TimerState::Idle);
    }

    #[test]
    fn notify_enabled_respects_config() {
        let app_no_notify = App::new(Config {
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            sessions_before_long_break: 4,
            no_notify: true,
        });
        assert!(!app_no_notify.notify_enabled);

        let app_notify = App::new(Config {
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            sessions_before_long_break: 4,
            no_notify: false,
        });
        assert!(app_notify.notify_enabled);
    }
}
