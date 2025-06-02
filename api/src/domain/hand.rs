use crate::domain::{HandId, HandState, Player, Suit, Team};

#[derive(Debug, Clone)]
pub struct Hand {
    id: HandId,
    dealer: Player,
    state: HandState,
    bidder: Option<Player>,
    bid_amount: Option<u32>,
}

impl Hand {
    pub fn new(dealer: Player) -> Self {
        Self {
            id: HandId::new(),
            dealer,
            state: HandState::WaitingForBid,
            bidder: None,
            bid_amount: None
        }
    }

    pub fn id(&self) -> HandId {
        self.id
    }

    pub fn dealer(&self) -> Player {
        self.dealer
    }

    pub fn state(&self) -> HandState {
        self.state
    }

    pub fn bidder(&self) -> Option<Player> {
        match self.state {
            HandState::WaitingForTrump { bidder, .. } => Some(bidder),
            HandState::NoMarriage { bidder, .. } => Some(bidder),
            HandState::WaitingForMeld { bidder, .. } => Some(bidder),
            HandState::WaitingForTricks { bidder, .. } => Some(bidder),
            _ => None
        }
    }

    pub fn bid_amount(&self) -> Option<u32> {
        match self.state {
            HandState::WaitingForTrump { bid_amount, .. } => Some(bid_amount),
            HandState::NoMarriage { bid_amount, .. } => Some(bid_amount),
            HandState::WaitingForMeld { bid_amount, .. } => Some(bid_amount),
            HandState::WaitingForTricks { bid_amount, .. } => Some(bid_amount),
            _ => None
        }
    }

    pub fn trump(&self) -> Option<Suit> {
        match self.state {
            HandState::WaitingForMeld { trump, .. } => Some(trump),
            HandState::WaitingForTricks { trump, .. } => Some(trump),
            HandState::Completed { trump, .. } => Some(trump),
            _ => None
        }
    }

    pub fn us_total(&self) -> i32 {
        match self.state {
            HandState::Completed { us_total, .. } => us_total.unwrap_or(0),
            _ => 0
        }
    }

    pub fn them_total(&self) -> i32 {
        match self.state {
            HandState::Completed { them_total, .. } => them_total.unwrap_or(0),
            _ => 0
        }
    }

    pub fn place_bid(&self, bidder: Player, bid_amount: u32) -> Result<Self, HandError> {
        if self.state != HandState::WaitingForBid {
            return Err(HandError::InvalidStateTransition("Hand is not waiting for bid".to_string()));
        }

        if !Hand::validate_bid_increment(bid_amount) {
            return Err(HandError::InvalidBid(
                format!("Invalid bid amount: {}. Must follow increment rules", bid_amount)
            ));
        }

        Ok(Self {
            bidder: Some(bidder),
            bid_amount: Some(bid_amount),
            state: HandState::WaitingForTrump{
                bidder,
                bid_amount
            },
            ..self.clone()
        })
    }

    pub fn declare_trump(&self, trump: Suit) -> Result<Self, HandError> {
        match self.state {
            HandState::WaitingForTrump { bidder: current_bidder, bid_amount } => {
                if trump != Suit::NoMarriage {
                    Ok(Self {
                        state: HandState::WaitingForMeld {
                            bidder: current_bidder,
                            bid_amount,
                            trump
                        },
                        ..self.clone()
                    })
                } else {
                    Ok(Self {
                        state: HandState::NoMarriage {
                            bidder: current_bidder,
                            bid_amount
                        },
                        ..self.clone()
                    })
                }
            },
            _ => Err(HandError::InvalidStateTransition("Hand is not waiting for trump".to_string()))
        }
    }

    pub fn record_meld(self, us: u32, them: u32) -> Result<Self, HandError> {
        let us_meld = Hand::validate_points(us);
        let them_meld = Hand::validate_points(them);


        match self.state {
            HandState::WaitingForMeld { bidder: current_bidder, bid_amount, trump } => {

                let bidding_team = current_bidder.team();
                let bidding_team_meld = match bidding_team {
                    Team::Us => us_meld,
                    Team::Them => them_meld
                };

                if bidding_team_meld.is_none() {
                    let (us_score, them_score) = match bidding_team {
                        Team::Us => (-(bid_amount as i32), them_meld.unwrap_or(0) as i32),
                        Team::Them => (us_meld.unwrap_or(0) as i32, -(bid_amount as i32))
                    };

                    Ok(Self {
                        state: HandState::Completed {
                            bidder: current_bidder,
                            bid_amount,
                            trump,
                            us_meld: us_meld,
                            them_meld: them_meld,
                            us_tricks: None,
                            them_tricks: None,
                            us_total: Some(us_score),
                            them_total:Some(them_score)
                        },
                        ..self.clone()
                    })
                } else {
                    Ok(Self {
                        state: HandState::WaitingForTricks {
                            bidder: current_bidder,
                            bid_amount,
                            trump,
                            us_meld,
                            them_meld
                        },
                        ..self.clone()
                    })
                }
            },
            HandState::NoMarriage { bidder: current_bidder, bid_amount } => {
                Ok(Self {
                    state: HandState::Completed {
                        bidder: current_bidder,
                        bid_amount,
                        trump: Suit::NoMarriage,
                        us_meld: if current_bidder.team() == Team::Us { None } else { us_meld },
                        them_meld: if current_bidder.team() == Team::Them { None } else { them_meld },
                        us_tricks: None,
                        them_tricks: None,
                        us_total: Some(-(bid_amount as i32)),
                        them_total: Some(them_meld.unwrap_or(0) as i32)
                    },
                    ..self.clone()
                })
            },
            _ => Err(HandError::InvalidStateTransition("Hand is not waiting for meld".to_string()))
        }
    }

