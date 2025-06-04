use crate::domain::HandError;

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),
    #[error("Invalid game operation: {0}")]
    InvalidOperation(String),
    #[error("Hand error: {0}")]
    HandError(#[from] HandError)
}
