//! Menu — Interactive text-based selection menu

use std::io::{self, Write};

/// A selectable menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub value: String,
}

impl MenuItem {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }
}

/// Display a numbered selection menu and return the selected value
pub fn menu(items: &[MenuItem], title: &str) -> Option<String> {
    let blue = "\x1b[34m";
    let cyan = "\x1b[36m";
    let white = "\x1b[37m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!();
    if !title.is_empty() {
        println!("{}{}{}", bold, title, reset);
        println!("{}", "─".repeat(title.len()));
    }
    println!();

    for (i, item) in items.iter().enumerate() {
        let num = format!("  [{}] ", i + 1);
        println!("{}{} {}{}", cyan, num, white, item.label,);
    }

    println!();
    print!("{}Select > {}{}{} ", blue, reset, bold, reset);
    let _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim();
            if let Ok(n) = trimmed.parse::<usize>() {
                if n >= 1 && n <= items.len() {
                    return Some(items[n - 1].value.clone());
                }
            }
            None
        }
        Err(_) => None,
    }
}

/// Display a yes/no menu and return the choice
pub fn yes_no_menu(message: &str) -> Option<bool> {
    let items = vec![MenuItem::new("Yes", "yes"), MenuItem::new("No", "no")];
    match menu(&items, message) {
        Some(v) if v == "yes" => Some(true),
        Some(v) if v == "no" => Some(false),
        _ => None,
    }
}
