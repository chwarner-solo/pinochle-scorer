use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameId, GameRepository, GameRepositoryError, Hand};
use crate::infrastructure::InMemoryGameRepository;
use tracing::{info, warn};

pub struct GetCompletedHands {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>, // &'a dyn GameRepository
}

impl GetCompletedHands {
    pub fn new(repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo: repo
        }
    }
    pub async fn execute(&self, game_id: GameId) -> Result<Vec<Hand>, GetCompletedHandsError> {
        info!(?game_id, "GetCompletedHands: fetching game");
        let game = self.game_repo.find_by_id(game_id).await?;
        
        if let Some(game) = game {
            info!(?game_id, "GetCompletedHands: game found, returning completed hands");
            return Ok(game.completed_hands());
        }
        warn!(?game_id, "GetCompletedHands: game NOT FOUND");
        Ok(Vec::new())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCompletedHandsError {
    #[error("Game Repository Error: {0}")]
    GameRepoError(#[from] GameRepositoryError)
}
