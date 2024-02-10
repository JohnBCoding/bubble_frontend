use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_refresh: Callback<MouseEvent>,
    //pub page_state: PageState,
    //pub on_change_state: Callback<PageState>,
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
        //let on_change_state = props.on_change_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // let value = event
            //     .target_unchecked_into::<HtmlAnchorElement>()
            //     .get_attribute("value")
            //     .unwrap();
            // let new_state = match value.as_str() {
            //     "dashboard" => PageState::Dashboard,
            //     "profile" => PageState::Profile,
            //     "inventory" => PageState::Inventory,
            //     "shopping" => PageState::Shopping,
            //     _ => PageState::Dashboard,
            // };

            // let menu = menu_ref.cast::<HtmlDivElement>().unwrap();
            // menu.set_class_name("navbar-menu-hide");

            // on_change_state.emit(new_state);
        })
    };

    html! {
        <>
            <div class="row">
                <span class="navbar-logo flex-center-y">{"Bubble"}</span>
                <button class="flex-end-x" onclick={&props.on_refresh}>{{"Refresh"}}</button>
                <button class="navbar-toggle" onclick={&on_toggle_menu}>{"="}</button>
            </div>
            <div class="navbar-menu-hide" ref={menu_ref}>
                <div class="col">
                    <a href="" class="navbar-link navbar-link-selected" value="feed" onclick={&on_navigate} >{"Feed"}</a>
                    <a href="" class="navbar-link" value="saved" onclick={&on_navigate} >{"Saved"}</a>
                    <a href="" class="navbar-link" value="logout" onclick={&on_navigate} >{"Logout"}</a>
                </div>
            </div>
        </>
    }
}
