use crate::components::in_game::InGame;
use crate::components::pre_game::PreGame;
use crate::context::game_state::{GameState, GameStatus};
use yew::{html, Component, Context, ContextProvider, Html};

pub struct Game {
    state_ref: GameState,
}

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        let game_state = GameState::new();
        Self {
            state_ref: game_state,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let state_ref = self.state_ref.clone();
        let binding = state_ref.clone().game_data;
        let game_data = binding.borrow();
        html! {
            <div class="game-container">
                <ContextProvider<GameState> context={state_ref}>
                    {
                        if game_data.game_status == GameStatus::PreGame {
                            html!{<PreGame/>}
                        } else if game_data.game_status == GameStatus::InProgress {
                            html!{<InGame/>}
                        } else {html!{}}
                    }


                </ContextProvider<GameState>>

            </div>
        }
    }
}