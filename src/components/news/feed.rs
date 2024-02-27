use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub user_id: String,
    pub refresh: bool,
    pub on_logout: Callback<bool>,
    pub on_refresh: Callback<bool>,
    pub on_update_alert_text: Callback<String>,
}

#[function_component(NewsFeed)]
pub fn news_feed(props: &Props) -> Html {
    let feed_state = use_state(|| None::<Feed>);

    if props.refresh {
        feed_state.set(None);
        props.on_refresh.emit(true);
    }

    // Populate feed, either by pulling from local storage or pulling new from backend
    if feed_state.is_none() {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        let loaded = if let Ok(feed_res) = local_storage.get("feed") {
            if let Some(feed_str) = feed_res {
                let feed = serde_json::from_str(&feed_str).unwrap();
                feed_state.set(Some(feed));
                true
            } else {
                false
            }
        } else {
            false
        };

        if !loaded {
            let feed_state = feed_state.clone();
            let on_logout = props.on_logout.clone();
            let on_update_alert_text = props.on_update_alert_text.clone();
            let user_id = props.user_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let uri_base = std::env!("SERVER_URI_BASE");
                let url = format!("{}/news/feed/{}", uri_base, user_id);
                let result = Request::get(&url)
                    .header("Content-Type", "application/json")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await;

                if let Ok(res) = result {
                    if res.status() == 200 {
                        // We good, deserialize data and set feed since we now have a feed
                        let feed = res.json::<Feed>().await.unwrap();
                        let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        let user_str = serde_json::to_string(&feed).unwrap();
                        let _ = local_storage.set("feed", &user_str);
                        
                        // Send alert if feed wasn't updated due to refresh cooldown
                        if let Some(cooldown) = feed.refresh_cooldown {
                            on_update_alert_text.emit(format!("Refresh Cooldown ({})", cooldown));
                        }
                        feed_state.set(Some(feed));
                    } else if res.status() == 429 { // Limited
                         // Put rate limited notification here
                    } else {
                        // Not good, likely expired token/unauth so log out user
                        on_logout.emit(true);
                    }
                }
            })
        }
    }

    let handle_on_save = {
        let feed_state = feed_state.clone();
        Callback::from(move |article_index: usize| {
            let feed_state = feed_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut feed = feed_state.deref().clone().unwrap();
                let mut articles = feed.data;
                let window = web_sys::window().unwrap();
                let local_storage = window.local_storage().unwrap().unwrap();
                if let Ok(user_res) = local_storage.get("user") {
                    if let Some(user_str) = user_res {
                        let user = serde_json::from_str::<User>(&user_str).unwrap();
                        let save = Save::new(&user.user_id, &articles[article_index]);
                        let save_str = serde_json::to_string(&save).unwrap();
                        let uri_base = std::env!("SERVER_URI_BASE");
                        let url = format!("{}/news/save", uri_base);
                        let result = Request::post(&url)
                            .header("Content-Type", "application/json")
                            .body(JsValue::from_str(&save_str))
                            .credentials(web_sys::RequestCredentials::Include)
                            .send()
                            .await;

                        if let Ok(res) = result {
                            if res.status() == 200 {
                                // We good
                                let saved_str = serde_json::to_string(&Vec::<Save>::new()).unwrap();
                                let _ = local_storage.set("saved", &saved_str);

                                // delete article from feed, resave feed
                                articles.remove(article_index);
                                feed.data = articles;
                                let feed_str = serde_json::to_string(&feed).unwrap();
                                let _ = local_storage.set("feed", &feed_str);
                                feed_state.set(Some(feed));
                            }
                        }
                        // send notification here
                    }
                }
            })
        })
    };

    let handle_on_rate = {
        let feed_state = feed_state.clone();
        Callback::from(move |(article_index, like)| {
            let feed_state = feed_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let feed = feed_state.deref().clone().unwrap();
                let articles = feed.data;
                let article_str = serde_json::to_string(&articles[article_index]).unwrap();
                let window = web_sys::window().unwrap();
                let local_storage = window.local_storage().unwrap().unwrap();
                if let Ok(user_res) = local_storage.get("user") {
                    if let Some(user_str) = user_res {
                        let user = serde_json::from_str::<User>(&user_str).unwrap();
                        let uri_base = std::env!("SERVER_URI_BASE");
                        let url = format!(
                            "{}/news/rate/{}/{}",
                            uri_base,
                            user.user_id,
                            if like { "like" } else { "dislike" }
                        );
                        let result = Request::post(&url)
                            .header("Content-Type", "application/json")
                            .body(JsValue::from_str(&article_str))
                            .credentials(web_sys::RequestCredentials::Include)
                            .send()
                            .await;

                        if let Ok(res) = result {
                            if res.status() == 200 {
                                // We good

                                // Remove article from state
                                let mut feed = feed_state.deref().clone().unwrap();
                                let mut articles = feed.data;
                                articles.remove(article_index);
                                feed.data = articles;

                                // Remove from storage
                                let feed_str = serde_json::to_string(&feed).unwrap();
                                let _ = local_storage.set("feed", &feed_str);

                                // Set new state
                                feed_state.set(Some(feed));
                            }
                        }
                        // send notification here
                    }
                }
            })
        })
    };

    let handle_on_delete = { Callback::from(move |_index: usize| {}) };

    let news_cards_html = {
        if let Some(feed) = feed_state.deref() {
            feed.data
                .iter()
                .enumerate()
                .map(|(index, article)| {
                    html! {
                        <NewsCard article={article.clone()} article_index={index} saved={None} on_save={&handle_on_save} on_rate={&handle_on_rate} on_delete={&handle_on_delete} />
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
