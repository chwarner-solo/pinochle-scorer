use crate::domain::Player;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StartNewGameRequest {
    pub dealer: Player,
}