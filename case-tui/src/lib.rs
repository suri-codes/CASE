pub mod core;

mod http;
mod sse;

mod helpers;
pub use helpers::*;

mod widgets;
pub use widgets::*;

mod tui;
pub use tui::{Event as TuiEvent, Tui};

pub use color_eyre::{Result, eyre::eyre};