    pub fn record_tricks(self, us: u32, them: u32) -> Result<Self, HandError> {

        if (us + them) != 50 {
            return Err(HandError::InvalidTricks(us, them));
        }

        let us_tricks = Hand::validate_points(us);
        let them_tricks = Hand::validate_points(them);

        match self.state {
            HandState::WaitingForTricks { bidder: current_bidder, bid_amount, trump, us_meld, them_meld } => {

                let mut us_total = if us_tricks.is_none() { 0 } else { us_meld.unwrap_or(0) as i32 + us_tricks.unwrap_or(0) as i32 };
                let mut them_total = if them_tricks.is_none() {0 } else { them_meld.unwrap_or(0) as i32 + them_tricks.unwrap_or(0) as i32};

                let bidding_team = current_bidder.team();
                let bidding_meld = match bidding_team {
                    Team::Us => us_meld.unwrap_or(0),
                    Team::Them => them_meld.unwrap_or(0)
                };
                let required_tricks = std::cmp::max(bid_amount - bidding_meld, 20);
                let bidding_tricks = match bidding_team {
                    Team::Us => us_tricks.unwrap_or(0),
                    Team::Them => them_tricks.unwrap_or(0)
                };

                if bidding_tricks < required_tricks {
                    match bidding_team {
                        Team::Us => us_total = -(bid_amount as i32),
                        Team::Them => them_total = -(bid_amount as i32)
                    }
                }

                Ok(Self {
                    state: HandState::Completed {
                        bidder: current_bidder,
                        bid_amount,
                        trump,
                        us_meld,
                        them_meld,
                        us_tricks: us_tricks,
                        them_tricks: them_tricks,
                        us_total: if (us_total == 0) { None } else { Some(us_total) },
                        them_total: if (them_total == 0) { None } else {  Some(them_total) }
                    },
                    ..self.clone()
                })
            },
            _ => Err(HandError::InvalidStateTransition("Hand is not waiting for tricks".to_string()))
        }
    }


    fn validate_points(meld: u32) -> Option<u32> {
        if meld < 20 {
            None
        } else {
            Some(meld)
        }
    }

