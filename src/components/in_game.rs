use std::rc::Rc;
use std::sync::mpsc::SendError;
use gloo_timers::future::TimeoutFuture;
use yew::{function_component, html, Component, Context, ContextHandle, Html, Properties};
use yew::platform::spawn_local;
use crate::components::current_player::CurrentPlayer;
use crate::components::enemy::{Enemy, EnemyPos};
use crate::context::game_state::GameState;

#[derive(PartialEq)]
enum Phase {
    Dealing,
    Sorting,
}

pub enum Msg {
    StateChanged(Rc<GameState>),
    PhaseChanged(Phase),
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
            phase: Phase::Sorting,
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
            Msg::PhaseChanged(phase) => {
                self.phase = phase;
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

        if self.phase == Phase::Dealing {
            let link = ctx.link().clone();
            let time = self.total_players as u32 * 4 * 1000;
            spawn_local( async move {
                TimeoutFuture::new(time).await;
                link.send_message(Msg::PhaseChanged(Phase::Sorting));
            });
        }

        html! {
            <>
                <Deck total_cards={self.card_left}/>
                {
                    if self.phase == Phase::Dealing {
                        html!{<CardDistribution total_players={total_players}/>}
                    } else {
                        html!{
                            <>
                                {for enemies}
                                <CurrentPlayer />
                            </>
                        }
                    }
                }


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

#[derive(Properties, Clone, PartialEq)]
pub struct CardDistributionProps {
    pub total_players: u8,
}
#[function_component]
pub fn CardDistribution(props: &CardDistributionProps) -> Html {
    let direction = ["throw-card-bottom", "throw-card-top", "throw-card-right", "throw-card-left"];
    html! {
        <>
            {
                (0..props.total_players*4).map(|i| {
                    let n : usize = i as usize % 4;
                    let dir = direction[n];
                    let style = format!("animation: {dir} 1s ease-in-out {i}s forwards;");
                    html!{
                        <div class="starting-card card card-back" style={style}></div>
                    }
                }).collect::<Html>()
            }
        </>
    }
}