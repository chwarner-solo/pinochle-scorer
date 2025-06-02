use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError};

pub struct RecordTricks<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> RecordTricks<'a> {
    pub async fn execute(&self, game_id: GameId, us: u32, them: u32) -> Result<Game, RecordTricksError> {
        let game = self.game_repo.find_by_id(game_id)?;
        match game {
            Some(game) => {
                let game = game.record_tricks(us, them)?;
                self.game_repo.save(game.clone())?;
                Ok(game)
            },
            None => Err(RecordTricksError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecordTricksError {
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
    use crate::domain::{Player, Suit};
    use uuid::Uuid;

    fn setup_game_with_hand_bid_trump_and_meld(repo: &InMemoryGameRepository) -> Game {
        let game = Game::new(Player::North)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Hearts)
            .unwrap()
            .record_meld(20, 10)
            .unwrap();
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn records_tricks_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_hand_bid_trump_and_meld(&repo);
        let interactor = RecordTricks { game_repo: &repo };
        let us = 25;
        let them = 25;
        let result = interactor.execute(game.id(), us, them).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        // Optionally, check updated fields
        let found = repo.find_by_id(game.id()).unwrap();
        assert_eq!(found, Some(updated));
    }

    #[tokio::test]
    async fn returns_game_not_found_if_missing() {
        let repo = InMemoryGameRepository::new();
        let interactor = RecordTricks { game_repo: &repo };
        let missing_id = GameId(Uuid::new_v4());
        let result = interactor.execute(missing_id, 10, 8).await;
        assert!(matches!(result, Err(RecordTricksError::GameNotFound)));
    }

    #[tokio::test]
    async fn returns_game_error_on_invalid_tricks() {
        let repo = InMemoryGameRepository::new();
        let game = Game::new(Player::North); // No hand started, so cannot record tricks
        repo.save(game.clone()).unwrap();
        let interactor = RecordTricks { game_repo: &repo };
        let result = interactor.execute(game.id(), 10, 8).await;
        assert!(matches!(result, Err(RecordTricksError::GameError(_))));
    }
}