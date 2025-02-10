use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use gloo_net::http::Request;
use gloo_net::websocket::futures::WebSocket;
use gloo_timers::future::TimeoutFuture;
use crate::context::game_state::GameData;
use crate::errors::game_error::GameError;
use crate::models::api_data::ConnectResponse;

const API_URL: &str = env!("API_URL");

pub async fn create_game() -> Result<String, GameError> {
    // let api_url: &str = &get_api_url();
    let api_url: &str = API_URL;
    web_sys::console::log_1(&api_url.into());
    // return Ok("test".to_string());
    let response = Request::get(&format!("{}/create", api_url)).send().await;


    match response {
        Err(e) => {
            Err(GameError::CreationFailed(e.to_string()))
        }
        Ok(response) => {
            let data: ConnectResponse = response.json::<ConnectResponse>().await.unwrap();
            Ok(data.game_id)
        }
    }

}

pub fn join_game(game_id: &str, name: &str) -> Result<(WebSocket), GameError> {
    let api_url: &str = API_URL;

    let socket = match WebSocket::open(&format!("{}/{}/join?name={}", api_url, game_id, name)) {
        Ok(ws) => ws,
        Err(e) => {
            return Err(GameError::JoinFailed(e.to_string()));
        }
    };
    Ok(socket)
    // let (writer, mut read) = socket.split();
}