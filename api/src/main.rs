use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use tokio::sync::Mutex;
use controller::router;
use crate::application::{DeclareTrump, GetCompletedHands, GetCurrentHand, GetGameStatus, GetRunningTotal, RecordBid, RecordMeld, RecordTricks, StartNewGame, StartNewHand};
use crate::domain::GameRepository;
use crate::infrastructure::InMemoryGameRepository;
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod domain;
mod application;
mod controller;
mod infrastructure;

#[derive(Clone)]
struct AppState {
    pub start_game: Arc<StartNewGame>,
    pub start_hand: Arc<StartNewHand>,
    pub record_bid: Arc<RecordBid>,
    pub declare_trump: Arc<DeclareTrump>,
    pub record_meld: Arc<RecordMeld>,
    pub record_tricks: Arc<RecordTricks>,
    pub get_completed_hands: Arc<GetCompletedHands>,
    pub get_current_hand: Arc<GetCurrentHand>,
    pub get_running_total: Arc<GetRunningTotal>,
    pub get_game_status: Arc<GetGameStatus>
}

struct App {
    app_state: AppState
}

impl App {

    fn router() -> Router {
        router()
    }
}

#[tokio::main]
async fn main() {
    // Set up tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = App::router();

    print_routes();

    println!("🚀 Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn print_routes() {
    println!("📍 Configured Routes:");
    println!("  POST /api/games/");
    println!("  GET  /api/games/:id/");
    println!("  POST /api/games/:id/start_hand");
    println!("  POST /api/games/:id/record_bid");
    println!("  POST /api/games/:id/declare_trump");
    println!("  POST /api/games/:id/record_meld");
    println!("  POST /api/games/:id/record_tricks");
    println!("  GET  /api/games/:id/running_total");
}