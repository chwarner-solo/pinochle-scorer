use std::sync::Arc;
use uuid::Uuid;
use crate::domain::{GameRepository, HandState, Player, GameState, GameId, GameRepositoryError, Game};

pub struct GetGameStatus {
    pub game_repo: Arc<dyn GameRepository + Send + Sync>
}

pub struct GameStatus {
    pub id: Uuid,
    pub game_state: Option<GameState>,
    pub hand_state: Option<HandState>,
    pub current_dealer: Option<Player>,
    pub bidder: Option<Player>,
    pub bid_amount: Option<u32>,
    pub us_meld: Option<u32>,
    pub them_meld: Option<u32>,
    pub us_tricks: Option<u32>,
    pub them_tricks: Option<u32>,
    pub us_total: Option<i32>,            // per-hand total
    pub them_total: Option<i32>,          // per-hand total
    pub us_running_total: i32,            // running total
    pub them_running_total: i32           // running total
}

impl Default for GameStatus {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            game_state: None,
            hand_state: None,
            current_dealer: None,
            bidder: None,
            bid_amount: None,
            us_meld: None,
            them_meld: None,
            us_tricks: None,
            them_tricks: None,
            us_total: None,
            them_total: None,
            us_running_total: 0,
            them_running_total: 0
        }
    }
}

impl From<Option<Game>> for GameStatus {
    fn from(value: Option<Game>) -> Self {
        match value {
            Some(game) => {
                let (us_running_total, them_running_total) = game.running_totals();
                Self {
                    id: game.id().0,
                    game_state: Some(game.state()),
                    hand_state: game.current_hand().map(|hand| hand.state()),
                    current_dealer: game.current_hand().map(|hand| hand.dealer()),
                    bidder: game.current_hand().map(|hand| hand.bidder()).unwrap_or(None),
                    bid_amount: game.current_hand().map(|hand| hand.bid_amount()).unwrap_or(None),
                    us_meld: game.current_hand().map(|hand| hand.us_meld()).unwrap_or(None),
                    them_meld: game.current_hand().map(|hand| hand.them_meld()).unwrap_or(None),
                    us_tricks: game.current_hand().map(|hand| hand.us_tricks()).unwrap_or(None),
                    them_tricks: game.current_hand().map(|hand| hand.them_tricks()).unwrap_or(None),
                    us_total: game.current_hand().map(|hand| hand.us_total()),
                    them_total: game.current_hand().map(|hand| hand.them_total()),
                    us_running_total,
                    them_running_total
                }
            },
            _ => GameStatus::default()
        }
    }
}

impl GetGameStatus {
    pub fn new(game_repo: Arc<dyn GameRepository + Send + Sync>) -> Self {
        Self {
            game_repo
        }
    }
    pub async fn execute(&self, game_id: GameId) -> Result<GameStatus, GetGameStatusError> {
        let game = self.game_repo.find_by_id(game_id).await?;
        Ok(GameStatus::from(game))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetGameStatusError {
    #[error("Repository error: {0}")]
    GameRepositoryError(#[from] GameRepositoryError)
}