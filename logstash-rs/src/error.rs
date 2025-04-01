use std::sync::PoisonError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("sender thread stopped: {0}")]
    SenderThreadStopped(String),
    #[error("address resolution error: {0}:{1}")]
    AddressResolution(String, u16),
    #[error("fatal internal error: {0}")]
    FatalInternal(String),
    #[error("buffer is full")]
    BufferFull(),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::FatalInternal(err.to_string())
    }
}
