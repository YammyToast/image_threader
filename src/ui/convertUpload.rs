use dioxus::prelude::*;

use super::props;

#[inline_props]
pub fn ConvertUpload<'a>(
    cx: Scope<'a>,
    on_state_change: EventHandler<'a, props::WindowTypes>,
    on_file_upload: EventHandler<'a, String>,
    file_obj: &'a props::FileObject,
) -> Element<'a> {
    let filename = match cx.props.file_obj.loaded {
        true => String::from(format!(
            "File Selected: {0}",
            cx.props.file_obj.file_address.clone()
        )),
        false => String::from(""),
    };
    let dimensions = match cx.props.file_obj.loaded {
        true => String::from(format!(
            "File Dimensions: {0} x {1}",
            cx.props.file_obj.width,
            cx.props.file_obj.height
        )),
        false => String::from("")
    };

    let submit_button = match cx.props.file_obj.loaded {
        true => rsx! {
            button { class: "cu-footer-button button-submit", onclick: move |event| {
                cx.props.on_state_change.call(props::WindowTypes::ConfigureConvert)
                },
                "Next"
            }
        },
        false => rsx! {
            div { class: "cu-footer-button button-submit-disabled ",
            "Next"
        }
        }
    };

    cx.render(rsx!(
        script {
            include_str!("./web/upload.js")
        }
        div { class: "cu-row-grid",
            div { class: "cu-title",
                "Upload an Image"},
            div {
                div { class: "cu-p",
                    "Select a file to upload. Supported image types are: "
                    br {}
                    ".png"
                    br {}
                    ".jpg"


                }
            },
            div {
                label { class: "cu-upload-card", r#for:"upload_default",
                    div { class: "cu-upload-button",
                        img {src: "./src/assets/cu-upload.svg"}
                        "Click to Upload"
                    }
                    div { class: "cu-upload-title",
                        filename
                        br {}
                        dimensions
                    }

                }
                input {
                    id: "upload_default",
                    r#type: "file",
                    accept: ".png,.jpg",
                    name: "files[]",

                }
                input {
                    id: "upload_data_buffer",
                    r#type: "text",
                    name: "upload_default_buffer",
                    oninput: move |event| {
                        cx.props.on_file_upload.call(event.value.clone());
                    }
                }
            },
            div { class: "cu-footer",
                button { class: "cu-footer-button button-exit", onclick: move |event| {
                    cx.props.on_state_change.call(props::WindowTypes::MainMenu)
                    },
                    "Back"
                }
                submit_button
            },

        }

    ))
}
