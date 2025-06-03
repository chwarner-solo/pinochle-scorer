use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Us,
    Them,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    NoMarriage
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    North,
    South,
    East,
    West,
}

impl Player {
    pub fn team(self) -> Team {
        match self {
            Player::North | Player::South => Team::Us,
            Player::East | Player::West => Team::Them,
        }
    }

    pub fn next_clockwise(self) -> Player {
        match self {
            Player::South => Player::West,
            Player::West => Player::North,
            Player::North => Player::East,
            Player::East => Player::South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    WaitingToStart,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandState {
    WaitingForBid,
    WaitingForTrump {
        bidder: Player,
        bid_amount: u32
    },
    NoMarriage {
        bidder: Player,
        bid_amount: u32
    },
    WaitingForMeld {
        bidder: Player,
        bid_amount: u32,
        trump: Suit
    },
    WaitingForTricks {
        bidder: Player,
        bid_amount: u32,
        trump: Suit,
        us_meld: Option<u32>,
        them_meld: Option<u32>
    },
    Completed {
        bidder: Player,
        bid_amount: u32,
        trump: Suit,
        us_meld: Option<u32>,
        them_meld: Option<u32>,
        us_tricks: Option<u32>,
        them_tricks: Option<u32>,
        us_total: Option<i32>,
        them_total: Option<i32>
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameId(pub Uuid);

impl GameId {
    pub fn new() -> GameId {
        GameId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandId(pub Uuid);

impl HandId {
    pub fn new() -> HandId {
        HandId(Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_next_clockwise() {
        assert_eq!(Player::North.next_clockwise(), Player::East);
        assert_eq!(Player::East.next_clockwise(), Player::South);
        assert_eq!(Player::South.next_clockwise(), Player::West);
        assert_eq!(Player::West.next_clockwise(), Player::North);
    }

    #[test]
    fn player_should_return_correct_team() {
        assert_eq!(Player::North.team(), Team::Us);
        assert_eq!(Player::South.team(), Team::Us);
        assert_eq!(Player::East.team(), Team::Them);
        assert_eq!(Player::West.team(), Team::Them);
    }
}