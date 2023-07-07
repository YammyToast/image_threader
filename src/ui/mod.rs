#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types


use dioxus::prelude::*;

mod menu;
mod props;
mod convertUpload;



// define a component that renders a div with the text "Hello, world!"
pub fn App(cx: Scope) -> Element {

    // let mut state = props::WindowTypes::MainMenu;
    let mut app_state = use_state(cx, || props::WindowTypes::ConvertUpload); 

    let stateHandler = move |value: props::WindowTypes| {app_state.set(value)};

    // let render: Element = match app_state.get() {
    //     props::WindowTypes::MainMenu => {
    //         rsx! {

    //         }
    //     },
    //     _ => menu::Menu(cx, app_state)
    // };



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

