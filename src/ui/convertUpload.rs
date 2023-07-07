use dioxus::prelude::*;

use super::props;

#[inline_props]
pub fn ConvertUpload<'a>(cx: Scope<'a>, on_state_change: EventHandler<'a, props::WindowTypes>) -> Element<'a> {
    cx.render(rsx!(
        div {
            "Convert Upload"
        }
        button { class: "menu-content-button", onclick: move |event| {
            cx.props.on_state_change.call(props::WindowTypes::MainMenu)
            },
            "Back"
        }
    ))
}
