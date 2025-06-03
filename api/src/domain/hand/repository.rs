use crate::domain::{Hand, HandId};

#[async_trait::async_trait]
pub trait HandRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Hand>, HandRepositoryError>;
    async fn find_by_id(&self, id: HandId) -> Result<Option<Hand>, HandRepositoryError>;
    async fn save(&self, hand: Hand) -> Result<(), HandRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum HandRepositoryError {
    
}