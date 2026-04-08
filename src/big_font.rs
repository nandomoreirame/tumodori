//! Big digit font using block characters.
//! Each digit is 11 lines tall and 10 characters wide.

const DIGIT_HEIGHT: usize = 11;

const DIGITS: [[&str; 11]; 10] = [
    // 0
    [
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "████  ████",
        "████  ████",
        "████  ████",
        "████  ████",
        "████  ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
    ],
    // 1
    [
        "    ████  ",
        "   █████  ",
        "  ██████  ",
        " ███████  ",
        "    ████  ",
        "    ████  ",
        "    ████  ",
        "    ████  ",
        "    ████  ",
        " ████████ ",
        " ████████ ",
    ],
    // 2
    [
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "      ████",
        "     ████ ",
        "   ████   ",
        "  ████    ",
        " ████     ",
        "████  ████",
        "██████████",
        "██████████",
    ],
    // 3
    [
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "      ████",
        "      ████",
        "   ██████ ",
        "      ████",
        "      ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
    ],
    // 4
    [
        "     ████ ",
        "    █████ ",
        "   ██████ ",
        "  ███████ ",
        " ████████ ",
        "████ ████ ",
        "██████████",
        "██████████",
        "     ████ ",
        "     ████ ",
        "     ████ ",
    ],
    // 5
    [
        "██████████",
        "██████████",
        "████      ",
        "████      ",
        "████████  ",
        "█████████ ",
        "      ████",
        "      ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
    ],
    // 6
    [
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "████      ",
        "████████  ",
        "█████████ ",
        "████  ████",
        "████  ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
    ],
    // 7
    [
        "██████████",
        "██████████",
        "████  ████",
        "      ████",
        "     ████ ",
        "    ████  ",
        "   ████   ",
        "   ████   ",
        "   ████   ",
        "   ████   ",
        "   ████   ",
    ],
    // 8
    [
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
    ],
    // 9
    [
        "  ██████  ",
        " ████████ ",
        "████  ████",
        "████  ████",
        "████  ████",
        " █████████",
        "  ████████",
        "      ████",
        "████  ████",
        " ████████ ",
        "  ██████  ",
    ],
];

const COLON: [&str; 11] = [
    "    ",
    "    ",
    "████",
    "████",
    "    ",
    "    ",
    "    ",
    "████",
    "████",
    "    ",
    "    ",
];

/// Render minutes and seconds as big text lines.
/// Clamps minutes to 99 and seconds to 59 to prevent index out of bounds.
pub fn render_time(minutes: u64, seconds: u64) -> Vec<String> {
    let minutes = minutes.min(99);
    let seconds = seconds.min(59);
    let d1 = (minutes / 10) as usize;
    let d2 = (minutes % 10) as usize;
    let d3 = (seconds / 10) as usize;
    let d4 = (seconds % 10) as usize;

    let mut lines = Vec::with_capacity(DIGIT_HEIGHT);
    for row in 0..DIGIT_HEIGHT {
        let line = format!(
            "{}  {}  {}  {}  {}",
            DIGITS[d1][row],
            DIGITS[d2][row],
            COLON[row],
            DIGITS[d3][row],
            DIGITS[d4][row],
        );
        lines.push(line);
    }
    lines
}

/// Width of the rendered big time string.
pub fn rendered_width() -> u16 {
    // 4 digits (10 each) + 1 colon (4) + 8 spaces (4x2) = 52
    52
}

/// Height of the rendered big time string.
pub fn rendered_height() -> u16 {
    DIGIT_HEIGHT as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_time_returns_correct_height() {
        let lines = render_time(25, 0);
        assert_eq!(lines.len(), DIGIT_HEIGHT);
    }

    #[test]
    fn render_time_lines_have_consistent_width() {
        let lines = render_time(12, 34);
        let first_width = lines[0].chars().count();
        for line in &lines {
            assert_eq!(line.chars().count(), first_width);
        }
    }

    #[test]
    fn render_time_clamps_minutes_over_99() {
        let lines = render_time(100, 30);
        assert_eq!(lines.len(), DIGIT_HEIGHT);
    }

    #[test]
    fn render_time_clamps_seconds_over_59() {
        let lines = render_time(25, 99);
        assert_eq!(lines.len(), DIGIT_HEIGHT);
    }

    #[test]
    fn rendered_dimensions_match_output() {
        let lines = render_time(0, 0);
        assert_eq!(lines.len() as u16, rendered_height());
    }
}
