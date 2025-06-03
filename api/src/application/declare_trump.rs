use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError, Suit};
use crate::infrastructure::InMemoryGameRepository;

pub struct DeclareTrump {
    pub game_repo: Arc<Mutex<dyn GameRepository + Send + Sync>>
}

impl DeclareTrump {
    pub fn new(repo: Arc<Mutex<dyn GameRepository + Send + Sync>>) -> Self {
        Self {
            game_repo: repo
        }
    }

    pub async fn execute(&self, game_id: GameId, trump: Suit) -> Result<Game, DeclareTrumpError> {
        let game = self.game_repo.lock().await.find_by_id(game_id).await?;
        match game {
            Some(game) => {
                let game = game.declare_trump(trump)?;
                self.game_repo.lock().await.save(game.clone()).await?;
                Ok(game)
            },
            None => Err(DeclareTrumpError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeclareTrumpError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}
