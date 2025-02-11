use crate::errors::game_error::GameError;
use crate::models::players::Player;
use crate::services::connection::{create_game, join_game};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use web_sys::console::log_1;
use yew::platform::spawn_local;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::Message;
use yew::Callback;
use crate::models::api_data::{GameResponse, MessageType, PlayerData};

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
    // pub game_data: Rc<RefCell<GameData>>,
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
    pub create_game: Callback<String>,
    pub join: Callback<(String, String)>,
    pub disconnect: Callback<()>,
    // pub join: Callback<String>,
    // pub create_game: Callback<()>,
    // pub action: Callback<String>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter
    }
}

impl GameState {

    pub fn new(create_game: Callback<String>, join: Callback<(String, String)>, disconnect: Callback<()>) -> GameState {
        Self {
            game_status: GameStatus::PreGame,
            game_id: None,
            card_left: 52,
            current_player_index: 0,
            player_turn_index: 0,
            player_turn_phase: PlayerPhase::P1,
            current_player_name: None,
            players: vec![],
            _event: None,
            counter: 0,
            create_game,
            join,
            disconnect
        }
    }

    pub fn clear(&mut self) {
        self.game_status = GameStatus::PreGame;
        self.game_id = None;
        self.card_left = 52;
        self.current_player_index = 0;
        self.player_turn_index = 0;
        self.player_turn_phase = PlayerPhase::P1;
        self.current_player_name = None;
        self.players = vec![];
        self.counter = 0;
        self._event = None;
    }
}