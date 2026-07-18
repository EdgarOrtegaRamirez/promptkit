//! PromptKit — Rich interactive CLI toolkit for Rust
//!
//! A collection of building blocks for creating beautiful, interactive command-line
//! applications with spinners, progress bars, confirmations, menus, checkboxes,
//! tables, and more.

pub mod checkbox;
pub mod confirm;
pub mod menu;
pub mod output;
pub mod progress;
pub mod prompt;
pub mod spinner;
pub mod table;
pub mod theme;

pub use checkbox::*;
pub use confirm::*;
pub use menu::*;
pub use output::*;
pub use progress::*;
pub use prompt::*;
pub use spinner::*;
pub use table::*;
pub use theme::*;
