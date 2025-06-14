use std::sync::Arc;
use std::time::Duration;
use axum::{
    Router,
    routing::{get, post},
    http::{
        StatusCode
    },
    response::{IntoResponse, Response},
    Json,
    extract::{Path, State},
    debug_handler
};
use axum::http::{header, HeaderMap, HeaderValue, Method, Request};
use crate::application::{
    DeclareTrump, DeclareTrumpError, 
    GetCompletedHands, GetCompletedHandsError, 
    GetCurrentHand, GetCurrentHandError, 
    GetRunningTotal, GetRunningTotalError, 
    RecordBid, RecordBidError, 
    RecordMeld, RecordMeldError, 
    RecordTricks, RecordTricksError, 
    StartNewGame, StartNewGameError, 
    StartNewHand, StartNewHandError,
    GetGameStatus, GetGameStatusError, GameStatus
};
use crate::AppState;
use crate::domain::{GameId, GameRepository, GameRepositoryError};
use crate::infrastructure::InMemoryGameRepository;
use serde_json::json;
use thiserror::Error;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;
use uuid::Uuid;

mod data_transfer;
mod error_response;
mod infrastructure;

use data_transfer::{StartNewGameRequest, 
    DeclareTrumpRequest, 
    HandResponse, 
    RecordBidRequest, 
    RecordMeldRequest, 
    RecordTricksRequest, 
    RunningTotalResponse, 
    StartNewHandRequest,
    GameResponse, GetGameStateResponse
};
use crate::controller::error_response::ToResponse;

fn create_cors_layer() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    cors
}

#[debug_handler]
// --- Handler stubs ---
pub async fn start_new_game_handler(State(state): State<AppState>, headers: HeaderMap, Json(payload): Json<StartNewGameRequest>) -> Result<Json<GameResponse>, AppError> {

    tracing::info!("=== START NEW GAME HANDLER ===");
    tracing::info!("All Headers: {:#?}", headers);
    tracing::info!("start_new_game_handler");

    let AppState { start_game, .. } = state;

    let game = start_game.execute(payload.dealer).await?;

    tracing::info!("game: {:#?}", game);

    let dto = GameResponse::from(&game);

    Ok(Json(dto))
}

#[debug_handler]
pub async fn start_new_hand_handler(State(state): State<AppState>, Json(payload): Json<StartNewHandRequest>) -> Result<Json<GameResponse>, AppError> {
    tracing::info!("start_new_hand_handler: begin");

    let AppState { start_hand, .. } = state;

    let game = start_hand.execute(GameId(payload.game_id)).await?;
    tracing::info!("start_new_hand_handler: got game");

    let dto = GameResponse::from(&game);

    tracing::info!("start_new_hand_handler: end");

    Ok(Json(dto))
}

#[debug_handler]
pub async fn get_game_status_handler(State(state): State<AppState>, Path(game_id): Path<String>) -> Result<Json<GetGameStateResponse>, AppError> {
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;
    tracing::info!("get_game_status_handler");
    let AppState { get_game_status, .. } = state;
    let status = get_game_status.execute(GameId(id)).await?;
    let dto = GetGameStateResponse::from(status);

    Ok(Json(dto))
}
pub async fn get_completed_hands_handler(State(state): State<AppState>, Path(game_id): Path<String>) -> Result<Json<Vec<HandResponse>>, AppError> {
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;
    tracing::info!("get_completed_hands_handler");
    let AppState { get_completed_hands, .. } = state;
    let hands = get_completed_hands.execute(GameId(id)).await?;
    let dto = hands.iter().map(HandResponse::from).collect();
    
    Ok(Json(dto))
}

pub async fn get_current_hand_handler(State(state): State<AppState>, Path(game_id): Path<String>) -> Result<Json<HandResponse>, AppError> {
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;
    tracing::info!("get_current_hand_handler");
    let AppState { get_current_hand, .. } = state;
    
    if let Some(hand) = get_current_hand.execute(GameId(id)).await? {
        let dto = HandResponse::from(&hand);
        Ok(Json(dto))
    } else {
        Err(AppError::GetParseUuidError(game_id.clone()))
    }
}

pub async fn get_running_total_handler(State(state): State<AppState>, Path(game_id): Path<String>) -> Result<Json<RunningTotalResponse>, AppError> {
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;

    tracing::info!("get_running_total_handler");
    let AppState { get_running_total, .. } = state;
    
    let total = get_running_total.execute(GameId(id)).await?;
    let dto = RunningTotalResponse::from(&total);
    
    Ok(Json(dto))
}

pub async fn record_bid_handler(State(state): State<AppState>, Path(game_id): Path<String>, Json(payload): Json<RecordBidRequest>) -> Result<Json<GameResponse>, AppError> {
    tracing::info!("record_bid_handler");
    let AppState { record_bid, .. } = state;
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;

    let game = record_bid.execute(GameId(id), payload.player, payload.bid).await?;
    let dto = GameResponse::from(&game);

    Ok(Json(dto))
}

