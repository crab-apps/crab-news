use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{action} \"{item}\". {reason}")]
    CrabError {
        action: String,
        item: String,
        reason: String,
    },
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Opml(#[from] opml::Error),
}
