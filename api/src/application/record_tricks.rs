use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError};
use crate::infrastructure::InMemoryGameRepository;

pub struct RecordTricks {
    pub game_repo: Arc<Mutex<dyn GameRepository + Send + Sync>>
}

impl RecordTricks {
    pub fn new(repo: Arc<Mutex<dyn GameRepository + Send + Sync>>) -> Self {
        Self {
            game_repo: repo
        }
    }

    pub async fn execute(&self, game_id: GameId, us: u32, them: u32) -> Result<Game, RecordTricksError> {
        let game = self.game_repo.lock().await.find_by_id(game_id).await?;
        match game {
            Some(game) => {
                let game = game.record_tricks(us, them)?;
                self.game_repo.lock().await.save(game.clone()).await?;
                Ok(game)
            },
            None => Err(RecordTricksError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecordTricksError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}
