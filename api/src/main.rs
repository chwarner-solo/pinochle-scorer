use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use tokio::sync::Mutex;
use controller::router;
use crate::application::{DeclareTrump, GetCompletedHands, GetCurrentHand, GetRunningTotal, RecordBid, RecordMeld, RecordTricks, StartNewGame, StartNewHand};
use crate::domain::GameRepository;
use crate::infrastructure::InMemoryGameRepository;
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use controller::environment::Environment;

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
    pub get_running_total: Arc<GetRunningTotal>
}

struct App {
    app_state: AppState
}

#[tokio::main]
async fn main() {
    let env = Environment::from_env();
    
    // Set up tracing subscriber for logging
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| env.tracing_level().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = router(&env);
    let bind_address = env.bind_address();

    print_routes();

    println!("🚀 Server starting on {} ({:?})", bind_address, env);
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();

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