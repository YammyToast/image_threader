use dioxus::prelude::*;

use super::props;

#[inline_props]
pub fn Menu<'a>(cx: Scope<'a>, on_state_change: EventHandler<'a, props::WindowTypes>) -> Element<'a> {
    cx.render(rsx!(
        div { class: "menu-row-grid",
        div { class: "menu-grid-item menu-header",
            img {
                src: "./src/assets/menu-title-logo.svg"

            }
        },
        div { class: "menu-grid-item menu-content",
            
            button { class: "menu-content-button", onclick: move |event| {
                cx.props.on_state_change.call(props::WindowTypes::ConvertUpload)
                },
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
