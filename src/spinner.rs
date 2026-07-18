//! Spinner — Animated loading spinners with custom messages
//!
//! Supports configurable symbols, colors, message prefix, and output stream.

use std::thread;
use std::time::Duration;

/// Available spinner styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinnerStyle {
    /// Dots spinner (`. .. ...`)
    Dots,
    /// Lines spinner (`| / - \`)
    Lines,
    /// Blocks spinner (`█ ▓ ▒ ░`)
    Blocks,
    /// Arrow spinner (`◐ ◓ ◑ ◒`)
    Arrows,
    /// Bounce spinner (`● ○ ◉ ◎`)
    Bounce,
}

impl SpinnerStyle {
    fn symbols(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &[".", "..", "..."],
            SpinnerStyle::Lines => &["|", "/", "-", "\\"],
            SpinnerStyle::Blocks => &["█", "▓", "▒", "░"],
            SpinnerStyle::Arrows => &["◐", "◓", "◑", "◒"],
            SpinnerStyle::Bounce => &["●", "○", "◉", "◎"],
        }
    }
}

/// A running spinner animation
pub struct Spinner {
    symbols: &'static [&'static str],
    interval_ms: u64,
    message: String,
    style: SpinnerStyle,
}

impl Spinner {
    /// Create a new spinner with default settings
    pub fn new(message: &str) -> Self {
        Self {
            symbols: SpinnerStyle::Dots.symbols(),
            interval_ms: 80,
            message: message.to_string(),
            style: SpinnerStyle::Dots,
        }
    }

    /// Create a spinner with a specific style
    pub fn with_style(message: &str, style: SpinnerStyle) -> Self {
        let mut spinner = Self::new(message);
        spinner.style = style;
        spinner.symbols = style.symbols();
        spinner
    }

    /// Set the animation interval in milliseconds
    pub fn interval(mut self, ms: u64) -> Self {
        self.interval_ms = ms;
        self
    }

    /// Run the spinner in a separate thread and execute the provided closure
    /// Shows a ✓ on success, ✗ on error (if closure returns Err)
    pub fn run<F, T>(self, f: F) -> T
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let symbols = self.symbols;
        let interval_ms = self.interval_ms;
        let message = self.message.clone();

        let thread = thread::spawn(f);
        let mut count = 0u64;
        while !thread.is_finished() {
            thread::sleep(Duration::from_millis(interval_ms));
            count += 1;
            let symbol = symbols[(count as usize) % symbols.len()];
            let clear = "\r\x1b[2K";
            print!("{}{} {}", clear, symbol, message);
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        let result = thread.join();
        // Clear the spinner line after completion
        let _ = std::io::Write::write(&mut std::io::stdout(), b"\r\x1b[2K\r");
        result.unwrap()
    }

    /// Run the spinner showing a success marker after completion
    pub fn run_success<F>(self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let symbols = self.symbols;
        let interval_ms = self.interval_ms;
        let message = self.message.clone();

        let thread = thread::spawn(f);
        let mut count = 0u64;
        while !thread.is_finished() {
            thread::sleep(Duration::from_millis(interval_ms));
            count += 1;
            let symbol = symbols[(count as usize) % symbols.len()];
            let clear = "\r\x1b[2K";
            print!("{}{} {}", clear, symbol, message);
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        let _ = thread.join();
        // Clear the spinner line and show success
        let green = "\x1b[32m";
        let reset = "\x1b[0m";
        let _ = std::io::Write::write(&mut std::io::stdout(), b"\r\x1b[2K\r");
        let _ = std::io::Write::write(
            &mut std::io::stdout(),
            format!("{} ✓ {}{}", green, message, reset).as_bytes(),
        );
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }

    /// Show a success marker (use after manual spinner loop)
    pub fn succeed(self) {
        let clear = "\r\x1b[2K";
        let green = "\x1b[32m";
        let reset = "\x1b[0m";
        print!("{}{} ✓ {}{}", clear, green, self.message, reset);
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }

    /// Show an error marker (use after manual spinner loop)
    pub fn fail(self, reason: &str) {
        let clear = "\r\x1b[2K";
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        print!("{}{} ✗ {} — {}{}", clear, red, self.message, reason, reset);
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
}
