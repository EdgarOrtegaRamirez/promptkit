//! PromptKit CLI — Rich interactive CLI toolkit
//!
//! A collection of building blocks for creating beautiful, interactive
//! command-line applications.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "promptkit", version, about = "Rich interactive CLI toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a spinner animation
    Spinner {
        /// Message to display
        #[arg(short, long, default_value = "Loading...")]
        message: String,

        /// Spinner style (dots, lines, blocks, arrows, bounce)
        #[arg(short, long, default_value = "dots")]
        style: String,

        /// Animation interval in milliseconds
        #[arg(short, long, default_value_t = 80)]
        interval: u64,
    },

    /// Show a progress bar
    Progress {
        /// Total count
        #[arg(short, long, default_value_t = 100)]
        total: u64,

        /// Message to display
        #[arg(short, long, default_value = "Processing")]
        message: String,
    },

    /// Ask a yes/no confirmation
    Confirm {
        /// Message to display
        #[arg(short, long, default_value = "Continue?")]
        message: String,

        /// Confirmation style (default, boolean, symbol)
        #[arg(short, long, default_value = "default")]
        style: String,
    },

    /// Display a selection menu
    Menu {
        /// Menu title
        #[arg(short, long)]
        title: Option<String>,

        /// Menu items as label:value pairs
        #[arg(short, long)]
        items: Vec<String>,
    },

    /// Display a checkbox selection
    Checkbox {
        /// Title
        #[arg(short, long)]
        title: Option<String>,

        /// Checkbox items as label:value pairs
        #[arg(short, long)]
        items: Vec<String>,
    },

    /// Display a formatted table
    Table {
        /// Column headers as comma-separated values
        #[arg(short, long)]
        headers: String,

        /// Table rows as pipe-separated values (one per line)
        #[arg(short, long)]
        rows: Vec<String>,
    },

    /// Ask a text input prompt
    Prompt {
        /// Prompt message
        #[arg(short, long)]
        message: String,

        /// Placeholder text
        #[arg(short, long)]
        placeholder: Option<String>,
    },

    /// Output utilities
    Output {
        #[command(subcommand)]
        command: OutputCommands,
    },
}

#[derive(Subcommand)]
enum OutputCommands {
    /// Print a success message
    Success {
        /// Message to print
        message: String,
    },
    /// Print an error message
    Error {
        /// Message to print
        message: String,
    },
    /// Print a warning message
    Warning {
        /// Message to print
        message: String,
    },
    /// Print info message
    Info {
        /// Message to print
        message: String,
    },
    /// Print a key-value pair
    Kv {
        /// Key
        #[arg(short, long)]
        key: String,
        /// Value
        #[arg(short, long)]
        value: String,
    },
    /// Print a separator
    Sep {},
    /// Print a heading
    Heading {
        /// Heading text
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Spinner {
            message,
            style,
            interval,
        } => {
            use promptkit::spinner::{Spinner, SpinnerStyle};
            let spinner_style = match style.to_lowercase().as_str() {
                "lines" => SpinnerStyle::Lines,
                "blocks" => SpinnerStyle::Blocks,
                "arrows" => SpinnerStyle::Arrows,
                "bounce" => SpinnerStyle::Bounce,
                _ => SpinnerStyle::Dots,
            };
            Spinner::with_style(&message, spinner_style)
                .interval(interval)
                .run_success(|| {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                });
        }
        Commands::Progress { total, message } => {
            use promptkit::progress::ProgressBar;
            let mut pb = ProgressBar::new(total, &message);
            for i in 0..total {
                std::thread::sleep(std::time::Duration::from_millis(50));
                pb.inc_by(i + 1);
            }
            pb.finish();
        }
        Commands::Confirm { message, style } => {
            use promptkit::confirm::{confirm, ConfirmStyle};
            let style = match style.to_lowercase().as_str() {
                "boolean" => ConfirmStyle::Boolean,
                "symbol" => ConfirmStyle::Symbol,
                _ => ConfirmStyle::Default,
            };
            let result = confirm(&message, style);
            println!("Answer: {}", if result { "Yes" } else { "No" });
        }
        Commands::Menu { title, items } => {
            use promptkit::menu::{menu, MenuItem};
            let menu_items: Vec<MenuItem> = items
                .iter()
                .map(|item| {
                    let parts: Vec<&str> = item.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        MenuItem::new(parts[0].trim(), parts[1].trim())
                    } else {
                        MenuItem::new(item, item)
                    }
                })
                .collect();
            let selected = menu(&menu_items, title.as_deref().unwrap_or("Select an option"));
            if let Some(val) = selected {
                println!("Selected: {}", val);
            } else {
                println!("No selection made.");
            }
        }
        Commands::Checkbox { title, items } => {
            use promptkit::checkbox::CheckboxItem;
            let mut checkbox_items: Vec<CheckboxItem> = items
                .iter()
                .map(|item| {
                    let parts: Vec<&str> = item.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        CheckboxItem::new(parts[0].trim(), parts[1].trim())
                    } else {
                        CheckboxItem::new(item, item)
                    }
                })
                .collect();
            let selected = promptkit::checkbox::checkbox(
                &mut checkbox_items,
                title.as_deref().unwrap_or("Select options"),
            );
            println!("Selected: {:?}", selected);
        }
        Commands::Table { headers, rows } => {
            let header_vec: Vec<&str> = headers.split(',').map(|s| s.trim()).collect();
            let table_rows: Vec<Vec<String>> = rows
                .iter()
                .map(|row| row.split('|').map(|s| s.trim().to_string()).collect())
                .collect();
            promptkit::table::print_table(&header_vec, table_rows);
        }
        Commands::Prompt {
            message,
            placeholder,
        } => {
            let mut p = promptkit::prompt::Prompt::new(&message);
            if let Some(ph) = placeholder {
                p = p.placeholder(&ph);
            }
            let answer = p.ask();
            println!("You entered: {}", answer);
        }
        Commands::Output { command } => match command {
            OutputCommands::Success { message } => promptkit::output::success(&message),
            OutputCommands::Error { message } => promptkit::output::error(&message),
            OutputCommands::Warning { message } => promptkit::output::warning(&message),
            OutputCommands::Info { message } => promptkit::output::print_info(&message),
            OutputCommands::Kv { key, value } => promptkit::output::print_kv(&key, &value),
            OutputCommands::Sep {} => promptkit::output::separator(),
            OutputCommands::Heading { text } => promptkit::output::heading(&text),
        },
    }
}
