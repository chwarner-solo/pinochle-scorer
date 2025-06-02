use crate::domain::{GameId, GameRepository, GameRepositoryError, Hand};

pub struct GetCurrentHand<'a>{
    pub game_repo: &'a dyn GameRepository
}

impl<'a> GetCurrentHand<'a> {
    pub fn execute(&self, game_id: GameId) -> Result<Option<Hand>, GetCurrentHandError> {
        let game = self.game_repo.find_by_id(game_id)?;
        
        if let Some(game) = game {
            return Ok(game.current_hand())
        };
        
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCurrentHandError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::InMemoryGameRepository;
    use crate::domain::{Game, Player, Suit};
    use uuid::Uuid;

    fn setup_game_with_hand(repo: &InMemoryGameRepository) -> Game {
        let game = Game::new(Player::North)
            .start_new_hand()
            .unwrap();
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn gets_current_hand_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_hand(&repo);
        let interactor = GetCurrentHand { game_repo: &repo };
        let result = interactor.execute(game.id());
        assert!(result.is_ok());
        let hand = result.unwrap();
        assert!(hand.is_some());
        // Optionally, check hand fields
    }

    #[tokio::test]
    async fn returns_none_if_no_current_hand() {
        let repo = InMemoryGameRepository::new();
        let game = Game::new(Player::North);
        repo.save(game.clone()).unwrap();
        let interactor = GetCurrentHand { game_repo: &repo };
        let result = interactor.execute(game.id());
        assert!(result.is_ok());
        let hand = result.unwrap();
        assert!(hand.is_none());
    }
    
}