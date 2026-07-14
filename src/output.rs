//! Output — Formatted output utilities (JSON, key-value, etc.)

use serde::Serialize;

/// Format a value as pretty JSON
pub fn print_json(data: &impl Serialize) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

/// Format a value as compact JSON
pub fn json(data: &impl Serialize) -> Result<String, Box<dyn std::error::Error>> {
    serde_json::to_string(data).map_err(Into::into)
}

/// Format a value as compact JSON (pretty)
pub fn json_pretty(data: &impl Serialize) -> Result<String, Box<dyn std::error::Error>> {
    serde_json::to_string_pretty(data).map_err(Into::into)
}

/// Print a key-value pair
pub fn print_kv(key: &str, value: &str) {
    println!("  {}: {}", key, value);
}

/// Print a separator line
pub fn separator() {
    println!("  {}", "─".repeat(50));
}

/// Print an info line
pub fn print_info(text: &str) {
    println!("\x1b[34m ℹ {}\x1b[0m", text);
}
/// Print a heading
pub fn heading(text: &str) {
    println!();
    println!("  {}", text);
    println!("  {}", "─".repeat(text.len()));
}

/// Print a success marker with text
pub fn success(msg: &str) {
    println!("\x1b[32m ✓ {}\x1b[0m", msg);
}

/// Print an error marker with text
pub fn error(msg: &str) {
    eprintln!("\x1b[31m ✗ {}\x1b[0m", msg);
}

/// Print a warning marker with text
pub fn warning(msg: &str) {
    println!("\x1b[33m ⚠ {}\x1b[0m", msg);
}

/// Write JSON to a file
pub fn write_json_file(path: &str, data: &impl Serialize) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(path, json)?;
    Ok(())
}
