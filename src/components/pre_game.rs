use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;
use yew::{function_component, html, use_context, use_state, Callback, Html, Properties, SubmitEvent};
use crate::context::game_state::GameState;
use crate::context::players::Player;

#[derive(Clone, PartialEq)]
enum PreGamePhase {
    Home,
    Waiting,
    Join,
    Create
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

    let phase = use_state(|| PreGamePhase::Join);
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
                } else if *phase == PreGamePhase::Waiting {
                    html!(<WaitingGame/>)
                } else {
                    html!(<JoinGame/>)
                }
            }
        </div>
    }
}

#[function_component]
pub fn JoinGame() -> Html {

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let target = e.target();
        let form = target.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok()).expect("Couldn't get HtmlFormElement");
        // let game_id = match form.get_with_name("game-id").and_then(|id| id.value_of()) {
        //     Some(id) => id,
        //     None => {return }
        // };
        let name = form.get_with_name("name").and_then(|name| Some(name.to_string()));
        // web_sys::console::log_1(&game_id.into());
        web_sys::console::log_1(&name.into());


    });
    html! {
        <div class="join-game">
            <form onsubmit={onsubmit}>
                <input name="game-id" type="text" placeholder="Please input game id"/>
                <input name="name" type="text" placeholder="Please your input name"/>
                <button type="submit">{"Join Game"}</button>
            </form>
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