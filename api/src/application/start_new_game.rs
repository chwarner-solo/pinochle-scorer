use crate::domain::{Game, GameRepository, GameRepositoryError, Player};

pub struct StartNewGame<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> StartNewGame<'a> {
    pub async fn execute(&self, dealer: Player) -> Result<Game, StartNewGameError> {
        let game = Game::new(dealer);
        self.game_repo.save(game.clone())?;
        
        Ok(game)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StartNewGameError {
    #[error("Game repository error: {0}")]
    GameRepositoryError(#[from] GameRepositoryError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::InMemoryGameRepository;
    use crate::domain::Player;

    #[tokio::test]
    async fn starts_new_game_successfully() {
        let repo = InMemoryGameRepository::new();
        let interactor = StartNewGame { game_repo: &repo };
        let result = interactor.execute(Player::North).await;
        assert!(result.is_ok());
        let game = result.unwrap();
        // Confirm the game is in the repo
        let found = repo.find_by_id(game.id());
        assert!(found.is_ok());
        assert_eq!(found.unwrap(), Some(game));
    }
}