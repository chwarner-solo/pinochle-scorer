use std::sync::Arc;
use dashmap::DashMap;
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

impl GameRepository for InMemoryGameRepository {
    fn find_all(&self) -> Result<Vec<Game>, GameRepositoryError> {
        Ok(self.games.iter().map(|game| game.value().clone()).collect())
    }

    fn find_by_id(&self, id: GameId) -> Result<Option<Game>, GameRepositoryError> {
        Ok(self.games.get(&id).map(|game| game.value().clone()))
    }

    fn save(&self, game: Game) -> Result<(), GameRepositoryError> {
        if let Some(orig) = self.games.get(&game.id()) {
            let updated_game = orig.value()
                .clone()
                .with_current_hand(game.current_hand())
                .with_state(game.state());
            self.games.insert(game.id(), updated_game);
        } else {
            self.games.insert(game.id(), game);
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

    #[test]
    fn save_and_find_by_id() {
        let repo = InMemoryGameRepository::new();
        let game = sample_game();
        repo.save(game.clone()).unwrap();
        let found = repo.find_by_id(game.id()).unwrap();
        assert_eq!(found, Some(game));
    }

    #[test]
    fn find_all_returns_all_saved_games() {
        let repo = InMemoryGameRepository::new();
        let game1 = sample_game();
        let game2 = sample_game();
        repo.save(game1.clone()).unwrap();
        repo.save(game2.clone()).unwrap();
        let all = repo.find_all().unwrap();
        assert!(all.contains(&game1));
        assert!(all.contains(&game2));
    }

    #[test]
    fn find_by_id_nonexistent_returns_none() {
        let repo = InMemoryGameRepository::new();
        let id = GameId(Uuid::new_v4());
        assert_eq!(repo.find_by_id(id).unwrap(), None);
    }

    #[test]
    fn save_twice_updates_game() {
        let repo = InMemoryGameRepository::new();
        let mut game = sample_game();
        repo.save(game.clone()).unwrap();
        // Update state
        let updated = game.clone().with_state(GameState::Completed);
        repo.save(updated.clone()).unwrap();
        let found = repo.find_by_id(game.id()).unwrap();
        assert_eq!(found, Some(updated));
    }
}
