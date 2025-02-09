use crate::context::game_state::GameState;
use gloo_timers::future::TimeoutFuture;
use web_sys::console::log_1;
use web_sys::wasm_bindgen::JsCast;
use web_sys::window;
use yew::platform::spawn_local;
use yew::{function_component, html, props, use_context, use_state, Callback, Html, Properties, SubmitEvent};

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
pub fn PreGame(_: &PreGameProps) -> Html {

    let phase = use_state(|| PreGamePhase::Home);
    // let _game_state: Rc<GameState> = use_context::<Rc<GameState>>().unwrap();

    let onclick = {
        let phase = phase.clone();
        move |action: Action| {
            Callback::from(move |_| {
                if action == Action::Join {
                    phase.set(PreGamePhase::Join);
                } else {
                    phase.set(PreGamePhase::Create);
                }
            })
        }
    };

    let join_onclick = {
        let onclick = onclick.clone();
        onclick(Action::Join)
    };
    let create_onclick = onclick(Action::Create);

    let create_callback = {
        let phase = phase.clone();
        Callback::from(move |_| {
            phase.set(PreGamePhase::Waiting);
    })};
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
                    html!(<CreateGame callback={create_callback}/>)
                } else{
                    html!(<JoinGame/>)
                }
            }
        </div>
    }
}

#[function_component]
pub fn JoinGame() -> Html {
    log_1(&"join".into());
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

#[derive(Properties, PartialEq, Clone)]
pub struct CreateGameProps {
    callback: Callback<()>,
}
#[function_component]
pub fn CreateGame(props: &CreateGameProps) -> Html {
    web_sys::console::log_1(&"CreateGame".into());
    let game_state: GameState = use_context::<GameState>().unwrap();
    let callback = props.callback.clone();
    let onsubmit = {
        let game_state = game_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let target = e.target();
            let form = target.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok()).expect("Couldn't get HtmlFormElement");
            let name_element = form.get_with_name("name").and_then(|name| name.dyn_into::<web_sys::HtmlInputElement>().ok()).unwrap();
            let _name = name_element.value();
            let mut game_state = game_state.clone();
            let callback = callback.clone();

            spawn_local(async move {
                match game_state.create_game().await {
                    Ok(_) => {
                        log_1(&"connect".into());
                        
                    }
                    Err(_) => {window().unwrap().alert_with_message("Failed to connect!").unwrap();}
                }

                callback.emit(());
                game_state.join_game().await.expect("TODO: panic message");

            });
        })
    };

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
    log_1(&"waiting".into());
    let game_state: GameState = use_context::<GameState>().unwrap();
    let copy_button_label = use_state(|| "📋");
    let game_data = game_state.game_data.borrow();
    let game_id = match &game_data.game_id {
        Some(game_id) => game_id.clone(),
        _ => return html!(),
    };

    let players = game_data.players.iter().map(|player|html!(<div class="wg-player">{player.name.clone()}</div>)).collect::<Html>();
    // let players = game_state.players.iter().map(|player|html!(<div class="wg-player">{player.name.clone()}</div>)).collect::<Html>();

    let copy_id = {
        let game_id = game_id.clone();
        let copy_button_label = copy_button_label.clone();
        Callback::from(move |_| {
        if let Some(window) = window() {
            let _ = window.navigator().clipboard().write_text(&*game_id);
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
                        if game_data.players.len() > 1 {
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