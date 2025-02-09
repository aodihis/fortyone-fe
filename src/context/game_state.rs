use std::cell::RefCell;
use std::rc::Rc;
use crate::models::players::Player;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use serde_json::Value;
use web_sys::wasm_bindgen::JsValue;
use yew::Callback;
use yew::platform::spawn_local;
use crate::models::api_data::ConnectResponse;
use crate::errors::game_error::GameError;
use crate::services::connection::create_game;

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
pub struct GameData {
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
}

#[derive(Clone)]
pub struct GameState {

    pub game_data: Rc<RefCell<GameData>>,
    // pub join: Callback<String>,
    // pub create_game: Callback<()>,
    // pub action: Callback<String>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        let game_data = self.game_data.borrow();
        game_data.counter == other.game_data.borrow().counter
    }
}

impl GameState {

    pub fn new() -> GameState {
        let dummy_players = vec![Player{
            name: "P1".to_string(),
            score: 0,
            bin: vec!["HA".to_string(),"S3".to_string(),"D10".to_string(),"C2".to_string()],
            hand: vec!["H3".to_string(),"S8".to_string(),"D3".to_string(),"C2".to_string()],
        },Player{
            name: "P2".to_string(),
            score: 0,
            bin: vec!["HA".to_string(),"S3".to_string(),"D10".to_string(),"C2".to_string()],
            hand: vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
        },Player{
            name: "P3".to_string(),
            score: 0,
            bin: vec!["HA".to_string(),"S3".to_string(),"D10".to_string(),"C2".to_string()],
            hand: vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
        },Player{
            name: "P4".to_string(),
            score: 0,
            bin: vec!["HA".to_string(),"S3".to_string(),"D10".to_string(),"C2".to_string()],
            hand: vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
        }];

        let game_data = GameData {
            game_status: GameStatus::PreGame,
            game_id: None,
            card_left: 52,
            current_player_index: 0,
            player_turn_index: 0,
            player_turn_phase: PlayerPhase::P1,
            current_player_name: Some(String::from("Mia")),
            players: dummy_players,
            _event: None,
            counter: 0,
        };
        Self {
            game_data: Rc::new(RefCell::new(game_data))
        }
    }
    pub async fn create_game(&mut self) -> Result<(), GameError> {
        let res = create_game().await?;
        // self.game_id = Some(res);
        Ok(())
    }
    pub async fn join_game(&mut self) -> Result<(), GameError> {

        spawn_local(async {

        });
        Ok(())
    }
}