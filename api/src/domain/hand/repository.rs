use crate::domain::{Hand, HandId};

pub trait HandRepository {
    fn find_all(&self) -> Result<Vec<Hand>, HandRepositoryError>;
    fn find_by_id(&self, id: HandId) -> Result<Option<Hand>, HandRepositoryError>;
    fn save(&self, hand: Hand) -> Result<(), HandRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum HandRepositoryError {
    
}