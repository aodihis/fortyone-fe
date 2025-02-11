use crate::components::in_game::InGame;
use crate::components::pre_game::PreGame;
use crate::context::game_state::{GameState, GameStatus};
use crate::models::api_data::{GameResponse, MessageType};
use crate::models::players::Player;
use crate::services::connection::{create_game, join_game};
use futures_util::future::{AbortHandle, Abortable};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;
use web_sys::window;
use yew::platform::spawn_local;
use yew::{html, Component, Context, ContextProvider, Html};

pub struct Game {
    state_ref: Rc<GameState>,
    writer: Rc<RefCell<Option<SplitSink<WebSocket, Message>>>>,
    reader_abort: Rc<RefCell<Option<AbortHandle>>>,
}

pub enum Msg {
    CreateGame(String),
    Disconnect,
    JoinGame(String, String),
    Listener(SplitStream<WebSocket>),
    GameJoined(String),
    GameUpdate(GameResponse),
}
impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let create_game = ctx.link().callback(|name| Msg::CreateGame(name));
        let join_game = ctx.link().callback(|(game_id, name)| Msg::JoinGame(game_id, name));
        let disconnect = ctx.link().callback(|_| Msg::Disconnect);
        let game_state = Rc::new(GameState::new(create_game, join_game, disconnect));

        Self {
            state_ref: game_state,
            writer: Rc::new(RefCell::new(None)),
            reader_abort: Rc::new(RefCell::new(None)),
        }
    }

     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateGame(name) => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    let game_id = create_game().await.expect("Failed to create Game");
                    link.send_message(Msg::JoinGame(game_id, name));
                });
                false
            },
            Msg::JoinGame(game_id, name) => {
                let link = ctx.link().clone();
                let wr = self.writer.clone();
                spawn_local(async move {
                    let socket = join_game(&game_id, &name);
                    match socket {
                        Ok(socket) => {
                            let (writer, reader) = socket.split();
                            *wr.borrow_mut() = Some(writer);
                            link.send_message(Msg::GameJoined(game_id));
                            link.send_message(Msg::Listener(reader));
                        }
                        Err(_) => {
                            if let Some(window) = window() {
                                window
                                    .alert_with_message("Connection error!")
                                    .unwrap();
                            }
                        }
                    }
                });
                false
            },
            Msg::GameJoined(game_id) => {
                Rc::make_mut(&mut self.state_ref).game_id = Some(game_id);
                Rc::make_mut(&mut self.state_ref).counter += 1;
                true
            },
            Msg::Listener(mut reader) => {
                let link = ctx.link().clone();
                let (abort_handle_inner, abort_registration) = AbortHandle::new_pair();
                *self.reader_abort.borrow_mut() = Some(abort_handle_inner.clone());

                let task = async move {
                    while let Some(msg) = reader.next().await {
                        match msg {
                            Ok(Message::Text(message)) => {
                                let game_response = serde_json::from_str::<GameResponse>(message.as_str()).unwrap();
                                link.send_message(Msg::GameUpdate(game_response));
                            }
                            Ok(Message::Bytes(_)) => {}
                            Err(_) => {
                                link.send_message(Msg::Disconnect);
                            }
                        };
                    }
                };

                let abort_task = Abortable::new(task, abort_registration);
                spawn_local(async move {
                    let _ = abort_task.await;
                });
                false
            },
            Msg::GameUpdate(response) => {
                log_1(&"Updating".into());
                if response.status != "success" {
                    return false;
                }
                match response.message_type {
                    MessageType::PlayerJoin => {
                        Rc::make_mut(&mut self.state_ref).players = response.data.unwrap().players.iter().map(|player|{
                            Player {
                                name: player.name.clone(),
                                bin: vec![],
                                hand: vec![],
                            }
                        }).collect();
                    },
                    MessageType::PlayerLeft => {
                        Rc::make_mut(&mut self.state_ref).players = response.data.unwrap().players.iter().map(|player|{
                            Player {
                                name: player.name.clone(),
                                bin: vec![],
                                hand: vec![],
                            }
                        }).collect();
                    },
                    _ => {}
                }
                Rc::make_mut(&mut self.state_ref).counter += 1;
                true
            }
            Msg::Disconnect => {
                Rc::make_mut(&mut self.state_ref).clear();
                if let Some(handle) = self.reader_abort.borrow_mut().take() {
                    handle.abort();
                }
                let wr = self.writer.borrow_mut().take();
                spawn_local(async move {
                    if let Some(mut writer) = wr {
                        writer.close().await.unwrap();
                    }
                });
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log_1(&"Rpm".into());
        let game_data = self.state_ref.clone();
        html! {
            <div class="game-container">
                <ContextProvider<Rc<GameState>> context={game_data.clone()}>
                    {
                        if game_data.game_status == GameStatus::PreGame {
                            html!{<PreGame/>}
                        } else if game_data.game_status == GameStatus::InProgress {
                            html!{<InGame/>}
                        } else {html!{}}
                    }

                </ContextProvider<Rc<GameState>>>

            </div>
        }
    }
}