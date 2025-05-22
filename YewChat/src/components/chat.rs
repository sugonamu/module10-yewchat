use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://i.pravatar.cc/150?u={}",
                                    u
                                )
                                    .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let my_name = self.users.get(0).map(|u| u.name.clone()).unwrap_or_default(); // Replace with actual current user

        html! {
            <div class="flex w-screen">
                <div class="flex-none w-56 h-screen bg-gray-100">
                    <div class="text-xl p-3">{"Users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-white rounded-lg p-2 items-center">
                                    <div class="relative">
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                        <span class="online-dot"></span>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-gray-400">
                                            {"Online"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-14 border-b-2 border-gray-300 flex items-center">
                        <div class="text-xl p-3">{"💬 Chat!"}</div>
                    </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300 flex flex-col p-4 space-y-2">
                        {
                            self.messages.iter().map(|m| {
                                let is_me = m.from == my_name;
                                let user = self.users.iter().find(|u| u.name == m.from);
                                let avatar = user.map(|u| u.avatar.clone()).unwrap_or_default();
                                html!{
                                    <div class={format!("flex items-end {}", if is_me { "justify-end" } else { "justify-start" })}>
                                        { if !is_me {
                                            html! { <img class="w-8 h-8 rounded-full mr-2" src={avatar.clone()} alt="avatar"/> }
                                        } else { html! {} } }
                                        <div class={format!("chat-bubble {}", if is_me { "me" } else { "other" })}>
                                            <div class="text-xs text-gray-500 mb-1">{&m.from}</div>
                                            {
                                                if m.message.ends_with(".gif") {
                                                    html! { <img class="mt-1" src={m.message.clone()} /> }
                                                } else {
                                                    html! { {m.message.clone()} }
                                                }
                                            }
                                        </div>
                                        { if is_me {
                                            html! { <img class="w-8 h-8 rounded-full ml-2" src={avatar} alt="avatar"/> }
                                        } else { html! {} } }
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                        // Typing indicator (show conditionally in real app)
                        <div class="typing-indicator mt-2">
                            <span class="typing-dot"></span>
                            <span class="typing-dot"></span>
                            <span class="typing-dot"></span>
                            <span class="ml-2 text-xs text-gray-500">{"Someone is typing..."}</span>
                        </div>
                    </div>
                    <div class="w-full h-14 flex px-3 items-center">
                        <button class="mr-2" title="Add emoji" onclick={Callback::from(|_| {
                            // Show emoji picker via JS interop or Yew logic
                            if let Some(picker) = web_sys::window()
                                .and_then(|w| w.document())
                                .and_then(|d| d.get_element_by_id("emoji-picker")) {
                                    picker.set_attribute("style", "position: fixed; bottom: 80px; right: 40px; display: block;").ok();
                            }
                        })}>{"😊"}</button>
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-full outline-none focus:text-gray-700" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center color-white">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}