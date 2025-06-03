use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{Game, GameRepository, GameRepositoryError, Player};

pub struct StartNewGame {
    pub game_repo: Arc<Mutex<dyn GameRepository + Send + Sync>>
}

impl StartNewGame {
    pub fn new(repo: Arc<Mutex<dyn GameRepository + Send + Sync>>) -> Self {
        Self {
            game_repo: repo
        }
    }

    pub async fn execute(&self, dealer: Player) -> Result<Game, StartNewGameError> {
        let game = Game::new(dealer);
        self.game_repo.lock().await.save(game.clone()).await?;
        
        Ok(game)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StartNewGameError {
    #[error("Game repository error: {0}")]
    GameRepositoryError(#[from] GameRepositoryError)
}
