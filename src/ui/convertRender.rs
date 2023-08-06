use dioxus::{prelude::*, html::pattern};

use regex::Regex;

use super::props;


#[inline_props]
pub fn ConvertRender<'a>(
    cx: Scope<'a>,
    on_state_change: EventHandler<'a, props::WindowTypes>,
    
) -> Element<'a> {
    

    cx.render(rsx!(
        script {
            include_str!("./web/render.js")
        }
        div { class: "cr-row-grid",
            div { class: "cr-main-column-grid",
                div { class: "cr-canvas", id: "cr-default-canvas" }

            }
            div { class: "cu-footer",
                button { class: "cu-footer-button button-exit", onclick: move |event| {
                    cx.props.on_state_change.call(props::WindowTypes::ConfigureConvert)
                    },
                    "Back"
                },

            }


        }
    ))
}
