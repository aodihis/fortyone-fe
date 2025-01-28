use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast;
use web_sys::window;
use yew::{function_component, html, use_context, use_state, Callback, Html, Properties, SubmitEvent};
use yew::platform::spawn_local;
use crate::context::game_state::GameState;
use crate::context::players::Player;
use gloo_timers::future::TimeoutFuture;

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
                } else if *phase == PreGamePhase::Waiting {
                    html!(<WaitingGame/>)
                } else if *phase == PreGamePhase::Create {
                    html!(<CreateGame/>)
                } else{
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
        let name_element = form.get_with_name("name").and_then(|name| name.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();
        let game_id_element = form.get_with_name("game-id").and_then(|name| name.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();

        let name = name_element.value();
        let game_id = game_id_element.value();

        if game_id.is_empty() || name.is_empty() {
            if let Some(window) = window() {
                window.alert_with_message("Please, input correct game id.").expect("Window is not ready");
            }
            return;
        }
    });
    html! {
        <div class="game-form">
            <form onsubmit={onsubmit}>
                <input name="game-id" type="text" placeholder="Please, input game id"/>
                <input name="name" type="text" placeholder="Please, input your name"/>
                <button type="submit">{"Join Game"}</button>
            </form>
        </div>
    }
}

#[function_component]
pub fn CreateGame() -> Html {

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let target = e.target();
        let form = target.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok()).expect("Couldn't get HtmlFormElement");
        let name_element = form.get_with_name("name").and_then(|name| name.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();
        let name = name_element.value();

    });

    html! {
        <div class="game-form">
            <form onsubmit={onsubmit}>
                <input name="name" type="text" placeholder="Please, input your name"/>
                <button type="submit">{"Join Game"}</button>
            </form>
        </div>
    }
}

#[function_component]
pub fn WaitingGame() -> Html {
    let game_state: Rc<GameState> = use_context::<Rc<GameState>>().unwrap();
    let copy_button_label = use_state(|| "📋");

    let game_id = "U23rads".to_string();
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

    let copy_id = {
        let game_id = game_id.clone();
        let copy_button_label = copy_button_label.clone();
        Callback::from(move |_| {
        if let Some(window) = window() {
            window.navigator().clipboard().write_text(&*game_id);
            copy_button_label.set("✅");
            {
                let copy_button_label = copy_button_label.clone();
                spawn_local(async move {
                    TimeoutFuture::new(1000).await; // Wait 2 seconds
                    copy_button_label.set("📋");
                });
            }
        }
    })};
    html! {
        <>
            <div class="waiting-game">
                <div class="box">
                        <h3>{"Waiting for players"}</h3>
                        <div class="wg-players">
                            {players}
                        </div>
                </div>
                <div class="box">
                    <div class="game-id-container">
                        <span class="game-id" id="game-id">{game_id}</span>
                        <button class="copy-id" title="Copy" onclick={copy_id}>
                           { *copy_button_label }
                        </button>
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
            </div>
        </>
    }
}