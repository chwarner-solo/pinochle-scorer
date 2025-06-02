use crate::domain::{Game, GameId, GameRepository, GameRepositoryError, Hand};

pub struct GetCompletedHands<'a> {
    pub game_repo: &'a dyn GameRepository
}

impl<'a> GetCompletedHands<'a> {
    pub fn execute(&self, game_id: GameId) -> Result<Vec<Hand>, GetCompletedHandsError> {
        let game = self.game_repo.find_by_id(game_id)?;
        
        if let Some(game) = game {
            return Ok(game.completed_hands());
        }
        
        Ok(Vec::new())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetCompletedHandsError {
    #[error("Game Repository Error: {0}")]
    GameRepoError(#[from] GameRepositoryError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::InMemoryGameRepository;
    use crate::domain::{Player};
    use uuid::Uuid;

    fn setup_game_with_completed_hands(repo: &InMemoryGameRepository) -> Game {
        let mut game = Game::new(Player::North)
            .start_new_hand()
            .unwrap();
        // Simulate completing a hand if your domain allows
        // game = game.complete_hand().unwrap();
        repo.save(game.clone()).unwrap();
        game
    }

    #[tokio::test]
    async fn gets_completed_hands_successfully() {
        let repo = InMemoryGameRepository::new();
        let game = setup_game_with_completed_hands(&repo);
        let interactor = GetCompletedHands { game_repo: &repo };
        let result = interactor.execute(game.id());
        assert!(result.is_ok());
        let hands = result.unwrap();
        // Check hands as needed
    }

}