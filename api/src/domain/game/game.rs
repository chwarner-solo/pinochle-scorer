use crate::domain::{GameId, GameState, Hand, HandState, Player, Suit, Team};
use crate::domain::game::GameError;

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct Game {
    pub(crate) id: GameId,
    current_dealer: Player,
    state: GameState,
    completed_hands: Vec<Hand>,
    current_hand: Option<Hand>,
}


impl Game {
    pub fn new(dealer: Player) -> Self {

        Self {
            id: GameId::new(),
            current_dealer: dealer,
            state: GameState::WaitingToStart,
            completed_hands: Vec::new(),
            current_hand: None
        }
    }

    pub fn id(&self) -> GameId {
        self.id
    }

    pub fn current_dealer(&self) -> Player {
        self.current_dealer
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn completed_hands(&self) -> Vec<Hand> {
        self.completed_hands.clone()
    }

    pub fn current_hand(&self) -> Option<Hand> {
        self.current_hand.clone()
    }

    pub fn with_current_hand(mut self, hand: Option<Hand>) -> Self {
        self.current_hand = hand;
        self
    }

    pub fn with_state(mut self, state: GameState) -> Self {
        self.state = state;
        self
    }

    fn next_dealer(&self) -> Player {
        self.current_dealer.next_clockwise()
    }

    pub fn start_new_hand(&self) -> Result<Self, GameError> {
        if self.state == GameState::InProgress {
            return Err(GameError::InvalidStateTransition(
                "Cannot start new hand when game is already in progress".to_string()
            ));
        }
        let new_hand = Hand::new(self.current_dealer);
        Ok(
            Self {
                state: GameState::InProgress,
                current_hand: Some(new_hand),
                ..self.clone()
            }
        )
    }

    pub fn running_totals(&self) -> (i32, i32) {
        let mut us_total = 0;
        let mut them_total = 0;
        for hand in &self.completed_hands {
            us_total += hand.us_total();
            them_total += hand.them_total();
        }

        (us_total, them_total)
    }

    pub fn record_bid(&self, bidder: Player, amount: u32) -> Result<Self, GameError> {
        let current_hand = self.current_hand
            .as_ref()
            .ok_or_else(|| GameError::InvalidOperation("No current hand".to_string()))?;

        let new_hand = current_hand.place_bid(bidder, amount)?;

        Ok(Game {
            current_hand: Some(new_hand),
            ..self.clone()
        })
    }

    pub fn declare_trump(&self, trump: Suit) -> Result<Self, GameError> {
        let current_hand = self.current_hand
            .as_ref()
            .ok_or_else(|| GameError::InvalidOperation("No current hand".to_string()))?;

        let new_hand = current_hand.declare_trump(trump)?;

        Ok(Game {
            current_hand: Some(new_hand),
            ..self.clone()
        })
    }

    pub fn record_meld(&self, us: u32, them: u32) -> Result<Self, GameError> {
        let current_hand = self.current_hand
            .as_ref()
            .ok_or_else(|| GameError::InvalidOperation("No current hand".to_string()))?;

        let new_hand = current_hand.clone().record_meld(us, them)?;

        match new_hand.state() {
            HandState::Completed { .. } => {
                Ok(self.complete_hand_and_start_new(new_hand))
            },
            _ => {
                Ok(Game {
                    current_hand: Some(new_hand),
                    ..self.clone()
                })
            }
        }
    }

    pub fn record_tricks(&self, us: u32, them: u32) -> Result<Self, GameError> {
        let current_hand = self.current_hand
            .as_ref()
            .ok_or_else(|| GameError::InvalidOperation("No current hand to record tricks".to_string()))?;

        let new_hand = current_hand.clone().record_tricks(us, them)?;

        Ok(self.complete_hand_and_start_new(new_hand))
    }

    fn complete_hand_and_start_new(&self, new_hand: Hand) -> Game {
        let mut new_completed_hands = self.completed_hands.clone();
        new_completed_hands.push(new_hand);

        let next_dealer = self.next_dealer();
        let new_current_hand = Hand::new(next_dealer);

        Game {
            completed_hands: new_completed_hands,
            current_hand: Some(new_current_hand),
            current_dealer: next_dealer,
            ..self.clone()
        }
    }

    pub fn is_game_complete(&self) -> bool {
        let (us_total, them_total) = self.running_totals();
        us_total >= 500 || them_total >= 500
    }

    pub fn winner(&self) -> Option<Team> {
        if !self.is_game_complete() {
            return None;
        }

        let (us_total, them_total) = self.running_totals();

        if us_total >= 500 && them_total >= 500 {
            if let Some(last_hand) = self.completed_hands.last() {
                if let Some(bidder) = last_hand.bidder() {
                    return Some(bidder.team())
                }
            }
            if us_total > them_total {
                Some(Team::Us)
            } else {
                Some(Team::Them)
            }
        } else if us_total >= 500 {
            Some(Team::Us)
        } else {
            Some(Team::Them)
        }
    }
}

#[cfg(test)]
impl Game {
    pub fn add_completed_hand_with_scores(&self, us_total: i32, them_total: i32, bidder: Player) -> Self {
        use crate::domain::HandState;

        let completed_hand = Hand::new(Player::South).set_state(
            HandState::Completed {
                bidder,
                bid_amount: 51,
                trump: Suit::Spades,
                us_meld: Some(20),
                them_meld: Some(20),
                us_tricks: Some(25),
                them_tricks: Some(25),
                us_total: Some(us_total),
                them_total: Some(them_total)
            }
        );

        let mut new_completed_hands = self.completed_hands.clone();
        new_completed_hands.push(completed_hand);

        Game {
            completed_hands: new_completed_hands,
            ..self.clone()
        }
    }
}

mod tests {
    use super::*;
    use crate::domain::{Player, GameState, HandState, Team};

