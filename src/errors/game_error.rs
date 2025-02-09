use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GameError {
    CreationFailed(String),
    // Add other error variants as needed
}

impl std::error::Error for GameError {}
impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::CreationFailed(msg) => write!(f, "Failed to create game: {}", msg),
        }
    }
}

