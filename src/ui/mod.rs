#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types

use dioxus::prelude::*;

mod menu;
mod props;



// define a component that renders a div with the text "Hello, world!"
pub fn App(cx: Scope) -> Element {
    let state: props::WindowState = props::WindowState::MainMenu;

    let render: Element = match state {
        props::WindowState::MainMenu => menu::Menu(cx),
        _ => menu::Menu(cx)
    };



    cx.render(rsx! {
        div {
            style { include_str!("./style.css") }
            main {
                div { id: "debug", "State: {state:?}" }
                div { class: "main-wrapper",
                    render
                }
            }
        }
    })
}

