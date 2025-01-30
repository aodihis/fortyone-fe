use std::rc::Rc;
use yew::{html, Component, Context, ContextProvider, Html};
use crate::components::pre_game::PreGame;
use crate::context::game_state::{GameState, GameStatus};



pub struct Game {
    game_state: Rc<GameState>,
}

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game_state = Rc::new(GameState {
            game_status: GameStatus::NotStarted,
            game_id: None,
            player_name: None,
            players: vec![],
            event: None,
            counter: 0,
        });

        Self {
            game_state,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let game_state = self.game_state.clone();

        html! {
            <div class="game-container">
                <ContextProvider<Rc<GameState>> context={game_state}>
                    <PreGame/>

                </ContextProvider<Rc<GameState>>>

            </div>
        }
    }
}