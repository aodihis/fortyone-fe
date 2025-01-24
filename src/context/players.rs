

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub pos: usize,
    pub score: u32,
    pub bin: Vec<String>,
    pub hand: Vec<String>,
}