#![recursion_limit = "512"]

mod components;
mod services;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::chat::Chat;
use components::login::Login;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type User = Rc<UserInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}

#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
        })
    });

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex w-screen h-screen">
                    <Switch<Route> render={Switch::render(switch)} />
                </div>
            </BrowserRouter>
        </ContextProvider<User>>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Login => html! {
            <div class="flex flex-col items-center w-full">
                <h2 class="text-2xl font-semibold text-center text-violet-700 mt-10">
                    {"✨ Welcome to YewChat – A Creative Chatroom ✨"}
                </h2>
                <Login />
            </div>
        },
        Route::Chat => html! {
            <Chat />
        },
        Route::NotFound => html! {
            <div class="flex justify-center items-center h-full w-full">
                <h1 class="text-3xl font-bold text-gray-700">{"404 - Page Not Found 🛰️"}</h1>
            </div>
        },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}
