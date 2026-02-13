use std::{io::stdout, panic};

use color_eyre::Result;
use ratatui::{
    Terminal,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};

/// Initializes the terminal for `ratatui`.
///
/// # Errors
/// Can error if stdout is misbehaving.
pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

/// Restores the terminal from alternate mode.
///
/// # Errors
/// Can error if stdout is misbehaving.
pub fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Makes sure to leave alternate mode after panicking.
///
/// # Panics
/// will panic if we can't install panic hooks.
pub fn install_panic_hook() {
    color_eyre::install().unwrap();
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal().unwrap();
        original_hook(panic_info);
    }));
}
