use dioxus::prelude::*;

use super::props;

#[inline_props]
pub fn ConvertUpload<'a>(cx: Scope<'a>, on_state_change: EventHandler<'a, props::WindowTypes>) -> Element<'a> {
    cx.render(rsx!(
        div { class: "cu-row-grid",
            div { class: "cu-title",
                "Upload an Image"},
            div {
                div { class: "cu-p",
                    "Select a file to upload. Supported image types are: "
                    br {}
                    "test"
                    br {}
                    "test"


                }
            },
            div {
                div { class: ""
                    

                }
                input { 
                    id: "upload_default",
                    r#type: "file",
                    accept: ".png,.jpg",
                }
            },
            div { class: "cu-footer",
                button { class: "cu-footer-button button-exit", onclick: move |event| {
                    cx.props.on_state_change.call(props::WindowTypes::MainMenu)
                    },
                    "Back"
                }
                button { class: "cu-footer-button button-submit", onclick: move |event| {
                    cx.props.on_state_change.call(props::WindowTypes::ConfigureConvert)
                    },
                    "Convert"
                }
            },

        }
        
    ))
}
