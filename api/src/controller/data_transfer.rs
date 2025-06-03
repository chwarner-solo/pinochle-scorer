use uuid::Uuid;
use crate::domain::{Game, Hand, Player, Suit};
use serde::{Serialize, Deserialize};
use crate::application::RunningTotal;

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

#[derive(Debug, Clone, Serialize)]
pub struct HandResponse {
    id: Uuid,
    state: String,
    dealer: Option<Player>,
    bidder: Option<Player>,
    bid_amount: Option<u32>,
    trump: Option<Suit>,
    us_total: Option<u32>,
    them_total: Option<u32>,
    us_meld: Option<u32>,
    them_meld: Option<u32>
}

impl From<&Hand> for HandResponse {
    fn from(hand: &Hand) -> Self {
        let id = hand.id().0;
        let state = hand.state();
        let dealer = hand.dealer();
        let bid_amount = hand.bid_amount();

        Self {
            id,
            state: state.to_string(),
            dealer: Some(dealer),
            bidder: None,
            bid_amount,
            trump: None,
            us_total: None,
            them_total: None,
            us_meld: None,
            them_meld: None
        }
    }
}

impl From<Option<&Hand>> for HandResponse {
    fn from(value: Option<&Hand>) -> Self {
        if let Some(hand) = value {
            let id = hand.id().0;
            let state = hand.state();
            let dealer = hand.dealer();
            let bid_amount = hand.bid_amount();
            
            Self {
                id,
                state: state.to_string(),
                dealer: Some(dealer),
                bidder: None,
                bid_amount,
                trump: None,
                us_total: None,
                them_total: None,
                us_meld: None,
                them_meld: None
            }
        } else {
            Self {
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
            }
        }
    }
}

