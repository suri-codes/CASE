pub mod core;

mod http;
mod sse;

mod helpers;
pub use helpers::*;

mod widgets;
use serde::{Deserialize, Serialize};
pub use widgets::*;

mod tui;
pub use tui::{Event as TuiEvent, Tui};

mod config;
pub use config::*;

pub use color_eyre::{Result, eyre::eyre};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
    Explorer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Quit,
    Penis,
}
