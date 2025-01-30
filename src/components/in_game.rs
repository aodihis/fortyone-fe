use std::sync::mpsc::SendError;
use yew::{html, Component, Context, Html};
use crate::components::enemy::{Enemy, EnemyPos};

enum Phase {
    Start
}
pub struct InGame{
    phase: Phase,
}

impl Component for InGame{
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        Self {
            phase: Phase::Start
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let total_players = 4;
        let player_index = 2;
        let enemies = (0..total_players - 1)
            .map(|i| {
                let pos = match i {
                    0 => EnemyPos::Top,
                    1 => EnemyPos::Right,
                    _ => EnemyPos::Left,
                };

                let adjusted_index = if player_index > i { i } else { i + 1 };

                html! { <Enemy index={adjusted_index} pos={pos} /> }
            })
            .collect::<Vec<Html>>();

        html! {
            <div class="game-container">
                {enemies}
            </div>
        }
    }
}