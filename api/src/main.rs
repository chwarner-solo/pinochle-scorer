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

impl App {

    fn router() -> Router {
        router()
    }
}

#[tokio::main]
async fn main() {
    // Set up tracing subscriber for logging
    tracing_subscriber::fmt::init();

    let app = App::router()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!("http_request", method = %request.method(), uri = %request.uri())
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!(status = %response.status(), latency = ?latency, "response sent");
                })
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::info!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
