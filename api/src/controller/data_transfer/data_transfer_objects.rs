use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::RunningTotal;
use crate::domain::{Game, Hand, Player, Suit};

#[derive(Debug, Clone, Deserialize)]
pub struct StartNewGameRequest {
    pub dealer: Player,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartNewGameResponse {
    game_id: Uuid,
    dealer: Player,
    state: String
}

impl From<&Game> for StartNewGameResponse {
    fn from(game: &Game) -> Self {
        Self {
            game_id: game.id().0,
            dealer: game.current_dealer(),
            state: game.state().to_string()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartNewHandRequest {
    pub(crate) game_id: Uuid
}

#[derive(Debug, Clone, Serialize)]
pub struct StartNewHandResponse {
    game_id: Uuid,
    dealer: Player,
    hand: Option<HandResponse>,
    state: String
}

impl From<&Game> for StartNewHandResponse {
    fn from(game: &Game) -> Self {

        let Game {id,  .. } = game;
        let current_hand = game.current_hand();
        let dealer = game.current_dealer();

        Self {
            game_id: game.id().0,
            dealer,
            hand: Option::from(HandResponse::from(current_hand.as_ref())),
            state: game.state().to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CompletedHandsResponse {
    hand: Vec<HandResponse>,
}

impl From<Vec<Hand>> for CompletedHandsResponse {
    fn from(value: Vec<Hand>) -> Self {
        Self {
            hand: value.iter()
                .map(HandResponse::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunningTotalResponse {
    us_total: i32,
    them_total: i32,
}

impl From<&RunningTotal> for RunningTotalResponse {
    fn from(value: &RunningTotal) -> Self {
        Self {
            us_total: value.us,
            them_total: value.them
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordBidRequest {
    pub player: Player,
    pub bid: u32
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordBidResponse {
    pub game_id: Uuid,
    pub game_state: String,
    pub bidder: Option<Player>,
    pub bid_amount: Option<u32>,
    pub hand_state: String
}

impl From<&Game> for RecordBidResponse {
    fn from(game: &Game) -> Self {
        if let Some(current_hand) = game.current_hand() {
            tracing::info!("current_hand: {:?}", current_hand);
            if let Some(bidder) = current_hand.bidder() {
                tracing::info!("bidder: {:?}", bidder );
            } else {
                tracing::info!("no bidder on current hand");
            }
        }
        Self {
            game_id: game.id().0,
            game_state: game.state().to_string(),
            bidder: game.current_hand().and_then(|h| h.bidder()),
            bid_amount: game.current_hand().and_then(|h| h.bid_amount()),
            hand_state: game.current_hand().unwrap().state().to_string()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclareTrumpRequest {
    pub trump: Suit
}

#[derive(Debug, Clone, Serialize)]
pub struct DeclareTrumpResponse {
    pub game_id: Uuid,
    pub game_state: String,
    pub bidder: String,
    pub bid_amount: u32,
    pub trump: String,
    pub hand_state: String
}

impl From<&Game> for DeclareTrumpResponse {

    fn from(game: &Game) -> Self {
        let (trump, hand_state, bidder, bid_amount) = game
            .current_hand()
            .and_then(|hand| {
                Some((
                    hand.trump().map(|t| t.to_string()).unwrap_or_default(),
                    hand.state().to_string(),
                    hand.bidder().map(|p| p.to_string()).unwrap_or_default(),
                    hand.bid_amount(),
                ))
            })
            .unwrap_or_else(|| ("".to_string(), "".to_string(), "".to_string(), None));

        Self {
            game_id: game.id().0,
            game_state: game.state().to_string(),
            bidder,
            bid_amount: bid_amount.unwrap_or(0),
            trump,
            hand_state
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordMeldRequest {
    pub us_meld: u32,
    pub them_meld: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordMeldResponse {
    pub game_id: Uuid,
    pub game_state: String,
    pub bidder: String,
    pub bid_amount: u32,
    pub trump: String,
    pub us_meld: u32,
    pub them_meld: u32,
    pub hand_state: String
}

impl From<&Game> for RecordMeldResponse {
    fn from(value: &Game) -> Self {
        let(bidder, bid_amount, trump, us_meld, them_meld, hand_state) = value
            .current_hand()
            .map(|h| (
                h.bidder().map(|p| p.to_string()).unwrap_or_default(),
                h.bid_amount(),
                h.trump().map(|t| t.to_string()).unwrap_or_default(),
                h.us_meld(),
                h.them_meld(),
                h.state().to_string()
            ))
            .unwrap_or_else(|| (
                "".to_string(),
                None,
                "".to_string(),
                None,
                None,
                "".to_string()
            ));

        Self {
            game_id: value.id().0,
            game_state: value.state().to_string(),
            bidder,
            bid_amount: bid_amount.unwrap_or(0),
            trump,
            us_meld: us_meld.unwrap_or(0),
            them_meld: them_meld.unwrap_or(0),
            hand_state
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordTricksRequest {
    pub us_tricks: u32,
    pub them_tricks: u32
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordTricksResponse {
    pub game_id: Uuid,
    pub game_state: String,
    pub bidder: String,
    pub bid_amount: u32,
    pub trump: String,
    pub us_meld: u32,
    pub them_meld: u32,
    pub us_tricks: u32,
    pub them_tricks: u32,
    pub us_total: i32,
    pub them_total: i32,
    pub hand_state: String
}

impl From<&Game> for RecordTricksResponse {
    fn from(value: &Game) -> Self {
        let (bidder, bid_amount, trump, us_meld, them_meld, us_tricks, them_tricks, us_total, them_total, hand_state) = value
            .current_hand()
            .map(|h| (
                h.bidder().map(|p| p.to_string()).unwrap_or_default(),
                h.bid_amount(),
                h.trump().map(|t| t.to_string()).unwrap_or_default(),
                h.us_meld(),
                h.them_meld(),
                h.us_tricks(),
                h.them_tricks(),
                h.us_total(),
                h.them_total(),
                h.state().to_string()
            ))
            .unwrap_or_else(|| (
                "".to_string(), // bidder
                None, // bid_amount
                "".to_string(), // trump
                None, // us_meld
                None, // them_meld
                None, // us_tricks
                None, // them_tricks
                0, // us_total
                0, // them_total
                "".to_string() // hand_state
            ));

        Self {
            game_id: value.id().0,
            game_state: value.state().to_string(),
            bidder,
            bid_amount: bid_amount.unwrap_or(0),
            trump,
            us_meld: us_meld.unwrap_or(0),
            them_meld: them_meld.unwrap_or(0),
            us_tricks: us_tricks.unwrap_or(0),
            them_tricks: them_tricks.unwrap_or(0),
            us_total,
            them_total,
            hand_state
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HandResponse {
    id: Uuid,
    state: String,
    dealer: Option<Player>,
    bidder: Option<Player>,
    bid_amount: Option<u32>,
    trump: Option<Suit>,
    us_total: Option<i32>,
    them_total: Option<i32>,
    us_meld: Option<u32>,
    them_meld: Option<u32>,
    us_tricks: Option<u32>,
    them_tricks: Option<u32>
}

impl From<&Hand> for HandResponse {
    fn from(hand: &Hand) -> Self {
        HandResponse {
            id: hand.id().0,
            state: hand.state().to_string(),
            dealer: Some(hand.dealer()),
            bidder: hand.bidder(),
            bid_amount: hand.bid_amount(),
            trump: hand.trump(),
            us_total: Some(hand.us_total()),
            them_total: Some(hand.them_total()),
            us_meld: hand.us_meld(),
            them_meld: hand.them_meld(),
            us_tricks: hand.us_tricks(),
            them_tricks: hand.them_tricks()
        }
    }
}

impl From<Option<&Hand>> for HandResponse {
    fn from(opt_hand: Option<&Hand>) -> Self {
        opt_hand.map(HandResponse::from).unwrap_or_else(|| HandResponse {
            id: Default::default(),
            state: "".to_string(),
            dealer: None,
            bidder: None,
            bid_amount: None,
            trump: None,
            us_total: None,
            them_total: None,
            us_meld: None,
            them_meld: None,
            us_tricks: None,
            them_tricks: None
        })
    }
}
