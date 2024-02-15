use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub user_id: String,
    pub refresh: bool,
    pub on_logout: Callback<bool>,
    pub on_refresh: Callback<bool>,
}

#[function_component(NewsSaved)]
pub fn news_saved(props: &Props) -> Html {
    let saved_state = use_state(|| None::<Vec<Save>>);

    if props.refresh {
        saved_state.set(None);
        props.on_refresh.emit(true);
    }

    // Populate saved feed, either by pulling from local storage or pulling new from backend
    if saved_state.is_none() {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        let loaded = if let Ok(saved_res) = local_storage.get("saved") {
            if let Some(saved_str) = saved_res {
                let saved: Vec<Save> = serde_json::from_str(&saved_str).unwrap();
                if saved.len() > 0 {
                    saved_state.set(Some(saved));
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if !loaded {
            let saved_state = saved_state.clone();
            let on_logout = props.on_logout.clone();
            let user_id = props.user_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let uri_base = std::env!("SERVER_URI_BASE");
                let url = format!("{}/news/saved/{}", uri_base, user_id);
                let result = Request::get(&url)
                    .header("Content-Type", "application/json")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await;

                if let Ok(res) = result {
                    if res.status() == 200 {
                        // We good, deserialize data and set feed since we now have a feed
                        let saved = res.json::<Vec<Save>>().await.unwrap();
                        let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        let saved_str = serde_json::to_string(&saved).unwrap();
                        let _ = local_storage.set("saved", &saved_str);
                        saved_state.set(Some(saved));
                    } else {
                        // Not good, likely expired token/unauth so log out user
                        on_logout.emit(true);
                    }
                }
            })
        }
    }

    let handle_on_delete = {
        let saved_state = saved_state.clone();
        let on_logout = props.on_logout.clone();
        Callback::from(move |index: usize| {
            let saved_state = saved_state.clone();
            let on_logout = on_logout.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut saved = saved_state.deref().clone().unwrap();
                let save_str = serde_json::to_string(&saved[index]).unwrap();
                let uri_base = std::env!("SERVER_URI_BASE");
                let url = format!("{}/news/delete_saved", uri_base);
                let result = Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(JsValue::from_str(&save_str))
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await;

                if let Ok(res) = result {
                    if res.status() == 200 {
                        // Deleted, now update state
                        saved.remove(index);
                        let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        let saved_str = serde_json::to_string(&saved).unwrap();
                        let _ = local_storage.set("saved", &saved_str);
                        saved_state.set(Some(saved));
                    } else {
                        // Not good, likely expired token/unauth so log out user
                        on_logout.emit(true);
                    }
                }

                // send notification here
            })
        })
    };

    let news_cards_html = {
        if let Some(saved) = saved_state.deref() {
            saved
                .iter()
                .enumerate()
                .map(|(index, save)| {
                    html! {
                        <NewsCard article={save.article.clone()} article_index={index} saved={save.clone()} on_delete={&handle_on_delete} />
                    }
                })
                .collect::<Html>()
        } else {
            html! {}
        }
    };

    html! {
        <div class="col overflow-y fade-in">
            { news_cards_html }
        </div>
    }
}
