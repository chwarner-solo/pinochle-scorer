use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError};
use crate::infrastructure::InMemoryGameRepository;

pub struct RecordMeld {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

impl RecordMeld {
    pub fn new(repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo: repo
        }
    }

    pub async fn execute(&self, game_id: GameId, us: u32, them: u32) -> Result<Game, RecordMeldError> {
        let game = self.game_repo.find_by_id(game_id).await?;
        match game {
            Some(game) => {
                let game = game.record_meld(us, them)?;
                self.game_repo.save(game.clone()).await?;
                Ok(game)
            },
            None => Err(RecordMeldError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecordMeldError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}

