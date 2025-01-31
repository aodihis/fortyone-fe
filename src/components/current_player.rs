use std::rc::Rc;
use yew::{Component, Context, ContextHandle, Html};
use crate::context::game_state::GameState;

pub enum Msg {
    StateChanged(Rc<GameState>),
}
pub struct CurrentPlayer {
    name: String,
    hand: Vec<String>,
    bin: Vec<String>,
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
            name: "".to_string(),
            hand: state.players[state.current_player_index].hand.clone(),
            bin: state.players[state.current_player_index].bin.clone(),
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
        todo!()
    }
}
