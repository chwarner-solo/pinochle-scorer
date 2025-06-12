use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::RunningTotal;
use crate::domain::{Game, Hand, Player, Suit, GameState};
use crate::domain::Player::South;

#[derive(Debug, Clone, Deserialize)]
pub struct StartNewGameRequest {
    pub dealer: Player,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartNewHandRequest {
    pub(crate) game_id: Uuid
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordBidRequest {
    pub player: Player,
    pub bid: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeclareTrumpRequest {
    pub trump: Suit
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordMeldRequest {
    pub us_meld: u32,
    pub them_meld: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordTricksRequest {
    pub us_tricks: u32,
    pub them_tricks: u32
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
    them_tricks: Option<u32>,
    required_tricks: Option<u32>,
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
            them_tricks: hand.them_tricks(),
            required_tricks: hand.tricks_to_save()
        }
    }
}

impl From<Hand> for HandResponse {
    fn from(hand: Hand) -> Self {
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
            them_tricks: hand.them_tricks(),
            required_tricks: hand.tricks_to_save()
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
            them_tricks: None,
            required_tricks: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub game_id: Uuid,
    pub game_state: Option<GameState>,
    pub dealer: Option<Player>,
    pub hand_state: Option<String>,
    pub bidder: Option<Player>,
    pub bid_amount: Option<u32>,
    pub trump: Option<Suit>,
    pub us_meld: Option<u32>,
    pub them_meld: Option<u32>,
    pub us_tricks: Option<u32>,
    pub them_tricks: Option<u32>,
    pub us_score: Option<i32>,
    pub them_score: Option<i32>,    
    pub us_hand_score: Option<i32>,
    pub them_hand_score: Option<i32>,
    pub required_tricks: Option<u32>,
}

impl From<&Game> for GameResponse {
    fn from(game: &Game) -> Self {
        let hand = game.current_hand();
        let (us_score, them_score) = game.running_totals();
        let required_tricks = game.current_hand().and_then(|h| h.tricks_to_save());
        let us_hand_score = game.current_hand().and_then(|h| Some(h.us_total()));
        let them_hand_score = game.current_hand().and_then(|h| Some(h.them_total()));
        let (us_score, them_score) = game.running_totals();

        GameResponse {
            game_id: game.id().0,
            game_state: Some(game.state()),
            dealer: Some(game.current_dealer()),
            hand_state: hand.clone().map(|h| h.state().to_string()),
            bidder: hand.clone().and_then(|h| h.bidder()),
            bid_amount: hand.clone().and_then(|h| h.bid_amount()),
            trump: hand.clone().and_then(|h| h.trump()),
            us_meld: hand.clone().and_then(|h| h.us_meld()),
            them_meld: hand.clone().and_then(|h| h.them_meld()),
            us_tricks: hand.clone().and_then(|h| h.us_tricks()),
            them_tricks: hand.clone().and_then(|h| h.them_tricks()),
            us_hand_score,
            them_hand_score,
            required_tricks,
            us_score: Some(us_score),
            them_score: Some(them_score),
        }
    }
}

impl From<Option<&Game>> for GameResponse {
    fn from(game_opt: Option<&Game>) -> Self {
        match game_opt {
            Some(game) => GameResponse::from(game),
            None => GameResponse {
                game_id: Uuid::nil(),
                game_state: Some(GameState::NoGame),
                dealer: None,
                hand_state: None,
                bidder: None,
                bid_amount: None,
                trump: None,
                us_meld: None,
                them_meld: None,
                us_tricks: None,
                them_tricks: None,
                us_score: None,
                them_score: None,
                us_hand_score: None,
                them_hand_score: None,
                required_tricks: None,
            }
        }
    }
}
