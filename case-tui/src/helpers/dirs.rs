use directories::ProjectDirs;
use std::{env, path::PathBuf, sync::LazyLock};

static PROJECT_NAME: LazyLock<String> = LazyLock::new(|| "CASE".to_owned());

static DATA_FOLDER: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    env::var(format!("{}_DATA", PROJECT_NAME.clone()))
        .ok()
        .map(PathBuf::from)
});

static CONFIG_FOLDER: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))
        .ok()
        .map(PathBuf::from)
});

/// Returns the directory that holds configuration information for the app.
pub fn get_config_dir() -> PathBuf {
    CONFIG_FOLDER.clone().unwrap_or_else(|| {
        project_directory().map_or_else(
            || PathBuf::from(".").join(".config"),
            |proj_dirs| proj_dirs.config_local_dir().to_path_buf(),
        )
    })
}

/// Will return the config file if it exists in the config directory.
#[must_use]
pub fn get_config_file() -> Option<PathBuf> {
    let file = {
        let mut dir = get_config_dir();
        dir.push("config.toml");

        dir
    };

    file.canonicalize().ok()
}

/// Returns the directory that holds data for the app.
pub fn get_data_dir() -> PathBuf {
    DATA_FOLDER.clone().unwrap_or_else(|| {
        project_directory().map_or_else(
            || PathBuf::from(".").join(".data"),
            |proj_dirs| proj_dirs.data_local_dir().to_path_buf(),
        )
    })
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "suri", env!("CARGO_PKG_NAME"))
}
