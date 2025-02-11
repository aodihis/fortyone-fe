use std::cell::RefCell;
use std::rc::Rc;
use crate::components::in_game::InGame;
use crate::components::pre_game::PreGame;
use crate::context::game_state::{GameState, GameStatus};
use yew::{html, Callback, Component, Context, ContextProvider, Html, Properties};
use crate::services::connection::create_game;

pub struct Game {
    state_ref: Rc<RefCell<GameState>>,
}

pub enum Msg {
    CreateGame,
    JoinGame(String, String),
}
impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let create_game = ctx.link().callback(|_| Msg::CreateGame);
        let join_game = ctx.link().callback(|(game_id, name)| Msg::JoinGame(game_id, name));
        let game_state = Rc::new(RefCell::new(GameState::new(create_game, join_game)));

        Self {
            state_ref: game_state,
        }
    }

     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateGame => {
                // let res = create_game();
                true
            },
            Msg::JoinGame(game_id, name) => {

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let state_ref = self.state_ref.clone();
        let binding = state_ref.clone();
        let game_data = binding.borrow();
        html! {
            <div class="game-container">
                <ContextProvider<Rc<RefCell<GameState>>> context={state_ref}>
                    {
                        if game_data.game_status == GameStatus::PreGame {
                            html!{<PreGame/>}
                        } else if game_data.game_status == GameStatus::InProgress {
                            html!{<InGame/>}
                        } else {html!{}}
                    }


                </ContextProvider<Rc<RefCell<GameState>>>>

            </div>
        }
    }
}