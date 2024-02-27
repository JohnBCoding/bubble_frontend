use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let logged_in_state = use_state(|| None::<User>);
    let page_state = use_state(|| PageState::Login);
    let alert_text_state = use_state(|| "".to_string());
    let refresh_state = use_state(|| false);

    // Load user if state is none
    if logged_in_state.is_none() {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        if let Ok(result) = local_storage.get("user") {
            if let Some(user_str) = result {
                let user = serde_json::from_str(&user_str).unwrap();
                logged_in_state.set(Some(user));
                page_state.set(PageState::Feed);
            }
        }
    }

    let handle_set_login = {
        let logged_in_state = logged_in_state.clone();
        let page_state = page_state.clone();
        Callback::from(move |user: User| {
            let window = web_sys::window().unwrap();
            let local_storage = window.local_storage().unwrap().unwrap();
            let _ = local_storage.set("user", &serde_json::to_string(&user).unwrap());
            logged_in_state.set(Some(user));
            page_state.set(PageState::Feed);
        })
    };

    let handle_on_logout = {
        let logged_in_state = logged_in_state.clone();
        let page_state = page_state.clone();
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let local_storage = window.local_storage().unwrap().unwrap();
            let _ = local_storage.clear();

            logged_in_state.set(None);
            page_state.set(PageState::Login);
        })
    };

    let handle_on_change_state = {
        let page_state = page_state.clone();
        Callback::from(move |new_state: PageState| {
            page_state.set(new_state);
        })
    };

    let handle_update_alert_text = {
        let alert_text_state = alert_text_state.clone();
        Callback::from(move |new_text: String| {
            let alert_text_state = alert_text_state.clone();
            alert_text_state.set(new_text);
            let _ = Timeout::new(3000, move || {
                alert_text_state.set("".to_string());
            })
            .forget();
        })
    };

    let handle_on_refresh = {
        let refresh_state = refresh_state.clone();
        Callback::from(move |_| {
            // Toggle refresh
            if *refresh_state {
                let window = web_sys::window().unwrap();
                let local_storage = window.local_storage().unwrap().unwrap();

                // Save user information as it doesn't need to be cleared fully
                let user_str = if let Ok(user_res) = local_storage.get("user") {
                    if let Some(user_str) = user_res {
                        user_str
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };

                // Clear storage, reset user information
                let _ = local_storage.clear();
                if !user_str.is_empty() {
                    let _ = local_storage.set("user", &user_str);
                }

                refresh_state.set(false);
            } else {
                refresh_state.set(true);
            }
        })
    };

    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <NavBar page_state={page_state.deref().clone()} alert_text={alert_text_state.deref().clone()} on_refresh_state={&handle_on_refresh} on_change_state={&handle_on_change_state} on_logout={&handle_on_logout}/>
            {
                match *page_state {
                    PageState::Login => {
                        html!{
                            <Login on_set_login={&handle_set_login}/>
                        }
                    }
                    PageState::Feed => {
                        html! {
                            <NewsFeed user_id={logged_in_state.deref().clone().unwrap().user_id} refresh={*refresh_state} on_refresh={&handle_on_refresh} on_logout={&handle_on_logout} on_update_alert_text={&handle_update_alert_text}/>
                        }
                    },
                    PageState::Saved => {
                        html!{<NewsSaved user_id={logged_in_state.deref().clone().unwrap().user_id} refresh={*refresh_state} on_refresh={&handle_on_refresh} on_logout={&handle_on_logout} />}
                    }
                }
            }
        </main>
    }
}
