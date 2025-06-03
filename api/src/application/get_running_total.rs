use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::{GameId, GameRepository, GameRepositoryError};
use crate::infrastructure::InMemoryGameRepository;

pub struct GetRunningTotal {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

impl GetRunningTotal {
    pub fn new(repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo: repo
        }
    }
    pub async fn execute(&self, game_id: GameId) -> Result<RunningTotal, GetRunningTotalError> {
        let game = self.game_repo.find_by_id(game_id).await?;
        
        if let Some(game) = game {
            let (us, them) = game.running_totals();
            Ok(RunningTotal { us, them })
        } else {
            Err(GetRunningTotalError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetRunningTotalError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError)
    
}

pub struct RunningTotal {
    pub us: i32,
    pub them: i32
}

