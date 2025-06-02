use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError};

pub struct RecordMeld<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> RecordMeld<'a> {
    pub async fn execute(&self, game_id: GameId, us: u32, them: u32) -> Result<Game, RecordMeldError> {
        let game = self.game_repo.find_by_id(game_id)?;
        match game {
            Some(game) => {
                let game = game.record_meld(us, them)?;
                self.game_repo.save(game.clone())?;
                Ok(game)
            },
            None => Err(RecordMeldError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecordMeldError {
    #[error("Game not found")]
    GameNotFound,
    #[error("Repository error: {0}")]
    RepositoryError(#[from] GameRepositoryError),
    #[error("Game error: {0}")]
    GameError(#[from] GameError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::InMemoryGameRepository;
    use crate::domain::{Game, Player, Suit};
    use uuid::Uuid;

    fn setup_game_with_hand_bid_and_trump(repo: &InMemoryGameRepository) -> Game {
        let game = Game::new(Player::North)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Hearts)
            .unwrap();
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn records_meld_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_hand_bid_and_trump(&repo);
        let interactor = RecordMeld { game_repo: &repo };
        let us = 20;
        let them = 10;
        let result = interactor.execute(game.id(), us, them).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        // Check that the meld totals are updated (domain logic may require more precise checks)
        assert_eq!(updated.current_hand().unwrap().us_meld(), Some(us));
        assert_eq!(updated.current_hand().unwrap().them_meld(), Some(them));
        let found = repo.find_by_id(game.id()).unwrap();
        assert_eq!(found, Some(updated));
    }

    #[tokio::test]
    async fn returns_game_not_found_if_missing() {
        let repo = InMemoryGameRepository::new();
        let interactor = RecordMeld { game_repo: &repo };
        let missing_id = GameId(Uuid::new_v4());
        let result = interactor.execute(missing_id, 20, 10).await;
        assert!(matches!(result, Err(RecordMeldError::GameNotFound)));
    }

    #[tokio::test]
    async fn returns_game_error_on_invalid_meld() {
        let repo = InMemoryGameRepository::new();
        let game = Game::new(Player::North); // No hand started, so cannot record meld
        repo.save(game.clone()).unwrap();
        let interactor = RecordMeld { game_repo: &repo };
        let result = interactor.execute(game.id(), 20, 10).await;
        assert!(matches!(result, Err(RecordMeldError::GameError(_))));
    }
}
