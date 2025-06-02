use crate::domain::{Game, GameError, GameId, GameRepository, GameRepositoryError, Suit};

pub struct DeclareTrump<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> DeclareTrump<'a> {
    pub async fn execute(&self, game_id: GameId, trump: Suit) -> Result<Game, DeclareTrumpError> {
        let game = self.game_repo.find_by_id(game_id)?;
        match game {
            Some(game) => {
                let game = game.declare_trump(trump)?;
                self.game_repo.save(game.clone())?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::InMemoryGameRepository;
    use crate::domain::{Player, Suit};
    use uuid::Uuid;

    fn setup_game_with_hand_and_bid(repo: &InMemoryGameRepository) -> Game {
        let game = Game::new(Player::North)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap();
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn declares_trump_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_hand_and_bid(&repo);
        let interactor = DeclareTrump { game_repo: &repo };
        let result = interactor.execute(game.id(), Suit::Hearts).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.current_hand().unwrap().trump(), Some(Suit::Hearts));
        let found = repo.find_by_id(game.id()).unwrap();
        assert_eq!(found, Some(updated));
    }

    #[tokio::test]
    async fn returns_game_not_found_if_missing() {
        let repo = InMemoryGameRepository::new();
        let interactor = DeclareTrump { game_repo: &repo };
        let missing_id = GameId(Uuid::new_v4());
        let result = interactor.execute(missing_id, Suit::Clubs).await;
        assert!(matches!(result, Err(DeclareTrumpError::GameNotFound)));
    }

    #[tokio::test]
    async fn returns_game_error_on_invalid_trump() {
        let repo = InMemoryGameRepository::new();
        let game = Game::new(Player::North); // No hand started, so cannot declare trump
        repo.save(game.clone()).unwrap();
        let interactor = DeclareTrump { game_repo: &repo };
        let result = interactor.execute(game.id(), Suit::Clubs).await;
        assert!(matches!(result, Err(DeclareTrumpError::GameError(_))));
    }
}