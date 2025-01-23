use yew::{function_component, html, Html, Properties};


#[derive(Properties, PartialEq, Clone)]
pub struct PreGameProps {

}
#[function_component]
pub fn PreGame(props: &PreGameProps) -> Html {
    html! {
        <div class="pre-game">
            <div class="options">
                <button>{"Create Game"}</button>
                <button>{"Join Game"}</button>
            </div>
        </div>
    }
}