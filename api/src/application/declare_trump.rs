use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError, Suit};
use crate::infrastructure::InMemoryGameRepository;
use tracing::{info, warn};

pub struct DeclareTrump {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

impl DeclareTrump {
    pub fn new(repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo: repo
        }
    }

    pub async fn execute(&self, game_id: GameId, trump: Suit) -> Result<Game, DeclareTrumpError> {
        info!(?game_id, trump=?trump, "DeclareTrump: fetching game");
        let game = self.game_repo.find_by_id(game_id).await?;
        match game {
            Some(game) => {
                info!(?game_id, trump=?trump, "DeclareTrump: game found, declaring trump");
                let game = game.declare_trump(trump)?;
                self.game_repo.save(game.clone()).await?;
                Ok(game)
            },
            None => {
                warn!(?game_id, trump=?trump, "DeclareTrump: game NOT FOUND");
                Err(DeclareTrumpError::GameNotFound(game_id))
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeclareTrumpError {
    #[error("Game not found: {0}")]
    GameNotFound(GameId),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}
