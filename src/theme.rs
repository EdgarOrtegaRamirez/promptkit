//! Theme — Color theming for all output components

use std::collections::HashMap;

/// ANSI color codes for theming
#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub colors: HashMap<String, String>,
}

impl Theme {
    /// Default ANSI theme
    pub fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert("success".to_string(), "\x1b[32m".to_string());    // Green
        colors.insert("error".to_string(), "\x1b[31m".to_string());     // Red
        colors.insert("warning".to_string(), "\x1b[33m".to_string());   // Yellow
        colors.insert("info".to_string(), "\x1b[34m".to_string());      // Blue
        colors.insert("label".to_string(), "\x1b[36m".to_string());     // Cyan
        colors.insert("dim".to_string(), "\x1b[2m".to_string());        // Dim
        colors.insert("bold".to_string(), "\x1b[1m".to_string());       // Bold
        colors.insert("reset".to_string(), "\x1b[0m".to_string());      // Reset
        Self { colors }
    }

    /// Apply a color to text
    pub fn apply(&self, text: &str, color: &str) -> String {
        if let Some(code) = self.colors.get(color) {
            format!("{}{}{}", code, text, self.colors.get("reset").unwrap_or(&String::new()))
        } else {
            text.to_string()
        }
    }

    /// Apply success color
    pub fn success(&self, text: &str) -> String {
        self.apply(text, "success")
    }

    /// Apply error color
    pub fn error(&self, text: &str) -> String {
        self.apply(text, "error")
    }

    /// Apply warning color
    pub fn warning(&self, text: &str) -> String {
        self.apply(text, "warning")
    }

    /// Apply info color
    pub fn info(&self, text: &str) -> String {
        self.apply(text, "info")
    }

    /// Print a success line
    pub fn print_success(&self, text: &str) {
        println!("{}", self.success(text));
    }

    /// Print an error line
    pub fn print_error(&self, text: &str) {
        eprintln!("{}", self.error(text));
    }

    /// Print a warning line
    pub fn print_warning(&self, text: &str) {
        println!("{}", self.warning(text));
    }

    /// Print an info line
    pub fn print_info(&self, text: &str) {
        println!("{}", self.info(text));
    }
}
