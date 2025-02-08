use crate::context::players::Player;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use serde_json::Value;
use web_sys::wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use crate::context::api_data::ConnectResponse;

#[derive(Debug,Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerPhase {
    P1,
    P2,
    Waiting,
}
#[derive(Clone)]
pub enum InGameMovement {
    _Draw,
    _Discard,
    _TakeBin,
    _Close
}

#[derive(Clone)]
pub struct InGameEvent {
    pub _movement: Option<InGameMovement>,
    _from: Option<Player>,
    _to: Option<Player>,
}

#[derive(Clone, PartialEq)]
pub enum GameStatus {
    PreGame,
    InProgress,
    _PostGame,
}

#[derive(Clone)]
pub struct GameState {
    pub game_status: GameStatus,
    pub game_id: Option<String>,
    pub card_left: u8,
    pub current_player_index: usize,
    pub current_player_name: Option<String>, // the person name who open the game
    pub player_turn_index: usize, // the turn index
    pub player_turn_phase: PlayerPhase,
    pub players: Vec<Player>,
    pub _event: Option<InGameEvent>,
    pub counter: usize,
    // pub join: Callback<String>,
    // pub create_game: Callback<()>,
    // pub action: Callback<String>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter
    }
}

const API_URL: &str = env!("API_URL");

impl GameState {
    pub async fn connect(&mut self) -> bool {
        web_sys::console::log_1(&API_URL.into());

        let response = Request::get(&format!("{}/create", API_URL)).send().await;

        if let Ok(response) = response {
            let data: ConnectResponse = response.json::<ConnectResponse>().await.unwrap();
            self.game_id = Option::from(data.game_id.clone());
            web_sys::console::log_1(&(data.game_id).into());

            spawn_local(async {

            });
            // let p = JsValue::from(&data["game_id"]);
            return true
        }
        false
    }
}