pub async fn declare_trump_handler(State(state): State<AppState>, Path(game_id): Path<String>, Json(payload): Json<DeclareTrumpRequest>) -> Result<Json<GameResponse>, AppError> {
    let AppState { declare_trump, .. } = state;
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;

    let game = declare_trump.execute(GameId(id), payload.trump).await?;

    let dto = GameResponse::from(&game);

    Ok(Json(dto))
}


#[debug_handler]
pub async fn record_meld_handler(State(state): State<AppState>, Path(game_id): Path<String>, Json(payload): Json<RecordMeldRequest>) -> Result<Json<GameResponse>, AppError> {
    let AppState { record_meld, .. } = state;
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;

    let game = record_meld.execute(GameId(id), payload.us_meld, payload.them_meld).await?;

    let dto = GameResponse::from(&game);

    Ok(Json(dto))
}

pub async fn record_tricks_handler(State(state): State<AppState>, Path(game_id): Path<String>, Json(payload): Json<RecordTricksRequest>) -> Result<Json<GameResponse>, AppError> {
    let AppState { record_tricks, .. } = state;
    let id = Uuid::parse_str(&game_id).map_err(|e| AppError::GetParseUuidError(game_id.clone()))?;

    let game = record_tricks.execute(GameId(id), payload.us_tricks, payload.them_tricks).await?;

    let dto = GameResponse::from(&game);

    Ok(Json(dto))
}

// --- Router setup ---
pub fn router() -> Router {
    let repo: Arc<dyn GameRepository> = Arc::new(InMemoryGameRepository::new());
    let start_game = Arc::new(StartNewGame::new(repo.clone()));
    let start_hand = Arc::new(StartNewHand::new(repo.clone()));
    let record_bid = Arc::new(RecordBid::new(repo.clone()));
    let declare_trump = Arc::new(DeclareTrump::new(repo.clone()));
    let record_meld = Arc::new(RecordMeld::new(repo.clone()));
    let record_tricks = Arc::new(RecordTricks::new(repo.clone()));
    let get_completed_hands = Arc::new(GetCompletedHands::new(repo.clone()));
    let get_current_hand = Arc::new(GetCurrentHand::new(repo.clone()));
    let get_running_total = Arc::new(GetRunningTotal::new(repo.clone()));
    let get_game_status = Arc::new(GetGameStatus::new(repo.clone()));

    let state = AppState {
        start_game,
        start_hand,
        record_bid,
        declare_trump,
        record_meld,
        record_tricks,
        get_completed_hands,
        get_current_hand,
        get_running_total,
        get_game_status,
    };

    let inner_router = Router::new()
        .route("/", get(get_game_status_handler))
        .route("/current_hand", get(get_current_hand_handler))
        .route("/running_total", get(get_running_total_handler))
        .route("/completed_hands", get(get_completed_hands_handler))
        .route("/record_bid", post(record_bid_handler))
        .route("/declare_trump", post(declare_trump_handler))
        .route("/record_meld", post(record_meld_handler))
        .route("/record_tricks", post(record_tricks_handler))
        .with_state(state.clone());


    let router = Router::new()
        .route("/api/games/", post(start_new_game_handler))
        .route("/api/games/start_hand", post(start_new_hand_handler))
        .nest("/api/games/{game_id}/", inner_router)
        .with_state(state)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    uri = ?request.uri(),
                    headers = ?request.headers(),
                )
            })
            .on_request(|_request: &Request<_>, _span: &Span| {
                tracing::info!("Started processing request");
            })
            .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                tracing::info!("Finished processing request in {:?}", latency)
            })
        )
        .layer(create_cors_layer());

    router
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    StartNewGameError(#[from] StartNewGameError),
    #[error(transparent)]
    StartNewHandError(#[from] StartNewHandError),
    #[error(transparent)]
    RecordBidError(#[from] RecordBidError),
    #[error(transparent)]
    DeclareTrumpError(#[from] DeclareTrumpError),
    #[error(transparent)]
    RecordMeldError(#[from] RecordMeldError),
    #[error(transparent)]
    RecordTricksError(#[from] RecordTricksError),
    #[error(transparent)]
    GetCompletedHandsError(#[from] GetCompletedHandsError),
    #[error(transparent)]
    GetGameStatusError(#[from] GetGameStatusError),
    #[error(transparent)]
    GetCurrentHandError(#[from] GetCurrentHandError),
    #[error(transparent)]
    GetRunningTotalError(#[from] GetRunningTotalError),
    #[error("Parse error: {0}")]
    GetParseUuidError(String),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match self {
            AppError::StartNewGameError(e) => e.to_response(),
            AppError::StartNewHandError(e) => e.to_response(),
            AppError::RecordBidError(e) => e.to_response(),
            AppError::DeclareTrumpError(e) => e.to_response(),
            AppError::RecordMeldError(e) => e.to_response(),
            AppError::RecordTricksError(e) => e.to_response(),
            AppError::GetCompletedHandsError(e) => e.to_response(),
            AppError::GetCurrentHandError(e) => e.to_response(),
            AppError::GetRunningTotalError(e) => e.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500),
        };

        let body = Json(json!({
            "error":{
                "code": error_code,
                "message": error_message
            }
        }));

        (status, body).into_response()
    }
}
