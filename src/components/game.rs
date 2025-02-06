use std::rc::Rc;
use yew::{html, Component, Context, ContextProvider, Html};
use crate::components::in_game::InGame;
use crate::components::pre_game::PreGame;
use crate::context::game_state::{GamePhase, GameState, GameStatus};
use crate::context::players::Player;

pub struct Game {
    game_state: Rc<GameState>,
}

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dummy_players = vec![Player{
            name: "P1".to_string(),
            score: 0,
            bin: vec![],
            hand: vec!["HA".to_string(),"S3".to_string(),"D10".to_string(),"C2".to_string()],
        },Player{
            name: "P2".to_string(),
            score: 0,
            bin: vec![],
            hand: vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
        },Player{
            name: "P3".to_string(),
            score: 0,
            bin: vec![],
            hand: vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
        },Player{
            name: "P4".to_string(),
            score: 0,
            bin: vec![],
            hand: vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()],
        }];
        let game_state = Rc::new(GameState {
            game_status: GameStatus::InProgress,
            game_id: None,
            card_left: 52,
            current_player_index: 0,
            current_turn_index: 0,
            current_turn_phase: GamePhase::P1,
            player_name: Some(String::from("Mia")),
            players: dummy_players,
            event: None,
            counter: 0,
        });

        Self {
            game_state,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let game_state = self.game_state.clone();

        html! {
            <div class="game-container">
                <ContextProvider<Rc<GameState>> context={game_state}>
                    {
                        if self.game_state.game_status == GameStatus::PreGame {
                            html!{<PreGame/>}
                        } else if self.game_state.game_status == GameStatus::InProgress {
                            html!{<InGame/>}
                        } else {html!{}}
                    }


                </ContextProvider<Rc<GameState>>>

            </div>
        }
    }
}