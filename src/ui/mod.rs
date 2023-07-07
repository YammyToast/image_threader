#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types

use dioxus::prelude::*;

// define a component that renders a div with the text "Hello, world!"
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style { include_str!("./style.css") }
            main {
                div { class: "main-wrapper",
                    div { class: "menu-row-grid",
                        div { class: "menu-grid-item menu-header",
                            img {
                                src: "./src/assets/menu-title-logo.svg"

                            }
                        },
                        div { class: "menu-grid-item",
                            div { class: "menu-content-button",     
                                img {src: "./src/assets/menu-image.svg", alt: "Convert from Image"}
                                div { class: "menu-content-button-text",
                                    "Convert from Image"
                                }
                            },
                            div { class: "menu-content-button",     
                                img {src: "./src/assets/menu-image.svg"}
                                div { class: "menu-content-button-text",
                                    "View Converted Image"
                                }
                            },
                        },
                        div { class: "menu-grid-item menu-footer",
                            div { class: "menu-footer-text",
                                "YammyToast, 2023"
                            }

                        }

                    }
                }
            }
        }
    })
}
