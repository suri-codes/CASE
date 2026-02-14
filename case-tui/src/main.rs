use std::sync::Arc;

use case::Tui;
use case::TuiEvent;
use case::TuiViewModel;
use case::core;
use case::core::Core;
use case::core::update;
use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;

use shared::{Effect, Event};
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedReceiver;

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
    color_eyre::install()?;
    case::init_logging()?;

    let core = core::new();
    let (tx, rx) = unbounded::<Effect>();

    let mut tui = Tui::new()?;

    tui.start();
    tui.enter()?;

    // How do we do ts.
    //
    // We need to be able to shoot render calls from two different areas.
    // 1. From the effect handler from the core
    // 2. From the event handler from the TUI.
    //
    // Do we just slap the TUI inside an Arc<Mutex<>>.

    let tui = Arc::new(Mutex::new(tui));

    // This is the TUI event handler.
    let event_handler = tokio::spawn({
        let core = core.clone();
        let tui = tui.clone();
        let tui_event_rx = tui
            .lock()
            .await
            .take_event_rx()
            .expect("The event_rx should not be taken yet.");

        let tx = tx.clone();

        event_handler(core, tui, tui_event_rx, tx)
    });

    let effect_handler = tokio::spawn({
        let core = core.clone();
        let tui = tui.clone();

        effect_handler(core, tui, rx)
    });

    let res = tokio::select! {
        result = event_handler => result.unwrap(),
        result = effect_handler => result.unwrap(),
    };

    tui.lock().await.exit()?;
    res // If res is Result<(), E>, this propagates the error
}

async fn event_handler(
    core: Core,
    tui: Arc<Mutex<Tui>>,
    mut tui_event_rx: UnboundedReceiver<TuiEvent>,
    effect_tx: Sender<Effect>,
) -> Result<()> {
    // What I'm seeing is that this might have to have the ability to fire off render events too?
    while let Some(event) = tui_event_rx.recv().await {
        use crossterm::event::KeyCode;

        let event = match event {
            case::TuiEvent::Key(key_event) => match key_event.code {
                KeyCode::Char('j') => Some(Event::Increment),
                KeyCode::Char('k') => Some(Event::Decrement),
                KeyCode::Char('g') => Some(Event::Get),
                KeyCode::Char('q') => {
                    // just exit
                    return tui.lock().await.exit();
                }

                _ => None,
            },
            TuiEvent::Resize(_, _) => {
                let view = core.view();

                tui.lock()
                    .await
                    .draw(|f| f.render_widget(TuiViewModel::from(view), f.area()))
                    .map_err(|e| eyre!(e.to_string()))?;

                None
            }
            _ => continue,
        };

        let Some(event) = event else { continue };

        update(&core, event, &effect_tx)?;
    }
    Ok(())
}

async fn effect_handler(
    core: Core,
    tui: Arc<Mutex<Tui>>,
    effect_rx: Receiver<Effect>,
) -> Result<()> {
    while let Ok(effect) = effect_rx.recv() {
        if let Effect::Render(_) = effect {
            let view = core.view();

            tui.lock()
                .await
                .draw(|f| f.render_widget(TuiViewModel::from(view), f.area()))
                .map_err(|e| eyre!(e.to_string()))?;
        }
    }
    Ok(())
}