    fn validate_bid_increment(amount: u32) -> bool {
        if amount < 50 {
            return false;
        } else if amount < 60 {
            return true;
        } else if amount < 100 {
            return amount % 5 == 0;
        } else {
            return amount % 10 == 0;
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HandError {
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Invalid bid: {0}")]
    InvalidBid(String),

    #[error("Total tricks must not exceed 50: {0} + {1}")]
    InvalidTricks(u32, u32)
}

#[cfg(test)]

impl Hand {
    pub fn set_state(&self, state: HandState) -> Self {
        Self {
            state,
            ..self.clone()
        }
    }

    pub fn get_validate_points(meld: u32) -> Option<u32> {
        Hand::validate_points(meld)
    }
}
mod tests {
    use super::*;

    #[test]
    fn new_hand_should_start_waiting_for_bid() {
        let hand = Hand::new(Player::South);

        assert_eq!(hand.dealer, Player::South);
        assert_eq!(hand.state, HandState::WaitingForBid);
    }

    #[test]
    fn new_hand_should_have_unique_id() {
        let hand1 = Hand::new(Player::South);
        let hand2 = Hand::new(Player::South);

        assert_ne!(hand1.id, hand2.id);
    }

    #[test]
    fn should_accept_valid_bid() {
        let hand = Hand::new(Player::South);
        let id = hand.id;
        let result = hand.place_bid(Player::North, 51);

        assert!(result.is_ok());

        let hand = result.unwrap();

        let state = match hand.state {
            HandState::WaitingForTrump { bidder, bid_amount } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
            },
            _ => panic!("Expected WaitingForTrump state")
        };
        assert_eq!(hand.id, id);
        assert_eq!(hand.bidder(), Some(Player::North));
        assert_eq!(hand.bid_amount(), Some(51));
    }

    #[test]
    fn should_reject_bid_below_minimum() {
        let mut hand = Hand::new(Player::South);

        let result = hand.place_bid(Player::North, 49);

        assert!(result.is_err());
    }

    #[test]
    fn should_accept_valid_bid_increments_below_sixty() {
        let hand = Hand::new(Player::South);

        assert!(hand.place_bid(Player::North, 50).is_ok());
        assert!(hand.place_bid(Player::North, 51).is_ok());
        assert!(hand.place_bid(Player::North, 52).is_ok());
        assert!(hand.place_bid(Player::North, 59).is_ok());
    }

    #[test]
    fn should_accept_valid_bid_increments_above_sixty_to_ninety() {
        let hand = Hand::new(Player::South);

        assert!(hand.place_bid(Player::North, 60).is_ok());
        assert!(hand.place_bid(Player::North, 65).is_ok());
        assert!(hand.place_bid(Player::North, 70).is_ok());
        assert!(hand.place_bid(Player::North, 95).is_ok());
    }

    #[test]
    fn should_accept_valid_bid_increments_hundred_plus() {
        let hand = Hand::new(Player::South);

        assert!(hand.place_bid(Player::North, 100).is_ok());
        assert!(hand.place_bid(Player::North, 110).is_ok());
        assert!(hand.place_bid(Player::North, 120).is_ok());
        assert!(hand.place_bid(Player::North, 200).is_ok());
    }

    #[test]
    fn should_reject_invalid_bid_increments() {
        let hand = Hand::new(Player::South);

        assert!(hand.place_bid(Player::North, 61).is_err());
        assert!(hand.place_bid(Player::North, 63).is_err());
        assert!(hand.place_bid(Player::North, 77).is_err());

        assert!(hand.place_bid(Player::North, 105).is_err());
        assert!(hand.place_bid(Player::North, 121).is_err());
        assert!(hand.place_bid(Player::North, 151).is_err());
    }

    #[test]
    fn should_accept_minimum_bid() {
        let hand = Hand::new(Player::South);
        let id = hand.id;
        let result = hand.place_bid(Player::North, 50);

        assert!(result.is_ok());
        let hand = result.unwrap();

        match hand.state() {
            HandState::WaitingForTrump { bidder, bid_amount } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 50);
            },
            _ => panic!("Expected WaitingForTrump state")
        }
        assert_eq!(hand.id, id);
        assert_eq!(hand.bidder(), Some(Player::North));
        assert_eq!(hand.bid_amount(), Some(50));
    }

    #[test]
    fn should_reject_bid_when_not_waiting_for_bid() {
        let mut hand = Hand::new(Player::South);
        hand.state = HandState::WaitingForTrump {
            bidder: Player::North,
            bid_amount: 51
        };

        let result = hand.place_bid(Player::North, 51);

        assert!(result.is_err());

    }

    #[test]
    fn should_accept_valid_trump_declaration_with_marriage() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
        ;

