use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| "".to_string());
    let user = use_context::<User>().expect("No context found.");

    // Generate avatar preview URL
    let avatar_url = if !username.is_empty() {
        format!("https://api.dicebear.com/7.x/bottts/svg?seed={}", *username)
    } else {
        "https://api.dicebear.com/7.x/bottts/svg?seed=guest".to_string()
    };

    let oninput = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone();
        })
    };

    html! {
        <div class="bg-gradient-to-br from-indigo-600 to-purple-700 flex items-center justify-center h-screen w-screen">
            <div class="bg-white p-8 rounded-xl shadow-xl max-w-md w-full">
                <h1 class="text-2xl font-bold mb-6 text-gray-800 text-center">{"🚀 Enter your name to start chatting!"}</h1>
                <form class="flex flex-col space-y-4 items-center">
                    <img src={avatar_url} class="w-24 h-24 rounded-full shadow-lg" alt="avatar preview" />
                    <input
                        type="text"
                        value={(*username).clone()}
                        oninput={oninput}
                        placeholder="Enter your username"
                        class="rounded-full px-5 py-2 border border-gray-300 text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500 w-full text-center"
                        required=true
                        maxlength="20"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            type="button"
                            class="bg-indigo-600 text-white px-6 py-2 rounded-full font-semibold hover:bg-indigo-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {"Join Chat"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
