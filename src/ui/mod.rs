#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types

use dioxus::prelude::*;

// define a component that renders a div with the text "Hello, world!"
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            main {
                "ImageThreader"
            }
        }
    })
}
