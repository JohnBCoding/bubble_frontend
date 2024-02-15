use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub user_id: String,
    pub refresh: bool,
    pub on_logout: Callback<bool>,
    pub on_refresh: Callback<bool>,
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
                        // We good, deserialize data and set feed since we now have a feed
                        let feed = res.json::<Feed>().await.unwrap();
                        let window = web_sys::window().unwrap();
                        let local_storage = window.local_storage().unwrap().unwrap();
                        let user_str = serde_json::to_string(&feed).unwrap();
                        let _ = local_storage.set("feed", &user_str);
                        feed_state.set(Some(feed));
                    } else {
                        // Not good, likely expired token/unauth so log out user
                        on_logout.emit(true);
                    }
                }
            })
        }
    }

    let handle_on_delete = { Callback::from(move |index: usize| {}) };

    let news_cards_html = {
        if let Some(feed) = feed_state.deref() {
            feed.data
                .iter()
                .enumerate()
                .map(|(index, article)| {
                    html! {
                        <NewsCard article={article.clone()} article_index={index} saved={None} on_delete={&handle_on_delete} />
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
