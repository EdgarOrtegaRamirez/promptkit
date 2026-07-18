//! Prompt — Text input prompts with optional validation

use std::io::{self, Write};

/// A prompt with optional placeholder and validator
#[allow(clippy::type_complexity)]
pub struct Prompt {
    message: String,
    placeholder: Option<String>,
    validator: Option<Box<dyn Fn(&str) -> Option<String>>>,
}

impl Prompt {
    /// Create a new prompt
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            placeholder: None,
            validator: None,
        }
    }

    /// Add a placeholder hint
    pub fn placeholder(mut self, text: &str) -> Self {
        self.placeholder = Some(text.to_string());
        self
    }

    /// Add a validation function that returns an error message for invalid input
    pub fn validate<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) -> Option<String> + 'static,
    {
        self.validator = Some(Box::new(f));
        self
    }

    /// Display the prompt and return the user's input
    pub fn ask(self) -> String {
        let white = "\x1b[37m";
        let dim = "\x1b[2m";
        let reset = "\x1b[0m";

        let mut input = String::new();

        loop {
            if let Some(ref placeholder) = self.placeholder {
                print!(
                    "{}{}{}{} (default: {}){}: ",
                    white, self.message, reset, dim, placeholder, reset
                );
            } else {
                print!("{}{}{}: ", white, self.message, reset);
            }
            let _ = io::stdout().flush();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim().to_string();
                    let value = if trimmed.is_empty() {
                        self.placeholder.clone().unwrap_or_default()
                    } else {
                        trimmed
                    };

                    if let Some(ref validator) = self.validator {
                        if let Some(err) = validator(&value) {
                            println!("  \x1b[31mError: {} {}", reset, err);
                            input.clear();
                            continue;
                        }
                    }

                    return value;
                }
                Err(_) => {
                    input.clear();
                    continue;
                }
            }
        }
    }
}
