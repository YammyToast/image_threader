#![allow(non_snake_case)]


use dioxus::prelude::*;

mod menu;
mod props;
mod convertUpload;



pub fn App(cx: Scope) -> Element {

    let mut app_state = use_state(cx, || props::WindowTypes::ConvertUpload); 
    let mut file_obj = use_state(cx, || )

    let stateHandler = move |value: props::WindowTypes| {app_state.set(value)};



    cx.render(rsx! {
        div {
            script { 
                include_str!("./web/jquery-3.7.0.min.js")
            }
            style { include_str!("./web/style.css") }
            main {
                div { id: "debug", "State: {app_state:?}" }
                div { class: "main-wrapper",
                    match app_state.get() {
                        props::WindowTypes::MainMenu => rsx! { menu::Menu {
                            on_state_change: stateHandler
                        }},
                        props::WindowTypes::ConvertUpload => rsx! { convertUpload::ConvertUpload {
                            on_state_change: stateHandler
                        }},
                        _ => rsx! {menu::Menu {
                            on_state_change: stateHandler
                        }}

                    }
                }
            }
        }
    })
}

