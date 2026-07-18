//! Progress — Progress bar with configurable format and style

#[allow(unused_imports)]
use crate::theme::Theme;
use std::io::{self, Write};

/// A progress bar
pub struct ProgressBar {
    current: u64,
    total: u64,
    message: String,
    bar_width: usize,
    prefix: String,
    suffix: String,
}

impl ProgressBar {
    /// Create a new progress bar
    pub fn new(total: u64, message: &str) -> Self {
        Self {
            current: 0,
            total,
            message: message.to_string(),
            bar_width: 30,
            prefix: String::new(),
            suffix: String::new(),
        }
    }

    /// Set a custom bar width
    pub fn bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    /// Set a prefix for the progress message
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    /// Set a suffix for the progress message
    pub fn suffix(mut self, suffix: &str) -> Self {
        self.suffix = suffix.to_string();
        self
    }

    fn render(&mut self) {
        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        let filled = (self.bar_width as f64 * (percentage / 100.0)) as usize;
        let empty = self.bar_width.saturating_sub(filled);

        let mut bar = String::new();
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in 0..empty {
            bar.push('░');
        }

        let green = "\x1b[32m";
        let dark_green = "\x1b[38;5;22m";
        let light_gray = "\x1b[90m";
        let reset = "\x1b[0m";

        let filled_color = if filled >= self.bar_width {
            green
        } else {
            dark_green
        };

        let mut line = String::new();
        line.push_str("\r\x1b[2K ");
        line.push_str(filled_color);
        line.push_str(&bar);
        line.push_str(reset);
        line.push(' ');
        line.push_str(&self.prefix);
        line.push_str(&self.message);
        line.push(' ');
        line.push_str(light_gray);
        line.push_str(&format!(
            "{}/{} {:.1}%",
            self.current, self.total, percentage
        ));
        line.push_str(reset);
        line.push('\n');

        let _ = io::Write::write_all(&mut io::stdout(), line.as_bytes());
        let _ = io::stdout().flush();
    }

    /// Advance the progress bar
    pub fn inc(&mut self) {
        if self.current < self.total {
            self.current += 1;
            self.render();
        }
    }

    /// Advance by a specific amount
    pub fn inc_by(&mut self, n: u64) {
        self.current = std::cmp::min(self.current + n, self.total);
        self.render();
    }

    /// Set the current position
    pub fn set(&mut self, pos: u64) {
        self.current = std::cmp::min(pos, self.total);
        self.render();
    }

    /// Finish and show a complete bar
    pub fn finish(mut self) {
        self.current = self.total;
        self.render();
        let mut line = String::new();
        line.push_str("\n\x1b[32m ✓ ");
        line.push_str(&self.message);
        line.push_str(&format!(" {}/{}\x1b[0m", self.current, self.total));
        let _ = io::Write::write_all(&mut io::stdout(), line.as_bytes());
        let _ = io::stdout().flush();
    }
}
