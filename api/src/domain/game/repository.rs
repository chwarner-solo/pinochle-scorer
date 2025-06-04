use crate::domain::game::{Game, GameId};

#[async_trait::async_trait]
pub trait GameRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Game>, GameRepositoryError>;
    async fn find_by_id(&self, id: GameId) -> Result<Option<Game>, GameRepositoryError>;
    async fn save(&self, game: Game) -> Result<(), GameRepositoryError>;
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GameRepositoryError {
    #[error("Game repository error: {0}")]
    GameRepositoryError(String),
    #[error("Game isn't found: {0}")]
    GameDoesNotExist(GameId),
}