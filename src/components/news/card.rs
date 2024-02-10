use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub article: Article,
}

#[function_component(NewsCard)]
pub fn news_card(props: &Props) -> Html {
    let on_click_action = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();

            use gloo_console::log;
            log!("button");
        })
    };

    html! {
        <a class="news-card-container row expand-x" href={props.article.url.clone()} target="_blank">
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
                    <button class="news-card-button" onclick={&on_click_action}>{"\u{1F44D}"}</button>
                    <button class="news-card-button" onclick={&on_click_action}>{"\u{1F44E}"}</button>
                    <button class="news-card-button" onclick={&on_click_action}>{"\u{1F4BE}"}</button>
                </div>
            </div>
        </a>
    }
}
