use std::fs;
use std::fs::OpenOptions;

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::helpers::dirs::get_data_dir;

const LOG_FILE_NAME: &str = "case.log";

const LOG_ENV_VAR: &str = "CASE_LOG_LEVEL";

/// Inits logging for the application.
///
/// Creates a logfile in the `data_dir` for the application, respecting
/// the preferences of the host's operating system.
///
/// Can specify `LOG_LEVEL` through the `CASE_LOG_LEVEL` environment variable.
///
/// # Errors
/// Can error if any part of initialization fails, however
/// such a case is unlikely.
pub fn init_logging() -> crate::Result<()> {
    let directory = get_data_dir();
    fs::create_dir_all(directory.clone())?;

    let log_path = directory.join(LOG_FILE_NAME);

    let log_file = OpenOptions::new()
        .read(true)
        .create(true)
        .truncate(false)
        .append(true)
        .open(log_path)?;

    let env_filter = EnvFilter::builder().with_default_directive(tracing::Level::INFO.into());

    // If the `RUST_LOG` environment variable is set, use that as the default, otherwise use the
    // value of the `LOG_ENV_VAR` environment variable. If the `LOG_ENV_VAR` environment variable contains
    // errors, then this will return an error.
    let env_filter = env_filter
        .try_from_env()
        .or_else(|_| env_filter.with_env_var(LOG_ENV_VAR.to_owned()).from_env())?;

    let file_subscriber = fmt::layer()
        .with_file(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_subscriber)
        .try_init()?;

    Ok(())
}
