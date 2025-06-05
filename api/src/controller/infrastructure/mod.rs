use axum::http::StatusCode;
use crate::application::{DeclareTrumpError, GetCompletedHandsError, GetCurrentHandError, GetRunningTotalError, RecordBidError, RecordMeldError, RecordTricksError, StartNewGameError, StartNewHandError};
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

impl ToResponse for DeclareTrumpError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            DeclareTrumpError::GameNotFound(game_id) => (StatusCode::NOT_FOUND, game_id.to_string(), 404),
            DeclareTrumpError::RepositoryError(repo_error) => repo_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for GetCompletedHandsError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            GetCompletedHandsError::GameRepoError(repo_error) => repo_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for GetCurrentHandError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            GetCurrentHandError::GameNotFound(game_id) => (StatusCode::NOT_FOUND, game_id.to_string(), 404),
            GetCurrentHandError::RepositoryError(repo_error) => repo_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}


impl ToResponse for GetRunningTotalError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            GetRunningTotalError::GameNotFound => (StatusCode::NOT_FOUND, self.to_string(), 404),
            GetRunningTotalError::RepositoryError(repo_error) => repo_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for RecordBidError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            RecordBidError::GameNotFound(game_id) => (StatusCode::NOT_FOUND, game_id.to_string(), 404),
            RecordBidError::RepositoryError(repo_error) => repo_error.to_response(),
            RecordBidError::GameError(game_error) => game_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for RecordMeldError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            RecordMeldError::GameNotFound => (StatusCode::NOT_FOUND, self.to_string(), 404),
            RecordMeldError::RepositoryError(repo_error) => repo_error.to_response(),
            RecordMeldError::GameError(game_error) => game_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for RecordTricksError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            RecordTricksError::GameNotFound => (StatusCode::NOT_FOUND, self.to_string(), 404),
            RecordTricksError::RepositoryError(repo_error) => repo_error.to_response(),
            RecordTricksError::GameError(game_error) => game_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for StartNewGameError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            StartNewGameError::GameRepositoryError(repo_error) => repo_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}

impl ToResponse for StartNewHandError {
    fn to_response(&self) -> (StatusCode, String, u16) {
        match self {
            StartNewHandError::GameNotFound(game_id) => (StatusCode::NOT_FOUND, game_id.to_string(), 404),
            StartNewHandError::RepositoryError(repo_error) => repo_error.to_response(),
            StartNewHandError::GameError(game_error) => game_error.to_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), 500)
        }
    }
}
