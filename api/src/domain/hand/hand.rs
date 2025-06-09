use crate::domain::{HandError, HandId, HandState, Player, Suit, Team};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    
    pub fn with_state(mut self, state: HandState) -> Self {
        self.state = state;
        self
    }

    pub fn bidder(&self) -> Option<Player> {
        match self.state {
            HandState::WaitingForTrump { bidder, .. } => {
                tracing::info!("Waiting for Trump, Bidder: {:?} ", bidder);
                Some(bidder)
            },
            HandState::NoMarriage { bidder, .. } => {
                tracing::info!("No Marriage,Bidder: {:?} ", bidder);
                Some(bidder)
            },
            HandState::WaitingForMeld { bidder, .. } => {
                tracing::info!("Waiting for Meld, Bidder:  {:?} ", bidder);
                Some(bidder)
            },
            HandState::WaitingForTricks { bidder, .. } => {
                tracing::info!("Waiting for Tricks, Bidder:  {:?} ", bidder);
                Some(bidder)
            },
            HandState::Completed { bidder, .. } => {
                tracing::info!("Completed, Bidder:  {:?} ", bidder);
                Some(bidder)
            },
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
    
    pub fn us_meld(&self) -> Option<u32> {
        match self.state {
            HandState::Completed { us_meld, .. } => us_meld,
            HandState::WaitingForTricks { us_meld, .. } => us_meld,
            _ => None
        }
    }
    
    pub fn them_meld(&self) -> Option<u32> {
        match self.state {
            HandState::Completed { them_meld, .. } => them_meld,
            HandState::WaitingForTricks { them_meld, .. } => them_meld,
            _ => None
        }
    }

    pub fn us_tricks(&self) -> Option<u32> {
        match self.state {
            HandState::Completed { us_tricks, .. } => us_tricks,
            _ => None
        }
    }

    pub fn them_tricks(&self) -> Option<u32> {
        match self.state {
            HandState::Completed { them_tricks, .. } => them_tricks,
            _ => None
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
                let bidding_team_meld = Self::team_meld(bidding_team, us_meld, them_meld);

                if bidding_team_meld.is_none() {
                    let (us_score, them_score) = Self::failed_meld_scores(bidding_team, bid_amount, us_meld, them_meld);
                    return Ok(Self {
                        state: HandState::Completed {
                            bidder: current_bidder,
                            bid_amount,
                            trump,
                            us_meld,
                            them_meld,
                            us_tricks: None,
                            them_tricks: None,
                            us_total: Some(us_score),
                            them_total: Some(them_score),
                        },
                        ..self.clone()
                    });
                }

                Ok(Self {
                    state: HandState::WaitingForTricks {
                        bidder: current_bidder,
                        bid_amount,
                        trump,
                        us_meld,
                        them_meld,
                    },
                    ..self.clone()
                })
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
                        them_total: Some(them_meld.unwrap_or(0) as i32),
                    },
                    ..self.clone()
                })
            },
            _ => Err(HandError::InvalidStateTransition("Hand is not waiting for meld".to_string())),
        }
    }

    fn team_meld(team: Team, us_meld: Option<u32>, them_meld: Option<u32>) -> Option<u32> {
        match team {
            Team::Us => us_meld,
            Team::Them => them_meld,
        }
    }

    fn failed_meld_scores(team: Team, bid_amount: u32, us_meld: Option<u32>, them_meld: Option<u32>) -> (i32, i32) {
        match team {
            Team::Us => (-(bid_amount as i32), them_meld.unwrap_or(0) as i32),
            Team::Them => (us_meld.unwrap_or(0) as i32, -(bid_amount as i32)),
        }
    }

    pub fn record_tricks(self, us: u32, them: u32) -> Result<Self, HandError> {
        tracing::info!("Validating tricks");

        // If both are zero, fail
        if us == 0 && them == 0 {
            return Err(HandError::InvalidTricks(us, them));
        }

        let mut final_us = us;
        let mut final_them = them;

        // If either is zero, infer the other
        if us == 0 {
            final_us = 50 - them;
        }
        if them == 0 {
            final_them = 50 - us;
        }

        // Must sum to 50
        if final_us + final_them != 50 {
            return Err(HandError::InvalidTricks(final_us, final_them));
        }

        tracing::info!("Validated tricks {0} {1} {2}", final_us, final_them, 50);
        tracing::info!("Validating hand state: {:?}", self.state);
        let HandState::WaitingForTricks { bidder, bid_amount, trump, us_meld, them_meld } = self.state else {
            return Err(HandError::InvalidStateTransition("Hand is not waiting for tricks".to_string()));
        };
        tracing::info!("Validated hand state");
        tracing::info!("Identifying bidding team");
        let bidding_team = bidder.team();
        tracing::info!("Calculating required tricks");
        let required_tricks = Self::required_tricks(bid_amount, us_meld, them_meld, bidding_team);
        tracing::info!("Identifying tricks for bidding team");
        let bidder_tricks = match bidding_team {
            Team::Us => final_us,
            Team::Them => final_them
        };
        tracing::info!("Calculating team totals");
        let us_total = Self::calculate_team_total(us_meld.unwrap_or(0), final_us);
        let them_total = Self::calculate_team_total(them_meld.unwrap_or(0), final_them);
        tracing::info!("Applying bidding penalty");
        let (us_total, them_total) = Self::apply_bidding_penalty(
            bidding_team,
            us_total,
            them_total,
            bid_amount,
            bidder_tricks,
            required_tricks);
        tracing::info!("Normalizing melds");
        let us_meld = Self::normalize_meld(us_meld);
        let them_meld = Self::normalize_meld(them_meld);
        tracing::info!("Normalizing tricks");
        let us_tricks = Some(final_us);
        let them_tricks = Some(final_them);
        tracing::info!("Normalizing totals");
        let us_total = Self::normalize_total(us_total);
        let them_total = Self::normalize_total(them_total);
        tracing::info!("Returning completed hand");
        Ok(Self {
            state: HandState::Completed {
                bidder,
                bid_amount,
                trump,
                us_meld,
                them_meld,
                us_tricks,
                them_tricks,
                us_total,
                them_total
            },
            ..self.clone()
        })
    }
    
    pub fn tricks_to_save(&self) -> Option<u32> {
        if self.bid_amount().is_none() {
            return None;
        }
        let them_meld = self.state.them_meld().unwrap_or(0);
        let us_meld = self.state.us_meld().unwrap_or(0);
        let bid_amount = self.bid_amount.unwrap_or(0);
        
        Some(Hand::required_tricks(bid_amount, Some(us_meld), Some(them_meld), self.bidder?.team()))
    }

    fn required_tricks(bid_amount: u32, us_meld: Option<u32>, them_meld: Option<u32>, bidding_team: Team) -> u32 {
        match bidding_team {
            Team::Us => (std::cmp::max((bid_amount as i32) - (us_meld.unwrap_or(0) as i32), 20)) as u32,
            Team::Them => (std::cmp::max((bid_amount as i32) - (them_meld.unwrap_or(0) as i32), 20)) as u32
        } 
    }

    fn normalize_meld(meld: Option<u32>) ->  Option<u32> {
        match meld {
            Some(m) if m < 20 => None,
            other => other,
        }
    }
    
    fn normalize_total(total: i32) -> Option<i32 >{
        if total == 0 { None } else { Some(total)}
    }

    fn calculate_team_total(meld: u32, tricks: u32) -> i32 {
        if tricks < 20 {
            0
        } else if meld < 20 {
            tricks as i32
        } else {
            (meld + tricks) as i32
        }
    }
    
    fn apply_bidding_penalty(
        bidding_team: Team, 
        us_total: i32, them_total: i32, 
        bid_amount: u32, bidding_tricks: u32, 
        required_tricks: u32) -> (i32, i32) 
    {
        if bidding_tricks < required_tricks {
            match bidding_team {
                Team::Us => (-(bid_amount as i32), them_total),
                Team::Them => (us_total, -(bid_amount as i32))
            }
        } else {
            (us_total, them_total)
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
            HandState::Completed {bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::NoMarriage);
                assert_eq!(us_meld, None);
                assert_eq!(them_meld, Some(32));
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(32));
            },
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
            HandState::Completed {bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, Some(24));
                assert_eq!(them_meld, Some(32));
                assert_eq!(us_tricks, Some(27));
                assert_eq!(them_tricks, Some(23));
                assert_eq!(us_total, Some(51));
                assert_eq!(them_total, Some(55));
            },
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
            HandState::Completed {us_tricks, them_tricks, us_total, them_total, ..} => {
                assert_eq!(us_tricks, Some(36));
                assert_eq!(them_tricks, Some(14));
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
            HandState::Completed {us_tricks, them_tricks, us_total, them_total, ..} => {
                assert_eq!(us_tricks, Some(30));
                assert_eq!(them_tricks, Some(20));
                assert_eq!(us_total, Some(-60i32));
                assert_eq!(them_total, Some(52i32));
            },
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

    #[test]
    fn should_allow_zero_meld_for_both_teams() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap();

        let result = hand.record_meld(0, 0);
        assert!(result.is_ok(), "Expected Ok for zero melds, got: {:?}", result);
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed {bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, None, "us_meld should be None for zero");
                assert_eq!(them_meld, None, "them_meld should be None for zero");
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                // Forfeiting meld means negative bid for bidding team, zero for other
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(0));
            },
            _ => panic!("Expected Completed state")
        }
    }

    #[test]
    fn should_proceed_to_tricks_if_one_team_has_meld_20_or_more() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap();
        let result = hand.record_meld(0, 24);
        assert!(result.is_ok(), "Expected Ok for melds 0,24");
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed {bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, None);
                assert_eq!(them_meld, Some(24));
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(24));
            },
            _ => panic!("Expected Completed state")
        }
    }

    #[test]
    fn should_forfeit_hand_if_contract_team_meld_is_below_20() {
        let hand = Hand::new(Player::South)
            .place_bid(Player::North, 51)
            .unwrap()
            .declare_trump(Suit::Spades)
            .unwrap();
        let result = hand.record_meld(0, 24);
        assert!(result.is_ok(), "Expected Ok for melds 0,24");
        let new_hand = result.unwrap();

        match new_hand.state() {
            HandState::Completed {bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total} => {
                assert_eq!(bidder, Player::North);
                assert_eq!(bid_amount, 51);
                assert_eq!(trump, Suit::Spades);
                assert_eq!(us_meld, None);
                assert_eq!(them_meld, Some(24));
                assert_eq!(us_tricks, None);
                assert_eq!(them_tricks, None);
                assert_eq!(us_total, Some(-51));
                assert_eq!(them_total, Some(24));
            },
            _ => panic!("Expected Completed state")
        }
    }
}