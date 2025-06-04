use std::sync::Arc;
use tokio::sync::Mutex;
use crate::application::RecordTricksError;
use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError};

pub struct StartNewHand {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

impl StartNewHand {
    pub fn new(game_repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
             game_repo,
        }
    }
    
    pub async fn execute(&self, game_id: GameId) -> Result<Game, StartNewHandError>
    {
        let game = self.game_repo.find_by_id(game_id).await?;
        
        match game {
            Some(game) => {
                let game = game.start_new_hand()?;
                self.game_repo.save(game.clone()).await?;
                Ok(game)
            },
            None => Err(StartNewHandError::GameNotFound(game_id))
        }
        
        
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StartNewHandError {
    #[error("Game not found: {0}")]
    GameNotFound(GameId),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}