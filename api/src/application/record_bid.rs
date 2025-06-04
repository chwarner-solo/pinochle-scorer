use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError, Player};
use crate::infrastructure::InMemoryGameRepository;

pub struct RecordBid {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

impl RecordBid {
    pub fn new(repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo: repo
        }
    }

    pub async fn execute(&self, game_id: GameId, player: Player, bid: u32) -> Result<Game, RecordBidError> {
        let mut game = self.game_repo.find_by_id(game_id).await?;
        match game {
            Some(game) => {
                let game = game.record_bid(player, bid)?;
                self.game_repo.save(game.clone()).await?;
                Ok(game)
            },
            None => Err(RecordBidError::GameNotFound(game_id))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecordBidError {
    #[error("Game not found: {0}")]
    GameNotFound(GameId),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}
