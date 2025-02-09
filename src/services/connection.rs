use std::env;
use gloo_net::http::Request;
use crate::errors::game_error::GameError;
use crate::models::api_data::ConnectResponse;

const API_URL: &str = env!("API_URL");

pub async fn create_game() -> Result<String, GameError> {
    // let api_url: &str = &get_api_url();
    let api_url: &str = API_URL;
    web_sys::console::log_1(&api_url.into());

    return Ok("test".to_string());
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