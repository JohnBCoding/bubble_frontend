use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    //pub page_state: PageState,
    //pub on_change_state: Callback<PageState>,
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
        <a class="news-card-container row expand-x" href="https://www.tmz.com/2020/08/04/rafael-nadal-us-open-tennis-covid-19-concerns/" target="_blank">
            <div class="col expand-x">
                <h1>{"Rafael Nadal Pulls Out Of U.S. Open Over COVID-19 Concerns"}</h1>
                <p>{"Rafael Nadal is officially OUT of the U.S. Open ... the tennis legend said Tuesday it's just too damn unsafe for him to travel to America during the COVID-19 pandemic. \"The situation is very complicated worldwide,\" Nadal wrote in a statement. \"The…"}</p>
                <div class="row flex-end-y">
                    <span>{"TMZ.com ( TMZ Staff )"}</span>
                    <span class="flex-end-x">{"2020-08-05"}</span>
                </div>
            </div>
            <div class="col">
                <img class="" src={"https://imagez.tmz.com/image/fa/4by3/2020/08/04/fad55ee236fc4033ba324e941bb8c8b7_md.jpg"} />
                <div class="row flex-end-y">
                    <button class="news-card-button" onclick={&on_click_action}>{"\u{1F44D}"}</button>
                    <button class="news-card-button" onclick={&on_click_action}>{"\u{1F44E}"}</button>
                    <button class="news-card-button" onclick={&on_click_action}>{"\u{1F4BE}"}</button>
                </div>
            </div>
        </a>
    }
}
