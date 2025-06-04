use axum::http::StatusCode;

pub trait ToResponse {
    fn to_response(&self) -> (StatusCode, String, u16);
}