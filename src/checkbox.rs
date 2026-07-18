//! Checkbox — Multi-select checkbox list

use std::io::{self, Write};

/// A checkbox option
#[derive(Debug, Clone)]
pub struct CheckboxItem {
    pub label: String,
    pub value: String,
    pub checked: bool,
}

impl CheckboxItem {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
            checked: false,
        }
    }
}

/// Display checkboxes and return checked values
pub fn checkbox(items: &mut [CheckboxItem], title: &str) -> Vec<String> {
    let green = "\x1b[32m";
    let white = "\x1b[37m";
    let dim = "\x1b[2m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!();
    if !title.is_empty() {
        println!("{}{}{}", bold, title, reset);
    }
    println!();

    // Display the checkboxes
    for item in items.iter() {
        let checked_str = if item.checked { green } else { dim };
        let check = if item.checked { "✓" } else { " " };
        println!(
            "  {}{}{} {}{}",
            checked_str, check, reset, white, item.label
        );
    }

    println!();
    println!("  {dim}(space=toggle, enter=confirm){reset}");
    print!("  Selection > ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            if let Ok(n) = trimmed.parse::<usize>() {
                if n < items.len() {
                    items[n].checked = !items[n].checked;
                }
            }
            // Return all checked items
            items
                .iter()
                .filter(|i| i.checked)
                .map(|i| i.value.clone())
                .collect()
        }
        Err(_) => Vec::new(),
    }
}

/// Simple multi-select that allows choosing multiple items by number
pub fn multi_select(items: &[String], title: &str) -> Vec<usize> {
    let _blue = "\x1b[34m";
    let cyan = "\x1b[36m";
    let white = "\x1b[37m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!();
    if !title.is_empty() {
        println!("{}{}{}", bold, title, reset);
    }
    println!();

    for (i, item) in items.iter().enumerate() {
        println!("  {}[{}] {} {}{}", cyan, i + 1, reset, white, item);
    }

    println!();
    println!("  {bold}Enter comma-separated numbers (e.g., 1,3,5) or 'all' to select all{reset}");
    print!("  Selection > ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim().to_lowercase();
            if trimmed == "all" || trimmed == "a" {
                return (0..items.len()).collect();
            }
            trimmed
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .filter(|&n| n >= 1 && n <= items.len())
                .map(|n| n - 1)
                .collect()
        }
        Err(_) => Vec::new(),
    }
}
