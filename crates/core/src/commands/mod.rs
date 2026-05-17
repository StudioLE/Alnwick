//! Command implementations for the CLI and web interfaces.

mod add;
#[cfg(feature = "server")]
mod cli_runner;
mod cover;
mod define;
mod download;
mod emulate;
mod fetch;
mod podcast_options;
#[cfg(feature = "server")]
mod podcast_selector;

pub use add::*;
#[cfg(feature = "server")]
pub use cli_runner::*;
pub use cover::*;
pub use define::*;
pub use download::*;
pub use emulate::*;
pub use fetch::*;
pub use podcast_options::*;
#[cfg(feature = "server")]
pub use podcast_selector::*;
