use dioxus::prelude::*;

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "menu-row-grid",
        div { class: "menu-grid-item menu-header",
            img {
                src: "./src/assets/menu-title-logo.svg"

            }
        },
        div { class: "menu-grid-item menu-content",
            
            button { class: "menu-content-button",
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
    ))
}
