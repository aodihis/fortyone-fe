use std::rc::Rc;
use yew::{html, Component, Context, ContextHandle, Html, Properties};
use crate::context::game_state::GameState;



#[derive(Clone, PartialEq)]
pub enum EnemyPos {
    Left,
    Right,
    Top
}


pub enum Msg {
    StateChanged(Rc<GameState>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct EnemyProps {
    pub index: u8,
    pub pos: EnemyPos,
}
pub struct Enemy {
    index: u8,
    pos: EnemyPos,
    name: String,
    total_cards: u8,
    bin: Vec<String>,
    _listener: ContextHandle<Rc<GameState>>
}

impl Component for Enemy {
    type Message = Msg;
    type Properties = EnemyProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (state, _listener) = ctx.link()
            .context::<Rc<GameState>>(ctx.link().callback(Msg::StateChanged))
            .expect("context to be set");
        let index = ctx.props().index;
        let pos = ctx.props().pos.clone();
        Self{
            index,
            pos,
            name: state.players[index as usize].name.clone(),
            total_cards: state.players[index as usize].hand.len() as u8,
            bin: vec![],
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                let index = self.index;
                self.total_cards = state.players[index as usize].hand.len() as u8;
                self.bin = state.players[index as usize].bin.clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = match self.pos {
            EnemyPos::Left => {"left-enemy"}
            EnemyPos::Right => {"right-enemy"}
            EnemyPos::Top => {"top-enemy"}
        };

        let items = (0..self.total_cards).collect::<Vec<_>>();

        html! {
             <div class={class}>
                <div class="player-area">
                    {
                        items.iter().map(|_| html!{<div class="card card-back"></div>}).collect::<Html>()
                    }
                </div>
                <div class="player-name">{self.name.clone()}</div>
            </div>
        }

    }
}