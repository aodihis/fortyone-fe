use std::rc::Rc;
use yew::{classes, html, Component, Context, ContextHandle, Html};
use crate::context::game_state::GameState;
use crate::utils::card_class;
pub enum Msg {
    StateChanged(Rc<GameState>),
}
pub struct CurrentPlayer {
    name: String,
    hand: Vec<String>,
    bin: Vec<String>,
    score: i32,
    _listener:ContextHandle<Rc<GameState>>
}

impl Component for CurrentPlayer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (state, _listener) = ctx.link()
            .context::<Rc<GameState>>(ctx.link().callback(Msg::StateChanged))
            .expect("context to be set");

        Self {
            name: match state.player_name {
                Some(ref name) => name.clone(),
                None => String::from(""),
            },
            hand: state.players[state.current_player_index].hand.clone(),
            bin: state.players[state.current_player_index].bin.clone(),
            score: state.players[state.current_player_index].score,
            _listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html!{
            <div class="current-player">
                <div class="player-area">
                    {
                        self.hand.iter().map(|h| {
                            let card_class = card_class(h);
                            html!{
                                <div class={classes!("card", card_class)}></div>
                            }
                        }).collect::<Html>()
                    }

                </div>
                <div class="player-name">{self.name.clone()}</div>
            </div>
        }
    }
}
