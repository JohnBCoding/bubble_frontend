use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let logged_in_state = use_state(|| Some(true));

    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <NavBar />
            if logged_in_state.is_none() {
                <Login />
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
