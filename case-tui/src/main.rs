use case::Tui;
use case::TuiViewModel;
use case::core;
use case::core::update;
use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use crossbeam_channel::unbounded;

use crossterm::event::KeyEvent;
use shared::{Effect, Event};
use tracing::error;

#[derive(Parser, Clone)]
enum Command {
    Get,
    Inc,
    Dec,
    Watch,
}

impl From<Command> for Event {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Get => Self::Get,
            Command::Inc => Self::Increment,
            Command::Dec => Self::Decrement,
            Command::Watch => Self::StartWatch,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    case::install_panic_hook();
    case::init_logging()?;

    let core = core::new();
    let (tx, rx) = unbounded::<Effect>();

    let mut tui = Tui::new()?;

    tui.start();
    tui.enter()?;

    // This is the TUI event handler.
    tokio::spawn({
        let core = core.clone();
        let mut tui_event_rx = tui.take_event_rx().expect("event rx should exist by now");

        async move {
            while let Some(event) = tui_event_rx.recv().await {
                use crossterm::event::KeyCode;
                let event = match event {
                    case::TuiEvent::Key(key_event) => match key_event.code {
                        KeyCode::Char('j') => Some(Event::Increment),
                        KeyCode::Char('k') => Some(Event::Decrement),
                        KeyCode::Char('g') => Some(Event::Get),

                        _ => None,
                    },
                    _ => continue,
                };

                let Some(event) = event else { continue };

                update(&core, event, &tx)
                    .map_err(|e| error!("{}", e.to_string()))
                    .unwrap();
            }
        }
    });

    // This is the effect handler for the core.
    while let Ok(effect) = rx.recv() {
        if let Effect::Render(_) = effect {
            let view = core.view();

            tui.draw(|f| f.render_widget(TuiViewModel::from(view), f.area()))
                .map_err(|e| eyre!(e.to_string()))?;
        }
    }
    tui.exit()?;
    Ok(())
}
