use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph},
};

use crate::big_font;
use crate::timer::{Phase, Timer, TimerState};

pub fn draw(frame: &mut Frame, timer: &Timer) {
    let area = frame.area();

    // Content block: phase label (1) + big timer (11) + spacing (1) + session (1) + progress (1) + spacing (1) + controls (1)
    let content_height: u16 = 17;
    let content_width = big_font::rendered_width() + 4; // padding

    let center = centered_rect(content_width, content_height, area);

    let chunks = Layout::vertical([
        Constraint::Length(1), // phase label
        Constraint::Length(1), // spacing
        Constraint::Length(big_font::rendered_height()), // big timer
        Constraint::Length(1), // spacing
        Constraint::Length(1), // session + progress inline
        Constraint::Length(1), // spacing
        Constraint::Length(1), // controls
    ])
    .split(center);

    // Phase label
    let phase_text = Paragraph::new(Line::from(vec![Span::styled(
        timer.phase.label().to_uppercase(),
        Style::default()
            .fg(phase_color(timer.phase))
            .add_modifier(Modifier::BOLD),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(phase_text, chunks[0]);

    // Big timer
    let remaining = timer.remaining();
    let mins = remaining.as_secs() / 60;
    let secs = remaining.as_secs() % 60;
    let time_lines = big_font::render_time(mins, secs);

    let color = timer_color(timer.state, timer.phase);
    let big_timer_lines: Vec<Line> = time_lines
        .iter()
        .map(|line| Line::from(Span::styled(line.as_str(), Style::default().fg(color))))
        .collect();

    let big_timer = Paragraph::new(big_timer_lines).alignment(Alignment::Center);
    frame.render_widget(big_timer, chunks[2]);

    // Session counter
    let session_text = Paragraph::new(Line::from(vec![Span::styled(
        format!("Session {}", timer.completed_sessions + 1),
        Style::default().fg(Color::DarkGray),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(session_text, chunks[4]);

    // Controls
    let controls = match timer.state {
        TimerState::Idle => "[Space] Start  [s] Skip  [q] Quit",
        TimerState::Running => "[Space] Pause  [s] Skip  [q] Quit",
        TimerState::Paused => "[Space] Resume  [r] Reset  [q] Quit",
        TimerState::Finished => "[Space] Next  [r] Reset  [q] Quit",
    };
    let controls_text = Paragraph::new(Line::from(vec![Span::styled(
        controls,
        Style::default().fg(Color::DarkGray),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(controls_text, chunks[6]);

    // Progress bar at the bottom
    let progress = timer.progress();
    let bottom_bar = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(phase_color(timer.phase))
                .bg(Color::DarkGray),
        )
        .ratio(progress)
        .label("");
    frame.render_widget(gauge, bottom_bar);
}

fn timer_color(state: TimerState, phase: Phase) -> Color {
    match state {
        TimerState::Idle => Color::DarkGray,
        TimerState::Running => phase_color(phase),
        TimerState::Paused => Color::Yellow,
        TimerState::Finished => Color::White,
    }
}

fn phase_color(phase: Phase) -> Color {
    match phase {
        Phase::Work => Color::Red,
        Phase::ShortBreak => Color::Green,
        Phase::LongBreak => Color::Blue,
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);

    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0])[0]
}
