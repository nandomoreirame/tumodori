use clap::Parser;

/// A terminal-based Pomodoro timer built with Ratatui.
#[derive(Debug, Clone, Parser)]
#[command(name = "tumodori", version, about)]
pub struct Config {
    /// Work session duration in minutes.
    #[arg(short = 'w', long = "work", default_value_t = 25)]
    pub work_minutes: u64,

    /// Short break duration in minutes.
    #[arg(short = 's', long = "short-break", default_value_t = 5)]
    pub short_break_minutes: u64,

    /// Long break duration in minutes.
    #[arg(short = 'l', long = "long-break", default_value_t = 15)]
    pub long_break_minutes: u64,

    /// Number of work sessions before a long break.
    #[arg(short = 'n', long = "sessions", default_value_t = 4)]
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
}
