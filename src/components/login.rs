use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    //pub page_state: PageState,
    //pub on_change_state: Callback<PageState>,
}

#[function_component(Login)]
pub fn logion(props: &Props) -> Html {
    html! {
        <div class="login-container col expand-x">
            <h1 class="flex-center-x">{"Login"}</h1>
            <div class="col">
                <label for="email">{"Email"}</label>
                <input id="email" placeholder={"Email..."}/>
            </div>
            <div class="col">
                <label for="password">{"Password"}</label>
                <input id="password" type="password" placeholder={"Password..."}/>
            </div>
            <button>{"Login"}</button>
            <button>{"Register"}</button>
        </div>
    }
}
