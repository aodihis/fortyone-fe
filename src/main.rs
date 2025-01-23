mod components;
mod context;

use yew::prelude::*;
use crate::components::game::Game;

#[function_component]
fn App() -> Html {

    html! {
        <Game/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}