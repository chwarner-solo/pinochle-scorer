use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{GameId, GameRepository, GameRepositoryError, Hand};
use crate::infrastructure::InMemoryGameRepository;
use tracing::{info, warn};

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
        info!(?game_id, "GetCurrentHand: fetching game");
        let game = self.game_repo.find_by_id(game_id).await?;
        
        if let Some(game) = game {
            info!(?game_id, "GetCurrentHand: game found, returning current hand");
            return Ok(game.current_hand())
        };
        warn!(?game_id, "GetCurrentHand: game NOT FOUND");
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCurrentHandError {
    #[error("Game not found: {0}")]
    GameNotFound(GameId),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError)
}
