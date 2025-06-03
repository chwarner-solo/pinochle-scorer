use std::sync::Arc;
use dashmap::DashMap;
use crate::domain::{Hand, HandId, HandRepository, HandRepositoryError};

pub struct InMemoryHandRepository {
    hands: Arc<DashMap<HandId, Hand>>
}

impl InMemoryHandRepository {
    pub fn new() -> Self {
        Self {
            hands: Arc::new(DashMap::new())
        }
    }
}

#[async_trait::async_trait]
impl HandRepository for InMemoryHandRepository {
    async fn find_all(&self) -> Result<Vec<Hand>, HandRepositoryError> {
        Ok(self.hands.iter().map(|hand| hand.value().clone()).collect())
    }

    async fn find_by_id(&self, id: HandId) -> Result<Option<Hand>, HandRepositoryError> {
        Ok(self.hands.get(&id)
            .map(|hand| hand.value().clone()))
    }

    async fn save(&self, hand: Hand) -> Result<(), HandRepositoryError> {
        if let Some(orig) = self.hands.get(&hand.id()) {
            let updated_hand = orig.value()
                .clone()
                .with_state(hand.state());
            self.hands.insert(hand.id(), updated_hand);
        } else {
            self.hands.insert(hand.id(), hand);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Hand, HandId, Player, HandState, Suit};
    use uuid::Uuid;

    fn sample_hand() -> Hand {
        // Construct a minimal valid Hand.
        Hand::new(Player::North)
    }

    #[tokio::test]
    async fn save_and_find_by_id() {
        let repo = InMemoryHandRepository::new();
        let hand = sample_hand();
        repo.save(hand.clone()).await.unwrap();
        let found = repo.find_by_id(hand.id()).await.unwrap();
        assert_eq!(found, Some(hand));
    }

    #[tokio::test]
    async fn find_all_returns_all_saved_hands() {
        let repo = InMemoryHandRepository::new();
        let hand1 = sample_hand();
        let hand2 = sample_hand();
        repo.save(hand1.clone()).await.unwrap();
        repo.save(hand2.clone()).await.unwrap();
        let all = repo.find_all().await.unwrap();
        assert!(all.contains(&hand1));
        assert!(all.contains(&hand2));
    }

    #[tokio::test]
    async fn find_by_id_nonexistent_returns_none() {
        let repo = InMemoryHandRepository::new();
        let id = HandId(Uuid::new_v4());
        assert_eq!(repo.find_by_id(id).await.unwrap(), None);
    }
}
