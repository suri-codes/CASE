use thiserror::Error;

#[derive(Error, Debug)]
/// The various errors from the `shared` crate.
pub enum Error {
    /// Any errors pertaining to `NodeId` handling
    #[error("Node Id error! Could be invalid.")]
    NodeIdError(#[from] sakura::NodeIdError),
}

/// Result type used across this crate.
pub type Result<T> = std::result::Result<T, Error>;
