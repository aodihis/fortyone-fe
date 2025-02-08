use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ConnectResponse {
    pub game_id: String,
    pub num_of_players: usize,
}