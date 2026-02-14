use color_eyre::{Result, eyre::eyre};
use crossbeam_channel::Sender;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::spawn;
use tracing::debug;

use shared::{Counter, Effect, Event};

use crate::{http, sse};

pub type Core = Arc<shared::Core<Counter>>;

#[must_use]
pub fn new() -> Core {
    Arc::new(shared::Core::new())
}

/// # Errors
///
/// Can error if processing an effect fails.
pub fn update(core: &Core, event: Event, tx: &Sender<Effect>) -> Result<()> {
    debug!("event: {:?}", event);

    for effect in core.process_event(event) {
        process_effect(core, effect, tx)?;
    }
    Ok(())
}

/// # Errors
///
/// Can error in many scenarios.
pub fn process_effect(core: &Core, effect: Effect, tx: &Sender<Effect>) -> Result<()> {
    debug!("effect: {:?}", effect);

    match effect {
        render @ Effect::Render(_) => {
            tx.send(render).map_err(|e| eyre!("{e:?}"))?;
        }

        Effect::Http(mut request) => {
            spawn({
                let core = core.clone();
                let tx = tx.clone();

                async move {
                    let response = http::request(&request.operation).await;

                    for effect in core.resolve(&mut request, response.into())? {
                        process_effect(&core, effect, &tx)?;
                    }
                    Result::<()>::Ok(())
                }
            });
        }

        Effect::ServerSentEvents(mut request) => {
            spawn({
                let core = core.clone();
                let tx = tx.clone();
                let operation = request.operation.clone();

                async move {
                    let mut stream = sse::request(&operation).await?;

                    while let Ok(Some(response)) = stream.try_next().await {
                        for effect in core.resolve(&mut request, response)? {
                            process_effect(&core, effect, &tx)?;
                        }
                    }
                    Result::<()>::Ok(())
                }
            });
        }
    }
    Ok(())
}
