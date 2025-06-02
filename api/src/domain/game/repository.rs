use crate::domain::game::{Game, GameId};

pub trait GameRepository {
    fn find_all(&self) -> Result<Vec<Game>, GameRepositoryError>;
    fn find_by_id(&self, id: GameId) -> Result<Option<Game>, GameRepositoryError>;
    fn save(&self, game: Game) -> Result<(), GameRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum GameRepositoryError {
    
}