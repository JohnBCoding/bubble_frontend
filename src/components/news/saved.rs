use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub saved: Option<Vec<Article>>,
}

#[function_component(SavedNews)]
pub fn saved_news(props: &Props) -> Html {
    let saved_card_html = if let Some(saved) = props.saved.clone() {
        saved
            .iter()
            .map(|article| {
                html! {
                    <NewsCard article={article.clone()}/>
                }
            })
            .collect::<Html>()
    } else {
        html! {}
    };

    html! {
        <div class="col overflow-y fade-in">
            { saved_card_html }
        </div>
    }
}
