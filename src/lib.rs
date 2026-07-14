//! PromptKit — Rich interactive CLI toolkit for Rust
//!
//! A collection of building blocks for creating beautiful, interactive command-line
//! applications with spinners, progress bars, confirmations, menus, checkboxes,
//! tables, and more.

pub mod spinner;
pub mod progress;
pub mod confirm;
pub mod menu;
pub mod checkbox;
pub mod table;
pub mod prompt;
pub mod theme;
pub mod output;

pub use spinner::*;
pub use progress::*;
pub use confirm::*;
pub use menu::*;
pub use checkbox::*;
pub use table::*;
pub use prompt::*;
pub use theme::*;
pub use output::*;
