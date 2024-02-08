use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <NavBar />
            <div class="col overflow-y">
                <NewsCard />
                <NewsCard />
                <NewsCard />
                <NewsCard />
                <NewsCard />
                <NewsCard />
                <NewsCard />
            </div>
        </main>
    }
}
