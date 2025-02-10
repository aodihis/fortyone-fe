use crate::context::game_state::{GameState, PlayerPhase};
use crate::utils::card_class;
use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Context, ContextHandle, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct CurrentPlayerProps {
    pub on_bin_click : Callback<usize>,
}
pub enum Msg {
    StateChanged(GameState),
}
pub struct CurrentPlayer {
    index: usize,
    is_turn: bool,
    player_phase: PlayerPhase,
    name: String,
    hand: Vec<String>,
    bin: Vec<String>,
    _listener:ContextHandle<GameState>
}

impl Component for CurrentPlayer {
    type Message = Msg;
    type Properties = CurrentPlayerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (state, _listener) = ctx.link()
            .context::<GameState>(ctx.link().callback(Msg::StateChanged))
            .expect("context to be set");
        let game_data = state.game_data.borrow();
        let is_turn = game_data.player_turn_index == game_data.current_player_index;
        Self {
            index: game_data.current_player_index,
            is_turn,
            player_phase: {
                if is_turn {
                    game_data.player_turn_phase.clone()
                } else {
                    PlayerPhase::Waiting
                }
            },
            name: match game_data.current_player_name {
                Some(ref name) => name.clone(),
                None => String::from(""),
            },
            hand: game_data.players[game_data.current_player_index].hand.clone(),
            bin: game_data.players[game_data.current_player_index].bin.clone(),
            _listener,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                let game_data = state.game_data.borrow();
                let is_turn = game_data.player_turn_index == game_data.current_player_index;
                self.is_turn = is_turn;
                self.player_phase = {
                    if is_turn {
                        game_data.player_turn_phase.clone()
                    } else {
                        PlayerPhase::Waiting
                    }
                };
                self.hand = game_data.players[game_data.current_player_index].hand.clone();
                self.bin = game_data.players[game_data.current_player_index].bin.clone();
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
