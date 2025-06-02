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

impl HandRepository for InMemoryHandRepository {
    fn find_all(&self) -> Result<Vec<Hand>, HandRepositoryError> {
        Ok(self.hands.iter().map(|hand| hand.value().clone()).collect())
    }

    fn find_by_id(&self, id: HandId) -> Result<Option<Hand>, HandRepositoryError> {
        Ok(self.hands.get(&id)
            .map(|hand| hand.value().clone()))
    }

    fn save(&self, hand: Hand) -> Result<(), HandRepositoryError> {
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

    #[test]
    fn save_and_find_by_id() {
        let repo = InMemoryHandRepository::new();
        let hand = sample_hand();
        repo.save(hand.clone()).unwrap();
        let found = repo.find_by_id(hand.id()).unwrap();
        assert_eq!(found, Some(hand));
    }

    #[test]
    fn find_all_returns_all_saved_hands() {
        let repo = InMemoryHandRepository::new();
        let hand1 = sample_hand();
        let hand2 = sample_hand();
        repo.save(hand1.clone()).unwrap();
        repo.save(hand2.clone()).unwrap();
        let all = repo.find_all().unwrap();
        assert!(all.contains(&hand1));
        assert!(all.contains(&hand2));
    }

    #[test]
    fn find_by_id_nonexistent_returns_none() {
        let repo = InMemoryHandRepository::new();
        let id = HandId(Uuid::new_v4());
        assert_eq!(repo.find_by_id(id).unwrap(), None);
    }

    #[test]
    fn save_twice_updates_hand() {
        let repo = InMemoryHandRepository::new();
        let mut hand = sample_hand();
        repo.save(hand.clone()).unwrap();
        // Update state
        let updated = hand.clone().with_state(HandState::Completed{
            bidder: Player::North,
            bid_amount: 51,
            trump: Suit::Spades,
            us_meld: None,
            them_meld: None,
            us_tricks: None,
            them_tricks: None,
            us_total: None,
            them_total: None
        });
        repo.save(updated.clone()).unwrap();
        let found = repo.find_by_id(hand.id()).unwrap();
        assert_eq!(found, Some(updated));
    }
}
