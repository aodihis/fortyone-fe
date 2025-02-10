use crate::components::current_player::CurrentPlayer;
use crate::components::enemy::{Enemy, EnemyPos};
use crate::context::game_state::{GameState, PlayerPhase};
use crate::utils::card_class;
use gloo_timers::future::TimeoutFuture;
use web_sys::console::log_1;
use yew::platform::spawn_local;
use yew::{classes, function_component, html, use_context, Callback, Component, Context, ContextHandle, Html, Properties};

#[derive(PartialEq)]
enum Phase {
    Dealing,
    Sorting,
}

pub enum Msg {
    StateChanged(GameState),
    PhaseChanged(Phase),
    CardBinShow(Option<usize>)
}
pub struct InGame{
    bin_index: Option<usize>,
    phase: Phase,
    player_phase: PlayerPhase,
    total_players: u8,
    card_left: u8,
    current_player_index: usize,
    player_turn_index: usize,
    _listener: ContextHandle<GameState>
}

impl Component for InGame{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let (state, _listener) = ctx.link()
            .context::<GameState>(ctx.link().callback(Msg::StateChanged))
            .expect("context to be set");
        let game_data = state.game_data.borrow();
        Self {
            player_phase: game_data.player_turn_phase.clone(),
            bin_index: None,
            phase: Phase::Sorting,
            total_players: game_data.players.len() as u8,
            current_player_index: game_data.current_player_index,
            player_turn_index: game_data.player_turn_index,
            card_left: game_data.card_left,
            _listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                log_1(&"State Changed".into());
                let game_data = state.game_data.borrow();
                self.total_players = game_data.players.len() as u8;
                self.current_player_index = game_data.current_player_index;
                self.card_left = game_data.card_left;
               true
            }
            Msg::PhaseChanged(phase) => {
                self.phase = phase;
                true
            }
            Msg::CardBinShow(index) => {
                self.bin_index = index;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let total_players = self.total_players;
        let player_index = self.current_player_index;

        let link = ctx.link().clone();

        let on_bin_click = {
            let link = link.clone();
            Callback::from(move |index: usize| {

                web_sys::console::log_1(&index.into());
                link.send_message(Msg::CardBinShow(Some(index)));
        })};

        let enemies = (0..total_players - 1)
            .map(|i| {
                let pos = match i {
                    0 => EnemyPos::Top,
                    1 => EnemyPos::Right,
                    _ => EnemyPos::Left,
                };

                let adjusted_index = if player_index > i as usize { i } else { i + 1 };
                html! { <Enemy index={adjusted_index} pos={pos} on_bin_click={on_bin_click.clone()}/> }
            }).collect::<Vec<Html>>();

        if self.phase == Phase::Dealing {
            let link = link.clone();
            let time = self.total_players as u32 * 4 * 1000;
            spawn_local( async move {
                TimeoutFuture::new(time).await;
                link.send_message(Msg::PhaseChanged(Phase::Sorting));
            });
        }

        let onclose_bin = Callback::from(move |_| {
            link.send_message(Msg::CardBinShow(None));
        });

        let on_draw = {
            let is_player_draw = self.player_turn_index == self.current_player_index && self.player_phase == PlayerPhase::P1;
            Callback::from(move |_| {
                if is_player_draw {}
            })
        };

        html! {
            <>
                {
                    if self.bin_index.is_some() {
                        html!{<CardBinShowCase onclose={onclose_bin} bin_index={self.bin_index.unwrap()}/>}
                    } else {
                        html!{}
                    }
                }
                <Deck total_cards={self.card_left} on_draw={on_draw}/>
                {
                    if self.phase == Phase::Dealing {
                        html!{<CardDistribution total_players={total_players}/>}
                    } else {
                        html!{
                            <>
                                {for enemies}
                                <CurrentPlayer on_bin_click={on_bin_click} />
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
    pub on_draw: Callback<()>
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
    let on_click = props.on_draw.reform(|_|());
    html! {
        <div class="deck" onclick={on_click}>
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

#[derive(Properties, Clone, PartialEq)]
pub struct CardBinShowCaseProps {
    pub onclose: Callback<()>,
    pub bin_index: usize,
}

#[function_component]
pub fn CardBinShowCase(props: &CardBinShowCaseProps) -> Html {

    let game_state: GameState = use_context::<GameState>().unwrap();
    let game_data = game_state.game_data.borrow();
    let onclose = {
        let cb = props.onclose.clone();
        Callback::from(move |_| {
            cb.emit(())
        })
    };

    let bin_index = props.bin_index;
    let cards = game_data.players[bin_index].bin.iter().rev().map(|card| {
        let class = card_class(card);
        html!{
            <div class={classes!("card", class)}></div>
        }
    }).collect::<Vec<Html>>();

    html!{
         <div class="overlay" id="popup">
            <div class="card-popup-container">
                <button class="card-popup-close-button" onclick={onclose}>{"x"}</button>
                <div class="card-list-container">
                    { for cards }
                </div>
            </div>
        </div>
    }
}