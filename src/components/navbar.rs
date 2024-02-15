use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub page_state: PageState,
    pub on_refresh_state: Callback<bool>,
    pub on_change_state: Callback<PageState>,
    pub on_logout: Callback<bool>,
}

#[function_component(NavBar)]
pub fn navbar(props: &Props) -> Html {
    let menu_ref = use_node_ref();

    let on_toggle_menu = {
        let menu_ref = menu_ref.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let menu = menu_ref.cast::<HtmlDivElement>().unwrap();
            if menu.class_name().contains("navbar-menu-hide") {
                menu.set_class_name("navbar-menu row expand-x expand-y");
            } else {
                menu.set_class_name("navbar-menu-hide");
            }
        })
    };

    let on_navigate = {
        let menu_ref = menu_ref.clone();
        let on_change_state = props.on_change_state.clone();
        let on_logout = props.on_logout.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let value = event
                .target_unchecked_into::<HtmlAnchorElement>()
                .get_attribute("value")
                .unwrap();
            let new_state = match value.as_str() {
                "feed" => PageState::Feed,
                "saved" => PageState::Saved,
                "logout" => {
                    on_logout.emit(true);
                    PageState::Login
                }
                _ => PageState::Feed,
            };

            let menu = menu_ref.cast::<HtmlDivElement>().unwrap();
            menu.set_class_name("navbar-menu-hide");

            on_change_state.emit(new_state);
        })
    };

    let on_refresh = {
        let on_refresh_state = props.on_refresh_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            on_refresh_state.emit(true);
        })
    };

    html! {
        <>
            <div class="row">
                <span class="navbar-logo flex-center-y">{"Bubble"}</span>
                if props.page_state != PageState::Login {
                    <button class="flex-end-x" onclick={&on_refresh}>{{"Refresh"}}</button>
                    <button class="navbar-toggle" onclick={&on_toggle_menu}>{"="}</button>
                }
            </div>
            <div class="navbar-menu-hide" ref={menu_ref}>
                {
                    match props.page_state {
                        PageState::Feed => {
                            html! {
                                <div class="col">
                                    <a href="" class="navbar-link navbar-link-selected" value="feed" onclick={&on_navigate} >{"Feed"}</a>
                                    <a href="" class="navbar-link" value="saved" onclick={&on_navigate}>{"Saved"}</a>
                                    <a href="" class="navbar-link" value="logout" onclick={&on_navigate}>{"Logout"}</a>
                                </div>
                            }
                        },
                        PageState::Saved => {
                            html! {
                                <div class="col">
                                    <a href="" class="navbar-link" value="feed" onclick={&on_navigate} >{"Feed"}</a>
                                    <a href="" class="navbar-link navbar-link-selected" value="saved" onclick={&on_navigate}>{"Saved"}</a>
                                    <a href="" class="navbar-link" value="logout" onclick={&on_navigate}>{"Logout"}</a>
                                </div>
                            }
                        },
                        _=> {html!{}}
                    }
                }
            </div>
        </>
    }
}
