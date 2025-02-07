use crate::context::game_state::{GameState, PlayerPhase};
use crate::utils::card_class;
use std::rc::Rc;
use yew::{classes, html, Component, Context, ContextHandle, Html};

pub enum Msg {
    StateChanged(Rc<GameState>),
}
pub struct CurrentPlayer {
    is_turn: bool,
    player_phase: PlayerPhase,
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

        let is_turn = state.player_turn_index == state.current_player_index;
        Self {
            is_turn,
            player_phase: {
                if is_turn {
                    state.player_turn_phase.clone()
                } else {
                    PlayerPhase::Waiting
                }
            },
            name: match state.current_player_name {
                Some(ref name) => name.clone(),
                None => String::from(""),
            },
            hand: state.players[state.current_player_index].hand.clone(),
            bin: state.players[state.current_player_index].bin.clone(),
            score: state.players[state.current_player_index].score,
            _listener,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StateChanged(state) => {
                let is_turn = state.player_turn_index == state.current_player_index;
                self.is_turn = is_turn;
                self.player_phase = {
                    if is_turn {
                        state.player_turn_phase.clone()
                    } else {
                        PlayerPhase::Waiting
                    }
                };
                self.hand = state.players[state.current_player_index].hand.clone();
                self.bin = state.players[state.current_player_index].bin.clone();
                self.score = state.players[state.current_player_index].score;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let last_five_bin: Vec<_> = self.bin.iter().rev().take(5).clone().collect();
        html!{
            <>
                <div class="current-player">
                    <div class="discard-pile bottom-discard">
                        {
                            last_five_bin.iter().rev().map(|x| {
                                let card_class = card_class(x);
                                html! {
                                    <div class={classes!("discard-card", card_class)}></div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

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
                    <div class="player-name"><span>{self.name.clone()}</span>
                    </div>
                </div>
            </>
        }
    }
}
