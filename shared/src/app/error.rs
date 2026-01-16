use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{action} \"{item}\". {reason}")]
    Internal {
        action: String,
        item: String,
        reason: String,
    },
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Opml(#[from] opml::Error),
    #[error("{0}")]
    Feed(#[from] feed_rs::parser::ParseFeedError),
}

impl Error {
    pub fn set_error(action: &str, item: &str, reason: &str) -> Self {
        Error::Internal {
            action: action.to_string(),
            item: item.to_string(),
            reason: reason.to_string(),
        }
    }
}
