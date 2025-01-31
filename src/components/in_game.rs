use std::rc::Rc;
use std::sync::mpsc::SendError;
use yew::{function_component, html, Component, Context, ContextHandle, Html, Properties};
use crate::components::enemy::{Enemy, EnemyPos};
use crate::context::game_state::GameState;

enum Phase {
    Start
}

pub enum Msg {
    StateChanged(Rc<GameState>),
}
pub struct InGame{
    phase: Phase,
    total_players: u8,
    card_left: u8,
    current_player_index: usize,
    _listener: ContextHandle<Rc<GameState>>
}

impl Component for InGame{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let (state, _listener) = ctx.link()
            .context::<Rc<GameState>>(ctx.link().callback(Msg::StateChanged))
            .expect("context to be set");

        Self {
            phase: Phase::Start,
            total_players: state.players.len() as u8,
            current_player_index: state.current_player_index,
            card_left: state.card_left,
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                self.total_players = state.players.len() as u8;
                self.current_player_index = state.current_player_index;
                self.card_left = state.card_left;
               true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let total_players = self.total_players;
        let player_index = self.current_player_index;
        let enemies = (0..total_players - 1)
            .map(|i| {
                let pos = match i {
                    0 => EnemyPos::Top,
                    1 => EnemyPos::Right,
                    _ => EnemyPos::Left,
                };

                let adjusted_index = if player_index > i as usize { i } else { i + 1 };

                html! { <Enemy index={adjusted_index} pos={pos} /> }
            }).collect::<Vec<Html>>();

        html! {
            <>
                <Deck total_cards={self.card_left}/>
                {for enemies}
            </>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct DeckProps {
    pub total_cards: u8,
}
#[function_component]
pub fn Deck(props: &DeckProps) -> Html {
    let cards = (0..props.total_cards).map(|i| {
            let tr = i/4;
            let style = format!("transform: translate(-{tr}px, -{tr}px);");
            html!{
                <div class="card card-back" style={style}>
                </div>
            }
        }).collect::<Vec<Html>>();
    html! {
        <div class="deck">
            { for cards }
        </div>
    }
}