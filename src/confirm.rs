//! Confirm — Interactive yes/no confirmation prompts

use std::io::{self, Write};

/// Confirmation style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmStyle {
    /// Yes/No with brackets [Y/n]
    Default,
    /// True/False with brackets [y/N]
    Boolean,
    /// Simple ✓/✗
    Symbol,
}

/// Ask a yes/no confirmation
pub fn confirm(message: &str, style: ConfirmStyle) -> bool {
    let prompt = match style {
        ConfirmStyle::Default => "[Y/n]",
        ConfirmStyle::Boolean => "[y/N]",
        ConfirmStyle::Symbol => "✓/✗",
    };

    let _green = "\x1b[32m";
    let white = "\x1b[37m";
    let reset = "\x1b[0m";

    loop {
        print!("{} {} {}{}: ", message, white, prompt, reset);
        let _ = io::stdout().flush();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim().to_lowercase();
                match style {
                    ConfirmStyle::Default => {
                        if trimmed.is_empty() || trimmed == "y" || trimmed == "yes" {
                            return true;
                        } else if trimmed == "n" || trimmed == "no" {
                            return false;
                        }
                    }
                    ConfirmStyle::Boolean => {
                        if trimmed.is_empty() || trimmed == "n" || trimmed == "no" {
                            return false;
                        } else if trimmed == "y" || trimmed == "yes" {
                            return true;
                        }
                    }
                    ConfirmStyle::Symbol => {
                        if trimmed.is_empty()
                            || trimmed == "y"
                            || trimmed == "yes"
                            || trimmed == "✓"
                        {
                            return true;
                        } else if trimmed == "n" || trimmed == "no" || trimmed == "✗" {
                            return false;
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }
}

/// Ask a yes confirmation (default yes, lowercase n means no)
pub fn confirm_yes(message: &str) -> bool {
    confirm(message, ConfirmStyle::Default)
}

/// Ask a no confirmation (default no, lowercase y means yes)
pub fn confirm_no(message: &str) -> bool {
    confirm(message, ConfirmStyle::Boolean)
}
