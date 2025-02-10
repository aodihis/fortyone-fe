use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    PlayerJoin,
    PlayerLeft,
    GameEvent,
    EndGame,
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ConnectResponse {
    pub game_id: String,
    pub num_of_players: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResponse {
    pub message_type: MessageType,
    pub status: String,
    pub data: Option<GameData>,
    pub message: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub player_id: Option<String>,
    pub player_pos: Option<u8>,
    pub num_of_players: Option<u8>,
    pub card_left: Option<u8>,
    pub current_turn: Option<u8>,
    pub current_phase: Option<String>,
    pub event: Option<GameEvent>,
    pub players: Vec<PlayerData>,
    pub winner_name: Option<String>,
    pub scores: Option<Vec<i16>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
    pub hand: Vec<String>,
    pub bin: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum GameEventType {
    GameStart,
    Draw,
    TakeBin,
    Discard,
    Close,

}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GameEvent {
    event_type: GameEventType,
    from: Option<u8>,
    to: Option<u8>,
}