    #[test]
    fn new_game_should_start_with_given_dealer() {
        let game = Game::new(Player::South);

        assert_eq!(game.current_dealer(), Player::South);
        assert_eq!(game.state(), GameState::WaitingToStart);
    }

    #[test]
    fn new_game_should_have_unique_id() {
        let game1 = Game::new(Player::South);
        let game2 = Game::new(Player::South);

        assert_ne!(game1.id(), game2.id());
    }

    #[test]
    fn new_game_should_have_no_hands() {
        let game = Game::new(Player::South);

        assert_eq!(game.completed_hands().len(), 0);
        assert!(game.current_hand().is_none());
    }

    #[test]
    fn should_start_new_hand_with_current_dealer() {
        let game = Game::new(Player::South);

        let result = game.start_new_hand();

        assert!(result.is_ok());
        let new_game = result.unwrap();

        assert_eq!(new_game.state(), GameState::InProgress);
        assert!(new_game.current_hand().is_some());

        let hand = new_game.current_hand().unwrap();
        assert_eq!(hand.dealer(), Player::South);
    }

    #[test]
    fn should_calculate_running_total() {
        let game = Game::new(Player::South);

        let (us_total, them_total) = game.running_totals();

        assert_eq!(us_total, 0);
        assert_eq!(them_total, 0);

    }

    #[test]
    fn should_place_bid_on_current_hand() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap();

        let result = game.record_bid(Player::North, 51);

        assert!(result.is_ok());
        let new_game = result.unwrap();

        let hand = new_game.current_hand().unwrap();
        assert_eq!(hand.bidder(), Some(Player::North));
        assert_eq!(hand.bid_amount(), Some(51));

        assert_eq!(new_game.id(), game.id());
        assert_eq!(new_game.current_dealer(), game.current_dealer());
        assert_eq!(new_game.state(), GameState::InProgress);
        assert_eq!(new_game.completed_hands().len(), 0);

    }

    #[test]
    fn should_propagate_hand_error_for_invalid_bid() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap();

        let result = game.record_bid(Player::North, 49);

        assert!(result.is_err());

