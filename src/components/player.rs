use crate::context::game_state::{GameState, PlayerPhase};
use crate::utils::card_class;
use std::rc::Rc;
use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Context, ContextHandle, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct CurrentPlayerProps {
    pub on_bin_click : Callback<usize>,
}
pub enum Msg {
    StateChanged(Rc<GameState>),
}
pub struct ThePlayer {
    index: usize,
    is_turn: bool,
    player_phase: PlayerPhase,
    name: String,
    hand: Vec<String>,
    bin: Vec<String>,
    _listener:ContextHandle<Rc<GameState>>
}

impl Component for ThePlayer {
    type Message = Msg;
    type Properties = CurrentPlayerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (state, _listener) = ctx.link()
            .context::<Rc<GameState>>(ctx.link().callback(Msg::StateChanged))
            .expect("context to be set");
        let is_turn = state.current_turn_index == state.player_index;
        Self {
            index: state.player_index,
            is_turn,
            player_phase: {
                if is_turn {
                    state.current_turn_phase.clone()
                } else {
                    PlayerPhase::Waiting
                }
            },
            name: state.player_name.clone(),
            hand: state.players[state.player_index].hand.clone(),
            bin: state.players[state.player_index].bin.clone(),
            _listener,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                let is_turn = state.current_turn_index == state.player_index;
                self.is_turn = is_turn;
                self.player_phase = {
                    if is_turn {
                        state.current_turn_phase.clone()
                    } else {
                        PlayerPhase::Waiting
                    }
                };
                self.hand = state.players[state.player_index].hand.clone();
                self.bin = state.players[state.player_index].bin.clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let last_five_bin: Vec<_> = self.bin.iter().rev().take(5).clone().collect();
        let bin_click = {
            let index = self.index;
            ctx.props().on_bin_click.reform(move |_| index)
        };

        let is_draw_phase = self.is_turn && self.player_phase == PlayerPhase::P1;
        let is_discard_phase = self.is_turn && self.player_phase == PlayerPhase::P2;
        let on_take_bin = Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if is_draw_phase {

            }
        });

        let on_discard = Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if is_draw_phase {

            }
        });

        html!{
            <>
                <div class="current-player">
                    <div class="discard-pile bottom-discard" onclick={bin_click} oncontextmenu={on_take_bin}>
                        {
                            last_five_bin.iter().rev().map(|x| {
                                let card_class = card_class(x);
                                html! {
                                    <div class={classes!("discard-card", card_class)}></div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="game-info">
                        {
                            if !self.is_turn {
                                html! { "Waiting for the other player's turn!" }
                            } else if self.player_phase == PlayerPhase::P1 {
                                html! { <div><p>{ "Your turn to draw or take from the bin!" }</p><p>{"Click on deck or right click on bin"}</p></div> }
                            } else {
                                html!{ <div><p>{ "Remove a card from your hand!" }</p><p>{"Left click on card to discard or right click to end"}</p></div> }

                            }
                        }
                    </div>
                    <div class="player-area">
                        {
                            self.hand.iter().map(|h| {
                                let card_class = card_class(h);
                                html!{
                                    <div class={classes!("card", card_class)} oncontextmenu={on_discard.clone()}></div>
                                }
                            }).collect::<Html>()
                        }

                    </div>
                    <div class="player-name"><span>{self.name.clone()}</span>
                    </div>
                </div>
            </>
        }
    }
}
