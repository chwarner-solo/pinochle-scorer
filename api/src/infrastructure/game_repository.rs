use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::Mutex;
use crate::domain::{Game, GameId, GameRepository, GameRepositoryError};

pub struct InMemoryGameRepository {
    games: Arc<DashMap<GameId, Game>>
}

impl InMemoryGameRepository {
    pub fn new() -> Self {
        Self {
            games: Arc::new(DashMap::new())
        }
    }
}

#[async_trait::async_trait]
impl GameRepository for InMemoryGameRepository {
    async fn  find_all(&self) -> Result<Vec<Game>, GameRepositoryError> {
        Ok(self.games.iter().map(|game| game.value().clone()).collect())
    }

    async fn find_by_id(&self, id: GameId) -> Result<Option<Game>, GameRepositoryError> {
        if !self.games.contains_key(&id) {
           return Err(GameRepositoryError::GameDoesNotExist(id));
        }
        Ok(self.games.get(&id).map(|game| game.value().clone()))
    }

    async fn save(&self, game: Game) -> Result<(), GameRepositoryError> {

        if self.games.contains_key(&game.id()) {
            let updated_game = {
                let orig = self.games.get(&game.id()).unwrap();

                orig.value()
                    .clone()
                    .with_current_hand(game.current_hand())
                    .with_state(game.state())
                    .with_completed_hands(game.completed_hands())

            };

            self.games.insert(game.id().clone(), updated_game);
        } else {
            self.games.insert(game.id().clone(), game.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Game, GameId, Player, GameState};
    use uuid::Uuid;

    fn sample_game() -> Game {
        // Construct a minimal valid Game.
        Game::new(Player::North)
    }

    #[tokio::test]
    async fn save_and_find_by_id() {
        let repo = InMemoryGameRepository::new();
        let game = sample_game();
        repo.save(game.clone()).await.unwrap();
        let found = repo.find_by_id(game.id()).await.unwrap();
        assert_eq!(found, Some(game));
    }

    #[tokio::test]
    async fn find_all_returns_all_saved_games() {
        let repo = InMemoryGameRepository::new();
        let game1 = sample_game();
        let game2 = sample_game();
        repo.save(game1.clone()).await.unwrap();
        repo.save(game2.clone()).await.unwrap();
        let all = repo.find_all().await.unwrap();
        assert!(all.contains(&game1));
        assert!(all.contains(&game2));
    }

    #[tokio::test]
    async fn find_by_id_nonexistent_returns_none() {
        let repo = InMemoryGameRepository::new();
        let id = GameId(Uuid::new_v4());
        let game = repo.find_by_id(id).await;
        
        assert!(game.is_err());
        
        assert_eq!(game.unwrap_err(), GameRepositoryError::GameDoesNotExist(id));
    }

}
