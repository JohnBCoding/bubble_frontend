use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let logged_in_state = use_state(|| None::<User>);

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

    if logged_in_state.is_some() {
        wasm_bindgen_futures::spawn_local(async move {
            let uri_base = std::env!("SERVER_URI_BASE");
            let url = format!("{}/news/feed", uri_base);
            let result = Request::get(&url)
                .header("Content-Type", "application/json")
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await;
        })
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

    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <NavBar />
            if logged_in_state.is_none() {
                <Login on_set_login={&handle_set_login}/>
            } else {
                <div class="col overflow-y">
                    <NewsCard />
                    <NewsCard />
                    <NewsCard />
                    <NewsCard />
                    <NewsCard />
                    <NewsCard />
                    <NewsCard />
                </div>
            }
        </main>
    }
}
