use yew::Callback;
use crate::context::players::Player;


#[derive(Clone)]
pub enum InGameMovement {
    Draw,
    Discard,
    TakeBin,
    Close
}

#[derive(Clone)]
pub struct InGameEvent {
    pub movement: Option<InGameMovement>,
    from: Option<Player>,
    to: Option<Player>,
}

#[derive(Clone)]
pub enum GameStatus {
    NotStarted,
    Waiting,
    InProgress,
    GameOver,
}

#[derive(Clone)]
pub struct GameState {
    pub game_status: GameStatus,
    pub game_id: Option<String>,
    pub player_name: Option<String>,
    pub players: Vec<Player>,
    pub event: Option<InGameEvent>,
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