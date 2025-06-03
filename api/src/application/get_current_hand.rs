use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{GameId, GameRepository, GameRepositoryError, Hand};
use crate::infrastructure::InMemoryGameRepository;

pub struct GetCurrentHand{
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

impl GetCurrentHand {
    pub fn new(repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo: repo
        }
    }
    pub async fn execute(&self, game_id: GameId) -> Result<Option<Hand>, GetCurrentHandError> {
        let game = self.game_repo.find_by_id(game_id).await?;
        
        if let Some(game) = game {
            return Ok(game.current_hand())
        };
        
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCurrentHandError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError)
}
