use std::rc::Rc;
use yew::{function_component, html, use_context, use_state, Callback, Html, Properties};
use crate::context::game_state::GameState;
use crate::context::players::Player;

#[derive(Clone, PartialEq)]
enum PreGamePhase {
    Home,
    Waiting,
}

#[derive(Clone, PartialEq)]
enum Action {
    Join,
    Create,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PreGameProps {

}
#[function_component]
pub fn PreGame(props: &PreGameProps) -> Html {

    let phase = use_state(|| PreGamePhase::Waiting);
    let game_state: Rc<GameState> = use_context::<Rc<GameState>>().unwrap();

    let onclick = {
        let phase = phase.clone();
        move |action: Action| {
            Callback::from(move |_| {
                if action == Action::Join {
                    phase.set(PreGamePhase::Waiting);
                } else {
                    phase.set(PreGamePhase::Waiting);
                }
            })
        }
    };

    let join_onclick = {
        let onclick = onclick.clone();
        onclick(Action::Join)
    };
    let create_onclick = onclick(Action::Create);

    html! {
        <div class="pre-game">
            {
                if *phase == PreGamePhase::Home {
                    html! {
                        <div class="options">
                            <button onclick={create_onclick}>{"Create Game"}</button>
                            <button onclick={join_onclick}>{"Join Game"}</button>
                        </div>
                    }
                }else {
                    html!(<WaitingGame/>)
                }
            }
        </div>
    }
}

#[function_component]
pub fn WaitingGame() -> Html {
    let game_state: Rc<GameState> = use_context::<Rc<GameState>>().unwrap();
    let dummy_player = Player {
        name: "Kasandra".to_string(),
        pos: 0,
        score: 0,
        bin: vec![],
        hand: vec![],
    };
    let dummy = vec![dummy_player.clone(), dummy_player.clone(), dummy_player.clone(), dummy_player];

    let players = dummy.iter().map(|player|html!(<div class="wg-player">{player.name.clone()}</div>)).collect::<Html>();
    // let players = game_state.players.iter().map(|player|html!(<div class="wg-player">{player.name.clone()}</div>)).collect::<Html>();


    html! {
        <>
            <div class="waiting-game">
                <h3>{"Waiting for players"}</h3>
                <div class="wg-players">
                    {players}
                </div>
                {
                    if dummy.len() > 1 {
                        html! {<button class="">{"Start Game"}</button>}
                    } else {
                        html!{}
                    }
                }
                <button class="">{"Leave"}</button>

            </div>
        </>
    }
}