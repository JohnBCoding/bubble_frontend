use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_set_login: Callback<User>,
}

#[function_component(Login)]
pub fn logion(props: &Props) -> Html {
    let email_state = use_state(|| "".to_string());
    let password_state = use_state(|| "".to_string());

    let on_change_email = {
        let email_state = email_state.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();
            let value = event.target_unchecked_into::<HtmlInputElement>().value();

            email_state.set(value);
        })
    };

    let on_change_password = {
        let password_state = password_state.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();
            let value = event.target_unchecked_into::<HtmlInputElement>().value();

            password_state.set(value);
        })
    };

    let on_login = {
        let email_state = email_state.clone();
        let password_state = password_state.clone();
        let on_set_login = props.on_set_login.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let action_type = "login".to_string();
            let user = UserLogin {
                email: email_state.deref().clone(),
                password: password_state.deref().clone(),
                return_secure_token: true,
            };
            let user_json = serde_json::to_string(&user).unwrap();
            let on_set_login = on_set_login.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let uri_base = std::env!("SERVER_URI_BASE");
                let url = match action_type.as_str() {
                    "create" => format!("{}/auth/register", uri_base),
                    _ => format!("{}/auth/login", uri_base),
                };
                let result = Request::post(&url)
                    .header("Content-Type", "application/json")
                    .credentials(web_sys::RequestCredentials::Include)
                    .body(JsValue::from_str(&user_json))
                    .send()
                    .await;

                match result {
                    Ok(res) => {
                        let user = res.json::<User>().await.unwrap();
                        on_set_login.emit(user);
                    }
                    Err(err) => {
                        use gloo_console::log;
                        log!(format!("{:?}", err));
                    }
                }
            });
        })
    };

    html! {
        <div class="login-container col expand-x">
            <h1 class="flex-center-x">{"Login"}</h1>
            <div class="col">
                <label for="email">{"Email"}</label>
                <input id="email" placeholder={"Email..."} onchange={&on_change_email}/>
            </div>
            <div class="col">
                <label for="password">{"Password"}</label>
                <input id="password" type="password" placeholder={"Password..."} onchange={&on_change_password}/>
            </div>
            <button onclick={&on_login}>{"Login"}</button>
            <button>{"Register"}</button>
        </div>
    }
}
