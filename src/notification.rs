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

#[cfg(test)]
mod tests {
    use super::*;

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
