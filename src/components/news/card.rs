use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub article: Article,
    pub article_index: usize,
    pub saved: Option<Save>,
    pub on_save: Callback<usize>,
    pub on_rate: Callback<(usize, bool)>,
    pub on_delete: Callback<usize>,
}

#[function_component(NewsCard)]
pub fn news_card(props: &Props) -> Html {
    let on_click_action = {
        let article_index = props.article_index.clone();
        let on_save = props.on_save.clone();
        let on_rate = props.on_rate.clone();
        let on_delete = props.on_delete.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();

            let value = if let Some(str) = event
                .target_unchecked_into::<HtmlAnchorElement>()
                .get_attribute("name")
            {
                str
            } else {
                event.target_unchecked_into::<SvgElement>().id()
            };

            match value.as_str() {
                "save" => on_save.emit(article_index),
                "like" => on_rate.emit((article_index, true)),
                "dislike" => on_rate.emit((article_index, false)),
                "delete" => on_delete.emit(article_index),
                _ => {}
            }
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
                if props.saved.is_none(){
                    <div class="row flex-end-y">
                        <a class="news-card-button" name="like" onclick={&on_click_action}>
                            <svg id={"like"} class="news-card-svg" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                                <path id={"like"} d="M14.8285 14.8284L16.2427 13.4142L12.0001 9.17161L7.75745 13.4142L9.17166 14.8285L12.0001 12L14.8285 14.8284Z" fill="#6d4541"/>
                                <path id={"like"} fill-rule="evenodd" clip-rule="evenodd" d="M1 19C1 21.2091 2.79086 23 5 23H19C21.2091 23 23 21.2091 23 19V5C23 2.79086 21.2091 1 19 1H5C2.79086 1 1 2.79086 1 5V19ZM5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21Z" fill="#6d4541"/>
                            </svg>
                        </a>
                        <a class="news-card-button" name="dislike" onclick={&on_click_action}>
                            <svg id={"dislike"} class="news-card-svg" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                                <path id={"dislike"} d="M7.75739 10.5858L9.1716 9.17154L12 12L14.8284 9.17157L16.2426 10.5858L12 14.8284L7.75739 10.5858Z" fill="#6d4541" />
                                <path id={"dislike"} fill-rule="evenodd" clip-rule="evenodd" d="M1 5C1 2.79086 2.79086 1 5 1H19C21.2091 1 23 2.79086 23 5V19C23 21.2091 21.2091 23 19 23H5C2.79086 23 1 21.2091 1 19V5ZM5 3H19C20.1046 3 21 3.89543 21 5V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V5C3 3.89543 3.89543 3 5 3Z" fill="#6d4541" />
                            </svg>
                        </a>
                        <a class="news-card-button" name="save" onclick={&on_click_action}>
                            <svg id={"save"} class="news-card-svg"  xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                                <path id={"save"} d="M11 14.5V16.5H13V14.5H15V12.5H13V10.5H11V12.5H9V14.5H11Z" fill="#6d4541" />
                                <path id={"save"} fill-rule="evenodd" clip-rule="evenodd" d="M4 1.5C2.89543 1.5 2 2.39543 2 3.5V4.5C2 4.55666 2.00236 4.61278 2.00698 4.66825C0.838141 5.07811 0 6.19118 0 7.5V19.5C0 21.1569 1.34315 22.5 3 22.5H21C22.6569 22.5 24 21.1569 24 19.5V7.5C24 5.84315 22.6569 4.5 21 4.5H11.874C11.4299 2.77477 9.86384 1.5 8 1.5H4ZM9.73244 4.5C9.38663 3.9022 8.74028 3.5 8 3.5H4V4.5H9.73244ZM3 6.5C2.44772 6.5 2 6.94772 2 7.5V19.5C2 20.0523 2.44772 20.5 3 20.5H21C21.5523 20.5 22 20.0523 22 19.5V7.5C22 6.94772 21.5523 6.5 21 6.5H3Z" fill="#6d4541" />
                            </svg>
                        </a>
                    </div>
                } else {
                    <div class="row flex-end-y">
                        <a class="news-card-button" name="delete" onclick={&on_click_action}>
                            <svg id={"delete"} class="news-card-svg" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" class="" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                                <path id={"delete"} d="M9 14.5V12.5H15V14.5H9Z" fill="#6d4541" />
                                <path id={"delete"} fill-rule="evenodd" clip-rule="evenodd" d="M4 1.5C2.89543 1.5 2 2.39543 2 3.5V4.5C2 4.55666 2.00236 4.61278 2.00698 4.66825C0.838141 5.07811 0 6.19118 0 7.5V19.5C0 21.1569 1.34315 22.5 3 22.5H21C22.6569 22.5 24 21.1569 24 19.5V7.5C24 5.84315 22.6569 4.5 21 4.5H11.874C11.4299 2.77477 9.86384 1.5 8 1.5H4ZM9.73244 4.5C9.38663 3.9022 8.74028 3.5 8 3.5H4V4.5H9.73244ZM3 6.5C2.44772 6.5 2 6.94772 2 7.5V19.5C2 20.0523 2.44772 20.5 3 20.5H21C21.5523 20.5 22 20.0523 22 19.5V7.5C22 6.94772 21.5523 6.5 21 6.5H3Z" fill="#6d4541" />
                            </svg>
                        </a>
                    </div>
                }

            </div>
        </a>
    }
}
