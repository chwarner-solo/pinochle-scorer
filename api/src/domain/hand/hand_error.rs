#[derive(Debug, thiserror::Error)]
pub enum HandError {
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Invalid bid: {0}")]
    InvalidBid(String),

    #[error("Total tricks must not exceed 50: {0} + {1}")]
    InvalidTricks(u32, u32)
}
