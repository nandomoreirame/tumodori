use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::config::Config;
use crate::notification;
use crate::timer::Timer;
use crate::ui;

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
        let tick_rate = Duration::from_millis(100);

        loop {
            terminal.draw(|frame| ui::draw(frame, &self.timer))?;

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
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
