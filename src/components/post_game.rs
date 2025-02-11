use std::rc::Rc;
use yew::{html, Properties, Html, function_component, use_context, classes};
use crate::context::game_state::GameState;
use crate::utils::card_class;

#[derive(PartialEq, Properties)]
pub struct Props {
}

#[function_component]
pub fn PostGame(_: &Props) -> Html {
    let game_state: Rc<GameState> = use_context::<Rc<GameState>>().unwrap();

    let winner = game_state.winner.clone();


    html! {
        <div class="overlay" id="post-game-overlay">
            <div class="post-game">
                <h2>{"Game Over"}</h2>
                <table>
                <colgroup>
                    <col style="width: 30%;"/>
                    <col style="width: 50%;"/>
                    <col style="width: 20%;"/>
                </colgroup>
                    <thead>
                        <tr><th>{"Player"}</th><th>{"Cards"}</th><th>{"Score"}</th></tr>
                    </thead>
                    <tbody>
                        {
                            game_state.players.iter().map(|player| {
                                html!{
                                    <tr>
                                        <td>{player.name.clone()}</td>
                                        <td>
                                            <div class="cards">
                                                {
                                                    player.hand.iter().map(|card| {
                                                        let class = card_class(card);
                                                        html!{
                                                            <div class={classes!("card", class)}></div>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                            </div>
                                        </td>
                                        <td>{player.score}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
                {
                    if let Some(winner) = winner {
                        html!{<h3 class="winner-name">{format!{"Winner: {}", winner}}</h3>}
                    } else {
                        html!{}
                    }
                }
            </div>
        </div>
    }
}