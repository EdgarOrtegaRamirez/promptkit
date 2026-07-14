# PromptKit

**Rich interactive CLI toolkit** — spinners, progress bars, confirmations, menus, checkboxes, tables, and more.

PromptKit is a collection of building blocks for creating beautiful, interactive command-line applications in Rust. It provides a simple, composable API for common CLI patterns.

## Features

- **Spinners** — Animated loading indicators with 5 styles (dots, lines, blocks, arrows, bounce), configurable speed, and success/failure markers
- **Progress Bars** — Real-time progress tracking with Unicode block rendering, percentage display, and custom prefixes/suffixes
- **Confirmations** — Yes/no prompts with multiple styles (default, boolean, symbol) and input validation
- **Menus** — Numbered selection menus with keyboard input and labeled options
- **Checkboxes** — Multi-select checkbox lists with keyboard navigation
- **Tables** — Formatted tabular data with left/right/center alignment and auto-width calculation
- **Text Prompts** — Input prompts with placeholders, validation functions, and default values
- **Output Utilities** — Color-coded success/error/warning/info messages, key-value pairs, JSON formatting, and more

## Installation

```bash
cargo install promptkit
```

Or add it as a dependency:

```toml
[dependencies]
promptkit = "1.0"
```

## Quick Start

### Spinner

```rust
use promptkit::spinner::{Spinner, SpinnerStyle};

// Simple spinner with success marker
Spinner::new("Loading data...")
    .run_success(|| {
        // Your async work here
        std::thread::sleep(std::time::Duration::from_secs(2));
    });
```

### Progress Bar

```rust
use promptkit::progress::ProgressBar;

let pb = ProgressBar::new(100, "Processing files");
for i in 0..100 {
    // Do work
    pb.inc_by(1);
}
pb.finish();
```

### Confirmation

```rust
use promptkit::confirm::confirm_yes;

if confirm_yes("Delete all files?") {
    println!("Deleting...");
} else {
    println!("Cancelled.");
}
```

### Menu

```rust
use promptkit::menu::{menu, MenuItem};

let items = vec![
    MenuItem::new("Build", "build"),
    MenuItem::new("Test", "test"),
    MenuItem::new("Deploy", "deploy"),
];

let selected = menu(&items, "Select action");
```

### Table

```rust
use promptkit::table::print_table;

let headers = vec!["Name", "Status", "Age"];
let rows = vec![
    vec!["Alice".to_string(), "Active".to_string(), "30".to_string()],
    vec!["Bob".to_string(), "Inactive".to_string(), "25".to_string()],
    vec!["Charlie".to_string(), "Active".to_string(), "35".to_string()],
];

print_table(&headers, rows);
```

### Text Prompt

```rust
use promptkit::prompt::Prompt;

let name = Prompt::new("Enter your name")
    .placeholder("John Doe")
    .validate(|s| {
        if s.is_empty() {
            Some("Name cannot be empty".to_string())
        } else {
            None
        }
    })
    .ask();
```

## CLI Usage

```bash
# Spinner animation
promptkit spinner --message "Working..." --style lines --interval 100

# Progress bar
promptkit progress --total 50 --message "Downloading"

# Confirmation
promptkit confirm --message "Continue?" --style default

# Menu
promptkit menu --title "Select" --items "Build:build" --items "Test:test" --items "Deploy:deploy"

# Table
promptkit table --headers "Name,Status,Age" --rows "Alice,Active,30" --rows "Bob,Inactive,25"

# Prompt
promptkit prompt --message "Your name" --placeholder "John"

# Output utilities
promptkit output success "Done!"
promptkit output error "Failed!"
promptkit output warning "Caution!"
promptkit output kv --key "Name" --value "PromptKit"
promptkit output heading "My Section"
```

## Architecture

```
promptkit/
├── src/
│   ├── lib.rs          # Public API re-exports
│   ├── spinner.rs      # Animated loading spinners
│   ├── progress.rs     # Progress bars
│   ├── confirm.rs      # Yes/no confirmations
│   ├── menu.rs         # Selection menus
│   ├── checkbox.rs     # Multi-select checkboxes
│   ├── table.rs        # Formatted tables
│   ├── prompt.rs       # Text input prompts
│   ├── theme.rs        # Color theming
│   ├── output.rs       # Formatted output utilities
│   └── main.rs         # CLI binary
├── Cargo.toml
├── LICENSE
└── README.md
```

## Design Decisions

1. **ANSI colors over libraries** — Uses raw ANSI escape codes for zero external dependencies on terminal output
2. **Blocking I/O** — All prompts use blocking stdin/stdout for simplicity and reliability
3. **Composable primitives** — Each component is independent and can be used standalone
4. **No TUI dependency** — Lightweight, no ratatui/crossterm required for basic usage (they're optional deps)

## License

MIT — see [LICENSE](LICENSE) for details.
