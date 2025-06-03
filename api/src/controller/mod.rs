use std::sync::Arc;
use axum::{Router, routing::{get, post}};
use tokio::sync::Mutex;
use crate::application::{DeclareTrump, RecordBid, RecordMeld, RecordTricks, StartNewGame};
use crate::AppState;
use crate::domain::{GameRepository, Player};
use crate::infrastructure::InMemoryGameRepository;
use axum::{Json, extract::{Path, State}};
use serde_json::json;

mod data_transfer;

use data_transfer::{StartNewGameRequest};

// --- Handler stubs ---
pub async fn start_new_game_handler(State(state): State<AppState>, Json(payload): Json<StartNewGameRequest>) -> Json<serde_json::Value> {

    tracing::info!("start_new_game_handler");

    let game = state.start_game.execute(payload.dealer).await.unwrap();

    tracing::info!("game: {:#?}", game);

    Json(json!({"message": "start_new_game stub"}))
}

pub async fn start_new_hand_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    tracing::info!("start_new_hand_handler");
    Json(json!({"message": "start_new_hand stub", "game_id": game_id}))
}

pub async fn get_completed_hands_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    tracing::info!("get_completed_hands_handler");
    Json(json!({"message": "get_completed_hands stub", "game_id": game_id}))
}

pub async fn get_current_hand_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    tracing::info!("get_current_hand_handler");
    Json(json!({"message": "get_current_hand stub", "game_id": game_id}))
}

pub async fn get_running_total_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    tracing::info!("get_running_total_handler");
    Json(json!({"message": "get_running_total stub", "game_id": game_id}))
}

pub async fn record_bid_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    tracing::info!("record_bid_handler");
    Json(json!({"message": "record_bid stub", "game_id": game_id}))
}

pub async fn declare_trump_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    Json(json!({"message": "declare_trump stub", "game_id": game_id}))
}

pub async fn record_meld_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    Json(json!({"message": "record_meld stub", "game_id": game_id}))
}

pub async fn record_tricks_handler(State(_state): State<AppState>, Path(game_id): Path<String>) -> Json<serde_json::Value> {
    Json(json!({"message": "record_tricks stub", "game_id": game_id}))
}

// --- Router setup ---
pub fn router() -> Router {
    let repo: Arc<Mutex<dyn GameRepository>> = Arc::new(Mutex::new(InMemoryGameRepository::new()));
    let start_game = Arc::new(StartNewGame::new(repo.clone()));
    let record_bid = Arc::new(RecordBid::new(repo.clone()));
    let declare_trump = Arc::new(DeclareTrump::new(repo.clone()));
    let record_meld = Arc::new(RecordMeld::new(repo.clone()));
    let record_tricks = Arc::new(RecordTricks::new(repo.clone()));

    let state = AppState {
        repo,
        start_game,
        record_bid,
        declare_trump,
        record_meld,
        record_tricks
    };

    let inner_router = Router::new()
        .route("/", get(get_completed_hands_handler))
        .route("/", post(start_new_hand_handler))
        .route("/current_hand", get(get_current_hand_handler))
        .route("/running_total", get(get_running_total_handler))
        .route("/record_bid", post(record_bid_handler))
        .route("/declare_trump", post(declare_trump_handler))
        .route("/record_meld", post(record_meld_handler))
        .route("/record_tricks", post(record_tricks_handler))
        .with_state(state.clone());


    let router = Router::new()
        .route("/api/games", post(start_new_game_handler))
        .nest("/api/games/{game_id}", inner_router)
        .with_state(state);

    router
}