        let result = hand.declare_trump(Suit::Spades);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::WaitingForMeld { bidder, bid_amount, trump } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
            },
            _ => panic!("Expected NoMarriage state")
        }
    }

    #[test]
    fn should_transition_to_no_marriage_when_bidder_lacks_marriage() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
        ;

        let result = hand.declare_trump(Suit::NoMarriage);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::NoMarriage { bidder, bid_amount } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
            },
            _ => panic!("Expected NoMarriage state")
        }

        assert_eq!(new_hand.trump(), None);
    }

    #[test]
    fn should_reject_trump_declaration_when_not_waiting_for_trump() {
        let mut hand = Hand::new(Player::South);
        hand.state = HandState::WaitingForMeld {
            bidder: Player::North,
            bid_amount: 51,
            trump: Suit::Spades
        };

        let result = hand.declare_trump(Suit::Spades);

        assert!(result.is_err());
    }

    #[test]
    fn should_preserve_hand_id_after_trump_declaration() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
        ;

        let result = hand.declare_trump(Suit::Spades);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        assert_eq!(new_hand.id, hand.id);
        assert_eq!(new_hand.dealer, hand.dealer);
    }

    #[test]
    fn should_accept_valid_meld_from_both_teams() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
        ;

        let result = hand.record_meld(24, 32);

        assert!(result.is_ok());

        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::WaitingForTricks {bidder, bid_amount, trump, us_meld, them_meld} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, Some(24));
                assert_eq!(them_meld, Some(32));
            },
            _ => panic!("Expected WaitingForTricks state")
        }
    }

    #[test]
    fn should_forfeit_team_meld_when_below_twenty() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
        ;

        let result = hand.record_meld(19, 32);

        assert!(result.is_ok());

        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed {bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, None);
                assert_eq!(them_meld, Some(32));
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(32));
            },
            _ => panic!("Expected WaitingForTricks state")
        }
    }

    #[test]
    fn should_set_bidding_team_when_meld_less_then_twenty() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            ;

        let result = hand.record_meld(19, 32);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed { bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, None);
                assert_eq!(them_meld, Some(32));
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(32));
            }
            _ => panic!("Expected Complete state")
        }
    }

    #[test]
    fn should_set_bidding_team_and_score_meld_when_meld_greater_then_twenty_for_non_bidding_team() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::NoMarriage)
            .unwrap()
            ;

        let result = hand.record_meld(0, 32);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed { bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::NoMarriage);
                assert_eq!(us_meld, None);
                assert_eq!(them_meld, Some(32));
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(32));
            }
            _ => panic!("Expected Complete state")
        }
    }
    #[test]
    fn validate_meld_should_return_none_for_values_below_twenty() {
        let hand = Hand::new(Player::South);

        assert_eq!(Hand::get_validate_points(19), None);
        assert_eq!(Hand::get_validate_points(0), None);
        assert_eq!(Hand::get_validate_points(15), None);
    }

    #[test]
    fn validate_meld_should_return_some_for_values_twenty_and_above() {
        let hand = Hand::new(Player::South);

        assert_eq!(Hand::get_validate_points(20), Some(20));
        assert_eq!(Hand::get_validate_points(25), Some(25));
        assert_eq!(Hand::get_validate_points(50), Some(50));
    }

    #[test]
    fn should_record_valid_tricks_from_both_teams() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = hand.record_tricks(27,23);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed { bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total } => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, Some(24));
                assert_eq!(them_meld, Some(32));
                assert_eq!(us_tricks, Some(27));
                assert_eq!(them_tricks, Some(23));
                assert_eq!(us_total, Some(51));
                assert_eq!(them_total, Some(55));
            }
            _ => panic!("Expected Complete state")
        }
    }

    #[test]
    fn should_error_when_tricks_total_is_invalid() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = hand.record_tricks(26,25);

        assert!(result.is_err());
    }

    #[test]
    fn should_forfeit_non_bidding_team_meld_when_tricks_below_twenty() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = hand.record_tricks(36,14);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed { us_tricks, them_tricks, us_total, them_total, .. } => {
                assert_eq!(us_tricks, Some(36));
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(60));
                assert_eq!(them_total, None);
            },
            _ => panic!("Expected Completed state")
        }
    }

    #[test]
    fn should_set_biding_team_when_contract_not_made() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 60)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap();

        let result = hand.record_tricks(30,20);

        assert!(result.is_ok());
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed { us_tricks, them_tricks, us_total, them_total, .. } => {
                assert_eq!(us_tricks, Some(30));
                assert_eq!(them_tricks, Some(20));
                assert_eq!(us_total, Some(-60i32));
                assert_eq!(them_total, Some(52i32));
            }
            _ => panic!("Expected Complete state")
        }
        ;
    }

    #[test]
    fn us_total_should_return_zero_for_non_completed_states() {
        let hand = Hand::new(Player::South);
        assert_eq!(hand.us_total(), 0);

        let hand = hand.place_bid(Player::North, 51).unwrap();
        assert_eq!(hand.us_total(), 0);

        let hand = hand.declare_trump(Suit::Spades).unwrap();
        assert_eq!(hand.us_total(), 0);

        let hand = hand.record_meld(24, 32).unwrap();
        assert_eq!(hand.us_total(), 0);
    }

    #[test]
    fn them_total_should_return_zero_for_non_completed_states() {
        let hand = Hand::new(Player::South);
        assert_eq!(hand.them_total(), 0);

        let hand = hand.place_bid(Player::North, 51).unwrap();
        assert_eq!(hand.them_total(), 0);

        let hand = hand.declare_trump(Suit::Spades).unwrap();
        assert_eq!(hand.them_total(), 0);

        let hand = hand.record_meld(24, 32).unwrap();
        assert_eq!(hand.them_total(), 0);
    }

    #[test]
    fn totals_should_return_correct_values_for_completed_state() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::West, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap()
            .record_meld(24, 32)
            .unwrap()
            .record_tricks(26, 24)
            .unwrap();

        assert_eq!(hand.us_total(), 50);   // 24 meld + 26 tricks
        assert_eq!(hand.them_total(), 56); // 32 meld + 24 tricks
    }
}