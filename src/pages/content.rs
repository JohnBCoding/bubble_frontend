use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let logged_in_state = use_state(|| None::<User>);
    let page_state = use_state(|| PageState::Feed);

    // Load user if state is none
    if logged_in_state.is_none() {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        if let Ok(result) = local_storage.get("user") {
            if let Some(user_str) = result {
                let user = serde_json::from_str(&user_str).unwrap();
                logged_in_state.set(Some(user));
            }
        }
    }

    // If logged in, get news feed
    if let Some(user) = logged_in_state.deref() {
        if user.feed.is_none() {
            let logged_in_state = logged_in_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let uri_base = std::env!("SERVER_URI_BASE");
                let url = format!("{}/news/feed", uri_base);
                let result = Request::get(&url)
                    .header("Content-Type", "application/json")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await;

                if let Ok(res) = result {
                    if res.status() == 200 {
                        // We good, deserialize data and set feed/save updated user since we now have a feed
                        let feed = res.json::<Feed>().await.unwrap();
                        let mut user = logged_in_state.deref().clone().unwrap();
                        user.feed = Some(feed);
                        let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        let user_str = serde_json::to_string(&user).unwrap();
                        local_storage.set("user", &user_str);
                        logged_in_state.set(Some(user));
                    } else {
                        // Not good, likely expired token/unauth so log out user
                        logged_in_state.set(None);
                        let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        local_storage.clear();
                    }
                }
            })
        }
    };

    let handle_set_login = {
        let logged_in_state = logged_in_state.clone();
        Callback::from(move |user: User| {
            let window = web_sys::window().unwrap();
            let local_storage = window.local_storage().unwrap().unwrap();
            local_storage.set("user", &serde_json::to_string(&user).unwrap());
            logged_in_state.set(Some(user));
        })
    };

    let handle_on_change_state = {
        let page_state = page_state.clone();
        Callback::from(move |new_state: PageState| {
            page_state.set(new_state);
        })
    };

    let handle_on_refresh = {
        let logged_in_state = logged_in_state.clone();
        Callback::from(move |event: MouseEvent| {
            let mut user = logged_in_state.deref().clone().unwrap();
            user.feed = None;

            logged_in_state.set(Some(user));
        })
    };

    let news_cards_html = {
        if let Some(user) = logged_in_state.deref().clone() {
            if let Some(feed) = user.feed {
                feed.data
                    .iter()
                    .map(|article| {
                        html! {
                            <NewsCard article={article.clone()}/>
                        }
                    })
                    .collect::<Html>()
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    };

    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <NavBar page_state={page_state.deref().clone()} on_refresh={&handle_on_refresh} on_change_state={&handle_on_change_state}/>
            if logged_in_state.is_none() {
                <Login on_set_login={&handle_set_login}/>
            } else {
                {
                    match *page_state {
                        PageState::Feed => {
                            html! {
                            <div class="col overflow-y fade-in">
                                {news_cards_html}
                            </div>}
                        }
                        PageState::Saved => {
                            html!{<SavedNews saved={logged_in_state.deref().clone().unwrap().saved} />}
                        }
                    }
                }
            }
        </main>
    }
}
