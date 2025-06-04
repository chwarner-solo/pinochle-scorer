use axum::http::StatusCode;
use crate::controller::error_response::ToResponse;
use crate::domain::{GameError, GameRepositoryError, HandError, HandRepositoryError};

impl ToResponse for GameError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
    }
}

impl ToResponse for HandError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            HandError::InvalidBid(_) => (StatusCode::BAD_REQUEST, self.to_string(), 400),
            HandError::InvalidTricks(_, _) => (StatusCode::BAD_REQUEST, self.to_string(), 400),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for GameRepositoryError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            GameRepositoryError::GameDoesNotExist(game_id) => (StatusCode::NOT_FOUND, game_id.to_string(), 404),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for HandRepositoryError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}
