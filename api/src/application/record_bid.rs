use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError, Player};

pub struct RecordBid<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> RecordBid<'a> {
    
    pub async fn execute(&self, game_id: GameId, player: Player, bid: u32) -> Result<Game, RecordBidError> {
        let mut game = self.game_repo.find_by_id(game_id)?;
        match game {
            Some(game) => {
                let game = game.record_bid(player, bid)?;
                self.game_repo.save(game.clone())?;
                Ok(game)
            },
            None => Err(RecordBidError::GameNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RecordBidError {
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
    use crate::domain::{Player, Game, GameId, Suit};
    use uuid::Uuid;

    fn setup_game_with_hand(repo: &InMemoryGameRepository) -> Game {
        let mut game = Game::new(Player::North)
            .start_new_hand()
            .unwrap();
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn records_bid_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_hand(&repo);
        let interactor = RecordBid { game_repo: &repo };;
        let result = interactor.execute(game.id(), Player::North, 51).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        // Check that the bid is recorded
        assert_eq!(updated.current_hand().unwrap().bidder(), Some(Player::North));
        assert_eq!(updated.current_hand().unwrap().bid_amount(), Some(51));
        // Confirm the repo has the updated game
        let found = repo.find_by_id(game.id()).unwrap();
        assert_eq!(found, Some(updated));
    }

    #[tokio::test]
    async fn returns_game_not_found_if_missing() {
        let repo = InMemoryGameRepository::new();
        let interactor = RecordBid { game_repo: &repo };
        let missing_id = GameId(Uuid::new_v4());
        let result = interactor.execute(missing_id, Player::North, 51).await;
        assert!(matches!(result, Err(RecordBidError::GameNotFound)));
    }

    #[tokio::test]
    async fn returns_game_error_on_invalid_bid() {
        let repo = InMemoryGameRepository::new();
        let game = Game::new(Player::North); // No hand started, so cannot bid
        repo.save(game.clone()).unwrap();
        let interactor = RecordBid { game_repo: &repo };
        let result = interactor.execute(game.id(), Player::North, 51).await;
        assert!(matches!(result, Err(RecordBidError::GameError(_))));
    }
}