use crate::domain::{GameId, GameRepository, GameRepositoryError};

pub struct GetRunningTotal<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> GetRunningTotal<'a> {
    pub fn execute(&self, game_id: GameId) -> Result<RunningTotal, GetRunningTotalError> {
        let game = self.game_repo.find_by_id(game_id)?;
        
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::InMemoryGameRepository;
    use crate::domain::{Game, Player};
    use uuid::Uuid;

    fn setup_game_with_scores(repo: &InMemoryGameRepository) -> Game {
        let mut game = Game::new(Player::North)
            .start_new_hand()
            .unwrap();
        // Optionally, record scores if needed
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn gets_running_total_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_scores(&repo);
        let interactor = GetRunningTotal { game_repo: &repo };
        let result = interactor.execute(game.id());
        assert!(result.is_ok());
        let total = result.unwrap();
        // Optionally, check total fields
    }

    #[tokio::test]
    async fn returns_error_if_game_not_found() {
        let repo = InMemoryGameRepository::new();
        let interactor = GetRunningTotal { game_repo: &repo };
        let missing_id = GameId(Uuid::new_v4());
        let result = interactor.execute(missing_id);
        assert!(result.is_err());
    }
}