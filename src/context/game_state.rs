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

    pub fn new(create_game: Callback<String>, join: Callback<(String, String)>) -> GameState {
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
            join
        }
    }
    // pub async fn create_game(&mut self) -> Result<(), GameError> {
    //     let res = create_game().await?;
    //     let mut game_data = self.game_data.borrow_mut();
    //     game_data.game_id = Some(res);
    //     game_data.counter += 1;
    //     // self.game_id = Some(res);
    //     Ok(())
    // }
    // pub async fn join_game(&mut self, name: &str) -> Result<(), GameError> {
    //
    //     log_1(&"joingin".into());
    //     let game_data = self.game_data.borrow();
    //     let game_id = game_data.game_id.clone().unwrap();
    //     let ws = join_game(&game_id, name)?;
    //     let (writer, mut read) = ws.split();
    //     // self.ws = Some(ws.c);
    //     let game_data_arc = self.game_data.clone();
    //     spawn_local(async move {
    //         while let Some(Ok(msg)) = read.next().await {
    //             match msg {
    //                 Message::Text(message) => {
    //                     log_1(&message.clone().into());
    //                     let game_response = serde_json::from_str::<GameResponse>(message.as_str()).unwrap();
    //                     match game_response.message_type {
    //                         MessageType::PlayerJoin => {
    //                             let players = game_response.data.unwrap().players;
    //                             GameState::handle_players_change(game_data_arc.clone(), &players, true);
    //                             // self.handle_players_change(&players);
    //                             // self.game_data.borrow_mut().counter += 1;
    //                         }
    //                         MessageType::PlayerLeft => {}
    //                         MessageType::GameEvent => {}
    //                         MessageType::EndGame => {}
    //                     };
    //                 }
    //                 Message::Bytes(message ) => {}
    //             }
    //             // log_1(&msg.to_into());
    //         }
    //     });
    //     Ok(())
    // }

    // fn handle_players_change(game_data_arc: Rc<RefCell<GameData>>, players: &Vec<PlayerData>, update_count: bool) {
    //         let mut game_data = game_data_arc.borrow_mut();
    //         game_data.players = players.into_iter().map(|player| {
    //                Player {
    //                    name: player.name.clone(),
    //                    bin: player.bin.clone(),
    //                    hand: player.hand.clone(),
    //                }
    //            }).collect();
    //         if update_count {
    //             game_data.counter += 1;
    //         }
    // }
}