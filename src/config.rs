use clap::Parser;

/// A terminal-based Pomodoro timer built with Ratatui.
#[derive(Debug, Clone, Parser)]
#[command(name = "tumodori", version, about)]
pub struct Config {
    /// Work session duration in minutes (1-1440).
    #[arg(short = 'w', long = "work", default_value_t = 25, value_parser = clap::value_parser!(u64).range(1..=1440))]
    pub work_minutes: u64,

    /// Short break duration in minutes (1-1440).
    #[arg(short = 's', long = "short-break", default_value_t = 5, value_parser = clap::value_parser!(u64).range(1..=1440))]
    pub short_break_minutes: u64,

    /// Long break duration in minutes (1-1440).
    #[arg(short = 'l', long = "long-break", default_value_t = 15, value_parser = clap::value_parser!(u64).range(1..=1440))]
    pub long_break_minutes: u64,

    /// Number of work sessions before a long break (1-100).
    #[arg(short = 'n', long = "sessions", default_value_t = 4, value_parser = clap::value_parser!(u32).range(1..=100))]
    pub sessions_before_long_break: u32,

    /// Disable desktop notifications.
    #[arg(long = "no-notify", default_value_t = false)]
    pub no_notify: bool,
}

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

    #[test]
    fn rejects_zero_work_minutes() {
        let result = Config::try_parse_from(["tumodori", "--work", "0"]);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_sessions() {
        let result = Config::try_parse_from(["tumodori", "--sessions", "0"]);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_excessive_work_minutes() {
        let result = Config::try_parse_from(["tumodori", "--work", "1441"]);
        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_range() {
        let config = Config::parse_from(["tumodori", "-w", "50", "-s", "10", "-l", "20", "-n", "6"]);
        assert_eq!(config.work_minutes, 50);
        assert_eq!(config.short_break_minutes, 10);
        assert_eq!(config.long_break_minutes, 20);
        assert_eq!(config.sessions_before_long_break, 6);
    }
}
