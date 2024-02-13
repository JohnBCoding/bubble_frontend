use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub article: Article,
}

#[function_component(NewsCard)]
pub fn news_card(props: &Props) -> Html {
    let on_click_action = {
        let article = props.article.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();

            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let article = article.clone();
            match value.as_str() {
                "save" => wasm_bindgen_futures::spawn_local(async move {
                    let window = web_sys::window().unwrap();
                    let local_storage = window.local_storage().unwrap().unwrap();
                    if let Ok(user_res) = local_storage.get("user") {
                        if let Some(user_str) = user_res {
                            let user = serde_json::from_str::<User>(&user_str).unwrap();
                            let save = Save::new(&user.user_id, &article);
                            let save_str = serde_json::to_string(&save).unwrap();
                            let uri_base = std::env!("SERVER_URI_BASE");
                            let url = format!("{}/news/save", uri_base);
                            let result = Request::post(&url)
                                .header("Content-Type", "application/json")
                                .body(JsValue::from_str(&save_str))
                                .credentials(web_sys::RequestCredentials::Include)
                                .send()
                                .await;

                            // send notification here
                        }
                    }
                }),
                _ => {}
            }
            use gloo_console::log;
            log!("button");
        })
    };

    html! {
        <a class="news-card-container row expand-x fade-in" href={props.article.url.clone()} target="_blank">
            <div class="col expand-x">
                <h1>{&props.article.title}</h1>
                <p>{&props.article.description}</p>
                <div class="row flex-end-y">
                    <span>{format!("{} ( {} )", &props.article.author, &props.article.source)}</span>
                    <span class="flex-end-x">{&props.article.published_at}</span>
                </div>
            </div>
            <div class="col">
                <img class="" src={props.article.image.clone()} />
                <div class="row flex-end-y">
                    <button class="news-card-button" value={"like"} onclick={&on_click_action}>{"\u{1F44D}"}</button>
                    <button class="news-card-button" value={"dislike"} onclick={&on_click_action}>{"\u{1F44E}"}</button>
                    <button class="news-card-button" value={"save"} onclick={&on_click_action}>{"\u{1F4BE}"}</button>
                </div>
            </div>
        </a>
    }
}