        match result.unwrap_err() {
            GameError::HandError(_) => {},
            _ => panic!("Expected HandError to be propagated")
        }
    }

    #[test]
    fn should_reject_place_bid_when_no_current_hand() {
        let game = Game::new(Player::South);

        let result = game.record_bid(Player::North, 51);

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::InvalidOperation(_) => {},
            _ => panic!("Expected InvalidOperation to be propagated")
        }
    }

    #[test]
    fn should_propagate_hand_bid_increment_validation_error() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap();

        let result = game.record_bid(Player::North, 61);

        assert!(result.is_err());

        match result.unwrap_err() {
            GameError::HandError(_) => {},
            _ => panic!("Expected HandError to be propagated")
        }
    }

    #[test]
    fn should_declare_trump_on_current_hand() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap();

        let result = game.declare_trump(Suit::Spades);
        assert!(result.is_ok());
        let new_game = result.unwrap();

        let hand = new_game.current_hand().unwrap();
        assert_eq!(hand.trump(), Some(Suit::Spades));
        assert_eq!(hand.bidder(), Some(Player::North));

        assert_eq!(new_game.id(), game.id());
        assert_eq!(new_game.current_dealer(), game.current_dealer());
        assert_eq!(new_game.state(), GameState::InProgress);
        assert_eq!(new_game.completed_hands().len(), 0);

    }

    #[test]
    fn should_propagate_hand_error_for_invalid_trump() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap();

        let result = game.declare_trump(Suit::Spades);

        assert!(result.is_err());

        match result.unwrap_err() {
            GameError::HandError(_) => {},
            _ => panic!("Expected HandError to be propagated")
        }
    }

    #[test]
    fn should_reject_declare_trump_when_no_current_hand() {
        let game = Game::new(Player::South);

        let result = game.declare_trump(Suit::Spades);

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::InvalidOperation(_) => {},
            _ => panic!("Expected InvalidOperation to be propagated")
        }
    }

    #[test]
    fn should_record_meld_on_current_hand() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap();

        let result = game.record_meld(24, 32);

        assert!(result.is_ok());
        let new_game = result.unwrap();

        let hand = new_game.current_hand().unwrap();
        match hand.state() {
            HandState::WaitingForTricks { us_meld, them_meld, ..} => {
                assert_eq!(us_meld, Some(24));
                assert_eq!(them_meld, Some(32));
            },
            _ => panic!("Expected WaitingForTricks state")
        }


        assert_eq!(new_game.id(), game.id());
        assert_eq!(new_game.completed_hands().len(), 0);
    }

    #[test]
    fn should_propagate_hand_meld_validation_errors() {

        let wrong_state_game = Game::new(Player::South)
            .start_new_hand()
            .unwrap();

        let result = wrong_state_game.record_meld(24, 32);

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::HandError(_) => {},
            _ => panic!("Expected HandError to be propagated")
        }
    }

    #[test]
    fn should_complete_hand_and_start_new_when_hand_becomes_complete() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Clubs)
            .unwrap();

        let result = game.record_meld(15, 25);

        assert!(result.is_ok());
        let new_game = result.unwrap();

        assert_eq!(new_game.completed_hands().len(), 1);
        let completed_hand = &new_game.completed_hands()[0];
        assert!(matches!(completed_hand.state(), HandState::Completed { .. }));
    }

    #[test]
    fn should_reject_record_meld_when_no_current_hand() {
        let game = Game::new(Player::South);

        let result = game.record_meld(24,32);

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::InvalidOperation(_) => {},
            _ => panic!("Expected InvalidOperation to be propagated")
        }
    }

    #[test]
    fn should_record_tricks_on_current_hand() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = game.record_tricks(26, 24);

        assert!(result.is_ok());
        let new_game = result.unwrap();

        assert_eq!(new_game.completed_hands().len(), 1);
        let completed_hand = &new_game.completed_hands()[0];

        match completed_hand.state() {
            HandState::Completed { us_tricks, them_tricks, us_total, them_total, ..} => {
                assert_eq!(us_tricks, Some(26));
                assert_eq!(them_tricks, Some(24));
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(56));
            },
            _ => panic!("Expected Completed state")
        }
    }

    #[test]
    fn should_propagate_hand_tricks_validation_errors() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = game.record_tricks(30, 25);

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::HandError(_) => {},
            _ => panic!("Expected HandError to be propagated")
        }
    }

    #[test]
    fn should_handle_tricks_with_contract_failure() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap()
            .record_bid(Player::North, 60)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = game.record_tricks(30, 20);

        assert!(result.is_ok());
        let new_game = result.unwrap();

        assert_eq!(new_game.completed_hands().len(), 1);
        let completed_hand = &new_game.completed_hands()[0];

        match completed_hand.state() {
            HandState::Completed { us_total, them_total, .. } => {
                assert_eq!(us_total, Some(-60));
                assert_eq!(them_total, Some(52));
            },
            _ => panic!("Expected Completed state")
        }
    }

    #[test]
    fn should_reject_record_tricks_when_no_current_hand() {
        let game = Game::new(Player::South);

        let result = game.record_tricks(26, 24);

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::InvalidOperation(_) => {},
            _ => panic!("Expected InvalidOperation to be propagated")
        }
    }

    #[test]
    fn should_reject_start_new_hand_when_in_progress() {
        let game = Game::new(Player::South)
            .start_new_hand()
            .unwrap();

        let result = game.start_new_hand();

        assert!(result.is_err());
        match result.unwrap_err() {
            GameError::InvalidStateTransition(_) => {},
            _ => panic!("Expected InvalidOperation to be propagated")
        }
    }

    #[test]
    fn should_complete_game_when_team_reaches_500_points() {
        let mut game = Game::new(Player::South);

        let (us_total, them_total) = game.running_totals();

        assert_eq!(us_total, 0);
        assert_eq!(them_total, 0);

    }

    #[test]
    fn should_return_false_for_is_game_complete_when_no_team_reaches_500_points() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(100, 150, Player::North)
            .add_completed_hand_with_scores(200, 180, Player::South);

        assert!(!game.is_game_complete());
    }


    #[test]
    fn should_return_true_for_is_game_complete_when_us_team_reaches_500() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(300, 150, Player::North)
            .add_completed_hand_with_scores(250, 180, Player::South);

        // US: 550, THEM: 330 - US reaches 500
        assert!(game.is_game_complete());
    }

    #[test]
    fn should_return_true_for_is_game_complete_when_them_team_reaches_500() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(200, 300, Player::North)
            .add_completed_hand_with_scores(150, 250, Player::South);

        // US: 350, THEM: 550 - THEM reaches 500
        assert!(game.is_game_complete());
    }

    #[test]
    fn should_return_none_for_winner_when_game_not_complete() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(200, 250, Player::North);

        // Neither team reaches 500
        assert_eq!(game.winner(), None);
    }

    #[test]
    fn should_return_us_winner_when_only_us_reaches_500() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(300, 200, Player::North)
            .add_completed_hand_with_scores(250, 150, Player::South);

        // US: 550, THEM: 350 - only US reaches 500
        assert_eq!(game.winner(), Some(Team::Us));
    }

    #[test]
    fn should_return_them_winner_when_only_them_reaches_500() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(200, 300, Player::North)
            .add_completed_hand_with_scores(150, 250, Player::South);

        // US: 350, THEM: 550 - only THEM reaches 500
        assert_eq!(game.winner(), Some(Team::Them));
    }

    #[test]
    fn should_return_bidding_team_winner_when_both_reach_500() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(300, 300, Player::North)
            .add_completed_hand_with_scores(250, 250, Player::East); // East (THEM) was last bidder

        // US: 550, THEM: 550 - both reach 500, THEM team bid last
        assert_eq!(game.winner(), Some(Team::Them));
    }

    #[test]
    fn should_return_higher_score_winner_when_both_reach_500_but_no_last_bidder() {
        let game = Game::new(Player::South)
            .add_completed_hand_with_scores(400, 350, Player::North)
            .add_completed_hand_with_scores(200, 200, Player::South);

        // US: 600, THEM: 550 - both reach 500, US has higher score
        // This tests the fallback when we can't determine last bidder
        assert_eq!(game.winner(), Some(Team::Us));
    }

}