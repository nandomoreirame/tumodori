use std::io::Write;
use std::process::Command;
use std::thread;

use notify_rust::Notification;
#[cfg(target_os = "linux")]
use notify_rust::Urgency;

use crate::timer::Phase;

#[cfg(target_os = "linux")]
const ALARM_SOUND: &str = "/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga";

pub fn notification_content(phase: Phase) -> (&'static str, &'static str) {
    let title = "tumodori";
    match phase {
        Phase::Work => (title, "Work session complete!"),
        Phase::ShortBreak => (title, "Short break is over!"),
        Phase::LongBreak => (title, "Long break is over!"),
    }
}

#[cfg(target_os = "linux")]
fn urgency_for(phase: Phase) -> Urgency {
    match phase {
        Phase::Work => Urgency::Critical,
        Phase::ShortBreak | Phase::LongBreak => Urgency::Normal,
    }
}

pub fn send(phase: Phase) {
    let (title, body) = notification_content(phase);

    let mut notification = Notification::new();
    notification.summary(title).body(body);

    #[cfg(target_os = "linux")]
    notification.urgency(urgency_for(phase));

    let _ = notification.show();

    // Terminal bell
    let _ = std::io::stdout().write_all(b"\x07");
    let _ = std::io::stdout().flush();

    // Play alarm sound in background (Linux only)
    #[cfg(target_os = "linux")]
    thread::spawn(|| {
        let _ = Command::new("pw-play")
            .arg(ALARM_SOUND)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .or_else(|_| {
                Command::new("paplay")
                    .arg(ALARM_SOUND)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_phase_message() {
        let (title, body) = notification_content(Phase::Work);
        assert_eq!(title, "tumodori");
        assert_eq!(body, "Work session complete!");
    }

    #[test]
    fn short_break_message() {
        let (title, body) = notification_content(Phase::ShortBreak);
        assert_eq!(title, "tumodori");
        assert_eq!(body, "Short break is over!");
    }

    #[test]
    fn long_break_message() {
        let (title, body) = notification_content(Phase::LongBreak);
        assert_eq!(title, "tumodori");
        assert_eq!(body, "Long break is over!");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn work_urgency_is_critical() {
        assert!(matches!(urgency_for(Phase::Work), Urgency::Critical));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn break_urgency_is_normal() {
        assert!(matches!(urgency_for(Phase::ShortBreak), Urgency::Normal));
        assert!(matches!(urgency_for(Phase::LongBreak), Urgency::Normal));
    }
}
